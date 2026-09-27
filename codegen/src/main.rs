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
//! - Targets (per-version) live under `src/messages/fix43/`; shared registries are `src/tags.rs`
//!   and `src/convert.rs`; enums per-version in `messages/fix43/fields.rs`.

#![allow(dead_code)] // scaffold: spec fields are unused until the emitters read them

mod spec;

use std::path::{Path, PathBuf};

/// Workspace root (this crate's parent directory).
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("codegen has a parent dir").to_path_buf()
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
/// TODO — the emit logic (your part). Sketch:
///   1. Parse `fix43_xml_path()` into a codegen model (field name/tag/type, ordered values +
///      descriptions, per-message ordered fields + required flags, groups). Base it on the old
///      `build/code_generator.rs` `XmlFixSpec` parse — NOT the runtime `DataDictionary`, which is
///      lossy for codegen (drops value descriptions/order and field order).
///   2. Push one entry per generated file, e.g.:
///        files.push((PathBuf::from("tags.rs"),                  emit_tags(&spec)));
///        files.push((PathBuf::from("messages/fix43/fields.rs"), emit_fields(&spec)));
///        files.push((PathBuf::from("messages/fix43/logon.rs"),  emit_logon(&spec)));
///   Start with `tags.rs` (pure data) to prove the pipeline, then enums, then Logon.
///
/// Casing helpers: `heck` (`ToUpperCamelCase` for enum variants from SCREAMING_SNAKE descriptions,
/// `ToSnakeCase` for accessor names, `ToShoutySnakeCase` for tag consts).
fn generate() -> Vec<(PathBuf, String)> {
    let _spec = spec::parse(&fix43_xml_path());
    let files: Vec<(PathBuf, String)> = Vec::new();
    // files.push((PathBuf::from("tags.rs"), emit_tags(&_spec)));
    files
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

fn main() {
    let root = engine_src_root();
    let files = generate();
    if files.is_empty() {
        println!("codegen: nothing to emit yet (generate() is a stub — add emit logic).");
        return;
    }
    for (rel, contents) in files {
        let path = root.join(&rel);
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).expect("create output dir");
        }
        std::fs::write(&path, rustfmt(&contents)).expect("write generated file");
        println!("wrote {}", path.display());
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
