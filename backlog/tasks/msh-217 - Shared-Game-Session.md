---
id: MSH-217
title: Shared Game Session
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-20T23:25:00Z'
updated_date: '2026-11-21T22:42:00Z'
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
