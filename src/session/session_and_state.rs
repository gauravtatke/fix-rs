use crate::data_dictionary::DataDictionary;
use crate::message::*;
use crate::session::*;
use getset::Getters;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

#[derive(Debug, Default)]
struct SessionState;

impl SessionState {
    fn new() -> Self {
        SessionState
    }
}

#[derive(Debug, Default, Getters)]
pub struct Session {
    pub session_id: SessionId,
    heartbeat_intrvl: u32,
    is_active: bool,
    reset_on_logon: bool,
    reset_on_logout: bool,
    reset_on_disconnect: bool,
    msg_q: VecDeque<Message>,
    state: SessionState,
    responder: Option<Arc<Mutex<TcpStream>>>,
    #[getset(get = "pub")]
    data_dictionary: DataDictionary,
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
        let data_dict_path: String = session_setting
            .get_or_default(session_id, DATA_DICTIONARY_FILE_PATH)
            .unwrap_or_else(|| "resources/FIX43.xml".to_string());
        // .unwrap_or("resources/FIX43.xml");
        let data_dictionary = DataDictionary::from_xml(data_dict_path);
        Self {
            session_id: session_id.clone(),
            heartbeat_intrvl: heartbeat_interval,
            reset_on_disconnect,
            reset_on_logon,
            reset_on_logout,
            msg_q: VecDeque::new(),
            is_active: false,
            state: SessionState::default(),
            responder: None,
            data_dictionary,
        }
    }

    pub fn verify(&self, msg: &Message) -> Result<(), &'static str> {
        todo!()
    }

    pub fn send_to_target(&self, msg: &str) {
        todo!()
    }
}
