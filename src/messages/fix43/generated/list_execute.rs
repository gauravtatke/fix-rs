//! `ListExecute` (app message, MsgType `35=L`): typed facade over the raw
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

/// Typed facade over a `ListExecute` [`Message`] (35=L). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct ListExecute {
    msg: Message,
}

impl Default for ListExecute {
    fn default() -> Self {
        ListExecute::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `ListExecute`, checking the msg type.
impl TryFrom<Message> for ListExecute {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a ListExecute" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<ListExecute> for Message {
    fn from(typed_msg: ListExecute) -> Self {
        typed_msg.msg
    }
}

impl ListExecute {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "L";

    /// An empty `ListExecute` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- ListID (tag 66, STRING, required) ---

    /// Set `ListID` (tag 66, required).
    pub fn set_list_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::LIST_ID, v.to_string());
    }

    /// `ListID` (tag 66, required).
    pub fn list_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::LIST_ID)
    }

    /// Whether `ListID` (tag 66) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_list_id(&self) -> bool {
        self.msg.body().has_field(tags::LIST_ID)
    }

    // --- ClientBidID (tag 391, STRING, optional) ---

    /// Set `ClientBidID` (tag 391, optional).
    pub fn set_client_bid_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::CLIENT_BID_ID, v.to_string());
    }

    /// `ClientBidID` (tag 391, optional).
    pub fn client_bid_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::CLIENT_BID_ID)
    }

    /// Whether `ClientBidID` (tag 391) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_client_bid_id(&self) -> bool {
        self.msg.body().has_field(tags::CLIENT_BID_ID)
    }

    // --- BidID (tag 390, STRING, optional) ---

    /// Set `BidID` (tag 390, optional).
    pub fn set_bid_id(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::BID_ID, v.to_string());
    }

    /// `BidID` (tag 390, optional).
    pub fn bid_id(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::BID_ID)
    }

    /// Whether `BidID` (tag 390) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_bid_id(&self) -> bool {
        self.msg.body().has_field(tags::BID_ID)
    }

    // --- TransactTime (tag 60, UTCTIMESTAMP, required) ---

    /// Set `TransactTime` (tag 60, required); written in FIX `UTCTIMESTAMP` format.
    pub fn set_transact_time(&mut self, v: DateTime<Utc>) {
        self.msg.body_mut().set_field(tags::TRANSACT_TIME, convert::format_fix_utc_timestamp(v));
    }

    /// `TransactTime` (tag 60, required). `Err(InvalidFormat)` unless a valid FIX
    /// `UTCTIMESTAMP`.
    pub fn transact_time(&self) -> Result<DateTime<Utc>, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TRANSACT_TIME)?;
        convert::parse_fix_utc_timestamp(&raw, tags::TRANSACT_TIME)
    }

    /// Whether `TransactTime` (tag 60) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_transact_time(&self) -> bool {
        self.msg.body().has_field(tags::TRANSACT_TIME)
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
