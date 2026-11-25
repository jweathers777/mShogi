---
id: MSH-247
title: Remove Test-Only Native Redo
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T17:13:00Z'
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

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

Handoff: claimed by @claude on branch claude/msh-247-remove-redo. No code change yet.
<!-- SECTION:NOTES:END -->
