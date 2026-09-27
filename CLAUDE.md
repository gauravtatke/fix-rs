# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

fix-rs is a work-in-progress FIX protocol engine in Rust (inspired by QuickFIX/J — reference checkout at
`/Users/gauravtatke/quickfixj`). See **[ROADMAP.md](ROADMAP.md)** for the actual goals, scope, and current state before
making architectural decisions — notably: v1 targets FIX 4.3 with a **synchronous** network architecture by design
(async/Tokio is an explicit v2 goal, not now). It is also a Rust-learning project, but per the roadmap's refined goals,
**design decisions are made on product merit, not on learning value** — learning is an output, not an input. See
ROADMAP.md "Goals".

**Current state:** the v1 engine is complete and live-verified against QuickFIX/J — synchronous acceptor **and**
initiator, full session state machine (logon/heartbeat/test-request/logout, seq/CompID checks, scheduling), automatic
session-level error responses, and an application request/response flow. The project is now in **v1.1 (M3): typed
message codegen** — a generated per-message typed layer over the raw `Message` API. The task breakdown lives in
`TASKS.md` (kept on the `dev` branch).

## Crate layout: library + binary

The package builds **both** a library crate (`fix_rs`, the engine — root is `src/lib.rs`) and a **demo binary**
(`src/main.rs` + `src/sample_app.rs`) that uses it. `lib.rs` declares the module tree, carries the build-script
`include!`, and re-exports the public API (e.g. `pub use application::Application`). External code (and the demo bin)
uses `fix_rs::…` paths; see `examples/minimal_acceptor.rs` for a minimal library-usage example.

## Commands

- Build: `cargo build`
- Run the demo (loads `src/FixConfig.toml`, binds/dials per config): `cargo run`
- Run the example: `cargo run --example minimal_acceptor`
- Run all tests: `cargo test` (runs the `lib` target and the `bin` target separately)
- Run a single test: `cargo test test_name`
- Run tests in one module: `cargo test session_tests::`
- Format (settings in `rustfmt.toml`): `cargo fmt` — the tree is kept `cargo fmt --check` clean.

There is no CI config and no clippy config in this repo.

## Module layout

Version-neutral core with per-version messages layered on top:

- `core/` — the wire model (`message.rs`: `Message`/`FieldMap`/`Group` + parse/serialize/validate) and the runtime
  `DataDictionary` (`dictionary.rs`, parses `resources/FIX43.xml`). Both version-neutral.
- `session/` — the session state machine: `mod.rs` (`Session` + `verify`/`next_*`/admin builders + the `Responder`
  trait), `state.rs` (`SessionState`), `id.rs` (`SessionId`), `schedule.rs` (`SessionSchedule`), `settings.rs` (config).
- `transport/` — networking: `registry.rs` (`SessionMap`), `timer.rs` (`start_timer`, per-second heartbeat tick), and
  `sync/` (the `std::net` + `std::thread` implementation: `acceptor.rs`, `initiator.rs`, `connection.rs`, `reader.rs`,
  `responder.rs`). An async sibling would live under `transport/` later.
- `messages/` — the per-version typed message layer (`fix43/`; **WIP**, v1.1).
- Shared, version-neutral: `application.rs` (the `Application` trait + `DefaultApplication`), `errors.rs` (all error
  types), `convert.rs` (FIX value ⇄ Rust format: bool `Y`/`N`, `Decimal` money, dates/times), `tags.rs` (field→tag
  registry).

## Build-time code generation (legacy, being removed)

`build/main.rs` is a custom Cargo build script (`build = "build/main.rs"` in `Cargo.toml`). At compile time it parses
`resources/FIX43.xml` via `build/code_generator.rs`, renders Handlebars templates (`build/templates.rs`), and writes
`$OUT_DIR/fields.rs` + an `$OUT_DIR/mod.rs` (`pub mod fields;`). `src/lib.rs` pulls this in with
`include!(concat!(env!("OUT_DIR"), "/mod.rs"))`, making `fields::*` available (used by `core/message.rs` via
`crate::fields::*`) without being checked into the repo.

This is the **old** codegen and is slated for removal in **M3 task 3.9**, replaced by the standalone typed-message
generator (which emits committed source under `messages/fixNN/`). Note there are two separate `FixType` enums: one in
`build/code_generator.rs` (compile-time) and one in `core/dictionary.rs` (runtime, drives parsing/validation).

## Runtime architecture

**Startup (`src/main.rs`)**: loads `SessionProperties` from `src/FixConfig.toml`, builds one `SessionMap` holding every
session (both roles), starts the timer thread, then spawns one `IoAcceptor` thread per bind address and one
`IoInitiator` thread per initiator session, and joins them.

**Configuration (`src/session/settings.rs`)**: parsed with `toml` + `serde` — a `[Default]` block plus N `[[Session]]`
blocks; per-session values override the default (merge at the `toml::Value` level, then `try_into` a typed struct).
Validation lives in `TryFrom<FixProperties> for SessionConfig` (required fields, begin-string whitelist,
connection-type-specific port/host checks, start/end pairing). All supported keys are documented in
`src/FixConfig.example.toml`.

**Sessions (`src/session/mod.rs`)**: `Session` owns its `SessionId`, `SessionState`, `Arc<DataDictionary>`,
`Box<dyn Application>`, an optional `Box<dyn Responder>`, and config flags. `verify`/`next_message`/`next_tick` drive the
protocol; `on_app_msg_received` returns `Vec<Message>` (the app returns messages, the engine stamps headers and sends
them — no app→session cycle). `Session::verify` is fully implemented (not a stub).

**Registry & networking (`src/transport/`)**: `SessionMap` is an immutable `Arc<HashMap<SessionId, Arc<Mutex<Session>>>>`
built once via `FromIterator`; each session is independently lockable. `IoAcceptor` reads inbound connections and derives
the `SessionId` from the raw message; `IoInitiator` dials out and auto-reconnects. The shared read/dispatch pump lives in
`transport/sync/connection.rs`. Invariant: lock a session only per-message dispatch, never across a blocking read.

**Message model (`src/core/message.rs`)**: `Message` = `header` + `body` + `trailer` (each a `FieldMap`). `FieldMap`
stores fields (tag → raw string) plus nested `Group`s and preserves field order. Typed access is
`FieldMap::get_field::<T: FromStr>(tag)`; repeating groups use `add_group_instance`/`group_instances`/`body_group*`
(count auto-managed — fixes the old `set_group`/`add_group` desync).

**Data dictionary (`src/core/dictionary.rs`)**: parses `resources/FIX43.xml` (or a per-session `data_dictionary`
override) at runtime into lookup tables (field/tag maps, allowed values, types, per-message required/allowed fields,
nested groups). This is what `Message::from_str` and validation consult — a different parse pass from the build-time
codegen above.

## FIX protocol specification reference

The official FIX 4.3 spec (with errata) is available locally as Markdown:
`session_context/fix-specifications/FIX-43-with_errata_20020920/*.md` (7 volumes — Volume 1 covers message
structure/session-level rules). Prefer reading these over general knowledge when a question turns on exact protocol
behavior. `session_context/` is gitignored (local notes only), so these exist on disk but aren't in the tracked repo.
There's also a FIX 4.4 spec and a 2010 FIX repository archive alongside it.

## Gotchas

- `SessionSchedule` (`src/session/schedule.rs`) computes weekly session windows by walking backward/forward from today
  to the configured `start_day`/`end_day`; exercise both same-day and cross-week-boundary cases (see `schedule_tests`).
- The `Responder` trait lives in `session/mod.rs` (the session's outbound port); the TCP implementation
  (`TcpResponder`) is in `transport/sync/responder.rs`. A future async transport implements the same trait.
- `messages/fix43/group_variants/` is exploratory (the chosen group shape, `c_indexed`); it is intentionally untracked
  and not part of the build. The full A–D ergonomics comparison is on branch `explore/repeating-group-ergonomics`.
