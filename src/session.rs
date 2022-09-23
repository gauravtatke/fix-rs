#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;
use crate::session::session_constants::*;
use core::panic;
use getset::Getters;
use serde::Deserialize;
use std::collections::{hash_map, HashMap, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::iter::{IntoIterator, Iterator, Peekable};
use std::path::Path;
use std::str::{FromStr, Lines};
use std::{default, hash};
use std::{fmt, fmt::Formatter, fs};

pub mod session_constants {
    // Begin Strings
    pub const FIX42_BEGIN_STR: &str = "FIX.4.2";
    pub const FIX43_BEGIN_STR: &str = "FIX.4.3";
    pub const FIX44_BEGIN_STR: &str = "FIX.4.4";

    // Connection types
    pub const ACCEPTOR_CONN_TYPE: &str = "acceptor";
    pub const INITIATOR_CONN_TYPE: &str = "initiator";

    // config toml's section name
    pub const DEFAULT_SECTION_NAME: &str = "Default";
    pub const SESSION_SECTION_NAME: &str = "Session";

    // settings name
    pub const BEGIN_STRING_SETTING: &str = "begin_string";
    pub const SENDER_COMPID_SETTING: &str = "sender_comp_id";
    pub const SENDER_SUBID_SETTING: &str = "sender_sub_id";
    pub const SENDER_LOCATIONID_SETTING: &str = "sender_location_id";
    pub const TARGET_COMPID_SETTING: &str = "target_comp_id";
    pub const TARGET_SUBID_SETTING: &str = "target_sub_id";
    pub const TARGET_LOCATIONID_SETTING: &str = "target_location_id";
    pub const SESSION_QUALIFIER_SETTING: &str = "session_qualifier";
    pub const CONNECTION_TYPE_SETTING: &str = "connection_type";
    pub const SOCKET_ACCEPT_PORT_SETTING: &str = "socket_accept_port";
    pub const SOCKET_CONNECT_PORT_SETTING: &str = "socket_connect_port";
    pub const SOCKET_CONNECT_HOST_SETTING: &str = "socket_connect_host";
    pub const RESET_ON_LOGON_SETTING: &str = "reset_on_logon";
    pub const RESET_ON_LOGOUT_SETTING: &str = "reset_on_logout";
    pub const RESET_ON_DISCONNECT_SETTING: &str = "reset_on_disconnect";
    pub const HEARTBEAT_INTERVAL_SETTING: &str = "heartbeat_interval";
}

#[derive(Debug)]
pub struct Properties {
    default_session_id: SessionId,
    session_settings: HashMap<SessionId, HashMap<String, String>>,
}

impl Properties {
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        let toml_str = fs::read_to_string(p).expect("unable to open the config file");
        Self::from_str(&toml_str)
    }

    pub fn get_or_default<F: FromStr>(&self, session_id: &SessionId, name: &str) -> Option<F> {
        let default_properties = self.session_settings.get(&self.default_session_id).unwrap();
        self.session_settings
            .get(session_id)
            .and_then(|props| props.get(name))
            .or_else(|| default_properties.get(name))
            .and_then(|val| val.as_str().parse::<F>().ok())
    }

    pub fn default_property<'a, F: FromStr>(&self, name: &str) -> Option<F> {
        self.get_or_default(&self.default_session_id, name)
    }

    pub fn from_str(s: &str) -> Self {
        let mut default_found = false;
        let mut lines = s.lines().peekable();
        let mut default_session_id: Option<SessionId> = None;
        let mut setting_map = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.starts_with('[') && line.ends_with(']') && line.contains(DEFAULT_SECTION_NAME) {
                if default_found {
                    // duplicate default section
                    panic!("duplicate default section found");
                }
                let default_section = parse_table(&mut lines);
                default_found = true;
                default_session_id = Some(SessionId::from_map(&default_section));
                setting_map.insert(default_session_id.clone().unwrap(), default_section);
            } else if line.starts_with('[') && line.ends_with(']') && default_found {
                // some other section in config file
                let section = parse_table(&mut lines);
                let session_id = SessionId::from_map(&section);
                setting_map.insert(session_id, section);
            }
        }
        if !default_found {
            panic!("default section not found");
        }
        let properties = Self {
            default_session_id: default_session_id.unwrap(),
            session_settings: setting_map,
        };
        properties.check();
        properties
    }

    pub fn session_ids(&self) -> Vec<&SessionId> {
        self.session_settings
            .keys()
            .filter(|&id| !id.eq(&self.default_session_id))
            .collect::<Vec<&SessionId>>()
    }

    fn check(&self) {
        let connection_type: String = match self.default_property(CONNECTION_TYPE_SETTING) {
            Some(s) => s,
            None => panic!("connection type not found"),
        };
        if connection_type != ACCEPTOR_CONN_TYPE && connection_type != INITIATOR_CONN_TYPE {
            panic!("invalid connection type");
        }
        for session_id in self.session_ids() {
            // verify ports
            if connection_type == ACCEPTOR_CONN_TYPE {
                if self.get_or_default::<u16>(session_id, SOCKET_ACCEPT_PORT_SETTING).is_none() {
                    panic!("acceptor port not found");
                }
            } else {
                if self.get_or_default::<String>(session_id, SOCKET_CONNECT_HOST_SETTING).is_none()
                    || self.get_or_default::<u16>(session_id, SOCKET_CONNECT_PORT_SETTING).is_none()
                {
                    panic!("socket connect host or port is missing");
                }
            }

            // verify begin string
            let begin_string = self
                .get_or_default::<String>(session_id, BEGIN_STRING_SETTING)
                .expect("begin string is missing");
            if begin_string != FIX42_BEGIN_STR
                && begin_string != FIX43_BEGIN_STR
                && begin_string != FIX44_BEGIN_STR
            {
                panic!("invalid begin string");
            }

            // verify comp_ids
            if self.get_or_default::<String>(session_id, SENDER_COMPID_SETTING).is_none()
                || self.get_or_default::<String>(session_id, TARGET_COMPID_SETTING).is_none()
            {
                panic!("sender and/or target compid missing");
            }
        }
    }
}

fn parse_table(lines: &mut Peekable<Lines>) -> HashMap<String, String> {
    // takes only the lines between 2 sections and creates a map out of it
    // let peekable_lines = lines.peekable();
    let mut properties = HashMap::new();
    while let Some(line) = lines.next_if(|&l| !l.trim().starts_with('[')) {
        let line = line.trim();
        if !line.is_empty() {
            let (prop_key, prop_val) = line
                .split_once('=')
                .and_then(|(key, val)| {
                    Some((
                        key.trim().trim_start_matches('"').trim_end_matches('"'),
                        val.trim().trim_start_matches('"').trim_end_matches('"'),
                    ))
                })
                .unwrap();
            properties.insert(prop_key.to_string(), prop_val.to_string());
        }
    }
    properties
}

#[derive(Debug, Default, PartialEq, Eq, Getters, Clone)]
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

impl Hash for SessionId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl SessionId {
    fn new(
        begin_str: &str, sender_comp: &str, sender_sub: &str, sender_loc: &str, target_comp: &str,
        target_sub: &str, target_loc: &str, session_qual: &str,
    ) -> Self {
        let get_optional_str = |s: &str| match s {
            "" => None,
            s => Some(s.to_string()),
        };

        let create_id = || {
            let mut session_id = String::new();
            session_id.push_str(begin_str);
            session_id.push(':');
            session_id.push_str(sender_comp);
            if !sender_sub.is_empty() {
                session_id.push('/');
                session_id.push_str(sender_sub);
            }
            if !sender_loc.is_empty() {
                session_id.push('/');
                session_id.push_str(sender_loc);
            }
            session_id.push_str("->");
            session_id.push_str(target_comp);
            if !target_sub.is_empty() {
                session_id.push('/');
                session_id.push_str(target_sub);
            }
            if !target_loc.is_empty() {
                session_id.push('/');
                session_id.push_str(target_loc);
            }
            session_id
        };

        Self {
            begin_string: begin_str.to_string(),
            sender_compid: sender_comp.to_string(),
            sender_subid: get_optional_str(sender_sub),
            sender_locationid: get_optional_str(sender_loc),
            target_compid: target_comp.to_string(),
            target_subid: get_optional_str(target_sub),
            target_locationid: get_optional_str(target_loc),
            session_qualifier: get_optional_str(session_qual),
            id: create_id(),
        }
    }

    fn from_map(prop_map: &HashMap<String, String>) -> Self {
        let begin_string = prop_map.get(BEGIN_STRING_SETTING).map(|v| v.as_str()).unwrap_or("");
        let sender_compid = prop_map.get(SENDER_COMPID_SETTING).map(|v| v.as_str()).unwrap_or("");
        let sender_subid = prop_map.get(SENDER_SUBID_SETTING).map(|v| v.as_str()).unwrap_or("");
        let sender_locid =
            prop_map.get(SENDER_LOCATIONID_SETTING).map(|v| v.as_str()).unwrap_or("");
        let target_compid = prop_map.get(TARGET_COMPID_SETTING).map(|v| v.as_str()).unwrap_or("");
        let target_subid = prop_map.get(TARGET_SUBID_SETTING).map(|v| v.as_str()).unwrap_or("");
        let target_locid =
            prop_map.get(TARGET_LOCATIONID_SETTING).map(|v| v.as_str()).unwrap_or("");
        let session_qual =
            prop_map.get(SESSION_QUALIFIER_SETTING).map(|v| v.as_str()).unwrap_or("");

        Self::new(
            begin_string,
            sender_compid,
            sender_subid,
            sender_locid,
            target_compid,
            target_subid,
            target_locid,
            session_qual,
        )
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Default)]
struct SessionState;

impl SessionState {
    fn new() -> Self {
        SessionState
    }
}

#[derive(Debug, Default)]
pub struct Session {
    pub session_id: SessionId,
    heartbeat_intrvl: u32,
    is_active: bool,
    reset_on_logon: bool,
    reset_on_logout: bool,
    reset_on_disconnect: bool,
    msg_q: VecDeque<Message>,
    state: SessionState,
    // io_conn: Option<SocketConnector>,
}

impl Session {
    fn set_session_id(&mut self, sid: SessionId) {
        self.session_id = sid;
    }

    pub fn with_settings(session_id: &SessionId, session_setting: &Properties) -> Self {
        // setting should have begin_string, sender_compid and target_compid
        // it should also have either accept port or (connect_host, connect_port)
        let heartbeat_interval: u32 =
            session_setting.get_or_default(session_id, HEARTBEAT_INTERVAL_SETTING).unwrap_or(30);
        let reset_on_logon: bool =
            session_setting.get_or_default(session_id, RESET_ON_LOGON_SETTING).unwrap_or(true);
        let reset_on_logout: bool =
            session_setting.get_or_default(session_id, RESET_ON_LOGOUT_SETTING).unwrap_or(true);
        let reset_on_disconnect: bool =
            session_setting.get_or_default(session_id, RESET_ON_DISCONNECT_SETTING).unwrap_or(true);
        Self {
            session_id: session_id.clone(),
            heartbeat_intrvl: heartbeat_interval,
            reset_on_disconnect,
            reset_on_logon,
            reset_on_logout,
            msg_q: VecDeque::new(),
            is_active: false,
            state: SessionState::default(),
        }
    }
}

#[cfg(test)]
mod session_setting_tests {
    use super::*;

    #[test]
    fn session_sample_config_test() {
        // println!("{:#?}", &session_config.sessions);

        let cargo_toml = r#" 
            [Default]
            connection_type = "acceptor"
            sender_comp_id = "sender"
            target_comp_id= "target"
            socket_accept_port = 10117

            [Session]
            sender_comp_id = "sender_1"
            target_comp_id = "target_1"

            [Session]
            sender_comp_id = "sender_order"
            target_comp_id = "target_order"
            session_qualifier = "order"
"#;

        let properties = Properties::from_str(cargo_toml);
        println!("{:#?}", properties);
        let accept_port = properties
            .get_or_default::<u16>(&properties.default_session_id, SOCKET_ACCEPT_PORT_SETTING);
        println!("{:?}", accept_port);
        // let cargo_value = cargo_toml.parse::<SessionSetting>().unwrap();
        // println!("{:#?}", cargo_value);
        // for (key, val) in cargo_value.as_table().unwrap().iter() {
        //     println!("key: {:?}, val: {:#?}", key, val);
        // }
    }

    #[test]
    #[should_panic(expected = "default section not found")]
    fn test_no_default_section() {
        let cfg_toml = r#"
            [[Session]]
            sender_comp_id = "sender"
            target_comp_id = "target"
        "#;
        let settings = Properties::from_str(cfg_toml);
    }

    #[test]
    #[should_panic(expected = "`connection_type` not found")]
    fn test_default_no_connection_type() {
        let cfg_toml = r#"
            [Default]
            sender_comp_id = "sender"
            target_comp_id = "target"

            [[Session]]
            sender_comp_id = "sender"
            target_comp_id = "target"
        "#;
        let settings = Properties::from_str(cfg_toml);
    }

    #[test]
    fn test_no_mandatory_fields() {
        // no begin_string, no sender_compid, no target_compid
    }

    #[test]
    fn test_only_default_settings() {}

    #[test]
    fn test_default_and_session_overrides() {}
}
