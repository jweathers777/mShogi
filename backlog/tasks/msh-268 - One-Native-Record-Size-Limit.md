---
id: MSH-268
title: Remove Game Size and Engine Request Limits
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-25T22:13:00Z'
labels:
  - core
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 3800
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John (2026-11-25): no limit on game size or on requests to the engine; the limits are arbitrary and came from early overengineering. Remove: the per-game record byte caps (Tori and Chu 512 KiB; Shogi, Xiangqi and Tenjiku 1 MiB); the 1 MiB saved record wrapper cap (core native.rs, console record_io.rs); the 1 MiB engine request cap (mshogi-engine MAX_REQUEST_BYTES in server.rs and bridge.rs, runner experiments check); the 2 MiB engine response cap (mshogi-engine MAX_RESPONSE_BYTES and every reader or check that uses it); the 16 MiB desktop saved document cap (MAX_ARCHIVE_BYTES); and the 4,096-byte input line cap where it limits a game record or a UCI/USI position command (console MAX_LINE_BYTES). Check the current code for any other cap on the length or bytes of a game and remove it. Keep game rules that end a game (for example the Shogi move-count rule), cancellation, time limits, search limits, and limits that do not depend on game length (diagnostic text, position strings, root metadata).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 No code path refuses a game, a saved file, an engine request or an engine reply because of its length or byte size; the listed constants, capacity errors and the capacity messages in the user interface that become unused are removed; production line counts before and after are recorded.
- [ ] #2 Every test, fixture and test helper that enforces a removed limit is removed or updated. No new test is added only to show that a removed limit is gone.
- [ ] #3 Every document that names a removed limit is updated (for example docs/native-resource-admission.md, docs/desktop and docs/m4 contracts and rules profiles); search docs/ for each removed value and constant. Game rules and their expected test results are unchanged; workspace, frontend and affected-platform checks pass.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory every length/byte/event/ply cap on games, saved files, engine requests and replies (core, engine, console, runner, desktop session, Tauri host, frontend, scripts, tools), including derived constants and reserved headroom.
2. Remove the per-game record byte caps, the envelope cap, the engine request/response caps, the desktop archive cap and the console line cap; remove derived checks and headroom; remove resulting dead errors, resource categories and UI messages.
3. Remove or update tests, fixtures (with their generators) and scripts that enforce removed limits; add no proof-of-removal tests.
4. Update docs that name removed limits.
5. Record kept limits with reasons; record open questions with needs-john.
6. Run fmt, clippy, workspace tests, frontend check and tests, touched Python tests; CI on Linux/macOS/Windows incl. process and persistence suites.
7. Fresh-agent review; fix findings; integrate; finalize; clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (MSH-209, 2026-11-25): may the per-game native record limits become one limit? Recommendation: yes. Use one limit for every game, equal to the room for the payload in the shared envelope (1 MiB minus the envelope header), and check it in one place in the shared record code. Tori and Chu games can then grow about twice as long before a refusal; Shogi, Xiangqi and Tenjiku lose only the header room; no game rule changes. Set the task to Ready when John approves.

Finding, 2026-11-25: both 1 MiB caps are our own choices (MSH-68, MSH-69); no recorded reason for the number. The engine request cap (mshogi-engine MAX_REQUEST_BYTES) is also 1 MiB and a full request carries the record plus overhead, so a record just under 1 MiB can be saved but may be refused by the engine. Revised recommendation, waiting for John: one record limit of 1 MiB for all games; derive the saved wrapper and engine request caps from it plus a fixed overhead allowance.

John, 2026-11-25: I prefer no limit on the game size or the requests to the engine. This replaces the earlier question and recommendation. Accepted consequence: a very large or damaged file or input can use much memory; loading stays cancellable. Not in this task: the 4,096-byte stderr and diagnostic caps. Very long games load slowly until MSH-250, MSH-251 and MSH-227 are done.

John, 2026-11-25: also remove the 2 MiB cap on the engine reply, and any tests that enforce these limits.

John, 2026-11-25: no new tests to show that the removed limits are gone; update the documentation that refers to them.

Checkpoint 1 (claude, branch claude/msh-268-remove-size-limits): core production caps removed (WIP, tests not yet updated, does not build its tests yet): native envelope cap, Shogi/Xiangqi/Tori/Chu/Tenjiku record byte caps, Xiangqi per-submit 4 KiB record headroom check. Kept: per-command 4096-byte parse cap in Tori/Chu/Tenjiku (strict_command; one command, not game length), MAX_COMPACT_ROOT_BYTES (root metadata), Xiangqi MAX_POSITION_BYTES/MAX_FEN_BYTES (position strings), u32/u64 counter overflow checks (integer representation). Production lines before: crates/*/src without test/conformance files 68538; desktop/src + src-tauri without tests/fixtures 3930. Next: core tests; engine MAX_REQUEST_BYTES/MAX_RESPONSE_BYTES (lib, server, bridge, incremental, shared; FailureKind::OversizedResponse); console MAX_LINE_BYTES and record_io; runner experiments (admit_with_match_capacity, request/response checks, max_steps>1024 derived from Tori limit); desktop MAX_ARCHIVE_BYTES, EncodedDocumentBytes/CanonicalRecordBytes, shared.rs envelope+4096 check, capacity fixtures; frontend; scripts; docs.

Checkpoint 2 (claude): Rust caps removed in core (incl. tests, Chu fixture CHU-RECORD-OVERSIZED_RECORD removed, corpus hash/counts 249/239 updated), engine (MAX_REQUEST_BYTES, MAX_RESPONSE_BYTES, FailureKind::OversizedResponse), console (MAX_LINE_BYTES, record_io file/save caps), runner experiments (envelope/request/response checks; max_steps<=1024 derived from the old Tori limit), desktop (MAX_ARCHIVE_BYTES, EncodedDocumentBytes, command envelope+4096 cap, Tori oversized-Open fixture). Workspace builds. Next: frontend TS (oversized_response, encoded_document_bytes texts/tests), scripts, docs, full checks, CI, review.

Checkpoint 3 (claude): implementation complete on branch, commit f416b51. Local checks: fmt, clippy (workspace; desktop-session all features), cargo test --workspace 1157 passed 0 failed, desktop-session --all-features 197 passed, npm run check, vitest 586 passed (Node 24.22.3), python test_gui_smoke+test_macos_verification 141 passed, docs/m4/chu validate + tests passed. Production lines (crates/*/src without test/conformance files): 68538 before, 68247 after; desktop/src + src-tauri without tests/fixtures 3930 -> 3928. CI dispatched: rust process all (45709713563), rust persistence all (45709717987), desktop host ubuntu (45709722103). Next: fresh review, merge main, integrate.

Kept limits and reasons (not about game length): per-command 4096-byte parse bound in Tori/Chu/Tenjiku (one command); MAX_COMPACT_ROOT_BYTES 64 KiB and import source 1024 bytes (root metadata); Xiangqi MAX_POSITION_BYTES/MAX_FEN_BYTES and reference-tool FEN/SFEN caps (position strings); u32/u64 counter overflow checks and identity headroom (integer representation); 4096-byte diagnostic/stderr caps (excluded by John); MovementInspectionQuery 1024 bytes (fixed small query); verification selector 128 bytes; conformance/dev corpus input bounds (fixture files, not games); desktop KEPT_RESULTS 256 (drops the oldest in-memory analysis result, refuses nothing, not saved); 15-second validation budget and bridge watchdogs (time limits); runner config 1 MiB caps (configuration input).

Removed derived limits: runner experiments max_steps <= 1024 (its stated reason was the old Tori record limit); runner admit_with_match_capacity, request/response checks and Chess match root 1 MiB check; Xiangqi 4 KiB record headroom per submit; console record_io read/save caps; desktop SharedDesktopCommand envelope+4096 cap; Chu fixture CHU-RECORD-OVERSIZED_RECORD (corpus 249 rows, 239 native, hash updated); Tori oversized-Open frontend fixture. Pipes: the engine bridge writes stdin and reads stdout/stderr on separate threads, so large messages cannot deadlock; the Tenjiku console test helper now writes stdin from a thread.

Review (fresh general-purpose subagent, given the diff and the task): no real bugs. Chu fixture removal consistent (hash, 249/239 counts, 1,069 = 1,072 - 3 checks). Fixed findings: restored the long Chess record (4,100 observations) engine round trip in mshogi-engine tests/shared.rs; renamed ResourceCategory CanonicalRecordBytes to NativeRootBytes (wire native_root_bytes) because only root admission produces it; removed the unused ResourceUnit::Bytes and TS unit 'bytes'; Chess RecordFailure::Capacity now maps to NativeResource::Counter; fixed stale Tori/Chu Resource::EncodedBytes doc comments; NativeCapacity frontend tests use realistic identity-headroom facts. Not changed: capacity_usage()/CapacityUsage stay as a diagnostic measurement (used by the Tenjiku work example and tests). Runner limits that can depend on game length are kept and sent to John in MSH-273 (Runner Game Length Limits, needs-john). Duplicate MAX_ARCHIVE_OBSERVATIONS/KEPT_RESULTS constant moved to MSH-272 (One Kept Result Count). Accepted consequence noted: the 15-second validation budget can refuse reopening a very long game until MSH-250/251/227.
<!-- SECTION:NOTES:END -->
