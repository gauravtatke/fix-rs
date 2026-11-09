//! Hand-written tests for the generated `ExecutionReport` facade
//! (`super::super::generated::execution_report`).
//!
//! The accessor branches are covered by `market_data_snapshot_full_refresh.rs`; these cover the
//! report as an order-flow reply: its MsgType, a fill and a reject built through the facade being
//! accepted by the engine's parser, and same-code enums on different tags kept apart.

use rust_decimal::Decimal;
use std::str::FromStr;

use super::wire_roundtrip;
use crate::core::message::Message;
use crate::errors::{SessionRejectError, TypedError};
use crate::messages::fix43::execution_report::ExecutionReport;
use crate::messages::fix43::fields::{ExecType, OrdStatus, Side};

fn dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

/// An ExecutionReport with every required field set through the facade.
fn report(
    exec_type: ExecType,
    ord_status: OrdStatus,
    cum: &str,
    leaves: &str,
    avg: &str,
) -> ExecutionReport {
    let mut er = ExecutionReport::new();
    er.set_order_id("O-1");
    er.set_cl_ord_id("ORD-1");
    er.set_exec_id("E-1");
    er.set_exec_type(exec_type);
    er.set_ord_status(ord_status);
    er.set_symbol("IBM");
    er.set_side(Side::Buy);
    er.set_order_qty(dec("100"));
    er.set_cum_qty(dec(cum));
    er.set_leaves_qty(dec(leaves));
    er.set_avg_px(dec(avg));
    er
}

#[test]
fn new_stamps_msgtype_8() {
    assert_eq!(ExecutionReport::new().message().get_msg_type().unwrap(), "8");
}

#[test]
fn try_from_rejects_new_order_single() {
    let mut msg = Message::new();
    msg.set_header_field(35, "D");
    assert_eq!(
        ExecutionReport::try_from(msg).unwrap_err(),
        TypedError::WrongMsgType {
            expected: "8",
            got: "D".to_string()
        }
    );
}

// A full fill as the acceptor would send it: accepted by the engine's parser, and the money and
// quantity fields read back exactly (no float rounding: `100.125` stays `100.125`).
#[test]
fn fill_report_is_accepted_by_the_parser_and_reads_back() {
    let mut er = report(ExecType::Fill, OrdStatus::Filled, "100", "0", "100.125");
    er.set_last_qty(dec("60"));
    er.set_last_px(dec("100.25"));

    let er = ExecutionReport::try_from(wire_roundtrip(er.into()).expect("engine accepts the fill"))
        .unwrap();

    assert_eq!(er.order_id().unwrap(), "O-1");
    assert_eq!(er.cl_ord_id().unwrap(), "ORD-1");
    assert_eq!(er.cum_qty().unwrap(), dec("100"));
    assert_eq!(er.leaves_qty().unwrap(), dec("0"));
    assert_eq!(er.avg_px().unwrap(), dec("100.125"));
    assert_eq!(er.last_qty().unwrap(), dec("60"));
    assert_eq!(er.last_px().unwrap(), dec("100.25"));
}

// ExecType::Fill and OrdStatus::Filled are both `2` on the wire, on different tags (150 / 39).
// Writing one must not leak into the other. A replace has ExecType `5`, OrdStatus `0`: different
// codes, each valid for both tags, so a crossed tag would read a wrong but valid variant.
#[test]
fn exec_type_and_ord_status_are_independent_fields() {
    let er = report(ExecType::Replace, OrdStatus::New, "0", "100", "0");
    let er = ExecutionReport::try_from(wire_roundtrip(er.into()).unwrap()).unwrap();
    assert_eq!(er.exec_type().unwrap(), ExecType::Replace);
    assert_eq!(er.ord_status().unwrap(), OrdStatus::New);
    assert_eq!(er.message().get_body_field::<String>(150).unwrap(), "5");
    assert_eq!(er.message().get_body_field::<String>(39).unwrap(), "0");
}

// A reject report carries a reason in Text; the parse layer accepts it and Text reads back.
#[test]
fn reject_report_is_accepted_by_the_parser() {
    let mut er = report(ExecType::Rejected, OrdStatus::Rejected, "0", "0", "0");
    er.set_text("unknown symbol");
    let er = ExecutionReport::try_from(wire_roundtrip(er.into()).unwrap()).unwrap();
    assert_eq!(er.exec_type().unwrap(), ExecType::Rejected);
    assert_eq!(er.text().unwrap(), "unknown symbol");
}

// A value the enum can't represent, sent raw (as a buggy counterparty would), is rejected by the
// parser before the facade ever sees it.
#[test]
fn out_of_range_exec_type_is_rejected_by_the_parser() {
    let mut msg: Message = report(ExecType::New, OrdStatus::New, "0", "100", "0").into();
    msg.set_body_field(150, "Z");
    assert_eq!(wire_roundtrip(msg).unwrap_err(), SessionRejectError::ValueOutOfRange { tag: 150 });
}
