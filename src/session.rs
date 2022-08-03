#![allow(dead_code)]
#![allow(unused_imports)]

use getset::Getters;
use std::collections::VecDeque;
use std::hash::Hash;
use std::io::prelude::*;
use std::iter::{IntoIterator, Iterator};
use std::str::FromStr;
use std::{fmt, fmt::Formatter, fs};

use crate::message::*;
// use crate::network::*;

use serde::Deserialize;

const FIX42_BEGIN_STR: &str = "FIX.4.2";
const FIX43_BEGIN_STR: &str = "FIX.4.3";
const FIX44_BEGIN_STR: &str = "FIX.4.4";

const ACCEPTOR_CONN_TYPE: &str = "acceptor";
const INITIATOR_CONN_TYPE: &str = "initiator";

#[derive(Debug, Deserialize, Clone, Getters)]
#[getset(get = "with_prefix")]
struct Setting {
    connection_type: Option<String>,
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
    session_qualifier: Option<String>,
}

impl FromStr for Setting {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

impl Setting {
    fn is_empty(&self) -> bool {
        self.connection_type.is_none()
            && self.begin_string.is_none()
            && self.sender_compid.is_none()
            && self.sender_subid.is_none()
            && self.sender_locationid.is_none()
            && self.target_compid.is_none()
            && self.target_locationid.is_none()
            && self.target_subid.is_none()
            && self.on_behalf_of_compid.is_none()
            && self.on_behalf_of_subid.is_none()
            && self.on_behalf_of_locationid.is_none()
            && self.deliver_to_compid.is_none()
            && self.deliver_to_subid.is_none()
            && self.deliver_to_locationid.is_none()
            && self.socket_accept_port.is_none()
            && self.socket_connect_host.is_none()
            && self.socket_connect_port.is_none()
            && self.hearbeat_interval.is_none()
            && self.reset_on_logon.is_none()
            && self.reset_on_disconnect.is_none()
            && self.reset_on_logout.is_none()
            && self.session_qualifier.is_none()
    }

    fn merge_with(&mut self, other: &Self) {
        // if value in self is not set but set in other, then other's value is updated in self
        // otherwise self values are retained
        if self.connection_type.is_none() {
            self.connection_type = other.connection_type.clone();
        }
        if self.begin_string.is_none() {
            self.begin_string = other.begin_string.clone();
        }
        if self.sender_compid.is_none() {
            self.sender_compid = other.sender_compid.clone();
        }
        if self.sender_locationid.is_none() {
            self.sender_locationid = other.sender_locationid.clone();
        }
        if self.sender_subid.is_none() {
            self.sender_subid = other.sender_subid.clone();
        }
        if self.target_compid.is_none() {
            self.target_compid = other.target_compid.clone();
        }
        if self.target_locationid.is_none() {
            self.target_locationid = other.target_locationid.clone();
        }
        if self.target_subid.is_none() {
            self.target_subid = other.target_subid.clone();
        }
        if self.on_behalf_of_compid.is_none() {
            self.on_behalf_of_compid = other.on_behalf_of_compid.clone();
        }
        if self.on_behalf_of_locationid.is_none() {
            self.on_behalf_of_locationid = other.on_behalf_of_locationid.clone();
        }
        if self.on_behalf_of_subid.is_none() {
            self.on_behalf_of_subid = other.on_behalf_of_subid.clone();
        }
        self.deliver_to_compid =
            self.deliver_to_compid.clone().or_else(|| other.deliver_to_compid.clone());
        self.deliver_to_locationid =
            self.deliver_to_locationid.clone().or_else(|| other.deliver_to_locationid.clone());
        self.deliver_to_subid =
            self.deliver_to_subid.clone().or_else(|| other.deliver_to_subid.clone());
        self.socket_accept_port = self.socket_accept_port.or(other.socket_accept_port);
        self.socket_connect_host =
            self.socket_connect_host.clone().or_else(|| other.socket_connect_host.clone());
        self.socket_connect_port = self.socket_connect_port.or(other.socket_connect_port);
        self.hearbeat_interval = self.hearbeat_interval.or(other.hearbeat_interval);
        self.reset_on_disconnect = self.reset_on_disconnect.or(other.reset_on_disconnect);
        self.reset_on_logon = self.reset_on_logon.or(other.reset_on_logon);
        self.reset_on_logout = self.reset_on_logout.or(other.reset_on_logout);
        self.session_qualifier =
            self.session_qualifier.clone().or_else(|| other.session_qualifier.clone());
    }
}

fn validate(setting: &Setting) {
    // validate the configuration
    if setting.connection_type.is_none()
        || setting.sender_compid.is_none()
        || setting.target_compid.is_none()
        || setting.begin_string.is_none()
    {
        panic!("ConnectionType, BeginString, SenderCompId or TargetCompId cannot be empty");
    }

    if let Some(ct) = setting.connection_type.as_ref() {
        if ct.eq_ignore_ascii_case(ACCEPTOR_CONN_TYPE) {
            if setting.socket_accept_port.is_none() {
                panic!("SocketAcceptPort is not specified for ConnectionType ACCEPTOR");
            }
        } else if ct.eq_ignore_ascii_case(INITIATOR_CONN_TYPE) {
            if setting.socket_connect_host.is_none() || setting.socket_connect_port.is_none() {
                panic!("Either SocketConnectHost or SocketConnectPost is not specified for connection type INITIATOR");
            }
        } else {
            panic!("Invalid connection type");
        }
    }

    if let Some(bs) = setting.begin_string.as_ref() {
        if !(bs.eq_ignore_ascii_case(FIX42_BEGIN_STR)
            || bs.eq_ignore_ascii_case(FIX43_BEGIN_STR)
            || bs.eq_ignore_ascii_case(FIX44_BEGIN_STR))
        {
            panic!("Invalid BeginString. Only FIX.4.2, FIX.4.3, FIX.4.4 are supported");
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SessionConfig {
    default: Setting,
    sessions: Option<Vec<Setting>>,
}

impl SessionConfig {
    pub fn from_toml(config_path: &str) -> Self {
        let contents = fs::read_to_string(config_path).expect("could not read from toml file");
        let mut config = SessionConfig::from_str(&contents).unwrap();
        let default_setting = config.default.clone();
        for cf in config.iter_mut() {
            cf.merge_with(&default_setting);
            validate(cf);
        }
        config
    }

    fn iter(&self) -> std::slice::Iter<Setting> {
        self.sessions.as_ref().unwrap().iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<Setting> {
        self.sessions.as_mut().unwrap().iter_mut()
    }
}

impl FromStr for SessionConfig {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Debug, PartialEq, Hash)]
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
    fn new(setting: &Setting) -> Self {
        let session_id = Self::session_id_from_setting(setting);
        Self {
            begin_string: setting.get_begin_string().clone().unwrap(),
            sender_compid: setting.get_sender_compid().clone().unwrap(),
            sender_subid: setting.get_sender_subid().clone(),
            sender_locationid: setting.get_sender_locationid().clone(),
            target_compid: setting.get_target_compid().clone().unwrap(),
            target_subid: setting.get_target_subid().clone(),
            target_locationid: setting.get_target_locationid().clone(),
            session_qualifier: setting.get_session_qualifier().clone(),
            id: session_id,
        }
    }

    fn session_id_from_setting(setting: &Setting) -> String {
        let begin_str = setting.get_begin_string().clone().unwrap();
        let sender_comp = setting.get_sender_compid().clone().unwrap();
        let mut session_id = format!("{}:{}", &begin_str, &sender_comp);
        if setting.get_sender_subid().is_some() {
            session_id.push('/');
            session_id.push_str(setting.get_sender_subid().as_ref().unwrap().as_str());
        }
        if setting.get_sender_locationid().is_some() {
            session_id.push('/');
            session_id.push_str(setting.get_sender_locationid().as_ref().unwrap());
        }
        session_id.push_str("->");
        session_id.push_str(setting.get_target_compid().clone().unwrap().as_str());
        if setting.get_target_subid().is_some() {
            session_id.push('/');
            session_id.push_str(setting.get_target_subid().as_ref().unwrap());
        }
        if setting.get_target_locationid().is_some() {
            session_id.push('/');
            session_id.push_str(setting.get_target_locationid().as_ref().unwrap());
        }
        if setting.get_session_qualifier().is_some() {
            session_id.push(':');
            session_id.push_str(setting.get_session_qualifier().as_ref().unwrap());
        }
        session_id
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug)]
struct SessionState;

impl SessionState {
    fn new() -> Self {
        SessionState
    }
}

// #[derive(Debug)]
// pub struct Session {
//     pub session_id: SessionId,
//     heartbeat_intrvl: u32,
//     is_active: bool,
//     reset_on_logon: bool,
//     reset_on_disconnect: bool,
//     msg_q: VecDeque<Message>,
//     state: SessionState,
//     io_conn: Option<SocketConnector>,
// }

// impl Default for Session {
//     fn default() -> Self {
//         Self {
//             session_id: SessionId::new("DEFAULT", "sender", None, None, "target", None, None, None),
//             heartbeat_intrvl: 30,
//             is_active: false,
//             reset_on_disconnect: false,
//             reset_on_logon: true,
//             msg_q: VecDeque::with_capacity(16),
//             state: SessionState::new(),
//             io_conn: None,
//         }
//     }
// }

// impl Session {
//     pub fn new() -> Self {
//         Default::default()
//     }

//     fn set_session_id(&mut self, sid: SessionId) {
//         self.session_id = sid;
//     }

//     pub fn with_settings(setting: &SessionSetting) -> Self {
//         // setting should have begin_string, sender_compid and target_compid
//         // it should also have either accept port or (connect_host, connect_port)
//         let mut a_session = Session::new();
//         let b_str = setting.begin_string.as_ref().unwrap();
//         let sender = setting.sender_compid.as_ref().unwrap();
//         let target = setting.target_compid.as_ref().unwrap();
//         a_session
//             .set_session_id(SessionId::new(b_str, sender, None, None, target, None, None, None));
//         a_session
//     }

//     fn set_socket_connector(&mut self, conn: SocketConnector) {
//         self.io_conn = Some(conn);
//     }

//     fn send_msg(&mut self, msg: Message) {
//         if let Some(con) = self.io_conn.as_ref() {
//             con.send(msg);
//         } else {
//             self.msg_q.push_back(msg);
//         }
//     }

//     fn recv_msg(&self) -> Message {
//         if let Some(con) = self.io_conn.as_ref() {
//             con.recv()
//         } else {
//             Message::new()
//         }
//     }
// }

#[cfg(test)]
mod session_tests {
    use super::*;

    #[test]
    fn session_config_test() {
        let session_config = SessionConfig::from_toml("src/FixConfig.toml");
        // println!("{:#?}", &session_config.sessions);

        let cargo_toml = toml::toml! {
            conn_type = "acceptor"
            [default]
            sender = "sender"
            target = "target"

            [[session]]
            sender = "sender_1"
            target = "target_1"

            [[session]]
            sender = "sender_order"
            target = "target_order"
            session_qualifier = "order"

        };

        println!("{:?}", cargo_toml);
        for (key, val) in cargo_toml.as_table().unwrap().iter() {
            println!("key: {:?}, val: {:?}", key, val);
        }
    }
}
