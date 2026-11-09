---
id: MSH-147
title: Chess Effectiveness Diagnosis
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-09T20:00:00Z'
updated_date: '2026-11-09T20:14:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Root owns codex/rewrite; no competing Chess worker. Level1 local diagnosis, no platform-sensitive product change. Freeze a twelve-position independent tactical/strategic/endgame/control panel plus full-history failures. Extend the existing diagnostic example with explicit time budgets and structured complete-result/root/PV telemetry, retaining native replay/restoration. Measure sequential 1/5/15/30s runs (~10 minutes), then selected TT/selectivity controls and CPU samples only where results discriminate causes. Compare established classical approaches and evaluation terms; distinguish speed, depth, decision quality and strength. Independent review of instrumentation/findings; concise canonical evidence and priority reconciliation before the next bounded correction. No opponent matches or full platform campaign.
<!-- SECTION:PLAN:END -->
