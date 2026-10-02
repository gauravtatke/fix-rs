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

> **Repository restructure (2026-09-27) — path mapping.** The source tree was reorganized into layered modules and
> split into a **library (`fix_rs`) + demo binary**; the crate root is now `src/lib.rs` (it carried the build-script
> `include!` until 3.9b removed it), and `src/main.rs` is the demo binary. Task text written before this uses the old paths — map them as:
> `src/fix43/`→`src/messages/fix43/`, `src/common.rs`→`src/convert.rs`, `src/fix_errors.rs`→`src/errors.rs`,
> `src/message.rs`→`src/core/message.rs`, `src/data_dictionary.rs`→`src/core/dictionary.rs`,
> `src/network.rs`→`src/transport/{registry,timer}.rs`, `src/io/`→`src/transport/sync/` (`fix_message_reader`→`reader`,
> `tcp_responder`→`responder`), `src/session/session_and_state.rs`→`src/session/mod.rs`, `session_settings.rs`→
> `settings.rs`, `session_id.rs`→`id.rs`, `session_schedule.rs`→`schedule.rs`. `src/types.rs` was deleted.

---

## M1 — Harden the message/dictionary layer

No networking, no sessions — just making `Message`/`FieldMap` parsing and serialization correct and dictionary-driven.
Touches `src/message.rs` and `src/quickfix_errors.rs` only.

- [x] **1.1 — Fix serialization ordering.**
  Change `FieldMap.fields` from `HashMap<Tag, StringField>` to `indexmap::IndexMap<Tag, StringField>`
  (`src/message.rs`). Add round-trip tests (parse a fixture, `.to_string()` it, assert it matches the original). *Done —
  4 round-trip tests passing (`msg_test_round_trip_*`).*

- [x] **1.2 — Expose `SessionRejectError` for testing.**
  In `src/quickfix_errors.rs`: make `SessionRejectError` `pub`, derive `Debug, PartialEq, Eq, Clone, Copy` on it, add
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
  undefined-tag, wrong-msgtype, each asserting the specific `SessionRejectError` via `kind()`. *(Done — see
  `session_context/2026-07-26.md` for the design reasoning behind where each check ended up living, and a real gap found
  in the process: the group-instance-level required-field check needed `rg_dd`, not the `dd` parameter, since `dd` is
  silently inherited unchanged across recursive nested-group calls.)*

- [x] **1.4 — Wire enum-value + type-format checks.**
  *Depends on 1.3 (same validation helper).* If `dd.get_field_values(tag)` is `Some(set)`, check membership →
  `value_out_of_range_err`. Validate the raw string against `dd.get_field_type(tag)`'s category (numeric types must
  parse as such, `Boolean` must be `Y`/`N`, etc.) → `incorrect_data_format_err`. Done when: new tests cover an
  out-of-domain enum value and a malformed numeric field, each asserting the right `SessionRejectError`.

- [x] **1.5 — Wire incoming checksum/body-length verification.**
  *Independent of 1.3/1.4 — can be done before or after them.* In `from_str`, recompute expected body length/checksum
  from the raw string and compare to the incoming `9=`/`10=` fields (check body length first, it's cheaper) →
  `invalid_body_len_err`/`invalid_checksum`. Done when: un-stub `msg_test_invalid_checksum` and
  `msg_test_invalid_body_length` with deliberately corrupted fixtures (flip a digit in `9=` or `10=`).

- [x] **1.6 — SOH-in-Data-field handling. DONE (2026-09-20).**
  `Message::from_str` was rewritten from a blind `split_terminator(SOH)` to a positional byte-cursor tokenizer: a
  `DATA`-typed field's value is read by byte count (so embedded SOH is preserved), and every field — data or not — must
  be SOH-terminated (a missing terminator is rejected). The length is read from the *previous* token (`vdeq.back()`),
  since FIX requires the length field to immediately precede its data field — so no `tag-1`/`89→93` calc is needed (the
  sole real exception, Signature 89←93, falls out for free). A guard rejects a wrong/short/overrunning length rather
  than panicking on the slice. `NonDataFieldIncludeSOHChar` (373=17) is **descoped** — QFJ doesn't emit it either — so
  `msg_test_soh_in_non_data_field` is `#[ignore]`d. See `session_context/qfj-data-field-parsing.md`. Done:
  `msg_test_soh_in_data_field` (+ Signature special case, missing/short/overrunning/non-numeric length, and
  `msg_test_missing_trailing_soh_rejected`) pass; 227 tests green.

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

## M3 — Typed message codegen (v1.1 phase)

Generate, per message type in a FIX XML, a typed Rust facade over `Message` with strict-typed
per-field accessors + generic get/set for custom fields — the Rust equivalent of QFJ's generated
message classes. **Full design is settled in `session_context/v1.1-codegen-design-discussion.md`**
(decisions D1–D11); read it before starting. Key shape: `struct Logon { message: Message }` facade
(D1); named accessors taking the field's bare natural type (D2) — `Decimal` for money (D8), real
`enum`s for value-constrained fields (D3), `chrono` for dates, `impl Into<String>` for strings;
`has_*()` presence; `TryFrom<Message>`/`Into<Message>` at the seams; raw access via
`message()`/`message_mut()` (D1a). Generated by a **standalone binary → committed `.rs` modules**
+ freshness test (D4a), all of FIX43, XML-driven (D7). Groups = nested facades over the instance
`FieldMap`, zero-copy views, auto-managed count (D9).

**Layout (D11):** per-version message modules `src/fix43/` (→ `src/fix44/` …); shared version-neutral
`src/tags.rs` (field→tag registry) + `src/common.rs` (FIX value⇄Rust format helpers: bool `Y`/`N`,
UTC timestamp, `Decimal` later); `TypedError` in `src/fix_errors.rs`. **Engine admin messages stay
raw/version-neutral (D10)** — the typed layer is for the app; `generate_logon` etc. are NOT migrated.

**Ownership split for this milestone** (agreed): the **hand-written reference messages + all tests
+ comments are Claude's**; the **standalone codegen binary is the user's**. The hand-written
references are the golden target the generator must reproduce (diffed by the freshness test).

- [x] **3.0 — Design decisions.** Captured in `session_context/v1.1-codegen-design-discussion.md`
  (D1–D9). Done.

- [x] **3.1 — Hand-write reference `Logon` (Claude). DONE.**
  `src/fix43/logon.rs`: `Logon { message: Message }` facade — named natural-type setters/getters +
  `has_*` for all scalar Logon fields, `TryFrom<Message>` (checks 35=A → `TypedError`),
  `From<Logon> for Message`, `message()`/`message_mut()`. First value enum `EncryptMethod`
  (`src/fix43/fields.rs`, `to_fix`/`from_fix`). Body-field tags from the shared `src/tags.rs`; bool
  `Y`/`N` via `src/common.rs`. 11 tests (round-trip, TryFrom accept/reject, enum↔FIX-int, bool Y/N,
  presence, missing-required). 247 tests green. Golden reference for 3.4.

- [x] **3.2 — FIX-type → Rust-type map + internal format layer (Claude, factored from 3.1). DONE
  (2026-09-27).** `src/common.rs` now has the full format layer: bool (`Y`/`N`), `Decimal` money
  (PRICE/AMT/QTY/PRICEOFFSET, D8 — exact, exponent rejected), UTC timestamp (tolerant of
  absent/variable fractional), `NaiveDate` (UTCDATE/LOCALMKTDATE `YYYYMMDD`), `NaiveTime`
  (UTCTIMEONLY). Plain passthrough types (`f64` for FLOAT/PERCENTAGE, `i32` for INT, `u32` for
  LENGTH/NUMINGROUP/SEQNUM/TAGNUM, `char`, `String` incl. DATA/COUNTRY/CURRENCY/EXCHANGE/MONTHYEAR/
  MULTIPLEVALUESTRING) need no helper — Rust's `FromStr`/`Display` already match FIX. **The full
  FIX 4.3 type→Rust map lives as the authoritative table in `common.rs`'s module doc** (what the
  generator consults). `rust_decimal`/`rust_decimal_macros` deps added. 13 round-trip/rejection
  tests (incl. 18-digit exactness where `f64` rounds, non-leap Feb 29, exponent reject). Also
  flipped `c_indexed`'s MDEntryPx/MDEntrySize from the `f64` placeholder to `Decimal`.

- [x] **3.3 — Hand-write a couple of field enums (Claude). DONE (2026-09-27).** `EncryptMethod`
  (INT), plus `Side` (12 values) and `OrdType` (23 values) as CHAR enums in
  `messages/fix43/fields.rs` — values sourced exactly from `FIX43.xml`, `to_fix -> char` /
  `from_fix(char) -> Option`, tests round-trip every variant + reject unknowns. **D11 enum-sharing
  resolved: per-version** (not a shared union) — preserves D3's invalid-value-unrepresentable per
  version; see the D11 enum-sharing note in the design log.

- [x] **3.4 — Codegen scaffold (workspace + skeleton). DONE (2026-09-27; plumbing by Claude).**
  Cargo workspace (root `fix-rs` + `codegen` member, edition 2024; clippy lints hoisted to
  `[workspace.lints]`). `codegen/src/main.rs`: `generate() -> Vec<(PathBuf, String)>` (pure core,
  stub), `rustfmt()` wiring, `main` (writes), and the **freshness test** `committed_output_is_fresh`
  (rerun generate → diff committed files → fail with "run `cargo run -p codegen` and commit").
  `codegen/src/spec.rs`: roxmltree parse of the `<fields>` section into raw-data
  `FixSpec/FieldDef/FieldValue` (ported from `build/code_generator.rs`, minus Handlebars / the buggy
  f32 type map / the `ENVal_` prefix). ~~Emission = plain string-building + rustfmt (not a template
  engine)~~ — **superseded 2026-10-02 (design log D4d): minijinja templates** for every generated
  file (`tags.rs.j2`, `fields.rs.j2`); `generate()` replaced by an `OUTPUTS`
  table of `(path, emitter fn)` rendered + written one file at a time, which the freshness test
  iterates too. Run: `cargo run -p codegen`; check: `cargo test -p codegen`.

- [x] **3.5 — Emit `tags.rs` + `fields.rs` value enums (USER). DONE (2026-10-02).** First real emitters; both work with
  the current `<fields>`-only parser (no message parsing needed yet).
    - [x] **3.5a — `tags.rs`. DONE (2026-09-30).** Every field → `pub const <NAME>: u32 = <tag>;`,
      **sorted by tag** for a stable diff. Names via heck shouty-snake, matching the hand-written casing
      incl. acronyms (`MDEntryPx`→`MD_ENTRY_PX`, `NoMDEntries`→`NO_MD_ENTRIES`). `cargo run -p codegen`
      regenerates `src/tags.rs` (all 635 FIX43 fields + generated-file header), freshness green, engine
      compiles. Also delivered:
        - **`codegen/src/naming.rs`** — the single XML-name → Rust-name layer. A `match` correction
          table applied *before* heck (`NoPartyIDs`/`NoNestedPartyIDs`/`NoRoutingIDs` → `…Ids`,
          `IOIid` → `IoiId`, `XMLnonFIX` → `XmlNonFix`, keyword `Yield` → `YieldValue`), an
          `ACCEPTED_NAMES` list for reviewed-and-kept names (`Rule80A`, `OutMainCntryUIndex`), and
          `const_name()`. Rule: emitters never call heck on a raw XML name. Later emitters add
          `fn_name`/`type_name` siblings.
        - **`cargo run -p codegen -- audit <dictionary.xml>`** (`codegen/src/audit.rs`) — reports
          names heck may mis-case: precise (single-letter segment, Rust keyword, collision) + noisy
          (acronym-then-lowercase, digits). Reports only; decisions go in `naming.rs`. Checked
          FIX42/44/50SP2: the `…IDs` pattern dominates (22 hits in 5.0SP2).
        - **Guard tests** (10 new, codegen crate): FIX43 has no undecided precise findings; final
          (post-correction) names are clean + unique; each detector category fires; naming table
          pinned.
    - [x] **3.5b — `fields.rs` enums. DONE (2026-10-02).** Every value-constrained FIX43 field →
      a real enum (D3, per-version): **117 enums / 975 variants**, generated from QFJ's dictionary
      (3.5c) with **zero variant-name corrections needed**.
        - `codegen/templates/fields.rs.j2` + `emitter::emit_field_enums` (minijinja, D4d). A view
          model (`FieldEnum`/`Variant`) carries everything pre-decided — names, literals, order — so
          the template holds layout only.
        - **Shape (D3 addendum):** explicit `match` in both directions for every FIX type — no
          `#[repr]` / discriminants / `as i32` (superseded the hand-written INT trick).
          `to_fix(self) -> char | i32 | &'static str`; `from_fix(value: char | i32 | &str) ->
          Option<Self>` as a plain match (`=> Some(..)`, `_ => None`). Variants sorted by wire code;
          doc line per enum (tag + FIX type) and per variant (wire value).
        - **Type map:** CHAR→`char`, INT/NUMINGROUP→`i32`, STRING/MULTIPLEVALUESTRING→`&'static str`
          (one token; splitting is the message accessor's job), anything else → panic. The literal
          is formatted from the *Rust* type (one source of truth — fixed a NUMINGROUP bug where
          `NoSides` got `"1"` for an `i32`). **BOOLEAN fields are skipped** (→ `bool`).
        - Names via `naming::enum_name` / `variant_name` (correction table, then heck UpperCamel).
        - Guard: panic naming the field on a duplicate wire code or variant name within one enum.
        - `messages/fix43/generated/fields.rs` is generated (hand-written reference replaced —
          original at `git show fe21f06:src/messages/fix43/generated/fields.rs`); `tests/fields.rs` +
          `tests/logon.rs` green against it, freshness green.
        - **The audit is NOT extended for values (decided):** digit-leading/keyword/collision are all
          compile errors, so `cargo build` is the guard; the audit only earns its keep for silent
          mis-casing.
      **Follow-up (minor):** the INT-numeric sort in `to_field_enum` still keys on
      `fd.fix_type == "INT"`; switch it to `rust_type == "i32"` like the literal, so NUMINGROUP codes
      sort numerically too (no visible effect today — `NoSides` has only `1`, `2`).
      **Emit into `messages/fix43/generated/` (generator-owned subtree, incl. its `mod.rs` index);
      hand-written tests live in `messages/fix43/tests/` and are never generated — design log D4c.**
    - [x] **3.5c — Switch the dictionary to QFJ's `FIX43.xml` (design log D12). DONE (2026-10-02).**
      Pulled ahead of 3.5b's naming work: QFJ's file generates with no variant corrections, making the
      old file's 17 digit-leading fixes moot. All QFJ dictionaries (4.0–5.0SP2, FIXT1.1) copied to
      `resources/` (untracked, reference only).
        - **Baseline + patches (D12b):** commit `1ed1a71` = QFJ's file verbatim; a following commit
          patches it in place — header comment (source, QFJ version) + a `<!-- MODIFIED: -->` comment
          at each change. Patches: MatchType S1–S5 (QFJ had only S5) and collapsed duplicate
          M1/M2/MT (per-market meanings in the spec).
        - **Strict loader kept (D12a):** duplicate enum codes remain a load error — fixed in the XML,
          never tolerated in code. Only relaxation: root `type` optional, defaults to `"FIX"` (runtime
          loader + legacy `build.rs`), as QFJ does.
        - `FixType::DayOfMonth` (DAYOFMONTH, int; fields 205/314, retired in 4.3) — validated as int,
          in `convert.rs`'s type table (→ `i32`).
        - Codegen: `tags.rs` +20 consts; `naming.rs` key `IOIid` → `IOIID` (const stays `IOI_ID`).
        - Tests (Claude): Forex/EncryptMethod variant renames + per-Forex wire-char pins;
          `test_major_minor_type` (missing/empty `type` → `"FIX"`, present `type` kept); `55=` added to
          the two NewOrderList fixtures (body length 215→231, checksum recomputed). 264 engine + 11
          codegen tests green, freshness green, `cargo fmt --check` clean.

- [~] **3.6 — Runtime `Group`/`FieldMap` firming-up (enabler for groups).**
  DONE (seeded 2026-09-26, by Claude alongside 3.7): `FieldMap::add_group_instance` (append +
  auto-bump count — fixes the set_group/add_group desync), `push_group_instance`, `group_instances`,
  `get_group_mut`; `Group::as_slice/as_mut_slice`; `Message::body_group*` (`src/message.rs`).
  Remaining: any extra helpers the generator needs once the group shape is finalized.

- [~] **3.7 — Group ergonomics comparison + reference (Claude).**
  Built A/B/C/D variants to compare group build/read call sites (D9 validation). **CHOSEN =
  `c_indexed`** (2026-09-27): facade over the message's group instances, closure builder +
  `index(i)`/`index_mut(i)` returning borrowed views *by value* + `iter`/`iter_mut`/`len` —
  **zero-copy AND zero `unsafe`**, and it recurses into nested groups (the flat variant D doesn't).
  The B-reworked lead (transparent-slice cast) was overturned for its `unsafe` (see
  `feedback_no_unsafe` + D9 resolution in the design log). NOT the `Index`/`IndexMut` traits (would
  force the cast), but `index(i)`/`index_mut(i)` **panic on OOB** matching the trait's contract —
  sound because the parse layer validates group count == instances, so OOB is a caller bug (D5's
  `Result` still governs *field* access). Full A/B/C/D exploration preserved on branch
  `explore/repeating-group-ergonomics`; dev keeps only `c_indexed`. `preset_md_entries(n)` +
  by-value `add(MdEntry)` prototyped then deferred to a later convenience layer. **Remaining:**
  promote the shape into the real MarketData reference (hand-written `MarketDataSnapshotFullRefresh`
  with `Decimal` money fields) once D8/3.2 lands.

- [ ] **3.8 — Extend `spec::parse` to messages; emit admin message facades (USER).**
    - [ ] **3.8a — Parse `<messages>`.** msg_type / msg_cat + each message's ordered fields + required
      flags, resolving `<component>` references (the parser currently handles `<fields>` only).
    - [ ] **3.8b — Emit `Logon`** (the old 3.4 "Logon byte-identical" first goal). Facade over
      `Message`: named natural-type accessors (via the `convert` map + enums), `has_*`,
      `TryFrom<Message>` (checks 35=A) + `From<_> for Message`, `message()`/`message_mut()`. Reproduce
      the 3.1 hand-write — now the *pure* `messages/fix43/generated/logon.rs` (its tests were
      extracted to `messages/fix43/tests/logon.rs`, D4c), so the generated file is the byte-for-byte
      target. Done: generated `Logon` compiles, the hand-written `tests/logon.rs` still passes against
      it, freshness green.
    - [ ] **3.8c — Emit remaining admin messages.** Heartbeat/TestRequest/Logout/Reject/
      ResendRequest/SequenceReset. Done: each has typed accessors + `TryFrom`/`Into`, tests green.

- [~] **3.9 — Groups + app messages + remove the old `build.rs` codegen (USER).**
    - [ ] **3.9a — Groups + app messages.** Extend the parser to repeating groups (nested, via
      component refs); emit the group facade in the **`c_indexed` shape** (closure builder,
      `index`/`index_mut` panic-on-OOB, zero-copy, auto count, nesting by recursion) +
      `MarketDataRequest`/`MarketDataSnapshotFullRefresh` + `NewOrderSingle`/`ExecutionReport`. Done:
      generated group messages match the 3.7 shape, tests green.
    - [x] **3.9b — Remove `build.rs`. DONE (2026-10-02, Claude).** Its only consumer was
      `core/message.rs` `from_vec` (`BeginString/BodyLength/MsgType/CheckSum::field()` → tags
      8/9/35/10): now `tags::BEGIN_STRING/BODY_LENGTH/MSG_TYPE/CHECK_SUM`, `use crate::fields::*`
      dropped. Removed the `include!` from `lib.rs`, `build = "build/main.rs"` and the whole
      `[build-dependencies]` (roxmltree/handlebars/serde/heck) from `Cargo.toml`, deleted `build/`.
      Clean build has no build-script warnings; 264 engine + 11 codegen tests, freshness, fmt green.
      Pulled ahead of 3.9a: once `tags.rs` was generated nothing else depended on it. CLAUDE.md's
      "Build-time code generation" section replaced by a "Code generation (`codegen/` crate)" one.
    - [ ] **3.9c — Full dictionary + module org.** Emit all FIX43 messages; organize per
      message/category (not one file); leave a per-version feature-gate hook (single version now).

- [ ] **3.10 — Factory + Cracker (D6) + SampleApp migration (USER app code; Claude tests).**
  `MessageFactory` (msgType → typed message) + `MessageCracker`-style typed inbound dispatch wired
  into the `from_app` seam; migrate `SampleApp` to build/read typed messages. Done when: SampleApp's
  `NewOrderSingle`→`ExecutionReport` flow uses typed messages end-to-end (re-verify vs QFJ Banzai).

**M3 exit criteria**: a committed, XML-driven typed message layer for all of FIX43 (facade over
`Message`, typed accessors, enums, `Decimal` money, zero-copy groups), generated by a standalone
binary guarded by a freshness test; `SampleApp` uses it end-to-end; old per-field `build.rs` codegen
removed.

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
- [x] **5.5 — `IoInitiator`.** `src/io/acceptor.rs`: binds `TcpListener`, spawns reader thread per connection,
  dispatches via `reverse_session_id` lookup. Supporting changes: `Session::set_responder()`,
  `session_id_from_raw()`/`reverse_session_id_from_raw()` in `message.rs`.
- [x] **5.6 — Timer thread.** `src/network.rs`: `start_timer()` spawns background thread, sleep 1s → tick all sessions.
- [x] **5.7 — Wire `main.rs` + integration test with QFJ Banzai.**
    - `SessionConfig::to_session()` constructs Session from config.
    - `connection_type()` and `socket_accept_port()` getters exposed on `SessionConfig`.
    - `main.rs` wired: parse config → build `SessionMap` → start timer → spawn one `IoInitiator` thread per bind
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
- **Option B — app-in-session (CHOSEN):** keep `app: Box<dyn Application>` as a field of `Session`. This is *not* a
  cycle, because `on_app_msg_received` returns `Vec<Message>` — the app returns the messages it wants sent and the
  engine sends them, so there is no `app → session` edge. No `SessionEntry`, no split-borrow ceremony.

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
  `test_request_response_send_via_return_value` (V→W returned and captured) and
  `test_donotsend_skips_without_error_or_seqnum_bump`. *Follow-up:* strengthen the response-path test to assert stamped
  headers (BeginString/CompIDs/MsgSeqNum), per the original "done when".

- [x] **6.4b — (Bonus, not originally scoped.) External / streaming outbound send path.**
  Added while studying how QFJ streams market data (`session_context/qfj-outbound-market-data.md`): the push model for
  unsolicited sends (a market-data feed sending whenever data is available, not on a poll).
    - `SessionMap::send(&self, sid, msg) -> Result<(), SendError>` — the `Session.sendToTarget` analogue: registry
      lookup + per-session lock + `send_app_message`. Callable from any thread holding a `SessionMap` clone.
    - `SendError { SessionNotFound, NotLoggedOn }` in `quickfix_errors.rs`.
    - `Application::poll_outbound` (default empty) + timer wiring — a low-rate seam only; **not** the market-data path.
    - Tests: external-thread push (`test_session_map_send_pushes_from_external_thread`), `NotLoggedOn`/`SessionNotFound`
      guards, `poll_outbound` streaming.
    - Rule to remember: the per-session `Mutex` is not reentrant, so never call `SessionMap::send` for a session's own
      id from inside that session's callback (deadlock) — use the `Vec<Message>` return path there.

- [x] **6.5 — Sample application (`src/sample_app.rs`).**
  `SampleApp` implements `Application`. `on_app_msg_received` handles two inbound app types:
    - `V` (MarketDataRequest) → `W` (MarketDataSnapshotFullRefresh) echoing MDReqID (262) + a `NoMDEntries` (268)
      group with dummy bid/offer entries.
    - `D` (NewOrderSingle) → two ExecutionReports (35=8): a New ack (`150=0/39=0`, LeavesQty=OrderQty) followed by a
      full fill (`150=F/39=2`, LeavesQty=0, CumQty=qty, LastPx/LastQty), echoing ClOrdID/Side/Symbol.
    - all other types → `Ok(vec![])`. Inbound field reads use `?` + `AppError::FieldNotFound { tag }` (no panics on
      malformed peer messages). Wired into
      `main.rs` (replaces `DefaultApplication`). 7 unit tests. *Note:* the task originally scoped only V→W, but Banzai
      is an order-entry client (no MarketData UI), so `D`→`8` was added as the flow an actual QFJ counterparty can
      drive; V→W is kept and unit-tested for a future MarketData-capable simulator.

- [x] **6.6 — Integration test with QFJ Banzai.**
  End-to-end against a QFJ Banzai initiator (BANZAI→EXEC, FIX.4.3, port 9879):
    1. Logon handshake completes.
    2. Banzai sends `NewOrderSingle (35=D)` from its order-entry UI.
    3. fix-rs acceptor responds with two `ExecutionReport (35=8)` — New ack then full fill.
    4. Banzai's order row flips to Filled and an execution appears in its Execution blotter.
    5. Heartbeat exchange continues normally. *(V→W was not exercised live — Banzai has no MarketData client; it is
       covered by unit tests instead.)*

**M6 exit criteria**: **MET.** Application-level messages flow end-to-end between a real QFJ initiator and the fix-rs
acceptor (`D`→`8` order/execution round trip, verified in Banzai's UI). No ownership cycle — the app never references
the session; it returns messages and the engine sends them (Option B). Architecture supports unsolicited/streaming sends
via `SessionMap::send` (6.4b).

**v1 PHASE COMPLETE** — M1, M2, M4, M5, M6 done, plus M7 (session error responses) and M8 (initiator + auto-reconnect,
live-verified). The NewOrderSingle→ExecutionReport flow that was originally scoped under "v1.1" is part of the v1 phase
(delivered live in M6).

## Release phases toward v1.0 (see ROADMAP.md "Scope")

The remaining work is two phases that culminate in the **v1.0 release** (v1.0 = v1 + v1.1 + v1.2). Sequenced by the
phase order:

1. **v1.1 — M3 typed message codegen** (sketched above; re-break into concrete tasks when we start it) — next up.
2. **v1.2 — Resend/seq robustness** — message store + ResendRequest / gap fill / sequence-number persistence (the
   too-high-seq path is stubbed across M4/M5/M7). Needs its own breakdown. **v1.0 is cut when this lands.**
3. **7.6 (b)** — scripted end-to-end TCP simulator (deferred from M7); independent of the phase order.

## M7 — Automatic session-level error responses (post-v1 / v1.1)

**Why this exists.** As of v1, the engine *detects* every inbound error but never *responds* on the wire. In
`handle_connection`, both `Message::from_str(...)?` (parse/validation errors — `SessionRejectError`) and
`session.next_message(...)?` (session errors — `InboundMsgError`; and app errors — `BusinessMsgReject`) propagate via
`?`, which ends the connection thread — the TCP connection is **silently dropped**. No `Reject (35=3)`, no `BusinessMessageReject
(35=j)`, no `Logout (35=5)` with a reason ever goes out. This was a deliberate v1 simplification (see task 4.7, which
skipped Reject generation). Our tests only assert the error is *returned/detected*, never that a corrective FIX message
is emitted — because the engine emits none.

**Scope boundary.** App-level rejects (a broken counterparty contract) are the application builder's responsibility —
the app returns its own reject/response via the `Vec<Message>` from `on_app_msg_received`. This milestone is only about
the errors the **engine must handle automatically**, before/without the app ever seeing the message: parse/validation
failures and session-level failures.

**Correct FIX 4.3 behavior by error class** (Vol 1/2):

| Error class (current type)                                                                                 | Correct response                                                                                                               | Today                       |
|------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|-----------------------------|
| Invalid field: bad type, value out of range, required tag missing, tag out of order (`SessionRejectError`) | **Reject (35=3)** with `RefSeqNum(45)`, `SessionRejectReason(373)`, opt `RefTagID(371)`; increment target seq; keep connection | drops connection            |
| Bad checksum / body length (`SessionRejectError`)                                                          | **Silently drop** the message (garbled → RefSeqNum untrusted); do NOT Reject; keep connection                                  | drops connection            |
| CompID mismatch (`InboundMsgError`)                                                                        | **Logout (35=5)** with text, then disconnect                                                                                   | disconnects, no Logout sent |
| MsgSeqNum too low, no PossDup (`InboundMsgError`)                                                          | **Logout** + disconnect (fatal)                                                                                                | disconnects, no Logout sent |
| MsgSeqNum too high                                                                                         | **ResendRequest (35=2)** — deferred to a later milestone (needs message store/resend)                                          | —                           |
| App-level (`BusinessMsgReject`)                                                                            | app's responsibility (return a reject message), or engine **BusinessMessageReject (35=j)** as a fallback                       | drops connection            |

**The architectural change.** Errors currently propagate *up and out* of the session. To respond, the session must
*catch* the error and translate it into an outbound message, then continue (recoverable) or disconnect (fatal) — instead
of `?`-ing to the acceptor. The core work is an explicit **recoverable vs fatal** classification.

- [x] **7.1 — `generate_reject` builder + error classification.**
  `Session::generate_reject(ref_seq_num, reason: SessionRejectReason)` — sibling of the existing admin builders. Stamps
  `35=3` + tags 45/373 (+ 371/58 when known), runs through `initialize_header`/`send_raw`. `SessionRejectError` is now
  the error type itself (a data-carrying enum deriving `thiserror::Error`), with `code()` (tag 373), `ref_tag()` (371),
  `text()` (58), and `is_garbled()` — the recoverable-vs-fatal classification for the parse side (garbled → drop, else →
  reject). Session-side classification is inlined at the point of use in 7.3 rather than a separate `is_fatal()`, since
  `InboundMsgError` is nearly all-fatal. Done: `13e7d05`.

- [x] **7.2 — Parse-side error → response.**
  In the acceptor read loop, a bad `Message::from_str`: garbled (`is_garbled()`) → `warn` + drop + keep reading;
  well-formed-but-invalid → `reject_message` (RefSeqNum from raw via `seq_num_from_raw`, `initialize_header` +
  `on_admin_msg_sending` + `send_raw`, target-seq advanced) + keep reading. First message bad → `disconnect` (a Reject
  is post-logon). Done: `0cfcdab`.

- [x] **7.3 — Session-error side: catch, classify, respond (was the "un-box `next_message`" step).**
  Today `next_message` returns `Result<(), Box<dyn Error>>` and the acceptor `?`-es it, so any session error still kills
  the connection with nothing on the wire. Split into (all done; committed across 7a2f8e3 / 059e62c / 7.3c commit):

  Note on ordering: getting into 7.3 showed the dependency runs opposite to a naive "classify first" reading — you can't
  classify a `InboundMsgError` in `next_message` until *every* inbound path converges to one typed result, and the
  `next_*`
  helpers + `dispatch_to_app` still `?` two app-callback error types (`RejectLogon`, `BusinessMsgReject`). So 7.3
  mirrors 7.3a's rhythm: **converge the types first (7.3b, no behavior change), then classify + act (7.3c, the wire
  behavior).** See
  `session_context/error-conversion-map_err-vs-from.md` for the `#[from]` vs `map_err` reasoning.

    - [x] **7.3a — Un-box the verify layer (types only, no behavior change).**
      `verify_msg` / `verify_seq_number` return `Result<(), SessionError>` instead of `Box<dyn Error>`; map the internal
      `get_field` `FieldError`s → `SessionError::MissingHeaderField { tag }` via per-call `map_err` (NOT a blanket
      `From` — translation, not propagation). `tag` was added to
      `FieldError` (self-describing). No need to match on the `FieldError` variant in the session (4/5 verify fields are
      `String`, whose parse is `Infallible` → only `TagNotFound` reachable). Done: 197 tests green.

    - [x] **7.3b — Converge the inbound chain to one typed result (types only, no behavior change).**
      Make `next_*`, `dispatch_to_app`, and `next_message` return `Result<(), SessionError>` (drop the
      `Box`). Fold the two app-callback errors in:
        - `RejectLogon` → `InboundMsgError` via `#[from]` (the self-contained, propagation case — e.g. a
          `LogonRejected(#[from] RejectLogon)` variant); `on_admin_msg_received(...)?` then auto-converts.
        - `BusinessMsgReject` → a **throwaway** `App(#[from] AppError)` variant so it propagates as a
          `InboundMsgError`
          and the acceptor still `?`s it → connection still drops exactly as today (zero behavior change). The real
          handling (log-and-keep-alive stopgap, then 35=j) lands in 7.3c/7.4; delete this variant then. (Chosen over
          handling `BusinessMsgReject` locally now, to keep 7.3b strictly type-only.)
          Also convert the three `return Err(Box::from(SessionError::...))` in `next_logon` to plain
          `return Err(...)`. Done when: it compiles and 197 tests stay green (behavior identical — bad message still
          drops the connection).

    - [x] **7.3c — Classify + act + control signal (the wire behavior).**
      `next_message` (or a thin wrapper) catches the `InboundMsgError`, applies recoverable-vs-fatal + the pre-logon
      gate (before `logon_received`, everything is fatal → disconnect, since Reject is post-logon), and acts:
      `MissingHeaderField` → `reject_message` (recoverable, keep reading); fatal (BeginStringMismatch, CompIdMismatch,
      SeqNumTooLow, InvalidStateForMsgType, OutOfSessionTime) +
      `LogonRejected` → `generate_logout(reason)` + `disconnect`; `App` → stopgap log + keep the connection (do NOT
      disconnect; real fix is 7.4), and drop the throwaway `App` variant in favor of handling `BusinessMsgReject`
      at `dispatch_to_app`. `next_message` returns a control signal (e.g.
      `ControlFlow`) so the acceptor's two `?` calls become a `match` (keep looping vs break) and all FIX policy stays
      in `Session`. Watch the seq-num double-advance: `verify_seq_number` bumps target seq on its OK path,
      `reject_message` bumps it itself — trace each error's origin relative to the bump. Done when: a bad post-logon
      field → Reject + connection survives + seq advanced once; a fatal error → Logout + disconnect; a rejected logon
      tears down cleanly; an `BusinessMsgReject` no longer drops the connection.

- [x] **7.4 — `send_business_msg_reject` (35=j).**
  Engine-level fallback for `BusinessMsgReject` returned by `on_app_msg_received`. `Session::send_business_msg_reject
  (ref_seq_num, ref_msg_type, &reason)` builds a `35=j` with `372` RefMsgType + `380` BusinessRejectReason (required),
  plus `45` RefSeqNum and `58` Text (reason `Display`), routed through `send_app_message` (35=j is an app message).
  Wired into `dispatch_to_app`'s `Err` arm (log + reply + `?` on SendError, fatal like the Ok path). Boundary: business
  rejects are primarily the app's job (it returns specific rejects in its `Vec<Message>`); this is the catch-all for
  when the app signals a generic error. `BusinessMsgReject` was reshaped to the 380 reason set (the old FieldNotFound/
  IncorrectDataFormat were dropped — the engine already validates those during `from_str`). Done:
  unit-tested (`test_send_business_msg_reject_builds_35j`,
  `test_next_message_app_error_business_rejects_without_teardown`).

- [x] **7.5 — Tests. DONE (2026-09-14, session 2).**
  Unit tests (MockResponder asserting the `3`/`5`/`j` on the wire, RefSeqNum, reason code, seq-num advance). 7.3c
  classifier coverage (recoverable Reject / fatal Logout+disconnect / pre-logon gate / AppError no-teardown). Added the
  dedicated `LogonRejected → Logout+disconnect` case (pre-logon Logout + post-logon graceful arm), a `CompIdMismatch`
  pre-logon **silent**-disconnect case (FIX 4.3 security exception), `SeqNumTooLow`/`SendErr`/`OutOfSessionTime` fatal
  classification, and — closing the "handle every reason right, even the unreached ones" gap — two exhaustive builder
  tests: `test_reject_message_builds_correct_reject_for_every_reason` (all 20 non-garbled `SessionRejectError`
  variants → correct 35=3: 373/371/58/45) and `test_send_business_msg_reject_builds_35j_for_every_reason` (all 9
  `BusinessMsgReject` reasons → correct 35=j: 380/372/45).

- [~] **7.6 — Protocol test harness / simulator. (a) DONE; (b) remaining.**
  (a) **DONE (2026-09-14, session 2):** in-process raw-input harness in `session_tests` drives the real inbound seam
  `acceptor::process_inbound_msg` (widened to `pub(crate)` for test access) — raw SOH string → `Message::from_str` with
  the real `FIX43.xml` dictionary → garbled-drop / Reject (35=3) / dispatch — via `MockResponder`, no socket. Cases:
  garbled (bad BodyLength/CheckSum) → dropped silently; well-formed-but-invalid (undefined tag, tag-not-for-msgtype,
  value-out-of-range, incorrect-data-format, structural) → Reject (35=3) with the right 373 code + RefSeqNum; a valid
  Logon → parses, logs on, confirming Logon (35=A). Dictionary parsed once via a `lazy_static` + clone. (b) **TODO:** a
  small scripted TCP simulator connecting to the acceptor (send/expect scripts) — true end-to-end, catches the
  acceptor/IO seam + `establish_session` (needs a real socket, so unreachable by (a)). QFJ AT `.def` engine is the
  reference; strategy in `session_context/qfj-testing-strategy.md`.

  **Validation-coverage map (three layers), from the session-2 audit:**
    - **L1 parse-time (`from_str`/`from_vec` + dictionary → `SessionRejectError`):** live/emitted variants well covered
      in
      `message.rs` parser tests + the seam covered by the harness. **Response-building for ALL 20 non-garbled reasons is
      now tested** (even unreached ones). All tag-373 `code()` values verified spec-correct.
    - **L2 session-own (`verify_msg`/`verify_seq_number` → `InboundMsgError`):** unit + classifier coverage for
      MissingHeaderField (recoverable), CompIdMismatch/SeqNumTooLow/SendErr/OutOfSessionTime (fatal), pre-logon gate.
    - **L3 app callbacks:** RejectLogon (pre + post-logon), BusinessMsgReject (all 9 reasons), DonotSend.
    - **"Group B" missing parser validations — mostly DONE (2026-09-19):** `InvalidTag`(373=0, unknown tag; was falling
      to `UndefinedTag`/3 — corrected per FIX 4.3 Vol 2 §14a, which uses reason 0; `UndefinedTag`/3 is now documented as
      vestigial), `TagAppearsMoreThanOnce`(373=13, non-group tag duplicated in a section; count tag also covered), and
      unknown-MsgType → `InvalidMessageType`(373=11) are now emitted *and* tested (`msg_test_invalid_tag`,
      `msg_test_duplicate_*`, `msg_test_invalid_msg_type`). Also closed untested audit gaps while here:
      `TagSpecifiedOutOfOrder`(14, both header-in-body and non-trailer-in-trailer), `RepeatingGroupsOutOfOrder`(15),
      `TagSpecifiedWithoutValue`(4). Validation was refactored so every message-layer check goes through a `validate_*`
      helper (`validate_known_msg_type`, `validate_tag_for_duplicacy`, `validate_body_field`, `validate_trailer_field`).
      **Still open:** `NonDataFieldIncludeSOHChar`(17) — the SOH-in-data case, tracked under stretch task 1.6. "Group A"
      variants (Decryption/Signature/CompId/SendingTime/Xml/AppVersion, 373=7/8/9/10/12/18) stay reserved pending their
      features — kept, spec-complete, correctly coded.

**M7 exit criteria**: inbound messages that fail engine-level validation produce the correct FIX response
(Reject/Logout/drop) automatically, the connection survives recoverable errors, and the behavior is covered by automated
tests (unit + a harness per 7.6). Too-high seq (ResendRequest) and full resend remain deferred.

---

## M8 — Initiator support (post-v1)

**Why this exists.** v1 shipped acceptor-only (M5). The initiator *session logic already exists* — `SessionState`
carries `is_initiator`, `next_tick` (`src/session/mod.rs`) already generates a Logon when
`is_initiator && !logon_sent && responder.is_some()`, and `generate_logon` (with `EncryptMethod`/`HeartBtInt`) is shared
with the acceptor path. What's missing is only the **IO half**: nothing dials out and *sets the responder* for an
initiator. For acceptors, `handle_connection` (`src/io/acceptor.rs`) does that from the inbound socket; there is no
symmetric outbound connector, `main.rs:33` filters initiators out of the `SessionMap`, and the `socket_addrs` collection
in `main.rs` would panic on an initiator (`socket_accept_port().unwrap()` on every session).

This milestone is a **mirror of the acceptor path**, reusing `TcpResponder`, `FixMessageReader`, the
`acceptor::process_inbound_msg` seam, and the existing `next_tick` logon logic — no session-layer changes expected.
Reference: QFJ `SocketInitiator` / `IoSessionInitiator`, and `session_context/qfj-session-message-flow.md`.

**Key difference from the acceptor.** The acceptor is *read-first* (waits for the peer's Logon, then derives the
`SessionId` via `reverse_session_id_from_raw`). The initiator is *send-first* and already knows its own `SessionId`
(from config), so there is no `establish_session` / reverse-id lookup — it connects, sets the responder, and lets the
timer send the Logon.

**Invariant to preserve** (same as `handle_connection`, acceptor.rs:48): lock the session only for the dispatch of each
message, **never across the blocking `read_message()`** — otherwise the timer thread can't tick heartbeats while the
reader is parked on a read.

- [x] **8.1 — Config getter + `main.rs` wiring.**
  Exposed `socket_connect_host()` on `SessionConfig` via `#[getset(get = "pub")]` (added `Getters` to the derive). In
  `main.rs`: the connection-type `filter` is gone (the `SessionMap` holds both roles; the timer no-ops responder-less
  initiators via the `responder.is_none()` guard in `next_tick`), and the accept-address set is now collected only from
  *acceptor* sessions (`.filter(connection_type == Acceptor)`) so an initiator in config no longer panics on
  `socket_accept_port().unwrap()`.

- [x] **8.2 — `IoInitiator`: connect + wire responder.**
  New `src/io/initiator.rs`, symmetric to `IoAcceptor`. `IoInitiator` holds the target `Arc<Mutex<Session>>` (a clone of
  the map entry — *not* the whole map, since an initiator never has to identify an inbound connection), the `SessionId`,
  and the connect `SocketAddr`. `start()`: `TcpStream::connect(addr)` → `try_clone` (write half → `TcpResponder`, read
  half → `FixMessageReader`) → lock + `set_responder`. Also extracted the shared pump/teardown into
  `src/io/connection.rs` (`run_connection` + `process_inbound_msg` + `close_connection` + `wire_display`), reused by both
  roles; `acceptor.rs` now calls `run_connection` too. Done: integration test
  `test_initiator_connects_wires_responder_and_sends_logon` (throwaway `TcpListener`, asserts connect + Logon emitted +
  clean teardown).

- [x] **8.3 — Initiator reader loop + logon.**
  Implemented as the shared `connection::run_connection` (see 8.2): loops `read_message()` → lock → `process_inbound_msg`
  → dispatch, locking only per message (never across the blocking read, so the timer can still tick heartbeats), and
  calls `disconnect` on EOF/error. Logon is left to the existing `next_tick` (single source of logon truth) — no second
  path added. Log identity uses the session's own-perspective id, matching the acceptor. *Note:* the "heartbeat exchange
  with `logon_received == true`" half of the original "done when" needs a real responding acceptor — verified live under
  8.6, not in the unit test (which drives `next_tick` by hand).

- [x] **8.4 — Spawn initiator threads in `main.rs`.**
  For each `ConnectionType::Initiator` session, `main.rs` builds an `IoInitiator` (pulling its `Arc` out of the shared
  `SessionMap` via `get`, and its connect addr from `socket_connect_host`/`socket_connect_port`) and spawns a thread
  running `start()`, joined alongside the acceptor handles. Timer covers all sessions. *(The failed-dial panic caveat
  noted here was resolved in 8.5 — `start()` now retries instead of panicking.)*

- [x] **8.5 — Reconnect loop. DONE.**
  `IoInitiator::start()` now wraps the dial + `run_connection` pump in an infinite retry loop: on any disconnect
  (clean EOF/Logout or failed dial) it waits `reconnect_interval` and re-dials, gated by `schedule().is_session_time()`
  (out-of-session it polls on a short interval instead of dialing). No max-retry — matches QFJ, which reconnects
  forever; the loop stops only on session schedule or an explicit stop signal. `state.reset()` on reconnect rides the
  existing `reset_on_*` flags (M5 5.9), so with `reset_on_logon` the re-logon starts clean at seq 1. Added:
    - `reconnect_interval` config key (`Option<u16>` seconds, default 30 via `unwrap_or`) threaded through
      `FixProperties`/`SessionConfig`/`Session`; ignored-for-acceptor validation warning updated.
    - Graceful stop: an `Arc<AtomicBool>` `stop` flag on `IoInitiator` + `interruptible_sleep` (checks the flag between
      sleep slices) so the loop can be halted between connections (a stop while parked in the blocking read still needs
      the socket to close first).
  Done when: killing the peer socket makes the initiator retry and re-logon when the peer returns — covered by
  `test_initiator_reconnects_and_relogons_after_peer_drop` (drop peer → second accept → fresh Logon). Both initiator
  tests now stop+join cleanly (no orphaned threads). 233 tests green. **Verified live (2026-09-26):** with
  `reconnect_interval = 5` in `src/FixCfg.toml`, fix-rs stayed up while the QFJ executor was killed and restarted — it
  retried every 5s and auto re-logged-on at seq 1 (`141=Y` handshake) with heartbeats resuming, no fix-rs restart. See
  `session_context/2026-09-20-initiator.md`.

- [x] **8.6 — Integration test against a QFJ acceptor. DONE (2026-09-20).**
  fix-rs initiator (`BANZAI->EXEC`, `src/FixCfg.toml`) dialed the QFJ `executor` acceptor (FIX.4.3, port 9879) and
  completed a clean session: Logon out at seq 1 (correct CompIDs, `108=5`, no `141`), executor's Logon back at seq 1 →
  `Logon received` + `on_logon`, then `35=0` heartbeats both directions in lockstep (seq 2,3,4,… monotonic, no gaps,
  rejects or logout). Executor run with `ResetOnLogon=Y`. Run logged in `session_context/2026-09-20-initiator.md`.

- [x] **8.7 — Fix `reset_on_logon` timing for initiators.**
  `reset_on_logon` used to reset in `next_logon` (on the *inbound* Logon) for all roles — correct for an acceptor, wrong
  for an initiator (which by then has already sent its Logon at seq 1, so the reset rewound the sender and the next
  heartbeat reused seq 1 → counterparty Logout). Fixed both paths:
    1. **Config reset**: an initiator now resets *before sending* in `next_tick`'s `is_initiator && !logon_sent` branch
       (and sets `reset_sent`); the reset in `next_logon` is gated to acceptors
       (`reset_on_logon && !self.state.is_initiator`).
    2. **Wire-level `141=Y`**: `next_logon` now resets on a `141=Y` request only when we did *not* initiate the reset
       (`reset_requested && !reset_sent`), so an initiator isn't rewound by the acknowledging `141=Y` in the response.
       `generate_logon` stamps `141=Y` during a reset handshake (`reset_sent` for the initiator kicking it off,
       `reset_received` for an acceptor echoing) — QFJ's bilateral handshake (`Session.java:2094`/`2623`).
       `reset_received` is set deterministically per inbound Logon so a stale value can't make an acceptor echo `141=Y`.
  Done: 4 tests — `test_initiator_reset_on_logon_keeps_sender_seq_monotonic` (reset-before-send + no rewind on the
  `141=Y` echo), `test_generate_logon_omits_reset_flag_without_handshake`,
  `test_acceptor_reset_on_logon_resets_without_echoing_flag`, `test_acceptor_echoes_reset_flag_when_requested`. 232 green.
  Also **verified live** (2026-09-20): with `reset_on_logon = true` in `src/FixCfg.toml`, fix-rs restarts reconnect
  cleanly at seq 1 against the QFJ executor (sends `141=Y`, executor resets, no seq-too-low Logout) — the reconnect case
  the executor's persisted store used to break.

**M8 exit criteria**: fix-rs can run as an initiator — dials out, completes the Logon handshake against a real QFJ
acceptor, exchanges heartbeats, and reconnects cleanly after a drop. Acceptor path unchanged; no session-layer logic
duplicated (logon still flows through `next_tick`).
