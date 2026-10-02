//! The single place an XML name becomes a Rust name.
//!
//! heck mis-splits a few FIX names (acronym plurals like `IDs`, run-together acronyms like `IOIID`) and a
//! few collide with Rust keywords. [`corrected_name`] fixes those *before* heck runs, and the
//! casing functions ([`const_name`], later snake/camel siblings) always apply it — so emitters
//! must call these, never heck on a raw XML name. The audit (`audit.rs`) finds new cases.

use heck::{ToShoutySnakeCase, ToUpperCamelCase};

/// Rust keywords (strict + reserved, 2024 edition). A snake_case accessor with one of these names
/// won't compile without `r#` or a rename.
pub(crate) const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
    "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "try", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield",
];

/// XML names whose heck output looks suspicious to the audit (e.g. a single-letter segment) but
/// has been reviewed and is fine as-is — `Rule80A` -> `RULE80_A`. Listed here so the audit and the
/// guard test skip them. A name needing a *different* spelling belongs in [`corrected_name`] instead.
pub(crate) const ACCEPTED_NAMES: &[&str] = &["OutMainCntryUIndex", "Rule80A"];

/// The heck-friendly spelling of an XML name that heck would otherwise case wrongly, or `None` if
/// the name needs no correction. Applied *before* heck, so one entry fixes every casing at once
/// (`NoPartyIDs` -> `NoPartyIds` gives `NO_PARTY_IDS`, `no_party_ids`, `NoPartyIds`). Only names
/// heck gets wrong go here; reviewed-and-fine names go in [`ACCEPTED_NAMES`].
pub(crate) fn corrected_name(xml_name: &str) -> Option<&'static str> {
    match xml_name {
        // Plural acronym: heck splits `IDs` into `I` + `Ds` (`NO_PARTY_I_DS`).
        "NoNestedPartyIDs" => Some("NoNestedPartyIds"),
        "NoPartyIDs" => Some("NoPartyIds"),
        "NoRoutingIDs" => Some("NoRoutingIds"),
        // Two acronyms run together (`IOI` + `ID`): heck sees one word (`IOIID`).
        "IOIID" => Some("IoiId"),
        // Lowercase word between acronyms: heck reads it as `XM` + `Lnon` + `FIX` (`XM_LNON_FIX`).
        "XMLnonFIX" => Some("XmlNonFix"),
        // `yield` is a reserved Rust keyword, so the snake_case getter `yield()` would not
        // compile. Renamed in every casing (`YIELD_VALUE`, `yield_value()`, `YieldValue`); the
        // wire tag is unchanged (236).
        "Yield" => Some("YieldValue"),
        _ => None,
    }
}

/// SHOUTY_SNAKE const name for an XML name (`MDEntryPx` -> `MD_ENTRY_PX`), used for the tag
/// constants in `tags.rs`. Applies [`corrected_name`] first (`NoPartyIDs` -> `NO_PARTY_IDS`).
pub(crate) fn const_name(xml_name: &str) -> String {
    corrected_name(xml_name).unwrap_or(xml_name).to_shouty_snake_case()
}

pub(crate) fn variant_name(xml_name: &str) -> String {
    corrected_name(xml_name).unwrap_or(xml_name).to_upper_camel_case()
}

pub(crate) fn enum_name(xml_name: &str) -> String {
    corrected_name(xml_name).unwrap_or(xml_name).to_upper_camel_case()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test is one class of name `const_name` can receive.

    #[test]
    fn plain_name_goes_straight_through_heck() {
        // Not in the table: heck alone, including a correctly split leading acronym.
        assert_eq!(corrected_name("MDEntryPx"), None);
        assert_eq!(const_name("MDEntryPx"), "MD_ENTRY_PX");
        assert_eq!(const_name("HeartBtInt"), "HEART_BT_INT");
    }

    #[test]
    fn corrected_names_produce_the_intended_consts() {
        // Every table entry, pinned to the const it must yield — a typo in a corrected spelling
        // (e.g. `NoPartyIDS`) would silently change a public name; this catches it.
        let cases = [
            ("NoPartyIDs", "NO_PARTY_IDS"),
            ("NoNestedPartyIDs", "NO_NESTED_PARTY_IDS"),
            ("NoRoutingIDs", "NO_ROUTING_IDS"),
            ("IOIID", "IOI_ID"),
            ("XMLnonFIX", "XML_NON_FIX"),
            ("Yield", "YIELD_VALUE"),
        ];
        for (xml, expected) in cases {
            assert!(corrected_name(xml).is_some(), "{xml} should be in the correction table");
            assert_eq!(const_name(xml), expected, "const for {xml}");
        }
    }

    #[test]
    fn accepted_names_are_not_corrected() {
        // Accepted = reviewed and kept as heck produces it; it must not also be in the table
        // (that would make the two lists contradict each other).
        for &name in ACCEPTED_NAMES {
            assert_eq!(corrected_name(name), None, "{name} is both accepted and corrected");
        }
        assert_eq!(const_name("Rule80A"), "RULE80_A");
        assert_eq!(const_name("OutMainCntryUIndex"), "OUT_MAIN_CNTRY_U_INDEX");
    }

    #[test]
    fn table_lookup_is_exact_match() {
        // Case matters and there is no prefix/substring matching: only the exact XML spelling
        // is corrected, so a different name that merely contains `IDs` is left to heck.
        assert_eq!(corrected_name("nopartyids"), None);
        assert_eq!(corrected_name("NoPartyIDsExtra"), None);
    }
}
