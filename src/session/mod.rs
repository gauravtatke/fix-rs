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
use crate::quickfix_errors::{SendError, SessionError};
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
    // The session OWNS its application. There is no back-edge: the app never
    // holds a reference to the session. Inbound callbacks *return* the messages
    // they want sent (Vec<Message>), and the session sends them — so the
    // ownership graph is a one-way Session -> app, with no cycle to leak.
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
        // Disjoint-field borrow: &mut self.app alongside &self.id is fine — the
        // compiler sees they don't overlap. No app parameter needed.
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
    // Admin messages (MsgType 0-5, A) → on_admin_msg_received; all others → on_app_msg_received.
    //
    // The app-message callback RETURNS the responses it wants sent. Because the
    // returned Vec is owned, the borrow of `self.app` ends the moment the call
    // returns — which frees `self` for the `send_app_message` loop below. This
    // is the whole trick: the app never reaches back into the session, so there
    // is no cycle and no borrow conflict, even though the app lives inside self.
    fn dispatch_to_app(&mut self, msg_type: &str, msg: &Message) -> Result<(), Box<dyn Error>> {
        if is_admin_msg_type(msg_type) {
            self.app.on_admin_msg_received(&self.id, msg)?;
        } else {
            let responses = self.app.on_app_msg_received(&self.id, msg)?;
            for resp in responses {
                self.send_app_message(resp)?;
            }
        }
        Ok(())
    }

    // The single outbound path for application messages. Stamps the standard
    // header, gives the app a final peek (which may veto with DonotSend), then
    // serializes and pushes through the responder. DonotSend is a normal "skip"
    // — the message is dropped and the sequence number is left untouched — not
    // an error, so it never tears down the session.
    pub(crate) fn send_app_message(&mut self, mut msg: Message) -> Result<(), SendError> {
        // Mirror QFJ sendRaw: app messages only go out on a logged-on session
        // with a live connection. Guarding here is what makes external-thread
        // senders (a market-data feed calling SessionMap::send) safe — without
        // it, a race against a not-yet-connected or just-disconnected session
        // would panic on send_raw's `responder.unwrap()`.
        if self.responder.is_none() || !self.state.logon_received {
            return Err(SendError::NotLoggedOn);
        }
        self.initialize_header(&mut msg);
        if self.app.on_app_msg_sending(&self.id, &mut msg).is_err() {
            info!("{} outgoing app message vetoed (DonotSend)", self.id);
            return Ok(());
        }
        self.send_raw(&mut msg);
        Ok(())
    }

    // Timer-driven outbound seam for messages the app sends on its own
    // initiative (e.g. a streaming market-data feed) with no inbound trigger.
    // The engine polls the app each tick; the app drains whatever it has queued
    // (fed by its own source thread over a channel) and we send each message.
    // The app still holds no reference to the session — the orchestrator drives.
    pub(crate) fn poll_outbound(&mut self) -> Result<(), Box<dyn Error>> {
        if self.responder.is_none() || !self.state.logon_received {
            return Ok(());
        }
        let outbound = self.app.poll_outbound(&self.id);
        for msg in outbound {
            self.send_app_message(msg)?;
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
        self.app.on_admin_msg_sending(&self.id, &mut msg);
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
        self.app.on_admin_msg_sending(&self.id, &mut msg);
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
        self.app.on_admin_msg_sending(&self.id, &mut msg);
        self.send_raw(&mut msg);
    }

    // Builds and sends a TestRequest (MsgType=1) with the given TestReqID.
    fn generate_test_request(&mut self, req_id: &str) {
        let mut msg = Message::new();
        msg.header_mut().set_field(StringField::new(35, "1"));
        msg.set_field(StringField::new(112, req_id));
        self.initialize_header(&mut msg);
        self.app.on_admin_msg_sending(&self.id, &mut msg);
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
        self.app.on_admin_msg_received(&self.id, msg)?;
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
        self.app.on_admin_msg_received(&self.id, msg)?;
        Ok(())
    }

    // Handles inbound TestRequest (MsgType=1). Responds with Heartbeat echoing TestReqID.
    fn next_test_request(&mut self, msg: &mut Message) -> Result<(), Box<dyn Error>> {
        self.verify_msg(msg)?;
        self.verify_seq_number(msg)?;
        self.app.on_admin_msg_received(&self.id, msg)?;
        let test_req_id = msg.get_field::<String>(112).ok();
        self.generate_heartbeat(test_req_id.as_deref());
        Ok(())
    }

    // Handles inbound Logout (MsgType=5). If we didn't initiate the logout,
    // responds with our own Logout. Always disconnects after notifying app.
    fn next_logout(&mut self, msg: &mut Message) -> Result<(), Box<dyn Error>> {
        self.verify_msg(msg)?;
        self.app.on_admin_msg_received(&self.id, msg)?;
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

    // Shared, thread-safe record of which callbacks fired. Since the app now
    // moves INTO the session, the test keeps a clone of this handle to inspect
    // calls afterward — the same shared-handle pattern MockState uses for the
    // responder. This is the one real ergonomic cost of app-in-session, and it
    // is small.
    #[derive(Clone)]
    struct AppSpy {
        calls: Arc<Mutex<Vec<String>>>,
    }

    impl AppSpy {
        fn new() -> Self {
            Self {
                calls: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn calls(&self) -> Vec<String> {
            self.calls.lock().unwrap().clone()
        }

        fn record(&self, name: &str) {
            self.calls.lock().unwrap().push(name.to_string());
        }
    }

    struct TestApplication {
        spy: AppSpy,
    }

    impl TestApplication {
        fn new(spy: AppSpy) -> Self {
            Self { spy }
        }
    }

    impl Application for TestApplication {
        fn on_create(&mut self, _session_id: &SessionId) {
            self.spy.record("on_create");
        }
        fn on_logon(&mut self, _session_id: &SessionId) {
            self.spy.record("on_logon");
        }
        fn on_logout(&mut self, _session_id: &SessionId) {
            self.spy.record("on_logout");
        }
        fn on_admin_msg_sending(&mut self, _session_id: &SessionId, _message: &mut Message) {
            self.spy.record("to_admin");
        }
        fn on_admin_msg_received(
            &mut self,
            _session_id: &SessionId,
            _message: &Message,
        ) -> Result<(), RejectLogon> {
            self.spy.record("from_admin");
            Ok(())
        }
        fn on_app_msg_sending(
            &mut self,
            _session_id: &SessionId,
            _message: &mut Message,
        ) -> Result<(), DonotSend> {
            self.spy.record("to_app");
            Ok(())
        }
        fn on_app_msg_received(
            &mut self,
            _session_id: &SessionId,
            _message: &Message,
        ) -> Result<Vec<Message>, AppError> {
            self.spy.record("from_app");
            Ok(vec![])
        }
    }

    // Default helper: session with a no-op app inside. Used by tests that don't
    // inspect callbacks (they check sent wire messages or session state).
    fn make_session() -> (Session, MockState) {
        let (session, mock_state, _spy) = make_session_spy();
        (session, mock_state)
    }

    // Session with an inspectable TestApplication inside; returns the spy handle
    // so the test can assert on which callbacks fired.
    fn make_session_spy() -> (Session, MockState, AppSpy) {
        let id = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let state = SessionState::new(30, false, Instant::now());
        let schedule = SessionSchedule::new(None, None, None, None, chrono_tz::UTC);
        let mock_state = MockState::new();
        let responder = MockResponder::new(mock_state.clone());
        let dd = DataDictionary::default();
        let spy = AppSpy::new();
        let app = Box::new(TestApplication::new(spy.clone()));
        let session = Session::new(id, state, schedule, Some(Box::new(responder)), dd, app);
        (session, mock_state, spy)
    }

    #[test]
    fn test_dispatch_to_app_dispatches_admin_to_from_admin() {
        let (mut session, _, app) = make_session_spy();
        let msg = Message::new();

        for msg_type in &["0", "1", "2", "3", "4", "5", "A"] {
            session.dispatch_to_app(msg_type, &msg).unwrap();
        }

        let calls = app.calls();
        assert_eq!(calls.len(), 7);
        assert!(calls.iter().all(|c| c == "from_admin"));
    }

    #[test]
    fn test_dispatch_to_app_dispatches_app_to_from_app() {
        let (mut session, _, app) = make_session_spy();
        let msg = Message::new();

        for msg_type in &["D", "8", "G", "AE"] {
            session.dispatch_to_app(msg_type, &msg).unwrap();
        }

        let calls = app.calls();
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
        let (mut session, _) = make_session();
        assert!(session.valid_logon_state("A"));
        assert!(!session.valid_logon_state("0"));
        assert!(!session.valid_logon_state("D"));
    }

    #[test]
    fn test_valid_logon_state_after_logon() {
        let (mut session, _) = make_session();
        session.state.logon_received = true;
        assert!(session.valid_logon_state("A"));
        assert!(session.valid_logon_state("0"));
        assert!(session.valid_logon_state("D"));
        assert!(session.valid_logon_state("5"));
    }

    #[test]
    fn test_valid_logon_state_during_logout() {
        let (mut session, _) = make_session();
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
        let (mut session, _) = make_session();
        let msg = make_msg("A", "TARGET", "SENDER", 1);
        assert!(session.verify_msg(&msg).is_ok());
    }

    #[test]
    fn test_verify_msg_rejects_begin_string_mismatch() {
        use crate::message::StringField;
        let (mut session, _) = make_session();
        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        msg.header_mut().set_field(StringField::new(8, "FIX.4.4"));
        assert!(session.verify_msg(&msg).is_err());
    }

    #[test]
    fn test_verify_msg_rejects_non_logon_before_logon() {
        let (mut session, _) = make_session();
        let msg = make_msg("D", "TARGET", "SENDER", 1);
        assert!(session.verify_msg(&msg).is_err());
    }

    #[test]
    fn test_verify_msg_rejects_compid_mismatch() {
        let (mut session, _) = make_session();
        session.state.logon_received = true;
        let msg = make_msg("D", "WRONG", "SENDER", 1);
        assert!(session.verify_msg(&msg).is_err());
    }

    #[test]
    fn test_verify_seq_number_rejects_low_seq_num() {
        let (mut session, _) = make_session();
        session.state.next_target_msg_seq_num = 5;
        let msg = make_msg("D", "TARGET", "SENDER", 3);
        assert!(session.verify_seq_number(&msg).is_err());
    }

    #[test]
    fn test_verify_seq_number_warns_high_seq_num_but_passes() {
        let (mut session, _) = make_session();
        let msg = make_msg("D", "TARGET", "SENDER", 10);
        assert!(session.verify_seq_number(&msg).is_ok());
    }

    #[test]
    fn test_verify_seq_number_increments_target_seq_num() {
        let (mut session, _) = make_session();
        assert_eq!(session.state.next_target_msg_seq_num, 1);

        let msg = make_msg("D", "TARGET", "SENDER", 1);
        session.verify_seq_number(&msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    #[test]
    fn test_verify_msg_resets_test_request_counter() {
        let (mut session, _) = make_session();
        session.state.logon_received = true;
        session.state.test_request_counter = 3;

        let msg = make_msg("D", "TARGET", "SENDER", 1);
        session.verify_msg(&msg).unwrap();
        assert_eq!(session.state.test_request_counter, 0);
    }

    #[test]
    fn test_next_logon_sets_logon_received() {
        let (mut session, _) = make_session();
        session.is_active = true;
        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();
        assert!(session.state.logon_received);
    }

    #[test]
    fn test_next_logon_acceptor_sends_logon_response() {
        let (mut session, mock_state) = make_session();
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
        let (mut session, mock_state) = make_initiator_session();
        session.is_active = true;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_logon_calls_from_admin_and_on_logon() {
        let (mut session, _, app) = make_session_spy();
        session.is_active = true;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        session.next_logon(&mut msg).unwrap();

        let calls = app.calls();
        assert!(calls.contains(&"from_admin".to_string()));
        assert!(calls.contains(&"on_logon".to_string()));
    }

    #[test]
    fn test_next_logon_rejects_inactive_session() {
        let (mut session, _) = make_session();
        session.is_active = false;
        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        assert!(session.next_logon(&mut msg).is_err());
        assert!(!session.state.logon_received);
    }

    #[test]
    fn test_next_logon_resets_seq_nums_on_reset_flag() {
        let (mut session, _) = make_session();
        session.is_active = true;
        session.state.next_target_msg_seq_num = 10;
        session.state.next_sender_msg_seq_num = 15;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        msg.set_field(StringField::new(141, "Y"));
        session.next_logon(&mut msg).unwrap();

        assert!(session.state.logon_received);
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
        let (mut session, mock_state) = make_session();
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
        let (mut session, _, app) = make_session_spy();
        session.generate_logon();

        let calls = app.calls();
        assert_eq!(calls, vec!["to_admin"]);
    }

    #[test]
    fn test_generate_logon_increments_seq_num() {
        let (mut session, _) = make_session();
        assert_eq!(session.state.next_sender_msg_seq_num, 1);
        session.generate_logon();
        assert_eq!(session.state.next_sender_msg_seq_num, 2);
    }

    #[test]
    fn test_generate_logout_with_reason() {
        let (mut session, mock_state) = make_session();
        session.generate_logout(Some("Session ended"));

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "5"));
        assert!(sent_message_contains(wire, 58, "Session ended"));
    }

    #[test]
    fn test_generate_logout_without_reason() {
        let (mut session, mock_state) = make_session();
        session.generate_logout(None);

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "5"));
        assert!(!sent_message_contains(wire, 58, ""));
    }

    #[test]
    fn test_generate_heartbeat_without_test_req_id() {
        let (mut session, mock_state) = make_session();
        session.generate_heartbeat(None);

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "0"));
        assert!(!wire.contains("112="));
    }

    #[test]
    fn test_generate_heartbeat_with_test_req_id() {
        let (mut session, mock_state) = make_session();
        session.generate_heartbeat(Some("TEST123"));

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "0"));
        assert!(sent_message_contains(wire, 112, "TEST123"));
    }

    #[test]
    fn test_generate_test_request() {
        let (mut session, mock_state) = make_session();
        session.generate_test_request("REQ456");

        let sent = mock_state.sent();
        let wire = &sent[0];
        assert!(sent_message_contains(wire, 35, "1"));
        assert!(sent_message_contains(wire, 112, "REQ456"));
    }

    #[test]
    fn test_multiple_generates_increment_seq_nums() {
        let (mut session, mock_state) = make_session();
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

    fn make_logged_on_session() -> (Session, MockState) {
        let (mut session, mock_state) = make_session();
        session.is_active = true;
        session.state.logon_received = true;
        (session, mock_state)
    }

    fn make_logged_on_session_spy() -> (Session, MockState, AppSpy) {
        let (mut session, mock_state, spy) = make_session_spy();
        session.is_active = true;
        session.state.logon_received = true;
        (session, mock_state, spy)
    }

    // --- next_heartbeat tests ---

    #[test]
    fn test_next_heartbeat_increments_target_seq_num() {
        let (mut session, _) = make_logged_on_session();
        assert_eq!(session.state.next_target_msg_seq_num, 1);

        let mut msg = make_msg("0", "TARGET", "SENDER", 1);
        session.next_heartbeat(&mut msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    #[test]
    fn test_next_heartbeat_calls_from_admin() {
        let (mut session, _, app) = make_logged_on_session_spy();

        let mut msg = make_msg("0", "TARGET", "SENDER", 1);
        session.next_heartbeat(&mut msg).unwrap();

        let calls = app.calls();
        assert_eq!(calls, vec!["from_admin"]);
    }

    #[test]
    fn test_next_heartbeat_does_not_send_response() {
        let (mut session, mock_state) = make_logged_on_session();

        let mut msg = make_msg("0", "TARGET", "SENDER", 1);
        session.next_heartbeat(&mut msg).unwrap();

        assert!(mock_state.sent().is_empty());
    }

    // --- next_test_request tests ---

    #[test]
    fn test_next_test_request_sends_heartbeat_with_echoed_test_req_id() {
        let (mut session, mock_state) = make_logged_on_session();

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
        let (mut session, mock_state) = make_logged_on_session();

        let mut msg = make_msg("1", "TARGET", "SENDER", 1);
        session.next_test_request(&mut msg).unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "0"));
        assert!(!sent[0].contains("112="));
    }

    #[test]
    fn test_next_test_request_increments_target_seq_num() {
        let (mut session, _) = make_logged_on_session();

        let mut msg = make_msg("1", "TARGET", "SENDER", 1);
        session.next_test_request(&mut msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    // --- next_logout tests ---

    #[test]
    fn test_next_logout_resets_flags_after_disconnect() {
        let (mut session, _) = make_logged_on_session();

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
        let (mut session, mock_state) = make_logged_on_session();

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "5"));
    }

    #[test]
    fn test_next_logout_no_response_when_we_initiated() {
        let (mut session, mock_state) = make_logged_on_session();
        session.state.logout_sent = true;

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_logout_disconnects() {
        let (mut session, mock_state) = make_logged_on_session();

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        assert!(mock_state.is_disconnected());
    }

    #[test]
    fn test_next_logout_calls_on_logout() {
        let (mut session, _, app) = make_logged_on_session_spy();

        let mut msg = make_msg("5", "TARGET", "SENDER", 1);
        session.next_logout(&mut msg).unwrap();

        let calls = app.calls();
        assert!(calls.contains(&"on_logout".to_string()));
    }

    // --- next_message default branch (app messages) ---

    #[test]
    fn test_next_message_app_msg_calls_from_app() {
        let (mut session, _, app) = make_logged_on_session_spy();

        let mut msg = make_msg("D", "TARGET", "SENDER", 1);
        session.next_message(&mut msg).unwrap();

        let calls = app.calls();
        assert_eq!(calls, vec!["from_app"]);
    }

    #[test]
    fn test_next_message_app_msg_increments_target_seq_num() {
        let (mut session, _) = make_logged_on_session();

        let mut msg = make_msg("D", "TARGET", "SENDER", 1);
        session.next_message(&mut msg).unwrap();
        assert_eq!(session.state.next_target_msg_seq_num, 2);
    }

    // --- next_tick helpers ---

    fn make_initiator_session() -> (Session, MockState) {
        let id = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let state = SessionState::new(30, true, Instant::now());
        let schedule = SessionSchedule::new(None, None, None, None, chrono_tz::UTC);
        let mock_state = MockState::new();
        let responder = MockResponder::new(mock_state.clone());
        let dd = DataDictionary::default();
        let app = Box::new(TestApplication::new(AppSpy::new()));
        let session = Session::new(id, state, schedule, Some(Box::new(responder)), dd, app);
        (session, mock_state)
    }

    fn make_session_with_schedule(
        is_initiator: bool,
        schedule: SessionSchedule,
    ) -> (Session, MockState) {
        let id = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let state = SessionState::new(30, is_initiator, Instant::now());
        let mock_state = MockState::new();
        let responder = MockResponder::new(mock_state.clone());
        let dd = DataDictionary::default();
        let app = Box::new(TestApplication::new(AppSpy::new()));
        let session = Session::new(id, state, schedule, Some(Box::new(responder)), dd, app);
        (session, mock_state)
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
        let (mut session, _) = make_session();
        session.responder = None;
        session.is_active = true;
        session.next_tick().unwrap();
        // no panic, no messages sent
    }

    #[test]
    fn test_next_tick_inactive_is_noop() {
        let (mut session, mock_state) = make_session();
        session.is_active = false;
        session.next_tick().unwrap();
        assert!(mock_state.sent().is_empty());
    }

    // --- next_tick tests: pre-logon, initiator ---

    #[test]
    fn test_next_tick_initiator_sends_logon_in_session_time() {
        let (mut session, mock_state) = make_initiator_session();
        session.is_active = true;

        session.next_tick().unwrap();

        assert!(session.state.logon_sent);
        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "A"));
    }

    #[test]
    fn test_next_tick_initiator_does_not_resend_logon() {
        let (mut session, mock_state) = make_initiator_session();
        session.is_active = true;
        session.state.logon_sent = true;

        session.next_tick().unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_tick_initiator_disconnects_on_logon_timeout() {
        let (mut session, mock_state) = make_initiator_session();
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
        let (mut session, mock_state) = make_session_with_schedule(true, schedule);
        session.is_active = true;

        session.next_tick().unwrap();

        assert!(!session.state.logon_sent);
        assert!(mock_state.sent().is_empty());
    }

    // --- next_tick tests: pre-logon, acceptor ---

    #[test]
    fn test_next_tick_acceptor_pre_logon_is_noop() {
        let (mut session, mock_state) = make_session();
        session.is_active = true;

        session.next_tick().unwrap();

        assert!(mock_state.sent().is_empty());
        assert!(!mock_state.is_disconnected());
    }

    // --- next_tick tests: logged on, outside session time ---

    #[test]
    fn test_next_tick_logged_on_outside_schedule_sends_logout() {
        let schedule = expired_schedule();
        let (mut session, mock_state) = make_session_with_schedule(false, schedule);
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
        let (mut session, mock_state) = make_session_with_schedule(false, schedule);
        session.is_active = true;
        session.state.logon_received = true;
        session.state.logout_sent = true;

        session.next_tick().unwrap();

        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_tick_logout_timeout_disconnects() {
        let schedule = expired_schedule();
        let (mut session, mock_state) = make_session_with_schedule(false, schedule);
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
        let (mut session, mock_state) = make_logged_on_session();
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
        let (mut session, mock_state) = make_logged_on_session();
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
        let (mut session, mock_state) = make_logged_on_session();
        // No receive for > 2x interval (>60s) with counter > 1
        session.state.last_received_time = Instant::now() - Duration::from_secs(61);
        session.state.test_request_counter = 2;

        session.next_tick().unwrap();

        assert!(mock_state.is_disconnected());
        assert!(mock_state.sent().is_empty());
    }

    #[test]
    fn test_next_tick_no_action_when_all_timers_fresh() {
        let (mut session, mock_state) = make_logged_on_session();
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
        let (mut session, _) = make_logged_on_session();
        assert!(session.responder.is_some());
        session.disconnect();
        assert!(session.responder.is_none());
    }

    #[test]
    fn test_disconnect_resets_all_flags() {
        let (mut session, _) = make_logged_on_session();
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
        let (mut session, mock_state) = make_logged_on_session();
        assert!(!mock_state.is_disconnected());

        session.disconnect();

        assert!(mock_state.is_disconnected());
    }

    #[test]
    fn test_disconnect_calls_on_logout() {
        let (mut session, _, app) = make_logged_on_session_spy();

        session.disconnect();

        let calls = app.calls();
        assert!(calls.contains(&"on_logout".to_string()));
    }

    #[test]
    fn test_disconnect_without_responder_is_safe() {
        let (mut session, _) = make_session();
        session.responder = None;
        session.disconnect();
        assert!(session.responder.is_none());
    }

    // --- reset flag tests ---

    #[test]
    fn test_reset_on_disconnect_resets_seq_nums() {
        let (mut session, _) = make_logged_on_session();
        session.reset_on_disconnect = true;
        session.state.next_sender_msg_seq_num = 10;
        session.state.next_target_msg_seq_num = 8;

        session.disconnect();

        assert_eq!(session.state.next_sender_msg_seq_num, 1);
        assert_eq!(session.state.next_target_msg_seq_num, 1);
    }

    #[test]
    fn test_no_reset_on_disconnect_preserves_seq_nums() {
        let (mut session, _) = make_logged_on_session();
        session.state.next_sender_msg_seq_num = 10;
        session.state.next_target_msg_seq_num = 8;

        session.disconnect();

        assert_eq!(session.state.next_sender_msg_seq_num, 10);
        assert_eq!(session.state.next_target_msg_seq_num, 8);
    }

    #[test]
    fn test_reset_on_logon_resets_seq_nums_before_verify() {
        let (mut session, _) = make_session();
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
        let (mut session, _) = make_session();
        session.is_active = true;
        session.state.next_target_msg_seq_num = 10;

        let mut msg = make_msg("A", "TARGET", "SENDER", 1);
        assert!(session.next_logon(&mut msg).is_err());
        assert!(!session.state.logon_received);
    }

    #[test]
    fn test_reset_on_logout_resets_seq_nums() {
        let (mut session, _) = make_logged_on_session();
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
        let (mut session, _) = make_logged_on_session();
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
        let (mut session, mock_state) = make_logged_on_session();
        session.disconnect();

        session.next_tick().unwrap();

        // No messages sent after disconnect — responder is gone
        assert!(mock_state.sent().is_empty());
    }

    // --- outbound app-message path (the point of app-in-session) ---
    //
    // A configurable business app. It can answer an inbound app message with a
    // canned response (request/response), veto its own outbound sends
    // (DonotSend), and stream messages from an internal queue when polled — the
    // market-data-feed shape: one request in, many messages out over time. It
    // holds NO reference to the session; it only returns messages.
    struct ScriptedApp {
        feed: VecDeque<Message>,
        respond: bool,
        veto: bool,
    }

    impl ScriptedApp {
        fn new(feed: VecDeque<Message>, respond: bool, veto: bool) -> Self {
            Self {
                feed,
                respond,
                veto,
            }
        }
    }

    fn app_msg(msg_type: &str) -> Message {
        let mut m = Message::new();
        m.header_mut().set_field(StringField::new(35, msg_type));
        m
    }

    impl Application for ScriptedApp {
        fn on_create(&mut self, _s: &SessionId) {}
        fn on_logon(&mut self, _s: &SessionId) {}
        fn on_logout(&mut self, _s: &SessionId) {}
        fn on_admin_msg_sending(&mut self, _s: &SessionId, _m: &mut Message) {}
        fn on_admin_msg_received(
            &mut self,
            _s: &SessionId,
            _m: &Message,
        ) -> Result<(), RejectLogon> {
            Ok(())
        }
        fn on_app_msg_sending(
            &mut self,
            _s: &SessionId,
            _m: &mut Message,
        ) -> Result<(), DonotSend> {
            if self.veto { Err(DonotSend) } else { Ok(()) }
        }
        fn on_app_msg_received(
            &mut self,
            _s: &SessionId,
            _m: &Message,
        ) -> Result<Vec<Message>, AppError> {
            if self.respond {
                // MarketDataSnapshotFullRefresh-ish reply to the request.
                Ok(vec![app_msg("W")])
            } else {
                Ok(vec![])
            }
        }
        fn poll_outbound(&mut self, _s: &SessionId) -> Vec<Message> {
            // Everything the feed produced since the last poll.
            self.feed.drain(..).collect()
        }
    }

    fn make_logged_on_session_with_app(app: Box<dyn Application>) -> (Session, MockState) {
        let id = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let state = SessionState::new(30, false, Instant::now());
        let schedule = SessionSchedule::new(None, None, None, None, chrono_tz::UTC);
        let mock_state = MockState::new();
        let responder = MockResponder::new(mock_state.clone());
        let dd = DataDictionary::default();
        let mut session = Session::new(id, state, schedule, Some(Box::new(responder)), dd, app);
        session.is_active = true;
        session.state.logon_received = true;
        (session, mock_state)
    }

    // Request/response: inbound app message → app RETURNS a response → the engine
    // sends it. No back-reference into the session, no channels.
    #[test]
    fn test_request_response_send_via_return_value() {
        let app = Box::new(ScriptedApp::new(VecDeque::new(), true, false));
        let (mut session, mock_state) = make_logged_on_session_with_app(app);

        let mut req = make_msg("V", "TARGET", "SENDER", 1); // MarketDataRequest
        session.next_message(&mut req).unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "W"));
    }

    // Streaming: the feed keeps producing after the single request. Two queued
    // ticks are sent on a poll with NO inbound message driving them — this is the
    // "market-data feed continues until logout" case, handled by poll_outbound.
    #[test]
    fn test_poll_outbound_streams_without_inbound_trigger() {
        let mut feed = VecDeque::new();
        feed.push_back(app_msg("W"));
        feed.push_back(app_msg("W"));
        let app = Box::new(ScriptedApp::new(feed, false, false));
        let (mut session, mock_state) = make_logged_on_session_with_app(app);

        session.poll_outbound().unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 2);
        assert!(sent_message_contains(&sent[0], 35, "W"));
        assert!(sent_message_contains(&sent[1], 35, "W"));
        // Sequence numbers advance normally on the unsolicited stream.
        assert!(sent_message_contains(&sent[0], 34, "1"));
        assert!(sent_message_contains(&sent[1], 34, "2"));
    }

    // DonotSend is a normal skip, not an error: the message is dropped, the
    // session is NOT torn down, and the sequence number is left untouched.
    #[test]
    fn test_donotsend_skips_without_error_or_seqnum_bump() {
        let app = Box::new(ScriptedApp::new(VecDeque::new(), true, true));
        let (mut session, mock_state) = make_logged_on_session_with_app(app);
        let seq_before = session.state.next_sender_msg_seq_num;

        let mut req = make_msg("V", "TARGET", "SENDER", 1);
        assert!(session.next_message(&mut req).is_ok());

        assert!(mock_state.sent().is_empty());
        assert_eq!(session.state.next_sender_msg_seq_num, seq_before);
    }

    // QFJ-style outbound market-data push: a feed thread that holds ONLY a
    // SessionMap clone (no session or app reference) streams a snapshot on its
    // own thread, whenever data is available. This is the fix-rs analogue of
    // Session.sendToTarget, and the reason app-in-session needs no back-reference
    // for unsolicited sends — the producer drives, via the registry + lock.
    #[test]
    fn test_session_map_send_pushes_from_external_thread() {
        use crate::network::SessionMap;

        let (session, mock_state) = make_logged_on_session();
        let id = session.id.clone();
        let map: SessionMap = vec![(id.clone(), session)].into_iter().collect();

        let feed_map = map.clone();
        let feed_id = id.clone();
        std::thread::spawn(move || {
            let mut md = Message::new();
            md.header_mut().set_field(StringField::new(35, "W")); // MarketDataSnapshot
            feed_map.send(&feed_id, md).unwrap();
        })
        .join()
        .unwrap();

        let sent = mock_state.sent();
        assert_eq!(sent.len(), 1);
        assert!(sent_message_contains(&sent[0], 35, "W"));
    }
}
