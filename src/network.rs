#![allow(dead_code)]
#![allow(unused_imports)]

use getset::{Getters, Setters};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::str::{self, FromStr};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
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

pub(crate) const SOCKET_ACCEPT_HOST_IP: &str = "127.0.0.1";
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

#[derive(Debug, Getters, Setters)]
struct SocketDescriptor {
    #[getset(get)]
    addr: SocketAddr,
    #[getset(get, set)]
    is_connected: bool,
}

impl SocketDescriptor {
    fn new(sock: SocketAddr) -> Self {
        Self {
            addr: sock,
            is_connected: false,
        }
    }
}

impl Hash for SocketDescriptor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
    }
}

impl PartialEq for SocketDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl PartialEq<SocketAddr> for SocketDescriptor {
    fn eq(&self, other: &SocketAddr) -> bool {
        self.addr == *other
    }
}

impl Eq for SocketDescriptor {}

#[derive(Debug)]
pub struct SocketAcceptor<A: Application + Send + Sync> {
    settings: Properties,
    connection_type: ConnectionType,
    session_map: Arc<Mutex<HashMap<SessionId, Session>>>,
    sock_descriptors: Arc<Mutex<HashMap<SocketAddr, bool>>>,
    receiver: Option<mpsc::Receiver<String>>,
    sender: mpsc::Sender<String>,
    // connection: Arc<Mutex<HashMap<SessionId, OwnedWriteHalf>>>,
    app: Arc<A>,
}

impl<A: Application + Send + Sync + 'static> SocketAcceptor<A> {
    pub fn new(settings: Properties, app: A) -> Self {
        let session_map = create_sessions(&settings);
        let socket_desc = create_socket_descriptors(&settings);
        let connection_type: ConnectionType =
            settings.default_property(CONNECTION_TYPE_SETTING).unwrap();
        let (tx, rx) = mpsc::channel(64);
        Self {
            settings,
            connection_type,
            session_map: Arc::new(Mutex::new(session_map)),
            sock_descriptors: Arc::new(Mutex::new(socket_desc)),
            receiver: Some(rx),
            sender: tx,
            // connection: Arc::new(Mutex::new(HashMap::new())),
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

    // pub fn get_session(&self, sid: &SessionId) -> Option<&Session> {
    //     self.session_map.lock().unwrap().get(sid)
    // }

    pub fn initialize(&mut self) {
        let smap = self
            .session_map
            .lock()
            .unwrap()
            .iter()
            .map(|(sid, s)| (sid.clone(), s.clone()))
            .collect::<HashMap<SessionId, Session>>();
        for (sid, _) in smap.iter() {
            // get the socket accept port
            let session_accept_port: u16 =
                self.settings.get_or_default(sid, SOCKET_ACCEPT_PORT_SETTING).unwrap();
            println!("Got the port: {}", session_accept_port);
            let tx = self.sender.clone();
            let socket_desc = Arc::clone(&self.sock_descriptors);
            let sessions = Arc::clone(&self.session_map);
            let sid_clone = sid.clone();
            tokio::spawn(async move {
                let mut connections: HashMap<SocketAddr, Arc<Mutex<OwnedWriteHalf>>> =
                    HashMap::new();
                let sock_addrs = ("127.0.0.1", session_accept_port);
                let existing_sock_addr = sock_addrs
                    .clone()
                    .to_socket_addrs()
                    .unwrap()
                    .find(|addr| socket_desc.lock().unwrap().contains_key(addr))
                    .unwrap();
                let already_connected =
                    socket_desc.lock().unwrap().get(&existing_sock_addr).cloned().unwrap();

                if already_connected {
                    // for the session_id, add this socket_addr
                    let responder = Arc::clone(connections.get(&existing_sock_addr).unwrap());
                    {
                        // extra scope because mutex guard is not Send
                        let mut session_guard = sessions.lock().unwrap();
                        let session = session_guard.get_mut(&sid_clone).unwrap();
                        session.set_responder(responder);
                    }
                } else {
                    let listener = TcpListener::bind(sock_addrs).await.unwrap();
                    let local_addr = listener.local_addr().unwrap();
                    socket_desc.lock().unwrap().insert(local_addr, true);
                    println!("Port binding done");
                    loop {
                        let (stream, _) = listener.accept().await.unwrap();
                        println!("Accepted connection");
                        let local_addr = stream.local_addr().unwrap();
                        let (owned_read_half, owned_write_half) = stream.into_split();
                        let responder = Arc::new(Mutex::new(owned_write_half));
                        connections.insert(local_addr, Arc::clone(&responder));
                        {
                            let mut session_guard = sessions.lock().unwrap();
                            let sessn = session_guard.get_mut(&sid_clone).unwrap();
                            sessn.set_responder(responder);
                        }
                        incoming_messages(owned_read_half, &tx).await;
                    }
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
                {
                    let session_guard = sessions.lock().unwrap();
                    let session = session_guard.get(&session_id).unwrap();
                    let dd: &DataDictionary = session.data_dictionary();
                    if let Ok(message) = Message::from_str(&s, dd) {
                        println!("msg parsed: {:?}", &message);
                        if let Ok(_) = session.verify(&message) {
                            app.from_app(session_id, message);
                        } else {
                            session.send_to_target("session_verification failed").await;
                        }
                    } else {
                        session.send_to_target("invalid message").await;
                    }
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
    for session_id in settings.session_ids() {
        let session = Session::with_settings(session_id, settings);
        session_map.insert(session_id.clone(), session);
    }
    session_map
}

fn create_socket_descriptors(settings: &Properties) -> HashMap<SocketAddr, bool> {
    let mut descriptor = HashMap::new();
    let connection_type: ConnectionType =
        settings.default_property(CONNECTION_TYPE_SETTING).unwrap();
    for session_id in settings.session_ids() {
        let (host, port): (String, u16) = match connection_type {
            ConnectionType::ACCEPTOR => (
                SOCKET_ACCEPT_HOST_IP.to_string(),
                settings.get_or_default(session_id, SOCKET_ACCEPT_PORT_SETTING).unwrap(),
            ),
            ConnectionType::INITIATOR => (
                settings.get_or_default(session_id, SOCKET_CONNECT_HOST_SETTING).unwrap(),
                settings.get_or_default(session_id, SOCKET_CONNECT_PORT_SETTING).unwrap(),
            ),
        };
        let addr_str = format!("{}:{}", host, port);
        let sock_address = addr_str.parse::<SocketAddr>().unwrap();
        descriptor.insert(sock_address, false);
    }
    descriptor
}

async fn incoming_messages<R: AsyncReadExt + Unpin>(tcp_stream: R, tx: &mpsc::Sender<String>) {
    println!("handling connection");
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    let mut buf_reader = BufReader::new(tcp_stream);
    loop {
        read_message(&mut buf_reader, &mut buf).await;
        tx.send(String::from_utf8_lossy(&buf[..buf.len()]).to_string()).await.unwrap();
        buf.clear();
    }
}

async fn read_message<R: AsyncBufReadExt + Unpin>(reader: &mut R, buf: &mut Vec<u8>) {
    loop {
        let bytes_read = reader.read_until(SOH as u8, buf).await.unwrap();
        // println!("bytes received: {:?}", &buf);
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
