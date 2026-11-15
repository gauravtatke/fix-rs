//! Hand-written tests for the generated `Fix43Message` enum
//! (`super::super::generated::fix43_message`): the factory/cracker over every FIX 4.3 message.
//!
//! The oracle for "which variant" is the dictionary XML itself, read here with roxmltree rather
//! than through the generator or the runtime `DataDictionary`: every `<message name msgtype>` must
//! land in the variant named `name`. Variant names are read from the derived `Debug` output
//! (`NewOrderSingle(NewOrderSingle { .. })`), so the test needs no hand-kept 67-arm match.

use super::wire_roundtrip;
use crate::core::message::Message;
use crate::messages::fix43::Fix43Message;
use crate::messages::fix43::fields::{EncryptMethod, PartyIdSource, Side};
use crate::messages::fix43::logon::Logon;
use crate::messages::fix43::new_order_single::NewOrderSingle;

/// `(name, msgtype)` of every `<message>` in the FIX 4.3 dictionary, in document order.
fn dictionary_messages() -> Vec<(String, String)> {
    let xml = std::fs::read_to_string("resources/FIX43.xml").unwrap();
    let doc = roxmltree::Document::parse(&xml).unwrap();
    doc.descendants()
        .filter(|n| n.has_tag_name("message"))
        .map(|n| {
            (n.attribute("name").unwrap().to_owned(), n.attribute("msgtype").unwrap().to_owned())
        })
        .collect()
}

/// The variant's name: derived `Debug` prints a tuple variant as `Name(..)`.
fn variant_name(m: &Fix43Message) -> String {
    let dbg = format!("{m:?}");
    dbg[..dbg.find('(').unwrap()].to_owned()
}

/// A bare message with only MsgType set (or none, for `None`).
fn raw(msg_type: Option<&str>) -> Message {
    let mut msg = Message::new();
    if let Some(mt) = msg_type {
        msg.set_header_field(35, mt);
    }
    msg
}

// ---- Known MsgTypes: each lands in its own variant ----

// Every message the dictionary defines, admin and app, one- and two-character codes, maps to the
// variant of the same name. Catches a missing arm (would be `Unknown`) and a crossed arm (pattern
// of one message, variant of another).
#[test]
fn every_dictionary_msg_type_maps_to_its_variant() {
    let messages = dictionary_messages();
    assert_eq!(messages.len(), 67);
    for (name, msg_type) in messages {
        let typed = Fix43Message::from(raw(Some(&msg_type)));
        assert_eq!(variant_name(&typed), name, "35={msg_type}");
    }
}

// The variant holds a working facade over the same message: typed accessors read what was set,
// including a repeating group.
#[test]
fn variant_payload_is_the_facade_over_the_same_message() {
    let mut nos = NewOrderSingle::new();
    nos.set_cl_ord_id("ORD-1");
    nos.set_side(Side::Buy);
    nos.add_no_party_ids_entry(|p| {
        p.set_party_id("TRADER-7");
        p.set_party_id_source(PartyIdSource::GenerallyAcceptedMarketParticipantIdentifier);
    });

    let Fix43Message::NewOrderSingle(nos) = Fix43Message::from(Message::from(nos)) else {
        panic!("35=D should crack to NewOrderSingle");
    };
    assert_eq!(nos.cl_ord_id().unwrap(), "ORD-1");
    assert_eq!(nos.side().unwrap(), Side::Buy);
    let parties = nos.no_party_ids().unwrap();
    assert_eq!(parties.len(), 1);
    assert_eq!(parties.index(0).party_id().unwrap(), "TRADER-7");
}

// A message as it arrives off the wire (full header, parsed and validated) cracks the same way,
// with the header the session stamped still there.
#[test]
fn parsed_inbound_message_cracks_with_header_intact() {
    let mut logon = Logon::new();
    logon.set_encrypt_method(EncryptMethod::None);
    logon.set_heart_bt_int(30);
    let parsed = wire_roundtrip(logon.into()).expect("engine accepts the logon");

    let Fix43Message::Logon(logon) = Fix43Message::from(parsed) else {
        panic!("35=A should crack to Logon");
    };
    assert_eq!(logon.heart_bt_int().unwrap(), 30);
    assert_eq!(logon.message().get_header_field::<String>(49).unwrap(), "BANZAI");
}

// ---- Anything else: `Unknown`, message unchanged ----

// Each a different way of not being a FIX 4.3 MsgType: an undefined code, a user-defined code,
// a defined code in the wrong case (`AE` is defined, `ae` isn't — codes are case-sensitive, and
// FIX does define lowercase ones: `d` is SecurityDefinition, `D` NewOrderSingle), a defined code extended (`AE` is TradeCaptureReport, `AEX`
// is nothing — exact match, not prefix), a defined code with padding, and a blank value.
#[test]
fn undefined_msg_type_is_unknown_with_message_unchanged() {
    for msg_type in ["ZZ", "U1", "ae", "AEX", " D", ""] {
        let mut msg = raw(Some(msg_type));
        msg.set_body_field(58, "custom");
        let before = msg.to_string();

        let Fix43Message::Unknown(msg) = Fix43Message::from(msg) else {
            panic!("35={msg_type:?} should be Unknown");
        };
        assert_eq!(msg.to_string(), before, "35={msg_type:?}");
    }
}

// No MsgType at all is `Unknown` too, not a panic: the conversion is total.
#[test]
fn missing_msg_type_is_unknown_with_message_unchanged() {
    let mut msg = raw(None);
    msg.set_body_field(58, "no type");
    let before = msg.to_string();

    let Fix43Message::Unknown(msg) = Fix43Message::from(msg) else {
        panic!("a message without 35 should be Unknown");
    };
    assert_eq!(msg.to_string(), before);
}

// ---- Back to `Message` ----

// Message -> Fix43Message -> Message gives back the same message, for a typed variant and for
// `Unknown` alike.
#[test]
fn round_trip_back_to_message_is_lossless() {
    let mut nos = NewOrderSingle::new();
    nos.set_cl_ord_id("ORD-2");
    nos.set_symbol("IBM");
    let known: Message = nos.into();
    let mut unknown = raw(Some("U1"));
    unknown.set_body_field(58, "custom");

    for msg in [known, unknown] {
        let before = msg.to_string();
        assert_eq!(Message::from(Fix43Message::from(msg)).to_string(), before);
    }
}

// A variant built directly (the outbound direction, no cracking involved) converts to the
// facade's message.
#[test]
fn variant_built_directly_converts_to_its_message() {
    let msg = Message::from(Fix43Message::Logon(Logon::new()));
    assert_eq!(msg.get_msg_type().unwrap(), "A");
}
