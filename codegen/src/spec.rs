//! FIX dictionary parse — the codegen input model.
//!
//! Parses a QuickFIX-format dictionary (`resources/FIX43.xml`) into **raw data** (`*Def` types):
//! the XML as written, nothing interpreted.
//!
//! - **Fields:** name / tag / FIX type string / allowed values. Naming and Rust typing are the
//!   emitter's job (`naming.rs`, the D8 type map in `src/convert.rs`), not the parser's.
//! - **Messages, components, groups:** one ordered list of [`Item`]s each — fields, groups and
//!   component references in document order, every `required` flag exactly as its own attribute
//!   says, `<component>` kept as a reference (never inlined). Effective required-ness and
//!   component inlining happen later, in the emitter's `resolve_items`.
//!
//! The parser is strict (design log D12a): a malformed `required`, a duplicate component name,
//! an unexpected element or a missing name/attribute is a panic naming the problem, never a
//! silent default. A missing `<components>` section is fine (FIX 4.0–4.2 predate components).

use roxmltree::{Document, Node};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

/// The whole parsed dictionary.
#[derive(Debug, Default)]
pub(crate) struct FixSpec {
    /// e.g. "FIX.4.3" (from the root element's type/major/minor; `type` defaults to "FIX").
    pub(crate) begin_string: String,
    /// Every `<field>` definition, in document order.
    pub(crate) fields: Vec<FieldDef>,
    /// Component definitions by name. `BTreeMap` so any iteration (e.g. emitting component
    /// types) is in a stable order — generated output must be deterministic.
    pub(crate) components: BTreeMap<String, ComponentDef>,
    /// Every `<message>` definition, in document order.
    pub(crate) messages: Vec<MessageDef>,
}

/// One `<message name=.. msgtype=.. msgcat=..>` definition.
#[derive(Debug, Clone)]
pub(crate) struct MessageDef {
    /// e.g. "Logon".
    pub(crate) name: String,
    /// The MsgType (tag 35) value, e.g. "A".
    pub(crate) msg_type: String,
    /// "admin" (session-level) or "app" (the `msgcat` attribute).
    pub(crate) category: String,
    /// The message body, in document order (header/trailer are not part of it).
    pub(crate) items: Vec<Item>,
}

/// One `<component name=..>` definition from the `<components>` section — a named, reusable run
/// of items that messages, groups and other components reference.
#[derive(Debug, Clone)]
pub(crate) struct ComponentDef {
    /// e.g. "Parties".
    pub(crate) name: String,
    /// The component's contents, in document order.
    pub(crate) items: Vec<Item>,
}

/// One entry in a message / group / component body, exactly as the XML has it. `required` is the
/// entry's own attribute — a parent's flag is never folded in (that's `resolve_items`' job).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Item {
    /// `<field name=.. required=..>` — a reference to a [`FieldDef`] by name.
    Field { name: String, required: bool },
    /// `<group name=.. required=..>` — a repeating group; `name` is also its NumInGroup count
    /// field. `items` are one instance's contents, in order (the first is the delimiter).
    Group {
        name: String,
        required: bool,
        items: Vec<Item>,
    },
    /// `<component name=.. required=..>` — a reference to a [`ComponentDef`], not inlined.
    Component { name: String, required: bool },
}

/// One `<field number=.. name=.. type=..>` definition, with its allowed values (if any).
#[derive(Debug, Default, Clone)]
pub(crate) struct FieldDef {
    /// e.g. "EncryptMethod".
    pub(crate) name: String,
    /// The field's tag number, e.g. 98.
    pub(crate) tag: u32,
    /// The raw FIX type string ("CHAR", "PRICE", "INT", …) — the emitter maps it to a Rust type
    /// via the D8 map in `src/convert.rs`, not here.
    pub(crate) fix_type: String,
    /// Allowed values, in document order (empty for free-valued fields).
    pub(crate) values: Vec<FieldValueDef>,
}

/// One `<value enum=.. description=..>` — raw code + description. The emitter derives the Rust
/// variant name from `description` (via `naming::variant_name`).
#[derive(Debug, Default, Clone, Serialize)]
pub(crate) struct FieldValueDef {
    /// The on-wire code, e.g. "1", "A".
    pub(crate) code: String,
    /// The dictionary description, e.g. "BUY", "SELL_SHORT_EXEMPT".
    pub(crate) description: String,
}

/// Parse a FIX dictionary XML (e.g. `resources/FIX43.xml`) into a [`FixSpec`].
pub(crate) fn parse(xml_path: &Path) -> FixSpec {
    let text = std::fs::read_to_string(xml_path)
        .unwrap_or_else(|e| panic!("{} should be readable: {e}", xml_path.display()));
    parse_str(&text)
}

/// Parse dictionary XML text into a [`FixSpec`]. Split from [`parse`] so tests can feed inline XML.
pub(crate) fn parse_str(text: &str) -> FixSpec {
    let doc = Document::parse(text).expect("FIX dictionary XML should be well-formed");
    let root = doc.root_element();

    let begin_string = format!(
        "{}.{}.{}",
        root.attribute("type").unwrap_or("FIX"),
        root.attribute("major").unwrap_or("?"),
        root.attribute("minor").unwrap_or("?"),
    );

    let fields: Vec<FieldDef> = lookup(&doc, "fields")
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("field"))
        .map(parse_field)
        .collect();
    let components = parse_components(&doc);
    let messages = parse_messages(&doc);
    FixSpec {
        begin_string,
        fields,
        messages,
        components,
    }
}

/// Component definitions by name. A dictionary without a `<components>` section (FIX 4.0-4.2
/// predate components) has none. A name defined twice is a dictionary error (D12a), not "last one
/// wins".
fn parse_components(doc: &Document) -> BTreeMap<String, ComponentDef> {
    let mut components = BTreeMap::new();
    let Some(section) = find_section(doc, "components") else {
        return components;
    };
    for node in section.children().filter(|n| n.is_element() && n.has_tag_name("component")) {
        let def = ComponentDef {
            name: name_attr(node),
            items: parse_items(node),
        };
        if let Some(previous) = components.insert(def.name.clone(), def) {
            panic!("duplicate component definition `{}`", previous.name);
        }
    }
    components
}

fn parse_messages(doc: &Document) -> Vec<MessageDef> {
    lookup(doc, "messages")
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("message"))
        .map(|child| {
            let name = name_attr(child);
            let msg_type = child
                .attribute("msgtype")
                .unwrap_or_else(|| panic!("missing message type in {}", child.tag_name().name()));
            let category = child.attribute("msgcat").unwrap_or_else(|| {
                panic!("missing message category in {}", child.tag_name().name())
            });
            MessageDef {
                name,
                category: category.to_string(),
                msg_type: msg_type.to_string(),
                items: parse_items(child),
            }
        })
        .collect()
}

fn parse_field(field: Node) -> FieldDef {
    let name =
        field.attribute("name").expect("every <field> should have a name attribute").to_owned();
    let tag = field.attribute("number").and_then(|s| s.parse::<u32>().ok()).unwrap_or_else(|| {
        panic!("field {name} should have a numeric `number` attribute (its tag)")
    });
    let fix_type = field
        .attribute("type")
        .unwrap_or_else(|| panic!("field {name} should have a type attribute"))
        .to_owned();
    let values = field
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("value"))
        .map(|v| FieldValueDef {
            code: v
                .attribute("enum")
                .unwrap_or_else(|| {
                    panic!("every <value> of field {name} should have an enum attribute")
                })
                .to_owned(),
            description: v
                .attribute("description")
                .unwrap_or_else(|| {
                    panic!("every <value> of field {name} should have a description attribute")
                })
                .to_owned(),
        })
        .collect();
    FieldDef {
        name,
        tag,
        fix_type,
        values,
    }
}

/// Find a top-level child element of the document root by tag name (case-insensitive).
fn lookup<'a, 'i>(doc: &'a Document<'i>, name: &str) -> Node<'a, 'i> {
    find_section(doc, name).unwrap_or_else(|| panic!("dictionary should have a <{name}> section"))
}

/// Like [`lookup`], for a section that may legitimately be absent.
fn find_section<'a, 'i>(doc: &'a Document<'i>, name: &str) -> Option<Node<'a, 'i>> {
    doc.root_element().children().find(|n| n.tag_name().name().eq_ignore_ascii_case(name))
}

fn name_attr(node: Node) -> String {
    node.attribute("name")
        .unwrap_or_else(|| panic!("field {node:?} should have a name attribute"))
        .to_owned()
}

/// The item's `required` flag. Strictly `Y` or `N` (D12a): a typo or a missing attribute is a
/// dictionary error, not a silent "optional".
fn required_attr(node: Node) -> bool {
    match node.attribute("required") {
        Some("Y") => true,
        Some("N") => false,
        other => panic!(
            "<{} name=\"{}\"> has required={other:?}; expected \"Y\" or \"N\"",
            node.tag_name().name(),
            node.attribute("name").unwrap_or("?"),
        ),
    }
}

fn parse_items(node: Node) -> Vec<Item> {
    node.children()
        .filter(|n| n.is_element())
        .map(|n| {
            let name = name_attr(n);
            let required = required_attr(n);
            match n.tag_name().name() {
                "field" => Item::Field { name, required },
                "group" => Item::Group {
                    name,
                    required,
                    items: parse_items(n),
                },
                "component" => Item::Component { name, required },
                other => panic!("unexpected <{other}> in {}", n.tag_name().name()),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    //! The parse model records the XML **as written**: every item in document order, `required`
    //! exactly as its own attribute says (a parent's flag is never folded in), and `<component>`
    //! kept as a reference (never inlined). Deciding what a flag *means* in context — e.g. a
    //! required field inside an optional component — is the emitter's job, not the parser's.
    //!
    //! Scenarios mirror the runtime dictionary's XML fixtures (`src/core/dictionary.rs`
    //! `dictionary_tests`), but assert the raw model rather than effective required-ness.
    use super::*;

    const Y: bool = true;
    const N: bool = false;

    fn f(name: &str, required: bool) -> Item {
        Item::Field {
            name: name.to_owned(),
            required,
        }
    }
    fn g(name: &str, required: bool, items: Vec<Item>) -> Item {
        Item::Group {
            name: name.to_owned(),
            required,
            items,
        }
    }
    fn c(name: &str, required: bool) -> Item {
        Item::Component {
            name: name.to_owned(),
            required,
        }
    }

    /// A minimal dictionary around the given `<messages>` / `<components>` bodies. `<fields>` is
    /// empty: the parser doesn't resolve item names against field definitions.
    fn spec(messages: &str, components: &str) -> FixSpec {
        parse_str(&format!(
            r#"<fix major="4" minor="3"><fields/><components>{components}</components><messages>{messages}</messages></fix>"#
        ))
    }

    fn msg<'a>(spec: &'a FixSpec, msg_type: &str) -> &'a MessageDef {
        spec.messages
            .iter()
            .find(|m| m.msg_type == msg_type)
            .unwrap_or_else(|| panic!("message {msg_type} should be parsed"))
    }

    fn comp<'a>(spec: &'a FixSpec, name: &str) -> &'a ComponentDef {
        spec.components.get(name).unwrap_or_else(|| panic!("component {name} should be parsed"))
    }

    /// The item named `name` among `items` (top level only).
    fn item<'a>(items: &'a [Item], name: &str) -> &'a Item {
        items
            .iter()
            .find(|i| match i {
                Item::Field { name: n, .. }
                | Item::Group { name: n, .. }
                | Item::Component { name: n, .. } => n == name,
            })
            .unwrap_or_else(|| panic!("item {name} should be present"))
    }

    // ---- Messages: metadata and plain fields ----------------------------------------------

    #[test]
    fn messages_keep_document_order_and_metadata() {
        let s = spec(
            r#"<message name="Logon" msgtype="A" msgcat="admin"/>
               <message name="NewOrderSingle" msgtype="D" msgcat="app"/>"#,
            "",
        );
        let meta: Vec<_> = s
            .messages
            .iter()
            .map(|m| (m.name.as_str(), m.msg_type.as_str(), m.category.as_str()))
            .collect();
        assert_eq!(meta, [("Logon", "A", "admin"), ("NewOrderSingle", "D", "app")]);
    }

    #[test]
    fn message_with_no_items_parses_empty() {
        // Boundary: zero items (cardinality 0).
        let s = spec(r#"<message name="Empty" msgtype="0" msgcat="admin"/>"#, "");
        assert!(msg(&s, "0").items.is_empty());
    }

    #[test]
    fn message_fields_keep_order_and_raw_required() {
        // test_msgs_with_fields: Y and N both recorded, order as written (not sorted by name).
        let s = spec(
            r#"<message name="M" msgtype="6" msgcat="app">
                 <field name="mfield2" required="N"/>
                 <field name="mfield1" required="Y"/>
               </message>"#,
            "",
        );
        assert_eq!(msg(&s, "6").items, [f("mfield2", N), f("mfield1", Y)]);
    }

    #[test]
    fn message_interleaving_all_item_kinds_keeps_exact_order() {
        // FIX field order is significant, so kinds must not be bucketed (fields, then groups…).
        let s = spec(
            r#"<message name="M" msgtype="8" msgcat="app">
                 <field name="mfield1" required="Y"/>
                 <group name="group1" required="Y"><field name="gfield11" required="Y"/></group>
                 <field name="mfield2" required="N"/>
                 <component name="CompWithOnlyFields" required="N"/>
                 <group name="group4" required="N"><field name="gfield41" required="Y"/></group>
               </message>"#,
            "",
        );
        assert_eq!(
            msg(&s, "8").items,
            [
                f("mfield1", Y),
                g("group1", Y, vec![f("gfield11", Y)]),
                f("mfield2", N),
                c("CompWithOnlyFields", N),
                g("group4", N, vec![f("gfield41", Y)]),
            ]
        );
    }

    // ---- Groups ---------------------------------------------------------------------------

    #[test]
    fn required_and_optional_groups_with_their_fields() {
        // test_msg_with_groups.
        let s = spec(
            r#"<message name="MessageWithReqAndNonReqGroups" msgtype="8" msgcat="app">
                 <field name="mfield1" required="Y"/>
                 <group name="group1" required="Y">
                   <field name="gfield11" required="Y"/>
                   <field name="gfield12" required="N"/>
                 </group>
                 <field name="mfield2" required="N"/>
                 <group name="group2" required="N">
                   <field name="gfield21" required="Y"/>
                   <field name="gfield22" required="N"/>
                 </group>
               </message>"#,
            "",
        );
        assert_eq!(
            msg(&s, "8").items,
            [
                f("mfield1", Y),
                g("group1", Y, vec![f("gfield11", Y), f("gfield12", N)]),
                f("mfield2", N),
                g("group2", N, vec![f("gfield21", Y), f("gfield22", N)]),
            ]
        );
    }

    #[test]
    fn group_required_flag_is_never_folded_into_its_items() {
        // The special class: every (group required) × (item required) combination. The runtime
        // dictionary ANDs these for validation; the parse model must keep both flags untouched
        // so that policy stays a single decision in the emitter.
        let s = spec(
            r#"<message name="M" msgtype="8" msgcat="app">
                 <group name="reqGroup" required="Y">
                   <field name="a" required="Y"/><field name="b" required="N"/>
                   <group name="sub1" required="Y"/><group name="sub2" required="N"/>
                   <component name="C1" required="Y"/><component name="C2" required="N"/>
                 </group>
                 <group name="optGroup" required="N">
                   <field name="a" required="Y"/><field name="b" required="N"/>
                   <group name="sub1" required="Y"/><group name="sub2" required="N"/>
                   <component name="C1" required="Y"/><component name="C2" required="N"/>
                 </group>
               </message>"#,
            "",
        );
        let inner = vec![
            f("a", Y),
            f("b", N),
            g("sub1", Y, vec![]),
            g("sub2", N, vec![]),
            c("C1", Y),
            c("C2", N),
        ];
        assert_eq!(msg(&s, "8").items, [g("reqGroup", Y, inner.clone()), g("optGroup", N, inner)]);
    }

    #[test]
    fn group_with_required_and_optional_subgroups() {
        // test_req_group_having_subgroups (required parent) + test_non_req_group_having_subgroups
        // (optional parent): subgroup flags are their own either way.
        let body = |parent_req: &str| {
            format!(
                r#"<group name="group6" required="{parent_req}">
                     <field name="mfield2" required="Y"/>
                     <group name="group7" required="Y"><field name="gfield41" required="Y"/></group>
                     <group name="group3" required="N">
                       <field name="gfield31" required="Y"/>
                       <field name="gfield32" required="N"/>
                     </group>
                   </group>"#
            )
        };
        let s = spec(
            &format!(
                r#"<message name="Req" msgtype="E" msgcat="app">{}</message>
                   <message name="Opt" msgtype="F" msgcat="app">{}</message>"#,
                body("Y"),
                body("N")
            ),
            "",
        );
        let subgroups = vec![
            f("mfield2", Y),
            g("group7", Y, vec![f("gfield41", Y)]),
            g("group3", N, vec![f("gfield31", Y), f("gfield32", N)]),
        ];
        assert_eq!(msg(&s, "E").items, [g("group6", Y, subgroups.clone())]);
        assert_eq!(msg(&s, "F").items, [g("group6", N, subgroups)]);
    }

    #[test]
    fn subgroup_as_first_item_of_group() {
        // test_group_with_subgroup_as_first_field: a group's first item can itself be a group.
        let s = spec(
            r#"<message name="M" msgtype="E" msgcat="app">
                 <group name="group6" required="Y">
                   <group name="group7" required="N"><field name="gfield41" required="Y"/></group>
                   <field name="gfield21" required="Y"/>
                 </group>
               </message>"#,
            "",
        );
        assert_eq!(
            msg(&s, "E").items,
            [g(
                "group6",
                Y,
                vec![g("group7", N, vec![f("gfield41", Y)]), f("gfield21", Y)]
            )]
        );
    }

    #[test]
    fn groups_nest_beyond_two_levels() {
        // Recursion depth > 2 (FIX43's deepest is 2; later versions go deeper).
        let s = spec(
            r#"<message name="M" msgtype="E" msgcat="app">
                 <group name="L1" required="Y">
                   <group name="L2" required="N">
                     <group name="L3" required="Y">
                       <group name="L4" required="N"><field name="leaf" required="Y"/></group>
                     </group>
                   </group>
                 </group>
               </message>"#,
            "",
        );
        assert_eq!(
            msg(&s, "E").items,
            [g(
                "L1",
                Y,
                vec![g(
                    "L2",
                    N,
                    vec![g("L3", Y, vec![g("L4", N, vec![f("leaf", Y)])])]
                )]
            )]
        );
    }

    #[test]
    fn empty_group_parses_with_no_items() {
        // Boundary: zero items inside a group. Not valid FIX, but the parser records, it
        // doesn't judge.
        let s = spec(
            r#"<message name="M" msgtype="E" msgcat="app"><group name="g" required="N"/></message>"#,
            "",
        );
        assert_eq!(msg(&s, "E").items, [g("g", N, vec![])]);
    }

    // ---- Groups containing components ------------------------------------------------------

    #[test]
    fn group_with_only_a_component_keeps_the_reference() {
        // test_group_having_only_component: the reference is not replaced by the component's
        // fields (FIX43's News/NoRelatedSym is this shape).
        let s = spec(
            r#"<message name="M" msgtype="B" msgcat="app">
                 <field name="mfield1" required="Y"/>
                 <group name="group5" required="Y">
                   <component name="CompWithOnlyFields" required="Y"/>
                 </group>
                 <field name="mfield2" required="N"/>
               </message>"#,
            r#"<component name="CompWithOnlyFields">
                 <field name="cfield1" required="Y"/>
                 <field name="cfield2" required="N"/>
               </component>"#,
        );
        assert_eq!(
            msg(&s, "B").items,
            [
                f("mfield1", Y),
                g("group5", Y, vec![c("CompWithOnlyFields", Y)]),
                f("mfield2", N),
            ]
        );
    }

    #[test]
    fn group_with_fields_and_component_refs() {
        // test_req_group_with_comps / test_non_req_group_with_comps: refs keep their position
        // among the group's fields and their own required flag, whatever the group's flag is.
        let body = |group_req: &str| {
            format!(
                r#"<group name="group5" required="{group_req}">
                     <field name="gfield21" required="Y"/>
                     <component name="CompWithOnlyFields" required="Y"/>
                     <component name="Comp2WithFields" required="N"/>
                   </group>"#
            )
        };
        let s = spec(
            &format!(
                r#"<message name="Req" msgtype="B" msgcat="app">{}</message>
                   <message name="Opt" msgtype="C" msgcat="app">{}</message>"#,
                body("Y"),
                body("N")
            ),
            "",
        );
        let inner = vec![
            f("gfield21", Y),
            c("CompWithOnlyFields", Y),
            c("Comp2WithFields", N),
        ];
        assert_eq!(msg(&s, "B").items, [g("group5", Y, inner.clone())]);
        assert_eq!(msg(&s, "C").items, [g("group5", N, inner)]);
    }

    #[test]
    fn group_with_component_and_subgroup_containing_component() {
        // test_group_having_comp_and_subgroup: group -> component ref, and group -> subgroup ->
        // component ref; the referenced components themselves contain groups.
        let s = spec(
            r#"<message name="M" msgtype="E" msgcat="app">
                 <field name="mfield1" required="N"/>
                 <group name="group6" required="Y">
                   <field name="mfield2" required="Y"/>
                   <component name="CompWithOnlyReqGroup" required="Y"/>
                   <group name="group7" required="N">
                     <field name="gfield41" required="Y"/>
                     <component name="CompWithFieldAndNonReqGroup" required="N"/>
                   </group>
                   <group name="group3" required="Y">
                     <field name="gfield31" required="Y"/>
                     <field name="gfield32" required="N"/>
                   </group>
                 </group>
               </message>"#,
            r#"<component name="CompWithOnlyReqGroup">
                 <group name="group2" required="Y">
                   <field name="gfield21" required="N"/>
                   <field name="gfield22" required="Y"/>
                 </group>
               </component>
               <component name="CompWithFieldAndNonReqGroup">
                 <field name="cfield1" required="Y"/>
                 <field name="cfield2" required="N"/>
                 <group name="group1" required="N">
                   <field name="gfield11" required="Y"/>
                   <field name="gfield12" required="N"/>
                 </group>
               </component>"#,
        );
        assert_eq!(
            msg(&s, "E").items,
            [
                f("mfield1", N),
                g(
                    "group6",
                    Y,
                    vec![
                        f("mfield2", Y),
                        c("CompWithOnlyReqGroup", Y),
                        g("group7", N, vec![f("gfield41", Y), c("CompWithFieldAndNonReqGroup", N)]),
                        g("group3", Y, vec![f("gfield31", Y), f("gfield32", N)]),
                    ]
                ),
            ]
        );
        assert_eq!(
            comp(&s, "CompWithOnlyReqGroup").items,
            [g("group2", Y, vec![f("gfield21", N), f("gfield22", Y)])]
        );
        assert_eq!(
            comp(&s, "CompWithFieldAndNonReqGroup").items,
            [
                f("cfield1", Y),
                f("cfield2", N),
                g("group1", N, vec![f("gfield11", Y), f("gfield12", N)]),
            ]
        );
    }

    // ---- Components: references and definitions -------------------------------------------

    #[test]
    fn component_reference_in_message_is_not_inlined() {
        // test_msg_with_component: the message holds references; the component's own fields
        // live only in its definition.
        let s = spec(
            r#"<message name="MsgWithCompHavingFields" msgtype="6" msgcat="app">
                 <field name="mfield1" required="Y"/>
                 <field name="mfield2" required="N"/>
                 <component name="CompWithOnlyFields" required="Y"/>
                 <component name="Comp2WithFields" required="N"/>
               </message>"#,
            r#"<component name="CompWithOnlyFields">
                 <field name="cfield1" required="Y"/>
                 <field name="cfield2" required="N"/>
               </component>
               <component name="Comp2WithFields">
                 <field name="gfield11" required="Y"/>
                 <field name="gfield12" required="N"/>
               </component>"#,
        );
        assert_eq!(
            msg(&s, "6").items,
            [
                f("mfield1", Y),
                f("mfield2", N),
                c("CompWithOnlyFields", Y),
                c("Comp2WithFields", N),
            ]
        );
        assert_eq!(comp(&s, "CompWithOnlyFields").items, [f("cfield1", Y), f("cfield2", N)]);
        assert_eq!(comp(&s, "Comp2WithFields").items, [f("gfield11", Y), f("gfield12", N)]);
    }

    #[test]
    fn component_definitions_with_fields_and_group() {
        // test_req_comp_having_group / test_non_req_comp_having_group: a definition can hold a
        // group after its fields; the group's flags are as written in the definition, independent
        // of whether the *reference* to the component is required.
        let comps = r#"<component name="CompWithFieldAndNonReqGroup">
                 <field name="cfield1" required="Y"/>
                 <field name="cfield2" required="N"/>
                 <group name="group1" required="N">
                   <field name="gfield11" required="Y"/>
                   <field name="gfield12" required="N"/>
                 </group>
               </component>
               <component name="CompWithFieldsAndReqGroup">
                 <field name="cfield3" required="Y"/>
                 <field name="cfield4" required="N"/>
                 <group name="group2" required="Y">
                   <field name="gfield21" required="Y"/>
                   <field name="gfield22" required="N"/>
                 </group>
               </component>"#;
        let s = spec(
            r#"<message name="Req" msgtype="6" msgcat="app">
                 <component name="CompWithFieldAndNonReqGroup" required="Y"/>
                 <component name="CompWithFieldsAndReqGroup" required="Y"/>
               </message>
               <message name="Opt" msgtype="7" msgcat="app">
                 <component name="CompWithFieldAndNonReqGroup" required="N"/>
                 <component name="CompWithFieldsAndReqGroup" required="N"/>
               </message>"#,
            comps,
        );
        assert_eq!(
            comp(&s, "CompWithFieldAndNonReqGroup").items,
            [
                f("cfield1", Y),
                f("cfield2", N),
                g("group1", N, vec![f("gfield11", Y), f("gfield12", N)]),
            ]
        );
        assert_eq!(
            comp(&s, "CompWithFieldsAndReqGroup").items,
            [
                f("cfield3", Y),
                f("cfield4", N),
                g("group2", Y, vec![f("gfield21", Y), f("gfield22", N)]),
            ]
        );
        assert_eq!(
            msg(&s, "6").items,
            [
                c("CompWithFieldAndNonReqGroup", Y),
                c("CompWithFieldsAndReqGroup", Y)
            ]
        );
        assert_eq!(
            msg(&s, "7").items,
            [
                c("CompWithFieldAndNonReqGroup", N),
                c("CompWithFieldsAndReqGroup", N)
            ]
        );
    }

    #[test]
    fn component_definitions_with_only_a_group() {
        // test_req_comp_having_only_group / test_non_req_comp_having_only_group (FIX43's
        // Parties/Stipulations are this shape).
        let s = spec(
            "",
            r#"<component name="CompWithOnlyNonReqGroup">
                 <group name="group1" required="N">
                   <field name="gfield11" required="N"/>
                   <field name="gfield12" required="Y"/>
                 </group>
               </component>
               <component name="CompWithOnlyReqGroup">
                 <group name="group2" required="Y">
                   <field name="gfield21" required="N"/>
                   <field name="gfield22" required="Y"/>
                 </group>
               </component>"#,
        );
        assert_eq!(
            comp(&s, "CompWithOnlyNonReqGroup").items,
            [g("group1", N, vec![f("gfield11", N), f("gfield12", Y)])]
        );
        assert_eq!(
            comp(&s, "CompWithOnlyReqGroup").items,
            [g("group2", Y, vec![f("gfield21", N), f("gfield22", Y)])]
        );
    }

    #[test]
    fn component_definition_referencing_another_component_keeps_the_reference() {
        // Components inside components (absent in FIX43, present from FIX44): the inner one
        // stays a reference too, including inside the outer component's group.
        let s = spec(
            "",
            r#"<component name="Outer">
                 <field name="o1" required="Y"/>
                 <component name="Inner" required="N"/>
                 <group name="og" required="N"><component name="Inner" required="Y"/></group>
               </component>
               <component name="Inner"><field name="i1" required="Y"/></component>"#,
        );
        assert_eq!(
            comp(&s, "Outer").items,
            [f("o1", Y), c("Inner", N), g("og", N, vec![c("Inner", Y)])]
        );
        assert_eq!(comp(&s, "Inner").items, [f("i1", Y)]);
    }

    #[test]
    fn required_flag_belongs_to_each_reference_not_the_component() {
        // One definition, referenced as required by one message and optional by another (FIX43:
        // Instrument is Y in NewOrderSingle, N in News). The flag lives on the reference.
        let s = spec(
            r#"<message name="A1" msgtype="1" msgcat="app"><component name="Shared" required="Y"/></message>
               <message name="A2" msgtype="2" msgcat="app"><component name="Shared" required="N"/></message>"#,
            r#"<component name="Shared"><field name="s1" required="Y"/></component>"#,
        );
        assert_eq!(msg(&s, "1").items, [c("Shared", Y)]);
        assert_eq!(msg(&s, "2").items, [c("Shared", N)]);
        assert_eq!(s.components.len(), 1, "a definition is parsed once, however often used");
    }

    #[test]
    fn components_are_keyed_by_name_in_sorted_order() {
        // Determinism: iteration order is by name, not document order (generated output must not
        // depend on map iteration order).
        let s =
            spec("", r#"<component name="Zeta"/><component name="Alpha"/><component name="Mid"/>"#);
        let names: Vec<_> = s.components.keys().map(String::as_str).collect();
        assert_eq!(names, ["Alpha", "Mid", "Zeta"]);
        assert!(s.components.iter().all(|(k, v)| *k == v.name));
    }

    // ---- Things the parser must ignore -----------------------------------------------------

    #[test]
    fn comments_and_whitespace_between_items_are_ignored() {
        // Our patched FIX43.xml carries `<!-- MODIFIED: -->` comments; they and whitespace text
        // nodes must not become items, at any depth.
        let s = spec(
            r#"<message name="M" msgtype="8" msgcat="app">
                 <!-- a comment -->
                 <field name="a" required="Y"/>
                 <group name="g" required="N">
                   <!-- inside a group -->
                   <field name="b" required="N"/>
                 </group>
               </message>"#,
            r#"<component name="C"><!-- inside a component --><field name="x" required="Y"/></component>"#,
        );
        assert_eq!(msg(&s, "8").items, [f("a", Y), g("g", N, vec![f("b", N)])]);
        assert_eq!(comp(&s, "C").items, [f("x", Y)]);
    }

    // ---- Field definitions (light: unchanged by the message work) ---------------------------

    #[test]
    fn field_definitions_keep_document_order_tag_type_and_values() {
        let s = parse_str(
            r#"<fix major="4" minor="3"><fields>
                 <field number="54" name="Side" type="CHAR">
                   <value enum="1" description="BUY"/><value enum="2" description="SELL"/>
                 </field>
                 <field number="11" name="ClOrdID" type="STRING"/>
               </fields><components/><messages/></fix>"#,
        );
        assert_eq!(s.begin_string, "FIX.4.3");
        let summary: Vec<_> =
            s.fields.iter().map(|f| (f.name.as_str(), f.tag, f.fix_type.as_str())).collect();
        assert_eq!(summary, [("Side", 54, "CHAR"), ("ClOrdID", 11, "STRING")]);
        let codes: Vec<_> = s.fields[0].values.iter().map(|v| v.code.as_str()).collect();
        assert_eq!(codes, ["1", "2"]);
        assert!(s.fields[1].values.is_empty());
    }

    // ---- Malformed input: fail loudly, naming the problem (D12a) ---------------------------

    #[test]
    #[should_panic(expected = "required")]
    fn required_attribute_other_than_y_or_n_panics() {
        // A typo must not silently become "optional".
        spec(
            r#"<message name="M" msgtype="8" msgcat="app"><field name="a" required="y"/></message>"#,
            "",
        );
    }

    #[test]
    #[should_panic(expected = "required")]
    fn missing_required_attribute_panics() {
        // Every field/group/component reference in FIX43 carries Y or N; absence is an error.
        spec(r#"<message name="M" msgtype="8" msgcat="app"><field name="a"/></message>"#, "");
    }

    #[test]
    #[should_panic(expected = "required")]
    fn missing_required_attribute_deep_in_a_component_group_panics() {
        // Same rule at depth (component -> group -> field): the check is in the shared recursion.
        spec(
            "",
            r#"<component name="C"><group name="g" required="N"><field name="a"/></group></component>"#,
        );
    }

    #[test]
    #[should_panic(expected = "unexpected <value>")]
    fn unexpected_element_in_message_panics() {
        spec(
            r#"<message name="M" msgtype="8" msgcat="app"><value name="v" required="N"/></message>"#,
            "",
        );
    }

    #[test]
    #[should_panic(expected = "unexpected <message>")]
    fn unexpected_element_nested_in_group_panics() {
        spec(
            r#"<message name="M" msgtype="8" msgcat="app">
                 <group name="g" required="N"><message name="x" required="N"/></group>
               </message>"#,
            "",
        );
    }

    #[test]
    #[should_panic(expected = "name")]
    fn item_without_name_panics() {
        spec(r#"<message name="M" msgtype="8" msgcat="app"><field required="Y"/></message>"#, "");
    }

    #[test]
    #[should_panic(expected = "message type")]
    fn message_without_msgtype_panics() {
        spec(r#"<message name="M" msgcat="app"/>"#, "");
    }

    #[test]
    #[should_panic(expected = "message category")]
    fn message_without_msgcat_panics() {
        spec(r#"<message name="M" msgtype="8"/>"#, "");
    }

    #[test]
    #[should_panic(expected = "duplicate")]
    fn duplicate_component_definition_panics() {
        // A BTreeMap collect would silently keep the last definition.
        spec(
            "",
            r#"<component name="C"><field name="a" required="Y"/></component>
                    <component name="C"><field name="b" required="N"/></component>"#,
        );
    }

    #[test]
    #[should_panic(expected = "<messages>")]
    fn missing_messages_section_panics() {
        parse_str(r#"<fix major="4" minor="3"><fields/><components/></fix>"#);
    }

    #[test]
    fn dictionary_without_components_section_has_no_components() {
        // FIX 4.0-4.2 predate components: an absent section is a format difference, not an error.
        let s = parse_str(
            r#"<fix major="4" minor="2"><fields/><messages>
                 <message name="M" msgtype="8" msgcat="app"><field name="a" required="Y"/></message>
               </messages></fix>"#,
        );
        assert!(s.components.is_empty());
        assert_eq!(msg(&s, "8").items, [f("a", Y)]);
    }

    // ---- The real dictionary (resources/FIX43.xml) -----------------------------------------

    fn fix43() -> FixSpec {
        parse(&crate::fix43_xml_path())
    }

    #[test]
    fn fix43_section_counts() {
        let s = fix43();
        assert_eq!(s.begin_string, "FIX.4.3");
        assert_eq!(s.fields.len(), 655);
        assert_eq!(s.messages.len(), 67);
        assert_eq!(s.components.len(), 10);
        assert_eq!(s.messages.first().unwrap().name, "Heartbeat");
        assert_eq!(s.messages.last().unwrap().name, "RegistrationInstructionsResponse");
    }

    #[test]
    fn fix43_logon_exact() {
        // The 3.8b target: scalar fields plus the optional NoMsgTypes group in the middle.
        let s = fix43();
        let logon = msg(&s, "A");
        assert_eq!((logon.name.as_str(), logon.category.as_str()), ("Logon", "admin"));
        assert_eq!(
            logon.items,
            [
                f("EncryptMethod", Y),
                f("HeartBtInt", Y),
                f("RawDataLength", N),
                f("RawData", N),
                f("ResetSeqNumFlag", N),
                f("MaxMessageSize", N),
                g("NoMsgTypes", N, vec![f("RefMsgType", N), f("MsgDirection", N)]),
                f("TestMessageIndicator", N),
                f("Username", N),
                f("Password", N),
            ]
        );
    }

    #[test]
    fn fix43_new_order_single_keeps_parties_as_a_reference() {
        let s = fix43();
        assert_eq!(msg(&s, "D").items[3], c("Parties", N));
        assert_eq!(*item(&msg(&s, "D").items, "Instrument"), c("Instrument", Y));
    }

    #[test]
    fn fix43_parties_definition_exact() {
        // A component that is only a group.
        let s = fix43();
        assert_eq!(
            comp(&s, "Parties").items,
            [g(
                "NoPartyIDs",
                N,
                vec![
                    f("PartyID", N),
                    f("PartyIDSource", N),
                    f("PartyRole", N),
                    f("PartySubID", N)
                ]
            )]
        );
    }

    #[test]
    fn fix43_news_group_holding_only_a_component() {
        let s = fix43();
        assert_eq!(
            *item(&msg(&s, "B").items, "NoRelatedSym"),
            g("NoRelatedSym", N, vec![c("Instrument", N)])
        );
    }

    #[test]
    fn fix43_new_order_cross_nests_component_refs_inside_group_and_subgroup() {
        // NoSides (required) -> Parties ref, OrderQtyData ref (required), NoAllocs subgroup ->
        // NestedParties ref: every nesting shape in one real message.
        let s = fix43();
        let Item::Group {
            required,
            items: sides,
            ..
        } = item(&msg(&s, "s").items, "NoSides")
        else {
            panic!("NoSides should be a group");
        };
        assert!(*required);
        assert_eq!(sides[0], f("Side", Y));
        assert_eq!(*item(sides, "Parties"), c("Parties", N));
        assert_eq!(*item(sides, "OrderQtyData"), c("OrderQtyData", Y));
        assert_eq!(
            *item(sides, "NoAllocs"),
            g(
                "NoAllocs",
                N,
                vec![
                    f("AllocAccount", N),
                    f("IndividualAllocID", N),
                    c("NestedParties", N),
                    f("AllocQty", N),
                ]
            )
        );
    }
}
