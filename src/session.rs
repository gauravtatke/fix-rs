#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::VecDeque;
use std::fmt::{self, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::iter::{IntoIterator, Iterator};

use crate::message::*;
use crate::network::*;

use serde_derive::Deserialize;

const FIX42_BEGIN_STR: &str = "FIX.4.2";
const FIX43_BEGIN_STR: &str = "FIX.4.3";
const FIX44_BEGIN_STR: &str = "FIX.4.4";

const ACCEPTOR_CONN_TYPE: &str = "acceptor";
const INITIATOR_CONN_TYPE: &str = "initiator";
#[derive(Debug, Deserialize)]
pub struct SessionConfig {
    default: DefaultSetting,
    sessions: Option<Vec<SessionSetting>>,
}

impl SessionConfig {
    pub fn from_toml(config: &str) -> Self {
        let mut toml_file = File::open(config).expect("Could not find file");
        let mut contents = String::with_capacity(1024);
        toml_file
            .read_to_string(&mut contents)
            .expect("Could not read from file");
        let mut session_config: SessionConfig = toml::from_str(&contents).unwrap();
        if session_config.is_session_empty() {
            // initialize it with default
            session_config.sessions = None
        }
        session_config
    }

    pub fn default_setting(&self) -> &DefaultSetting {
        &self.default
    }

    //TODO: give correct implementation
    pub fn is_session_empty(&self) -> bool {
        if self.sessions.is_none() {
            return true;
        }
        for s in self.sessions.as_ref().unwrap().iter() {
            if !s.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn iter(&self) -> std::slice::Iter<SessionSetting> {
        self.sessions.as_ref().unwrap().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<SessionSetting> {
        self.sessions.as_mut().unwrap().iter_mut()
    }
}

impl IntoIterator for &SessionConfig {
    type Item = SessionSetting;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let new_session_option = self.sessions.clone();
        new_session_option
            .unwrap_or_else(|| {
                let mut temp_session_setting = SessionSetting::default();
                temp_session_setting.merge_setting(&self.default);
                vec![temp_session_setting]
            })
            .into_iter()
    }
}

// impl IntoIterator for &mut SessionConfig {
//     type Item = SessionSetting;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self)
// }

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultSetting {
    connection_type: String,
    begin_string: Option<String>,
    sender_compid: Option<String>,
    sender_subid: Option<String>,
    sender_locationid: Option<String>,
    target_compid: Option<String>,
    target_subid: Option<String>,
    target_locationid: Option<String>,
    on_behalf_of_compid: Option<String>,
    on_behalf_of_subid: Option<String>,
    on_behalf_of_locationid: Option<String>,
    deliver_to_compid: Option<String>,
    deliver_to_subid: Option<String>,
    deliver_to_locationid: Option<String>,
    socket_accept_port: Option<u16>,
    socket_connect_host: Option<String>,
    socket_connect_port: Option<u16>,
    hearbeat_interval: Option<u16>,
    reset_on_logon: Option<char>,
    reset_on_logout: Option<char>,
    reset_on_disconnect: Option<char>,
}

impl DefaultSetting {
    pub fn set_connection_type(&mut self, conn_type: String) {
        self.connection_type = conn_type;
    }

    pub fn get_connection_type(&self) -> String {
        self.connection_type.clone()
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct SessionSetting {
    begin_string: Option<String>,
    sender_compid: Option<String>,
    sender_subid: Option<String>,
    sender_locationid: Option<String>,
    target_compid: Option<String>,
    target_subid: Option<String>,
    target_locationid: Option<String>,
    on_behalf_of_compid: Option<String>,
    on_behalf_of_subid: Option<String>,
    on_behalf_of_locationid: Option<String>,
    deliver_to_compid: Option<String>,
    deliver_to_subid: Option<String>,
    deliver_to_locationid: Option<String>,
    socket_accept_port: Option<u16>,
    socket_connect_host: Option<String>,
    socket_connect_port: Option<u16>,
    heartbeat_interval: Option<u16>,
    reset_on_logon: Option<char>,
    reset_on_logout: Option<char>,
    reset_on_disconnect: Option<char>,
    session_qualifier: Option<String>,
}

impl SessionSetting {
    pub fn is_empty(&self) -> bool {
        // returns true if all variables are None,
        // false otherwise
        self.begin_string.is_none()
            && self.sender_compid.is_none()
            && self.target_compid.is_none()
            && self.socket_accept_port.is_none()
            && self.socket_connect_host.is_none()
            && self.socket_connect_port.is_none()
    }

    pub fn get_socket_connect_host(&self) -> Option<String> {
        self.socket_connect_host.clone()
    }

    pub fn get_socket_connect_port(&self) -> Option<u16> {
        self.socket_connect_port
    }

    pub fn get_socket_accept_port(&self) -> Option<u16> {
        self.socket_accept_port
    }

    pub fn set_begin_string(&mut self, bgn_str: String) {
        self.begin_string = Some(bgn_str);
    }

    pub fn set_sender_compid(&mut self, s_cmp_id: String) {
        self.sender_compid = Some(s_cmp_id);
    }

    pub fn set_target_compid(&mut self, t_cmp_id: String) {
        self.target_compid = Some(t_cmp_id);
    }

    pub fn set_socket_accept_port(&mut self, port: u16) {
        self.socket_accept_port = Some(port);
    }

    pub fn set_socket_connect_host(&mut self, host: String) {
        self.socket_connect_host = Some(host);
    }

    pub fn set_socket_connect_port(&mut self, port: u16) {
        self.socket_connect_port = Some(port);
    }

    pub fn merge_setting(&mut self, def: &DefaultSetting) -> &mut Self {
        // takes original setting and overwrites it with other one
        let conn_type = def.get_connection_type();
        // if self.connection_type.is_none() {
        //     self.set_connection_type(def.connection_type.clone().expect("connection type None"));
        // }

        if self.begin_string.is_none() {
            self.set_begin_string(def.begin_string.clone().expect("begin str None"));
        }

        if self.sender_compid.is_none() {
            self.set_sender_compid(def.sender_compid.clone().expect("sender compid None"));
        }

        if self.target_compid.is_none() {
            self.set_target_compid(def.target_compid.clone().expect("target compid None"));
        }

        if self.socket_accept_port.is_none() && conn_type.eq_ignore_ascii_case("acceptor") {
            self.set_socket_accept_port(def.socket_accept_port.expect("accept port None"));
        }

        if self.socket_connect_host.is_none() && conn_type.eq_ignore_ascii_case("initiator") {
            self.set_socket_connect_host(
                def.socket_connect_host.clone().expect("connect host None"),
            );
        }

        if self.socket_connect_port.is_none() && conn_type.eq_ignore_ascii_case("initiator") {
            self.set_socket_connect_port(def.socket_connect_port.unwrap());
        }

        self
    }
}

#[derive(Debug)]
pub struct SessionId {
    begin_string: String,
    sender_compid: String,
    sender_subid: Option<String>,
    sender_locationid: Option<String>,
    target_compid: String,
    target_subid: Option<String>,
    target_locationid: Option<String>,
    session_qualifier: Option<String>,
    id: String,
}

impl SessionId {
    fn new(
        bgn_str: &str,
        sender: &str,
        sender_sub: Option<String>,
        sender_loc: Option<String>,
        target: &str,
        target_sub: Option<String>,
        target_loc: Option<String>,
        s_qual: Option<String>,
    ) -> Self {
        // let mut id = String::with_capacity(16);
        // id.push_str(string: &str)
        let mut sid = SessionId {
            sender_compid: sender.to_owned(),
            target_compid: target.to_owned(),
            begin_string: bgn_str.to_owned(),
            sender_subid: sender_sub,
            sender_locationid: sender_loc,
            target_subid: target_sub,
            target_locationid: target_loc,
            session_qualifier: s_qual,
            id: String::new(),
        };

        sid.create_id();
        sid
    }

    fn create_id(&mut self) {
        let sid = self.to_string();
        self.id = sid;
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}->{}",
            self.begin_string, self.sender_compid, self.target_compid
        )
    }
}

#[derive(Debug)]
struct SessionState;

impl SessionState {
    fn new() -> Self {
        SessionState
    }
}

#[derive(Debug)]
pub struct Session {
    pub session_id: SessionId,
    heartbeat_intrvl: u32,
    is_active: bool,
    reset_on_logon: bool,
    reset_on_disconnect: bool,
    msg_q: VecDeque<Message>,
    state: SessionState,
    io_conn: Option<SocketConnector>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_id: SessionId::new("DEFAULT", "sender", None, None, "target", None, None, None),
            heartbeat_intrvl: 30,
            is_active: false,
            reset_on_disconnect: false,
            reset_on_logon: true,
            msg_q: VecDeque::with_capacity(16),
            state: SessionState::new(),
            io_conn: None,
        }
    }
}

impl Session {
    pub fn new() -> Self {
        Default::default()
    }

    fn set_session_id(&mut self, sid: SessionId) {
        self.session_id = sid;
    }

    pub fn with_settings(setting: &SessionSetting) -> Self {
        // setting should have begin_string, sender_compid and target_compid
        // it should also have either accept port or (connect_host, connect_port)
        let mut a_session = Session::new();
        let b_str = setting.begin_string.as_ref().unwrap();
        let sender = setting.sender_compid.as_ref().unwrap();
        let target = setting.target_compid.as_ref().unwrap();
        a_session.set_session_id(SessionId::new(
            b_str, sender, None, None, target, None, None, None,
        ));
        a_session
    }

    fn set_socket_connector(&mut self, conn: SocketConnector) {
        self.io_conn = Some(conn);
    }

    fn send_msg(&mut self, msg: Message) {
        if let Some(con) = self.io_conn.as_ref() {
            con.send(msg);
        } else {
            self.msg_q.push_back(msg);
        }
    }

    fn recv_msg(&self) -> Message {
        if let Some(con) = self.io_conn.as_ref() {
            con.recv()
        } else {
            Message::new()
        }
    }
}

#[cfg(test)]
mod session_tests {
    use super::*;

    #[test]
    fn session_config_test() {
        let session_config = SessionConfig::from_toml("src/FixConfig.toml");
        println!("{:?}", session_config);
    }
}
