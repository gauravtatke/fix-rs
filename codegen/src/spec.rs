//! FIX dictionary parse — the codegen input model.
//!
//! Ported from the old `build/code_generator.rs` (roxmltree-based), but slimmed to **raw data**:
//! it stores each field's name / tag / FIX type string / value codes + descriptions, and leaves
//! all *naming and typing* to the emit step. That's deliberate — the emitter applies `heck` casing
//! (to match the hand-written references) and the D8 FIX-type→Rust map (`src/convert.rs`), so the
//! old `get_primitive_type` (buggy `f32` for money) and the `ENVal_` variant prefix are **not**
//! carried over.
//!
//! TODO (your part): extend the parse to **messages** (msg_type, per-message ordered fields +
//! required flags) and **repeating groups** (resolving `<component>` refs), which the old parser
//! stubbed but never populated — needed before generating message facades. For now it parses the
//! `<fields>` section, which is enough for `tags.rs` and the value enums (`fields.rs`).

use roxmltree::{Document, Node};
use std::path::Path;

/// The whole parsed dictionary (currently: begin-string + field definitions).
#[derive(Debug, Default)]
pub struct FixSpec {
    /// e.g. "FIX.4.3" (from the root element's type/major/minor).
    pub begin_string: String,
    /// Every `<field>` definition, in document order.
    pub fields: Vec<FieldDef>,
    // TODO: pub messages: Vec<MessageDef>,  (msg_type, ordered fields, required, groups)
}

/// One `<field number=.. name=.. type=..>` definition, with its allowed values (if any).
#[derive(Debug, Default, Clone)]
pub struct FieldDef {
    pub name: String,
    pub tag: u32,
    /// The raw FIX type string ("CHAR", "PRICE", "INT", …) — the emitter maps it to a Rust type
    /// via the D8 map in `src/convert.rs`, not here.
    pub fix_type: String,
    /// Allowed enum values (empty for free-valued fields).
    pub values: Vec<FieldValue>,
}

/// One `<value enum=.. description=..>` — raw code + description. The emitter derives the Rust
/// variant name from `description` (heck `UpperCamelCase`), matching the hand-written enums.
#[derive(Debug, Default, Clone)]
pub struct FieldValue {
    /// The on-wire code, e.g. "1", "A".
    pub code: String,
    /// The dictionary description, e.g. "BUY", "SELL_SHORT_EXEMPT".
    pub description: String,
}

/// Parse a FIX dictionary XML (e.g. `resources/FIX43.xml`) into a [`FixSpec`].
pub fn parse(xml_path: &Path) -> FixSpec {
    let text = std::fs::read_to_string(xml_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", xml_path.display()));
    let doc = Document::parse(&text).expect("FIX dictionary XML failed to parse");
    let root = doc.root_element();

    let begin_string = format!(
        "{}.{}.{}",
        root.attribute("type").unwrap_or("FIX"),
        root.attribute("major").unwrap_or("?"),
        root.attribute("minor").unwrap_or("?"),
    );

    let fields = lookup(&doc, "fields")
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("field"))
        .map(parse_field)
        .collect();

    FixSpec { begin_string, fields }
}

fn parse_field(field: Node) -> FieldDef {
    let name = field.attribute("name").expect("field has a name").to_owned();
    let tag = field
        .attribute("number")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or_else(|| panic!("field {name} has no numeric tag"));
    let fix_type = field.attribute("type").expect("field has a type").to_owned();
    let values = field
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("value"))
        .map(|v| FieldValue {
            code: v.attribute("enum").expect("value has an enum code").to_owned(),
            description: v.attribute("description").expect("value has a description").to_owned(),
        })
        .collect();
    FieldDef { name, tag, fix_type, values }
}

/// Find a top-level child element of the document root by tag name (case-insensitive).
fn lookup<'a, 'i>(doc: &'a Document<'i>, name: &str) -> Node<'a, 'i> {
    doc.root_element()
        .children()
        .find(|n| n.tag_name().name().eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("<{name}> section not found in dictionary"))
}
