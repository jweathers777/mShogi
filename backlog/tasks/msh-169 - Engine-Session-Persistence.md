---
id: MSH-169
title: Engine Session Persistence
status: Done
assignee:
  - '@claude-fable'
created_date: '2026-11-14T09:57:00Z'
updated_date: '2026-11-14T14:36:00Z'
labels:
  - handle:engine-session-persistence
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/shared-search-core.md
priority: medium
ordinal: 173000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Engine Session Persistence.

**Problem:** The desktop and the internal execution protocol start one engine process for each search request (`mshogi_engine::serve_once`, `execute_chess` makes a new `SearchCoordinator`). The transposition table and move statistics of MSH-164 (Shared Search Core) therefore start empty at each move on that path. Only the UCI path keeps them for the full game. The independent review of MSH-164 found this.

**Objective:** The engine state lasts for the moves of one game on the desktop path too.

**Approach:** Decide between a persistent engine process for a game session and another way to keep the state, within the single internal execution contract of MSH-116 and the process supervision rules of the desktop. This changes process lifetime, cancellation and shutdown, so it is a Level 2 change. Measure the gain first with the MSH-163 harness (warm table against cold table at equal time); if the gain is small, record that and close without the change.

**Completion:** A measured decision, and if the change is made, the desktop path keeps the engine state between moves with cancellation and shutdown still correct.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The strength difference between a kept table and a new table at each move is measured with the game test and recorded
- [x] #2 If the change is made: two successive desktop searches of one game use the same engine state, and a new game or a loaded game does not reuse it wrongly
- [x] #3 If the change is made: cancellation, replacement and shutdown of a search still stop the engine within the present limits on the targeted platforms
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Server: `mshogi internal-search` serves request lines until end of input with one session. A timed Chess request uses the kept coordinator when the last timed game is an exact prefix of the new game; if not, it starts a new game state. A request without a time budget gets a new coordinator, so deterministic limits stay reproducible.
2. Bridge: one kept child serves successive requests. Cancellation, replacement, failure, timeout and shutdown still kill the child; the next request starts a new one. A kept child that ended before it gave output is replaced once without a failure.
3. Tests: session behavior through `serve`; bridge with the real binary and with the failure fixture; full workspace tests.
4. Measure kept table against cleared table with the MSH-163 game test (a UCI wrapper that sends ucinewgame before each go is the cleared engine).
5. Level 2 by risk (process supervision): local macOS checks plus the smallest sufficient Linux and Windows CI suites for the bridge and console tests. Independent review.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner decision, 2026-11-14: John directed that the change is made in all cases. The measurement of criterion 1 stays as a record of the gain, not as a condition for the change. Worker: @claude-fable, branch claude/msh-169-engine-session, worktree ../mShogi-msh169.

Verification Level 2 (process supervision). Record: docs/m5/evidence/msh169/verification.json. Local macOS: 1180 workspace tests passed at 2274e03a. Targeted process suite passed on ubuntu-24.04, macos-15 and windows-2022 (run 43592161265). Independent review (Claude Opus 5.5, fresh context): no defect against design A-05; one low-to-medium defect (silent restart of a crashed kept child) and the stderr observation corrected; corrections not re-reviewed. Not run: packaged desktop journey; desktop session and Tauri host tests on Linux and Windows. Deferred to the next Level 3 checkpoint.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
The desktop bridge now keeps one engine child between searches, and `mshogi internal-search` serves request lines until end of input. A timed Chess search uses the transposition table and move statistics of the earlier moves when the last timed game is an exact prefix of the new game; a new, loaded or taken-back game starts empty. A search without a time budget always runs on new state, so depth and node limits stay reproducible. Cancellation, replacement of a running search, each failure, the watchdog and shutdown still stop the child; the next search starts a new one. A kept child that ended normally while idle is replaced without a failure; one that fails during a request is UnexpectedExit. The wire types did not change. Why: the one-shot process discarded the 64 MB table of MSH-164 (Shared Search Core) at each move; John directed the change in all cases.

Measured gain (criterion 1): engine 6fef75d1 with the kept table against the same engine with a cleared table at each move, 200 ms per move: +29 -6 =29, Elo +130.7 [+78.3, +189.5], SPRT H1 at 32 pairs. This is UCI self-play, not a measurement through the bridge and not a rating.

Limits: a cancelled search loses the kept state. The idle child holds its table (64 MB). Only Chess keeps state; a game that moves to the shared core adds its state to the same session. Integrated on codex/rewrite as f7ab97f3, af8164d2, 852d31c3. Documents: docs/desktop/internal-execution-v4.md (Engine session), docs/desktop/m3-design.md, docs/m5/shared-search-core.md.
<!-- SECTION:FINAL_SUMMARY:END -->
