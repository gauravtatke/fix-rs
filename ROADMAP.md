# ROADMAP

This is the working design/goal doc for fix-rs. It's expected to change as the project evolves — treat it as a living
document, not a fixed spec.

## Goals

1. **Build a good, robust, well-engineered FIX protocol implementation in Rust** — usable, functionally correct,
   maintainable. This is the primary driver. **Design and technology decisions are made on product merit** (API
   usability, correctness/safety, robustness, maintainability, performance, fit with the existing engine) — nothing
   else. Two clarifications on what this means: (a) where a pattern is **idiomatic**, that counts as *evidence of*
   maintainability — idiom feeds the criteria, it just isn't the tiebreaker; (b) "well-engineered" includes **simplicity
   proportional to the need** (YAGNI) — the best design for the current scope, not the most general one possible.

2. **Learn Rust and become proficient in it.** Current level: between beginner and intermediate. This is a real goal but
   an *output* of the work, not an input to design decisions:
   - **Learning never selects a design.** We pick the best design for the product; whatever Rust it requires is the
     learning. We don't choose an option because it teaches a concept, or avoid one because it's already known.
   - **Whatever we build, we build idiomatically.** Idiomatic Rust still matters — but at the *implementation* level,
     because it *is* good engineering (clear, maintainable). It is not a decision point when choosing a design pattern
     at the broader level.
   - **Exceptions exist**; we decide case-by-case when one comes up (e.g. a deliberate, scoped learning detour).

## Scope

The work is organized as three **development phases** (v1 → v1.1 → v1.2) that culminate in the first public
**release, v1.0**. So "v1/v1.1/v1.2" are internal milestones toward shipping, not separate releases; **v1.0 is cut once
v1.2 is complete** (v1.0 = v1 + v1.1 + v1.2). Everything here is FIX **4.3** only and **synchronous** by design (FIX 4.4
and async/Tokio are post-1.0 — see v2); config is `toml` + `serde::Deserialize` (QuickFIX/J's one `[Default]` + N
per-session override merge).

### v1 — engine + happy path (DONE)

- Session happy path end-to-end between an initiator and an acceptor: Logon, Heartbeat, TestRequest, Logout.
- `MarketDataRequest` (35=V, subscribe) → `MarketDataSnapshotFullRefresh` (35=W, response).
- **`NewOrderSingle` (35=D) placed, accepted and rejected via the `ExecutionReport` (35=8) flow** (moved in from the old
  v1.1 scope — it was delivered live in M6).
- Also landed in this phase beyond the original "done" bar: automatic session-level error responses (Reject/Logout/drop,
  M7) and initiator support with auto-reconnect (M8).
- **Not** in v1: resend request / gap fill / sequence-number persistence (that's v1.2).

### v1.1 — typed message codegen

- Generate per-message-type structs (`Logon`, `NewOrderSingle`, `MarketDataRequest`, `MarketDataSnapshotFullRefresh`, …)
  with typed field accessors, replacing today's raw `Message`/tag-lookup API (milestone **M3**). This is an
  architecture/refactor phase — it changes how messages are *typed*, it does not add protocol functionality.

### v1.2 — protocol robustness

- Message store + **ResendRequest / gap fill / sequence-number persistence** — closes the too-high-seq path that is
  currently stubbed (just logs a warning) across M4/M5/M7. Plus additional application message types as needed.

### Release v1.0

- Cut when **v1.2 is complete**. v1.0 is the first public release: a **typed, robust, synchronous FIX 4.3 engine**
  (acceptor + initiator) with the happy path, order/execution flow, automatic error responses, and full
  resend/sequence robustness.

### v2 (post-1.0, future)

- Introduce **Tokio** and the async stack as an enhancement on top of the working synchronous v1.0.
- **Structured logging infrastructure**: replace direct `log::info!` calls with a proper `Log` trait
  (incoming/outgoing/event categories, per-session context) and an async logging backend. On low-latency paths,
  synchronous logging can stall the message-processing thread — the logger should hand off formatted output to a
  dedicated writer thread (or ring buffer) so the hot path never blocks on I/O. Design should support pluggable backends
  (screen, file, network) similar to QFJ's `LogFactory`/`Log` abstraction.
- FIX **4.4** (prove the dictionary-driven design ports to a second XML dictionary).

The work is broken into independently-pickupable tasks (milestones M1–M8) in `TASKS.md` (kept on the dev branch).

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

**The v1 phase is complete** — M1, M2, M4, M5, and M6 are all done, and the two post-v1 milestones that fold into the v1
phase are done too: M7 (automatic session-level error responses, except one deferred sub-task) and M8 (initiator support
+ auto-reconnect, live-verified). M3 (typed message codegen) is **not** part of the v1 phase — it is now the **v1.1**
phase. Next toward the **v1.0 release** is v1.1 (M3), then v1.2 (protocol robustness). Per-task detail lives in
`TASKS.md` (kept on the dev branch).

- **M1 (message/dictionary layer hardening) is done.** Tasks 1.1–1.6 complete. `Message`/`FieldMap` parsing is fully
  dictionary-driven with all validation going through `SessionRejectError`. 1.6 (SOH-in-Data-field handling) landed via
  a positional byte-cursor tokenizer; only the `NonDataFieldIncludeSOHChar` (373=17) sub-case is descoped (QFJ doesn't
  emit it either).
- **M2 (config rewrite) is done.** `toml`+`serde` based config with default/session merge. Hand-rolled parser deleted.
  14 config tests.
- **M3 (typed message codegen) is not started — it is the v1.1 phase.** Would rework build-time codegen to emit
  per-message-type structs (`Logon`, `NewOrderSingle`, …) with typed accessors, matching the roadmap's two-layer goal.
  The v1 phase shipped on the raw `Message` API instead; M3 is the next phase toward the v1.0 release, not a blocker for
  what already works.
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

The v1 phase is done; the remaining phases lead to the **v1.0 release**, in order:

- **v1.1 — M3 typed message codegen** (next up). Biggest architectural piece and the roadmap's two-layer shape; heavy on
  Rust-learning (build-time codegen, trait design). Still a sketch in TASKS.md — needs breaking into concrete tasks
  first.
- **v1.2 — protocol robustness.** Message store + ResendRequest / gap fill / sequence-number persistence. Biggest
  *functional* gap: the too-high-seq path is stubbed across M4/M5/M7 (currently just logs a warning). **v1.0 releases
  when this lands.**
- **7.6 (b)** — scripted TCP simulator for true end-to-end coverage of the acceptor/IO seam (deferred from M7;
  independent of the phase order, can slot in whenever).

**M8 (initiator support) is done.** fix-rs dials out and completes Logon + heartbeat + clean reconnect against the QFJ
executor (verified live 2026-09-20). 8.5 (auto-reconnect loop) landed: `IoInitiator::start()` retries forever on a
`reconnect_interval` (config key, default 30s), gated by session schedule, with a stop flag for graceful shutdown — a
failed/dropped dial now re-dials instead of panicking the thread.
