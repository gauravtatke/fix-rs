#![allow(dead_code)]

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
pub const DATA_DICTIONARY_FILE_PATH: &str = "data_dictionary";


pub mod session_and_state;
pub mod session_id;
pub mod session_schedule;
pub mod session_settings;

pub use session_and_state::*;
pub use session_id::*;
pub use session_schedule::*;
pub use session_settings::*;
