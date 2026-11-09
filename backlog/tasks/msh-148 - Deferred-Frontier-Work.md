---
id: MSH-148
title: Deferred Frontier Work
status: Ready
assignee: []
created_date: '2026-11-09T20:36:00Z'
labels:
  - handle:deferred-frontier-work
milestone: m-5
dependencies:
  - MSH-147
documentation:
  - docs/m5/chess-effectiveness-diagnosis.md
priority: high
ordinal: 153000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Deferred Frontier Work. MSH147 identifies millions of eager quiescent quiet-candidate projections before captures that usually cut off immediately. Remove demonstrably unused work while preserving native authority, history, claims, existing frontier admission and move ordering. This is a general execution-efficiency increment, not new pruning or a strength claim. Do not adopt the broad endgame-LMR experiment that lost an independently winning rook move.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Defer unnecessary quiet frontier work using the existing search/native boundary, preserving admitted move order, completed results at controlled fixed work, native effects/history/claims and finite cancellation/error restoration.
- [ ] #2 Deterministic cutoff and reached-quiet regressions plus existing varied sacrifice/check-evasion/threat/defensive-resource coverage pass; bounded representative measurements establish actual avoided work and practical cost without claiming playing strength from speed.
- [ ] #3 Level1 local verification and independent review support normal integration; evidence retains baseline/corrected sources and limitations, with later coherent evaluation work and adaptive reassessment recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
