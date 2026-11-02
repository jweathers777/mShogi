---
id: MSH-86
title: Deliver the authoritative chu shogi native handoff for shared consumers
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-02T21:47:00Z'
labels:
  - m4
  - owner-msh-43
milestone: m-4
dependencies:
  - MSH-82
references:
  - >-
    backlog/tasks/msh-43 -
    Deliver-complete-chu-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/testing/conformance-policy.md
  - docs/m4/chu/profile.md
  - docs/m4/chu/fixtures.md
  - docs/m4/chu/handoff.md
type: task
ordinal: 86000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-43. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

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
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

MSH82 modern-profile refinement2026-11-02: mshogi.chu.jcsa2019.v1 and docs/m4/chu supply complete sourced clauses,250 independently expected rows and direct native/shared handoff. Dependency82 unchanged; Ready is actionable scope, not permission to start before82Done. Original4AC/DoD preserved/unmet. AC4 refinement: no child split is useful now; one coherent native state→complete legality/Lion→Game/history/canonical-record batch with internal independent checkpoints avoids redispatch fragmentation. No extra dependencies. Required native work:12×12/original+once-promoted identities, ordered compound routes/derived discarded captures, capture-based royalty/finite mate, native pre-fourth last-opportunity admission, narrow exhaustion/immediate response/actual draw agreement, strict authoritative replay and exact reversal. Reuse generic conformance infrastructure with a real Chu adapter/corpus; current data validation is0native executed. Require complete LegalAll, all native fixture assertions,7/42 divide, deterministic sequences, meaningful properties/negative mutations/replay/undo, independent code review and applicable native CI. External engines/historical facsimile not gates; product GUI43/shared78 remain separate. No implementation plan prescribed or work started.
<!-- SECTION:NOTES:END -->
