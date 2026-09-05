# ROADMAP

This is the working design/goal doc for fix-rs. It's expected to change as the project evolves — treat it as a living document, not a fixed spec.

## Goals

1. **Learn Rust and become proficient in it.** Current level: between beginner and intermediate. This is a primary goal, not a side effect — code choices should favor idiomatic Rust and understanding over speed of delivery.
2. **Build a fully functional FIX protocol implementation in Rust.**

## Scope

### V1
- FIX **4.3** only to start. FIX 4.4 is deferred until the architecture is proven on one version — the dictionary-driven design should port to a second XML dictionary with minimal changes, but that's a later milestone, not v1 core.
- **Synchronous** network architecture, deliberately, for learning purposes (understanding blocking IO, threads, etc. before reaching for async).
- Config loaded via `toml` + `serde::Deserialize` (replacing the current hand-rolled INI-style parser), modeling QuickFIX/J's "one `[Default]` block + N per-session override blocks" merge pattern.
- "Done" bar for v1 is a **happy path**: Logon, Heartbeat, TestRequest, and Logout working end-to-end between an initiator and an acceptor, plus two typed application-level messages exchanged end-to-end — `MarketDataRequest` (35=V, subscribe) and `MarketDataSnapshotFullRefresh` (35=W, response). No resend request / gap fill / sequence number persistence in v1.

### V1.1 and beyond
- **v1.1**: `NewOrderSingle` placed, accepted and rejected (`ExecutionReport` flow).
- **v1.2+**: other message types, plus protocol robustness — resend request, gap fill, sequence number persistence.

### V2 (future)
- Introduce **Tokio** and the async stack as a feature enhancement on top of a working synchronous v1.
- **Structured logging infrastructure**: replace direct `log::info!` calls with a proper `Log` trait (incoming/outgoing/event categories, per-session context) and an async logging backend. On low-latency paths, synchronous logging can stall the message-processing thread — the logger should hand off formatted output to a dedicated writer thread (or ring buffer) so the hot path never blocks on I/O. Design should support pluggable backends (screen, file, network) similar to QFJ's `LogFactory`/`Log` abstraction.

Note: the current codebase (as of the initial CLAUDE.md pass) already has a Tokio-based async prototype in `src/io/`, `src/network.rs`, etc. That predates this roadmap and is experimental — it does not reflect the v1 synchronous direction and can be discarded/rewritten as needed.

See **[TASKS.md](TASKS.md)** for the v1 work broken into independently-pickupable tasks (milestones M1–M6).

## Inspiration

Based on **QuickFIX/J** (pure Java FIX implementation). Local checkout for reference: `/Users/gauravtatke/quickfixj`. Familiarity comes from testing applications built on it (as a software tester), not from having built with it directly — so QuickFIX/J is a reference for protocol behavior and structure, not a spec to copy blindly. Deviate from its architecture where the Rust tech stack calls for it.

### Understanding of QuickFIX/J's structure (to validate/adapt, not assume correct)
1. A base layer of classes/methods handling generic message send/receive (transport-agnostic protocol mechanics: sessions, sequence numbers, message store, etc.).
2. On top of that, protocol-version-specific generated message classes giving strict type checking (e.g. a `NewOrderSingle` type with typed field accessors, rather than raw tag/value lookups).
3. fix-rs should aim for a similar two-layer shape: a generic engine layer, plus generated/typed message types per FIX version.

## Current project state

- **M1 (message/dictionary layer hardening) is done.** Tasks 1.1–1.5 complete; 1.6 (SOH-in-Data-field handling) explicitly descoped as a stretch goal. `Message`/`FieldMap` parsing is fully dictionary-driven with all validation going through `SessionRejectError`/`SessionRejectReason`.
- **M2 (config rewrite) is done.** `toml`+`serde` based config with default/session merge. Hand-rolled parser deleted. 14 config tests.
- **M3 (typed message codegen) is not started.** Next up — rework build-time codegen to emit per-message-type structs.
- **M4 (session state machine) is done.** `SessionState`, `SessionId`, `Application` trait, `Responder` trait, `Session::verify`, admin message builders (`generate_logon`/`logout`/`heartbeat`/`test_request`), inbound dispatch (`next_message`), and timer logic (`next_tick`). All testable without real sockets via `MockResponder`.
- **M5 (sync networking) is done.** Tokio prototype replaced with `std::net`/`std::thread`. `FixMessageReader`, `TcpResponder`, `SessionMap`, `IoAcceptor` (binds `TcpListener`, spawns reader thread per connection), timer thread, disconnect cleanup, and reset flags (`reset_on_logon`/`reset_on_logout`/`reset_on_disconnect`). Integration-tested against QFJ Banzai — Logon handshake and heartbeat exchange run cleanly. 172 tests passing.
- **M6 (close out v1) is not started.** Wire `MarketDataRequest`/`MarketDataSnapshotFullRefresh` end-to-end — depends on M3.
