#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;
use crate::quickfix_errors::{AppError, DonotSend, RejectLogon};
use crate::session::*;

// Naming: on_<admin|app>_msg_<sending|received>.
//
// Outbound hooks (engine → counterparty, called before the message leaves):
//   on_admin_msg_sending — modify outbound admin messages (e.g. add credentials to Logon)
//   on_app_msg_sending   — modify or cancel outbound app messages (DonotSend)
//
// Inbound hooks (counterparty → engine → application):
//   on_admin_msg_received — admin message arrived; can reject logon (RejectLogon)
//   on_app_msg_received   — app message arrived; return response messages to send back
//                           (Vec<Message>). Empty vec = nothing to send. The engine stamps
//                           headers (CompIDs, MsgSeqNum, SendingTime) and serializes each
//                           returned message — the application only builds the body.
pub trait Application: Send {
    fn on_create(&mut self, session_id: &SessionId);
    fn on_logon(&mut self, session_id: &SessionId);
    fn on_logout(&mut self, session_id: &SessionId);
    fn on_admin_msg_sending(&mut self, session_id: &SessionId, message: &mut Message);
    fn on_admin_msg_received(
        &mut self,
        session_id: &SessionId,
        message: &Message,
    ) -> Result<(), RejectLogon>;
    fn on_app_msg_sending(
        &mut self,
        session_id: &SessionId,
        message: &mut Message,
    ) -> Result<(), DonotSend>;
    fn on_app_msg_received(
        &mut self,
        session_id: &SessionId,
        message: &Message,
    ) -> Result<Vec<Message>, AppError>;

    // Unsolicited outbound seam. The engine polls this each timer tick; the app
    // returns any messages it wants sent on its own initiative (e.g. a streaming
    // market-data feed that keeps going after a single request). The default is
    // "nothing to send", so apps that only do request-response ignore it. The
    // app never calls the session — it just hands back messages when asked.
    fn poll_outbound(&mut self, _session_id: &SessionId) -> Vec<Message> {
        Vec::new()
    }
}

pub struct DefaultApplication;

impl DefaultApplication {
    pub fn new() -> Self {
        Self
    }
}

impl Application for DefaultApplication {
    fn on_create(&mut self, session_id: &SessionId) {}

    fn on_logon(&mut self, session_id: &SessionId) {}
    fn on_logout(&mut self, session_id: &SessionId) {}
    fn on_admin_msg_sending(&mut self, sesssion_id: &SessionId, message: &mut Message) {}
    fn on_admin_msg_received(
        &mut self,
        session_id: &SessionId,
        message: &Message,
    ) -> Result<(), RejectLogon> {
        Ok(())
    }
    fn on_app_msg_sending(
        &mut self,
        session_id: &SessionId,
        message: &mut Message,
    ) -> Result<(), DonotSend> {
        Ok(())
    }

    fn on_app_msg_received(
        &mut self,
        session_id: &SessionId,
        message: &Message,
    ) -> Result<Vec<Message>, AppError> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod application_tests {
    use super::*;

    #[test]
    fn test_default_application_callbacks_do_not_panic() {
        let mut app = DefaultApplication::new();
        let sid = SessionId::new("FIX.4.3", "SENDER", "TARGET");
        let mut msg = Message::new();

        app.on_create(&sid);
        app.on_logon(&sid);
        app.on_logout(&sid);
        app.on_admin_msg_sending(&sid, &mut msg);
        assert!(app.on_admin_msg_received(&sid, &msg).is_ok());
        assert!(app.on_app_msg_sending(&sid, &mut msg).is_ok());
        assert!(app.on_app_msg_received(&sid, &msg).is_ok());
    }
}
