#![allow(dead_code)]

pub mod id;
pub mod schedule;
pub mod settings;
pub(crate) mod state;

pub use id::*;
pub use settings::*;

use crate::application::Application;
use crate::data_dictionary::DataDictionary;
use crate::message::{Message, StringField};
use crate::quickfix_errors::SessionError;
use crate::session::schedule::SessionSchedule;
use log::{info, warn};
use state::SessionState;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

pub struct Session {
    id: SessionId,
    is_active: bool,
    // Config flags that control *when* Session calls state.reset() —
    // kept here (not in SessionState) because they're policy decisions,
    // not mutable runtime state.
    reset_on_logon: bool,
    reset_on_logout: bool,
    reset_on_disconnect: bool,
    state: SessionState,
    schedule: SessionSchedule,
    responder: Option<Box<dyn Responder>>,
    data_dict: Arc<DataDictionary>,
    app: Box<dyn Application>,
}

fn is_admin_msg_type(msg_type: &str) -> bool {
    matches!(msg_type, "0" | "1" | "2" | "3" | "4" | "5" | "A")
}

impl Session {
    pub(crate) fn new(
        id: SessionId,
        state: SessionState,
        schedule: SessionSchedule,
        responder: Option<Box<dyn Responder>>,
        dictionary: DataDictionary,
        app: Box<dyn Application>,
    ) -> Self {
        Self {
            id,
            is_active: false,
            reset_on_logon: false,
            reset_on_logout: false,
            reset_on_disconnect: false,
            state,
            schedule,
            responder,
            data_dict: Arc::new(dictionary),
            app,
        }
    }

    pub fn set_responder(&mut self, responder: Box<dyn Responder>) {
        self.responder = Some(responder);
    }

    pub fn dictionary(&self) -> &DataDictionary {
        &self.data_dict
    }

    // Tears down the connection and resets session to pre-logon state.
    // Called when the reader thread detects EOF/error (peer disconnected).
    pub fn disconnect(&mut self) {
        if let Some(ref responder) = self.responder {
            responder.disconnect();
        }
        self.responder = None;
        self.state.logon_sent = false;
        self.state.logon_received = false;
        self.state.logout_sent = false;
        self.state.logout_received = false;
        info!("{} event: Disconnected", self.id);
        self.app.on_logout(&self.id);
        // Seq num reset goes last — flags and on_logout must fire regardless.
        if self.reset_on_disconnect {
            self.state.reset(Instant::now());
        }
    }

    // Before logon, only Logon messages are valid. During logout (sent but
    // not received), only Logout and SequenceReset are accepted.
    fn valid_logon_state(&self, msg_type: &str) -> bool {
        if !self.state.logon_received {
            return matches!(msg_type, "A");
        }
        if self.state.logout_sent && !self.state.logout_received {
            return matches!(msg_type, "5" | "4");
        }
        true
    }

    // Routes a verified inbound message to the appropriate Application callback.
    // Admin messages (MsgType 0-5, A) → from_admin; all others → from_app.
    fn dispatch_to_app(&mut self, msg_type: &str, msg: &Message) -> Result<(), Box<dyn Error>> {
        if is_admin_msg_type(msg_type) {
            self.app.from_admin(&self.id, msg)?;
        } else {
            self.app.from_app(&self.id, msg)?;
        }
        Ok(())
    }

    // Validates BeginString, logon state, and CompID match.
    // Does not check sequence numbers or dispatch to Application callbacks.
    fn verify_msg(&mut self, msg: &Message) -> Result<(), Box<dyn Error>> {
        let begin_string = msg.header().get_field::<String>(8)?;
        if self.id.begin_string() != &begin_string {
            return Err(Box::from(SessionError::BeginStringMismatch {
                expected: self.id.begin_string().clone(),
                received: begin_string,
            }));
        }
        let msg_type = msg.get_msg_type()?;
        self.state.last_received_time = Instant::now();
        self.state.test_request_counter = 0;
        if !self.valid_logon_state(msg_type.as_str()) {
            return Err(Box::from(SessionError::InvalidStateForMsgType {
                msg_type: msg_type.clone(),
            }));
        }
        let sender_compid = msg.header().get_field::<String>(49)?;
        let target_compid = msg.header().get_field::<String>(56)?;
        if !self.id.sender_comp_id().eq(&target_compid)
            || !self.id.target_comp_id().eq(&sender_compid)
        {
            return Err(Box::from(SessionError::CompIdMismatch {
                expected_sender: self.id.sender_comp_id().clone(),
                expected_target: self.id.target_comp_id().clone(),
            }));
        }
        Ok(())
    }

    // Stamps standard header fields (BeginString, CompIDs, MsgSeqNum, SendingTime)
    // on any outbound message. Shared by admin builders and the outbound app path.
    fn initialize_header(&mut self, msg: &mut Message) {
        msg.header_mut().set_field(StringField::new(8, self.id.begin_string()));
        msg.header_mut().set_field(StringField::new(49, self.id.sender_comp_id()));
        msg.header_mut().set_field(StringField::new(56, self.id.target_comp_id()));
        if let Some(sender_subid) = self.id.sender_sub_id() {
            msg.header_mut().set_field(StringField::new(50, sender_subid));
        }
        if let Some(sender_locid) = self.id.sender_location_id() {
            msg.header_mut().set_field(StringField::new(142, sender_locid));
        }
        if let Some(target_subid) = self.id.target_sub_id() {
            msg.header_mut().set_field(StringField::new(57, target_subid));
        }
        if let Some(target_locid) = self.id.target_location_id() {
            msg.header_mut().set_field(StringField::new(143, target_locid));
        }
        msg.set_sending_time();
        msg.header_mut()
            .set_field(StringField::new(34, &self.state.next_sender_msg_seq_num.to_string()));
    }

    // Finalizes (body length, checksum) and pushes the message through the Responder.
    fn send_raw(&mut self, msg: &mut Message) {
        msg.set_body_len();
        msg.set_checksum();
        let wire = msg.to_string();
        info!("{} outgoing: {}", self.id, wire.replace('\x01', "|"));
        self.responder.as_ref().unwrap().send(&wire);
        self.state.incr_next_sender_msg_seq_num();
        self.state.last_sent_time = Instant::now();
    }

    // Builds and sends a Logon (MsgType=A) with HeartBtInt.
    fn generate_logon(&mut self) {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "A"));
        msg.set_field(StringField::new(98, "0"));
        msg.set_field(StringField::new(108, &self.state.heartbeat_interval.to_string()));
        self.initialize_header(&mut msg);
        self.app.to_admin(&self.id, &mut msg);
        self.send_raw(&mut msg);
    }

    // Builds and sends a Logout (MsgType=5) with optional Text reason.
    fn generate_logout(&mut self, reason: Option<&str>) {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "5"));
        self.initialize_header(&mut msg);
        if let Some(reason) = reason {
            msg.set_field(StringField::new(58, reason));
        }
        self.app.to_admin(&self.id, &mut msg);
        self.send_raw(&mut msg);
    }

    // Builds and sends a Heartbeat (MsgType=0). Echoes TestReqID if responding to a TestRequest.
    fn generate_heartbeat(&mut self, test_req_id: Option<&str>) {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "0"));
        if let Some(test_reqid) = test_req_id {
            msg.set_field(StringField::new(112, test_reqid));
        }
        self.initialize_header(&mut msg);
        self.app.to_admin(&self.id, &mut msg);
        self.send_raw(&mut msg);
    }

    // Builds and sends a TestRequest (MsgType=1) with the given TestReqID.
    fn generate_test_request(&mut self, req_id: &str) {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "1"));
        msg.set_field(StringField::new(112, req_id));
        self.initialize_header(&mut msg);
        self.app.to_admin(&self.id, &mut msg);
        self.send_raw(&mut msg);
    }

    // Inbound dispatch: extracts MsgType, routes to per-type handler.
    // App-level messages fall through to verify → seq check → dispatch_to_app.
    pub(crate) fn next_message(&mut self, msg: &mut Message) -> Result<(), Box<dyn Error>> {
        let msg_type = msg.get_msg_type()?;
        match msg_type.as_str() {
            "A" => self.next_logon(msg),
            "5" => self.next_logout(msg),
            "1" => self.next_test_request(msg),
            "0" => self.next_heartbeat(msg),
            _ => {
                self.verify_msg(msg)?;
                self.verify_seq_number(msg)?;
                self.dispatch_to_app(&msg_type, msg)?;
                Ok(())
            }
        }
    }

    // Checks MsgSeqNum: rejects if too low, warns if too high, increments expected.
    fn verify_seq_number(&mut self, msg: &Message) -> Result<(), Box<dyn Error>> {
        let incoming_seq_num = msg.header().get_field::<u32>(34)?;
        if incoming_seq_num > self.state.next_target_msg_seq_num {
            warn!(
                "message seq num {} exceeds expected {}",
                incoming_seq_num, self.state.next_target_msg_seq_num
            );
        }
        if incoming_seq_num < self.state.next_target_msg_seq_num {
            return Err(Box::from(SessionError::SeqNumTooLow {
                received: incoming_seq_num,
                expected: self.state.next_target_msg_seq_num,
            }));
        }
        self.state.incr_next_target_msg_seq_num();
        Ok(())
    }

    // Handles inbound Logon (MsgType=A). Guards: must be active and in session time.
    // Resets seq nums if ResetSeqNumFlag=Y, verifies, notifies app, and
    // acceptors respond with their own Logon.
    fn next_logon(&mut self, msg: &mut Message) -> Result<(), Box<dyn Error>> {
        if !self.is_active {
            return Err(Box::from(SessionError::InvalidStateForMsgType {
                msg_type: "A".to_string(),
            }));
        }
        if !self.schedule.is_session_time() {
            return Err(Box::from(SessionError::OutOfSessionTime));
        }

        // Config-level reset — must precede verify_seq_number so seq 1 is accepted.
        if self.reset_on_logon {
            self.state.reset(Instant::now());
        }

        // Wire-level reset (tag 141=Y) — counterparty explicitly requests it.
        if let Ok(reset_seq_flag) = msg.get_field::<String>(141)
            && reset_seq_flag == "Y"
        {
            self.state.reset(Instant::now());
        }
        self.verify_msg(msg)?;
        self.verify_seq_number(msg)?;
        self.app.from_admin(&self.id, msg)?;
        self.state.logon_received = true;
        info!("{} event: Logon received", self.id);
        if !self.state.is_initiator {
            self.state.logon_sent = true;
            self.generate_logon();
        }
        self.app.on_logon(&self.id);
        Ok(())
    }

    // Handles inbound Heartbeat (MsgType=0). Verify, seq check, notify app. No response.
    fn next_heartbeat(&mut self, msg: &mut Message) -> Result<(), Box<dyn Error>> {
        self.verify_msg(msg)?;
        self.verify_seq_number(msg)?;
        self.app.from_admin(&self.id, msg)?;
        Ok(())
    }

    // Handles inbound TestRequest (MsgType=1). Responds with Heartbeat echoing TestReqID.
    fn next_test_request(&mut self, msg: &mut Message) -> Result<(), Box<dyn Error>> {
        self.verify_msg(msg)?;
        self.verify_seq_number(msg)?;
        self.app.from_admin(&self.id, msg)?;
        let test_req_id = msg.get_field::<String>(112).ok();
        self.generate_heartbeat(test_req_id.as_deref());
        Ok(())
    }

    // Handles inbound Logout (MsgType=5). If we didn't initiate the logout,
    // responds with our own Logout. Always disconnects after notifying app.
    fn next_logout(&mut self, msg: &mut Message) -> Result<(), Box<dyn Error>> {
        self.verify_msg(msg)?;
        self.app.from_admin(&self.id, msg)?;
        info!("{} event: Logout received", self.id);
        if !self.state.logout_sent {
            self.generate_logout(None);
        }
        self.disconnect();
        // After disconnect so the logout response uses the correct seq num.
        if self.reset_on_logout {
            self.state.reset(Instant::now());
        }
        Ok(())
    }

    // Timer-driven session lifecycle, called periodically by the event loop.
    // Three phases:
    //   1. Pre-logon: initiator sends Logon if in session time, disconnects on timeout
    //   2. Post-logon, outside schedule: initiates graceful Logout
    //   3. Post-logon, in schedule: heartbeat escalation (heartbeat → test request → disconnect)
    pub(crate) fn next_tick(&mut self) -> Result<(), Box<dyn Error>> {
        let now = Instant::now();
        if self.responder.is_none() || !self.is_active {
            return Ok(());
        }

        // not logged on — initiator tries to connect, acceptor waits
        if !self.state.logon_received {
            if self.schedule.is_session_time() && self.state.is_initiator && !self.state.logon_sent
            {
                self.generate_logon();
                self.state.logon_sent = true;
            } else if self.state.is_logon_timed_out(now) {
                self.disconnect();
            }
            return Ok(());
        }

        // logged on but session time ended — initiate graceful logout
        if !self.schedule.is_session_time() {
            if !self.state.logout_sent {
                self.generate_logout(None);
                self.state.logout_sent = true;
            } else if self.state.is_logout_timed_out(now) {
                self.disconnect();
            }
            return Ok(());
        }

        // logged on, in session time — heartbeat management
        if self.state.is_timed_out(now) {
            self.disconnect();
        } else if self.state.is_test_request_needed(now) {
            self.generate_test_request("test");
            self.state.test_request_counter += 1;
        } else if self.state.is_heartbeat_needed(now) {
            self.generate_heartbeat(None);
            self.state.last_sent_time = now;
        }

        Ok(())
    }
}

pub trait Responder: Send + Sync {
    fn send(&self, msg: &str) -> bool;
    fn disconnect(&self);
}

#[cfg(test)]
mod responder_tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    struct MockResponder {
        sent_messages: Mutex<VecDeque<String>>,
        disconnected: Mutex<bool>,
    }

    impl MockResponder {
        fn new() -> Self {
            Self {
                sent_messages: Mutex::new(VecDeque::new()),
                disconnected: Mutex::new(false),
            }
        }

        fn is_disconnected(&self) -> bool {
            *self.disconnected.lock().unwrap()
        }
    }

    impl Responder for MockResponder {
        fn send(&self, msg: &str) -> bool {
            self.sent_messages.lock().unwrap().push_back(msg.to_string());
            true
        }

        fn disconnect(&self) {
            *self.disconnected.lock().unwrap() = true;
        }
    }

    #[test]
    fn test_send_captures_messages_in_order() {
        let mock = MockResponder::new();
        assert!(mock.send("8=FIX.4.3\x0135=A\x01"));
        assert!(mock.send("8=FIX.4.3\x0135=0\x01"));

        let sent = mock.sent_messages.lock().unwrap();
        assert_eq!(sent.len(), 2);
        assert_eq!(sent[0], "8=FIX.4.3\x0135=A\x01");
        assert_eq!(sent[1], "8=FIX.4.3\x0135=0\x01");
    }

    #[test]
    fn test_disconnect_sets_disconnected_flag() {
        let mock = MockResponder::new();
        assert!(!mock.is_disconnected());

        mock.disconnect();
        assert!(mock.is_disconnected());
    }

    fn _assert_send<T: Send>() {}

    #[test]
    fn test_session_is_send() {
        _assert_send::<Session>();
    }
}

#[cfg(test)]
mod session_tests {
    use super::*;
    use crate::application::Application;
    use crate::quickfix_errors::{AppError, DonotSend, RejectLogon};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::time::Duration;

    // Shared state survives after Session drops the responder on disconnect,
    // so tests can still inspect sent messages and disconnect status.
    #[derive(Clone)]
    struct MockState {
        sent_messages: Arc<Mutex<VecDeque<String>>>,
        disconnected: Arc<Mutex<bool>>,
    }

    impl MockState {
        fn new() -> Self {
            Self {
                sent_messages: Arc::new(Mutex::new(VecDeque::new())),
                disconnected: Arc::new(Mutex::new(false)),
            }
        }

        fn sent(&self) -> VecDeque<String> {
            self.sent_messages.lock().unwrap().clone()
        }

        fn is_disconnected(&self) -> bool {
            *self.disconnected.lock().unwrap()
        }
    }

    struct MockResponder {
        state: MockState,
    }

    impl MockResponder {
        fn new(state: MockState) -> Self {
            Self { state }
        }
    }

    impl Responder for MockResponder {
        fn send(&self, msg: &str) -> bool {
            self.state.sent_messages.lock().unwrap().push_back(msg.to_string());
            true
        }

        fn disconnect(&self) {
            *self.state.disconnected.lock().unwrap() = true;
        }
    }

    struct TestApplication {
        calls: RefCell<Vec<String>>,
    }

    impl TestApplication {
        fn new() -> Self {
            Self {
                calls: RefCell::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<String> {
            self.calls.borrow().clone()
        }
    }

    impl Application for TestApplication {
        fn on_create(&mut self, _session_id: &SessionId) {
            self.calls.borrow_mut().push("on_create".to_string());
        }
        fn on_logon(&mut self, _session_id: &SessionId) {
            self.calls.borrow_mut().push("on_logon".to_string());
        }
        fn on_logout(&mut self, _session_id: &SessionId) {
            self.calls.borrow_mut().push("on_logout".to_string());
        }
        fn to_admin(&mut self, _session_id: &SessionId, _message: &mut Message) {
            self.calls.borrow_mut().push("to_admin".to_string());
        }
        fn from_admin(
            &mut self,
            _session_id: &SessionId,
            _message: &Message,
        ) -> Result<(), RejectLogon> {
            self.calls.borrow_mut().push("from_admin".to_string());
            Ok(())
        }
        fn to_app(
            &mut self,
            _session_id: &SessionId,
            _message: &mut Message,
        ) -> Result<(), DonotSend> {
            self.calls.borrow_mut().push("to_app".to_string());
            Ok(())
        }
        fn from_app(
            &mut self,
            _session_id: &SessionId,
            _message: &Message,
        ) -> Result<(), AppError> {
            self.calls.borrow_mut().push("from_app".to_string());
            Ok(())
        }
    }

    fn make_session(app: Box<dyn Application>) -> (Session, MockState) {
        let id = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let state = SessionState::new(30, false, Instant::now());
        let schedule = SessionSchedule::new(None, None, None, None, chrono_tz::UTC);
        let mock_state = MockState::new();
        let responder = MockResponder::new(mock_state.clone());
        let dd = DataDictionary::default();
        (Session::new(id, state, schedule, Some(Box::new(responder)), dd, app), mock_state)
    }

    #[test]
    fn test_dispatch_to_app_dispatches_admin_to_from_admin() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        let msg = Message::new();

        let app = session.app.as_ref() as *const dyn Application as *const TestApplication;

        for msg_type in &["0", "1", "2", "3", "4", "5", "A"] {
            session.dispatch_to_app(msg_type, &msg).unwrap();
        }

        let calls = unsafe { &*app }.calls();
        assert_eq!(calls.len(), 7);
        assert!(calls.iter().all(|c| c == "from_admin"));
    }

    #[test]
    fn test_dispatch_to_app_dispatches_app_to_from_app() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        let msg = Message::new();

        let app = session.app.as_ref() as *const dyn Application as *const TestApplication;

        for msg_type in &["D", "8", "G", "AE"] {
            session.dispatch_to_app(msg_type, &msg).unwrap();
        }

        let calls = unsafe { &*app }.calls();
        assert_eq!(calls.len(), 4);
        assert!(calls.iter().all(|c| c == "from_app"));
    }

    #[test]
    fn test_is_admin_msg_type() {
        for t in &["0", "1", "2", "3", "4", "5", "A"] {
            assert!(is_admin_msg_type(t), "{} should be admin", t);
        }
        for t in &["D", "8", "G", "AE", "B", "7"] {
            assert!(!is_admin_msg_type(t), "{} should not be admin", t);
        }
    }

    #[test]
    fn test_valid_logon_state_before_logon() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        assert!(session.valid_logon_state("A"));
        assert!(!session.valid_logon_state("0"));
        assert!(!session.valid_logon_state("D"));
    }

    #[test]
    fn test_valid_logon_state_after_logon() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.state.logon_received = true;
        assert!(session.valid_logon_state("A"));
        assert!(session.valid_logon_state("0"));
        assert!(session.valid_logon_state("D"));
        assert!(session.valid_logon_state("5"));
    }

    #[test]
    fn test_valid_logon_state_during_logout() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.state.logon_received = true;
        session.state.logout_sent = true;
        assert!(session.valid_logon_state("5"));
        assert!(session.valid_logon_state("4"));
        assert!(!session.valid_logon_state("0"));
        assert!(!session.valid_logon_state("D"));
    }

    fn make_msg(msg_type: &str, sender: &str, target: &str, seq_num: u32) -> Message {
        use crate::message::StringField;
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(8, "FIX.4.3"));
        msg.header_mut().set_field(StringField::new(35, msg_type));
        msg.header_mut().set_field(StringField::new(49, sender));
        msg.header_mut().set_field(StringField::new(56, target));
        msg.header_mut().set_field(StringField::new(34, &seq_num.to_string()));
        msg
    }

    #[test]
    fn test_verify_msg_passes_valid_logon() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        let msg = make_msg("A", "TARGET", "SENDER", 1);
        assert!(session.verify_msg(&msg).is_ok());
    }

    #[test]
    fn test_verify_msg_rejects_begin_string_mismatch() {
        use crate::message::StringField;
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        msg.header_mut().set_field(StringField::new(8, "FIX.4.4"));
        assert!(session.verify_msg(&msg).is_err());
    }

    #[test]
    fn test_verify_msg_rejects_non_logon_before_logon() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        let msg = make_msg("D", "TARGET", "SENDER", 1);
        assert!(session.verify_msg(&msg).is_err());
    }

    #[test]
    fn test_verify_msg_rejects_compid_mismatch() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.state.logon_received = true;
        let msg = make_msg("D", "WRONG", "SENDER", 1);
        assert!(session.verify_msg(&msg).is_err());
    }

    #[test]
    fn test_verify_seq_number_rejects_low_seq_num() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.state.next_target_msg_seq_num = 5;
        let msg = make_msg("D", "TARGET", "SENDER", 3);
        assert!(session.verify_seq_number(&msg).is_err());
    }

    #[test]
    fn test_verify_seq_number_warns_high_seq_num_but_passes() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        let msg = make_msg("D", "TARGET", "SENDER", 10);
        assert!(session.verify_seq_number(&msg).is_ok());
    }

    #[test]
    fn test_verify_seq_number_increments_target_seq_num() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        assert_eq!(session.state.next_target_msg_seq_num, 1);

        let msg = make_msg("D", "TARGET", "SENDER", 1);
        session.verify_seq_number(&msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    #[test]
    fn test_verify_msg_resets_test_request_counter() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.state.logon_received = true;
        session.state.test_request_counter = 3;

        let msg = make_msg("D", "TARGET", "SENDER", 1);
        session.verify_msg(&msg).unwrap();
        assert_eq!(session.state.test_request_counter, 0);
    }

    #[test]
    fn test_next_logon_sets_logon_received() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.is_active = true;
        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();
        assert!(session.state.logon_received);
    }

    #[test]
    fn test_next_logon_acceptor_sends_logon_response() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.is_active = true;
        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();

        assert!(session.state.logon_sent);
        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "A"));
    }

    #[test]
    fn test_next_logon_initiator_does_not_send_logon_response() {
        let (mut session, mock_state) = make_initiator_session(Box::new(TestApplication::new()));
        session.is_active = true;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_logon_calls_from_admin_and_on_logon() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.is_active = true;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();

        let calls = get_test_app(&session).calls();
        assert!(calls.contains(&"from_admin".to_string()));
        assert!(calls.contains(&"on_logon".to_string()));
    }

    #[test]
    fn test_next_logon_rejects_inactive_session() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.is_active = false;
        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        assert!(session.next_logon(&mut msg).is_err());
        assert!(!session.state.logon_received);
    }

    #[test]
    fn test_next_logon_resets_seq_nums_on_reset_flag() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.is_active = true;
        session.state.next_target_msg_seq_num = 10;
        session.state.next_sender_msg_seq_num = 15;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        msg.set_field(StringField::new(141, "Y"));
        session.next_logon(&mut msg).unwrap();

        assert!(session.state.logon_received);
    }

    fn get_test_app(session: &Session) -> &TestApplication {
        unsafe { &*(session.app.as_ref() as *const dyn Application as *const TestApplication) }
    }

    fn sent_message_contains(wire: &str, tag: u32, expected_value: &str) -> bool {
        let prefix = format!("{}=", tag);
        wire.split('\x01')
            .find(|seg| seg.starts_with(&prefix))
            .map(|seg| &seg[prefix.len()..] == expected_value)
            .unwrap_or(false)
    }

    #[test]
    fn test_generate_logon_sends_correct_message() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.generate_logon();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);

        let wire = &sent[0];
        assert!(sent_message_contains(wire, 8, "FIX.4.3"));
        assert!(sent_message_contains(wire, 35, "A"));
        assert!(sent_message_contains(wire, 49, "SENDER"));
        assert!(sent_message_contains(wire, 56, "TARGET"));
        assert!(sent_message_contains(wire, 34, "1"));
        assert!(sent_message_contains(wire, 108, "30"));
    }

    #[test]
    fn test_generate_logon_calls_to_admin() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.generate_logon();

        let calls = get_test_app(&session).calls();
        assert_eq!(calls, vec!["to_admin"]);
    }

    #[test]
    fn test_generate_logon_increments_seq_num() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        assert_eq!(session.state.next_sender_msg_seq_num, 1);
        session.generate_logon();
        assert_eq!(session.state.next_sender_msg_seq_num, 2);
    }

    #[test]
    fn test_generate_logout_with_reason() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.generate_logout(Some("Session ended"));

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "5"));
        assert!(sent_message_contains(wire, 58, "Session ended"));
    }

    #[test]
    fn test_generate_logout_without_reason() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.generate_logout(None);

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "5"));
        assert!(!sent_message_contains(wire, 58, ""));
    }

    #[test]
    fn test_generate_heartbeat_without_test_req_id() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.generate_heartbeat(None);

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "0"));
        assert!(!wire.contains("112="));
    }

    #[test]
    fn test_generate_heartbeat_with_test_req_id() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.generate_heartbeat(Some("TEST123"));

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "0"));
        assert!(sent_message_contains(wire, 112, "TEST123"));
    }

    #[test]
    fn test_generate_test_request() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.generate_test_request("REQ456");

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "1"));
        assert!(sent_message_contains(wire, 112, "REQ456"));
    }

    #[test]
    fn test_multiple_generates_increment_seq_nums() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        assert_eq!(session.state.next_sender_msg_seq_num, 1);

        session.generate_logon();
        session.generate_heartbeat(None);
        session.generate_test_request("T1");
        session.generate_logout(None);

        assert_eq!(session.state.next_sender_msg_seq_num, 5);

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 4);
        assert!(sent_message_contains(&sent[0], 34, "1"));
        assert!(sent_message_contains(&sent[1], 34, "2"));
        assert!(sent_message_contains(&sent[2], 34, "3"));
        assert!(sent_message_contains(&sent[3], 34, "4"));
    }

    fn make_logged_on_session(app: Box<dyn Application>) -> (Session, MockState) {
        let (mut session, mock_state) = make_session(app);
        session.is_active = true;
        session.state.logon_received = true;
        (session, mock_state)
    }

    // --- next_heartbeat tests ---

    #[test]
    fn test_next_heartbeat_increments_target_seq_num() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));
        assert_eq!(session.state.next_target_msg_seq_num, 1);

        let mut msg = make_msg("0", "TARGET", "SENDER", 1);
        session.next_heartbeat(&mut msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    #[test]
    fn test_next_heartbeat_calls_from_admin() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("0", "TARGET", "SENDER", 1);
        session.next_heartbeat(&mut msg).unwrap();

        let calls = get_test_app(&session).calls();
        assert_eq!(calls, vec!["from_admin"]);
    }

    #[test]
    fn test_next_heartbeat_does_not_send_response() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("0", "TARGET", "SENDER", 1);
        session.next_heartbeat(&mut msg).unwrap();

        assert!(mock_state.sent().is_empty());
    }

    // --- next_test_request tests ---

    #[test]
    fn test_next_test_request_sends_heartbeat_with_echoed_test_req_id() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("1", "TARGET", "SENDER", 1);
        msg.set_field(StringField::new(112, "TEST123"));
        session.next_test_request(&mut msg).unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "0"));
        assert!(sent_message_contains(&sent[0], 112, "TEST123"));
    }

    #[test]
    fn test_next_test_request_without_test_req_id() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("1", "TARGET", "SENDER", 1);
        session.next_test_request(&mut msg).unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "0"));
        assert!(!sent[0].contains("112="));
    }

    #[test]
    fn test_next_test_request_increments_target_seq_num() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("1", "TARGET", "SENDER", 1);
        session.next_test_request(&mut msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    // --- next_logout tests ---

    #[test]
    fn test_next_logout_resets_flags_after_disconnect() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();
        // disconnect() resets all logon/logout flags — session is back to pre-logon state
        assert!(!session.state.logout_received);
        assert!(!session.state.logon_received);
        assert!(!session.state.logout_sent);
        assert!(!session.state.logon_sent);
    }

    #[test]
    fn test_next_logout_sends_response_when_not_initiated() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "5"));
    }

    #[test]
    fn test_next_logout_no_response_when_we_initiated() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));
        session.state.logout_sent = true;

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_logout_disconnects() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        assert!(mock_state.is_disconnected());
    }

    #[test]
    fn test_next_logout_calls_on_logout() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        let calls = get_test_app(&session).calls();
        assert!(calls.contains(&"on_logout".to_string()));
    }

    // --- next_message default branch (app messages) ---

    #[test]
    fn test_next_message_app_msg_calls_from_app() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("D", "TARGET", "SENDER", 1);
        session.next_message(&mut msg).unwrap();

        let calls = get_test_app(&session).calls();
        assert_eq!(calls, vec!["from_app"]);
    }

    #[test]
    fn test_next_message_app_msg_increments_target_seq_num() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));

        let mut msg = make_msg("D", "TARGET", "SENDER", 1);
        session.next_message(&mut msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    // --- next_tick helpers ---

    fn make_initiator_session(app: Box<dyn Application>) -> (Session, MockState) {
        let id = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let state = SessionState::new(30, true, Instant::now());
        let schedule = SessionSchedule::new(None, None, None, None, chrono_tz::UTC);
        let mock_state = MockState::new();
        let responder = MockResponder::new(mock_state.clone());
        let dd = DataDictionary::default();
        (Session::new(id, state, schedule, Some(Box::new(responder)), dd, app), mock_state)
    }

    fn make_session_with_schedule(
        app: Box<dyn Application>,
        is_initiator: bool,
        schedule: SessionSchedule,
    ) -> (Session, MockState) {
        let id = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let state = SessionState::new(30, is_initiator, Instant::now());
        let mock_state = MockState::new();
        let responder = MockResponder::new(mock_state.clone());
        let dd = DataDictionary::default();
        (Session::new(id, state, schedule, Some(Box::new(responder)), dd, app), mock_state)
    }

    // A schedule that is always outside session time (window already passed today).
    fn expired_schedule() -> SessionSchedule {
        use chrono::NaiveTime;
        SessionSchedule::new(
            Some(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            None,
            Some(NaiveTime::from_hms_opt(0, 0, 1).unwrap()),
            None,
            chrono_tz::UTC,
        )
    }

    // --- next_tick tests: no responder / inactive ---

    #[test]
    fn test_next_tick_no_responder_is_noop() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.responder = None;
        session.is_active = true;
        session.next_tick().unwrap();
        // no panic, no messages sent
    }

    #[test]
    fn test_next_tick_inactive_is_noop() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.is_active = false;
        session.next_tick().unwrap();
        assert!(mock_state.sent().is_empty());
    }

    // --- next_tick tests: pre-logon, initiator ---

    #[test]
    fn test_next_tick_initiator_sends_logon_in_session_time() {
        let (mut session, mock_state) = make_initiator_session(Box::new(TestApplication::new()));
        session.is_active = true;

        session.next_tick().unwrap();

        assert!(session.state.logon_sent);
        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "A"));
    }

    #[test]
    fn test_next_tick_initiator_does_not_resend_logon() {
        let (mut session, mock_state) = make_initiator_session(Box::new(TestApplication::new()));
        session.is_active = true;
        session.state.logon_sent = true;

        session.next_tick().unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_tick_initiator_disconnects_on_logon_timeout() {
        let (mut session, mock_state) = make_initiator_session(Box::new(TestApplication::new()));
        session.is_active = true;
        session.state.logon_sent = true;
        // Push last_sent_time far enough back to trigger logon timeout (>10s)
        session.state.last_sent_time = Instant::now() - Duration::from_secs(15);

        session.next_tick().unwrap();

        assert!(mock_state.is_disconnected());
    }

    #[test]
    fn test_next_tick_initiator_no_logon_outside_session_time() {
        let schedule = expired_schedule();
        let (mut session, mock_state) =
            make_session_with_schedule(Box::new(TestApplication::new()), true, schedule);
        session.is_active = true;

        session.next_tick().unwrap();

        assert!(!session.state.logon_sent);
        assert!(mock_state.sent().is_empty());
    }

    // --- next_tick tests: pre-logon, acceptor ---

    #[test]
    fn test_next_tick_acceptor_pre_logon_is_noop() {
        let (mut session, mock_state) = make_session(Box::new(TestApplication::new()));
        session.is_active = true;

        session.next_tick().unwrap();

        assert!(mock_state.sent().is_empty());
        assert!(!mock_state.is_disconnected());
    }

    // --- next_tick tests: logged on, outside session time ---

    #[test]
    fn test_next_tick_logged_on_outside_schedule_sends_logout() {
        let schedule = expired_schedule();
        let (mut session, mock_state) =
            make_session_with_schedule(Box::new(TestApplication::new()), false, schedule);
        session.is_active = true;
        session.state.logon_received = true;

        session.next_tick().unwrap();

        assert!(session.state.logout_sent);
        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "5"));
    }

    #[test]
    fn test_next_tick_logged_on_outside_schedule_does_not_resend_logout() {
        let schedule = expired_schedule();
        let (mut session, mock_state) =
            make_session_with_schedule(Box::new(TestApplication::new()), false, schedule);
        session.is_active = true;
        session.state.logon_received = true;
        session.state.logout_sent = true;

        session.next_tick().unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_tick_logout_timeout_disconnects() {
        let schedule = expired_schedule();
        let (mut session, mock_state) =
            make_session_with_schedule(Box::new(TestApplication::new()), false, schedule);
        session.is_active = true;
        session.state.logon_received = true;
        session.state.logout_sent = true;
        // Push last_sent_time back to trigger logout timeout (>2s)
        session.state.last_sent_time = Instant::now() - Duration::from_secs(5);

        session.next_tick().unwrap();

        assert!(mock_state.is_disconnected());
    }

    // --- next_tick tests: logged on, in session time, heartbeat escalation ---

    #[test]
    fn test_next_tick_sends_heartbeat_when_needed() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));
        // Push last_sent_time back past heartbeat interval (30s)
        session.state.last_sent_time = Instant::now() - Duration::from_secs(31);
        // Keep last_received_time recent so test request doesn't trigger
        session.state.last_received_time = Instant::now();

        session.next_tick().unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "0"));
    }

    #[test]
    fn test_next_tick_sends_test_request_over_heartbeat() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));
        // No receive for > 1.5x interval (>45s) triggers test request
        session.state.last_received_time = Instant::now() - Duration::from_secs(46);
        session.state.last_sent_time = Instant::now();

        session.next_tick().unwrap();

        assert_eq!(session.state.test_request_counter, 1);
        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "1"));
    }

    #[test]
    fn test_next_tick_disconnects_on_timeout() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));
        // No receive for > 2x interval (>60s) with counter > 1
        session.state.last_received_time = Instant::now() - Duration::from_secs(61);
        session.state.test_request_counter = 2;

        session.next_tick().unwrap();

        assert!(mock_state.is_disconnected());
        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_tick_no_action_when_all_timers_fresh() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));
        // Both timestamps are recent — nothing should fire
        session.state.last_sent_time = Instant::now();
        session.state.last_received_time = Instant::now();

        session.next_tick().unwrap();

        assert!(mock_state.sent().is_empty());
        assert!(!mock_state.is_disconnected());
    }

    // --- disconnect tests ---

    #[test]
    fn test_disconnect_clears_responder() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));
        assert!(session.responder.is_some());
        session.disconnect();
        assert!(session.responder.is_none());
    }

    #[test]
    fn test_disconnect_resets_all_flags() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));
        session.state.logon_sent = true;
        session.state.logout_sent = true;
        session.state.logout_received = true;

        session.disconnect();

        assert!(!session.state.logon_sent);
        assert!(!session.state.logon_received);
        assert!(!session.state.logout_sent);
        assert!(!session.state.logout_received);
    }

    #[test]
    fn test_disconnect_calls_responder_disconnect() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));
        assert!(!mock_state.is_disconnected());

        session.disconnect();

        assert!(mock_state.is_disconnected());
    }

    #[test]
    fn test_disconnect_calls_on_logout() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));

        session.disconnect();

        let calls = get_test_app(&session).calls();
        assert!(calls.contains(&"on_logout".to_string()));
    }

    #[test]
    fn test_disconnect_without_responder_is_safe() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.responder = None;
        session.disconnect();
        assert!(session.responder.is_none());
    }

    // --- reset flag tests ---

    #[test]
    fn test_reset_on_disconnect_resets_seq_nums() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));
        session.reset_on_disconnect = true;
        session.state.next_sender_msg_seq_num = 10;
        session.state.next_target_msg_seq_num = 8;

        session.disconnect();

        assert_eq!(session.state.next_sender_msg_seq_num, 1);
        assert_eq!(session.state.next_target_msg_seq_num, 1);
    }

    #[test]
    fn test_no_reset_on_disconnect_preserves_seq_nums() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));
        session.state.next_sender_msg_seq_num = 10;
        session.state.next_target_msg_seq_num = 8;

        session.disconnect();

        assert_eq!(session.state.next_sender_msg_seq_num, 10);
        assert_eq!(session.state.next_target_msg_seq_num, 8);
    }

    #[test]
    fn test_reset_on_logon_resets_seq_nums_before_verify() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.is_active = true;
        session.reset_on_logon = true;
        session.state.next_sender_msg_seq_num = 10;
        session.state.next_target_msg_seq_num = 10;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();

        assert!(session.state.logon_received);
    }

    #[test]
    fn test_no_reset_on_logon_rejects_low_seq_num() {
        let (mut session, _) = make_session(Box::new(TestApplication::new()));
        session.is_active = true;
        session.state.next_target_msg_seq_num = 10;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        assert!(session.next_logon(&mut msg).is_err());
        assert!(!session.state.logon_received);
    }

    #[test]
    fn test_reset_on_logout_resets_seq_nums() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));
        session.reset_on_logout = true;
        session.state.next_sender_msg_seq_num = 10;
        session.state.next_target_msg_seq_num = 8;

        let mut msg = make_msg("5", "TARGET", "SENDER", 8);
        session.next_logout(&mut msg).unwrap();

        assert_eq!(session.state.next_sender_msg_seq_num, 1);
        assert_eq!(session.state.next_target_msg_seq_num, 1);
    }

    #[test]
    fn test_no_reset_on_logout_preserves_seq_nums() {
        let (mut session, _) = make_logged_on_session(Box::new(TestApplication::new()));
        session.state.next_sender_msg_seq_num = 10;
        session.state.next_target_msg_seq_num = 8;

        let mut msg = make_msg("5", "TARGET", "SENDER", 8);
        session.next_logout(&mut msg).unwrap();

        // disconnect clears flags but not seq nums; logout response uses seq 10
        assert_eq!(session.state.next_sender_msg_seq_num, 11);
        assert_eq!(session.state.next_target_msg_seq_num, 8);
    }

    #[test]
    fn test_next_tick_noop_after_disconnect() {
        let (mut session, mock_state) = make_logged_on_session(Box::new(TestApplication::new()));
        session.disconnect();

        session.next_tick().unwrap();

        // No messages sent after disconnect — responder is gone
        assert!(mock_state.sent().is_empty());
    }
}
