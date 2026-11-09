---
id: MSH-147
title: Chess Effectiveness Diagnosis
status: Done
assignee:
  - '@codex'
created_date: '2026-11-09T20:00:00Z'
updated_date: '2026-11-09T20:39:00Z'
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
- [x] #1 Representative reproducible time-scaling and controlled search evidence records complete move/PV/score/depth/work and histories, distinguishing verified limitations from hypotheses and general behavior from historical-position anecdotes.
- [x] #2 Independent tactical/endgame expectations, profiling and established conventional approaches distinguish overhead, frontier expansion/selectivity and evaluation shortcomings sufficiently to prioritize impactful engineering work.
- [x] #3 Concise findings, independent review and risk-appropriate local verification support an actionable conventional improvement order; MSH-145 priority is reconciled and later adaptive reassessment follows a substantial coherent round, not individual techniques.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Root owns codex/rewrite; no competing Chess worker. Level1 local diagnosis, no platform-sensitive product change. Freeze a twelve-position independent tactical/strategic/endgame/control panel plus full-history failures. Extend the existing diagnostic example with explicit time budgets and structured complete-result/root/PV telemetry, retaining native replay/restoration. Measure sequential 1/5/15/30s runs (~10 minutes), then selected TT/selectivity controls and CPU samples only where results discriminate causes. Compare established classical approaches and evaluation terms; distinguish speed, depth, decision quality and strength. Independent review of instrumentation/findings; concise canonical evidence and priority reconciliation before the next bounded correction. No opponent matches or full platform campaign.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Frozen12-position1/5/15/30s baseline at9501715 completed562.643s. Isolated controls at0a6e8cf retain default result/PV/work across12fixed-budget cases;48conventional tests and workspace clippy pass. Time resolves some score optimism; broad endgameLMR rejected because it loses unique winningRe1 despite greater depth. TT4xcapacity adds no completed ply in4cases. Millions of eager quiet projections plus CPU attribution prioritize MSH148 Deferred Frontier Work, then broader MSH145 conversion/race judgment. No matches or fullplatformcampaign; canonical docs/m5/evidence/msh147/verification.json and concise report ready for independent reconciliation.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed broad, independently reviewed diagnosis on codex/rewrite. Frozen baseline9501715 and diagnostic controls0a6e8cf preserve exact defaults and native history. Twelve positions at1/5/15/30s, isolated/fixed-depth controls, TT comparisons, six independent endgames and three CPU samples distinguish useful-depth limitations, costly eager frontier work and conversion/evaluation gaps. Broad endgame LMR loses a unique winning rook move and is rejected; larger TT gives no extra completed ply in tested cases. Local tests/clippy, native replay/restoration and artifact reconciliation pass; initial formatting/sampler failures retained honestly. Level1 acceptance; no opponent matches, new strength claim or full-platform campaign. Next MSH-148 Deferred Frontier Work, then MSH-145 Pawn Threat and Exchange Judgment and MSH-149 Classical King Exposure before adaptive reassessment.
<!-- SECTION:FINAL_SUMMARY:END -->
