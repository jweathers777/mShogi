---
id: MSH-145
title: Pawn Threat and Exchange Judgment
status: Ready
assignee: []
created_date: '2026-11-09T19:38:00Z'
updated_date: '2026-11-09T20:38:00Z'
labels:
  - handle:pawn-threat-and-exchange-judgment
milestone: m-5
dependencies:
  - MSH-144
  - MSH-147
  - MSH-148
documentation:
  - docs/m5/chess-progress-assessment-report.md
priority: high
ordinal: 150000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Pawn Threat and Exchange Judgment. Follow the completed conventional engineering round and MSH-144 assessment. Several actual losses exposed advancing passed-pawn threats and exchanges into difficult pawn endings. In the fresh Goldfish game,44.Ne5 at depth7/-18cp anticipated the bishop exchange but missed the king-interception continuation found after45.Kxe5 at depth11/-227cp. This is a reproduced prediction change, not proof of a uniquely losing move. Investigate the general search/evaluation weakness with preserved full histories and independent fixtures, then select established conventional corrections where supported. Preserve native rules/outcomes/claims/history, bounded work and cancellation; do not special-case games, revive recursive quiet-frontier explosions, introduce learned evaluation or transfer to other games. Routine match work retains15minutes aggregate; no substantial study for an individual improvement.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Establish the causes and generality of passed-pawn, king-interception or exchange-judgment errors from full-history reproductions and independent endgame/tactical expectations, distinguishing verified defects from heuristic limitations and engine opinions.
- [ ] #2 Implement supported general conventional corrections with independently established winning/drawing/losing and negative counterexamples, preserving native authority, draw claims, exact restoration, cancellation and finite search-frontier work.
- [ ] #3 Focused correctness, realistic-time profiling and tactical/endgame comparisons plus independent review and appropriate native integration evidence support the increment; limitations and further opportunities are recorded without an invented strength gain or another per-optimization tournament.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner course-correction after MSH-144: do not start automatically. MSH-147 (Chess Effectiveness Diagnosis) first compares general search/evaluation effectiveness and increasing thinking time, then determines this task priority. Prospective verification follows MSH-146 (Risk-Based Verification): default Level1 local evidence/review for platform-independent search/evaluation; explicitly escalate only actual platform-sensitive risks. Historical completed acceptance remains unchanged.

MSH147 broad diagnosis prioritizes MSH148 Deferred Frontier Work first, then this evaluation increment. Baseline avoided historical Ne5 at local depth8 but still showed deeper pessimism; independent queen-pawn, KNNK and rook endings expose broader conversion/scaling ambiguity. Preserve native authority and nearby winning/losing counterexamples. Broad low-material LMR was experimentally rejected after losing unique winningRe1; no blanket pruning/TT change is authorized by that result.
<!-- SECTION:NOTES:END -->
