---
id: MSH-218
title: Shared Protocol Runner
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-20T23:26:00Z'
updated_date: '2026-11-23T18:35:00Z'
labels:
  - runner
  - refactor
milestone: m-6
dependencies: []
priority: medium
ordinal: 225000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: merge the copied Shogi (USI) and Xiangqi (UCI0-9) experiment runners in crates/mshogi-runner into one generic implementation, with a per-game part for only the real differences (protocol dialect, notation, rules types, outcome mapping).

Finding (2026-11-21): with game and protocol names replaced by one placeholder, shogi.rs (1,110 lines) and xiangqi.rs (1,356) differ in 398 lines, so about 85% is the same untimed fresh-child-per-turn runner; shogi_match.rs (504) and xiangqi_match.rs (728) differ in 432 lines. The Chess runner (run.rs) is mostly different and is out of scope unless a real shared part shows up. The Shogi and Xiangqi runner tests repeat the same structure.

Approach: list the real differences, design the boundary from them (A-10; generalize only shared semantics), move the untimed runner first, then the persistent clocked match runner if the boundary holds. External protocol behavior, report formats and recorded-report replay do not change (A-09 allows changing internal APIs).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic runner (and, if it fits, one generic clocked match runner) serves Shogi and Xiangqi; per-game code holds only protocol, notation and rules differences; production line counts before and after are recorded
- [ ] #2 Behavior is unchanged: runner tests pass, written once against the shared code where they were copied, and the recorded Shogi and Xiangqi reports in docs/m4 still replay
- [ ] #3 One review by a fresh agent session of the boundary and the migration
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main 731a8ff. Trace Shogi/Xiangqi runner semantics and preserve recorded report formats and external protocol behavior. Branch codex/msh-218-shared-protocol-runner.
<!-- SECTION:NOTES:END -->
