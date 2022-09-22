#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;
use crate::session::session_constants::*;
use core::panic;
use getset::Getters;
use std::collections::{hash_map, HashMap, VecDeque};
use std::default;
use std::hash::Hash;
use std::io::prelude::*;
use std::iter::{IntoIterator, Iterator};
use std::str::FromStr;
use std::sync::atomic::AtomicPtr;
use std::{fmt, fmt::Formatter, fs};
use toml::{de, value, Value};

use serde::Deserialize;

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
    pub const SOCKET_ACCEPT_PORT: &str = "socket_accept_port";
    pub const SOCKET_CONNECT_PORT: &str = "socket_connect_port";
    pub const SOCKET_CONNECT_HOST: &str = "socket_connect_host";
}

#[derive(Debug, Default)]
pub struct SessionSetting {
    default_session_id: SessionId,
    settings: HashMap<SessionId, toml::value::Table>,
}

impl SessionSetting {
    pub fn new<S: AsRef<std::path::Path>>(toml_path: S) -> Self {
        let toml_str = fs::read_to_string(toml_path).expect("could not read config toml");
        let settings = SessionSetting::from_str(&toml_str).expect("could not parse toml to Value");
        settings.validate();
        settings
    }

    fn validate(&self) {
        // panics if there is an error
        let default_values = self.get_default_settings();
        let conn_type = default_values
            .get(CONNECTION_TYPE_SETTING)
            .unwrap_or_else(|| panic!("`connection_type` not found"))
            .as_str()
            .unwrap();
        if conn_type != ACCEPTOR_CONN_TYPE && conn_type != INITIATOR_CONN_TYPE {
            panic!("invalid connection type. only `acceptor` or `initiator` supported");
        }
    }

    pub fn get_default_session_id(&self) -> &SessionId {
        &self.default_session_id
    }

    pub fn is_default_session_id(&self, session_id: &SessionId) -> bool {
        &self.default_session_id == session_id
    }

    pub fn get_default_settings(&self) -> &value::Table {
        self.get_session_settings(&self.default_session_id)
    }

    pub fn get_session_settings(&self, session_id: &SessionId) -> &value::Table {
        self.settings.get(session_id).unwrap()
    }

    fn get_setting(&self, session_id: &SessionId, setting_name: &str) -> Option<&Value> {
        self.settings
            .get(session_id)
            .and_then(|table| table.get(setting_name))
            .or_else(|| self.get_default_settings().get(setting_name))
    }

    pub fn get_setting_as_integer(&self, session_id: &SessionId, s_name: &str) -> Option<i64> {
        self.get_setting(session_id, s_name).and_then(|val| val.as_integer())
    }

    pub fn session_iter(&self) -> hash_map::Iter<SessionId, value::Table> {
        self.settings.iter()
    }
}

impl FromStr for SessionSetting {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toml = s.parse::<Value>()?;
        let mut settings: HashMap<SessionId, toml::value::Table> = HashMap::new();
        let default_values = toml
            .get(DEFAULT_SECTION_NAME)
            .and_then(|v| v.as_table())
            .unwrap_or_else(|| panic!("default section not found"));
        let default_session_id = SessionId::from_setting(default_values);
        settings.insert(default_session_id.clone(), default_values.clone());

        if let Some(val) = toml.get(SESSION_SECTION_NAME) {
            if let Some(val_arr) = val.as_array() {
                for each_table in val_arr.iter() {
                    let table = each_table.as_table().cloned().unwrap();
                    let mut merged_table = settings.get(&default_session_id).cloned().unwrap();
                    for (table_key, table_val) in table.into_iter() {
                        merged_table.insert(table_key, table_val);
                    }
                    let session_id = SessionId::from_setting(&merged_table);
                    settings.insert(session_id, merged_table);
                }
            }
        }
        Ok(SessionSetting {
            default_session_id,
            settings,
        })
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Getters, Clone)]
#[getset(get = "with_prefix")]
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

    fn from_setting(setting: &toml::value::Table) -> Self {
        let begin_string = setting.get(BEGIN_STRING_SETTING).and_then(|v| v.as_str()).unwrap_or("");
        let sender_compid =
            setting.get(SENDER_COMPID_SETTING).and_then(|v| v.as_str()).unwrap_or("");
        let sender_subid = setting.get(SENDER_SUBID_SETTING).and_then(|v| v.as_str()).unwrap_or("");
        let sender_locid =
            setting.get(SENDER_LOCATIONID_SETTING).and_then(|v| v.as_str()).unwrap_or("");
        let target_compid =
            setting.get(TARGET_COMPID_SETTING).and_then(|v| v.as_str()).unwrap_or("");
        let target_subid = setting.get(TARGET_SUBID_SETTING).and_then(|v| v.as_str()).unwrap_or("");
        let target_locid =
            setting.get(TARGET_LOCATIONID_SETTING).and_then(|v| v.as_str()).unwrap_or("");
        let session_qual =
            setting.get(SESSION_QUALIFIER_SETTING).and_then(|v| v.as_str()).unwrap_or("");

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

    pub fn with_settings(session_id: &SessionId, session_setting: &value::Table) -> Self {
        // setting should have begin_string, sender_compid and target_compid
        // it should also have either accept port or (connect_host, connect_port)
        let heartbeat_interval = session_setting
            .get("heartbeat_interval")
            .and_then(|val| val.as_integer())
            .unwrap_or(30i64);
        let reset_on_logon =
            session_setting.get("reset_on_logon").and_then(|val| val.as_bool()).unwrap_or(true);
        let reset_on_logout =
            session_setting.get("reset_on_logout").and_then(|val| val.as_bool()).unwrap_or(true);
        let reset_on_disconnect = session_setting
            .get("reset_on_disconnect")
            .and_then(|val| val.as_bool())
            .unwrap_or(true);
        Self {
            session_id: session_id.clone(),
            heartbeat_intrvl: heartbeat_interval as u32,
            reset_on_disconnect,
            reset_on_logon,
            reset_on_logout,
            msg_q: VecDeque::new(),
            is_active: false,
            state: SessionState::default(),
        }
    }

    // fn set_socket_connector(&mut self, conn: SocketConnector) {
    //     self.io_conn = Some(conn);
    // }

    // fn send_msg(&mut self, msg: Message) {
    //     if let Some(con) = self.io_conn.as_ref() {
    //         con.send(msg);
    //     } else {
    //         self.msg_q.push_back(msg);
    //     }
    // }

    // fn recv_msg(&self) -> Message {
    //     if let Some(con) = self.io_conn.as_ref() {
    //         con.recv()
    //     } else {
    //         Message::new()
    //     }
    // }
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

            [[Session]]
            sender_comp_id = "sender_1"
            target_comp_id = "target_1"

            [[Session]]
            sender_comp_id = "sender_order"
            target_comp_id = "target_order"
            session_qualifier = "order"
"#;

        let cargo_value = cargo_toml.parse::<SessionSetting>().unwrap();
        println!("{:#?}", cargo_value);
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
        let settings = cfg_toml.parse::<SessionSetting>().unwrap();
        settings.validate();
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
        let settings = cfg_toml.parse::<SessionSetting>().unwrap();
        settings.validate();
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
