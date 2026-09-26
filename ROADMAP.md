# ROADMAP

This is the working design/goal doc for fix-rs. It's expected to change as the project evolves — treat it as a living
document, not a fixed spec.

## Goals

1. **Learn Rust and become proficient in it.** Current level: between beginner and intermediate. This is a primary goal,
   not a side effect — code choices should favor idiomatic Rust and understanding over speed of delivery.
2. **Build a fully functional FIX protocol implementation in Rust.**

## Scope

### V1

- FIX **4.3** only to start. FIX 4.4 is deferred until the architecture is proven on one version — the dictionary-driven
  design should port to a second XML dictionary with minimal changes, but that's a later milestone, not v1 core.
- **Synchronous** network architecture, deliberately, for learning purposes (understanding blocking IO, threads, etc.
  before reaching for async).
- Config loaded via `toml` + `serde::Deserialize` (replacing the current hand-rolled INI-style parser), modeling
  QuickFIX/J's "one `[Default]` block + N per-session override blocks" merge pattern.
- "Done" bar for v1 is a **happy path**: Logon, Heartbeat, TestRequest, and Logout working end-to-end between an
  initiator and an acceptor, plus two typed application-level messages exchanged end-to-end — `MarketDataRequest` (35=V,
  subscribe) and `MarketDataSnapshotFullRefresh` (35=W, response). No resend request / gap fill / sequence number
  persistence in v1.

### V1.1 and beyond

- **v1.1**: `NewOrderSingle` placed, accepted and rejected (`ExecutionReport` flow).
- **v1.2+**: other message types, plus protocol robustness — resend request, gap fill, sequence number persistence.

### V2 (future)

- Introduce **Tokio** and the async stack as a feature enhancement on top of a working synchronous v1.
- **Structured logging infrastructure**: replace direct `log::info!` calls with a proper `Log` trait
  (incoming/outgoing/event categories, per-session context) and an async logging backend. On low-latency paths,
  synchronous logging can stall the message-processing thread — the logger should hand off formatted output to a
  dedicated writer thread (or ring buffer) so the hot path never blocks on I/O. Design should support pluggable backends
  (screen, file, network) similar to QFJ's `LogFactory`/`Log` abstraction.

Note: the current codebase (as of the initial CLAUDE.md pass) already has a Tokio-based async prototype in `src/io/`,
`src/network.rs`, etc. That predates this roadmap and is experimental — it does not reflect the v1 synchronous direction
and can be discarded/rewritten as needed.

See **[TASKS.md](TASKS.md)** for the v1 work broken into independently-pickupable tasks (milestones M1–M6).

## Inspiration

Based on **QuickFIX/J** (pure Java FIX implementation). Local checkout for reference: `/Users/gauravtatke/quickfixj`.
Familiarity comes from testing applications built on it (as a software tester), not from having built with it directly —
so QuickFIX/J is a reference for protocol behavior and structure, not a spec to copy blindly. Deviate from its
architecture where the Rust tech stack calls for it.

### Understanding of QuickFIX/J's structure (to validate/adapt, not assume correct)

1. A base layer of classes/methods handling generic message send/receive (transport-agnostic protocol mechanics:
   sessions, sequence numbers, message store, etc.).
2. On top of that, protocol-version-specific generated message classes giving strict type checking (e.g. a
   `NewOrderSingle` type with typed field accessors, rather than raw tag/value lookups).
3. fix-rs should aim for a similar two-layer shape: a generic engine layer, plus generated/typed message types per FIX
   version.

## Current project state

**v1 is complete** — M1, M2, M4, M5, and M6 are all done. M3 (typed message codegen) was deferred by design; v1 uses the
raw `Message` API instead. M7 (automatic session-level error responses, post-v1) is done except one deferred sub-task.
See **[TASKS.md](TASKS.md)** for the per-task detail.

- **M1 (message/dictionary layer hardening) is done.** Tasks 1.1–1.6 complete. `Message`/`FieldMap` parsing is fully
  dictionary-driven with all validation going through `SessionRejectError`. 1.6 (SOH-in-Data-field handling) landed via
  a positional byte-cursor tokenizer; only the `NonDataFieldIncludeSOHChar` (373=17) sub-case is descoped (QFJ doesn't
  emit it either).
- **M2 (config rewrite) is done.** `toml`+`serde` based config with default/session merge. Hand-rolled parser deleted.
  14 config tests.
- **M3 (typed message codegen) is not started — deferred by design.** Would rework build-time codegen to emit
  per-message-type structs (`Logon`, `NewOrderSingle`, …) with typed accessors, matching the roadmap's two-layer goal.
  v1 shipped on the raw `Message` API instead, so this is a post-v1 architectural improvement, not a blocker.
- **M4 (session state machine) is done.** `SessionState`, `SessionId`, `Application` trait, `Responder` trait,
  `Session::verify`, admin message builders (`generate_logon`/`logout`/`heartbeat`/`test_request`), inbound dispatch
  (`next_message`), and timer logic (`next_tick`). All testable without real sockets via `MockResponder`.
- **M5 (sync networking) is done.** Tokio prototype replaced with `std::net`/`std::thread`. `FixMessageReader`,
  `TcpResponder`, `SessionMap`, `IoAcceptor` (binds `TcpListener`, spawns reader thread per connection), timer thread,
  disconnect cleanup, and reset flags (`reset_on_logon`/`reset_on_logout`/`reset_on_disconnect`). Integration-tested
  against QFJ Banzai — Logon handshake and heartbeat exchange run cleanly. Acceptor only; initiator support deferred.
- **M6 (close out v1) is done.** Application-level messages flow end-to-end against a real QFJ Banzai initiator
  (`NewOrderSingle 35=D` → two `ExecutionReport 35=8` acks, verified in Banzai's UI). Outbound sends go via the
  `Vec<Message>` return from `on_app_msg_received` (no ownership cycle) plus `SessionMap::send` for
  unsolicited/streaming sends. `V`→`W` (MarketData) is unit-tested but not exercised live (Banzai has no MarketData
  client).
- **M7 (automatic session-level error responses, post-v1) is nearly done.** The engine now responds on the wire to
  inbound validation/session errors — Reject (35=3) for recoverable field errors, Logout (35=5) for fatal ones, silent
  drop for garbled messages, BusinessMessageReject (35=j) as the app-error fallback — instead of silently dropping the
  connection. 7.1–7.5 and 7.6 (a) (in-process raw-input harness) are done; **7.6 (b)** (a scripted end-to-end TCP
  simulator) is deferred. Too-high-seq (ResendRequest) and full resend remain deferred to v1.2+.

## What's open next

Everything above is post-v1. In rough order of value:

- **M3 — typed message codegen.** Biggest architectural piece and the one v1 milestone skipped by design; heavy on
  Rust-learning (build-time codegen, trait design). Aligns with roadmap goal #2's two-layer shape.
- **v1.2 protocol robustness** — message store + ResendRequest / gap fill / sequence-number persistence. Biggest
  *functional* gap: the too-high-seq path is stubbed across M4/M5/M7 (currently just logs a warning).
- **7.6 (b)** — scripted TCP simulator for true end-to-end coverage of the acceptor/IO seam (deferred).

**M8 (initiator support) is done.** fix-rs dials out and completes Logon + heartbeat + clean reconnect against the QFJ
executor (verified live 2026-09-20). 8.5 (auto-reconnect loop) landed: `IoInitiator::start()` retries forever on a
`reconnect_interval` (config key, default 30s), gated by session schedule, with a stop flag for graceful shutdown — a
failed/dropped dial now re-dials instead of panicking the thread.
