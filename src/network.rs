#![allow(dead_code)]
#![allow(unused_imports)]

use dashmap::DashMap;
use getset::{Getters, Setters};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::str::{self, FromStr};
use std::sync::atomic::AtomicBool;
use std::sync::{mpsc::Receiver, mpsc::Sender, Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::{channel as tio_channel, Receiver as TioReceiver, Sender as TioSender};
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

#[derive(Debug, Getters, Setters)]
#[getset(get)]
pub struct SocketAcceptor<A: Application + Send + Sync> {
    settings: Properties,
    connection_type: ConnectionType,
    session_map: Arc<DashMap<SessionId, Session>>,
    sock_descriptors: Arc<Mutex<HashMap<SocketAddr, bool>>>,
    #[getset(set)]
    receiver: Option<TioReceiver<String>>, // receive raw string msg from socket handling task
    #[getset(set)]
    app: Arc<A>,
}

impl<A: Application + Send + Sync + 'static> SocketAcceptor<A> {
    pub fn new(settings: Properties, app: A) -> Self {
        let session_map = create_sessions(&settings);
        let socket_desc = create_socket_descriptors(&settings);
        let connection_type: ConnectionType =
            settings.default_property(CONNECTION_TYPE_SETTING).unwrap();
        Self {
            settings,
            connection_type,
            session_map: Arc::new(dashmap::DashMap::from_iter(session_map)),
            sock_descriptors: Arc::new(Mutex::new(socket_desc)),
            receiver: None,
            // sender: None,
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

    fn set_session_responder(&mut self, session_id: &SessionId, msg_sender: TioSender<String>) {
        // let mut sguard = self.session_map().lock().unwrap();
        // sguard.get_mut(session_id).map(|session| session.set_responder(Some(msg_sender)));
        let mut session = self.session_map().get_mut(session_id).unwrap();
        session.set_responder(Some(Arc::new(msg_sender)));
    }

    pub fn initialize(&mut self) {
        let session_socket = create_socket_session(self.settings());
        let (raw_tx, raw_rx) = tio_channel::<String>(64);
        for (sock_addr, id_set) in session_socket {
            let (msg_tx, msg_rx) = tio_channel::<String>(16);
            for sid in id_set {
                self.set_session_responder(&sid, msg_tx.clone())
            }
            let tx = raw_tx.clone();
            let socket_descriptor = Arc::clone(self.sock_descriptors());
            start_acceptor_task(sock_addr, socket_descriptor, tx, msg_rx);
        }

        start_receiver_task(raw_rx, Arc::clone(self.app()), Arc::clone(self.session_map()));
    }
}

fn start_acceptor_task(
    sock_addr: SocketAddr, socket_descriptor: Arc<Mutex<HashMap<SocketAddr, bool>>>,
    tx: TioSender<String>, msg_rx: TioReceiver<String>,
) {
    tokio::spawn(async move {
        let listener = TcpListener::bind(sock_addr).await.unwrap();
        let local_addr = listener.local_addr().unwrap();
        socket_descriptor.lock().unwrap().insert(local_addr, true);
        println!("Port binding done");
        // let mut msg_rx = Arc::new(msg_rx);
        let (stream, _) = listener.accept().await.unwrap();
        println!("Accepted connection");
        let local_addr = stream.local_addr().unwrap();
        // let (owned_read_half, owned_write_half) = stream.into_split();
        // let responder = Arc::new(Mutex::new(owned_write_half));
        // connections.insert(local_addr, Arc::clone(&responder));
        handle_message_io(stream, &tx, msg_rx).await;
    });
}

fn start_receiver_task<A: Application + Send + Sync + 'static>(
    mut rx: TioReceiver<String>, app: Arc<A>, sessions: Arc<DashMap<SessionId, Session>>,
) {
    std::thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            println!("received: {}", s);
            let session_id: SessionId = Message::get_reverse_session_id(&s);
            {
                let dd = sessions
                    .get(&session_id)
                    .map(|sess| Arc::clone(sess.data_dictionary()))
                    .unwrap();
                if let Ok(message) = Message::from_str(&s, &dd) {
                    println!("msg parsed");
                    if let Ok(_) = Session::verify(&message, &sessions) {
                        app.from_app(&session_id, &sessions, message);
                    } else {
                        // Session::send(test_logon(), session_id.clone(), Arc::clone(&sessions));
                        Session::sync_send_to_target(&session_id, &sessions, test_logon());
                    }
                } else {
                    // Session::send(test_logon(), session_id.clone(), Arc::clone(&sessions));
                }
            }
            // app.from_app(s);
        }
    });
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

fn create_socket_session(settings: &Properties) -> HashMap<SocketAddr, HashSet<SessionId>> {
    let mut result_map = HashMap::new();
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
        result_map
            .entry(sock_address)
            .and_modify(|set: &mut HashSet<SessionId>| {
                set.insert(session_id.clone());
            })
            .or_insert_with(|| {
                let mut s = HashSet::new();
                s.insert(session_id.clone());
                s
            });
    }
    result_map
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

fn start_internal_msg_receiver_task(mut write_stream: OwnedWriteHalf, mut rx: TioReceiver<String>) {
    tokio::spawn(async move {
        println!("starting internal msg receiv");
        // if there is message to be sent out to remote socket then read and send
        while let Some(msg) = rx.recv().await {
            println!("sending {}", &msg);
            let _res = write_stream.write_all(msg.as_bytes()).await.unwrap();
            println!("sent {}", &msg);
        }
    });
}

async fn handle_message_io(stream: TcpStream, tx: &TioSender<String>, rx: TioReceiver<String>) {
    println!("handling connection");
    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    let (read_half, write_half) = stream.into_split();
    let mut buf_reader = BufReader::new(read_half);
    start_internal_msg_receiver_task(write_half, rx);

    loop {
        println!("reading msg");
        read_message(&mut buf_reader, &mut buf).await;
        // send message back to application
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
