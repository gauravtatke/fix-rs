# v1 Task Breakdown

Companion to `ROADMAP.md` (the why/scope) and the approved M1 plan captured in `session_context/`. This doc exists to break v1 into pieces small enough to pick up one at a time, in separate sittings, without needing to hold the whole design in your head.

**How to use this**: work top to bottom within a milestone (tasks are ordered by dependency). Check a box when a task's own tests pass. Each task lists exactly which file(s) it touches and how to know it's done. When you finish a task, it's a natural commit boundary.

Milestones M1 and M2 are broken down in real detail (they're next up). M3–M6 are sketched at a coarser grain — detailed enough to start planning from, but expect to re-break them into smaller tasks (same way M1 is broken down here) once M1/M2 are done and we know what the message/config layers actually look like.

---

## M1 — Harden the message/dictionary layer

No networking, no sessions — just making `Message`/`FieldMap` parsing and serialization correct and dictionary-driven. Touches `src/message.rs` and `src/quickfix_errors.rs` only.

- [x] **1.1 — Fix serialization ordering.**
  Change `FieldMap.fields` from `HashMap<Tag, StringField>` to `indexmap::IndexMap<Tag, StringField>` (`src/message.rs`). Add round-trip tests (parse a fixture, `.to_string()` it, assert it matches the original). *Done — 4 round-trip tests passing (`msg_test_round_trip_*`).*

- [x] **1.2 — Expose `SessionRejectReason` for testing.**
  In `src/quickfix_errors.rs`: make `SessionRejectReason` `pub`, derive `Debug, PartialEq, Eq, Clone, Copy` on it, add `SessionRejectError::kind(&self) -> SessionRejectReason`. Pure enabler, no behavior change — should compile with zero other edits.
  Done when: `cargo build` is clean and you can write `assert_matches!(err.kind(), SessionRejectReason::SomeVariant)` from a test.

- [ ] **1.3 — Wire tag-validity + required-field checks.**
  *Depends on 1.2.* Add a shared validation helper called from `parse_header`/`parse_body`/`parse_trailer` (replacing bare `set_field` calls) that checks `dd.is_msg_field(msg_type, tag)`, distinguishing "unknown to the dictionary" (`undefined_tag_err`) from "known tag, wrong message type" (`tag_not_defined_for_msg`, already used in `parse_group` — extend its use here). Add a post-parse check in `from_vec` diffing `dd.get_msg_required_field(msg_type)` (+ `"header"`/`"trailer"`) against what's present → `required_tag_missing_err` on gaps.
  Watch for: the existing `msg_test_with_body_group`/`msg_test_with_group_and_subgroups` fixtures may not be fully FIX-spec-complete for required fields — if turning this on breaks them, add the missing required fields to those fixture strings rather than weakening the check.
  Done when: un-stub `msg_test_trailer_with_more_fields` (give it a real body) + new tests for missing-required-tag, undefined-tag, wrong-msgtype, each asserting the specific `SessionRejectReason` via `kind()`.

- [ ] **1.4 — Wire enum-value + type-format checks.**
  *Depends on 1.3 (same validation helper).* If `dd.get_field_values(tag)` is `Some(set)`, check membership → `value_out_of_range_err`. Validate the raw string against `dd.get_field_type(tag)`'s category (numeric types must parse as such, `Boolean` must be `Y`/`N`, etc.) → `incorrect_data_format_err`.
  Done when: new tests cover an out-of-domain enum value and a malformed numeric field, each asserting the right `SessionRejectReason`.

- [ ] **1.5 — Wire incoming checksum/body-length verification.**
  *Independent of 1.3/1.4 — can be done before or after them.* In `from_str`, recompute expected body length/checksum from the raw string and compare to the incoming `9=`/`10=` fields (check body length first, it's cheaper) → `invalid_body_len_err`/`invalid_checksum`.
  Done when: un-stub `msg_test_invalid_checksum` and `msg_test_invalid_body_length` with deliberately corrupted fixtures (flip a digit in `9=` or `10=`).

- [ ] **1.6 — (Stretch) SOH-in-Data-field handling.**
  *Independent — skip freely, none of v1's required messages need this.* `Message::from_str`'s tokenizer naively splits on SOH, which breaks for `DATA`-typed fields whose raw bytes legitimately contain SOH. If you pick this up: first verify the Length/Data tag-pairing convention against real pairs in `resources/FIX43.xml` — it is *not* reliably "tag minus 1" (e.g. header's `90/91 SecureDataLen/SecureData` breaks that pattern) — before writing tokenizer logic.
  Done when: `msg_test_soh_in_data_field`/`msg_test_soh_in_non_data_field` pass, or are left `#[ignore]`d with a one-line reason if descoped.

- [ ] **1.7 — Housekeeping.**
  Update `ROADMAP.md`'s milestone list to reflect however M1 actually shook out (note any deviations from this task list), and log the wrap-up in `session_context/`.

**M1 exit criteria**: `cargo test` green, all of `src/message.rs`'s error paths go through `SessionRejectError` (no stray `Result<T, String>` for validation failures), message layer rejects malformed/non-compliant input instead of silently accepting it.

---

## M2 — Config rewrite (toml + serde)

Replaces `src/session/session_settings.rs`'s hand-rolled parser. Independent of M1 — can be done in parallel or first, your call.

- [ ] **2.1 — Add `toml` dependency and define the target shape.**
  Add `toml = "0.5"` (or current stable) to `Cargo.toml`. Decide the final typed struct(s) you want out the other end (likely close to today's `Properties`/per-`SessionId` settings map). No merge logic yet — just get a `#[derive(Deserialize)]` struct that can parse a *single* `[Default]`-style block in isolation, with a unit test proving it.

- [ ] **2.2 — Implement the default + per-session merge.**
  *Depends on 2.1.* Recommended pattern (avoids custom `Deserialize` impls / `#[serde(flatten)]` entirely, which is where the last attempt got stuck): parse the whole file into `HashMap<String, toml::Value>` per `[Default]`/`[Session]` block first, then for each session block start from a clone of the default map and `.extend()` the session-specific keys on top (session wins on collision) — merge at the `toml::Value` level, *then* run the merged map through `try_into::<YourSettingsStruct>()`.
  Done when: a unit test with one `[Default]` + two `[Session]` blocks (mirroring the existing `session_sample_config_test` fixture in `session_settings.rs`) proves per-session overrides win and un-overridden keys fall back to default.

- [ ] **2.3 — Swap call sites over to the new loader.**
  *Depends on 2.2.* Replace `Properties::new`/`get_config`/`get_optional_config`/etc. call sites (`src/session/session_and_state.rs`, `src/network.rs`, `src/main.rs`) with the new struct. This is mostly mechanical once 2.2 is solid.

- [ ] **2.4 — Port/replace existing config tests + validation.**
  *Depends on 2.3.* The current `Properties::check()` validation (connection type, ports, begin string, comp IDs present) should have an equivalent — either hand-written post-deserialize checks or serde-level constraints. Port the existing `session_setting_tests` (`test_no_default_section`, `test_default_no_connection_type`, etc.) to the new loader.

**M2 exit criteria**: `src/FixConfig.toml` loads through real `toml`+`serde`, default/session merge is unit-tested, `session_settings.rs`'s hand-rolled parser is deleted.

---

## M3 — Typed message codegen (sketch — will be re-broken-down later)

Reworks `build/main.rs` + `build/code_generator.rs` + `build/templates.rs` to emit real per-message-type structs (e.g. `Logon`, `Heartbeat`, `MarketDataRequest`, `MarketDataSnapshotFullRefresh`) with typed field accessors, replacing the currently-unused per-field struct generation. Likely sub-tasks once we get here: (a) design the generated struct shape, (b) generate for the admin messages first (Logon/Heartbeat/TestRequest/Logout) since M4 needs them, (c) generate for MarketDataRequest/MarketDataSnapshotFullRefresh, (d) wire generated types into `Message`/`FieldMap` so they're not a parallel disconnected API.

## M4 — Session state machine (sketch)

Logon/Heartbeat/TestRequest/Logout sequencing and session state, building on `src/session/session_and_state.rs`, using the now-hardened `Message`/`DataDictionary` layer and M2's config. Likely sub-tasks: (a) `SessionState` struct (booleans/timestamps, no I/O), (b) logon/logout transition logic against `SessionState` in isolation (testable without a socket), (c) heartbeat/test-request timing logic, also testable in isolation.

## M5 — Sync networking (sketch)

Replaces the disposable Tokio prototype in `network.rs`/`io/` with `std::net::TcpStream` + `std::thread`. Likely sub-tasks: (a) one blocking read+process thread per session, (b) a shared timer thread for heartbeat/test-request interval checks (mirrors QuickFIX/J's single shared timer), (c) a `Mutex`-guarded writer half of the socket since both the per-session thread and the timer thread need to write, (d) acceptor side, (e) initiator side.

## M6 — Close out v1 (sketch)

Wire `MarketDataRequest`/`MarketDataSnapshotFullRefresh` end-to-end between a real initiator and acceptor using M3's typed messages and M4/M5's session/network layers. This is the "v1 is done" milestone.
