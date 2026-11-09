//! Hand-written tests for the generated FIX 4.3 typed messages (`super::generated`).
//!
//! Kept out of `generated/` so the generator's output stays pure code the freshness test can
//! reproduce byte-for-byte, and so these remain an *independent* oracle: a bug in the emitter
//! can't bake itself into the tests that are supposed to catch it. One module per generated file.

mod execution_report;
mod fields;
mod logon;
mod market_data_request;
mod market_data_snapshot_full_refresh;
mod new_order_single;

use std::sync::LazyLock;

use crate::core::dictionary::DataDictionary;
use crate::core::message::Message;
use crate::errors::SessionRejectError;

static DD: LazyLock<DataDictionary> =
    LazyLock::new(|| DataDictionary::from_xml("resources/FIX43.xml"));

/// Send `msg` through the engine's real inbound path: stamp the header fields the session would
/// add (BeginString, CompIDs, MsgSeqNum, SendingTime), compute BodyLength and CheckSum, serialize,
/// then `Message::from_str` against the FIX43 dictionary — which runs every parse-time check
/// (required fields, group structure and order, field types, enum values). `Ok` means a
/// counterparty engine would accept what the facade built.
fn wire_roundtrip(msg: Message) -> Result<Message, SessionRejectError> {
    let mut msg = msg;
    msg.set_header_field(8, "FIX.4.3");
    msg.set_header_field(49, "BANZAI");
    msg.set_header_field(56, "FIXIMULATOR");
    msg.set_header_field(34, "2");
    msg.set_header_field(52, "20261109-09:00:00.000");
    msg.set_body_len();
    msg.set_checksum();
    Message::from_str(&msg.to_string(), &DD)
}
