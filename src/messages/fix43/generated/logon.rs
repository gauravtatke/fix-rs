//! `Logon` (admin message, MsgType `35=A`): typed facade over the raw
//! [`Message`].
//!
//! **Generated — do not edit by hand.** Emitted by `cargo run -p codegen` from the FIX.4.3
//! data dictionary; `cargo test -p codegen` fails if this file drifts from the generator's output.
//!
//! Not generated yet (repeating groups come later): `NoMsgTypes`.
//! Until then, reach them through [`Logon::message`] / [`Logon::message_mut`].

// Every generated message imports the same set; not every message uses all of it.
#![allow(unused_imports)]

use super::fields;
use crate::convert;
use crate::core::message::Message;
use crate::errors::{FieldError, TypedError};
// Tags come from the shared, version-neutral registry (`crate::tags`): a tag number means the
// same field in every FIX version. MsgType (35) lives in the header.
use crate::tags;

/// Typed facade over a `Logon` [`Message`] (35=A). `new()` stamps the MsgType;
/// the engine fills the rest of the header (BeginString, CompIDs, MsgSeqNum, SendingTime) and the
/// trailer when the message is sent.
///
/// Per field: `set_x(..)` writes it, `x()` reads it (`Err(TagNotFound)` if absent,
/// `Err(InvalidFormat)` if present but not a valid value), `has_x()` checks presence only.
#[derive(Debug, Clone)]
pub struct Logon {
    msg: Message,
}

impl Default for Logon {
    fn default() -> Self {
        Logon::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `Logon`, checking the msg type.
impl TryFrom<Message> for Logon {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { msg: message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // Missing MsgType is still "not a Logon" from this facade's POV. (A blank `35=`
            // parses as `Ok("")` and lands in the arm above, with the same `got: ""`.)
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<Logon> for Message {
    fn from(typed_msg: Logon) -> Self {
        typed_msg.msg
    }
}

impl Logon {
    /// This message's MsgType (tag 35) value.
    pub const MSG_TYPE: &'static str = "A";

    /// An empty `Logon` with only MsgType (35) set.
    pub fn new() -> Self {
        let mut msg = Message::new();
        msg.set_header_field(35, Self::MSG_TYPE);
        Self { msg }
    }

    // --- EncryptMethod (tag 98, INT, required) ---

    /// Set `EncryptMethod` (tag 98, required).
    pub fn set_encrypt_method(&mut self, v: fields::EncryptMethod) {
        self.msg.body_mut().set_field(tags::ENCRYPT_METHOD, v.to_fix().to_string());
    }

    /// `EncryptMethod` (tag 98, required). `Err(InvalidFormat)` if the value isn't one
    /// of [`fields::EncryptMethod`]'s.
    pub fn encrypt_method(&self) -> Result<fields::EncryptMethod, FieldError> {
        let n = self.msg.body().get_field::<i32>(tags::ENCRYPT_METHOD)?;
        fields::EncryptMethod::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: tags::ENCRYPT_METHOD,
        })
    }

    /// Whether `EncryptMethod` (tag 98) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_encrypt_method(&self) -> bool {
        self.msg.body().has_field(tags::ENCRYPT_METHOD)
    }

    // --- HeartBtInt (tag 108, INT, required) ---

    /// Set `HeartBtInt` (tag 108, required).
    pub fn set_heart_bt_int(&mut self, v: i32) {
        self.msg.body_mut().set_field(tags::HEART_BT_INT, v.to_string());
    }

    /// `HeartBtInt` (tag 108, required).
    pub fn heart_bt_int(&self) -> Result<i32, FieldError> {
        self.msg.body().get_field::<i32>(tags::HEART_BT_INT)
    }

    /// Whether `HeartBtInt` (tag 108) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_heart_bt_int(&self) -> bool {
        self.msg.body().has_field(tags::HEART_BT_INT)
    }

    // --- RawDataLength (tag 95, LENGTH, optional) ---

    /// Set `RawDataLength` (tag 95, optional).
    pub fn set_raw_data_length(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::RAW_DATA_LENGTH, v.to_string());
    }

    /// `RawDataLength` (tag 95, optional).
    pub fn raw_data_length(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::RAW_DATA_LENGTH)
    }

    /// Whether `RawDataLength` (tag 95) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_raw_data_length(&self) -> bool {
        self.msg.body().has_field(tags::RAW_DATA_LENGTH)
    }

    // --- RawData (tag 96, DATA, optional) ---

    /// Set `RawData` (tag 96, optional).
    pub fn set_raw_data(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::RAW_DATA, v.to_string());
    }

    /// `RawData` (tag 96, optional).
    pub fn raw_data(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::RAW_DATA)
    }

    /// Whether `RawData` (tag 96) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_raw_data(&self) -> bool {
        self.msg.body().has_field(tags::RAW_DATA)
    }

    // --- ResetSeqNumFlag (tag 141, BOOLEAN, optional) ---

    /// Set `ResetSeqNumFlag` (tag 141, optional); written as `Y`/`N`.
    pub fn set_reset_seq_num_flag(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::RESET_SEQ_NUM_FLAG, convert::fix_bool_str(v));
    }

    /// `ResetSeqNumFlag` (tag 141, optional). `Err(InvalidFormat)` unless `Y`/`N`.
    pub fn reset_seq_num_flag(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::RESET_SEQ_NUM_FLAG)?;
        convert::parse_fix_bool(&raw, tags::RESET_SEQ_NUM_FLAG)
    }

    /// Whether `ResetSeqNumFlag` (tag 141) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_reset_seq_num_flag(&self) -> bool {
        self.msg.body().has_field(tags::RESET_SEQ_NUM_FLAG)
    }

    // --- MaxMessageSize (tag 383, LENGTH, optional) ---

    /// Set `MaxMessageSize` (tag 383, optional).
    pub fn set_max_message_size(&mut self, v: u32) {
        self.msg.body_mut().set_field(tags::MAX_MESSAGE_SIZE, v.to_string());
    }

    /// `MaxMessageSize` (tag 383, optional).
    pub fn max_message_size(&self) -> Result<u32, FieldError> {
        self.msg.body().get_field::<u32>(tags::MAX_MESSAGE_SIZE)
    }

    /// Whether `MaxMessageSize` (tag 383) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_max_message_size(&self) -> bool {
        self.msg.body().has_field(tags::MAX_MESSAGE_SIZE)
    }

    // --- TestMessageIndicator (tag 464, BOOLEAN, optional) ---

    /// Set `TestMessageIndicator` (tag 464, optional); written as `Y`/`N`.
    pub fn set_test_message_indicator(&mut self, v: bool) {
        self.msg.body_mut().set_field(tags::TEST_MESSAGE_INDICATOR, convert::fix_bool_str(v));
    }

    /// `TestMessageIndicator` (tag 464, optional). `Err(InvalidFormat)` unless `Y`/`N`.
    pub fn test_message_indicator(&self) -> Result<bool, FieldError> {
        let raw = self.msg.body().get_field::<String>(tags::TEST_MESSAGE_INDICATOR)?;
        convert::parse_fix_bool(&raw, tags::TEST_MESSAGE_INDICATOR)
    }

    /// Whether `TestMessageIndicator` (tag 464) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_test_message_indicator(&self) -> bool {
        self.msg.body().has_field(tags::TEST_MESSAGE_INDICATOR)
    }

    // --- Username (tag 553, STRING, optional) ---

    /// Set `Username` (tag 553, optional).
    pub fn set_username(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::USERNAME, v.to_string());
    }

    /// `Username` (tag 553, optional).
    pub fn username(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::USERNAME)
    }

    /// Whether `Username` (tag 553) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_username(&self) -> bool {
        self.msg.body().has_field(tags::USERNAME)
    }

    // --- Password (tag 554, STRING, optional) ---

    /// Set `Password` (tag 554, optional).
    pub fn set_password(&mut self, v: &str) {
        self.msg.body_mut().set_field(tags::PASSWORD, v.to_string());
    }

    /// `Password` (tag 554, optional).
    pub fn password(&self) -> Result<String, FieldError> {
        self.msg.body().get_field::<String>(tags::PASSWORD)
    }

    /// Whether `Password` (tag 554) is set. Presence only: a set but malformed
    /// value still counts.
    pub fn has_password(&self) -> bool {
        self.msg.body().has_field(tags::PASSWORD)
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
