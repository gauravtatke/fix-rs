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
//! - Emission is plain string building + `rustfmt` (not a template engine, not `quote`) — full
//!   control of doc-comments and exact bytes, which the freshness diff needs.
//! - `generate()` is a pure function `-> Vec<(path, contents)>`; the binary writes, the test
//!   compares. One source of truth for both.
//! - Per-version generated code lives under `src/messages/fix43/generated/` (the generator owns
//!   that whole subtree — its `mod.rs` index too; hand-written tests sit in the sibling
//!   `src/messages/fix43/tests/`, never here — see design log D4c). Shared registries are
//!   `src/tags.rs` and `src/convert.rs`; per-version enums in `messages/fix43/generated/fields.rs`.

#![allow(dead_code)] // scaffold: spec fields are unused until the emitters read them

mod audit;
mod naming;
mod spec;

use crate::spec::FixSpec;
use std::fmt::Write as _;
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

/// The engine crate's `src/` — where generated files land. Paths from `generate()` are relative
/// to this.
fn engine_src_root() -> PathBuf {
    workspace_root().join("src")
}

/// The FIX 4.3 dictionary the generator reads.
fn fix43_xml_path() -> PathBuf {
    workspace_root().join("resources/FIX43.xml")
}

/// The whole generation as a pure function: `(path relative to engine src/, file contents)`.
/// The binary writes these; the freshness test compares them. Keep it deterministic (stable
/// ordering, no timestamps) so the output is diffable.
///
/// One entry per generated file. `tags.rs` is done; next, in order: the value enums
/// (`messages/fix43/generated/fields.rs`), then `Logon` (`…/generated/logon.rs`), then the
/// generated `…/generated/mod.rs` index. `tags.rs` is a shared registry at the src root, not under
/// `generated/`. Never emit tests — they are hand-written under `messages/fix43/tests/` (D4c).
///
/// Names: convert XML names through `naming.rs` (corrections first, then `heck`) — never call
/// `heck` on a raw XML name, or the corrections are bypassed. `ToShoutySnakeCase` for tag consts,
/// `ToSnakeCase` for accessors, `ToUpperCamelCase` for types and enum variants.
fn generate() -> Vec<(PathBuf, String)> {
    let spec = spec::parse(&fix43_xml_path());
    vec![(PathBuf::from("tags.rs"), emit_tags(&spec))]
}

fn emit_tags(spec: &FixSpec) -> String {
    let mut sorted_field_ref = spec.fields.iter().collect::<Vec<_>>();
    sorted_field_ref.sort_by_key(|a| a.tag);

    let mut output = String::with_capacity(1024);
    // Module doc for the generated file. Each `\` at a line end continues the string literal and
    // skips the next line's leading whitespace, so every `//!` line lands at column 0.
    writeln!(
        output,
        "//! Shared FIX field-tag registry: `field name -> tag number`.\n\
         //!\n\
         //! **Generated — do not edit by hand.** Emitted by `cargo run -p codegen` from the {} data\n\
         //! dictionary; `cargo test -p codegen` fails if this file drifts from the generator's output.\n\
         //!\n\
         //! Version-neutral by design: FIX tag numbers are globally stable (a given number always\n\
         //! denotes the same field), so this one registry is shared by every `messages::fixNN` module\n\
         //! rather than duplicated per version. Sorted by tag number.\n",
        spec.begin_string
    )
        .unwrap();
    for field in sorted_field_ref {
        writeln!(output, "pub const {}: u32 = {};", naming::const_name(&field.name), field.tag)
            .unwrap();
    }
    output
}

/// Run `rustfmt` over generated source (stdin → stdout). Normalizes formatting so hand-emitted
/// code and the committed files compare equal. Falls back to the unformatted input if `rustfmt`
/// isn't available.
fn rustfmt(src: &str) -> String {
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
        Err(_) => return src.to_owned(),
    };
    // Best-effort: if the write or wait fails, return the input unchanged.
    if child.stdin.take().and_then(|mut s| s.write_all(src.as_bytes()).ok()).is_none() {
        return src.to_owned();
    }
    match child.wait_with_output() {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).into_owned(),
        _ => src.to_owned(),
    }
}

fn write_generated() {
    let root = engine_src_root();
    let files = generate();
    if files.is_empty() {
        println!("codegen: nothing to emit yet (generate() is a stub — add emit logic).");
        return;
    }
    for (rel, contents) in files {
        let path = root.join(&rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).expect("output dir should be creatable");
        }
        std::fs::write(&path, rustfmt(&contents)).expect("generated file should be writable");
        println!("wrote {}", path.display());
    }
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

    /// Fails if any committed generated file differs from a fresh `generate()` run — i.e. someone
    /// hand-edited generated code or changed the generator without regenerating. Fix by running
    /// `cargo run -p codegen` and committing the result.
    ///
    /// (Trivially passes while `generate()` is a stub; becomes meaningful as emitters are added.)
    #[test]
    fn committed_output_is_fresh() {
        let root = engine_src_root();
        let mut stale = Vec::new();
        for (rel, contents) in generate() {
            let expected = rustfmt(&contents);
            let actual = std::fs::read_to_string(root.join(&rel)).unwrap_or_default();
            if actual != expected {
                stale.push(rel.display().to_string());
            }
        }
        assert!(
            stale.is_empty(),
            "generated files are stale: {stale:?}\n\
             run `cargo run -p codegen` and commit the result",
        );
    }
}
