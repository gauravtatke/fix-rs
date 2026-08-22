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
      receiving a Logout triggers logout response + disconnect, receiving an application message triggers `from_app`.

- [ ] **4.8 — Timer logic (`next_tick`).**
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

## M5 — Sync networking (sketch)

Replaces the disposable Tokio prototype in `network.rs`/`io/` with `std::net::TcpStream` + `std::thread`. Likely
sub-tasks: (a) one blocking read+process thread per session, (b) a shared timer thread for heartbeat/test-request
interval checks (mirrors QuickFIX/J's single shared timer), (c) a `Mutex`-guarded writer half of the socket since both
the per-session thread and the timer thread need to write, (d) acceptor side, (e) initiator side.

## M6 — Close out v1 (sketch)

Wire `MarketDataRequest`/`MarketDataSnapshotFullRefresh` end-to-end between a real initiator and acceptor using M3's
typed messages and M4/M5's session/network layers. This is the "v1 is done" milestone.
