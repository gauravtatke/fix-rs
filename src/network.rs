#![allow(dead_code)]
#![allow(unused_imports)]

use dashmap::iter::Iter;
use dashmap::{DashMap, mapref::entry::Entry};
use getset::{Getters, Setters};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver as TioReceiver, Sender as TioSender, channel as tio_channel};

use crate::application::Application;
use crate::io::acceptor::IoAcceptor;
use crate::io::*;
use crate::message::*;
use crate::session::*;

pub(crate) const SOCKET_ACCEPT_HOST_IP: &str = "127.0.0.1";

type SessionRef<'a> = dashmap::mapref::one::Ref<'a, SessionId, Session>;

#[derive(Debug, Clone, Default)]
pub struct SessionMap {
    id_to_session: Arc<DashMap<SessionId, Session>>,
}

impl SessionMap {
    pub fn insert_session(&self, session_id: SessionId, session: Session) {
        self.id_to_session.insert(session_id, session);
    }

    pub fn get_session(&self, session_id: &SessionId) -> Option<SessionRef<'_>> {
        self.id_to_session.get(session_id)
    }

    pub fn from_iter<I: IntoIterator<Item = (SessionId, Session)>>(it: I) -> Self {
        Self {
            id_to_session: Arc::new(DashMap::from_iter(it)),
        }
    }

    pub fn entry(&self, session_id: &SessionId) -> Entry<'_, SessionId, Session> {
        self.id_to_session.entry(session_id.clone())
    }

    pub fn key_values_map(&self) -> HashMap<SessionId, Session> {
        self.id_to_session
            .iter()
            .map(|sref| (sref.key().clone(), sref.value().clone()))
            .collect::<HashMap<SessionId, Session>>()
    }
}

#[cfg(test)]
mod networkio_tests {}
