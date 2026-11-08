//! `MarketDataRequestReject` (app message, MsgType `35=Y`): typed facade over the raw
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

/// Typed facade over a `MarketDataRequestReject` [`Message`] (35=Y). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct MarketDataRequestReject {
    msg: Message,
}

impl Default for MarketDataRequestReject {
    fn default() -> Self {
        MarketDataRequestReject::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `MarketDataRequestReject`, checking the msg type.
impl TryFrom<Message> for MarketDataRequestReject {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a MarketDataRequestReject" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<MarketDataRequestReject> for Message {
    fn from(typed_msg: MarketDataRequestReject) -> Self {
        typed_msg.msg
    }
}

impl MarketDataRequestReject {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "Y";

    /// An empty `MarketDataRequestReject` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- MDReqID (tag 262, STRING, required) ---

    /// Set `MDReqID` (tag 262, required).
    pub fn set_md_req_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::MD_REQ_ID, v.to_string());
    }

    /// `MDReqID` (tag 262, required).
    pub fn md_req_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::MD_REQ_ID)
    }

    /// Whether `MDReqID` (tag 262) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_req_id(&self) -> bool {
        self.msg.body().has_field(tags::MD_REQ_ID)
    }

    // --- MDReqRejReason (tag 281, CHAR, optional) ---

    /// Set `MDReqRejReason` (tag 281, optional).
    pub fn set_md_req_rej_reason(&mut self, v: fields::MdReqRejReason) {
        self.msg.body_mut().set_field(tags::MD_REQ_REJ_REASON, v.to_fix().to_string());
    }

    /// `MDReqRejReason` (tag 281, optional). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::MdReqRejReason`]'s.
    pub fn md_req_rej_reason(&self) -> Result<fields::MdReqRejReason, FieldError> {
        let n = self.msg.body().get_field::<char>(tags::MD_REQ_REJ_REASON)?;
        fields::MdReqRejReason::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::MD_REQ_REJ_REASON,
        })
    }

    /// Whether `MDReqRejReason` (tag 281) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_md_req_rej_reason(&self) -> bool {
        self.msg.body().has_field(tags::MD_REQ_REJ_REASON)
    }

    // --- Text (tag 58, STRING, optional) ---

    /// Set `Text` (tag 58, optional).
    pub fn set_text(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::TEXT, v.to_string());
    }

    /// `Text` (tag 58, optional).
    pub fn text(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::TEXT)
    }

    /// Whether `Text` (tag 58) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_text(&self) -> bool {
        self.msg.body().has_field(tags::TEXT)
    }

    // --- EncodedTextLen (tag 354, LENGTH, optional) ---

    /// Set `EncodedTextLen` (tag 354, optional).
    pub fn set_encoded_text_len(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::ENCODED_TEXT_LEN, v.to_string());
    }

    /// `EncodedTextLen` (tag 354, optional).
    pub fn encoded_text_len(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::ENCODED_TEXT_LEN)
    }

    /// Whether `EncodedTextLen` (tag 354) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text_len(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_TEXT_LEN)
    }

    // --- EncodedText (tag 355, DATA, optional) ---

    /// Set `EncodedText` (tag 355, optional).
    pub fn set_encoded_text(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::ENCODED_TEXT, v.to_string());
    }

    /// `EncodedText` (tag 355, optional).
    pub fn encoded_text(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::ENCODED_TEXT)
    }

    /// Whether `EncodedText` (tag 355) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encoded_text(&self) -> bool {
        self.msg.body().has_field(tags::ENCODED_TEXT)
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
