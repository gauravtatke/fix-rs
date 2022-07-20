#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;

pub trait Application {
    fn to_app(msg: Message);
    fn from_app(msg: Message);
}

pub struct DefaultApplication;

impl DefaultApplication {
    pub fn new() -> Self {
        Self
    }
}

impl Application for DefaultApplication {
    fn to_app(msg: Message) {
        // do nothing
    }

    fn from_app(msg: Message) {
        // do nothing
    }
}
