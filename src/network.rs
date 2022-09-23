#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::{self, FromStr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{self, net::TcpListener, net::TcpStream, task::JoinHandle};

use crate::application::Application;
use crate::{data_dictionary::*, session};
// use crate::message::store::*;

use crate::message::*;
use crate::session::session_constants::*;
use crate::session::*;

// pub trait Connecter {
//     fn start(&self) -> Vec<thread::JoinHandle<()>>;
//     fn stop();
// }

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConnectionType {
    ACCEPTOR,
    INITIATOR,
}

impl FromStr for ConnectionType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(ACCEPTOR_CONN_TYPE) {
            Ok(ConnectionType::ACCEPTOR)
        } else if s.eq_ignore_ascii_case(INITIATOR_CONN_TYPE) {
            Ok(ConnectionType::INITIATOR)
        } else {
            Err("invalid connection type")
        }
    }
}

#[derive(Debug)]
pub struct SocketAcceptor {
    connection_type: ConnectionType,
    session_map: HashMap<SessionId, Session>,
    sockets: HashSet<SocketAddrV4>,
}

impl Default for SocketAcceptor {
    fn default() -> Self {
        Self {
            connection_type: ConnectionType::ACCEPTOR,
            session_map: HashMap::new(),
            sockets: HashSet::new(),
        }
    }
}

impl SocketAcceptor {
    pub fn new(settings: &Properties) -> Self {
        let mut socket_connector = SocketAcceptor::default();
        socket_connector.create_sessions(settings);
        socket_connector
    }

    pub fn send(&self, msg: Message) {
        println!("{:?}", msg);
    }

    pub fn recv(&self) -> Message {
        Message::new()
    }

    pub fn set_connection_type(&mut self, con_ty: ConnectionType) {
        self.connection_type = con_ty;
    }

    pub fn get_connection_type(&self) -> ConnectionType {
        self.connection_type
    }

    pub fn set_session(&mut self, sid: SessionId, session: Session) {
        self.session_map.insert(sid, session);
    }

    fn create_sessions(&mut self, settings: &Properties) {
        let connection_type: ConnectionType =
            settings.default_property(CONNECTION_TYPE_SETTING).unwrap();
        self.set_connection_type(connection_type);
        for session_id in settings.session_ids() {
            let session = Session::with_settings(session_id, settings);
            self.set_session(session_id.clone(), session);
        }
    }

    pub async fn initialize(&self, settings: &Properties) {
        let mut join_handles: Vec<JoinHandle<()>> = Vec::new();
        for (sid, _) in self.session_map.iter() {
            // get the socket accept port
            let session_accept_port: u16 =
                settings.get_or_default(sid, SOCKET_ACCEPT_PORT_SETTING).unwrap();
            println!("Got the port: {}", session_accept_port);

            let handle = tokio::spawn(async move {
                let listener = TcpListener::bind(("127.0.0.1", session_accept_port)).await.unwrap();
                println!("Port binding done");
                loop {
                    let (stream, _) = listener.accept().await.unwrap();
                    println!("Accepted connection");
                    handle_connection(stream).await;
                }
            });
            join_handles.push(handle);
        }
        for handle in join_handles {
            handle.await.unwrap();
        }
    }
}

async fn handle_connection(mut tcp_stream: TcpStream) {
    println!("handling connection");
    let mut buf = [0; 512];
    loop {
        match tcp_stream.read(&mut buf).await {
            Ok(bytes_read) => tcp_stream.write_all(&buf[..bytes_read]).await.unwrap(),
            Err(_) => break,
        };
    }
}

#[cfg(test)]
mod networkio_tests {}
