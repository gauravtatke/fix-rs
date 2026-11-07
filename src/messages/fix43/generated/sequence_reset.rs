//! `SequenceReset` (admin message, MsgType `35=4`): typed facade over the raw
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

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use rust_decimal::Decimal;

/// Typed facade over a `SequenceReset` [`Message`] (35=4). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct SequenceReset {
    msg: Message,
}

impl Default for SequenceReset {
    fn default() -> Self {
        SequenceReset::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `SequenceReset`, checking the msg type.
impl TryFrom<Message> for SequenceReset {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a SequenceReset" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<SequenceReset> for Message {
    fn from(typed_msg: SequenceReset) -> Self {
        typed_msg.msg
    }
}

impl SequenceReset {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "4";

    /// An empty `SequenceReset` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- GapFillFlag (tag 123, BOOLEAN, optional) ---

    /// Set `GapFillFlag` (tag 123, optional); written in FIX `BOOLEAN` format.
    pub fn set_gap_fill_flag(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::GAP_FILL_FLAG, convert::format_fix_bool(v));
    }

    /// `GapFillFlag` (tag 123, optional). `Err(InvalidFormat)` unless a valid FIX
    /// `BOOLEAN`.
    pub fn gap_fill_flag(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::GAP_FILL_FLAG)?;
        convert::parse_fix_bool(&raw, tags::GAP_FILL_FLAG)
    }

    /// Whether `GapFillFlag` (tag 123) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_gap_fill_flag(&self) -> bool {
        self.msg.body().has_field(tags::GAP_FILL_FLAG)
    }

    // --- NewSeqNo (tag 36, SEQNUM, required) ---

    /// Set `NewSeqNo` (tag 36, required).
    pub fn set_new_seq_no(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::NEW_SEQ_NO, v.to_string());
    }

    /// `NewSeqNo` (tag 36, required).
    pub fn new_seq_no(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::NEW_SEQ_NO)
    }

    /// Whether `NewSeqNo` (tag 36) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_new_seq_no(&self) -> bool {
        self.msg.body().has_field(tags::NEW_SEQ_NO)
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
