---
id: MSH-153
title: Classical Compensation Calibration
status: To Do
assignee: []
created_date: '2026-11-09T22:45:00Z'
updated_date: '2026-11-10T12:06:00Z'
labels:
  - handle:classical-compensation-calibration
milestone: m-5
dependencies:
  - MSH-152
  - MSH-155
documentation:
  - docs/m5/chess-tactical-forecasting.md
priority: high
ordinal: 158000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical Compensation Calibration.

## Problem
Residual compensation and continuation judgments remain uncertain; narrow score corrections may miss the main strength limitations.

## Objective
Improve general classical decision quality only where broader assessment establishes the need.

## Approach
Use MSH-154 (Chess Adversarial Assessment) and MSH-155 (Chess Time Scaling) to separate evaluation, horizon and selectivity. Held pending owner review of the revised roadmap; valuation work is not yet selected.

## Completion
Existing acceptance criteria remain intact. Any eventual correction needs general decision improvements, counterexamples and independent review; score agreement alone is insufficient.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Separate residual consequence valuation from insufficient horizon and selective admission using a compact predeclared independent control set, complete histories and discriminating bounded experiments; identify decision-quality outcomes and stopping criteria, distinguishing reference opinion from proof.
- [ ] #2 Implement practical general conventional corrections supported by diagnosis, preserving native authority/history, bounded search and cancellation; retain contrary cases and reject changes that improve the motivating score while worsening established decisions.
- [ ] #3 Level1 local correctness, independent tactical/endgame/positional controls, proportionate equal-work/equal-time comparisons and independent review support the increment; report unresolved limits and a justified next measurement recommendation without an unapproved strength campaign.
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
2026-11-10 owner direction: held pending review of MSH-154 (Chess Adversarial Assessment); no implementation authorized by readiness. Historical description retained below:

Handle: Classical Compensation Calibration. MSH-152 established consistent score backup along the Goldfish exchange: matched remaining-depth roots all value White +17, while deeper postexchange search changes that to -83. Strong reference analysis also prefers the original e4 but evaluates the already-poor position much lower; this is not proof that e4 was wrong. The residual gap may combine useful horizon, classical compensation and king/activity valuation. Diagnose this general limitation across full histories and independent materially imbalanced, defensive and endgame controls before choosing further conventional changes. Preserve prior evidence, native authority/history/claims, restoration and bounded cancellation. No preselected positional bonus, learned evaluation, cross-game transfer or automatic tournament. Substantial matched-version strength studies require owner approval.

MSH-155 (Chess Time Scaling) completed bounded diagnosis and returns for owner review. See docs/m5/chess-time-scaling.md and docs/m5/strength-roadmap.md. Remains held; no substantial implementation authorized.
<!-- SECTION:NOTES:END -->
