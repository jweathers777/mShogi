---
id: MSH-217
title: Shared Game Session
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-20T23:25:00Z'
updated_date: '2026-11-21T23:26:00Z'
labels:
  - desktop
  - refactor
milestone: m-6
dependencies: []
priority: medium
ordinal: 224000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: replace the copied per-game modules of crates/mshogi-desktop-session with one generic session implementation behind a per-game trait, so that each game supplies only its genuine differences (rules types, records, presentation, notation).

Finding (2026-11-20 review): the crate has about 25k production lines, more than the search crate for all six games. After replacing game names with one placeholder, shared_chu.rs and shared_tenjiku.rs differ in 149 of about 1,690 lines, chu_document.rs and tenjiku_document.rs in 362 of about 2,070, and the presentation modules in 363 of about 870; Tori and Xiangqi follow the same pattern with more divergence. The copies have drifted: Tenjiku restore and browse use stop-aware calls (to_game_with_stop, record_prefix_with_stop, legal_actions_with_stop) and Chu does not, so a Chu restore cannot be interrupted. The per-game test files repeat the same structure.

Approach: measure the per-game differences first, design the trait from the real differences (A-10; generalize only shared semantics), migrate one game pair (Chu and Tenjiku) to prove the boundary, then Tori, Xiangqi, Shogi and Chess where they fit. Native rules, saved-document format and session behavior do not change (A-09 allows changing internal APIs).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic session/document implementation serves the games; per-game code contains only game-specific behavior, and the production line count before and after is recorded
- [ ] #2 Behavior is unchanged: existing desktop-session tests pass (re-expressed once against the generic code instead of copied per game), saved documents of every game load and save identically, and the desktop host tests pass
- [ ] #3 Stop/cancellation behavior is the same for every game (the Chu restore and browse paths become interruptible like Tenjiku)
- [ ] #4 One review by a fresh agent session of the trait boundary and the migration
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Measured 2026-11-21 (main c0f67d7): crate production files (non-test files, inline test modules included) 24,851 lines. Normalized diffs (game names replaced): shared_chu/shared_tenjiku 149 of 859/832 lines; chu/tenjiku presentation 363 of 469/404; chu/tenjiku document 362 of 958/1112.
Real differences found: (a) stop-aware native calls (Tenjiku only; the Chu core has no to_game_with_stop, record_prefix_with_stop, legal_actions_with_stop); (b) Tenjiku has two profiles (profile read from the record), Chu one; (c) Tenjiku stationary capture (igui) PV label and choice shape; (d) Tenjiku status takes 'has legal actions'; (e) Tenjiku document decode is stricter (canonical control check, stop-aware decode, profile check) and has a measured larger work allowance; (f) close no-op condition differs (Tenjiku also checks paused, phase and browse stop).
Plan:
1. Chu core: add stop-aware to_game_with_stop, record_prefix_with_stop, legal_actions_with_stop (stop checked between replayed events and between candidate actions; no rules change; rules tests unchanged). Existing calls delegate with a never-stop closure.
2. Generic session module (shared_native.rs): NativeSession<G: NativeGame>, draft, runtime seed and work, execute, controller methods (queue candidate, replace, command, save, accept observation, start, poll, verify, close), display_observation. Trait NativeGame supplies only real differences: core types, stop-aware replay calls, presentation and status, PV action label, limits, wrappers (NativeSide, CanonicalRecord, ExecutionResult, SessionOperation, NativeWork/NativeCompleted, session accessors), resource scope and name. Migrate Chu and Tenjiku; Chu gets Tenjiku's stop-aware behavior (AC3).
3. Generic document (game_document.rs) for Chu and Tenjiku with Tenjiku's stop-aware paths; per-game constants (work allowance, profiles). Saved bytes must stay identical: add a test that round-trips the committed fixture documents of each game byte for byte before and after. Stricter decode checks are applied to Chu only if every existing Chu fixture still loads; otherwise keep them per game.
4. Presentation: shared parts into one module where the measured diff allows; game-specific parts stay.
5. Tori and Xiangqi (then Shogi and Chess) where they fit the trait; record where they do not and why.
6. Tests re-expressed once against the generic code; per-game test files keep only game-specific cases.
7. Line count after; checks; fresh-agent review of the trait boundary and migration; merge.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint 2026-11-21 (branch claude/msh-217-shared-session, 31ba49d): (1) Chu core stop-aware to_game_with_stop, record_prefix_with_stop, legal_actions_with_stop, submit_with_stop, with a core test. (2) native_game.rs: trait NativeGame (native types and calls; constants for the real differences) with one macro impl for Chu and Tenjiku (same core API). (3) game_document.rs: ValidatedGameDocument<G> replaces chu_document.rs and tenjiku_document.rs (now aliases plus test helpers); 958+1112 -> 1080+313+35+50 lines. All 322 desktop-session lib tests pass; clippy clean. Behavior notes: Chu document errors now use the Tenjiku forms (stopped flag and 'Chu document operation stopped before acceptance.'; native record capacity is a resource refusal, not 'invalid'). Next: generic session (shared_chu.rs, shared_tenjiku.rs), then presentation, then Tori/Xiangqi fit, byte-identity check of saved documents, tests, review.

Checkpoint 2026-11-22 (8c4afaa): generic session NativeSession<G: SessionGame> (native_session.rs) replaces ChuSession and TenjikuSession; generic snapshot game_snapshot<G>; one macro supplies the Chu and Tenjiku SessionGame impls (names, PV label, board, score meaning). Chu restore, browse and rewind are stop-aware. Lines: shared_chu+shared_tenjiku 859+832 -> 90+37 (+ native_session 1144); presentations 469+404 -> 329+280. Evidence: 322 lib tests pass; clippy (all features) clean; Tenjiku GUI generator output byte-identical between main and branch (21 files, separate main worktree and target); decode+re-encode of all 37 committed Chu and Tenjiku documents byte-identical between main and branch, 0 errors. Found outside scope: MSH-221 (Chu generator assertion fails on main; committed fixtures predate play_mode), MSH-222 (engine adapter copies). Next: Tori and Xiangqi fit to the traits.

Line count (production files; inline test modules included): 24,851 before (main c0f67d7) -> 23,688 after. Chu and Tenjiku modules 4,634 -> 3,463 (generic native_game 321, game_document 1080, native_session 1144; per game: chu_document 35, tenjiku_document 50, shared_chu 185 incl. stop tests, shared_tenjiku 39, presentations 329 + 280).
AC3 evidence: shared::chu::stop_tests - restore and browse (Inspect) refuse at every stop check; a longer history gives more checks during restore. Broken on purpose (Chu replay stop check removed): the per-event test failed; restored.
Tests: the per-game test files are mostly game-specific scenarios (10 test names appear in both games, with different bodies); all run against the generic code, so they were not merged.
Fit of other games (measured, normalized against the old Tenjiku copies): Tori document 1,608 changed lines of 1,168, session 639 of 1,139. The Tori document is a different design, not drift: its saved payload has no selected_event field (deny_unknown_fields), it has its own file worker (ToriDocumentWorker), raw preflight and its own work accounting (1,048,576), and the Tori core has no stop-aware calls or replay work. Xiangqi has its own archive module and format (shared_xiangqi_archive.rs); Shogi and Chess use the original shared runtime. Moving Tori to the generic document changes its saved bytes (selected_event), so it is a saved-format decision for John.

Independent review (fresh subagent, branch diff and task): no high-severity defect. Confirmed equal to main: closing reserve loop of both games, saved bytes (field order and serde attributes), snapshot fields, observation display, take_back_target, session operations; the Chu core change is not a rules change. Fixed: Chu browse-selection work charge stays 2x (SELECTION_WORK_FACTOR; Tenjiku 3x), so saved Chu documents near the 12M limit still load; one NativeGame::game_profile instead of two copies; verify_chu_execution and verify_tenjiku_execution moved to the game modules; NativeSession fields private again. Accepted intended changes (recorded): Chu observations are decoded with the stop-aware typed decoder (schema version and raw response-size checks; compact size still checked in validate); Chu controls are parsed after records and observations, so the first reported error of a multi-fault file can differ; record capacity refusals of both games are typed resource refusals; a stopped session replay says 'Native operation stopped before acceptance.'; nothing in production matches these strings.
<!-- SECTION:NOTES:END -->
