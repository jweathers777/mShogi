---
id: MSH-222
title: Shared Engine Adapters
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-21T23:10:00Z'
updated_date: '2026-11-26T02:51:00Z'
labels:
  - engine
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The per-game adapter modules in crates/mshogi-engine (shogi.rs, xiangqi.rs, tori.rs, chu.rs, tenjiku.rs; about 2,060 lines) repeat result, telemetry, validation and limits code; chu.rs and tenjiku.rs have the same structure. Replace them with one generic implementation behind a per-game trait, as MSH-217 (Shared Game Session) did for the desktop session. Start with Chu and Tenjiku, then include every other game whose adapter has the same structure. Protocols may change (A-09); update callers and fixtures.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic adapter serves Chu, Tenjiku and every other game with the same adapter structure; per-game code holds only game-specific behavior; production line counts before and after are recorded.
- [ ] #2 Engine protocol tests, desktop-session tests and the workspace checks pass; changed fixtures are regenerated.
- [ ] #3 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Measured 2026-11-25 (main c7ad8ba): the per-game adapter files are 1,760 lines (shogi 475, xiangqi 384, tori 262, chu 317, tenjiku 322; inline tests included), plus 250 lines of per-game execute functions in server.rs. Chu and Tenjiku differ only in names and two stop-aware calls; Tori has the same structure (it lacks the identity check in score_label and validate_with_stop, and adds a complete-depth check). Shogi and Xiangqi have the same result and telemetry shape with real differences: a choice type with a formal command (declaration, count claim), timed fallback (the shared core gives it to both, but only the Shogi adapter accepts it), and for Xiangqi an unresolved-adjudication result and an echoed no-win assessment.
Production lines before: mshogi-engine 4,921; mshogi-desktop-session 12,732; mshogi-runner 8,971; mshogi-console 4,813 (src files, without *tests.rs).
1. New module adapter.rs: trait EngineGame (native types, search identities and bounds, record queries, command and submit for line replay, search call) and generic EngineResult<G> and EngineTelemetry<G> with one score_label, one matches and one validate. Markers in mshogi_engine::game; ChuResult and similar names stay as type aliases.
2. One rule set for all five games: Choice/Finished/Incomplete/Unresolved shapes, timed fallback, complete-depth check (from Tori), node-limit check, choice never wins at its own root. Unresolved is a StopReason value; games without pending adjudication use an empty Pending type. Choice field is choice for every game; the redundant Xiangqi no_win_assessment echo is removed (the record holds it).
3. validate replays the reported line only; matches owns identity, limit and shape checks (every caller calls matches first).
4. server.rs: one generic execute for the five games; retained engines in one slot; incremental adapters use the trait for limits. ExecutionContext::valid_limits uses the trait for limits and analysis turn.
5. Update callers (desktop-session, runner, console, tests, examples), fixtures with their generators, and documents.
6. Checks; fixed-search baseline unchanged; line counts after; fresh-agent review; integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint 2026-11-26 (branch claude/msh-222-engine-adapters): adapter.rs (trait EngineGame, EngineResult<G>, EngineTelemetry<G>, one matches, validate and score_label) and game.rs (five impls) replace shogi.rs, xiangqi.rs, tori.rs, chu.rs, tenjiku.rs; server.rs has one execute path and one engine slot. Callers in desktop-session, runner, console and tests updated; engine tests pass (fixed-search baseline unchanged). Next: workspace and desktop checks, fixture regeneration, docs, line counts, review.

Checkpoint 2026-11-26 (1e95c95): protocol version 7; choice field in every game; Chu, Tori and Tenjiku frontend fixtures regenerated (diff is only action -> choice, 11 fields: search output unchanged); scripts and docs updated. Checks: fmt, clippy (workspace and desktop-session all features), workspace tests 1152 passed and 1 contract test fixed after (protocol literal), desktop-session all features 197 passed, frontend check and 586 tests passed, script Python tests passed, src-tauri lock current. CI process suite (run 45720363450) passed on Linux, macOS and Windows. Next: fresh-agent review (first attempt stopped by a usage limit), fixes, merge main, final checks, integrate.

Review (fresh subagent, 2026-11-26, given the diff and the task): no real defects. It checked each unified admission rule against the shared core and the root adapters (complete-depth, node-limit, nodes >= depth, root-win, Shogi declaration, Xiangqi claim), the engine slot and game switching in server.rs, and the callers (matches is always called before validate). Fixed from its suggestions: ExecutionObservation::matches uses one for_game dispatch; server.rs has one new_engine and one extends helper and kept() is now session_engine(); Chu, Tenjiku and Tori share a board_action_game macro; runner valid_work reads EngineGame bounds; new adapter tests for the unified rules (node limit, complete depth, root win, Shogi declaration and Xiangqi claim shapes, Xiangqi timed fallback, Unresolved status); the Tenjiku forgery test asserts the observation check; protocol 7 in the engine-session doc. Not done here: one generic runner admit function (MSH-263 scope, noted there); one SearchSession for Shogi and Xiangqi (MSH-256, noted there). Visible text changes, accepted as consistency fixes: Xiangqi stop label Stopped -> Search stopped; Xiangqi distance labels use the singular and 'Win at the analyzed position'; unknown Tori identities get no score label (as the other games).
<!-- SECTION:NOTES:END -->
