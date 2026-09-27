//! `Logon` (35=A) — hand-written reference typed message (task 3.1).
//!
//! Scalar fields only. The optional `NoMsgTypes` (384) repeating group is deferred to
//! the groups work (task 3.7), which settles the D9 zero-copy view shape.

use super::fields::EncryptMethod;
use crate::convert::{fix_bool_str, parse_fix_bool};
use crate::core::message::Message;
use crate::errors::{FieldError, TypedError};
// Tags come from the shared, version-neutral registry (crate::tags) — a tag number means
// the same field in every FIX version. MsgType (35) lives in the header.
use crate::tags::{
    ENCRYPT_METHOD, HEART_BT_INT, MAX_MESSAGE_SIZE, PASSWORD, RAW_DATA, RAW_DATA_LENGTH,
    RESET_SEQ_NUM_FLAG, TEST_MESSAGE_INDICATOR, USERNAME,
};

/// Typed facade over a Logon `Message` (D1). Construction stamps `35=A`; the engine
/// fills the rest of the header (BeginString/CompIDs/SeqNum/SendingTime) and the
/// trailer at send time.
#[derive(Debug, Clone)]
pub struct Logon {
    message: Message,
}

impl Logon {
    pub const MSG_TYPE: &'static str = "A";

    pub fn new() -> Self {
        let mut message = Message::new();
        message.set_header_field(35, Self::MSG_TYPE);
        Self { message }
    }

    // --- required fields ---

    pub fn set_encrypt_method(&mut self, v: EncryptMethod) {
        self.message.set_body_field(ENCRYPT_METHOD, v.to_fix().to_string());
    }
    pub fn encrypt_method(&self) -> Result<EncryptMethod, FieldError> {
        let n = self.message.get_body_field::<i32>(ENCRYPT_METHOD)?;
        EncryptMethod::from_fix(n).ok_or(FieldError::InvalidFormat {
            tag: ENCRYPT_METHOD,
        })
    }

    pub fn set_heart_bt_int(&mut self, v: i32) {
        self.message.set_body_field(HEART_BT_INT, v.to_string());
    }
    pub fn heart_bt_int(&self) -> Result<i32, FieldError> {
        self.message.get_body_field::<i32>(HEART_BT_INT)
    }

    // --- optional fields (each with a has_* presence check) ---

    pub fn set_reset_seq_num_flag(&mut self, v: bool) {
        self.message.set_body_field(RESET_SEQ_NUM_FLAG, fix_bool_str(v));
    }
    pub fn reset_seq_num_flag(&self) -> Result<bool, FieldError> {
        let raw = self.message.get_body_field::<String>(RESET_SEQ_NUM_FLAG)?;
        parse_fix_bool(&raw, RESET_SEQ_NUM_FLAG)
    }
    pub fn has_reset_seq_num_flag(&self) -> bool {
        self.message.get_body_field::<String>(RESET_SEQ_NUM_FLAG).is_ok()
    }

    pub fn set_max_message_size(&mut self, v: u32) {
        self.message.set_body_field(MAX_MESSAGE_SIZE, v.to_string());
    }
    pub fn max_message_size(&self) -> Result<u32, FieldError> {
        self.message.get_body_field::<u32>(MAX_MESSAGE_SIZE)
    }
    pub fn has_max_message_size(&self) -> bool {
        self.message.get_body_field::<String>(MAX_MESSAGE_SIZE).is_ok()
    }

    pub fn set_raw_data_length(&mut self, v: u32) {
        self.message.set_body_field(RAW_DATA_LENGTH, v.to_string());
    }
    pub fn raw_data_length(&self) -> Result<u32, FieldError> {
        self.message.get_body_field::<u32>(RAW_DATA_LENGTH)
    }
    pub fn has_raw_data_length(&self) -> bool {
        self.message.get_body_field::<String>(RAW_DATA_LENGTH).is_ok()
    }

    pub fn set_raw_data(&mut self, v: impl Into<String>) {
        self.message.set_body_field(RAW_DATA, v);
    }
    pub fn raw_data(&self) -> Result<String, FieldError> {
        self.message.get_body_field::<String>(RAW_DATA)
    }
    pub fn has_raw_data(&self) -> bool {
        self.message.get_body_field::<String>(RAW_DATA).is_ok()
    }

    pub fn set_test_message_indicator(&mut self, v: bool) {
        self.message.set_body_field(TEST_MESSAGE_INDICATOR, fix_bool_str(v));
    }
    pub fn test_message_indicator(&self) -> Result<bool, FieldError> {
        let raw = self.message.get_body_field::<String>(TEST_MESSAGE_INDICATOR)?;
        parse_fix_bool(&raw, TEST_MESSAGE_INDICATOR)
    }
    pub fn has_test_message_indicator(&self) -> bool {
        self.message.get_body_field::<String>(TEST_MESSAGE_INDICATOR).is_ok()
    }

    pub fn set_username(&mut self, v: impl Into<String>) {
        self.message.set_body_field(USERNAME, v);
    }
    pub fn username(&self) -> Result<String, FieldError> {
        self.message.get_body_field::<String>(USERNAME)
    }
    pub fn has_username(&self) -> bool {
        self.message.get_body_field::<String>(USERNAME).is_ok()
    }

    pub fn set_password(&mut self, v: impl Into<String>) {
        self.message.set_body_field(PASSWORD, v);
    }
    pub fn password(&self) -> Result<String, FieldError> {
        self.message.get_body_field::<String>(PASSWORD)
    }
    pub fn has_password(&self) -> bool {
        self.message.get_body_field::<String>(PASSWORD).is_ok()
    }

    // --- raw escape hatch (D1a) ---
    // Typed accessors above are the primary surface; drop to the raw `Message` only for
    // custom/unmodelled fields. Reaching `message_mut()` is a deliberate hop, which keeps
    // the generic `set_field` out of the way in autocomplete.

    pub fn message(&self) -> &Message {
        &self.message
    }
    pub fn message_mut(&mut self) -> &mut Message {
        &mut self.message
    }
}

impl Default for Logon {
    fn default() -> Self {
        Self::new()
    }
}

/// Inbound seam: adopt a raw `Message` as a typed `Logon`, checking the msg type.
impl TryFrom<Message> for Logon {
    type Error = TypedError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message.get_msg_type() {
            Ok(mt) if mt == Self::MSG_TYPE => Ok(Self { message }),
            Ok(got) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got,
            }),
            // No/blank MsgType is still "not a Logon" from this facade's POV.
            Err(_) => Err(TypedError::WrongMsgType {
                expected: Self::MSG_TYPE,
                got: String::new(),
            }),
        }
    }
}

/// Outbound seam: hand the underlying `Message` back to the engine to send.
impl From<Logon> for Message {
    fn from(logon: Logon) -> Self {
        logon.message
    }
}

#[cfg(test)]
mod logon_tests {
    use super::*;

    #[test]
    fn new_stamps_msgtype_a() {
        let logon = Logon::new();
        assert_eq!(logon.message().get_msg_type().unwrap(), "A");
    }

    #[test]
    fn set_get_roundtrip_scalar_and_enum() {
        let mut logon = Logon::new();
        logon.set_encrypt_method(EncryptMethod::None);
        logon.set_heart_bt_int(30);
        assert_eq!(logon.encrypt_method().unwrap(), EncryptMethod::None);
        assert_eq!(logon.heart_bt_int().unwrap(), 30);
    }

    // The enum's on-wire form is the FIX integer, not the Rust variant name.
    #[test]
    fn encrypt_method_maps_to_fix_int_on_the_wire() {
        let mut logon = Logon::new();
        logon.set_encrypt_method(EncryptMethod::PemDesMd5); // FIX value 6
        assert_eq!(logon.message().get_body_field::<String>(98).unwrap(), "6");
        assert_eq!(logon.encrypt_method().unwrap(), EncryptMethod::PemDesMd5);
    }

    // A value outside the FIX enum is a malformed field, not a silent default.
    #[test]
    fn unknown_encrypt_method_value_is_invalid_format() {
        let mut logon = Logon::new();
        logon.message_mut().set_body_field(98, "9");
        assert!(matches!(logon.encrypt_method(), Err(FieldError::InvalidFormat { tag: 98 })));
    }

    // FIX booleans are Y/N on the wire, never true/false.
    #[test]
    fn bool_field_uses_fix_y_n() {
        let mut logon = Logon::new();
        logon.set_reset_seq_num_flag(true);
        assert_eq!(logon.message().get_body_field::<String>(141).unwrap(), "Y");
        assert!(logon.reset_seq_num_flag().unwrap());
        logon.set_reset_seq_num_flag(false);
        assert_eq!(logon.message().get_body_field::<String>(141).unwrap(), "N");
        assert!(!logon.reset_seq_num_flag().unwrap());
    }

    #[test]
    fn malformed_bool_is_invalid_format() {
        let mut logon = Logon::new();
        logon.message_mut().set_body_field(141, "X");
        assert!(matches!(logon.reset_seq_num_flag(), Err(FieldError::InvalidFormat { tag: 141 })));
    }

    #[test]
    fn has_reports_optional_field_presence() {
        let mut logon = Logon::new();
        assert!(!logon.has_username());
        logon.set_username("trader1");
        assert!(logon.has_username());
        assert_eq!(logon.username().unwrap(), "trader1");
    }

    #[test]
    fn missing_required_field_is_tag_not_found() {
        let logon = Logon::new();
        assert!(matches!(logon.heart_bt_int(), Err(FieldError::TagNotFound { tag: 108 })));
    }

    #[test]
    fn try_from_accepts_matching_msgtype() {
        let mut msg = Message::new();
        msg.set_header_field(35, "A");
        msg.set_body_field(98, "0");
        msg.set_body_field(108, "30");
        let logon = Logon::try_from(msg).unwrap();
        assert_eq!(logon.encrypt_method().unwrap(), EncryptMethod::None);
        assert_eq!(logon.heart_bt_int().unwrap(), 30);
    }

    #[test]
    fn try_from_rejects_wrong_msgtype() {
        let mut msg = Message::new();
        msg.set_header_field(35, "0"); // Heartbeat, not Logon
        let err = Logon::try_from(msg).unwrap_err();
        assert_eq!(
            err,
            TypedError::WrongMsgType {
                expected: "A",
                got: "0".to_string()
            }
        );
    }

    #[test]
    fn into_message_returns_inner_with_fields() {
        let mut logon = Logon::new();
        logon.set_encrypt_method(EncryptMethod::None);
        logon.set_heart_bt_int(30);
        let msg: Message = logon.into();
        assert_eq!(msg.get_msg_type().unwrap(), "A");
        assert_eq!(msg.get_body_field::<i32>(108).unwrap(), 30);
    }
}
