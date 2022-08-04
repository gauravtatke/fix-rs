#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;
use getset::Getters;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::io::prelude::*;
use std::iter::{IntoIterator, Iterator};
use std::str::FromStr;
use std::sync::atomic::AtomicPtr;
use std::{fmt, fmt::Formatter, fs};
use toml::{de, value, Value};

use serde::Deserialize;

const FIX42_BEGIN_STR: &str = "FIX.4.2";
const FIX43_BEGIN_STR: &str = "FIX.4.3";
const FIX44_BEGIN_STR: &str = "FIX.4.4";

const ACCEPTOR_CONN_TYPE: &str = "acceptor";
const INITIATOR_CONN_TYPE: &str = "initiator";

const DEFAULT_SECTION_NAME: &str = "Default";
const SESSION_SECTION_NAME: &str = "Session";
const BEGIN_STRING_SETTING: &str = "begin_string";
const SENDER_COMPID_SETTING: &str = "sender_comp_id";
const SENDER_SUBID_SETTING: &str = "sender_sub_id";
const SENDER_LOCATIONID_SETTING: &str = "sender_location_id";
const TARGET_COMPID_SETTING: &str = "target_comp_id";
const TARGET_SUBID_SETTING: &str = "target_sub_id";
const TARGET_LOCATIONID_SETTING: &str = "target_location_id";
const SESSION_QUALIFIER_SETTING: &str = "session_qualifier";

#[derive(Debug, Default)]
pub struct SessionSetting {
    settings: HashMap<SessionId, toml::value::Table>,
}

impl SessionSetting {
    pub fn new(toml_path: &std::path::Path) -> Self {
        let toml_str = fs::read_to_string(toml_path).expect("could not read config toml");
        SessionSetting::from_str(&toml_str).expect("could not parse toml to Value")
    }
}

impl FromStr for SessionSetting {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toml = s.parse::<Value>()?;
        let mut settings: HashMap<SessionId, toml::value::Table> = HashMap::new();
        let default_values = toml.get(DEFAULT_SECTION_NAME).and_then(|v| v.as_table());
        let default_session_id = SessionId::new("DEFAULT", "", "", "", "", "", "", "");
        if let Some(table) = default_values {
            settings.insert(default_session_id.clone(), table.clone());
        } else {
            settings.insert(default_session_id.clone(), toml::value::Table::new());
        }

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
        Ok(SessionSetting { settings })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Getters, Clone)]
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
}
