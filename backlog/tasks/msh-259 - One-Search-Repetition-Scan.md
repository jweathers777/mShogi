---
id: MSH-259
title: One Search Repetition Scan
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T19:33:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Measure both drop-game bounds on native repetition fixtures and timed key histories. Preserve null-move boundaries. 2. Share occurrence traversal and retain the scoring and priority of each game. 3. Compare fixed-depth decisions, PV, scores and nodes against 7c15e68. For the game whose bound changes, compare the published blunder suite and test non-inferiority with paired openings at 5 s + 0.1 s, H0 -15 Elo, H1 0, alpha and beta 0.05, at most 1200 pairs. Keep the simpler search only if the strength and blunder results support it. 4. Run perft, differential and repository checks, obtain review, integrate and clean up. All cloud work within US$10.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-259-repetition. Frozen predecessor 7c15e68. Compare whole-game and irreversible-move bounds before selecting the shared drop-game scan. Next: measure bounds, consolidate, and verify all six games.

Branch codex/msh-259-repetition. At Tori fixture TORI-H-CAPTURE-DROP plies 6 and 11, full history found both hand-derived prior positions; the capture/drop cutoff missed both. M1 Pro release microbenchmark, 1M lookups per case (15/16 absent): full 8/32/128/320-ply histories 9/23/70/171 ns per lookup; four-ply bound 3–5 ns. Selected full known history since last null for Shogi and Tori. Shared scan in all six positions; native rules untouched. Bounded Batch frozen build in progress. Next: review, paired blunder and SPRT evidence, fixed-depth results, perft/differentials/checks.

Independent review found the old Tori bound and score in docs/m5/shared-search-core.md. Corrected the guide. Review found no defect in scan bounds, null undo, root thresholds or score priority. Focused capture/drop regression passed on the M1 Pro.
<!-- SECTION:NOTES:END -->
