//! Hand-written tests for the generated `Logon` facade (`super::super::generated::logon`).

use crate::core::dictionary::DataDictionary;
use crate::core::message::{Message, SOH};
use crate::errors::{FieldError, TypedError};
use crate::messages::fix43::fields::{EncryptMethod, MsgDirection};
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
    logon.set_encrypt_method(EncryptMethod::Pem); // FIX value 6
    assert_eq!(logon.message().get_body_field::<String>(98).unwrap(), "6");
    assert_eq!(logon.encrypt_method().unwrap(), EncryptMethod::Pem);
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

// Missing 35 fails `get_msg_type` (the `Err` arm); blank `35=` parses as `Ok("")` (the mismatch
// arm). Different arms, same outcome — pinned so a refactor of the match can't split them.
#[test]
fn try_from_rejects_missing_msgtype() {
    let mut msg = Message::new();
    msg.set_body_field(108, "30");
    let err = Logon::try_from(msg).unwrap_err();
    assert_eq!(
        err,
        TypedError::WrongMsgType {
            expected: "A",
            got: String::new()
        }
    );
}

#[test]
fn try_from_rejects_blank_msgtype() {
    let mut msg = Message::new();
    msg.set_header_field(35, "");
    let err = Logon::try_from(msg).unwrap_err();
    assert_eq!(
        err,
        TypedError::WrongMsgType {
            expected: "A",
            got: String::new()
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

// ---- NoMsgTypes(384) repeating group: RefMsgType(372, delimiter, STRING), MsgDirection(385, CHAR enum)
// Classes covered: build by `add_*_entry` vs by `set_*(n)` + index; read by get / index / iter;
// absent vs present-but-empty group; edit through the mut views; wire order through the generated
// NO_MSG_TYPES_FIELD_ORDER (setters scrambled, delimiter last); the parsed path via TryFrom; and
// field-level errors inside an entry (absent field, value outside the enum).

/// Tags of `msg` in wire order, after stamping the header/trailer the engine would add.
fn wire_tags(msg: Message) -> Vec<u32> {
    let mut msg = msg;
    msg.set_header_field(8, "FIX.4.3");
    msg.set_body_len();
    msg.set_checksum();
    msg.to_string()
        .split(SOH)
        .filter(|f| !f.is_empty())
        .map(|f| f.split_once('=').unwrap().0.parse().unwrap())
        .collect()
}

/// A Logon with two NoMsgTypes entries, built with the streaming `add_*_entry`.
fn logon_with_two_msg_types() -> Logon {
    let mut logon = Logon::new();
    logon.add_no_msg_types_entry(|e| {
        e.set_ref_msg_type("D");
        e.set_msg_direction(MsgDirection::Send);
    });
    logon.add_no_msg_types_entry(|e| {
        e.set_ref_msg_type("8");
        e.set_msg_direction(MsgDirection::Receive);
    });
    logon
}

#[test]
fn group_absent_reads_as_none() {
    let mut logon = Logon::new();
    assert!(logon.no_msg_types().is_none());
    assert!(logon.no_msg_types_mut().is_none());
}

#[test]
fn add_entry_builds_group_and_reads_back_in_order() {
    let logon = logon_with_two_msg_types();
    let g = logon.no_msg_types().expect("group created by add_*_entry");
    assert_eq!(g.len(), 2);
    assert!(!g.is_empty());
    let read: Vec<(String, MsgDirection)> =
        g.iter().map(|e| (e.ref_msg_type().unwrap(), e.msg_direction().unwrap())).collect();
    assert_eq!(
        read,
        vec![
            ("D".to_string(), MsgDirection::Send),
            ("8".to_string(), MsgDirection::Receive)
        ]
    );
    // The count field (384) tracks the entries.
    assert_eq!(logon.message().get_body_field::<u32>(384).unwrap(), 2);
}

#[test]
fn set_n_then_fill_by_index() {
    let mut logon = Logon::new();
    {
        let mut g = logon.set_no_msg_types(2);
        assert_eq!(g.len(), 2);
        g.index_mut(0).set_ref_msg_type("D");
        g.index_mut(1).set_ref_msg_type("8");
        g.index_mut(1).set_msg_direction(MsgDirection::Receive);
    }
    let g = logon.no_msg_types().unwrap();
    assert_eq!(g.index(0).ref_msg_type().unwrap(), "D");
    assert!(!g.index(0).has_msg_direction()); // never set on entry 0
    assert_eq!(g.index(1).msg_direction().unwrap(), MsgDirection::Receive);
    assert_eq!(logon.message().get_body_field::<u32>(384).unwrap(), 2);
}

// Present but empty is distinct from absent: Some(view) with no entries.
#[test]
fn set_zero_is_present_but_empty() {
    let mut logon = Logon::new();
    logon.set_no_msg_types(0);
    let g = logon.no_msg_types().expect("group exists");
    assert!(g.is_empty());
    assert_eq!(g.len(), 0);
    assert!(g.iter().next().is_none());
}

#[test]
fn get_out_of_range_is_none() {
    let logon = logon_with_two_msg_types();
    let g = logon.no_msg_types().unwrap();
    assert!(g.get(1).is_some()); // last valid index
    assert!(g.get(2).is_none()); // first invalid index
}

#[test]
#[should_panic(expected = "NoMsgTypes index out of range")]
fn index_out_of_range_panics() {
    let logon = logon_with_two_msg_types();
    logon.no_msg_types().unwrap().index(2);
}

#[test]
#[should_panic(expected = "NoMsgTypes index out of range")]
fn index_mut_out_of_range_panics() {
    let mut logon = logon_with_two_msg_types();
    logon.no_msg_types_mut().unwrap().index_mut(2);
}

#[test]
fn get_mut_out_of_range_is_none() {
    let mut logon = logon_with_two_msg_types();
    assert!(logon.no_msg_types_mut().unwrap().get_mut(2).is_none());
}

// Edit an existing group in place: one entry by index, then every entry via iter_mut; read
// back through as_ref (the mut view's read path) and through the read-only view.
#[test]
fn mut_views_edit_existing_entries() {
    let mut logon = logon_with_two_msg_types();
    {
        let mut g = logon.no_msg_types_mut().unwrap();
        let mut e = g.get_mut(0).unwrap();
        e.set_ref_msg_type("F");
        assert_eq!(e.as_ref().ref_msg_type().unwrap(), "F");
        for mut e in g.iter_mut() {
            e.set_msg_direction(MsgDirection::Receive);
        }
    }
    let g = logon.no_msg_types().unwrap();
    assert_eq!(g.index(0).ref_msg_type().unwrap(), "F");
    assert_eq!(g.index(1).ref_msg_type().unwrap(), "8"); // untouched
    assert!(g.iter().all(|e| e.msg_direction().unwrap() == MsgDirection::Receive));
}

// Wire order comes from the generated NO_MSG_TYPES_FIELD_ORDER: setters called delimiter-last,
// on both build paths, still serialize 384, then 372 (delimiter) before 385 in every entry.
#[test]
fn wire_order_is_dictionary_order_for_both_build_paths() {
    let mut by_add = Logon::new();
    by_add.add_no_msg_types_entry(|e| {
        e.set_msg_direction(MsgDirection::Send);
        e.set_ref_msg_type("D");
    });
    let mut by_set = Logon::new();
    {
        let mut g = by_set.set_no_msg_types(1);
        let mut e = g.index_mut(0);
        e.set_msg_direction(MsgDirection::Send);
        e.set_ref_msg_type("D");
    }
    for logon in [by_add, by_set] {
        assert_eq!(wire_tags(logon.into()), vec![8, 9, 35, 384, 372, 385, 10]);
    }
}

// Parsed path: a Logon received off the wire exposes its group through the facade.
#[test]
fn parsed_logon_group_reads_through_facade() {
    let dd = DataDictionary::from_xml("resources/FIX43.xml");
    let raw = "8=FIX.4.3|9=102|35=A|34=1|49=BANZAI|52=20221006-08:43:36.522|56=FIXIMULATOR|98=0|108=30|384=2|372=D|385=S|372=8|385=R|10=207|"
        .replace('|', &SOH.to_string());
    let msg = Message::from_str(&raw, &dd).expect("fixture parses");
    let logon = Logon::try_from(msg).unwrap();
    let g = logon.no_msg_types().unwrap();
    assert_eq!(g.len(), 2);
    assert_eq!(g.index(1).ref_msg_type().unwrap(), "8");
    assert_eq!(g.index(1).msg_direction().unwrap(), MsgDirection::Receive);
}

// Field errors inside an entry behave as on the message: absent -> TagNotFound, a value outside
// the enum -> InvalidFormat (set raw through the underlying message, as a bad counterparty would).
#[test]
fn entry_field_errors_match_message_field_errors() {
    let mut logon = Logon::new();
    logon.add_no_msg_types_entry(|e| e.set_ref_msg_type("D"));
    assert!(matches!(
        logon.no_msg_types().unwrap().index(0).msg_direction(),
        Err(FieldError::TagNotFound { tag: 385 })
    ));
    logon.message_mut().body_mut().get_group_mut(384).unwrap()[0].set_field(385, "X");
    assert!(matches!(
        logon.no_msg_types().unwrap().index(0).msg_direction(),
        Err(FieldError::InvalidFormat { tag: 385 })
    ));
}
