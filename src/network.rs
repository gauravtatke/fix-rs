#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::str::{self, FromStr};
use std::thread;

use crate::application::Application;
use crate::data_dictionary::*;
use crate::message::store::*;

use regex::Regex;

use crate::message::*;
use crate::session::*;

const ACCEPTOR_CONN_TYPE: &str = "acceptor";
const INITIATOR_CONN_TYPE: &str = "initiator";

pub trait Connecter {
    fn start(&self) -> Vec<thread::JoinHandle<()>>;
    fn stop();
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConnectionType {
    ACCEPTOR,
    INITIATOR,
}

#[derive(Debug)]
pub struct SocketConnector {
    connection_type: ConnectionType,
    session_map: HashMap<String, Session>,
    sockets: HashSet<SocketAddrV4>,
}

impl Default for SocketConnector {
    fn default() -> Self {
        Self {
            connection_type: ConnectionType::ACCEPTOR,
            session_map: HashMap::new(),
            sockets: HashSet::new(),
        }
    }
}
impl SocketConnector {
    pub fn new<M: MessageStore, L: LogStore, A: Application>(
        config: &mut SessionConfig,
        msg_store: &mut M,
        log_store: &mut L,
        app: A,
    ) -> Self {
        let mut socket_connector = SocketConnector::default();
        socket_connector.create_sessions(config);
        socket_connector
    }

    pub fn send(&self, msg: Message) {
        println!("{}", msg);
    }

    pub fn recv(&self) -> Message {
        Message::new()
    }

    pub fn set_connection_type(&mut self, con_ty: String) {
        if con_ty.eq_ignore_ascii_case(ACCEPTOR_CONN_TYPE) {
            self.connection_type = ConnectionType::ACCEPTOR;
        } else if con_ty.eq_ignore_ascii_case(INITIATOR_CONN_TYPE) {
            self.connection_type = ConnectionType::INITIATOR;
        } else {
            panic!(format!(
                "Invalid connection type param. Only {} and {} are allowed",
                ACCEPTOR_CONN_TYPE, INITIATOR_CONN_TYPE
            ));
        }
    }

    pub fn get_connection_type(&self) -> ConnectionType {
        self.connection_type
    }

    pub fn set_session(&mut self, sid: String, session: Session) {
        self.session_map.insert(sid, session);
    }

    pub fn create_sessions(&mut self, config: &mut SessionConfig) {
        let default_setting = config.default_setting().clone();
        let conn_type = default_setting.get_connection_type();
        // if conn_type.is_none() {
        //     panic!("No connection type provided");
        // }
        // self.set_connection_type(conn_type.unwrap());
        self.set_connection_type(conn_type);
        let settings_vec: Vec<&mut SessionSetting> =
            config.iter_mut().filter(|s| !s.is_empty()).collect();
        for stng in settings_vec.into_iter() {
            println!("setting {:?}", stng);
            let merge_set = stng.merge_setting(&default_setting);
            let new_session = Session::with_settings(&merge_set);
            self.set_session(new_session.session_id.to_string(), new_session);
            let sock_addr: SocketAddrV4;
            if self.get_connection_type() == ConnectionType::ACCEPTOR {
                sock_addr = SocketAddrV4::new(
                    Ipv4Addr::LOCALHOST,
                    stng.get_socket_accept_port().expect("no port specified"),
                );
                self.sockets.insert(sock_addr);
            } else {
                let ipv4 = Ipv4Addr::from_str(
                    stng.get_socket_connect_host()
                        .expect("no host specified")
                        .as_ref(),
                )
                .expect("cannot parse host to Ipv4Addr");
                sock_addr = SocketAddrV4::new(
                    ipv4,
                    stng.get_socket_connect_port().expect("no port specified"),
                );
                self.sockets.insert(sock_addr);
            }
        }
    }
}

struct FixReader<B: BufRead> {
    buf_reader: B,
    aux_buf: Vec<u8>,
}

impl<B: BufRead> FixReader<B> {
    fn new(buf_read: B) -> Self {
        Self {
            buf_reader: buf_read,
            aux_buf: Vec::with_capacity(64),
        }
    }

    fn read_message(&mut self, buff: &mut String) -> std::io::Result<usize> {
        // regular expression for end of fix message
        lazy_static! {
            static ref EOM_RE: Regex =
                // Regex::new(format!("{}10=\\d{{{}}}{}", SOH, 3, SOH).as_str()).unwrap();
                Regex::new(format!("{}10=\\d+{}", SOH, SOH).as_str()).unwrap();
        }
        let bytes_used = {
            let data_bytes = match self.buf_reader.fill_buf() {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            let str_data = str::from_utf8(data_bytes).unwrap();
            println!("str_data {}", str_data);
            match EOM_RE.find(str_data) {
                Some(mat) => {
                    buff.push_str(&str_data[..mat.end()]);
                    mat.end()
                }
                None => 0,
            }
        };
        println!("bytes used {}", bytes_used);
        self.buf_reader.consume(bytes_used);
        Ok(bytes_used)
    }

    fn read_message_new(&mut self) -> std::io::Result<String> {
        // 8=FIX.4.4|9=5|35=0|10=10|
        let delim = &[SOH as u8];
        // let mut fix_ver: [u8; 10] = [0; 10]; // this will include '=' after tag 9
        let mut message = String::with_capacity(512);
        // this will fill 10 bytes atleast so fix version will be retrieved
        let ver_len = self.buf_reader.read_until(SOH as u8, &mut self.aux_buf)?;
        // println!("version {}", str::from_utf8(&self.aux_buf[..]).unwrap());
        if ver_len == 0 || !self.aux_buf.ends_with(delim) {
            // either no data or partial data without any SOH is reached
            // this can only happen if connection is closed with no data or partial data
            // println!("version not proper");
            return Err(Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "partial message",
            ));
        }
        let body_len = self.buf_reader.read_until(SOH as u8, &mut self.aux_buf)?;
        // println!("body len field {}", str::from_utf8(&self.aux_buf[ver_len+2..ver_len+body_len-1]).unwrap());
        if body_len == 0 || !self.aux_buf.ends_with(delim) {
            // println!("body len not proper");
            return Err(Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "partial message",
            ));
        }

        let mut body_len_bytes = 0u32;
        for byt in &self.aux_buf[ver_len + 2..ver_len + body_len - 1] {
            // parse bytes into an u16
            body_len_bytes = body_len_bytes * 10 + (*byt as char).to_digit(10).unwrap();
            // println!("curr len {}, prev byte {}, str rep {}", body_len_bytes, *byt, str::from_utf8(&[*byt]).unwrap());
        }
        // println!("calculated body len {}", body_len_bytes);

        // now read exact bytes from bufreader
        let new_len = ver_len + body_len + body_len_bytes as usize; // 7 bytes for trailer
        self.aux_buf
            .resize(self.aux_buf.len() + body_len_bytes as usize, 0u8);
        self.buf_reader
            .read_exact(&mut self.aux_buf[ver_len + body_len..new_len])?;
        let trailer = self.buf_reader.read_until(SOH as u8, &mut self.aux_buf)?;
        if trailer == 0 || !self.aux_buf.ends_with(delim) {
            // println!("trailer not correct");
            return Err(Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "partial message",
            ));
        }
        message.push_str(str::from_utf8(&self.aux_buf[..]).unwrap());
        self.aux_buf.clear();
        Ok(message)
    }
}

impl Connecter for SocketConnector {
    fn start(&self) -> Vec<thread::JoinHandle<()>> {
        let mut join_handles: Vec<thread::JoinHandle<()>> = Vec::new();
        for socket in &self.sockets {
            println!("Socket {}", socket);
            let listener = TcpListener::bind(socket).expect("could not bind to socket");
            let new_thread = thread::Builder::new()
                .name(format!("thread for socket {}", socket))
                .spawn(move || {
                    for stream in listener.incoming() {
                        let stream = stream.unwrap();
                        // let mut buff = String::with_capacity(512);
                        let mut fix_reader = FixReader::new(BufReader::new(stream));
                        loop {
                            // buff.clear();
                            match fix_reader.read_message_new() {
                                Ok(s) => {
                                    println!("message read {}", s);
                                    thread::sleep(std::time::Duration::from_millis(5000));
                                }
                                Err(_) => {
                                    println!("Connection terminated");
                                    break;
                                }
                            }
                        }
                    }
                })
                .unwrap();
            join_handles.push(new_thread);
        }
        join_handles
    }

    fn stop() {}
}

mod validator {
    use super::*;
    pub fn validate_tag(msg: &str) {
        // validate that tag is correct according to data_dictionary
        // and value is permissible
        // get the message type
        // then iterate over list of tags/value and verify that each
    }
}

#[cfg(test)]
mod networkio_tests {
    use super::*;
    use crate::application::*;
    use crate::message::store::*;
    use crate::message::*;
    use crate::session::*;
    use rand::prelude::*;
    use std::thread;
    use std::time::Duration;

    fn test_message() -> String {
        let mut msg = Message::new();
        // let mut rng = rand::thread_rng();
        msg.header_mut().set_string(49, "Gaurav".to_string());
        msg.header_mut().set_string(56, "Tatke".to_string());
        msg.header_mut().set_msg_type("A");

        msg.body_mut().set_int(34, rand::random::<u32>());
        msg.body_mut().set_float(44, rand::random::<f64>());
        msg.body_mut().set_bool(654, rand::random::<bool>());
        msg.body_mut().set_char(54, 'b');
        msg.body_mut().set_string(1, "BOX_AccId".to_string());

        let body_len = msg.to_string().len();
        msg.header_mut().set_int(9, body_len as u16);
        msg.header_mut().set_string(8, "FIX.4.3".to_string());
        msg.trailer_mut().set_int(10, rand::random::<u16>());
        msg.to_string()
    }

    #[test]
    fn io_test() {
        let mut session_config = SessionConfig::from_toml("src/FixConfig.toml");
        let mut log_store = DefaultLogStore::new();
        let mut msg_store = DefaultMessageStore::new();
        let app = DefaultApplication::new();
        let mut acceptor =
            SocketConnector::new(&mut session_config, &mut msg_store, &mut log_store, app);
        let mut stream1 = TcpStream::connect("127.0.0.1:10114").expect("could not connect");
        // let mut stream2 = TcpStream::connect("127.0.0.1:10115").expect("could not connect");
        for i in 0..5 {
            let msg = test_message();
            println!("sending message on {:?} : {}", stream1, msg);
            stream1.write(msg.as_bytes()).unwrap();
            // let msg = test_message();
            // println!("sending message on {:?} : {}", stream2, msg);
            // stream2.write(msg.as_bytes()).unwrap();
        }
    }

    #[test]
    fn test_broken_message() {
        let mut stream1: TcpStream =
            TcpStream::connect("127.0.0.1:10114").expect("could not connect");
        let msg = test_message();
        let (msg_part1, msg_part2) = msg.split_at(msg.len() / 2);
        println!("Sending part 1 = {}", msg_part1);
        stream1.write_all(msg_part1.as_bytes()).unwrap();
        thread::sleep(Duration::from_millis(10000));
        println!("Sending part 2 = {}", msg_part2);
        stream1.write_all(msg_part2.as_bytes()).unwrap();
        // stream1.write_all(b"8=");
        thread::sleep(Duration::from_millis(5000));
        // stream1.write_all(b"FIX.4.3");
    }
}
