#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::{self, FromStr};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::{
    self, io::AsyncBufReadExt, io::BufReader, net::TcpListener, net::TcpStream, task::JoinHandle,
};

use crate::application::Application;
use crate::{data_dictionary::*, session};
// use crate::message::store::*;

use crate::message::*;
use crate::session::*;
use crate::session::*;

// pub trait Connecter {
//     fn start(&self) -> Vec<thread::JoinHandle<()>>;
//     fn stop();
// }

#[derive(Debug, Default, PartialEq)]
pub enum ConnectionType {
    #[default]
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
pub struct SocketAcceptor<A: Application + Send + Sync> {
    settings: Properties,
    connection_type: ConnectionType,
    session_map: Arc<HashMap<SessionId, Session>>,
    // sockets: HashSet<SocketAddrV4>,
    receiver: Option<mpsc::Receiver<String>>,
    sender: mpsc::Sender<String>,
    connection: Option<TcpStream>,
    app: Arc<A>,
}

impl<A: Application + Send + Sync + 'static> SocketAcceptor<A> {
    pub fn new(settings: Properties, app: A) -> Self {
        let session_map = create_sessions(&settings);
        let connection_type: ConnectionType =
            settings.default_property(CONNECTION_TYPE_SETTING).unwrap();
        let (tx, rx) = mpsc::channel(64);
        Self {
            settings,
            connection_type,
            session_map: Arc::new(session_map),
            receiver: Some(rx),
            sender: tx,
            connection: None,
            app: Arc::new(app),
        }
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

    pub fn get_connection_type(&self) -> &ConnectionType {
        &self.connection_type
    }

    pub fn get_session(&self, sid: &SessionId) -> Option<&Session> {
        self.session_map.get(sid)
    }

    pub fn initialize(&mut self) {
        for (sid, _) in self.session_map.iter() {
            // get the socket accept port
            let session_accept_port: u16 =
                self.settings.get_or_default(sid, SOCKET_ACCEPT_PORT_SETTING).unwrap();
            println!("Got the port: {}", session_accept_port);
            let tx = self.sender.clone();
            tokio::spawn(async move {
                let listener = TcpListener::bind(("127.0.0.1", session_accept_port)).await.unwrap();
                println!("Port binding done");
                loop {
                    let (stream, _) = listener.accept().await.unwrap();
                    println!("Accepted connection");
                    // tx.send("random value".to_string()).await.unwrap();
                    incoming_messages(stream, &tx).await;
                }
            });
        }

        // start a receiver task
        let receiver = self.receiver.take();
        let app = Arc::clone(&self.app);
        let sessions = Arc::clone(&self.session_map);
        tokio::spawn(async move {
            let mut receiver = receiver.unwrap();
            while let Some(s) = receiver.recv().await {
                println!("received: {}", s);
                let session_id: SessionId = Message::get_reverse_session_id(&s);
                let session = sessions.get(&session_id).unwrap();
                let dd: &DataDictionary = session.data_dictionary();
                if let Ok(message) = Message::from_str(&s, dd) {
                    if let Ok(_) = session.verify(&message) {
                        app.from_app(session_id, message);
                    } else {
                        session.send_to_target("session_verification failed")
                    }
                } else {
                    session.send_to_target("invalid message");
                }

                // app.from_app(s);
            }
        });
    }
}

fn create_sessions(settings: &Properties) -> HashMap<SessionId, Session> {
    let mut session_map = HashMap::new();
    let connection_type: ConnectionType =
        settings.default_property(CONNECTION_TYPE_SETTING).unwrap();
    // self.set_connection_type(connection_type);
    for session_id in settings.session_ids() {
        let session = Session::with_settings(session_id, settings);
        session_map.insert(session_id.clone(), session);
    }
    session_map
}

async fn handle_connection(mut tcp_stream: TcpStream, tx: &mpsc::Sender<String>) {
    println!("handling connection");
    let mut buf = [0; 512];
    loop {
        match tcp_stream.read(&mut buf).await {
            Ok(bytes_read) => {
                tx.send(String::from_utf8_lossy(&buf[..bytes_read]).to_string()).await.unwrap()
            }
            Err(_) => break,
        };
    }
}

async fn incoming_messages(tcp_stream: TcpStream, tx: &mpsc::Sender<String>) {
    println!("handling connection");
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    let mut buf_reader = BufReader::new(tcp_stream);
    loop {
        read_message(&mut buf_reader, &mut buf).await;
        tx.send(String::from_utf8_lossy(&buf[..buf.len()]).to_string()).await.unwrap();
        buf.clear();
    }
}

async fn read_message(reader: &mut BufReader<TcpStream>, buf: &mut Vec<u8>) {
    loop {
        let bytes_read = reader.read_until(SOH as u8, buf).await.unwrap();
        let slice_start = buf.len() - bytes_read;
        let slice_end = buf.len();
        // last read data
        let byte_slice = &buf[slice_start..slice_end];
        if byte_slice.starts_with(&[49, 48, 61]) {
            // b"10="
            // checksum tag found, break
            break;
        }
    }
}

#[cfg(test)]
mod networkio_tests {}
