---
id: MSH-259
title: One Search Repetition Scan
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T19:27:00Z'
labels:
  - search
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7450
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each search position scans its key history in rule_score with its own loop: Shogi looks back over the whole game history at each node (shogi/position.rs), Tori only back to the last capture or drop (tori/position.rs; a recorded approximation that can miss repetitions), Xiangqi and Chu back to the last irreversible move, and Tenjiku checks every earlier key, both sides to move included, with contains (tenjiku/position.rs). The scoring is a game rule; the scan is not. No measurement is recorded for the different bounds of the two drop games.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One shared repetition-history helper in the search core gives each game its earlier occurrences; per-game code keeps only its scoring.
- [ ] #2 Shogi and Tori use one scan bound rule, chosen by measurement; a change in search behavior passes the blunder suite and an SPRT match against the frozen predecessor for each affected game, and each unchanged game shows identical fixed-depth results.
- [ ] #3 Perft and differential tests pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-259-repetition. Frozen predecessor 7c15e68. Compare whole-game and irreversible-move bounds before selecting the shared drop-game scan. Next: measure bounds, consolidate, and verify all six games.
<!-- SECTION:NOTES:END -->
