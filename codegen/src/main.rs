//! fix-rs typed-message code generator (standalone, xtask-style).
//!
//! Parses `resources/FIX43.xml` and emits **committed** Rust source into the engine crate's
//! `src/` (the D4a decision: generated code lives in the repo, not build-time — like
//! rust-analyzer's `xtask codegen` and BurntSushi's `ucd-generate`).
//!
//! - Generate + write:  `cargo run -p codegen`
//! - Check for drift:    `cargo test -p codegen`  (the freshness test below)
//!
//! Design (see `session_context/v1.1-codegen-design-discussion.md`):
//! - Emission: every generated file renders from a `minijinja` template (`codegen/templates/`),
//!   which reads like the code it emits. Output always goes through `rustfmt`, so template
//!   whitespace doesn't matter.
//! - [`generate`] is the one place that decides which files exist and renders them, one at a
//!   time, into a sink: the binary's sink writes each file to disk, the freshness test's sink
//!   compares it with the committed copy. The file list is computed (one file per generated
//!   message, plus the `mod.rs` index listing them), so both must go through the same function.
//! - Per-version generated code lives under `src/messages/fix43/generated/` (the generator owns
//!   that whole subtree — its `mod.rs` index too; hand-written tests sit in the sibling
//!   `src/messages/fix43/tests/`, never here — see design log D4c). Shared registries are
//!   `src/tags.rs` and `src/convert.rs`; per-version enums in `messages/fix43/generated/fields.rs`.

#![allow(dead_code)] // scaffold: parts of the spec model are unused until later emitters read them

mod audit;
mod emitter;
mod naming;
mod spec;

use crate::emitter::{FieldView, emit_field_enums, emit_message, emit_tags, field_view};
use crate::spec::FixSpec;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Printed to stderr on a bad invocation (unknown subcommand, or `audit` without a path).
const USAGE: &str = "\
usage:
  cargo run -p codegen                             generate the typed layer into src/
  cargo run -p codegen -- audit <dictionary.xml>   report FIX names that heck may case badly";

/// Workspace root (this crate's parent directory).
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR should have a parent (the workspace root)")
        .to_path_buf()
}

/// The engine crate's `src/` — where generated files land. Paths passed to a [`generate`] sink
/// are relative to this.
fn engine_src_root() -> PathBuf {
    workspace_root().join("src")
}

/// The FIX 4.3 dictionary the generator reads.
fn fix43_xml_path() -> PathBuf {
    workspace_root().join("resources/FIX43.xml")
}

/// Where generated message modules live (relative to engine `src/`). The generator owns this whole
/// subtree, `mod.rs` included; hand-written tests live in the sibling `tests/` (D4c).
const FIX43_GENERATED: &str = "messages/fix43/generated";

/// Render every generated file, in order, handing each to `sink(path, contents)` before rendering
/// the next. `path` is relative to engine `src/`; `contents` is unformatted (the sink runs
/// `rustfmt`). Pure and deterministic — stable ordering, no timestamps — so output is diffable.
///
/// Files: `tags.rs` (shared registry at the src root), then under [`FIX43_GENERATED`]:
/// `fields.rs`, one `<message>.rs` per generated message, and the `mod.rs` index listing them.
/// Never emits tests — they are hand-written under `messages/fix43/tests/` (D4c).
///
/// Names: convert XML names through `naming.rs` (corrections first, then `heck`) — never call
/// `heck` on a raw XML name, or the corrections are bypassed.
fn generate(spec: &FixSpec, mut sink: impl FnMut(&str, String)) {
    sink("tags.rs", emit_tags(spec));
    sink(&format!("{FIX43_GENERATED}/fields.rs"), emit_field_enums(spec));

    // Keyed by dictionary name — what a message's items refer to fields by.
    let field_views: HashMap<String, FieldView> =
        spec.fields.iter().map(field_view).map(|f| (f.xml_name.clone(), f)).collect();
    let mut modules = vec!["fields".to_owned()];
    for msg in spec
        .messages
        .iter()
        .filter(|m| m.category == "admin" || m.name == "MarketDataSnapshotFullRefresh")
    {
        let module = naming::module_name(&msg.name);
        sink(&format!("{FIX43_GENERATED}/{module}.rs"), emit_message(spec, msg, &field_views));
        modules.push(module);
    }

    let index: String = modules.iter().map(|m| format!("pub mod {m};\n")).collect();
    sink(&format!("{FIX43_GENERATED}/mod.rs"), index);
}

/// Run `rustfmt` over generated source (stdin → stdout). Normalizes formatting so hand-emitted
/// code and the committed files compare equal. Falls back to the unformatted input if `rustfmt`
/// isn't available.
fn rustfmt(src: String) -> String {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut child = match Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return src;
        }
    };
    // Best-effort: if the write or wait fails, return the input unchanged.
    if child.stdin.take().and_then(|mut s| s.write_all(src.as_bytes()).ok()).is_none() {
        return src;
    }
    match child.wait_with_output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).into_owned(),
        _ => src,
    }
}

/// Write every generated file (formatted) under engine `src/`, one at a time.
fn write_generated() {
    let spec = spec::parse(&fix43_xml_path());
    let root = engine_src_root();
    generate(&spec, |relative_path, contents| {
        let path = root.join(relative_path);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).expect("output dir should be creatable");
        }
        std::fs::write(&path, rustfmt(contents)).expect("generated file should be writable");
        println!("wrote {}", path.display());
    });
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("audit") => {
            if let Some(path) = std::env::args().nth(2) {
                if let Err(e) = audit::run_audit(&path) {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            } else {
                eprintln!("{USAGE}");
                std::process::exit(1);
            }
        }
        Some(_) => {
            eprintln!("{USAGE}");
            std::process::exit(1);
        }
        None => write_generated(),
    }
}

#[cfg(test)]
mod freshness {
    use super::*;

    /// Fails if any committed generated file differs from a fresh [`generate`] render — i.e.
    /// someone hand-edited generated code or changed the generator without regenerating. Fix by
    /// running `cargo run -p codegen` and committing the result.
    #[test]
    fn committed_output_is_fresh() {
        let spec = spec::parse(&fix43_xml_path());
        let root = engine_src_root();
        let mut stale = Vec::new();
        generate(&spec, |relative_path, contents| {
            let expected = rustfmt(contents);
            let actual = std::fs::read_to_string(root.join(relative_path)).unwrap_or_default();
            if actual != expected {
                stale.push(relative_path.to_owned());
            }
        });
        assert!(
            stale.is_empty(),
            "generated files are stale: {stale:?}\n\
             run `cargo run -p codegen` and commit the result",
        );
    }
}
