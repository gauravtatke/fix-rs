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
//! - `OUTPUTS` is one table of `(path, emitter fn)`; the binary renders + writes each file in
//!   turn, the test renders + compares. One source of truth for both.
//! - Per-version generated code lives under `src/messages/fix43/generated/` (the generator owns
//!   that whole subtree — its `mod.rs` index too; hand-written tests sit in the sibling
//!   `src/messages/fix43/tests/`, never here — see design log D4c). Shared registries are
//!   `src/tags.rs` and `src/convert.rs`; per-version enums in `messages/fix43/generated/fields.rs`.

#![allow(dead_code)] // scaffold: spec fields are unused until the emitters read them

mod audit;
mod emitter;
mod naming;
mod spec;

use crate::spec::FixSpec;
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

/// The engine crate's `src/` — where generated files land. Paths in [`OUTPUTS`] are relative to
/// this.
fn engine_src_root() -> PathBuf {
    workspace_root().join("src")
}

/// The FIX 4.3 dictionary the generator reads.
fn fix43_xml_path() -> PathBuf {
    workspace_root().join("resources/FIX43.xml")
}

/// An emitter: renders one generated file's contents from the parsed dictionary. Pure and
/// deterministic (stable ordering, no timestamps) so the output is diffable.
type Emit = fn(&FixSpec) -> String;

/// Every generated file: `(path relative to engine src/, its emitter)`. The binary renders and
/// writes these one at a time; the freshness test renders and compares them. One table, so the
/// two can't disagree about which files exist.
///
/// Next, in order: `Logon` (`…/generated/logon.rs`), then the generated `…/generated/mod.rs`
/// index. `tags.rs` is a shared registry at the src root, not under `generated/`. Never emit
/// tests — they are hand-written under `messages/fix43/tests/` (D4c).
///
/// Names: convert XML names through `naming.rs` (corrections first, then `heck`) — never call
/// `heck` on a raw XML name, or the corrections are bypassed. `ToShoutySnakeCase` for tag consts,
/// `ToSnakeCase` for accessors, `ToUpperCamelCase` for types and enum variants.
const OUTPUTS: &[(&str, Emit)] = &[
    ("tags.rs", emitter::emit_tags),
    ("messages/fix43/generated/fields.rs", emitter::emit_field_enums),
];

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

/// Render, format and write each generated file in turn, so a file is on disk before the next
/// one is rendered.
fn write_generated() {
    let spec = spec::parse(&fix43_xml_path());
    let root = engine_src_root();
    for &(rel, emit) in OUTPUTS {
        let path = root.join(rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).expect("output dir should be creatable");
        }
        std::fs::write(&path, rustfmt(&emit(&spec))).expect("generated file should be writable");
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

    /// Fails if any committed generated file differs from a fresh render of its emitter — i.e.
    /// someone hand-edited generated code or changed the generator without regenerating. Fix by
    /// running `cargo run -p codegen` and committing the result.
    #[test]
    fn committed_output_is_fresh() {
        let spec = spec::parse(&fix43_xml_path());
        let root = engine_src_root();
        let mut stale = Vec::new();
        for &(rel, emit) in OUTPUTS {
            let expected = rustfmt(&emit(&spec));
            let actual = std::fs::read_to_string(root.join(rel)).unwrap_or_default();
            if actual != expected {
                stale.push(rel);
            }
        }
        assert!(
            stale.is_empty(),
            "generated files are stale: {stale:?}\n\
             run `cargo run -p codegen` and commit the result",
        );
    }
}
