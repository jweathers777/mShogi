---
id: MSH-152
title: Classical Tactical Forecasting
status: Ready
assignee: []
created_date: '2026-11-09T22:12:00Z'
labels:
  - handle:classical-tactical-forecasting
milestone: m-5
dependencies:
  - MSH-151
documentation:
  - docs/m5/chess-coherent-round-assessment-report.md
priority: high
ordinal: 157000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical Tactical Forecasting. MSH-151 fresh games remain near the prior conditional strength range despite accepted cost/calibration improvements. Repeated losses show completed PVs already anticipating exchanges while later compensation/conversion scores deteriorate, plus quiet rook invasion and king containment (including a pinned bishop after Rb8). Investigate general forecast instability using complete recorded histories and independent tactical/positional controls before selecting conventional corrections. A score swing is not proof of a first losing move, missed legal capture or exact tactical refutation; three queried late endgames preserved their already-drawn/lost board WDL. Focus on useful search and decision quality, including quiet continuations, selective admission and valuation interactions, without overfitting these games. Preserve native rules/history/claims/restoration/cancellation and finite frontier work. No learned evaluation or cross-game transfer; no per-optimization tournament.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Establish the cause and generality of significant forecast instability with complete histories, completed-iteration/PV telemetry, modest controlled work/time comparisons and independently established tactical/defensive counterexamples; distinguish evaluation, frontier/selectivity and useful-depth limits from hypotheses.
- [ ] #2 Implement supported general established conventional corrections where practical, preserving native authority, history/claims, exact restoration, cancellation and bounded work; do not conceal defects with limits, indiscriminate pruning removal or a recursive quiet-search explosion.
- [ ] #3 Level1 focused local correctness, independent varied tactical/positional/endgame negatives, proportionate performance/decision comparisons and independent review support the increment; report limitations without a claimed rating gain or another small-change tournament.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
