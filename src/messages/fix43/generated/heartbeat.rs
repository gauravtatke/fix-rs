//! `Heartbeat` (admin message, MsgType `35=0`): typed facade over the raw
//! [`Message`].
//!
//! **Generated — do not edit by hand.** Emitted by `cargo run -p codegen` from the FIX.4.3
//! data dictionary; `cargo test -p codegen` fails if this file drifts from the generator's output.

// Every generated message imports the same set; not every message uses all of it.
#![allow(unused_imports)]

use super::fields;
use crate::convert;
use crate::core::message::Message;
use crate::errors::FieldError;
// Tags come from the shared, version-neutral registry (`crate::tags`): a tag number means the
// same field in every FIX version. MsgType (35) lives in the header.
use crate::tags;

/// Typed facade over a `Heartbeat` [`Message`] (35=0). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct Heartbeat {
    msg: Message,
}

impl Default for Heartbeat {
    fn default() -> Self {
        Heartbeat::new()
    }
}

impl Heartbeat {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "0";

    /// An empty `Heartbeat` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- TestReqID (tag 112, STRING, optional) ---

    /// Set `TestReqID` (tag 112, optional).
    pub fn set_test_req_id(&mut self, v: &str) {
        self.msg.set_body_field(tags::TEST_REQ_ID, v.to_string());
    }

    /// `TestReqID` (tag 112, optional).
    pub fn test_req_id(&self) -> Result<String, FieldError> {
        self.msg.get_body_field::<String>(tags::TEST_REQ_ID)
    }

    /// Whether `TestReqID` (tag 112) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_test_req_id(&self) -> bool {
        self.msg.has_body_field(tags::TEST_REQ_ID)
    }

    /// The underlying raw message (e.g. for fields this facade has no accessor for).
    pub fn message(&self) -> &Message {
        &self.msg
    }

    /// Mutable access to the underlying raw message (e.g. to set a custom field, D1a).
    pub fn message_mut(&mut self) -> &mut Message {
        &mut self.msg
    }
}
