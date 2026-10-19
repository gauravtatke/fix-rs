//! `Heartbeat` (admin message, MsgType `35=0`): typed facade over the raw
//! [`Message`].
//!
//! **Generated — do not edit by hand.** Emitted by `cargo run -p codegen` from the FIX.4.3
//! data dictionary; `cargo test -p codegen` fails if this file drifts from the generator's output.

// Every generated message imports the same set; not every message uses all of it.
#![allow(unused_imports)]

use super::fields;
use crate::convert;
use crate::core::message::{FieldMap, Group, Message};
use crate::errors::{FieldError, TypedError};
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

/// Inbound seam: adopt a raw `Message` as a typed `Heartbeat`, checking the msg type.
impl TryFrom<Message> for Heartbeat {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a Heartbeat" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<Heartbeat> for Message {
    fn from(typed_msg: Heartbeat) -> Self {
        typed_msg.msg
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
        self.msg.body_mut().set_field(tags::TEST_REQ_ID, v.to_string());
    }

    /// `TestReqID` (tag 112, optional).
    pub fn test_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TEST_REQ_ID)
    }

    /// Whether `TestReqID` (tag 112) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_test_req_id(&self) -> bool {
        self.msg.body().has_field(tags::TEST_REQ_ID)
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
