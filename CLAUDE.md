# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project overview

fix-rs is a work-in-progress FIX protocol engine implementation in Rust (inspired by QuickFIX/J — reference checkout at
`/Users/gauravtatke/quickfixj`). See **[ROADMAP.md](ROADMAP.md)** for the project's actual goals, scope, and current
state before making architectural decisions — notably: v1 targets FIX 4.3/4.4 with a **synchronous** network
architecture by design (async/Tokio is an explicit v2 goal, not now), and this is as much a Rust-learning project as a
protocol implementation, so idiomatic/educational code matters as much as functionality.

The rest of this file (below) documents the *existing* code as of the initial pass, which is a Tokio-based async
prototype — that predates and conflicts with the v1 synchronous direction in the roadmap. Treat it as experimental
reference, not the target architecture, unless the roadmap changes. Message parsing and XML data-dictionary parsing are
the only functional pieces; session-level logic (logon/heartbeat/sequence number handling) is largely stubbed out
(`Session::verify` is a no-op, `Application` is a minimal trait with a `DefaultApplication` stub).

## Commands

- Build: `cargo build`
- Run the acceptor (binds ports from `src/FixConfig.toml` and blocks forever): `cargo run`
- Run all tests: `cargo test`
- Run a single test: `cargo test test_name` (e.g. `cargo test test_between_session`)
- Run tests in one module: `cargo test session_setting_tests::`
- Format (settings in `rustfmt.toml`): `cargo fmt`

There is no CI config and no clippy config in this repo.

## Build-time code generation

`build/main.rs` is a custom Cargo build script (`build = "build/main.rs"` in `Cargo.toml`). At compile time it:

1. Parses `resources/FIX43.xml` via `build/code_generator.rs` (`get_fix_spec`) into an `XmlFixSpec`.
2. Renders Handlebars templates from `build/templates.rs` (`FIELD_STRUCT`) to generate one Rust struct per FIX field.
3. Writes the generated code to `$OUT_DIR/fields.rs` plus an `$OUT_DIR/mod.rs` that does `pub mod fields;`.

`src/main.rs` pulls this in with `include!(concat!(env!("OUT_DIR"), "/mod.rs"))`, which is how the `fields::*` module
(e.g. `fields::MaxMessageSize` used in `session_and_state.rs`) becomes available without being checked into the repo. If
field-related types appear "missing," check `build/code_generator.rs`/`build/templates.rs` and the generated output
under `target/.../out/`, not `src/`.

Note: this build-time codegen is independent from and duplicates some logic in the runtime `DataDictionary` (see
below) — there are two separate `FixType` enums, one in `build/code_generator.rs` (compile-time, produces Rust primitive
types for generated field structs) and one in `src/data_dictionary.rs` (runtime, drives message parsing/validation).
`src/types.rs` contains a third, mostly dead/commented-out set of FIX primitive wrappers from an earlier design — treat
it as unused scaffolding, not a dependency.

## Runtime architecture

**Startup (`src/main.rs`)**: loads `Properties` from `src/FixConfig.toml`, constructs a `DefaultApplication`, builds a
`SocketAcceptor`, and calls `start_accepting_connections()`.

**Configuration (`src/session/session_settings.rs`)**: `Properties` hand-parses a TOML-like file (NOT via the `toml`
crate — `Properties::from_str` does manual line/section parsing). It requires a `[Default]` section first, followed by
any number of `[Session]` sections; per-session values override defaults. `Properties::check()` panics on
invalid/missing mandatory settings (connection type, ports, begin string, comp IDs), so config errors surface as panics
at startup, not `Result`s.

**Session identity (`src/session/session_id.rs`)**: `SessionId` is built via `SessionIdBuilder` (derive_builder) from
`(begin_string, sender_compid, target_compid, ...)` and computes a composite `id` string used for `Hash`
/equality/lookup.

**Sessions (`src/session/session_and_state.rs`)**: `Session` holds per-session state (heartbeat interval, reset flags,
an `Arc<DataDictionary>`, and an optional `responder` — a `TioBroadcastSender<String>` used to push outbound wire
messages to that session's socket writer task). `Session::sync_send_to_target` looks up the session in the shared
`SessionMap` and sends the serialized message string through its responder.

**Session registry (`src/network.rs`)**: `SessionMap` wraps a `DashMap<SessionId, Session>` for concurrent access across
tasks. `SocketAcceptor` groups sessions by their bind `SocketAddr` (`SocketDescriptor`) since multiple sessions can
share one listening port, and wires each session's `responder` to the `IoAcceptor` for its socket.

**Network IO (`src/io/acceptor.rs`)**: `IoAcceptor::start()` spawns a Tokio task that loops accepting TCP connections on
a bind address. Each accepted connection gets:

- a reader task (`start_socket_listener_task`) that reads SOH-delimited (`\x01`) FIX bytes until it sees the checksum
  tag `10=`, then forwards the raw message string over an `mpsc` channel to the application-side receiver task;
- a writer task (`start_app_listner_task`) that subscribes to a `broadcast` channel and writes any outbound message
  strings to the socket.

**Message dispatch (`start_receiver_task` in `src/network.rs`)**: a dedicated OS thread drains the `mpsc` receiver,
derives the `SessionId` from the raw message (`Message::get_reverse_session_id`), looks up that session's
`DataDictionary`, parses the raw string into a `Message`, and (if `Session::verify` passes) invokes
`Application::from_app`. This is the seam where inbound FIX messages become application-visible.

**Message model (`src/message.rs`)**: `Message` = `header` + `trailer` + `body`, where `FieldMap` stores `StringField`s
(tag -> raw string value) plus nested `Group`s and preserves field order. Field typed access goes through
`FieldMap::get_field::<T: FromStr>(tag)`.

**Data dictionary (`src/data_dictionary.rs`)**: parses `resources/FIX43.xml` (or a per-session override via the
`data_dictionary` config key) at runtime into lookup tables: field name/tag mappings, allowed field values, field types,
per-message-type required/allowed fields, and nested repeating groups (`GroupInfo`). This is what `Message::from_str`
and validation consult — it is a different parse pass from the build-time codegen described above.

## FIX protocol specification reference

The official FIX 4.3 spec (with errata) is available locally as Markdown, converted from the official spec docs:
`session_context/fix-specifications/FIX-43-with_errata_20020920/*.md`
(7 volumes — Volume 1 covers message structure/session-level rules, others cover categories of application
messages/fields). Prefer reading these over relying on general knowledge when a question turns on exact protocol
behavior (field ordering rules, required fields, session-level reject reasons, etc.) — `session_context/` is gitignored
(local notes only), so these files exist on disk but aren't part of the tracked repo. There's also a FIX 4.4 spec and a
2010 FIX repository archive alongside it in the same directory.

## Gotchas

- `SessionSchedule` (`src/session/session_schedule.rs`) computes weekly session windows by walking backward/forward from
  today to the configured `start_day`/`end_day`; if you touch this logic, exercise both the same-day and
  cross-week-boundary cases (see `schedule_tests`).
- Much of `src/network.rs` and `src/io/acceptor.rs` contains large commented-out blocks from earlier iterations of the
  connection-handling design — don't assume commented code reflects a supported path.
