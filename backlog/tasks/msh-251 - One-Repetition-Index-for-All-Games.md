---
id: MSH-251
title: One Repetition Index for All Games
status: Ready
assignee: []
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T11:55:00Z'
labels:
  - core
  - simplification
  - performance
  - model:large
  - effort:high
milestone: m-6
dependencies:
  - MSH-250
priority: medium
ordinal: 6800
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each native game finds earlier occurrences of a position in its own way: Chess keeps a hash-map count (chess/game.rs), Tori a digest index (tori/game.rs, MSH-208, the only measured one), while Shogi (shogi/game.rs occurrences) and Xiangqi (xiangqi/game.rs occurrences) scan all earlier identities after each command, Chu scans all earlier keys (chu/game.rs current_occurrences), and Tenjiku scans all earlier keys for each candidate action in legal_actions_with_stop and in bare-royal adjudication (tenjiku/game.rs). The repetition rules differ by game; the lookup does not need to differ. Do this after MSH-250 (Token-Only Native Request Context), which changes the same functions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic repetition index in mshogi-core (position key to occurrence indices, with append and truncate) serves all six games; per-game code keeps only its key type and its rule.
- [ ] #2 No full-history scan per command or per candidate action remains in Shogi, Xiangqi, Chu or Tenjiku.
- [ ] #3 Rules, perft and record tests pass unchanged; replay and legal-action time at full length are measured before and after.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measurement tools that exist (MSH-209): mshogi-core examples record_replay, resource_admission and tenjiku_rules_work.

MSH-250 (Token-Only Native Request Context), 2026-11-26: did not change any occurrences or repetition function. Chu replay at 1000 events is still about 615 us per event after MSH-250 (was 674); its repetition scans dominate. Baseline for this task: target/release/examples/resource_admission GAME N (shogi and xiangqi added), numbers in MSH-250 notes.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.
<!-- SECTION:NOTES:END -->
