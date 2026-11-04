---
id: MSH-87
title: Deliver the authoritative tenjiku shogi native handoff for shared consumers
status: To Do
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-04T09:19:00Z'
labels:
  - m4
  - owner-msh-44
milestone: m-4
dependencies:
  - MSH-83
references:
  - >-
    backlog/tasks/msh-44 -
    Deliver-complete-tenjiku-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/testing/conformance-policy.md
  - docs/m4/tenjiku/handoff.md
  - docs/m4/tenjiku/profile.md
  - docs/m4/tenjiku/fixtures.md
type: task
ordinal: 87000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-44. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

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

MSH83 current handoff4November2026: implement the coherent native kernel directly after ratified83, as one bounded assignment with internal state/transitions, LegalAll/effects, history/records and conformance/acceptance checkpoints. No additional child hierarchy is useful under current owner direction; originalAC4 where-useful refinement is satisfied by this direct bounded scope, without changing its text. Two whole immutable profiles: mshogi.tenjiku.adams1999-modern.v1 and mshogi.tenjiku.muller-completed.v1. Preserve actual FD/FE/HT/general differences, explicit shared A/S completions, original/promoted identities and per-piece refusal rights.350corpus rows:204native-boundary/129annotations/17consumer; all native observations pending87, not loader passes. Add thin native corpus adapters reusing shared comparators, independently exercise full action/effect/royal/history/undo/replay/negative/property cases. Native resource accounting must be measured;107 replay-retention/cancel/inspection risks are measurement obligations, not automatic refactor gates or inherited Chu constants.79 consumes actual handoff;44retains engine/console/notation/kanji/playable delivery. No implementation plan or activation is added by83.
<!-- SECTION:NOTES:END -->
