# v1 Task Breakdown

Companion to `ROADMAP.md` (the why/scope) and the approved M1 plan captured in `session_context/`. This doc exists to
break v1 into pieces small enough to pick up one at a time, in separate sittings, without needing to hold the whole
design in your head.

**How to use this**: work top to bottom within a milestone (tasks are ordered by dependency). Check a box when a task's
own tests pass. Each task lists exactly which file (s) it touches and how to know it's done. When you finish a task,
it's a natural commit boundary.

Milestones M1 and M2 are broken down in real detail (they're next up). M3–M6 are sketched at a coarser grain — detailed
enough to start planning from, but expect to re-break them into smaller tasks (same way M1 is broken down here) once
M1/M2 are done and we know what the message/config layers actually look like.

---

## M1 — Harden the message/dictionary layer

No networking, no sessions — just making `Message`/`FieldMap` parsing and serialization correct and dictionary-driven.
Touches `src/message.rs` and `src/quickfix_errors.rs` only.

- [x] **1.1 — Fix serialization ordering.**
  Change `FieldMap.fields` from `HashMap<Tag, StringField>` to `indexmap::IndexMap<Tag, StringField>`
  (`src/message.rs`). Add round-trip tests (parse a fixture, `.to_string()` it, assert it matches the original). *Done —
  4 round-trip tests passing (`msg_test_round_trip_*`).*

- [x] **1.2 — Expose `SessionRejectReason` for testing.**
  In `src/quickfix_errors.rs`: make `SessionRejectReason` `pub`, derive `Debug, PartialEq, Eq, Clone, Copy` on it, add
  `SessionRejectError::kind(&self) -> SessionRejectReason`. Pure enabler, no behavior change — should compile with zero
  other edits. Done when: `cargo build` is clean and you can write
  `assert_matches!(err.kind(), SessionRejectReason::SomeVariant)` from a test.

- [x] **1.3 — Wire tag-validity + required-field checks.**
  *Depends on 1.2.* Add a shared validation helper called from `parse_header`/`parse_body`/`parse_trailer` (replacing
  bare `set_field` calls) that checks `dd.is_msg_field(msg_type, tag)`, distinguishing "unknown to the dictionary"
  (`undefined_tag_err`) from "known tag, wrong message type" (`tag_not_defined_for_msg`, already used in `parse_group` —
  extend its use here). Add a post-parse check in `from_vec` diffing `dd.get_msg_required_field(msg_type)` (+
  `"header"`/`"trailer"`) against what's present → `required_tag_missing_err` on gaps. Watch for: the existing
  `msg_test_with_body_group`/`msg_test_with_group_and_subgroups` fixtures may not be fully FIX-spec-complete for
  required fields — if turning this on breaks them, add the missing required fields to those fixture strings rather than
  weakening the check. *(Done — both fixtures were missing real required fields; fixed rather than weakening the
  check.)*
  Done when: un-stub `msg_test_trailer_with_more_fields` (give it a real body) + new tests for missing-required-tag,
  undefined-tag, wrong-msgtype, each asserting the specific `SessionRejectReason` via `kind()`. *(Done — see
  `session_context/2026-07-26.md` for the design reasoning behind where each check ended up living, and a real gap found
  in the process: the group-instance-level required-field check needed `rg_dd`, not the `dd` parameter, since `dd` is
  silently inherited unchanged across recursive nested-group calls.)*

- [x] **1.4 — Wire enum-value + type-format checks.**
  *Depends on 1.3 (same validation helper).* If `dd.get_field_values(tag)` is `Some(set)`, check membership →
  `value_out_of_range_err`. Validate the raw string against `dd.get_field_type(tag)`'s category (numeric types must
  parse as such, `Boolean` must be `Y`/`N`, etc.) → `incorrect_data_format_err`. Done when: new tests cover an
  out-of-domain enum value and a malformed numeric field, each asserting the right `SessionRejectReason`.

- [x] **1.5 — Wire incoming checksum/body-length verification.**
  *Independent of 1.3/1.4 — can be done before or after them.* In `from_str`, recompute expected body length/checksum
  from the raw string and compare to the incoming `9=`/`10=` fields (check body length first, it's cheaper) →
  `invalid_body_len_err`/`invalid_checksum`. Done when: un-stub `msg_test_invalid_checksum` and
  `msg_test_invalid_body_length` with deliberately corrupted fixtures (flip a digit in `9=` or `10=`).

- [ ] **1.6 — (Stretch) SOH-in-Data-field handling.**
  *Independent — skip freely, none of v1's required messages need this.* `Message::from_str`'s tokenizer naively splits
  on SOH, which breaks for `DATA`-typed fields whose raw bytes legitimately contain SOH. If you pick this up: first
  verify the Length/Data tag-pairing convention against real pairs in `resources/FIX43.xml` — it is *not* reliably "tag
  minus 1" (e.g. header's `90/91 SecureDataLen/SecureData` breaks that pattern) — before writing tokenizer logic. Done
  when: `msg_test_soh_in_data_field`/`msg_test_soh_in_non_data_field` pass, or are left `#[ignore]`d with a one-line
  reason if descoped.

- [x] **1.7 — Housekeeping.**
  Update `ROADMAP.md`'s milestone list to reflect however M1 actually shook out (note any deviations from this task
  list), and log the wrap-up in `session_context/`.

**M1 exit criteria**: `cargo test` green, all of `src/message.rs`'s error paths go through `SessionRejectError` (no
stray `Result<T, String>` for validation failures), message layer rejects malformed/non-compliant input instead of
silently accepting it.

---

## M2 — Config rewrite (toml + serde)

Replaces `src/session/session_settings.rs`'s hand-rolled parser. Independent of M1 — can be done in parallel or first,
your call.

- [x] **2.1 — Add `toml` dependency and define the target shape.**
  Add `toml = "0.5"` (or current stable) to `Cargo.toml`. Decide the final typed struct (s) you want out the other end
  (likely close to today's `Properties`/per-`SessionId` settings map). No merge logic yet — just get a
  `#[derive(Deserialize)]` struct that can parse a *single* `[Default]`-style block in isolation, with a unit test
  proving it.

- [x] **2.2 — Implement the default + per-session merge.**
  *Depends on 2.1.* Recommended pattern (avoids custom `Deserialize` impls / `#[serde(flatten)]` entirely, which is
  where the last attempt got stuck): parse the whole file into `HashMap<String, toml::Value>` per `[Default]`/
  `[Session]` block first, then for each session block start from a clone of the default map and `.extend()` the
  session-specific keys on top (session wins on collision) — merge at the `toml::Value` level, *then* run the merged map
  through `try_into::<YourSettingsStruct>()`. Done when: a unit test with one `[Default]` + two `[Session]` blocks
  (mirroring the existing `session_sample_config_test` fixture in `session_settings.rs`) proves per-session overrides
  win and un-overridden keys fall back to default.

- [x] **2.3 — Swap call sites over to the new loader.**
  *Depends on 2.2.* Old `Properties` struct, all its methods, and all call sites (`session_and_state.rs`, `network.rs`,
  `session_schedule.rs`, `session_id.rs`) deleted outright — prototype code that will be rewritten in M4/M5, so
  mechanical migration was unnecessary. Old `ConfigErr` enum also removed.

- [x] **2.4 — Port/replace existing config tests + validation.**
  *Depends on 2.3.* Validation is fully covered by `TryFrom<FixProperties> for SessionConfig`: required fields,
  begin_string whitelist (FIX42/43/44), connection-type-specific port/host checks, start/end day pairing. 14 tests cover
  the new `SessionProperties`/`SessionConfig` pipeline. Old `Properties`-based tests deleted with the code they tested.

**M2 exit criteria**: `src/FixCfg.toml` loads through real `toml`+`serde`, default/session merge is unit-tested,
`session_settings.rs`'s hand-rolled parser is deleted. **All met.**

---

## M3 — Typed message codegen (sketch — will be re-broken-down later)

Reworks `build/main.rs` + `build/code_generator.rs` + `build/templates.rs` to emit real per-message-type structs (e.g.
`Logon`, `Heartbeat`, `MarketDataRequest`, `MarketDataSnapshotFullRefresh`) with typed field accessors, replacing the
currently-unused per-field struct generation. Likely sub-tasks once we get here: (a) design the generated struct shape,
(b) generate for the admin messages first (Logon/Heartbeat/TestRequest/Logout) since M4 needs them, (c) generate for
MarketDataRequest/MarketDataSnapshotFullRefresh, (d) wire generated types into `Message`/`FieldMap` so they're not a
parallel disconnected API.

## M4 — Session state machine

Session-level protocol logic: logon/heartbeat/test-request/logout sequencing, message dispatch, and the Application
trait for handing messages to/from user code. Reference: `session_context/qfj-session-message-flow.md`.

- [x] **4.1 — `SessionState` struct.**
  Pure-data struct in `src/session/session_and_state.rs`: logon/logout/reset booleans (`logon_sent`, `logon_received`,
  `logout_sent`, `logout_received`, `reset_sent`, `reset_received`, `is_initiator`), heartbeat interval (`u32`), timing
  fields (`last_sent_time`, `last_received_time` as `Instant` or equivalent), `test_request_counter` (`u32`),
  sender/target sequence numbers (`next_sender_msg_seq_num`, `next_target_msg_seq_num` as `u32`). No I/O anywhere in
  this struct. Methods: `is_heartbeat_needed()`, `is_test_request_needed()`, `is_timed_out()`, `is_logon_timed_out()`,
  `is_logout_timed_out()`, `incr_next_sender_msg_seq_num()`, `incr_next_target_msg_seq_num()`, `reset()`. Done when:
  unit tests cover each timing predicate with controlled timestamps (inject a "now" rather than calling `Instant::now`
  directly), and `reset()` clears everything back to initial state.

- [x] **4.2 — `SessionId` as a proper struct.**
  `SessionId` (formerly `SessionId`) defined in `src/session/session_settings.rs` with all identity fields, a
  precomputed `id` string, and a precomputed `reverse_id` string. Design decision: `reverse_id` is stored as a `String`
  field computed once at construction (not a method returning a new `SessionId`) — since the reverse is deterministic
  and needed on every inbound message, computing it once avoids repeated allocation. `SessionConfig::to_session_id()`
  produces a `SessionId`, and `SessionProperties` uses `HashMap<SessionId, SessionConfig>`. Manual `Hash`/`PartialEq`
  (on `id`
  only) + `Borrow<str>` enables string-key lookups on the map. Old `SessionId` renamed to `SessionId` (prototype code,
  will be removed). 2 tests verify reverse_id correctness (full fields + minimal).

- [x] **4.3 — `Application` trait + `DefaultApplication`.**
  Rewrite `src/application.rs`. The trait has 7 methods matching QFJ's interface:
    - `on_create(&mut self, session_id: &SessionId)` — session constructed
    - `on_logon(&mut self, session_id: &SessionId)` — logon complete
    - `on_logout(&mut self, session_id: &SessionId)` — disconnected
    - `to_admin(&mut self, message: &mut Message, session_id: &SessionId)` — outbound admin notification (no rejection)
    - `from_admin(&mut self, message: &Message, session_id: &SessionId) -> Result<(), RejectLogon>` — inbound admin (can
      reject logon)
    - `to_app(&mut self, message: &mut Message, session_id: &SessionId) -> Result<(), DoNotSend>` — outbound app (can
      cancel send)
    - `from_app(&mut self, message: &Message, session_id: &SessionId) -> Result<(), FixError>` — inbound app delivery
      (can throw UnsupportedMessageType etc.)

  `DefaultApplication` is a no-op impl (all methods return `Ok(())`/do nothing). Error types `RejectLogon` and
  `DoNotSend` added to `quickfix_errors.rs`. Done when: `DefaultApplication` compiles and a test calls each callback
  without panicking.

- [x] **4.4 — `Responder` trait.**
  Wire abstraction in `src/session/` (or `src/network.rs`): `fn send(&self, message: &str) -> bool` and
  `fn disconnect(&self)`. M4 provides a `MockResponder` (captures sent messages in a `Vec<String>` behind a `RefCell` or
  `Mutex`) for testing. Real TCP responder comes in M5. Done when: `MockResponder` compiles and a test verifies `send()`
  captures the message string.

- [x] **4.5 — Session struct + `verify()`.**
  Rewrite `Session` in `src/session/session_and_state.rs`. Session owns: `SessionId`, `SessionState`,
  `Arc<DataDictionary>`, `Box<dyn Application>`, `Option<Box<dyn Responder>>`, config fields (heartbeat interval,
  reset-on-logon/logout/disconnect flags). Delete the old prototype fields (`msg_q`, broadcast responder, etc.).
  `verify(&mut self, message: &Message) -> Result<bool>`:
    1. Update `last_received_time`, clear test-request counter
    2. Check `valid_logon_state(msg_type)` — is this message type legal given current logon/logout flags?
    3. Check CompID match (inbound SenderCompID == session's TargetCompID and vice versa)
    4. Check sequence number (too high / too low) — for v1, too-high just logs a warning (no ResendRequest), too-low
       rejects
    5. If all pass → `from_callback(msg_type, message)` → dispatches to `application.from_admin()` or
       `application.from_app()` based on admin/app classification Done when: unit tests cover verify passing, CompID
       mismatch, bad logon state, and the from_callback dispatch (using a test Application impl that records which
       callback was called).

- [x] **4.6 — Admin message builders.**
  Methods on Session: `generate_logon()`, `generate_logout(reason: Option<&str>)`,
  `generate_heartbeat(test_req_id: Option<&str>)`, `generate_test_request(id: &str)`. Each: creates a `Message`, sets
  the MsgType, calls `initialize_header(&mut header)` (stamps BeginString, SenderCompID, TargetCompID, MsgSeqNum,
  SendingTime from SessionId + SessionState), calls `application.to_admin()`, then `send_raw()` which serializes and
  pushes through the Responder.
  `initialize_header` is its own method since both admin builders and the outbound app path use it. Done when: unit
  tests verify that `generate_heartbeat` produces a message with correct header fields and the MockResponder captures
  the serialized output.

- [x] **4.7 — Inbound dispatch (`next_message`).**
  `Session::next_message(&mut self, message: Message)` — the MsgType switch:
    - `A` (Logon) → `next_logon`: validate session enabled + in session time, check ResetSeqNumFlag, verify, set
      logon-received, if acceptor generate logon response, call `application.on_logon()`
    - `0` (Heartbeat) → `next_heartbeat`: verify, incr target seq num
    - `1` (TestRequest) → `next_test_request`: verify, generate heartbeat response (echo TestReqID), incr target seq num
    - `5` (Logout) → `next_logout`: verify (skip seq num checks), set logout-received, if we didn't initiate → generate
      logout response, disconnect
    - default → verify (which calls `application.from_app()`), incr target seq num v1 skips: ResendRequest (`2`),
      SequenceReset (`4`), Reject (`3`) — log and increment seq num only. Done when: unit tests cover: receiving a Logon
      triggers logon-received + logon response, receiving a TestRequest triggers heartbeat with echoed TestReqID,
      receiving a Logout triggers logout response + disconnect, receiving an application message triggers
      `on_app_msg_received`.

- [x] **4.8 — Timer logic (`next_tick`).**
  `Session::next_tick(&mut self)` — called periodically (by the network layer in M5, but testable in isolation now):
    1. If not connected (no responder) → return
    2. If logon not received:
        - If initiator and logon not yet sent → `generate_logon()`
        - If logon sent but timed out → disconnect
    3. If logout timed out → disconnect
    4. If heartbeat interval > 0 and logged on:
        - `state.is_test_request_needed()` → `generate_test_request("TEST")`
        - `state.is_heartbeat_needed()` → `generate_heartbeat(None)`
        - `state.is_timed_out()` → disconnect Integrates with `SessionSchedule::is_session_time()` for session-window
          checking. Done when: unit tests verify: initiator generates logon on first tick, heartbeat fires after
          interval, test request fires after 1.5x interval with no response, disconnect fires after 2x interval.

**M4 exit criteria**: Session processes a Logon→Heartbeat→Logout sequence correctly using MockResponder, Application
callbacks fire at the right points, timer logic generates heartbeats and test requests on schedule. All testable without
real sockets.

## M5 — Sync networking

Replaces the disposable Tokio prototype in `network.rs`/`io/` with `std::net::TcpStream` + `std::thread`.

- [x] **5.1 — Delete Tokio prototype code.** Old async acceptor, broadcast channels, and task-spawning code removed.
- [x] **5.2 — `FixMessageReader`.** `src/io/fix_message_reader.rs`: `FixMessageReader<R: Read>` wraps `BufReader`, reads
  SOH-delimited FIX bytes until checksum tag `10=XXX\x01`. 4 tests.
- [x] **5.3 — `TcpResponder`.** `src/io/tcp_responder.rs`: `Responder` impl wrapping `Mutex<TcpStream>`. 5 tests.
- [x] **5.4 — `SessionMap`.** `src/network.rs`: `Arc<HashMap<SessionId, Arc<Mutex<Session>>>>`, built via
  `FromIterator`, immutable after construction. 6 tests.
- [x] **5.5 — `IoAcceptor`.** `src/io/acceptor.rs`: binds `TcpListener`, spawns reader thread per connection, dispatches
  via `reverse_session_id` lookup. Supporting changes: `Session::set_responder()`,
  `session_id_from_raw()`/`reverse_session_id_from_raw()` in `message.rs`.
- [x] **5.6 — Timer thread.** `src/network.rs`: `start_timer()` spawns background thread, sleep 1s → tick all sessions.
- [x] **5.7 — Wire `main.rs` + integration test with QFJ Banzai.**
    - `SessionConfig::to_session()` constructs Session from config.
    - `connection_type()` and `socket_accept_port()` getters exposed on `SessionConfig`.
    - `main.rs` wired: parse config → build `SessionMap` → start timer → spawn one `IoAcceptor` thread per bind
      address → join all.
    - `generate_logon()` fixed to include `EncryptMethod=0` (tag 98) — Banzai rejected Logon without it.
    - `Display` impl added for `SessionId`.
    - `SessionMap::len()` added.
    - Integration-tested against QFJ Banzai (initiator): Logon handshake succeeds, heartbeat exchange runs cleanly with
      sequence numbers in lockstep.

- [x] **5.8 — Disconnect cleanup.**
  When `handle_connection`'s read loop breaks (EOF or error), lock the session and clean up:
  clear the responder (`self.responder = None`), reset logon/logout flags (`logon_sent`,
  `logon_received`, `logout_sent`, `logout_received` → false), and call `app.on_logout()`. This stops the timer from
  sending heartbeats into a dead socket and leaves the session in a state where a new connection can re-logon cleanly.
    - `Session::disconnect()` method centralizes teardown: calls `responder.disconnect()`, sets responder to `None`,
      resets all logon/logout flags, calls `app.on_logout()`.
    - `next_logout` and all `next_tick` disconnect paths now route through `disconnect()`.
    - `handle_connection` in `acceptor.rs` calls `session.disconnect()` after the read loop breaks.
    - Test infrastructure refactored: `MockResponder` in `session_tests` now uses a shared
      `MockState` (`Arc<Mutex>`) pattern — state survives after `Session` drops the responder on disconnect, eliminating
      the unsafe `get_mock_responder` pointer cast.
    - 6 new disconnect tests added, all existing tests updated. 166 tests passing.

- [x] **5.9 — Apply reset flags.**
  *Depends on 5.8.* Act on the `reset_on_disconnect`, `reset_on_logon`, and `reset_on_logout`
  config flags that are already stored in `Session` but never used:
    - `reset_on_disconnect`: in the disconnect cleanup path (5.8), if true → `state.reset()`
      (zeroes seq nums back to 1).
    - `reset_on_logon`: in `next_logon`, before processing the inbound Logon → `state.reset()`. For acceptors this fires
      on receiving a Logon; for initiators it would fire before sending.
    - `reset_on_logout`: in `next_logout`, after processing the inbound Logout → `state.reset()`. These are operational
      choices agreed between counterparties (not part of the FIX spec itself). When all three are false (default), seq
      nums persist across reconnections — which requires ResendRequest support (deferred to v1.2+). For v1, setting
      `reset_on_logon = true` in
      `FixCfg.toml` is the practical workaround. Done when: unit tests verify each flag triggers
      `state.reset()` at the right point, and Banzai reconnect with `reset_on_logon = true` starts from seq 1 on both
      sides.

- [x] **5.10 — Cleanup.** Tokio dependency already removed from `Cargo.toml` and `src/io/mod.rs`
  (cleaned up during 5.1). No remaining references in the codebase.

**M5 exit criteria**: Acceptor binds, accepts TCP connections, completes Logon handshake, exchanges heartbeats with a
real QFJ initiator. Reconnections handled cleanly (disconnect detected, session state reset per config flags). Initiator
support deferred to a later milestone.

## M6 — Outbound app messages + close out v1

Wire application-level outbound messages end-to-end. M3 (typed message codegen) is deferred — v1 uses the raw `Message`
API.

**Design pivot (2026-09-06):** two approaches were explored for breaking the Session↔Application bidirectional problem:
- **Option A — SessionEntry:** pull `Box<dyn Application>` out of Session into a sibling `SessionEntry { session, app }`
  and use split borrows to touch both from one lock.
- **Option B — app-in-session (CHOSEN):** keep `app: Box<dyn Application>` as a field of `Session`. This is *not* a cycle,
  because `on_app_msg_received` returns `Vec<Message>` — the app returns the messages it wants sent and the engine sends
  them, so there is no `app → session` edge. No `SessionEntry`, no split-borrow ceremony.

Option B won on maintainer ergonomics (simpler to reason about and extend; app-developer experience is identical either
way; performance is a wash). Option A was built first and is preserved on branch `explore/session-entry-outbound-msg`
for reference. See `session_context/2026-09-05-outbound-design.md` (Addendum + "Decision (2026-09-06)") for the full
rationale.

- [x] **6.1 — Rename Application trait methods.**
  Rename for clarity (QFJ's `from`/`to` convention is non-obvious):
    - `on_admin_msg_sending` → `on_admin_msg_sending`
    - `on_admin_msg_received` → `on_admin_msg_received`
    - `on_app_msg_sending` → `on_app_msg_sending`
    - `on_app_msg_received` → `on_app_msg_received`
      Mechanical find-and-replace across: `src/application.rs` (trait + `DefaultApplication`),
      `src/session/mod.rs` (all call sites + `TestApplication` in tests). Done when: `cargo test` green, all 172 tests
      pass, no `on_app_msg_received`/`on_app_msg_sending`/`on_admin_msg_received`/
      `on_admin_msg_sending` method names remain.

- [x] **6.2 — Change `on_app_msg_received` return type.**
  *Depends on 6.1.* Change return from `Result<(), AppError>` to
  `Result<Vec<Message>, AppError>`. Update `DefaultApplication` (return `Ok(vec![])`),
  `TestApplication` in tests (same). In `dispatch_to_app`, collect the returned `Vec` but don't send yet — just drop it.
  This is a seam for 6.4. Done when: `cargo test` green, return type updated everywhere.

- [x] **6.3 — (Superseded by the Option B pivot.) Extract Application into SessionEntry.**
  *Was done, then reverted.* The SessionEntry extraction (remove `app` field, thread `app: &mut dyn Application` through
  ~12 methods, `SessionMap<Arc<Mutex<SessionEntry>>>`, split-borrow at call sites) was implemented and is preserved on
  branch `explore/session-entry-outbound-msg`. We then chose **Option B (app-in-session)** instead, so on `dev` the app
  is back as a `Session` field and there is no `SessionEntry`. What actually replaced this task:
    - `Session` owns `app: Box<dyn Application>` again; methods use `self.app` (no `app` parameter).
    - `SessionMap` holds `Arc<Mutex<Session>>`; `SessionEntry` deleted; acceptor/timer lose the destructure.
    - `SessionConfig::to_session(app)` takes the app and stores it on the Session.
    - Tests use a shared `AppSpy` handle (the codebase's existing `MockState` pattern) since the app moves into Session.

- [x] **6.4 — Add `send_app_message` + wire outbound responses.**
  New method `Session::send_app_message(&mut self, msg: Message) -> Result<(), SendError>` (app is `self.app`, not a
  parameter): guards on responder-present + logged-on (mirrors QFJ `sendRaw`'s `isLoggedOn`), calls `initialize_header`,
  calls `self.app.on_app_msg_sending` (`DonotSend` = silent skip, **not** an error/disconnect), calls `send_raw`.
  `dispatch_to_app` drains the `Vec<Message>` from `on_app_msg_received` and sends each. Covered by
  `test_request_response_send_via_return_value` (V→W returned and captured) and `test_donotsend_skips_without_error_or_seqnum_bump`.
  *Follow-up:* strengthen the response-path test to assert stamped headers (BeginString/CompIDs/MsgSeqNum), per the
  original "done when".

- [x] **6.4b — (Bonus, not originally scoped.) External / streaming outbound send path.**
  Added while studying how QFJ streams market data (`session_context/qfj-outbound-market-data.md`): the push model for
  unsolicited sends (a market-data feed sending whenever data is available, not on a poll).
    - `SessionMap::send(&self, sid, msg) -> Result<(), SendError>` — the `Session.sendToTarget` analogue: registry
      lookup + per-session lock + `send_app_message`. Callable from any thread holding a `SessionMap` clone.
    - `SendError { SessionNotFound, NotLoggedOn }` in `quickfix_errors.rs`.
    - `Application::poll_outbound` (default empty) + timer wiring — a low-rate seam only; **not** the market-data path.
    - Tests: external-thread push (`test_session_map_send_pushes_from_external_thread`), `NotLoggedOn`/`SessionNotFound`
      guards, `poll_outbound` streaming.
    - Rule to remember: the per-session `Mutex` is not reentrant, so never call `SessionMap::send` for a session's own id
      from inside that session's callback (deadlock) — use the `Vec<Message>` return path there.

- [x] **6.5 — Sample application (`src/sample_app.rs`).**
  `SampleApp` implements `Application`. `on_app_msg_received` handles two inbound app types:
    - `V` (MarketDataRequest) → `W` (MarketDataSnapshotFullRefresh) echoing MDReqID (262) + a `NoMDEntries` (268)
      group with dummy bid/offer entries.
    - `D` (NewOrderSingle) → two ExecutionReports (35=8): a New ack (`150=0/39=0`, LeavesQty=OrderQty) followed by a
      full fill (`150=F/39=2`, LeavesQty=0, CumQty=qty, LastPx/LastQty), echoing ClOrdID/Side/Symbol.
    - all other types → `Ok(vec![])`.
  Inbound field reads use `?` + `AppError::FieldNotFound { tag }` (no panics on malformed peer messages). Wired into
  `main.rs` (replaces `DefaultApplication`). 7 unit tests. *Note:* the task originally scoped only V→W, but Banzai is an
  order-entry client (no MarketData UI), so `D`→`8` was added as the flow an actual QFJ counterparty can drive; V→W is
  kept and unit-tested for a future MarketData-capable simulator.

- [x] **6.6 — Integration test with QFJ Banzai.**
  End-to-end against a QFJ Banzai initiator (BANZAI→EXEC, FIX.4.3, port 9879):
    1. Logon handshake completes.
    2. Banzai sends `NewOrderSingle (35=D)` from its order-entry UI.
    3. fix-rs acceptor responds with two `ExecutionReport (35=8)` — New ack then full fill.
    4. Banzai's order row flips to Filled and an execution appears in its Execution blotter.
    5. Heartbeat exchange continues normally.
  *(V→W was not exercised live — Banzai has no MarketData client; it is covered by unit tests instead.)*

**M6 exit criteria**: **MET.** Application-level messages flow end-to-end between a real QFJ initiator and the fix-rs
acceptor (`D`→`8` order/execution round trip, verified in Banzai's UI). No ownership cycle — the app never references
the session; it returns messages and the engine sends them (Option B). Architecture supports unsolicited/streaming
sends via `SessionMap::send` (6.4b).

**v1 COMPLETE** — M1, M2, M4, M5, M6 done (M3 typed-codegen deferred by design; 1.6 SOH-in-Data stretch open).

## M7 — Automatic session-level error responses (post-v1 / v1.1)

**Why this exists.** As of v1, the engine *detects* every inbound error but never *responds* on the wire. In
`handle_connection`, both `Message::from_str(...)?` (parse/validation errors — `SessionRejectError`) and
`session.next_message(...)?` (session errors — `SessionError`; and app errors — `AppError`) propagate via `?`, which
ends the connection thread — the TCP connection is **silently dropped**. No `Reject (35=3)`, no `BusinessMessageReject
(35=j)`, no `Logout (35=5)` with a reason ever goes out. This was a deliberate v1 simplification (see task 4.7, which
skipped Reject generation). Our tests only assert the error is *returned/detected*, never that a corrective FIX message
is emitted — because the engine emits none.

**Scope boundary.** App-level rejects (a broken counterparty contract) are the application builder's responsibility —
the app returns its own reject/response via the `Vec<Message>` from `on_app_msg_received`. This milestone is only about
the errors the **engine must handle automatically**, before/without the app ever seeing the message: parse/validation
failures and session-level failures.

**Correct FIX 4.3 behavior by error class** (Vol 1/2):

| Error class (current type) | Correct response | Today |
|---|---|---|
| Invalid field: bad type, value out of range, required tag missing, tag out of order (`SessionRejectError`) | **Reject (35=3)** with `RefSeqNum(45)`, `SessionRejectReason(373)`, opt `RefTagID(371)`; increment target seq; keep connection | drops connection |
| Bad checksum / body length (`SessionRejectError`) | **Silently drop** the message (garbled → RefSeqNum untrusted); do NOT Reject; keep connection | drops connection |
| CompID mismatch (`SessionError`) | **Logout (35=5)** with text, then disconnect | disconnects, no Logout sent |
| MsgSeqNum too low, no PossDup (`SessionError`) | **Logout** + disconnect (fatal) | disconnects, no Logout sent |
| MsgSeqNum too high | **ResendRequest (35=2)** — deferred to a later milestone (needs message store/resend) | — |
| App-level (`AppError`) | app's responsibility (return a reject message), or engine **BusinessMessageReject (35=j)** as a fallback | drops connection |

**The architectural change.** Errors currently propagate *up and out* of the session. To respond, the session must
*catch* the error and translate it into an outbound message, then continue (recoverable) or disconnect (fatal) — instead
of `?`-ing to the acceptor. The core work is an explicit **recoverable vs fatal** classification.

- [x] **7.1 — `generate_reject` builder + error classification.**
  `Session::generate_reject(ref_seq_num, reason: SessionRejectReason)` — sibling of the existing admin builders. Stamps
  `35=3` + tags 45/373 (+ 371/58 when known), runs through `initialize_header`/`send_raw`. `SessionRejectReason` is now
  the error type itself (a data-carrying enum deriving `thiserror::Error`), with `code()` (tag 373), `ref_tag()` (371),
  `text()` (58), and `is_garbled()` — the recoverable-vs-fatal classification for the parse side (garbled → drop, else →
  reject). Session-side classification is inlined at the point of use in 7.3 rather than a separate `is_fatal()`, since
  `SessionError` is nearly all-fatal. Done: `13e7d05`.

- [x] **7.2 — Parse-side error → response.**
  In the acceptor read loop, a bad `Message::from_str`: garbled (`is_garbled()`) → `warn` + drop + keep reading;
  well-formed-but-invalid → `reject_message` (RefSeqNum from raw via `seq_num_from_raw`, `initialize_header` +
  `on_admin_msg_sending` + `send_raw`, target-seq advanced) + keep reading. First message bad → `disconnect` (a Reject is
  post-logon). Done: `0cfcdab`.

- [ ] **7.3 — Session-error side: catch, classify, respond (was the "un-box `next_message`" step).**
  Today `next_message` returns `Result<(), Box<dyn Error>>` and the acceptor `?`-es it, so any session error still kills
  the connection with nothing on the wire. Split into:

  Note on ordering: getting into 7.3 showed the dependency runs opposite to a naive "classify
  first" reading — you can't classify a `SessionError` in `next_message` until *every* inbound path
  converges to one typed result, and the `next_*` helpers + `dispatch_to_app` still `?` two
  app-callback error types (`RejectLogon`, `AppError`). So 7.3 mirrors 7.3a's rhythm: **converge the
  types first (7.3b, no behavior change), then classify + act (7.3c, the wire behavior).** See
  `session_context/error-conversion-map_err-vs-from.md` for the `#[from]` vs `map_err` reasoning.

  - [x] **7.3a — Un-box the verify layer (types only, no behavior change).**
    `verify_msg` / `verify_seq_number` return `Result<(), SessionError>` instead of `Box<dyn Error>`;
    map the internal `get_field` `FieldError`s → `SessionError::MissingHeaderField { tag }` via
    per-call `map_err` (NOT a blanket `From` — translation, not propagation). `tag` was added to
    `FieldError` (self-describing). No need to match on the `FieldError` variant in the session
    (4/5 verify fields are `String`, whose parse is `Infallible` → only `TagNotFound` reachable).
    Done: 197 tests green.

  - [ ] **7.3b — Converge the inbound chain to one typed result (types only, no behavior change).**
    Make `next_*`, `dispatch_to_app`, and `next_message` return `Result<(), SessionError>` (drop the
    `Box`). Fold the two app-callback errors in:
    - `RejectLogon` → `SessionError` via `#[from]` (the self-contained, propagation case — e.g. a
      `LogonRejected(#[from] RejectLogon)` variant); `on_admin_msg_received(...)?` then auto-converts.
    - `AppError` → a **throwaway** `App(#[from] AppError)` variant so it propagates as a `SessionError`
      and the acceptor still `?`s it → connection still drops exactly as today (zero behavior change).
      The real handling (log-and-keep-alive stopgap, then 35=j) lands in 7.3c/7.4; delete this variant
      then. (Chosen over handling `AppError` locally now, to keep 7.3b strictly type-only.)
    Also convert the three `return Err(Box::from(SessionError::...))` in `next_logon` to plain
    `return Err(...)`. Done when: it compiles and 197 tests stay green (behavior identical — bad
    message still drops the connection).

  - [ ] **7.3c — Classify + act + control signal (the wire behavior).**
    `next_message` (or a thin wrapper) catches the `SessionError`, applies recoverable-vs-fatal + the
    pre-logon gate (before `logon_received`, everything is fatal → disconnect, since Reject is
    post-logon), and acts: `MissingHeaderField` → `reject_message` (recoverable, keep reading); fatal
    (BeginStringMismatch, CompIdMismatch, SeqNumTooLow, InvalidStateForMsgType, OutOfSessionTime) +
    `LogonRejected` → `generate_logout(reason)` + `disconnect`; `App` → stopgap log + keep the
    connection (do NOT disconnect; real fix is 7.4), and drop the throwaway `App` variant in favor of
    handling `AppError` at `dispatch_to_app`. `next_message` returns a control signal (e.g.
    `ControlFlow`) so the acceptor's two `?` calls become a `match` (keep looping vs break) and all FIX
    policy stays in `Session`. Watch the seq-num double-advance: `verify_seq_number` bumps target seq
    on its OK path, `reject_message` bumps it itself — trace each error's origin relative to the bump.
    Done when: a bad post-logon field → Reject + connection survives + seq advanced once; a fatal error
    → Logout + disconnect; a rejected logon tears down cleanly; an `AppError` no longer drops the
    connection.

- [ ] **7.4 — (Optional) `generate_business_reject` (35=j).**
  Engine-level fallback for `AppError` when the app returns an error rather than its own reject message. Decide the
  boundary vs. app responsibility. Done when: an `AppError` from `on_app_msg_received` produces a `35=j` (or is
  explicitly left to the app, documented).

- [ ] **7.5 — Tests + Banzai integration.**
  Unit tests (MockResponder asserting the `3`/`5`/`j` on the wire, RefSeqNum, reason code, seq-num advance). Live check:
  send a malformed/invalid order from Banzai and confirm the Reject/Logout appears in Banzai's log and the connection
  behaves correctly (stays up for a Reject, tears down for a Logout).

**M7 exit criteria**: inbound messages that fail engine-level validation produce the correct FIX response
(Reject/Logout/drop) automatically, the connection survives recoverable errors, and the behavior is unit-tested and
verified against Banzai. Too-high seq (ResendRequest) and full resend remain deferred.
