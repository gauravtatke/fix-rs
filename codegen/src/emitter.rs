use crate::naming;
use crate::spec::{FieldDef, FixSpec, Item, MessageDef};
use minijinja::{AutoEscape, Environment, UndefinedBehavior};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

// ---- Templates: one environment, every generated file renders through it. ----

/// All templates, embedded at compile time (cwd-independent; editing one triggers a rebuild).
const TEMPLATES: &[(&str, &str)] = &[
    ("tags.rs.j2", include_str!("../templates/tags.rs.j2")),
    ("fields.rs.j2", include_str!("../templates/fields.rs.j2")),
    ("message.rs.j2", include_str!("../templates/message.rs.j2")),
    ("field_methods.rs.j2", include_str!("../templates/field_methods.rs.j2")),
    ("group.rs.j2", include_str!("../templates/group.rs.j2")),
];

/// The minijinja environment every emitter renders with.
fn environment() -> Environment<'static> {
    let mut env = Environment::new();
    // A misspelled `{{ e.nme }}` must fail, not silently render as empty.
    env.set_undefined_behavior(UndefinedBehavior::Strict);
    // Rust source, not HTML: never escape quotes in literals.
    env.set_auto_escape_callback(|_| AutoEscape::None);
    // Drop the newline after a `{% %}` tag and the indentation before it, so block tags don't
    // leave blank lines (rustfmt normalizes the rest).
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    for &(name, source) in TEMPLATES {
        env.add_template(name, source)
            .unwrap_or_else(|e| panic!("{name} should be a valid template: {e:#}"));
    }
    env
}

/// Render the named template with a view model.
fn render(template: &str, view: &impl Serialize) -> String {
    environment()
        .get_template(template)
        .and_then(|t| t.render(view))
        .unwrap_or_else(|e| panic!("rendering {template} failed: {e:#}"))
}

// ---- Views: the `Serialize` inputs the templates render (every decision already made) ----

// tags.rs

/// Template input for `tags.rs.j2`.
#[derive(Debug, Serialize)]
struct TagsView {
    /// e.g. "FIX.4.3", for the file header.
    begin_string: String,
    /// Every field, sorted by tag number for a stable diff.
    tags: Vec<TagView>,
}

/// One `pub const <name>: u32 = <number>;` line of `tags.rs`.
#[derive(Debug, Serialize, Clone)]
struct TagView {
    /// SHOUTY_SNAKE const name, e.g. `MD_ENTRY_PX` (via `naming::const_name`).
    name: String,
    /// The tag number, e.g. 270.
    number: u32,
}

// Fields: one view per field, shared by fields.rs and every message file

/// One FIX field as the templates see it. Built once per field by [`field_view`] and shared:
/// `fields.rs.j2` uses the enum parts (`wire_type`, `wire_param_type`, `variants`),
/// `message.rs.j2` the accessor parts (`accessor`, `value_type`, `setter_param`, `tag_const`).
/// Flat on purpose: the enum parts are empty for fields without a value list.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct FieldView {
    /// The dictionary name, e.g. `EncryptMethod`. Also the key a message's [`Member::Field`]
    /// is looked up by.
    pub(crate) xml_name: String,
    /// Tag number, e.g. `98` — doc comments (`fields.rs.j2`).
    tag: u32,
    /// The `tags.rs` constant, e.g. `ENCRYPT_METHOD` — what accessors pass to `Message`
    /// (`message.rs.j2`).
    tag_const: String,
    /// Raw FIX type, e.g. `INT` — doc comments (`fields.rs.j2`).
    fix_type: String,
    /// snake_case accessor stem, e.g. `encrypt_method` — names `set_x` / `x` / `has_x`
    /// (`message.rs.j2`).
    accessor: String,
    /// What the getter returns. For a value-constrained field, the generated enum (`EncryptMethod`)
    /// — which is also the enum's type name in `fields.rs.j2`; otherwise the D8 Rust type
    /// (`String`, `bool`, `u32`, `Decimal`, …).
    value_type: String,
    /// What the setter takes: `value_type`, except strings are borrowed (`&str`).
    setter_param: String,
    /// Plain Rust type of the wire value, for value-constrained fields (empty otherwise):
    /// `to_fix()`'s return type (`fields.rs.j2`) and what an enum field's getter parses before
    /// calling `from_fix` (`message.rs.j2`). One of `char` / `i32` / `u32` / `&'static str`.
    wire_type: &'static str,
    /// `from_fix()`'s parameter type: `wire_type`, except a string is any `&str` (`to_fix` returns
    /// a literal, so must say `&'static str`; `from_fix` needn't). Empty when `wire_type` is.
    wire_param_type: &'static str,
    /// The enum's variants, sorted by wire code. Empty for fields without a value list, and for
    /// BOOLEAN fields (their `Y`/`N` values map to `bool`, not to a yes/no enum).
    variants: Vec<VariantView>,
}

/// One variant of a field's value enum.
#[derive(Debug, Clone, Serialize)]
struct VariantView {
    /// Rust variant name, e.g. `StopLimit` (via `naming::variant_name`).
    name: String,
    /// Raw wire code from the dictionary, e.g. `4` — doc comments only.
    code: String,
    /// The code as a Rust literal of the field's `wire_type`, e.g. `'4'`, `4` or `"4"` — the
    /// `to_fix` / `from_fix` match arms.
    literal: String,
}

// fields.rs (value enums)

/// Template input for `fields.rs.j2`.
#[derive(Debug, Serialize)]
struct FieldEnumsView {
    /// e.g. "FIX.4.3", for the file header.
    begin_string: String,
    /// The fields that get an enum (non-empty `variants`), in dictionary order.
    fields: Vec<FieldView>,
}

// messages/fixNN/generated/<message>.rs (message facades)

/// Template input for `message.rs.j2`: one message facade.
#[derive(Debug, Serialize)]
struct MessageView {
    /// e.g. "FIX.4.3", for the file header.
    begin_string: String,
    /// The message (and struct) name, e.g. `Logon`.
    name: String,
    /// "admin" or "app".
    category: String,
    /// The MsgType (tag 35) value, e.g. `A` — `MSG_TYPE` const and the header stamp.
    msg_type: String,
    /// The message's own fields after resolving components, in order (group members excluded:
    /// they are in `groups`).
    fields: Vec<MessageFieldView>,
    /// The message's top-level repeating groups, in order; nested groups hang off each one.
    groups: Vec<GroupView>,
}

/// A field as it appears in one particular message: the shared [`FieldView`] plus this
/// message's effective `required` (which differs between messages, so it isn't on `FieldView`).
#[derive(Debug, Serialize)]
struct MessageFieldView {
    /// Effective required-ness in this message, per [`resolve_items`].
    required: bool,
    /// The field itself.
    field: FieldView,
}

/// One repeating group as it appears in one particular message (D1: group types live in their
/// message's module), recursive for nested groups. Named after the group's count field.
#[derive(Debug, Serialize)]
struct GroupView {
    /// Read-only group view type, e.g. `NoMsgTypes` (UpperCamel count-field name).
    name: String,
    /// Mutable group view type, e.g. `NoMsgTypesMut`.
    name_mut: String,
    /// Read-only instance view type, e.g. `NoMsgTypesEntry`.
    entry_name: String,
    /// Mutable instance view type, e.g. `NoMsgTypesEntryMut`.
    entry_mut_name: String,
    /// snake_case name, e.g. `no_msg_types`: the parent's `no_x()` / `no_x_mut()` / `set_no_x(n)` /
    /// `add_no_x_entry(..)` methods.
    method_name: String,
    /// `tags::` const of the count field (NumInGroup), e.g. `NO_MSG_TYPES`.
    count_tag_const: String,
    /// `tags::` const of the delimiter, e.g. `REF_MSG_TYPE`: the group's first member, which the
    /// spec requires to be a field. Always `field_order[0]`.
    delim_tag_const: String,
    /// The instance's wire order: `tags::` consts of every member in dictionary order (a field's
    /// own tag, a nested group's count tag), e.g. `[REF_MSG_TYPE, MSG_DIRECTION]`. Passed to
    /// `add_group_instance` / `set_group`, which set it on each instance so it serializes in this
    /// order whatever order its fields are set in (FIX 4.3 Vol 1 §4).
    field_order: Vec<String>,
    /// Name of the generated const holding `field_order`, e.g. `NO_MSG_TYPES_FIELD_ORDER`.
    field_order_const: String,
    /// Effective required-ness of the group as an entry of its parent, per [`resolve_items`].
    required: bool,
    /// The group's own fields, in order (nested group members excluded: they are in `groups`).
    fields: Vec<MessageFieldView>,
    /// Groups nested directly in this one, in order. Splitting `fields` from `groups` loses their
    /// relative order, which methods don't need; the wire order is in `field_order`.
    groups: Vec<GroupView>,
}

// ---- Shared model: the resolved message / group body the views are built from ----

/// One entry of a message / group body after [`resolve_items`]: component references are
/// inlined and `required` is effective (contrast [`Item`], the raw XML entry).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Member {
    Field {
        name: String,
        required: bool,
    },
    Group {
        name: String,
        required: bool,
        members: Vec<Member>,
    },
}

// ---- View builders: XML definitions -> views (names, Rust types, literals, required flags) ----

/// Build the [`FieldView`] for one field definition. A field gets variants when it has a value
/// list and isn't BOOLEAN; then its `value_type` is the generated enum, otherwise the D8 type.
///
/// Panics on two values with the same wire code (QFJ's unpatched FIX43 `MatchType`) or the same
/// variant name — they would emit duplicate match arms / variants; fail here naming the field.
pub(crate) fn field_view(fd: &FieldDef) -> FieldView {
    let has_enum = !fd.values.is_empty() && fd.fix_type != "BOOLEAN";
    let (wire_type, variants) = if has_enum {
        let wire_type = wire_type_for(&fd.fix_type);
        let mut values = fd.values.iter().collect::<Vec<_>>();
        if matches!(wire_type, "i32" | "u32") {
            values.sort_by_key(|v| {
                v.code.parse::<i64>().unwrap_or_else(|_| {
                    panic!("{}: integer value `{}` should parse as a number", fd.name, v.code)
                })
            });
        } else {
            values.sort_by_key(|v| &v.code);
        }
        let variants: Vec<VariantView> = values
            .iter()
            .map(|v| VariantView {
                name: naming::variant_name(&v.description),
                code: v.code.clone(),
                literal: rust_literal(&v.code, wire_type),
            })
            .collect();
        let mut codes = HashSet::new();
        let mut names = HashSet::new();
        for v in &variants {
            assert!(codes.insert(&v.code), "{}: duplicate wire code `{}`", fd.name, v.code);
            assert!(names.insert(&v.name), "{}: duplicate variant name `{}`", fd.name, v.name);
        }
        (wire_type, variants)
    } else {
        ("", Vec::new())
    };

    let value_type = if has_enum {
        naming::enum_name(&fd.name)
    } else {
        value_type_for(&fd.fix_type).to_string()
    };
    let setter_param = if value_type == "String" {
        "&str".to_string()
    } else {
        value_type.clone()
    };

    FieldView {
        xml_name: fd.name.clone(),
        tag: fd.tag,
        tag_const: naming::const_name(&fd.name),
        fix_type: fd.fix_type.clone(),
        accessor: naming::method_name(&fd.name),
        value_type,
        setter_param,
        wire_type,
        wire_param_type: if wire_type.is_empty() {
            ""
        } else {
            wire_param_type(wire_type)
        },
        variants,
    }
}

/// Rust type of a value-constrained field's wire value (`FieldView::wire_type`). Only the FIX
/// types that carry value lists are valid; anything else panics so a new one surfaces loudly.
fn wire_type_for(fix_type: &str) -> &'static str {
    match fix_type {
        "CHAR" => "char",
        "INT" => "i32",
        "NUMINGROUP" => "u32",
        "STRING" | "MULTIPLEVALUESTRING" => "&'static str",
        _ => panic!("FIX type {fix_type} has values but no wire-type mapping"),
    }
}

/// `from_fix()`'s parameter type for a wire type (`FieldView::wire_param_type`): the same type,
/// except a `'static` string becomes any `&str`.
fn wire_param_type(wire_type: &'static str) -> &'static str {
    if wire_type == "&'static str" {
        "&str"
    } else {
        wire_type
    }
}

/// The getter's return type for a field *without* a value enum (`FieldView::value_type`): the
/// D8 FIX-type -> Rust-type map documented in `src/convert.rs`.
fn value_type_for(fix_type: &str) -> &'static str {
    match fix_type {
        "PRICE" | "AMT" | "QTY" | "PRICEOFFSET" => "Decimal",
        "FLOAT" | "PERCENTAGE" => "f64",
        "LENGTH" | "NUMINGROUP" | "SEQNUM" | "TAGNUM" | "DAYOFMONTH" => "u32",
        "BOOLEAN" => "bool",
        "INT" => "i32",
        "CHAR" => "char",
        "STRING"
        | "DATA"
        | "COUNTRY"
        | "CURRENCY"
        | "EXCHANGE"
        | "MONTHYEAR"
        | "MULTIPLEVALUESTRING" => "String",
        "UTCTIMESTAMP" => "DateTime<Utc>",
        "UTCDATE" | "LOCALMKTDATE" => "NaiveDate",
        "UTCTIMEONLY" => "NaiveTime",
        _ => panic!("FIX type {fix_type} has no Rust type mapping"),
    }
}

/// A wire code written as a Rust literal of `wire_type` (`VariantView::literal`): integers bare,
/// `char` in single quotes, strings in double quotes.
fn rust_literal(code: &str, wire_type: &str) -> String {
    match wire_type {
        "i32" | "u32" => code.to_string(),
        "char" => format!("'{code}'"),
        _ => format!("\"{code}\""),
    }
}

/// Inline component references and compute each member's effective `required`, matching the
/// runtime dictionary (`core/dictionary.rs`, itself mirroring QFJ's `DataDictionary`). `present`
/// is the enclosing container's context flag (`true` for a message):
/// - field, and a group as an entry: `own && present`
/// - a group's contents: the group's **own** flag
/// - a component's contents: the reference's **own** flag (the outer context is not applied)
fn resolve_items(spec: &FixSpec, items: &[Item], present: bool, out: &mut Vec<Member>) {
    for item in items {
        match item {
            Item::Field { name, required } => {
                out.push(Member::Field {
                    name: name.clone(),
                    required: *required && present,
                });
            }
            Item::Group {
                name,
                required,
                items,
            } => {
                let mut members = Vec::new();
                resolve_items(spec, items, *required, &mut members);
                out.push(Member::Group {
                    name: name.clone(),
                    required: *required && present,
                    members,
                });
            }
            Item::Component { name, required } => {
                let component = spec
                    .components
                    .get(name)
                    .unwrap_or_else(|| panic!("undefined component `{name}`"));
                resolve_items(spec, &component.items, *required, out);
            }
        }
    }
}

/// A field as it appears in `msg`: its shared [`FieldView`] plus its effective `required` there.
///
/// Panics, naming the message, if the dictionary doesn't define `name`.
fn member_to_field_view(
    name: &str,
    required: bool,
    msg: &MessageDef,
    field_views: &HashMap<String, FieldView>,
) -> MessageFieldView {
    MessageFieldView {
        required,
        field: field_views
            .get(name)
            .unwrap_or_else(|| panic!("message {} uses undefined field `{name}`", msg.name))
            .clone(),
    }
}

/// A repeating group as it appears in `msg` (`name` is its count field), with its members split
/// into `fields` and nested `groups` (recursively) and its wire order kept in `field_order`.
///
/// Panics, naming the group and message, if its first member isn't a field (the delimiter), or
/// (via [`member_to_field_view`]) if one of its fields isn't defined in the dictionary. The tag
/// consts (count, delimiter, `field_order`) are only named here, not looked up: an undefined
/// count field surfaces as a compile error in the generated code.
fn member_to_group_view(
    name: &str,
    required: bool,
    members: &[Member],
    msg: &MessageDef,
    field_views: &HashMap<String, FieldView>,
) -> GroupView {
    let group_fields: Vec<MessageFieldView> = members
        .iter()
        .filter_map(|m| match m {
            Member::Field { name, required } => {
                Some(member_to_field_view(name, *required, msg, field_views))
            }
            _ => None,
        })
        .collect();
    let subgroups: Vec<GroupView> = members
        .iter()
        .filter_map(|m| match m {
            Member::Group {
                name,
                required,
                members,
            } => Some(member_to_group_view(name, *required, members, msg, field_views)),
            _ => None,
        })
        .collect();
    let delim_tag_const = match members.first() {
        Some(Member::Field { name, .. }) => naming::const_name(name),
        _ => panic!(
            "group `{name}` in message `{}`: first member must be a field (the delimiter)",
            msg.name
        ),
    };
    // A field contributes its own tag, a nested group its count tag (its name): either way the
    // member's name is a dictionary field.
    let field_order = members
        .iter()
        .map(|m| match m {
            Member::Field { name, .. } | Member::Group { name, .. } => naming::const_name(name),
        })
        .collect();
    GroupView {
        name_mut: naming::group_name_mut(name),
        entry_name: naming::group_entry_name(name),
        entry_mut_name: naming::group_entry_mut_name(name),
        name: naming::group_name(name),
        method_name: naming::method_name(name),
        required,
        fields: group_fields,
        field_order,
        field_order_const: naming::group_field_order_const(name),
        count_tag_const: naming::const_name(name),
        delim_tag_const,
        groups: subgroups,
    }
}

// ---- Emitters: one per generated file kind; each builds its view and renders it ----

/// Render `src/tags.rs`: one `pub const <NAME>: u32 = <tag>;` per field, via `tags.rs.j2`.
pub(crate) fn emit_tags(spec: &FixSpec) -> String {
    let mut tags: Vec<TagView> = spec
        .fields
        .iter()
        .map(|f| TagView {
            name: naming::const_name(&f.name),
            number: f.tag,
        })
        .collect();
    tags.sort_by_key(|t| t.number);
    render(
        "tags.rs.j2",
        &TagsView {
            begin_string: spec.begin_string.clone(),
            tags,
        },
    )
}

/// Render `messages/fixNN/generated/fields.rs`: one enum (+ `to_fix`/`from_fix`) per
/// value-constrained field, via the `fields.rs.j2` template.
pub(crate) fn emit_field_enums(spec: &FixSpec) -> String {
    let fields: Vec<FieldView> =
        spec.fields.iter().map(field_view).filter(|f| !f.variants.is_empty()).collect();
    let view = FieldEnumsView {
        begin_string: spec.begin_string.clone(),
        fields,
    };
    render("fields.rs.j2", &view)
}

/// Render one message facade (`messages/fixNN/generated/<message>.rs`) via `message.rs.j2`.
/// `field_views` holds every field's [`FieldView`], keyed by dictionary name.
///
/// Panics if the message references a field the dictionary doesn't define.
pub(crate) fn emit_message(
    spec: &FixSpec,
    msg: &MessageDef,
    field_views: &HashMap<String, FieldView>,
) -> String {
    let mut members = Vec::new();
    resolve_items(spec, &msg.items, true, &mut members);
    let fields: Vec<MessageFieldView> = members
        .iter()
        .filter_map(|m| match m {
            Member::Field { name, required } => {
                Some(member_to_field_view(name, *required, msg, field_views))
            }
            Member::Group { .. } => None,
        })
        .collect();
    let groups = members
        .iter()
        .filter_map(|m| match m {
            Member::Group {
                name,
                required,
                members,
            } => Some(member_to_group_view(name, *required, members, msg, field_views)),
            _ => None,
        })
        .collect();
    let view = MessageView {
        begin_string: spec.begin_string.clone(),
        name: msg.name.clone(),
        category: msg.category.clone(),
        msg_type: msg.msg_type.clone(),
        fields,
        groups,
    };
    render("message.rs.j2", &view)
}

#[cfg(test)]
mod resolve_tests {
    //! `resolve_items` must give the same effective `required` as the runtime dictionary
    //! (`src/core/dictionary.rs`, itself mirroring QFJ's `DataDictionary`). Its rule: each
    //! container passes a *context* flag down —
    //!   - field:                  `own && context`
    //!   - group (as an entry):    `own && context`; its contents get the group's **own** flag
    //!   - component reference:    its contents get the reference's **own** flag (the outer
    //!                             context is NOT applied — a QFJ quirk we mirror on purpose)
    //!   - message:                context `true`
    //!
    //! Each scenario below is one of `dictionary_tests`' fixtures (same name, same XML); the
    //! expected flags are read off that test's `assert_msg` / `assert_group` required lists.
    use super::*;
    use crate::spec::parse_str;

    const Y: bool = true;
    const N: bool = false;

    fn f(name: &str, required: bool) -> Member {
        Member::Field {
            name: name.to_owned(),
            required,
        }
    }
    fn g(name: &str, required: bool, members: Vec<Member>) -> Member {
        Member::Group {
            name: name.to_owned(),
            required,
            members,
        }
    }

    /// The dictionary fixtures' shared components (dictionary_tests use them across scenarios).
    const COMPONENTS: &str = r#"
        <component name="CompWithOnlyFields">
            <field name="cfield1" required="Y"/>
            <field name="cfield2" required="N"/>
        </component>
        <component name="Comp2WithFields">
            <field name="gfield11" required="Y"/>
            <field name="gfield12" required="N"/>
        </component>
        <component name="CompWithFieldAndNonReqGroup">
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
        </component>
        <component name="CompWithOnlyNonReqGroup">
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
        </component>"#;

    /// Resolve the single message in `message_body` (wrapped as msgtype "X").
    fn resolve(message_body: &str) -> Vec<Member> {
        resolve_with(message_body, COMPONENTS)
    }

    fn resolve_with(message_body: &str, components: &str) -> Vec<Member> {
        let spec = parse_str(&format!(
            r#"<fix major="4" minor="3"><fields/><components>{components}</components><messages>
                 <message name="M" msgtype="X" msgcat="app">{message_body}</message>
               </messages></fix>"#
        ));
        let mut out = Vec::new();
        resolve_items(&spec, &spec.messages[0].items, true, &mut out);
        out
    }

    // ---- Message level ----------------------------------------------------------------------

    #[test]
    fn test_msgs_with_fields() {
        // Message context is `true`: a top-level field keeps its own flag.
        let r =
            resolve(r#"<field name="mfield1" required="Y"/><field name="mfield2" required="N"/>"#);
        assert_eq!(r, [f("mfield1", Y), f("mfield2", N)]);
    }

    #[test]
    fn test_msg_with_component() {
        // dictionary: required = [mfield1, cfield1]. A required component's required field stays
        // required; everything from the optional component becomes optional (inlined in place).
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <field name="mfield2" required="N"/>
               <component name="CompWithOnlyFields" required="Y"/>
               <component name="Comp2WithFields" required="N"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                f("mfield2", N),
                f("cfield1", Y),
                f("cfield2", N),
                f("gfield11", N),
                f("gfield12", N),
            ]
        );
    }

    #[test]
    fn test_msg_with_groups() {
        // dictionary: msg required = [mfield1, group1]; group1 required = [gfield11];
        // group2 (optional) required = none — gfield21 is Y in the XML but N effectively.
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <group name="group1" required="Y">
                 <field name="gfield11" required="Y"/>
                 <field name="gfield12" required="N"/>
               </group>
               <field name="mfield2" required="N"/>
               <group name="group2" required="N">
                 <field name="gfield21" required="Y"/>
                 <field name="gfield22" required="N"/>
               </group>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                g("group1", Y, vec![f("gfield11", Y), f("gfield12", N)]),
                f("mfield2", N),
                g("group2", N, vec![f("gfield21", N), f("gfield22", N)]),
            ]
        );
    }

    // ---- Components containing groups -------------------------------------------------------

    #[test]
    fn test_req_comp_having_group() {
        // dictionary: msg required = [mfield1, cfield1, cfield3, group2];
        // group1 required = none; group2 required = [gfield21].
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <component name="CompWithFieldAndNonReqGroup" required="Y"/>
               <field name="mfield2" required="N"/>
               <component name="CompWithFieldsAndReqGroup" required="Y"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                f("cfield1", Y),
                f("cfield2", N),
                g("group1", N, vec![f("gfield11", N), f("gfield12", N)]),
                f("mfield2", N),
                f("cfield3", Y),
                f("cfield4", N),
                g("group2", Y, vec![f("gfield21", Y), f("gfield22", N)]),
            ]
        );
    }

    #[test]
    fn test_non_req_comp_having_group() {
        // dictionary: msg required = [mfield1] only — the optional components make their fields
        // and the group2 entry optional — but group2's *contents* still require gfield21:
        // a group's contents take the group's own flag, not the component's.
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <component name="CompWithFieldAndNonReqGroup" required="N"/>
               <field name="mfield2" required="N"/>
               <component name="CompWithFieldsAndReqGroup" required="N"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                f("cfield1", N),
                f("cfield2", N),
                g("group1", N, vec![f("gfield11", N), f("gfield12", N)]),
                f("mfield2", N),
                f("cfield3", N),
                f("cfield4", N),
                g("group2", N, vec![f("gfield21", Y), f("gfield22", N)]),
            ]
        );
    }

    #[test]
    fn test_req_comp_having_only_group() {
        // dictionary: msg required = [mfield1, group2]; group1 none; group2 = [gfield22].
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <component name="CompWithOnlyReqGroup" required="Y"/>
               <component name="CompWithOnlyNonReqGroup" required="Y"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                g("group2", Y, vec![f("gfield21", N), f("gfield22", Y)]),
                g("group1", N, vec![f("gfield11", N), f("gfield12", N)]),
            ]
        );
    }

    #[test]
    fn test_non_req_comp_having_only_group() {
        // dictionary: msg required = [mfield1]; group2 contents still = [gfield22].
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <component name="CompWithOnlyReqGroup" required="N"/>
               <component name="CompWithOnlyNonReqGroup" required="N"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                g("group2", N, vec![f("gfield21", N), f("gfield22", Y)]),
                g("group1", N, vec![f("gfield11", N), f("gfield12", N)]),
            ]
        );
    }

    // ---- Groups containing components -------------------------------------------------------

    #[test]
    fn test_group_having_only_component() {
        // dictionary: group5 required = [cfield1].
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <group name="group5" required="Y">
                 <component name="CompWithOnlyFields" required="Y"/>
               </group>
               <field name="mfield2" required="N"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                g("group5", Y, vec![f("cfield1", Y), f("cfield2", N)]),
                f("mfield2", N),
            ]
        );
    }

    #[test]
    fn test_req_group_with_comps() {
        // dictionary: group5 required = [gfield21, cfield1].
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <group name="group5" required="Y">
                 <field name="gfield21" required="Y"/>
                 <component name="CompWithOnlyFields" required="Y"/>
                 <component name="Comp2WithFields" required="N"/>
               </group>
               <field name="mfield2" required="N"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                g(
                    "group5",
                    Y,
                    vec![
                        f("gfield21", Y),
                        f("cfield1", Y),
                        f("cfield2", N),
                        f("gfield11", N),
                        f("gfield12", N),
                    ]
                ),
                f("mfield2", N),
            ]
        );
    }

    #[test]
    fn test_non_req_group_with_comps() {
        // dictionary: group5 required = [cfield1] — the quirk. gfield21 (directly in the optional
        // group) is optional, but cfield1 (in a *required component* inside the optional group)
        // stays required: a component's contents take the reference's own flag only.
        let r = resolve(
            r#"<field name="mfield1" required="Y"/>
               <group name="group5" required="N">
                 <field name="gfield21" required="Y"/>
                 <component name="CompWithOnlyFields" required="Y"/>
                 <component name="Comp2WithFields" required="N"/>
               </group>
               <field name="mfield2" required="N"/>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                g(
                    "group5",
                    N,
                    vec![
                        f("gfield21", N),
                        f("cfield1", Y),
                        f("cfield2", N),
                        f("gfield11", N),
                        f("gfield12", N),
                    ]
                ),
                f("mfield2", N),
            ]
        );
    }

    #[test]
    fn test_msg_with_group_and_comps() {
        // dictionary: msg required = [mfield1, gfield11, group2]; group2 = [gfield21];
        // group4 = none.
        let r = resolve_with(
            r#"<field name="mfield1" required="Y"/>
               <component name="CompWithFieldsAndReqGroup" required="Y"/>
               <component name="CompWithOnlyFields" required="N"/>
               <field name="mfield2" required="N"/>
               <group name="group4" required="N">
                 <field name="gfield41" required="Y"/>
                 <field name="gfield42" required="N"/>
               </group>"#,
            // this dictionary test's CompWithFieldsAndReqGroup uses gfield11/12, not cfield3/4
            r#"<component name="CompWithFieldsAndReqGroup">
                 <field name="gfield11" required="Y"/>
                 <field name="gfield12" required="N"/>
                 <group name="group2" required="Y">
                   <field name="gfield21" required="Y"/>
                   <field name="gfield22" required="N"/>
                 </group>
               </component>
               <component name="CompWithOnlyFields">
                 <field name="cfield1" required="Y"/>
                 <field name="cfield2" required="N"/>
               </component>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", Y),
                f("gfield11", Y),
                f("gfield12", N),
                g("group2", Y, vec![f("gfield21", Y), f("gfield22", N)]),
                f("cfield1", N),
                f("cfield2", N),
                f("mfield2", N),
                g("group4", N, vec![f("gfield41", N), f("gfield42", N)]),
            ]
        );
    }

    // ---- Groups containing subgroups --------------------------------------------------------

    #[test]
    fn test_req_group_having_subgroups() {
        // dictionary: group6 required = [mfield2, group7]; group7 = [gfield41]; group3 = none.
        let r = resolve(
            r#"<field name="mfield1" required="N"/>
               <group name="group6" required="Y">
                 <field name="mfield2" required="Y"/>
                 <group name="group7" required="Y"><field name="gfield41" required="Y"/></group>
                 <group name="group3" required="N">
                   <field name="gfield31" required="Y"/>
                   <field name="gfield32" required="N"/>
                 </group>
               </group>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", N),
                g(
                    "group6",
                    Y,
                    vec![
                        f("mfield2", Y),
                        g("group7", Y, vec![f("gfield41", Y)]),
                        g("group3", N, vec![f("gfield31", N), f("gfield32", N)]),
                    ]
                ),
            ]
        );
    }

    #[test]
    fn test_non_req_group_having_subgroups() {
        // dictionary: group6 required = none (so the group7 *entry* is optional), but group7's
        // contents = [gfield41]: a subgroup's contents take the subgroup's own flag.
        let r = resolve(
            r#"<field name="mfield1" required="N"/>
               <group name="group6" required="N">
                 <field name="mfield2" required="Y"/>
                 <group name="group7" required="Y"><field name="gfield41" required="Y"/></group>
                 <group name="group3" required="N">
                   <field name="gfield31" required="Y"/>
                   <field name="gfield32" required="N"/>
                 </group>
               </group>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", N),
                g(
                    "group6",
                    N,
                    vec![
                        f("mfield2", N),
                        g("group7", N, vec![f("gfield41", Y)]),
                        g("group3", N, vec![f("gfield31", N), f("gfield32", N)]),
                    ]
                ),
            ]
        );
    }

    #[test]
    fn test_group_with_subgroup_as_first_field() {
        // dictionary: group6 required = [gfield21]; group7 is optional, so its gfield41 is too.
        let r = resolve(
            r#"<field name="mfield1" required="N"/>
               <group name="group6" required="Y">
                 <group name="group7" required="N"><field name="gfield41" required="Y"/></group>
                 <field name="gfield21" required="Y"/>
               </group>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", N),
                g("group6", Y, vec![g("group7", N, vec![f("gfield41", N)]), f("gfield21", Y)]),
            ]
        );
    }

    #[test]
    fn test_group_having_comp_and_subgroup() {
        // dictionary: msg = [group6]; group6 = [mfield2, group2, group3]; group2 = [gfield22];
        // group7 = none; group1 = none; group3 = [gfield31]. Every nesting shape at once:
        // group -> component -> group, and group -> subgroup -> component -> group.
        let r = resolve(
            r#"<field name="mfield1" required="N"/>
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
               </group>"#,
        );
        assert_eq!(
            r,
            [
                f("mfield1", N),
                g(
                    "group6",
                    Y,
                    vec![
                        f("mfield2", Y),
                        g("group2", Y, vec![f("gfield21", N), f("gfield22", Y)]),
                        g(
                            "group7",
                            N,
                            vec![
                                f("gfield41", N),
                                f("cfield1", N),
                                f("cfield2", N),
                                g("group1", N, vec![f("gfield11", N), f("gfield12", N)]),
                            ]
                        ),
                        g("group3", Y, vec![f("gfield31", Y), f("gfield32", N)]),
                    ]
                ),
            ]
        );
    }

    // ---- Beyond the dictionary fixtures -----------------------------------------------------

    #[test]
    fn component_inside_component_takes_only_its_own_reference_flag() {
        // dictionary.rs add_xml_component: "required attribute of each comp inside comp is
        // treated independently; it does not depend on outer component". So a required field in
        // a required Inner stays required even when Outer is optional.
        let r = resolve_with(
            r#"<component name="Outer" required="N"/>"#,
            r#"<component name="Outer">
                 <field name="o1" required="Y"/>
                 <component name="Inner" required="Y"/>
               </component>
               <component name="Inner"><field name="i1" required="Y"/></component>"#,
        );
        assert_eq!(r, [f("o1", N), f("i1", Y)]);
    }

    #[test]
    fn message_with_no_items_resolves_empty() {
        assert_eq!(resolve(""), []);
    }

    #[test]
    #[should_panic(expected = "NoSuchComponent")]
    fn reference_to_undefined_component_panics_naming_it() {
        // The parser keeps references unchecked; resolving is where a dangling one must fail,
        // and the message must say which (a bare `unwrap()` on None doesn't).
        resolve(r#"<component name="NoSuchComponent" required="Y"/>"#);
    }

    // ---- The real dictionary ----------------------------------------------------------------

    fn resolve_fix43(msg_type: &str) -> Vec<Member> {
        let spec = crate::spec::parse(&crate::fix43_xml_path());
        let msg = spec.messages.iter().find(|m| m.msg_type == msg_type).unwrap();
        let mut out = Vec::new();
        resolve_items(&spec, &msg.items, true, &mut out);
        out
    }

    #[test]
    fn fix43_logon_resolves_to_its_own_flags() {
        // No components: resolution is the identity on flags (the 3.8b target).
        assert_eq!(
            resolve_fix43("A"),
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
    fn fix43_new_order_single_inlines_components_with_their_flags() {
        // Instrument is required in D and requires Symbol -> Symbol required; Parties is optional
        // -> its NoPartyIDs entry optional. Components never appear as members.
        let r = resolve_fix43("D");
        let find = |name: &str| {
            r.iter()
                .find(|m| matches!(m, Member::Field { name: n, .. } | Member::Group { name: n, .. } if n == name))
                .unwrap_or_else(|| panic!("{name} should be a resolved member of D"))
                .clone()
        };
        assert_eq!(find("Symbol"), f("Symbol", Y));
        assert_eq!(find("SymbolSfx"), f("SymbolSfx", N));
        assert!(matches!(
            find("NoPartyIDs"),
            Member::Group {
                required: false,
                ..
            }
        ));
        assert_eq!(r[3], find("NoPartyIDs"), "Parties is inlined at its position (4th item)");
    }
}

#[cfg(test)]
mod group_view_tests {
    //! `member_to_group_view`: names, tag consts, wire order (`field_order`) and nesting of the
    //! `GroupView` the group templates render, plus its panics. Classes: flat vs nested group;
    //! first member written directly vs inlined from a component; plain vs corrected name; and the
    //! ways the input is rejected (first member a group, empty group, undefined member field).
    use super::*;
    use crate::spec::parse_str;

    /// Every field the fixtures use, numbered (tag consts come from these definitions).
    const FIELDS: &str = r#"
        <field number="1000" name="NoOuter" type="NUMINGROUP"/>
        <field number="1001" name="OuterDelim" type="STRING"/>
        <field number="1002" name="OuterFld" type="INT"/>
        <field number="1003" name="NoInner" type="NUMINGROUP"/>
        <field number="1004" name="InnerDelim" type="STRING"/>
        <field number="453" name="NoPartyIDs" type="NUMINGROUP"/>
        <field number="448" name="PartyID" type="STRING"/>
        <field number="452" name="PartyRole" type="INT"/>"#;

    /// The `GroupView` of the first top-level group in a message with body `body`.
    fn group_view_of(body: &str, components: &str) -> GroupView {
        let spec = parse_str(&format!(
            r#"<fix major="4" minor="3"><fields>{FIELDS}</fields><components>{components}</components>
               <messages><message name="M" msgtype="X" msgcat="app">{body}</message></messages></fix>"#
        ));
        let field_views: HashMap<String, FieldView> =
            spec.fields.iter().map(field_view).map(|f| (f.xml_name.clone(), f)).collect();
        let msg = &spec.messages[0];
        let mut members = Vec::new();
        resolve_items(&spec, &msg.items, true, &mut members);
        let (name, required, members) = members
            .iter()
            .find_map(|m| match m {
                Member::Group {
                    name,
                    required,
                    members,
                } => Some((name, *required, members)),
                Member::Field { .. } => None,
            })
            .expect("fixture has a group");
        member_to_group_view(name, required, members, msg, &field_views)
    }

    #[test]
    fn flat_group_names_consts_and_order() {
        let g = group_view_of(
            r#"<group name="NoOuter" required="N">
                 <field name="OuterDelim" required="Y"/>
                 <field name="OuterFld" required="N"/>
               </group>"#,
            "",
        );
        assert_eq!(
            (
                g.name.as_str(),
                g.name_mut.as_str(),
                g.entry_name.as_str(),
                g.entry_mut_name.as_str()
            ),
            ("NoOuter", "NoOuterMut", "NoOuterEntry", "NoOuterEntryMut")
        );
        assert_eq!(g.method_name, "no_outer");
        assert_eq!(g.count_tag_const, "NO_OUTER");
        assert_eq!(g.delim_tag_const, "OUTER_DELIM");
        assert_eq!(g.field_order, ["OUTER_DELIM", "OUTER_FLD"]);
        assert_eq!(g.field_order_const, "NO_OUTER_FIELD_ORDER");
        assert!(!g.required);
        let fields: Vec<_> = g.fields.iter().map(|f| f.field.xml_name.as_str()).collect();
        assert_eq!(fields, ["OuterDelim", "OuterFld"]);
        assert!(g.groups.is_empty());
    }

    // The nested group sits *between* the outer group's fields: `field_order` keeps that position
    // (its count tag), while `fields` / `groups` are split. The nested group gets its own view.
    #[test]
    fn nested_group_count_tag_keeps_its_position_in_field_order() {
        let g = group_view_of(
            r#"<group name="NoOuter" required="N">
                 <field name="OuterDelim" required="Y"/>
                 <group name="NoInner" required="N">
                   <field name="InnerDelim" required="N"/>
                 </group>
                 <field name="OuterFld" required="N"/>
               </group>"#,
            "",
        );
        assert_eq!(g.field_order, ["OUTER_DELIM", "NO_INNER", "OUTER_FLD"]);
        let fields: Vec<_> = g.fields.iter().map(|f| f.field.xml_name.as_str()).collect();
        assert_eq!(fields, ["OuterDelim", "OuterFld"]);
        assert_eq!(g.groups.len(), 1);
        let inner = &g.groups[0];
        assert_eq!(inner.name, "NoInner");
        assert_eq!(inner.delim_tag_const, "INNER_DELIM");
        assert_eq!(inner.field_order, ["INNER_DELIM"]);
        assert_eq!(inner.field_order_const, "NO_INNER_FIELD_ORDER");
    }

    // The delimiter can come from a component (FIX does this, e.g. NoLegs -> InstrumentLeg):
    // after inlining, the component's first field is the group's first member. Also the
    // corrected-name class: NoPartyIDs must come out as NoPartyIds / NO_PARTY_IDS everywhere.
    #[test]
    fn delimiter_inlined_from_component_and_corrected_name() {
        let g = group_view_of(
            r#"<group name="NoPartyIDs" required="N"><component name="Party" required="Y"/></group>"#,
            r#"<component name="Party">
                 <field name="PartyID" required="Y"/>
                 <field name="PartyRole" required="N"/>
               </component>"#,
        );
        assert_eq!(g.name, "NoPartyIds");
        assert_eq!(g.method_name, "no_party_ids");
        assert_eq!(g.count_tag_const, "NO_PARTY_IDS");
        assert_eq!(g.delim_tag_const, "PARTY_ID");
        assert_eq!(g.field_order, ["PARTY_ID", "PARTY_ROLE"]);
        assert_eq!(g.field_order_const, "NO_PARTY_IDS_FIELD_ORDER");
    }

    #[test]
    #[should_panic(expected = "group `NoOuter` in message `M`: first member must be a field")]
    fn first_member_a_group_panics() {
        group_view_of(
            r#"<group name="NoOuter" required="N">
                 <group name="NoInner" required="N"><field name="InnerDelim" required="N"/></group>
               </group>"#,
            "",
        );
    }

    #[test]
    #[should_panic(expected = "group `NoOuter` in message `M`: first member must be a field")]
    fn empty_group_panics() {
        group_view_of(r#"<group name="NoOuter" required="N"></group>"#, "");
    }

    // A field inside the group must be defined (checked through `member_to_field_view`). The
    // count field is not checked here by design: it only becomes a `tags::` reference, and an
    // undefined one fails when the generated code is compiled.
    #[test]
    #[should_panic(expected = "message M uses undefined field `Missing`")]
    fn undefined_member_field_panics() {
        group_view_of(
            r#"<group name="NoOuter" required="N">
                 <field name="OuterDelim" required="Y"/>
                 <field name="Missing" required="N"/>
               </group>"#,
            "",
        );
    }
}
