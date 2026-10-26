---
id: MSH-75
title: Independently verify complete standard-shogi delivery and reconcile MSH41
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T16:47:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-64
  - MSH-66
  - MSH-67
  - MSH-68
  - MSH-70
  - MSH-71
  - MSH-74
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-90
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Fresh independent game integration review and coverage ledger; appropriate native CI plus recorded running application evidence, reusing unchanged chess/M3 evidence.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Independent review traces every original MSH41AC1-8 to accepted profile,authoritative state/legal/reversal/history,independent corpus,console,own native engine,records and actual desktop evidence.
- [ ] #2 Runtime reference comparisons,real interoperability,self-play,strict archive/history/capacity and game-specific application journeys are supported; unavailable/failed checks remain blockers and source audit alone is not conformance.
- [ ] #3 Material findings and coverage gaps are resolved,with actual revisions/commands/environments and honest functional-versus-strength limits. Shared final acceptance stays in MSH46/MSH47.
- [ ] #4 The complete MSH41 delivery record is closed only after all its original obligations and required shared tickets satisfy acceptance and integration. No child completion alone marks the game complete.
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
MSH-90 is the required bounded common-console follow-up to MSH-68/69/70, owned once under MSH-46. Added to this final acceptance gate without removing existing dependencies or changing criteria. MSH-71/72 remain unstarted.
<!-- SECTION:NOTES:END -->
