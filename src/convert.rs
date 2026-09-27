//! Common, version-neutral FIX value <-> Rust conversion helpers — the "internal format
//! layer" (task 3.2). Shared across every FIX version and message type.
//!
//! These exist because Rust's own `Display`/`FromStr` do NOT match FIX's on-wire format:
//! a FIX boolean is `Y`/`N` (not `true`/`false`), a UTC timestamp is
//! `YYYYMMDD-HH:MM:SS.sss`, dates are `YYYYMMDD`, etc. Keeping every FIX type's format in one
//! place means generated accessors just call these, and a format fix lands once.
//!
//! # FIX 4.3 type -> Rust type map (what the generator emits per field type)
//!
//! | FIX type(s)                                   | Rust type        | conversion              |
//! |-----------------------------------------------|------------------|-------------------------|
//! | `PRICE` `AMT` `QTY` `PRICEOFFSET`             | `Decimal`        | `*_fix_decimal` (D8)    |
//! | `FLOAT` `PERCENTAGE`                          | `f64`            | plain `FromStr`/`Display` |
//! | `INT`                                         | `i32`            | plain                   |
//! | `LENGTH` `NUMINGROUP` `SEQNUM` `TAGNUM`       | `u32`            | plain                   |
//! | `CHAR`                                        | `char`           | plain                   |
//! | `BOOLEAN`                                     | `bool`           | `*_fix_bool`            |
//! | `STRING` `DATA` `COUNTRY` `CURRENCY` `EXCHANGE` `MONTHYEAR` `MULTIPLEVALUESTRING` | `String` | plain (`impl Into<String>`) |
//! | `UTCTIMESTAMP`                                | `DateTime<Utc>`  | `*_fix_utc_timestamp`   |
//! | `UTCDATE` `LOCALMKTDATE`                      | `NaiveDate`      | `*_fix_date`            |
//! | `UTCTIMEONLY`                                 | `NaiveTime`      | `*_fix_time_only`       |
//!
//! "plain" = Rust's own `FromStr`/`Display` already match FIX (integers, `char`, `f64`, strings),
//! so a generated accessor calls `get_field::<T>(tag)` / `v.to_string()` directly — no helper.
//! Helpers exist only where FIX differs from the Rust default. `MONTHYEAR`/`MULTIPLEVALUESTRING`
//! stay `String` for now (structured `NaiveDate`/`Vec<String>` deferred — rare, extra surface).

use crate::errors::FieldError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use rust_decimal::Decimal;
use std::str::FromStr;

// --- Boolean (FIX type BOOLEAN: `Y` / `N`) ---

/// Render a Rust `bool` as its FIX on-wire form.
pub(crate) fn fix_bool_str(v: bool) -> &'static str {
    if v { "Y" } else { "N" }
}

/// Parse a FIX boolean (`Y`/`N`); anything else is malformed for `tag`.
pub(crate) fn parse_fix_bool(raw: &str, tag: u32) -> Result<bool, FieldError> {
    match raw {
        "Y" => Ok(true),
        "N" => Ok(false),
        _ => Err(FieldError::InvalidFormat { tag }),
    }
}

// --- Decimal money/quantity (FIX types PRICE / AMT / QTY / PRICEOFFSET) ---
// D8: exact decimal, no binary-float rounding into orders/fills. `Decimal`'s own `Display` is
// already FIX-correct (plain decimal digits, no exponent, no grouping), and `FromStr` parses the
// wire form — so these helpers are thin wrappers that add FIX-shaped error handling and give the
// format layer one authoritative home.

/// Render a `Decimal` in FIX on-wire form (plain decimal string, no exponent).
pub(crate) fn format_fix_decimal(v: Decimal) -> String {
    v.to_string()
}

/// Parse a FIX decimal (PRICE/AMT/QTY/PRICEOFFSET); a malformed value is an error for `tag`.
/// Rejects scientific notation — FIX float format is plain digits with an optional sign and
/// decimal point, never an exponent.
pub(crate) fn parse_fix_decimal(raw: &str, tag: u32) -> Result<Decimal, FieldError> {
    if raw.contains(['e', 'E']) {
        return Err(FieldError::InvalidFormat { tag });
    }
    Decimal::from_str(raw).map_err(|_| FieldError::InvalidFormat { tag })
}

// --- UTC timestamp (FIX type UTCTIMESTAMP: `YYYYMMDD-HH:MM:SS[.sss]`) ---
// FIX allows the fractional-seconds part to be absent or of varying precision; parsing accepts
// any (or none), formatting always emits milliseconds (a valid, canonical form).

const FIX_UTC_TIMESTAMP_FMT: &str = "%Y%m%d-%H:%M:%S%.3f";

/// Render a UTC datetime in FIX `UTCTimestamp` form (millisecond precision).
pub(crate) fn format_fix_utc_timestamp(dt: DateTime<Utc>) -> String {
    dt.format(FIX_UTC_TIMESTAMP_FMT).to_string()
}

/// Parse a FIX `UTCTimestamp`, tolerating absent/variable fractional seconds; malformed -> error.
pub(crate) fn parse_fix_utc_timestamp(raw: &str, tag: u32) -> Result<DateTime<Utc>, FieldError> {
    // `%.f` consumes an optional fractional part (any number of digits, or none).
    NaiveDateTime::parse_from_str(raw, "%Y%m%d-%H:%M:%S%.f")
        .map(|ndt| Utc.from_utc_datetime(&ndt))
        .map_err(|_| FieldError::InvalidFormat { tag })
}

// --- Date only (FIX types UTCDATE / LOCALMKTDATE: `YYYYMMDD`) ---

const FIX_DATE_FMT: &str = "%Y%m%d";

/// Render a `NaiveDate` in FIX `YYYYMMDD` form.
pub(crate) fn format_fix_date(d: NaiveDate) -> String {
    d.format(FIX_DATE_FMT).to_string()
}

/// Parse a FIX date (`YYYYMMDD`); a malformed or impossible date is an error for `tag`. `chrono`
/// rejects out-of-range months/days and non-leap Feb 29, so those come back as `InvalidFormat`.
pub(crate) fn parse_fix_date(raw: &str, tag: u32) -> Result<NaiveDate, FieldError> {
    NaiveDate::parse_from_str(raw, FIX_DATE_FMT).map_err(|_| FieldError::InvalidFormat { tag })
}

// --- Time only (FIX type UTCTIMEONLY: `HH:MM:SS[.sss]`) ---
// Same fractional tolerance as UTCTimestamp: parse accepts absent/variable, format emits millis.

/// Render a `NaiveTime` in FIX `UTCTimeOnly` form (millisecond precision).
pub(crate) fn format_fix_time_only(t: NaiveTime) -> String {
    t.format("%H:%M:%S%.3f").to_string()
}

/// Parse a FIX `UTCTimeOnly`, tolerating absent/variable fractional seconds; malformed -> error.
pub(crate) fn parse_fix_time_only(raw: &str, tag: u32) -> Result<NaiveTime, FieldError> {
    NaiveTime::parse_from_str(raw, "%H:%M:%S%.f").map_err(|_| FieldError::InvalidFormat { tag })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    // --- bool -------------------------------------------------------------------------------

    #[test]
    fn bool_uses_fix_y_n() {
        assert_eq!(fix_bool_str(true), "Y");
        assert_eq!(fix_bool_str(false), "N");
        assert!(parse_fix_bool("Y", 141).unwrap());
        assert!(!parse_fix_bool("N", 141).unwrap());
        assert!(matches!(parse_fix_bool("true", 141), Err(FieldError::InvalidFormat { tag: 141 })));
    }

    // --- Decimal ----------------------------------------------------------------------------
    // Classes: typical price, integer-valued, negative (PriceOffset), high-precision (the f64
    // would round), zero, trailing-zero scale; rejects: non-numeric, empty, exponent, malformed.

    #[test]
    fn decimal_roundtrips_typical_price() {
        let d = parse_fix_decimal("93.25", 44).unwrap();
        assert_eq!(d, dec!(93.25));
        assert_eq!(format_fix_decimal(d), "93.25");
    }

    #[test]
    fn decimal_roundtrips_integer_negative_and_zero() {
        assert_eq!(format_fix_decimal(parse_fix_decimal("100", 44).unwrap()), "100");
        assert_eq!(format_fix_decimal(parse_fix_decimal("-0.5", 44).unwrap()), "-0.5");
        assert_eq!(format_fix_decimal(parse_fix_decimal("0", 44).unwrap()), "0");
    }

    #[test]
    fn decimal_is_exact_where_f64_would_round() {
        // 18 significant digits: f32 (the old bug) and even f64 can't hold this exactly.
        let raw = "123456789.123456789";
        let d = parse_fix_decimal(raw, 44).unwrap();
        assert_eq!(format_fix_decimal(d), raw);
    }

    #[test]
    fn decimal_preserves_trailing_zero_scale() {
        // A peer that sends "93.250" round-trips byte-identically (Decimal keeps the scale).
        assert_eq!(format_fix_decimal(parse_fix_decimal("93.250", 44).unwrap()), "93.250");
    }

    #[test]
    fn decimal_rejects_malformed() {
        for bad in ["abc", "", "9.9.9", " 93.25", "1e5", "1E5"] {
            assert!(
                matches!(parse_fix_decimal(bad, 44), Err(FieldError::InvalidFormat { tag: 44 })),
                "expected {bad:?} to be rejected"
            );
        }
    }

    // --- UTC timestamp ----------------------------------------------------------------------

    #[test]
    fn utc_timestamp_roundtrips_millis() {
        let raw = "20260926-14:30:00.000";
        let dt = parse_fix_utc_timestamp(raw, 52).unwrap();
        assert_eq!(format_fix_utc_timestamp(dt), raw);
    }

    #[test]
    fn utc_timestamp_parses_absent_and_variable_fraction() {
        // No fractional part, and microsecond precision, both accepted (FIX allows either).
        let no_frac = parse_fix_utc_timestamp("20260926-14:30:00", 52).unwrap();
        assert_eq!(format_fix_utc_timestamp(no_frac), "20260926-14:30:00.000");
        let micros = parse_fix_utc_timestamp("20260926-14:30:00.522123", 52).unwrap();
        assert_eq!(format_fix_utc_timestamp(micros), "20260926-14:30:00.522");
    }

    #[test]
    fn utc_timestamp_rejects_malformed() {
        assert!(matches!(
            parse_fix_utc_timestamp("nope", 52),
            Err(FieldError::InvalidFormat { tag: 52 })
        ));
    }

    // --- date -------------------------------------------------------------------------------
    // Classes: valid, leap-day valid; rejects: non-leap Feb 29, month/day out of range, wrong
    // separator/length, empty.

    #[test]
    fn date_roundtrips_and_accepts_leap_day() {
        let d = parse_fix_date("20260927", 75).unwrap();
        assert_eq!(format_fix_date(d), "20260927");
        assert_eq!(format_fix_date(parse_fix_date("20240229", 75).unwrap()), "20240229");
    }

    #[test]
    fn date_rejects_impossible_and_malformed() {
        for bad in [
            "20250229",
            "20261301",
            "20260931",
            "2026-09-27",
            "202609",
            "nope",
            "",
        ] {
            assert!(
                matches!(parse_fix_date(bad, 75), Err(FieldError::InvalidFormat { tag: 75 })),
                "expected {bad:?} to be rejected"
            );
        }
    }

    // --- time only --------------------------------------------------------------------------

    #[test]
    fn time_only_roundtrips_and_tolerates_fraction() {
        let t = parse_fix_time_only("14:30:00.000", 273).unwrap();
        assert_eq!(format_fix_time_only(t), "14:30:00.000");
        // absent fractional accepted, canonicalized to .000 on format
        let no_frac = parse_fix_time_only("14:30:00", 273).unwrap();
        assert_eq!(format_fix_time_only(no_frac), "14:30:00.000");
    }

    #[test]
    fn time_only_rejects_impossible_and_malformed() {
        for bad in ["25:00:00", "14:60:00", "14-30-00", "nope", ""] {
            assert!(
                matches!(
                    parse_fix_time_only(bad, 273),
                    Err(FieldError::InvalidFormat { tag: 273 })
                ),
                "expected {bad:?} to be rejected"
            );
        }
    }
}
