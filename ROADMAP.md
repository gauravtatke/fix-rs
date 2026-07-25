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

Note: the current codebase (as of the initial CLAUDE.md pass) already has a Tokio-based async prototype in `src/io/`, `src/network.rs`, etc. That predates this roadmap and is experimental — it does not reflect the v1 synchronous direction and can be discarded/rewritten as needed.

See **[TASKS.md](TASKS.md)** for the v1 work broken into independently-pickupable tasks (milestones M1–M6).

## Inspiration

Based on **QuickFIX/J** (pure Java FIX implementation). Local checkout for reference: `/Users/gauravtatke/quickfixj`. Familiarity comes from testing applications built on it (as a software tester), not from having built with it directly — so QuickFIX/J is a reference for protocol behavior and structure, not a spec to copy blindly. Deviate from its architecture where the Rust tech stack calls for it.

### Understanding of QuickFIX/J's structure (to validate/adapt, not assume correct)
1. A base layer of classes/methods handling generic message send/receive (transport-agnostic protocol mechanics: sessions, sequence numbers, message store, etc.).
2. On top of that, protocol-version-specific generated message classes giving strict type checking (e.g. a `NewOrderSingle` type with typed field accessors, rather than raw tag/value lookups).
3. fix-rs should aim for a similar two-layer shape: a generic engine layer, plus generated/typed message types per FIX version.

## Current project state

- `data_dictionary.rs` is the only part with real, tested functionality (parses the FIX XML spec into runtime lookup tables).
- Everything else (`message.rs`, `session/`, `network.rs`, `io/`, `application.rs`, the build-time codegen in `build/`) is experimentation, not a working implementation.
- Nothing is precious: any of it — including `data_dictionary.rs` — can be thrown away and rebuilt if the architecture calls for it. Don't let existing code anchor design decisions for v1.
