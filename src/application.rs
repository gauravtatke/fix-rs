#![allow(dead_code)]
#![allow(unused_imports)]

use crate::message::*;
use crate::quickfix_errors::{AppError, DonotSend, RejectLogon};
use crate::session::*;

// Naming convention: "to"/"from" describe direction relative to the counterparty,
// not the application. "App"/"Admin" is the message type.
//
// Outbound (engine → counterparty):
//   to_admin — hook before an admin message is sent; can modify (e.g. add credentials to Logon)
//   to_app   — hook before an app message is sent; can modify or cancel (DonotSend)
//
// Inbound (counterparty → engine → application):
//   from_admin — admin message received; can reject logon (RejectLogon)
//   from_app   — app message received; can reject (AppError)
pub trait Application: Send {
    fn on_create(&mut self, session_id: &SessionId);
    fn on_logon(&mut self, session_id: &SessionId);
    fn on_logout(&mut self, session_id: &SessionId);
    fn to_admin(&mut self, session_id: &SessionId, message: &mut Message);
    fn from_admin(&mut self, session_id: &SessionId, message: &Message) -> Result<(), RejectLogon>;

    fn to_app(&mut self, session_id: &SessionId, message: &mut Message) -> Result<(), DonotSend>;
    fn from_app(&mut self, session_id: &SessionId, message: &Message) -> Result<(), AppError>;
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
    fn to_admin(&mut self, sesssion_id: &SessionId, message: &mut Message) {}
    fn from_admin(&mut self, session_id: &SessionId, message: &Message) -> Result<(), RejectLogon> {
        Ok(())
    }
    fn to_app(&mut self, session_id: &SessionId, message: &mut Message) -> Result<(), DonotSend> {
        Ok(())
    }

    fn from_app(&mut self, session_id: &SessionId, message: &Message) -> Result<(), AppError> {
        Ok(())
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
        app.to_admin(&sid, &mut msg);
        assert!(app.from_admin(&sid, &msg).is_ok());
        assert!(app.to_app(&sid, &mut msg).is_ok());
        assert!(app.from_app(&sid, &msg).is_ok());
    }
}
