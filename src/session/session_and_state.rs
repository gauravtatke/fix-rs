use crate::data_dictionary::DataDictionary;
use crate::fields::MaxMessageSize;
use crate::message::*;
use crate::session::*;
use getset::Getters;
use getset::Setters;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[derive(Debug, Default, Clone)]
struct SessionState;

impl SessionState {
    fn new() -> Self {
        SessionState
    }
}

#[derive(Debug, Default, Getters, Setters, Clone)]
pub struct Session {
    pub session_id: SessionId,
    heartbeat_intrvl: u32,
    is_active: bool,
    reset_on_logon: bool,
    reset_on_logout: bool,
    reset_on_disconnect: bool,
    msg_q: VecDeque<Message>,
    state: SessionState,
    // session_map: Option<Arc<Mutex<HashMap<se>>>>,
    #[getset(set = "pub")]
    responder: Option<mpsc::Sender<Message>>,
    #[getset(get = "pub")]
    data_dictionary: Arc<DataDictionary>,
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
            data_dictionary: Arc::new(data_dictionary),
        }
    }

    pub fn verify(
        msg: &Message, sessions: &Arc<Mutex<HashMap<SessionId, Session>>>,
    ) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn send(
        msg: Message, session_id: SessionId, sessions: Arc<Mutex<HashMap<SessionId, Session>>>,
    ) {
        use std::thread;
        let handle = thread::spawn(move || {
            println!("synchrnous spawned thread");
            let guard = sessions.lock().unwrap();
            let session = guard.get(&session_id).unwrap();
            session.send_to_target(msg);
        });
        handle.join().unwrap();
    }

    pub fn send_to_target(&self, msg: Message) {
        let responder = self.responder.as_ref().unwrap();
        responder.blocking_send(msg).unwrap();
    }
}
