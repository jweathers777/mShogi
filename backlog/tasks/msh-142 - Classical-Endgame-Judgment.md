---
id: MSH-142
title: Classical Endgame Judgment
status: Ready
assignee: []
created_date: '2026-11-09T16:23:00Z'
labels:
  - handle:classical-endgame-judgment
milestone: m-5
dependencies:
  - MSH-141
documentation:
  - docs/m5/chess-strength-assessment-report.md
priority: high
ordinal: 147000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical Endgame Judgment. Continue owner-authorized conventional Chess development from MSH-139 (Chess Strength Assessment) and MSH-141 (Chess Frontier Reliability). The assessment exposed a uniquely winning rook-versus-pawn move missed for a draw, optimistic drawn-pawn/rook scores and poor exchanges into pawn endings. Research established specialized endgame evaluation/scaling, implement a coherent useful subset and general independent tests. Native Game remains the sole outcome/history authority; no learned model, external tablebase runtime dependency, rating target or cross-game transfer. Routine local matches share the15minute/task cumulative allowance; substantial studies are reserved for meaningful progress.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Implemented specialized classical endgame knowledge addresses general demonstrated judgment or conversion weaknesses, with documented applicability guards and honest heuristic versus exact distinctions.
- [ ] #2 Independent winning/drawing/losing positions, color and side-to-move counterexamples, promotion and history-sensitive guards validate the change; search/restoration/cancellation and native terminal authority are preserved.
- [ ] #3 Focused tactical/endgame and realistic-time comparisons identify gains and regressions without a rating claim; independent review and required native integration evidence support completion.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
