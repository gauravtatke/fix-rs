//! Hand-written tests for the generated `NewOrderSingle` facade
//! (`super::super::generated::new_order_single`).
//!
//! The accessor branches are covered by `market_data_snapshot_full_refresh.rs`; these cover what
//! is specific to the order: its MsgType, an order built through the facade being accepted by the
//! engine's own parser, an order received off the wire read through the facade, and the facade
//! not enforcing required fields (the parse layer does).

use chrono::{TimeZone, Utc};
use rust_decimal::Decimal;
use std::str::FromStr;

use super::{DD, wire_roundtrip};
use crate::core::message::{Message, SOH};
use crate::errors::{SessionRejectError, TypedError};
use crate::messages::fix43::fields::{
    ExecInst, HandlInst, OrdType, PartyIdSource, PartyRole, Side,
};
use crate::messages::fix43::new_order_single::NewOrderSingle;

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

/// A limit order with every NewOrderSingle required field except TransactTime (60).
fn limit_order_without_transact_time() -> NewOrderSingle {
    let mut nos = NewOrderSingle::new();
    nos.set_cl_ord_id("ORD-1");
    nos.set_handl_inst(HandlInst::AutomatedExecutionOrderPrivate);
    nos.set_symbol("IBM");
    nos.set_side(Side::Buy);
    nos.set_order_qty(dec("100"));
    nos.set_ord_type(OrdType::Limit);
    nos.set_price(dec("101.25"));
    nos
}

/// A limit order with every NewOrderSingle required field set through the facade.
fn limit_order() -> NewOrderSingle {
    let mut nos = limit_order_without_transact_time();
    nos.set_transact_time(Utc.with_ymd_and_hms(2026, 11, 9, 9, 0, 0).unwrap());
    nos
}

#[test]
fn new_stamps_msgtype_d() {
    assert_eq!(NewOrderSingle::new().message().get_msg_type().unwrap(), "D");
}

// An ExecutionReport (35=8) is not an order, even though it shares most of its fields.
#[test]
fn try_from_rejects_execution_report() {
    let mut msg = Message::new();
    msg.set_header_field(35, "8");
    assert_eq!(
        NewOrderSingle::try_from(msg).unwrap_err(),
        TypedError::WrongMsgType {
            expected: "D",
            got: "8".to_string()
        }
    );
}

// The facade's output passes the engine's full inbound validation and reads back the same:
// enums, Decimal money, the UTC timestamp, a multi-value field built from enum codes, and a
// repeating group (NoPartyIDs) whose entry setters are called out of dictionary order.
#[test]
fn built_order_is_accepted_by_the_parser_and_reads_back() {
    let mut nos = limit_order();
    let exec_inst = [ExecInst::Work, ExecInst::AllOrNone].map(|e| e.to_fix()).join(" ");
    nos.set_exec_inst(&exec_inst);
    nos.add_no_party_ids_entry(|p| {
        p.set_party_role(PartyRole::ExecutingFirm); // delimiter PartyID (448) set last
        p.set_party_id_source(PartyIdSource::Proprietary);
        p.set_party_id("FIRM-A");
    });

    let parsed = wire_roundtrip(nos.into()).expect("engine accepts the built order");
    let nos = NewOrderSingle::try_from(parsed).unwrap();

    assert_eq!(nos.cl_ord_id().unwrap(), "ORD-1");
    assert_eq!(nos.handl_inst().unwrap(), HandlInst::AutomatedExecutionOrderPrivate);
    assert_eq!(nos.symbol().unwrap(), "IBM");
    assert_eq!(nos.side().unwrap(), Side::Buy);
    assert_eq!(nos.transact_time().unwrap(), Utc.with_ymd_and_hms(2026, 11, 9, 9, 0, 0).unwrap());
    assert_eq!(nos.order_qty().unwrap(), dec("100"));
    assert_eq!(nos.ord_type().unwrap(), OrdType::Limit);
    assert_eq!(nos.price().unwrap(), dec("101.25"));
    assert_eq!(nos.exec_inst().unwrap(), "2 G");
    let party = nos.no_party_ids().unwrap().index(0);
    assert_eq!(party.party_id().unwrap(), "FIRM-A");
    assert_eq!(party.party_id_source().unwrap(), PartyIdSource::Proprietary);
    assert_eq!(party.party_role().unwrap(), PartyRole::ExecutingFirm);
}

// An order as a counterparty sends it: parsed by the engine, then read through the facade.
#[test]
fn received_order_reads_through_the_facade() {
    let raw = "8=FIX.4.3|9=137|35=D|34=2|49=BANZAI|52=20261109-09:00:00.000|56=FIXIMULATOR|11=ORD-7|21=1|18=2 9 C|55=MSFT|54=2|60=20261109-09:00:00|38=250|40=2|44=99.5|10=175|"
        .replace('|', &SOH.to_string());
    let nos =
        NewOrderSingle::try_from(Message::from_str(&raw, &DD).expect("fixture parses")).unwrap();
    assert_eq!(nos.side().unwrap(), Side::Sell);
    assert_eq!(nos.ord_type().unwrap(), OrdType::Limit);
    assert_eq!(nos.order_qty().unwrap(), dec("250"));
    assert_eq!(nos.price().unwrap(), dec("99.5"));
    // No fractional seconds on the wire: still a valid UTCTimestamp.
    assert_eq!(nos.transact_time().unwrap(), Utc.with_ymd_and_hms(2026, 11, 9, 9, 0, 0).unwrap());
    assert_eq!(nos.exec_inst().unwrap(), "2 9 C");
    assert!(!nos.has_account());
}

// The facade builds whatever it's given; required fields are enforced by the parse layer.
#[test]
fn order_missing_a_required_field_is_rejected_by_the_parser() {
    let msg = limit_order_without_transact_time().into();
    assert_eq!(
        wire_roundtrip(msg).unwrap_err(),
        SessionRejectError::RequiredTagMissing { tag: 60 }
    );
}
