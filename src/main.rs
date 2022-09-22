#![allow(dead_code, unused_imports, unused_variables, non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

mod application;
mod data_dictionary;
mod message;
mod network;
mod quickfix_errors;
mod session;

use data_dictionary::*;
use fields::*;
use message::*;
use network::SocketAcceptor;
use session::SessionSetting;
use tokio;

pub(crate) const FILE_PATH: &str = "resources/FIX43.xml";
pub(crate) const CONFIG_TOML_PATH: &str = "src/FixConfig.toml";

#[tokio::main]
async fn main() {
    let session_settings = SessionSetting::new(CONFIG_TOML_PATH);
    let acceptor = SocketAcceptor::new(&session_settings);
    acceptor.initialize(&session_settings).await;
}
