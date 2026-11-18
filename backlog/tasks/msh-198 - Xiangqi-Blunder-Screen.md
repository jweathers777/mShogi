---
id: MSH-198
title: Xiangqi Blunder Screen
status: To Do
assignee: []
created_date: '2026-11-18T19:25:00Z'
labels:
  - handle:xiangqi-blunder-screen
  - xiangqi
  - strength
milestone: m-5
dependencies:
  - MSH-197
references:
  - docs/m5/strength-roadmap.md
ordinal: 205000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Xiangqi Blunder Screen

**Problem.** After the classical rounds, the remaining clear blunders and their mechanisms must be known before John accepts the Xiangqi baseline.

**Objective.** The blunder-screen step of the per-game classical strength program (docs/m5/strength-roadmap.md), as in MSH-193 (Shogi Blunder Screen).

**Approach.** Judge mShogi's moves from the anchor games with the approved stronger judge (a measurement tool only). Fix the blunder rule, classes and fix rule before the results. Run one fix round only if the rule triggers.

**Completion.** A report, a reusable blunder suite with a held-out split, and John's stop-or-continue decision recorded.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The blunder rule, mechanism classes and fix rule are recorded before the results.
- [ ] #2 Clear blunders, rates and mechanism shares are reported; the positions are kept as a suite with a held-out split.
- [ ] #3 Any fix round follows the round rules of the program; otherwise the decision not to fix is recorded.
- [ ] #4 Independent review is recorded and findings are resolved.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
