---
id: MSH-173
title: Build the legacy Chu opponent and measure its rule agreement
status: Ready
assignee: []
created_date: '2026-11-14T14:14:00Z'
updated_date: '2026-11-25T07:39:00Z'
labels:
  - handle:frozen-legacy-opponent
milestone: m-5
dependencies: []
references:
  - docs/design.md
  - docs/m5/strength-roadmap.md
  - >-
    backlog/tasks/msh-52 -
    Establish-and-meet-measured-chu-shogi-engine-strength-criteria.md
priority: low
type: task
ordinal: 177000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Frozen Legacy Opponent.

**Problem:** John wants to see how the present Chu engine compares with the original C++ engine at revision `941cd4dc37e6e6210d4f993c96a553753e228b19` (alpha-beta, principal-variation search, iterative deepening, a hash table, quiescence). The runner cannot run that engine. Its printed notation omits the intermediate square of a Lion move (`Move::mVisited`), so text output cannot identify a move. It probably plays rules that differ from the selected JCSA 2019 profile: `ChuShogi.cpp` treats a repetition as a draw, and no Lion-trading rule was found in its source.

**Objective:** A reproducible headless legacy opponent that gives and accepts complete moves, and a measured answer on how often it disagrees with the native rules.

**Approach:** Build from the pinned revision. Keep each compatibility change in a separate patch, and do not change its search, evaluation or rules. Add a small machine interface that reads the move data (origin, ordered landings, promotion), not the printed notation, accepts a move history, and searches by time (the engine has `SetSearchTime`) or depth. Shape the interface for the adapter of MSH-176 (Legacy Runner Adapter); speaking the existing internal execution protocol is one option. Then let the engine play itself for a few games with each move checked by the native Chu rules, and count the disagreements by type. If disagreements would stop a large part of the games, stop and record that for a decision; do not change either rule set. This is a local experiment tool (Level 1), not a product, CI or conformance dependency. Keep build output outside Git.

**Completion:** The opponent builds reproducibly, its interface preserves complete moves, and the rule-agreement result is recorded with a continue or stop decision.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reproducible build from the pinned revision records the compatibility patch, compiler, platform and binary hash; search, evaluation and rules are unchanged; build output is outside Git
- [ ] #2 The interface gives and accepts complete Chu moves, including the intermediate square of Lion moves and promotion, with tests that use independent expected positions for routes with equal endpoints
- [ ] #3 A self-play probe with native checking records games, moves and disagreements by type, and a continue or stop decision
- [ ] #4 The legacy engine is not a product, CI or conformance dependency, and its license and notices are preserved
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ticket revision, 2026-11-15 (requested by John): MSH-173, MSH-174 (Legacy Complete Moves), MSH-175 (Opponent Adjudication Policy) and MSH-176 (Legacy Runner Adapter) became two tasks. This task now holds the build, the complete-move interface (former MSH-174) and a rule-agreement probe, which is the main feasibility risk. MSH-176 holds the comparison. MSH-174 and MSH-175 are archived.

Owner direction, 2026-11-16: deferred. John prefers MSH-185 (Chess Strength Anchor), then MSH-186 (Shogi Strength Harness) and MSH-187 (Shogi Strength Round). Do not select this task before those unless John asks.

Archived 2026-11-25 by John's decision.

Reason: a legacy C++ engine is an external opponent, and John directed no external opponents for games after Xiangqi. The Chu program is complete (MSH-52). The legacy source stays at tag legacy-cpp.
<!-- SECTION:NOTES:END -->
