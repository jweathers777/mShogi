---
id: MSH-247
title: Remove Test-Only Native Redo
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T17:29:00Z'
labels:
  - cleanup
  - core
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 3400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Tori, Chu and Tenjiku have redo tokens at two levels: Game::redo with GameRedo and a branch counter (tori/game.rs, chu/game.rs, tenjiku/game.rs), and LocalPosition::redo with LocalRedo (tori/legal.rs with transition::Redo, chu/legal.rs, tenjiku/legal.rs). Only mshogi-core tests call redo (checked 2026-11-25); the console, desktop, engine and runner use undo or prefix replay. The redo paths and the state that exists only for them are unused code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Game and LocalPosition redo, GameRedo, LocalRedo, transition::Redo and the branch counters that exist only to retire redo tokens are removed from Tori, Chu and Tenjiku; undo returns no redo token.
- [ ] #2 Tests that only check redo are removed; undo, rules, perft and record tests pass with unchanged expected results.
- [ ] #3 Production line counts before and after are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Check callers: only mshogi-core tests and the conformance fixtures call redo. No open task plans a use.
2. Delete Game::redo, GameRedo, LocalRedo, transition::Redo, the three branch counters at both levels, the effects fields of local undo tokens that only redo read. Keep node, next_node and revision, which undo and stale-token checks use.
3. Delete redo parts of tests. Where a test continued from the redone state, submit the action again. Keep all undo, rules, perft and record checks.
4. Delete the redo expectations from the Tori and Chu conformance fixtures, update ownership data, hashes, assertion counts and docs.
5. Run checks, count production lines, get one fresh review, integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

Handoff: claimed by @claude on branch claude/msh-247-remove-redo. No code change yet.

Caller check: only mshogi-core tests and the Tori/Chu conformance adapters called redo. The desktop Edit menu redo (src-tauri main.rs, PredefinedMenuItem) is the operating system text redo and stays. No open task plans a use.

Removed: Game::redo, GameRedo, LocalRedo, transition::Redo, redo, branch counters, and the effects field of the three local undo tokens (only redo read it). Kept node, next_node and revision, which undo and stale-token checks use.

Tests: redo parts removed. Where a test continued from the redone state, it submits the action again. Conformance data: 42 Tori and 4 Chu redo entries removed; hashes and totals updated (Tori local 483/244 to 403/202, Tori game 713 to 591, Chu complete 1076 to 1072). The hand-derived value gote at ply 7 of TORI-H-REPLAY-UNDO stays, as resubmit_at7_repetition_side. No other expected result changed.

Production lines (8 changed mshogi-core files: tori game/legal/transition/mod, chu game/legal, tenjiku game/legal): 6284 before, 5935 after. All non-test crates/*/src files: 71653 before, 71301 after.

Review: one fresh-session review (the change edits pinned conformance data and undo tests in the rules crate). 3 findings, all fixed: the dropped repetition-side expectation and two stale documentation statements.
<!-- SECTION:NOTES:END -->
