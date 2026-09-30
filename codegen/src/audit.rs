//! `cargo run -p codegen -- audit <dictionary.xml>` — report FIX names that heck may case badly.
//!
//! Reports only; it never decides. Each flagged name is then either corrected in
//! [`naming::corrected_name`] or accepted as-is in [`naming::ACCEPTED_NAMES`]; names already in
//! either are skipped, so a re-run shows only what is still undecided.
//!
//! Split for testability: [`audit`] is pure (XML text in, [`Findings`] out) so the guard test can
//! assert on it; [`run_audit`] adds the file I/O and printing for the CLI.

use crate::naming;
use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use roxmltree::Document;
use std::collections::BTreeMap;

/// What the audit found, one report row per flagged name. The first three categories are
/// *precise* (a hit is almost always a real bug); the last two are for a human to eyeball.
#[derive(Debug, Default)]
pub(crate) struct Findings {
    /// Total distinct names seen (fields, messages, components, groups).
    pub(crate) total_names: usize,
    pub(crate) single_letter: Vec<String>,
    pub(crate) keywords: Vec<String>,
    pub(crate) collisions: Vec<String>,
    pub(crate) acronym_lower: Vec<String>,
    pub(crate) digits: Vec<String>,
}

/// CLI entry: read the dictionary at `path`, audit it, print the report. Errors (unreadable file,
/// malformed XML) come back as a message for `main` to print and exit on.
pub(crate) fn run_audit(path: &str) -> Result<(), String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("could not read `{path}`: {e}"))?;
    let findings = audit(&text).map_err(|e| format!("`{path}` is not well-formed XML: {e}"))?;

    println!("== {path}: {} names", findings.total_names);
    report("SINGLE-LETTER SEGMENT (precise)", &findings.single_letter);
    report("RUST KEYWORD (precise)", &findings.keywords);
    report("COLLISION (precise)", &findings.collisions);
    report("ACRONYM FOLLOWED BY LOWERCASE (noisy, review by eye)", &findings.acronym_lower);
    report("DIGITS (informational)", &findings.digits);
    Ok(())
}

/// Audit a dictionary's XML text. Pure — no I/O — so tests can call it directly.
pub(crate) fn audit(text: &str) -> Result<Findings, roxmltree::Error> {
    let doc = Document::parse(text)?;
    let names_kind_map = names_and_kind(&doc);

    let mut findings = Findings {
        total_names: names_kind_map.len(),
        ..Findings::default()
    };
    // For the collision check: generated const name -> every XML name that produced it.
    let mut by_const: BTreeMap<String, Vec<&str>> = BTreeMap::new();

    for (&name, &kind) in &names_kind_map {
        // Already decided (corrected or accepted) — nothing left to report.
        if naming::ACCEPTED_NAMES.contains(&name) || naming::corrected_name(name).is_some() {
            continue;
        }
        // All three casings the generator will emit: SHOUTY consts (tags.rs), snake accessors,
        // UpperCamel types. A name can be fine in one and broken in another, so the row shows all.
        let shouty = name.to_shouty_snake_case();
        let snake = name.to_snake_case();
        let camel = name.to_upper_camel_case();
        let row = format!("{kind:<9} {name:<34} {shouty:<40} {snake:<40} {camel}");

        // Single-letter segment: heck split an acronym (`IDs` -> `I_DS`). Precise.
        if shouty.split('_').any(|seg| seg.len() == 1) {
            findings.single_letter.push(row.clone());
        // 2+ capitals then a lowercase letter: heck guesses the last capital starts a new word.
        // Right for `MDEntry` (MD|Entry), wrong for `IOIid` (IO|Iid). Noisy — needs a human.
        } else if has_acronym_then_lowercase(name) {
            findings.acronym_lower.push(row.clone());
        // Digits: heck attaches them to the preceding word (`Rule80A` -> `RULE80_A`). Info only.
        } else if name.chars().any(|c| c.is_ascii_digit()) {
            findings.digits.push(row.clone());
        }

        // Keyword: independent of the above — a name can be well-cased AND a keyword. Precise.
        if naming::RUST_KEYWORDS.contains(&snake.as_str()) {
            findings.keywords.push(row);
        }

        // Record which XML name produced this const: find its list (starting an empty one the
        // first time the const is seen) and add the name. Collisions are read off after the loop,
        // since a clash is only visible once both names have been seen.
        by_const.entry(shouty).or_default().push(name);
    }

    // Collision: two distinct XML names -> the same Rust const name (e.g. `SecurityIDSource` and
    // `SecurityIdSource` both -> `SECURITY_ID_SOURCE`), which would emit a duplicate `pub const`
    // and fail to compile. Any const with more than one source name is one. Precise.
    //
    // A group and its count field share one XML name (`NoPartyIDs` is both a <field> and a
    // <group>); that is not a collision — `names_and_kind` keys by name, so it is seen once.
    findings.collisions = by_const
        .iter()
        .filter(|(_, sources)| sources.len() > 1)
        .map(|(konst, sources)| format!("{konst} <- {sources:?}"))
        .collect();

    Ok(findings)
}

/// Every name heck will ever see, mapped to what kind of thing it names (`field`, `message`,
/// `component`, `group`). A `BTreeMap` dedups — a group or component is referenced from many
/// messages — and sorts, so the report is alphabetical. Borrows the names from `doc` (no copies).
fn names_and_kind<'a>(doc: &'a Document) -> BTreeMap<&'a str, &'a str> {
    let mut names_kind: BTreeMap<&str, &str> = BTreeMap::new();
    for node in doc.descendants().filter(|n| n.is_element()) {
        let parent = node.parent_element().map(|p| p.tag_name().name());
        let kind = match (node.tag_name().name(), parent) {
            ("field", Some("fields")) => "field", // definitions only, not <field> refs in messages
            ("message", _) => "message",
            ("component", Some("components")) => "component",
            ("group", _) => "group",
            _ => continue,
        };
        if let Some(name) = node.attribute("name") {
            // First kind wins: `NoPartyIDs` is both a field and a group; either label is fine.
            names_kind.entry(name).or_insert(kind);
        }
    }
    names_kind
}

/// True if the name has 2+ consecutive capitals immediately followed by a lowercase letter — the
/// spot where heck has to guess a word boundary. `windows(3)` walks overlapping 3-byte slices;
/// bytes are fine because FIX names are ASCII.
fn has_acronym_then_lowercase(name: &str) -> bool {
    name.as_bytes().windows(3).any(|w| {
        w[0].is_ascii_uppercase() && w[1].is_ascii_uppercase() && w[2].is_ascii_lowercase()
    })
}

/// Print one report section: a title with its count, then one indented row per finding.
fn report(title: &str, rows: &[String]) {
    println!("\n-- {title}: {}", rows.len());
    for row in rows {
        println!("   {row}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fix43_xml_path;
    use std::collections::BTreeSet;

    fn fix43_text() -> String {
        std::fs::read_to_string(fix43_xml_path()).expect("FIX43.xml should be readable")
    }

    /// The XML names in a category's report rows (column 2: `kind name shouty snake camel`).
    fn names_in(rows: &[String]) -> Vec<&str> {
        rows.iter().map(|r| r.split_whitespace().nth(1).expect("row has a name column")).collect()
    }

    // ---- The guard: the dictionary we generate from is fully decided. ----------------------

    /// Fails when FIX43 has a name that heck mis-cases, that is a Rust keyword, or that collides
    /// — and isn't yet corrected or accepted in `naming.rs`. Fix by running the audit
    /// (`cargo run -p codegen -- audit resources/FIX43.xml`) and deciding each flagged name.
    #[test]
    fn fix43_has_no_undecided_precise_findings() {
        let findings = audit(&fix43_text()).expect("FIX43.xml should parse");
        assert!(findings.total_names > 700, "sanity: FIX43 should have ~713 names");
        assert!(findings.single_letter.is_empty(), "undecided: {:#?}", findings.single_letter);
        assert!(findings.keywords.is_empty(), "undecided: {:#?}", findings.keywords);
        assert!(findings.collisions.is_empty(), "undecided: {:#?}", findings.collisions);
    }

    /// The audit skips corrected names, so it can't see a bad *correction*. Re-check the final
    /// names the generator actually emits (correction applied) across every FIX43 name: no
    /// single-letter segment (unless accepted), no keyword, and no two names -> one const.
    #[test]
    fn fix43_final_names_are_clean_and_unique() {
        let text = fix43_text();
        let doc = Document::parse(&text).expect("FIX43.xml should parse");
        let mut seen: BTreeMap<String, &str> = BTreeMap::new();

        for name in names_and_kind(&doc).into_keys() {
            let final_name = naming::corrected_name(name).unwrap_or(name);
            let konst = naming::const_name(name);

            if !naming::ACCEPTED_NAMES.contains(&name) {
                assert!(
                    !konst.split('_').any(|seg| seg.len() == 1),
                    "{name} -> {konst} has a single-letter segment",
                );
            }
            let snake = final_name.to_snake_case();
            assert!(
                !naming::RUST_KEYWORDS.contains(&snake.as_str()),
                "{name} -> `{snake}` is a keyword"
            );

            if let Some(other) = seen.insert(konst.clone(), name) {
                panic!("{other} and {name} both become {konst}");
            }
        }
    }

    // ---- The detector itself: each category fires on its defect. ---------------------------
    // Without these, a broken check would make the guard above pass vacuously.

    /// One name per defect class, plus the special classes that must NOT be reported.
    const SYNTHETIC: &str = r#"<fix type="FIX" major="9" minor="9">
      <messages>
        <message name="WidgetReport" msgtype="U1" msgcat="app">
          <field name="UndeclaredRef" required="N"/>
          <group name="NoWidgetIDs" required="N"><field name="Price3" required="N"/></group>
        </message>
        <message name="WidgetAck" msgtype="U2" msgcat="app">
          <group name="NoWidgetIDs" required="N"><field name="Price3" required="N"/></group>
        </message>
      </messages>
      <components>
        <component name="GadgetBlock"><field name="Price3" required="N"/></component>
      </components>
      <fields>
        <field number="9001" name="NoWidgetIDs" type="NUMINGROUP"/>
        <field number="9002" name="Type" type="STRING"/>
        <field number="9003" name="FooID" type="STRING"/>
        <field number="9004" name="FooId" type="STRING"/>
        <field number="9005" name="ABCdef" type="STRING"/>
        <field number="9006" name="Price3" type="PRICE"/>
        <field number="9007" name="NoPartyIDs" type="NUMINGROUP"/>
        <field number="9008" name="Rule80A" type="CHAR"/>
        <field number="9009" name="PlainName" type="STRING"/>
      </fields>
    </fix>"#;

    #[test]
    fn each_category_fires_on_its_defect() {
        let f = audit(SYNTHETIC).expect("synthetic XML should parse");
        // `IDs` plural -> NO_WIDGET_I_DS.
        assert_eq!(names_in(&f.single_letter), ["NoWidgetIDs"]);
        // `type` is a keyword as a snake_case accessor.
        assert_eq!(names_in(&f.keywords), ["Type"]);
        // Two distinct names -> one const.
        assert_eq!(f.collisions, [r#"FOO_ID <- ["FooID", "FooId"]"#]);
        // Informational categories.
        assert_eq!(names_in(&f.acronym_lower), ["ABCdef"]);
        assert_eq!(names_in(&f.digits), ["Price3"]);
    }

    #[test]
    fn decided_names_are_skipped() {
        // `NoPartyIDs` (corrected) and `Rule80A` (accepted) would otherwise be single-letter hits.
        let f = audit(SYNTHETIC).expect("synthetic XML should parse");
        let reported: BTreeSet<&str> = [&f.single_letter, &f.keywords, &f.acronym_lower, &f.digits]
            .into_iter()
            .flat_map(|rows| names_in(rows))
            .collect();
        assert!(!reported.contains("NoPartyIDs"), "corrected name reported");
        assert!(!reported.contains("Rule80A"), "accepted name reported");
    }

    #[test]
    fn collects_definitions_not_references_and_dedups() {
        let doc = Document::parse(SYNTHETIC).expect("synthetic XML should parse");
        let names = names_and_kind(&doc);
        // A <field> inside a message is a reference, not a definition — never collected.
        assert!(!names.contains_key("UndeclaredRef"));
        // Messages and components are collected with their kind.
        assert_eq!(names.get("WidgetReport"), Some(&"message"));
        assert_eq!(names.get("GadgetBlock"), Some(&"component"));
        // A group used by two messages, and sharing its name with its count field, is one entry
        // (so it can't collide with itself). 9 fields + 2 messages + 1 component, group merged.
        assert!(names.contains_key("NoWidgetIDs"));
        assert_eq!(names.len(), 12);
        let f = audit(SYNTHETIC).unwrap();
        assert!(!f.collisions.iter().any(|c| c.contains("NO_WIDGET")), "group vs its count field");
    }

    #[test]
    fn malformed_xml_is_an_error_not_a_panic() {
        assert!(audit("<fix><fields>").is_err());
    }
}
