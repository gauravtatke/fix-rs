#![allow(dead_code, unused_variables)]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

mod application;
mod data_dictionary;
mod message;
mod network;
mod quickfix_errors;
mod session;

use std::{thread, time::Duration};

use application::DefaultApplication;
use network::SocketAcceptor;
use session::*;
use tokio;

pub(crate) const FILE_PATH: &str = "resources/FIX43.xml";
pub(crate) const CONFIG_TOML_PATH: &str = "src/FixConfig.toml";

#[tokio::main]
async fn main() {
    let session_settings = Properties::new(CONFIG_TOML_PATH);
    let application = DefaultApplication::new();
    let mut acceptor = SocketAcceptor::new(session_settings, application);
    acceptor.initialize_new();
    loop {
        thread::sleep(Duration::from_millis(5000));
    }
}
