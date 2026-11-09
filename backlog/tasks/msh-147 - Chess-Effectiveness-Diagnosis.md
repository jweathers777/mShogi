---
id: MSH-147
title: Chess Effectiveness Diagnosis
status: Ready
assignee: []
created_date: '2026-11-09T20:00:00Z'
labels:
  - handle:chess-effectiveness-diagnosis
milestone: m-5
dependencies:
  - MSH-146
priority: high
ordinal: 152000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Effectiveness Diagnosis. Owner requests a broad diagnosis before automatically proceeding with MSH-145. Investigate useful search depth, frontier/selectivity, ordering/pruning/transposition effectiveness, native computational overhead and classical evaluation across independent tactical/positional/endgame positions plus preserved full-history failures. Compare increasing thinking time such as1/5/15/30seconds and controlled technique behavior. Rank general limitations before coherent corrections. No learned evaluation, cross-game transfer, per-optimization tournament or arbitrary strength target.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Representative reproducible time-scaling and controlled search evidence records complete move/PV/score/depth/work and histories, distinguishing verified limitations from hypotheses and general behavior from historical-position anecdotes.
- [ ] #2 Independent tactical/endgame expectations, profiling and established conventional approaches distinguish overhead, frontier expansion/selectivity and evaluation shortcomings sufficiently to prioritize impactful engineering work.
- [ ] #3 Concise findings, independent review and risk-appropriate local verification support an actionable conventional improvement order; MSH-145 priority is reconciled and later adaptive reassessment follows a substantial coherent round, not individual techniques.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
