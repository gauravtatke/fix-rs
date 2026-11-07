//! `BusinessMessageReject` (admin message, MsgType `35=j`): typed facade over the raw
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

/// Typed facade over a `BusinessMessageReject` [`Message`] (35=j). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct BusinessMessageReject {
    msg: Message,
}

impl Default for BusinessMessageReject {
    fn default() -> Self {
        BusinessMessageReject::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `BusinessMessageReject`, checking the msg type.
impl TryFrom<Message> for BusinessMessageReject {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a BusinessMessageReject" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<BusinessMessageReject> for Message {
    fn from(typed_msg: BusinessMessageReject) -> Self {
        typed_msg.msg
    }
}

impl BusinessMessageReject {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "j";

    /// An empty `BusinessMessageReject` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- RefSeqNum (tag 45, SEQNUM, optional) ---

    /// Set `RefSeqNum` (tag 45, optional).
    pub fn set_ref_seq_num(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::REF_SEQ_NUM, v.to_string());
    }

    /// `RefSeqNum` (tag 45, optional).
    pub fn ref_seq_num(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::REF_SEQ_NUM)
    }

    /// Whether `RefSeqNum` (tag 45) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ref_seq_num(&self) -> bool {
        self.msg.body().has_field(tags::REF_SEQ_NUM)
    }

    // --- RefMsgType (tag 372, STRING, required) ---

    /// Set `RefMsgType` (tag 372, required).
    pub fn set_ref_msg_type(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::REF_MSG_TYPE, v.to_string());
    }

    /// `RefMsgType` (tag 372, required).
    pub fn ref_msg_type(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::REF_MSG_TYPE)
    }

    /// Whether `RefMsgType` (tag 372) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_ref_msg_type(&self) -> bool {
        self.msg.body().has_field(tags::REF_MSG_TYPE)
    }

    // --- BusinessRejectRefID (tag 379, STRING, optional) ---

    /// Set `BusinessRejectRefID` (tag 379, optional).
    pub fn set_business_reject_ref_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::BUSINESS_REJECT_REF_ID, v.to_string());
    }

    /// `BusinessRejectRefID` (tag 379, optional).
    pub fn business_reject_ref_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::BUSINESS_REJECT_REF_ID)
    }

    /// Whether `BusinessRejectRefID` (tag 379) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_business_reject_ref_id(&self) -> bool {
        self.msg.body().has_field(tags::BUSINESS_REJECT_REF_ID)
    }

    // --- BusinessRejectReason (tag 380, INT, required) ---

    /// Set `BusinessRejectReason` (tag 380, required).
    pub fn set_business_reject_reason(&mut self, v: fields::BusinessRejectReason) {
        self.msg.body_mut().set_field(tags::BUSINESS_REJECT_REASON, v.to_fix().to_string());
    }

    /// `BusinessRejectReason` (tag 380, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::BusinessRejectReason`]'s.
    pub fn business_reject_reason(&self) -> Result<fields::BusinessRejectReason, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::BUSINESS_REJECT_REASON)?;
        fields::BusinessRejectReason::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::BUSINESS_REJECT_REASON,
        })
    }

    /// Whether `BusinessRejectReason` (tag 380) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_business_reject_reason(&self) -> bool {
        self.msg.body().has_field(tags::BUSINESS_REJECT_REASON)
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
