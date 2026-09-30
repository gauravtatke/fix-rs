//! Hand-written tests for the generated `Logon` facade (`super::super::generated::logon`).

use crate::core::message::Message;
use crate::errors::{FieldError, TypedError};
use crate::messages::fix43::fields::EncryptMethod;
use crate::messages::fix43::logon::Logon;

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
