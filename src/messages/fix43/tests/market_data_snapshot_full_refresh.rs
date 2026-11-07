//! Hand-written tests for the generated `MarketDataSnapshotFullRefresh` facade
//! (`super::super::generated::market_data_snapshot_full_refresh`).
//!
//! Wiring tests for the accessor branches in `field_methods.rs.j2`: one field per `convert`
//! codec, a STRING enum, a MULTIPLEVALUESTRING field, and codecs reached through a repeating-group
//! entry view. The codecs' own edge cases live in `convert.rs`'s tests; these check that each
//! branch calls the right codec, on the right tag, through the right receiver.

use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::errors::FieldError;
use crate::messages::fix43::fields::{ExecInst, MdEntryType, SecurityType};
use crate::messages::fix43::market_data_snapshot_full_refresh::MarketDataSnapshotFullRefresh;

const NO_MD_ENTRIES: u32 = 268;

fn md() -> MarketDataSnapshotFullRefresh {
    MarketDataSnapshotFullRefresh::new()
}

/// The raw wire text of a body field.
fn wire(m: &MarketDataSnapshotFullRefresh, tag: u32) -> String {
    m.message().get_body_field::<String>(tag).unwrap()
}

/// The raw wire text of a field in the `i`th NoMDEntries instance.
fn entry_wire(m: &MarketDataSnapshotFullRefresh, i: usize, tag: u32) -> String {
    m.message().body_group(NO_MD_ENTRIES).unwrap().as_slice()[i].get_field::<String>(tag).unwrap()
}

/// Overwrite a field in the `i`th NoMDEntries instance with raw (possibly malformed) text.
fn set_entry_raw(m: &mut MarketDataSnapshotFullRefresh, i: usize, tag: u32, raw: &str) {
    m.message_mut().body_group_mut(NO_MD_ENTRIES).unwrap().as_mut_slice()[i].set_field(tag, raw);
}

// --- Codec branch, message level ---

// Decimal's wire form is plain digits: trailing zeros kept as given, no exponent.
#[test]
fn decimal_field_roundtrips_in_fix_form() {
    let mut m = md();
    let v = Decimal::from_str("1234.50").unwrap();
    m.set_net_chg_prev_day(v);
    assert_eq!(wire(&m, 451), "1234.50");
    assert_eq!(m.net_chg_prev_day().unwrap(), v);
}

// Display/FromStr would accept `1e3`; the codec must not.
#[test]
fn decimal_in_scientific_notation_is_invalid_format() {
    let mut m = md();
    m.message_mut().set_body_field(451, "1e3");
    assert!(matches!(m.net_chg_prev_day(), Err(FieldError::InvalidFormat { tag: 451 })));
}

#[test]
fn date_field_roundtrips_as_yyyymmdd() {
    let mut m = md();
    let d = NaiveDate::from_ymd_opt(2026, 11, 7).unwrap();
    m.set_maturity_date(d);
    assert_eq!(wire(&m, 541), "20261107");
    assert_eq!(m.maturity_date().unwrap(), d);
}

// chrono's Display would write `2026-11-07`, which FIX doesn't accept.
#[test]
fn iso_date_is_invalid_format() {
    let mut m = md();
    m.message_mut().set_body_field(541, "2026-11-07");
    assert!(matches!(m.maturity_date(), Err(FieldError::InvalidFormat { tag: 541 })));
}

#[test]
fn time_only_field_roundtrips_with_millis() {
    let mut m = md();
    let t = NaiveTime::from_hms_milli_opt(12, 30, 5, 250).unwrap();
    m.set_total_volume_traded_time(t);
    assert_eq!(wire(&m, 450), "12:30:05.250");
    assert_eq!(m.total_volume_traded_time().unwrap(), t);
}

#[test]
fn malformed_time_only_is_invalid_format() {
    let mut m = md();
    m.message_mut().set_body_field(450, "25:00:00");
    assert!(matches!(m.total_volume_traded_time(), Err(FieldError::InvalidFormat { tag: 450 })));
}

// --- STRING enum branch (reads a String, then `from_fix(&raw)`) ---

#[test]
fn string_enum_roundtrips_through_its_fix_code() {
    let mut m = md();
    m.set_security_type(SecurityType::AssetBackedSecurities);
    assert_eq!(wire(&m, 167), "ABS");
    assert_eq!(m.security_type().unwrap(), SecurityType::AssetBackedSecurities);
}

// A multi-character code: the whole string is the code, not its first char.
#[test]
fn string_enum_parses_a_wire_value_set_raw() {
    let mut m = md();
    m.message_mut().set_body_field(167, "AMENDED");
    assert_eq!(m.security_type().unwrap(), SecurityType::AmendedAndRestated);
}

#[test]
fn unknown_string_enum_code_is_invalid_format() {
    let mut m = md();
    m.message_mut().set_body_field(167, "NOPE");
    assert!(matches!(m.security_type(), Err(FieldError::InvalidFormat { tag: 167 })));
}

// Codes are case-sensitive: `abs` isn't `ABS`.
#[test]
fn string_enum_code_is_case_sensitive() {
    let mut m = md();
    m.message_mut().set_body_field(167, "abs");
    assert!(matches!(m.security_type(), Err(FieldError::InvalidFormat { tag: 167 })));
}

// --- MULTIPLEVALUESTRING: plain String accessor, the enum as named codes (as QFJ) ---

// Several space-separated codes round-trip unchanged; one enum variant couldn't hold them.
#[test]
fn multiple_value_string_roundtrips_raw() {
    let mut m = md();
    m.set_financial_status("1 2");
    assert_eq!(wire(&m, 291), "1 2");
    assert_eq!(m.financial_status().unwrap(), "1 2");
}

// The enum still builds the value: named codes joined with spaces.
#[test]
fn multiple_value_string_built_from_enum_codes() {
    let mut m = md();
    let v = [ExecInst::Work, ExecInst::StayOnBidside, ExecInst::CallFirst]
        .map(|e| e.to_fix())
        .join(" ");
    m.add_no_md_entries_entry(|e| {
        e.set_md_entry_type(MdEntryType::Bid);
        e.set_exec_inst(&v);
    });
    assert_eq!(entry_wire(&m, 0, 18), "2 9 C");
    assert_eq!(m.no_md_entries().unwrap().index(0).exec_inst().unwrap(), "2 9 C");
}

// The accessor doesn't validate codes (incoming validation is the parse layer's job).
#[test]
fn multiple_value_string_getter_returns_unknown_codes_as_is() {
    let mut m = md();
    m.message_mut().set_body_field(291, "1 Z");
    assert_eq!(m.financial_status().unwrap(), "1 Z");
}

// --- Codecs through a group entry view (receiver is the entry's FieldMap, not the body) ---

#[test]
fn group_entry_codec_fields_roundtrip() {
    let mut m = md();
    let px = Decimal::from_str("101.25").unwrap();
    let d = NaiveDate::from_ymd_opt(2026, 11, 7).unwrap();
    let t = NaiveTime::from_hms_milli_opt(9, 15, 0, 0).unwrap();
    let exp = Utc.with_ymd_and_hms(2026, 11, 7, 21, 0, 0).unwrap();
    m.add_no_md_entries_entry(|e| {
        e.set_md_entry_type(MdEntryType::Trade);
        e.set_md_entry_px(px);
        e.set_md_entry_date(d);
        e.set_md_entry_time(t);
        e.set_expire_time(exp);
    });

    assert_eq!(entry_wire(&m, 0, 270), "101.25");
    assert_eq!(entry_wire(&m, 0, 272), "20261107");
    assert_eq!(entry_wire(&m, 0, 273), "09:15:00.000");
    assert_eq!(entry_wire(&m, 0, 126), "20261107-21:00:00.000");
    // Written to the entry, not the message body.
    assert!(m.message().get_body_field::<String>(270).is_err());

    let e = m.no_md_entries().unwrap().index(0);
    assert_eq!(e.md_entry_type().unwrap(), MdEntryType::Trade);
    assert_eq!(e.md_entry_px().unwrap(), px);
    assert_eq!(e.md_entry_date().unwrap(), d);
    assert_eq!(e.md_entry_time().unwrap(), t);
    assert_eq!(e.expire_time().unwrap(), exp);
}

// Each entry reads its own value: the getter's receiver is per entry.
#[test]
fn group_entries_read_their_own_codec_values() {
    let mut m = md();
    for px in ["100.5", "100.75"] {
        m.add_no_md_entries_entry(|e| {
            e.set_md_entry_type(MdEntryType::Bid);
            e.set_md_entry_px(Decimal::from_str(px).unwrap());
        });
    }
    let g = m.no_md_entries().unwrap();
    assert_eq!(g.index(0).md_entry_px().unwrap(), Decimal::from_str("100.5").unwrap());
    assert_eq!(g.index(1).md_entry_px().unwrap(), Decimal::from_str("100.75").unwrap());
}

// The incoming side tolerates FIX timestamps without fractional seconds.
#[test]
fn group_entry_timestamp_parses_without_millis() {
    let mut m = md();
    m.add_no_md_entries_entry(|e| e.set_md_entry_type(MdEntryType::Bid));
    set_entry_raw(&mut m, 0, 126, "20261107-21:00:00");
    let exp = Utc.with_ymd_and_hms(2026, 11, 7, 21, 0, 0).unwrap();
    assert_eq!(m.no_md_entries().unwrap().index(0).expire_time().unwrap(), exp);
}

// A malformed value inside an entry reports the entry field's tag.
#[test]
fn malformed_group_entry_codec_values_are_invalid_format() {
    let mut m = md();
    m.add_no_md_entries_entry(|e| e.set_md_entry_type(MdEntryType::Bid));
    set_entry_raw(&mut m, 0, 270, "1e2");
    set_entry_raw(&mut m, 0, 273, "9:15");
    set_entry_raw(&mut m, 0, 126, "2026-11-07T21:00:00Z");
    let e = m.no_md_entries().unwrap().index(0);
    assert!(matches!(e.md_entry_px(), Err(FieldError::InvalidFormat { tag: 270 })));
    assert!(matches!(e.md_entry_time(), Err(FieldError::InvalidFormat { tag: 273 })));
    assert!(matches!(e.expire_time(), Err(FieldError::InvalidFormat { tag: 126 })));
}

// An absent codec field is TagNotFound, not InvalidFormat: the `?` on the String read comes first.
#[test]
fn absent_codec_field_is_tag_not_found() {
    let m = md();
    assert!(matches!(m.maturity_date(), Err(FieldError::TagNotFound { tag: 541 })));
}
