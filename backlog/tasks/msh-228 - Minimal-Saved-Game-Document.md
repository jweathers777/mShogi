---
id: MSH-228
title: Minimal Saved Game Document
status: Done
assignee:
  - '@codex'
created_date: '2026-11-22T16:43:00Z'
updated_date: '2026-11-23T08:59:00Z'
labels:
  - desktop
  - refactor
  - simplification
milestone: m-6
dependencies:
  - MSH-226
priority: high
ordinal: 235000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The shared desktop document (game_document.rs) stores a full audit trail: a journal of every change (settings, rewinds, closes), a redundant current record, all branches removed by Take back and Undo, engine observations each with a full copy of the game up to that point, Xiangqi exact evidence, a runtime execution floor and the viewed history position. The UI uses almost none of it after reopening, and it drives most of the capacity reservations and work limits. John (2026-11-22) approved reducing the document to what a game file needs and changing the design accordingly (A-09 'Take back lineage' and the audit-trail wording).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The saved document holds only: game kind, rules profile, start position, human side, play mode, orientation, the game's commands, and optional per-move engine notes (score, line, depth, nodes); opening checks it by one replay of the commands
- [x] #2 Removed: the controls journal, current, initial_settings, retired branches, observation contexts, Xiangqi evidence ledger (or reduced to a short note), retired_execution_floor, selected_event, Close entries, mode, and the reservation and work-limit machinery that only these needed; line counts before and after recorded
- [x] #3 Take back and Undo cut the move list; reopening a file restores the game, sides and settings; docs/design.md is updated to the new rules (A-09 and audit wording)
- [x] #4 Desktop-session and workspace tests pass; fixtures regenerated; frontend tests pass; one fresh-agent review
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. New document: {record: current CanonicalRecord (it already holds the start position and every command), human, settings}. Opening = one native record parse (the core replays and checks it) plus profile and settings checks. No journal, no initial/current pair, no retired branches, no execution floor, no selection, no Close, no mode.
2. Engine results and Xiangqi exact evidence live only in session memory (cap 256); they are not saved. Undo/Take back cut the record with record_prefix.
3. Remove the work models (tori_work, shogi_work, xiangqi_work), MAX_DOCUMENT_WORK, reserve(), usage accounting, with_runtime_floor/with_reserved_close/close_document, and the NativeGame items only they used (undo, resign, engine_failure_stop, work constants, evidence_work/validate_evidence work).
4. Session: observations/evidence/current-run marking move into NativeSession and Draft; snapshot reads them there.
5. Replace the journal-era document tests with a focused test module; adapt session tests; regenerate fixtures; update docs/design.md (A-09, audit wording).
6. Checks, line counts, review, merge.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Decision (2026-11-22, in scope of John's 'cut this fat'): engine results are not saved; they stay in memory for the open session. Saved per-move engine notes can come back later as standard-format comments (adapter tasks).

Checkpoint 2026-11-22 (branch claude/msh-228-minimal-document, pushed): library done and clippy-clean (all features): game_document.rs rewritten (payload {record, human, settings}; with_command/with_rewind/with_settings); per-game *_document.rs are 10-line aliases; old document test dirs removed; new game_document_tests.rs; NativeGame lost work models/limits/undo/engine Stop; engine results and Xiangqi evidence live in NativeSession memory (newest 256 kept, no refusal); closing does not change the document; RuntimeWork lost floor; RuntimeSeed::Document removed; design.md and take-back.md updated. NEXT: fix test compile errors (about 56: tests reading document.observations()/evidence()/retired_branches()/controls()/usage()/with_runtime_floor; shared_chu.rs tests use with_observation/with_selection/floor; shared_tests observation-capacity test uses RuntimeSeed::Document -> delete; shared_tenjiku_tests uses measurement_document helpers -> delete those measurement tests), then examples write_chu_gui_documents.rs and write_tenjiku_gui_fixtures.rs (use with_command and session results), regenerate all fixtures (GUI .mshogi docs for Chu/Tenjiku/Tori/Shogi, frontend JSON), frontend tests, workspace checks, review.

HANDOFF 2026-11-23 (branch claude/msh-228-minimal-document, last commit pushed; task stays In Progress). Net change so far: 31 files, +460 / -6,463 lines; game_document.rs 1,175 -> 333, native_game.rs 1,290 -> 876 (work models, limits, undo, engine Stop, measurement tests removed).
DONE: minimal document {record, human, settings}; with_command/with_rewind/with_settings; engine results and Xiangqi evidence in session memory only (newest 256 kept, no refusal); close never changes the document; RuntimeSeed::Document and RuntimeWork.floor removed; design.md (A-09, moves rule, Take back) and docs/desktop/take-back.md updated; desktop-session lib tests pass with --all-features except every_committed_gui_fixture_document_opens (needs regenerated fixtures); clippy --workspace --all-targets clean; Tenjiku and Chu GUI generators compile (Chu capacity256 document removed: it tested the removed observation limit).
REMAINING (in order):
1. Three mshogi-console tests still expect saved results or a saved history position after reopen: desktop_tori.rs:109 (real_internal_tori_analysis_save_reopen_stopped_then_explicit_analysis_and_resume), tenjiku_internal.rs:299 and :708. Change them the way chu_internal.rs was changed in the last commit: after Open the view is live and observations are empty.
2. Regenerate every committed GUI document under desktop/scripts/fixtures with the generators (all need --features desktop-verification): write_chu_gui_documents, write_tenjiku_gui_fixtures, write_tori_gui_documents, write_document_fixtures (Shogi). Delete desktop/scripts/fixtures/chu-capacity256.mshogi and update chu-native-fixtures.json; then the fixture-open test passes.
3. desktop/scripts/gui-smoke.py (about line 2987, chu_load_fixture('capacity256') and the capacity journey) and test_gui_smoke.py (about line 1528) use the removed Chu capacity fixture: remove that journey.
4. Regenerate frontend fixtures (write_frontend_fixtures, write_tori_frontend_fixtures, write_chu_frontend_fixtures, write_tenjiku_frontend_fixtures) and run npx vitest run and npx tsc --noEmit in desktop/. Expect changes: observations not restored after reopen, reopen opens live, no document change on close.
5. Run fmt, clippy (also --all-features for mshogi-desktop-session), cargo test --workspace --locked --release; fresh-agent review (rules unchanged; check nothing reads saved results); record line counts; merge; then MSH-225 (Chess) and MSH-229 (audit of overbuilt auditing) can start.

Codex continuation: John assigned the pushed Claude handoff. Continue on codex/msh-228-minimal-document; finish console assertions, regenerate fixtures, remove capacity journey, run checks and platform CI, obtain fresh-agent review, integrate.

Checkpoint (Codex): remaining console reopen assertions updated; all GUI documents regenerated with minimal record/human/settings payload; removed Chu capacity fixture and adapted generic-game smoke journeys; frontend regeneration exposed stale Chu/Tenjiku fixture search defaults, now explicit deterministic limits. Fresh review found unbounded current-observation IDs (fixed, 260-result regression passed) and double replay on Open (parser handoff fix in progress). GUI observer tests and frontend checks in progress. Branch codex/msh-228-minimal-document. Remaining: finish single-replay API, complete checks and affected-platform CI, final review and merge.

Checkpoint: single-replay Open implemented through native parsers returning the validated record and replayed game; no rule changes. Fresh-agent review complete and findings fixed (bounded request IDs, single replay, exact historical-analysis source assertions). Frontend: 43 files / 609 tests pass on pinned Node 24.19.0; TypeScript passes; 129 Python GUI observer tests pass. fmt passes. Workspace tests and workspace/all-feature clippy still running. Persistence CI: https://github.com/jweathers777/mShogi/actions/runs/45139767465 (Linux/macOS/Windows), on product commit 6102123; later f92a776 adds only GUI observer assertions. Line counts: game_document.rs 1175 -> 356; native_game.rs 1290 -> 931. Next: resolve any check failures, record final results, merge main and clean task scratch. Branch codex/msh-228-minimal-document.

Verification update: workspace clippy and all-feature desktop-session clippy passed. Debug workspace tests failed in the four already-tracked Chu watchdog tests (MSH-199); evidence added there and uploaded to gs://mshogi-experiments-216675633801/runs/msh-228/workspace-debug.log. Release workspace suite and full all-feature desktop-session suite running. Linux persistence CI and routine Rust/frontend CI passed; macOS/Windows persistence pending. No GCP Batch VMs created; no paid experiment compute.

Persistence CI passed on Linux, macOS and Windows: run 45139767465. Routine Rust/frontend CI passed: run 45139766446. Full all-feature desktop-session and release workspace local checks remain pending. Code is complete and freshly reviewed; no process files or CI workflows changed.

Release follow-up: all six Chu subprocess tests passed in release (8.92 s), confirming MSH-199 debug-only failures. Full all-feature desktop-session suite passed 247 tests before final negative-test correction. Release workspace then exposed experiment consumers still reading payload.initial. Updated console experiment test, runner example and compound-action test to payload.record; shared fixture helper uses native Chu prefix(0) for the concluded royal fixture. Also corrected package-smoke reader and Tori negative tests that previously modified removed fields (false-positive rejection). Fresh reviewer checked these final edits and verified the four tactical roots match the original fixtures; no findings. Full release suite, all-feature session suite and clippy rerunning.

Final validation on 297be4b (same product implementation as persistence CI 6102123): cargo fmt --all -- --check PASS; cargo clippy --workspace --all-targets --locked -- -D warnings PASS; desktop-session all-target/all-feature clippy PASS. cargo test --workspace --locked --release -j 2 -- --test-threads=2 PASS (1227 passed, 11 ignored, no failures); all-feature desktop-session release suite PASS (247 passed). Frontend pinned Node24.19.0: Vitest 609 passed, TypeScript PASS. Python GUI observers 129 passed. Persistence CI Linux/macOS/Windows PASS (45139767465); routine Rust/frontend CI PASS (45139766446). Debug workspace run FAILED in four existing Chu watchdog tests, tracked in MSH-199; the same six-test Chu suite passes in release. No rules/search strength changes, no SPRT needed. Packaged GUI checks were not run (not a release). Fresh-agent reviews completed with all findings fixed. Logs: gs://mshogi-experiments-216675633801/runs/msh-228/ (60-day lifecycle); local Mac with two build jobs/test threads, no GCP Batch compute or VMs, storage-only cost. Final line counts: game_document.rs 1175 -> 356; native_game.rs 1290 -> 931. Integration next, then MSH-225 (Chess Document and Session Fit); MSH-229 remains the audit follow-up.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Integrated into main at 78ad550. Shared documents now hold only native record, human side and settings; Open reuses one validated replay, and Take back/Undo discard removed commands. Engine results and Xiangqi evidence stay in bounded session memory. Fixtures, consumers and design updated; about 6000 net lines removed. Fresh review findings fixed. Release workspace: 1227 passed, 11 ignored; all-feature session: 247 passed; frontend: 609 passed; Python observers: 129 passed; fmt, clippy and TypeScript passed; persistence CI passed on Linux/macOS/Windows. Debug Chu watchdog failures remain tracked in MSH-199. Logs archived under gs://mshogi-experiments-216675633801/runs/msh-228/. Next: MSH-225 (Chess Document and Session Fit), then MSH-229.
<!-- SECTION:FINAL_SUMMARY:END -->
