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
    // let data_dict = data_dictionary::DataDictionary::from_xml(FILE_PATH);
    // let no_order_grp = data_dict.get_msg_group("E", 73).unwrap();
    // let no_alloc_grp = no_order_grp.data_dictionary().get_msg_group("E", 78);
    // println!("{:#?}", no_alloc_grp);
}
