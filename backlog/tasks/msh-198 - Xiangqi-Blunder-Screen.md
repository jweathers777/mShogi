---
id: MSH-198
title: Xiangqi Blunder Screen
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-18T19:25:00Z'
updated_date: '2026-11-19T16:20:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Rules fixed before classifying (2026-11-19). Data: the 578 scored MSH-197 ladder games (60 s + 1 s) judged by full-strength Pikafish (1,000,000 nodes); mShogi's moves only. Note: the clear-blunder count (26) was already seen in MSH-197; no class or position was examined.
Definitions (MSH-193): p = 1/(1+exp(-cp/600)); mate scores 1/0; mover's view.
- Clear blunder: p_before >= 0.30 and p_after <= p_before - 0.30.
- Mistake (secondary, fixed now because clear blunders are rare here and mean loss is mShogi's main gap): p_before >= 0.20 and loss >= 0.15, not a clear blunder.
Mechanism classes, one per error by priority: (1) time trouble: mShogi's remaining clock < 2,000 ms before the move; (2) repetition rule: the game was lost by a native repetition fault within the next 20 plies; (3) allowed short mate: judge scores the after-position as mate in <= 7 plies for the opponent; (4) refutation captures a Chariot, Horse or Cannon; (5) refutation captures an Advisor, Elephant or Soldier; (6) other.
Fix rule: one fix round only if one class explains >= 30% of clear blunders plus mistakes and maps to a general change; otherwise none.
Checks: rerun mShogi at 10 s per position (fresh process, no history) and re-judge the new move with the same judge settings: avoided if no longer an error by the same definition. Blunder suite: all error positions with played move, judge move and class; held-out split by game (sha256(game id)[0] % 4 == 0).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-19: MSH-197 (Xiangqi Strength Anchor) now measures a reference ladder and judged move quality with the MSH-193 definitions on the ladder games. This screen can reuse that judging; its own work is the mechanism classes, the fix rule and the blunder suite.
<!-- SECTION:NOTES:END -->
