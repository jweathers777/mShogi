---
id: MSH-228
title: Minimal Saved Game Document
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-22T16:43:00Z'
updated_date: '2026-11-23T08:25:00Z'
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
- [ ] #1 The saved document holds only: game kind, rules profile, start position, human side, play mode, orientation, the game's commands, and optional per-move engine notes (score, line, depth, nodes); opening checks it by one replay of the commands
- [ ] #2 Removed: the controls journal, current, initial_settings, retired branches, observation contexts, Xiangqi evidence ledger (or reduced to a short note), retired_execution_floor, selected_event, Close entries, mode, and the reservation and work-limit machinery that only these needed; line counts before and after recorded
- [ ] #3 Take back and Undo cut the move list; reopening a file restores the game, sides and settings; docs/design.md is updated to the new rules (A-09 and audit wording)
- [ ] #4 Desktop-session and workspace tests pass; fixtures regenerated; frontend tests pass; one fresh-agent review
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
<!-- SECTION:NOTES:END -->
