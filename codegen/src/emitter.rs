use crate::naming;
use crate::spec::{FieldDef, FixSpec};
use minijinja::{AutoEscape, Environment, UndefinedBehavior};
use serde::Serialize;
use std::collections::HashSet;

// ---- Templates: one environment, every generated file renders through it. ----

/// All templates, embedded at compile time (cwd-independent; editing one triggers a rebuild).
const TEMPLATES: &[(&str, &str)] = &[
    ("tags.rs.j2", include_str!("../templates/tags.rs.j2")),
    ("fields.rs.j2", include_str!("../templates/fields.rs.j2")),
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

// ---- tags.rs ----

#[derive(Debug, Serialize)]
struct TagsView {
    /// e.g. "FIX.4.3", for the file header.
    begin_string: String,
    /// Every field, sorted by tag number for a stable diff.
    tags: Vec<Tag>,
}

#[derive(Debug, Serialize)]
struct Tag {
    /// SHOUTY_SNAKE const name, e.g. `MD_ENTRY_PX`.
    name: String,
    number: u32,
}

/// Render `src/tags.rs`: one `pub const <NAME>: u32 = <tag>;` per field, via `tags.rs.j2`.
pub(crate) fn emit_tags(spec: &FixSpec) -> String {
    let mut tags: Vec<Tag> = spec
        .fields
        .iter()
        .map(|f| Tag {
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

// ---- fields.rs (value enums) ----

fn formatted_code(code: &str, rust_type: &str) -> String {
    if rust_type == "i32" {
        code.to_string()
    } else if rust_type == "char" {
        format!("'{}'", code)
    } else {
        format!("\"{}\"", code)
    }
}

fn formatted_rust_type(ftype: &str) -> &'static str {
    match ftype {
        "INT" | "NUMINGROUP" => "i32",
        "CHAR" => "char",
        "STRING" | "MULTIPLEVALUESTRING" => "&'static str",
        _ => panic!("Unknown return type {}", ftype),
    }
}

/// Parameter type for `from_fix`: same as `to_fix`'s return type, except a string is borrowed
/// for any lifetime (the caller's `&str`), not only `'static`.
fn from_fix_arg_type(rust_type: &'static str) -> &'static str {
    if rust_type == "&'static str" {
        "&str"
    } else {
        rust_type
    }
}

// View model: everything the template needs, already decided (names, literals, order).

#[derive(Debug, Serialize)]
struct FieldEnumsView {
    /// e.g. "FIX.4.3", for the file header.
    begin_string: String,
    enums: Vec<FieldEnum>,
}

#[derive(Debug, Serialize)]
struct FieldEnum {
    /// Rust type name, e.g. `OrdType`.
    name: String,
    tag: u32,
    /// Raw FIX type, e.g. `CHAR` (doc comment only).
    fix_type: String,
    /// `to_fix` return type, e.g. `char`.
    rust_type: &'static str,
    /// `from_fix` parameter type, e.g. `char` or `&str`.
    arg_type: &'static str,
    /// Sorted by wire code.
    variants: Vec<Variant>,
}

#[derive(Debug, Serialize)]
struct Variant {
    /// Rust variant name, e.g. `StopLimit`.
    name: String,
    /// Raw wire code, e.g. `4` (doc comment only).
    code: String,
    /// The code as a Rust literal of `rust_type`, e.g. `'4'`.
    literal: String,
}

fn to_field_enum(fd: &FieldDef) -> FieldEnum {
    let mut sorted_values = fd.values.iter().collect::<Vec<_>>();
    let rust_type = formatted_rust_type(&fd.fix_type);
    if rust_type == "i32" {
        sorted_values.sort_by_key(|v| {
            v.code.parse::<i32>().unwrap_or_else(|_| {
                panic!("{}: INT value `{}` should parse as i32", fd.name, v.code)
            })
        });
    } else {
        sorted_values.sort_by_key(|v| &v.code);
    }
    let variants: Vec<Variant> = sorted_values
        .iter()
        .map(|v| Variant {
            name: naming::variant_name(&v.description),
            code: v.code.clone(),
            literal: formatted_code(&v.code, rust_type),
        })
        .collect();

    // Two values with the same code (QFJ's FIX43 `MatchType` has this) or the same variant name
    // would emit duplicate match arms / variants. Fail here with the field named, not in rustc.
    let mut codes = HashSet::new();
    let mut names = HashSet::new();
    for v in &variants {
        assert!(codes.insert(&v.code), "{}: duplicate wire code `{}`", fd.name, v.code);
        assert!(names.insert(&v.name), "{}: duplicate variant name `{}`", fd.name, v.name);
    }

    let rust_type = formatted_rust_type(&fd.fix_type);
    FieldEnum {
        name: naming::enum_name(&fd.name),
        tag: fd.tag,
        fix_type: fd.fix_type.clone(),
        rust_type,
        arg_type: from_fix_arg_type(rust_type),
        variants,
    }
}

/// Render `messages/fixNN/generated/fields.rs`: one enum (+ `to_fix`/`from_fix`) per
/// value-constrained field, via the `fields.rs.j2` template.
pub(crate) fn emit_field_enums(spec: &FixSpec) -> String {
    let enums: Vec<FieldEnum> = spec
        .fields
        .iter()
        .filter(|f| !f.values.is_empty() && f.fix_type != "BOOLEAN")
        .map(to_field_enum)
        .collect();
    let view = FieldEnumsView {
        begin_string: spec.begin_string.clone(),
        enums,
    };
    render("fields.rs.j2", &view)
}
