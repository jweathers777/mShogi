---
id: MSH-70
title: Deliver functional native shogi evaluation search and supervised results
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-67
  - MSH-69
  - MSH-64
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 70000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Independent solvable positions/trees, same board differing histories, zero/finite work,cancellation/replacement and actual internal-process smoke. Evaluate shogi defaults instead of copying chess depth/node constants.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A transparent native shogi evaluator includes hands/promoted pieces and named score perspective/units; bounded functional search uses the authoritative full legal set and history/adjudication,not a second rules approximation.
- [ ] #2 No-action,selected-profile history outcomes and valid declaration choices are consumed before static leaves; cancellation/resource exhaustion is incomplete work,not an invented game result or fallback move.
- [ ] #3 The supervised structured engine returns complete legal shogi choices and real observations under exact context/purpose/session/generation, preserving one execution owner and stale-result safety.
- [ ] #4 Independent small search/terminal fixtures and real child-process play for both human sides establish functional support,with recorded work limits and root restoration. No competitive-strength target or learned model is claimed.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
