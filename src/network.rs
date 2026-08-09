#![allow(dead_code)]
#![allow(unused_imports)]

use dashmap::{DashMap, mapref::entry::Entry};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use crate::application::Application;
use crate::session::*;

pub(crate) const SOCKET_ACCEPT_HOST_IP: &str = "127.0.0.1";

type SessionRef<'a> = dashmap::mapref::one::Ref<'a, LegacySessionId, Session>;

#[derive(Debug, Clone, Default)]
pub struct SessionMap {
    id_to_session: Arc<DashMap<LegacySessionId, Session>>,
}

impl SessionMap {
    pub fn insert_session(&self, session_id: LegacySessionId, session: Session) {
        self.id_to_session.insert(session_id, session);
    }

    pub fn get_session(&self, session_id: &LegacySessionId) -> Option<SessionRef<'_>> {
        self.id_to_session.get(session_id)
    }

    pub fn from_iter<I: IntoIterator<Item = (LegacySessionId, Session)>>(it: I) -> Self {
        Self {
            id_to_session: Arc::new(DashMap::from_iter(it)),
        }
    }

    pub fn entry(&self, session_id: &LegacySessionId) -> Entry<'_, LegacySessionId, Session> {
        self.id_to_session.entry(session_id.clone())
    }

    pub fn key_values_map(&self) -> HashMap<LegacySessionId, Session> {
        self.id_to_session
            .iter()
            .map(|sref| (sref.key().clone(), sref.value().clone()))
            .collect::<HashMap<LegacySessionId, Session>>()
    }
}

#[cfg(test)]
mod networkio_tests {}
