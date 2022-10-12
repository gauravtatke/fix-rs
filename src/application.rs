#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;
use crate::network::SessionMap;
use crate::session;
use crate::session::*;
use dashmap::DashMap;
use std::sync::Arc;

pub trait Application {
    fn to_app(msg: String);
    fn from_app(&self, session_id: &SessionId, sessions: &SessionMap, msg: Message);
}

pub struct DefaultApplication;

impl DefaultApplication {
    pub fn new() -> Self {
        Self
    }
}

impl Application for DefaultApplication {
    fn to_app(msg: String) {
        // do nothing
        println!("to_app: {:?}", msg);
    }

    fn from_app(&self, session_id: &SessionId, sessions: &SessionMap, msg: Message) {
        Session::sync_send_to_target(session_id, sessions, test_logon());
    }
}
