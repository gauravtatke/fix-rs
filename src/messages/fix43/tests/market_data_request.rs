//! Hand-written tests for the generated `MarketDataRequest` facade
//! (`super::super::generated::market_data_request`).
//!
//! The request is the group-heavy one: two required repeating groups (NoMDEntryTypes,
//! NoRelatedSym), the latter with an inlined component carrying a nested group (NoSecurityAltID).
//! These check that the built groups survive the engine's parser — count, delimiter-first order,
//! nesting — and read back through the group views.

use super::wire_roundtrip;
use crate::core::message::Message;
use crate::errors::{SessionRejectError, TypedError};
use crate::messages::fix43::fields::{MdEntryType, SubscriptionRequestType};
use crate::messages::fix43::market_data_request::MarketDataRequest;

/// A subscribe request: Bid + Offer for IBM (with a nested SecurityAltID) and MSFT.
fn subscribe_ibm_msft() -> MarketDataRequest {
    let mut mdr = MarketDataRequest::new();
    mdr.set_md_req_id("MDR-1");
    mdr.set_subscription_request_type(SubscriptionRequestType::SnapshotPlusUpdates);
    mdr.set_market_depth(1);
    for t in [MdEntryType::Bid, MdEntryType::Offer] {
        mdr.add_no_md_entry_types_entry(|e| e.set_md_entry_type(t));
    }
    mdr.add_no_related_sym_entry(|s| {
        s.set_security_id("459200101"); // delimiter Symbol (55) set after another field
        s.set_symbol("IBM");
        s.add_no_security_alt_id_entry(|a| {
            a.set_security_alt_id_source("4");
            a.set_security_alt_id("US4592001014");
        });
    });
    mdr.add_no_related_sym_entry(|s| s.set_symbol("MSFT"));
    mdr
}

#[test]
fn new_stamps_msgtype_v() {
    assert_eq!(MarketDataRequest::new().message().get_msg_type().unwrap(), "V");
}

// A snapshot (35=W) answers a request; it isn't one.
#[test]
fn try_from_rejects_snapshot() {
    let mut msg = Message::new();
    msg.set_header_field(35, "W");
    assert_eq!(
        MarketDataRequest::try_from(msg).unwrap_err(),
        TypedError::WrongMsgType {
            expected: "V",
            got: "W".to_string()
        }
    );
}

// Groups built through the facade pass the parser's group checks (count matches entries, each
// entry starts with its delimiter even when set out of order, nested group inside a group entry)
// and read back entry by entry.
#[test]
fn built_request_with_nested_groups_is_accepted_and_reads_back() {
    let parsed = wire_roundtrip(subscribe_ibm_msft().into()).expect("engine accepts the request");
    let mdr = MarketDataRequest::try_from(parsed).unwrap();

    assert_eq!(mdr.md_req_id().unwrap(), "MDR-1");
    assert_eq!(
        mdr.subscription_request_type().unwrap(),
        SubscriptionRequestType::SnapshotPlusUpdates
    );
    assert_eq!(mdr.market_depth().unwrap(), 1);

    let types: Vec<_> =
        mdr.no_md_entry_types().unwrap().iter().map(|e| e.md_entry_type().unwrap()).collect();
    assert_eq!(types, [MdEntryType::Bid, MdEntryType::Offer]);

    let syms = mdr.no_related_sym().unwrap();
    assert_eq!(syms.len(), 2);
    let ibm = syms.index(0);
    assert_eq!(ibm.symbol().unwrap(), "IBM");
    assert_eq!(ibm.security_id().unwrap(), "459200101");
    let alt = ibm.no_security_alt_id().unwrap();
    assert_eq!(alt.len(), 1);
    assert_eq!(alt.index(0).security_alt_id().unwrap(), "US4592001014");
    assert_eq!(alt.index(0).security_alt_id_source().unwrap(), "4");
    let msft = syms.index(1);
    assert_eq!(msft.symbol().unwrap(), "MSFT");
    // The nested group belongs to the IBM entry only.
    assert!(msft.no_security_alt_id().is_none());
}

// NoMDEntryTypes is required: a request without it is rejected by the parser, not by the facade.
#[test]
fn request_without_required_group_is_rejected_by_the_parser() {
    let mut mdr = MarketDataRequest::new();
    mdr.set_md_req_id("MDR-2");
    mdr.set_subscription_request_type(SubscriptionRequestType::Snapshot);
    mdr.set_market_depth(0);
    mdr.add_no_related_sym_entry(|s| s.set_symbol("IBM"));
    assert_eq!(
        wire_roundtrip(mdr.into()).unwrap_err(),
        SessionRejectError::RequiredTagMissing { tag: 267 }
    );
}
