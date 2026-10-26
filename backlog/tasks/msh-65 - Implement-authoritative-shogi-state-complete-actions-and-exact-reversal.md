---
id: MSH-65
title: Implement authoritative shogi state complete actions and exact reversal
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-62
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 65000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Independent movement/transition examples, malformed/local-invariant boundaries, nested exact reversal and material-conservation sequences. No outcome convention is inferred before SHOGI-P1.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A native shogi position owns Sente/Gote,9x9 coordinates,base/promoted pieces,hands,side/counter and declared import provenance without modifying chess position/action contracts.
- [ ] #2 Complete Move and Drop actions derive captures,demotion,ownership and hand changes natively; required/optional promotion and resource/representation rejection are explicit and atomic.
- [ ] #3 Checked state transitions and undo restore every board,hand,side,counter and incremental field exactly; invalid actions/imports leave state unchanged. Local validation is distinguished from historical reachability.
- [ ] #4 SFEN/USI coordinate primitives roundtrip independently expected initial/custom positions and preserve declared counters/history origin; they do not certify prior repetition or match outcomes.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
