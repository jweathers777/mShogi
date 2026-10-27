---
id: MSH-84
title: Deliver the authoritative xiangqi native handoff for shared consumers
status: To Do
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-27T16:18:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-80
  - MSH-91
  - MSH-92
  - MSH-93
  - MSH-94
  - MSH-95
  - MSH-105
references:
  - >-
    backlog/tasks/msh-45 -
    Deliver-complete-xiangqi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/coverage.md
type: task
ordinal: 84000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-45. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Refine after source/profile decisions into independently executable native slices; require rule corpus,exact reversal,record replay and real native consumer fixtures. The complete game remains open.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 After profile refinement,a native game-owned kernel exposes authoritative state,complete legal actions/derived effects,exact reversal,history/adjudication and canonical record contracts needed by its named shared consumer.
- [ ] #2 Independent rule/transition/history fixtures establish the handoff,including relevant board topology,hands/promotions,compound routes,royal identity and non-chess terminal behavior; no unverified rule convention becomes an interface default.
- [ ] #3 The shared extension consumes stable documented native payloads/capabilities without frontend rules or duplicate ownership; further game-specific engine/console/desktop work remains visible in its complete delivery record.
- [ ] #4 Before Ready,the game owner splits this native scope further where useful and records all child dependencies/verification. No implementation plan is prescribed by this planning task and no dependency on its own aggregate delivery is introduced.
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
MSH-80 refinement2026-10-27: original4AC/DoD retained. Native slices91 fixtures/reference,92 state/actions/reversal/codecs,93 full local legality,94 complete selected history/adjudication/canonical records,95 actual typed-record consumer supply this handoff. None depends on84 or45 final completion, so no aggregate cycle.95 extends the shared record/routing boundary only; engine97 and full session/archive98/99 remain later. All future tickets To Do pending80 ratification/research, not implementation authorization. R-EDITION/R-CYCLE/R-CLASS/R-NOWIN remain named obligations; no unsupported convention or scope reduction.

XIANGQI-P1 two-level3.2A refinement2026-10-27: add required105 boundedexact verifier to complete native handoff.94 owns fastsound/native contract;105 depends94/95 and never84/97, avoiding aggregatecycle. Original4AC/DoD/allprior dependencies preserved; native completeness cannot be claimed from recognizer misses or unknown proofs.
<!-- SECTION:NOTES:END -->
