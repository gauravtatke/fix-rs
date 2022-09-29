#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;
use crate::session::*;

pub trait Application {
    fn to_app(msg: String);
    fn from_app(&self, session_id: SessionId, msg: Message);
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

    fn from_app(&self, session_id: SessionId, msg: Message) {
        // do nothing
        println!("from_app: {}::{:?}", session_id, msg);
    }
}
