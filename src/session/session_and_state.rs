use crate::data_dictionary::DataDictionary;
use crate::io::TioBroadcastSender;
use crate::message::*;
use crate::network::SessionMap;
use crate::session::*;
use getset::Getters;
use getset::Setters;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
struct SessionState;

impl SessionState {
    fn new() -> Self {
        SessionState
    }
}

#[derive(Debug, Default, Getters, Setters, Clone)]
pub struct Session {
    pub session_id: LegacySessionId,
    heartbeat_intrvl: u32,
    is_active: bool,
    reset_on_logon: bool,
    reset_on_logout: bool,
    reset_on_disconnect: bool,
    msg_q: VecDeque<Message>,
    state: SessionState,
    // session_map: Option<Arc<Mutex<HashMap<se>>>>,
    #[getset(set = "pub")]
    responder: Option<TioBroadcastSender<String>>,
    #[getset(get = "pub")]
    data_dictionary: Arc<DataDictionary>,
}

impl Session {
    fn set_session_id(&mut self, sid: LegacySessionId) {
        self.session_id = sid;
    }

    pub fn verify(msg: &Message, sessions: &SessionMap) -> Result<(), &'static str> {
        Ok(())
    }

    // pub fn sync_send(
    //     msg: Message, session_id: &SessionId, sessions: &Arc<DashMap<SessionId, Session>>,
    // ) {
    //     let session = sessions.get(session_id).unwrap();
    //     session.send_to_target(msg);
    // }

    // pub fn send_to_target(&self, msg: Message) {
    //     let responder = self.responder.as_ref().unwrap();
    //     responder.blocking_send(msg.to_string()).unwrap();
    // }

    // pub async fn async_send(session_id: &SessionId, msg: Message) {
    //     let session =
    // }
    pub fn sync_send_to_target(session_id: &LegacySessionId, sessions: &SessionMap, msg: Message) {
        // let synchronous_send = true;
        let sess_ref = sessions.get_session(session_id).unwrap();
        let responder = sess_ref.responder.as_ref().unwrap().clone();
        responder.send(msg.to_string()).unwrap();
        // if !synchronous_send {
        //     tokio::spawn(async move {
        //         responder.send(msg.to_string()).await.unwrap();
        //     });
        // } else {
        //     responder.send(msg.to_string());
        // }
    }
}
