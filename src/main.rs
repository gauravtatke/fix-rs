#![allow(dead_code, unused_variables)]

include!(concat!(env!("OUT_DIR"), "/mod.rs"));

mod application;
mod data_dictionary;
mod io;
mod message;
mod network;
mod quickfix_errors;
mod session;

use crate::application::DefaultApplication;
use crate::io::acceptor::IoAcceptor;
use crate::network::{SOCKET_ACCEPT_HOST_IP, SessionMap};
use crate::session::{ConnectionType, SessionProperties};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};

pub(crate) const FILE_PATH: &str = "resources/FIX43.xml";
pub(crate) const FIX_CONFIG_PATH: &str = "src/FixCfg.toml";

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let settings_str = std::fs::read_to_string(FIX_CONFIG_PATH).unwrap();
    let properties = SessionProperties::from_str(&settings_str).unwrap();
    // log::info!("{:#?}", properties);
    // creating session map of all acceptor sessions, silently dropping initiator
    let session_map: SessionMap = properties
        .sessions()
        .iter()
        .filter(|(id, config)| config.connection_type() == ConnectionType::Acceptor)
        .map(|(id, config)| {
            let session = config.to_session(Box::new(DefaultApplication::new()));
            (id.clone(), session)
        })
        .collect();
    let socket_addrs = properties
        .sessions()
        .values()
        .map(|config| {
            SocketAddr::new(
                SOCKET_ACCEPT_HOST_IP.parse::<IpAddr>().unwrap(),
                config.socket_accept_port().unwrap(),
            )
        })
        .collect::<HashSet<SocketAddr>>();
    log::info!("Created {} session(s)", session_map.len());
    let timer_handle = network::start_timer(session_map.clone());
    let mut start_handle = Vec::new();
    for addr in socket_addrs {
        log::info!("Starting acceptor on {}", addr);
        let cloned_map = session_map.clone();
        let thread_handle = std::thread::spawn(move || {
            let io_acceptor = IoAcceptor::new(addr, cloned_map);
            io_acceptor.start().unwrap();
        });
        start_handle.push(thread_handle);
    }
    start_handle.drain(..).for_each(|handle| handle.join().unwrap());
}
