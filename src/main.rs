#![allow(dead_code, unused_variables)]

//! Demo binary: loads config, builds the session registry, and runs acceptors/initiators using
//! the `fix_rs` engine library. `SampleApp` is the reference `Application` implementation.

mod sample_app;

use crate::sample_app::SampleApp;
use fix_rs::session::{ConnectionType, SessionProperties};
use fix_rs::transport::sync::acceptor::IoAcceptor;
use fix_rs::transport::sync::initiator::IoInitiator;
use fix_rs::transport::{SOCKET_ACCEPT_HOST_IP, SessionMap};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};

const FILE_PATH: &str = "resources/FIX43.xml";
const FIX_CONFIG_PATH: &str = "src/FixConfig.toml";

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let settings_str = std::fs::read_to_string(FIX_CONFIG_PATH).unwrap();
    let properties = SessionProperties::from_str(&settings_str).unwrap();
    // One shared map holds every session (both roles): the timer ticks all of them,
    // acceptors are looked up in it by inbound connections, and SessionMap::send routes
    // unsolicited/cross-session messages through it.
    let session_map: SessionMap = properties
        .sessions()
        .iter()
        .map(|(id, config)| {
            let session = config.to_session(Box::new(SampleApp::new()));
            (id.clone(), session)
        })
        .collect();

    // ACCEPTOR PROCESSING
    let socket_accept_addrs = properties
        .sessions()
        .values()
        .filter(|config| config.connection_type() == ConnectionType::Acceptor)
        .map(|config| {
            SocketAddr::new(
                SOCKET_ACCEPT_HOST_IP.parse::<IpAddr>().unwrap(),
                config.socket_accept_port().unwrap(),
            )
        })
        .collect::<HashSet<SocketAddr>>();
    log::info!("Created {} session(s)", session_map.len());
    let timer_handle = fix_rs::transport::start_timer(session_map.clone());
    let mut start_handle = Vec::new();
    for addr in socket_accept_addrs {
        log::info!("Starting acceptor on {}", addr);
        let cloned_map = session_map.clone();
        let thread_handle = std::thread::spawn(move || {
            let io_acceptor = IoAcceptor::new(addr, cloned_map);
            io_acceptor.start().unwrap();
        });
        start_handle.push(thread_handle);
    }

    // INITIATOR processing
    let initiators = properties
        .sessions()
        .iter()
        .filter(|(_, config)| config.connection_type() == ConnectionType::Initiator)
        .map(|(id, config)| {
            let session = session_map.get(id).unwrap();
            let connect_addr = SocketAddr::new(
                config.socket_connect_host().as_ref().unwrap().parse::<IpAddr>().unwrap(),
                config.socket_connect_port().unwrap(),
            );
            IoInitiator::new(id.clone(), session, connect_addr)
        })
        .collect::<Vec<IoInitiator>>();
    for initiator in initiators {
        log::info!("Starting initiator on {}", initiator.connect_addr());
        let thread_handle = std::thread::spawn(move || {
            initiator.start().unwrap();
        });
        start_handle.push(thread_handle);
    }

    // wait for all threads to finish
    start_handle.drain(..).for_each(|handle| handle.join().unwrap());
}
