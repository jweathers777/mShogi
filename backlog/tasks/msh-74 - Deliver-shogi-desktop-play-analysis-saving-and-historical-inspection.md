---
id: MSH-74
title: Deliver shogi desktop play analysis saving and historical inspection
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-73
  - MSH-70
  - MSH-68
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 74000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Native/controller and frontend checks plus actual packaged shogi GUI journeys/screenshots; distinguish local game-specific evidence from final three-platform cross-game gate.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The actual desktop can select standard shogi,play either human side against the bundled native engine,and present readable9x9 owner-oriented pieces,hands,turn,history and complete native choices.
- [ ] #2 Drops and optional/mandatory promotions work by click and keyboard with clear cancellation/focus. Declaration eligibility versus committed formal action,outcomes,resignation,recovery and failures have profile-correct meanings.
- [ ] #3 Bounded read-only analysis and live engine work remain separate; cancellation/replacement/profile switching/historical selection preserve exact context and never apply stale or wrong-purpose actions.
- [ ] #4 Complete Save/Open and history inspection preserve hands,profile,commands/events,conclusions and observation provenance. Reopen is stopped; Return to live/Resume and unsaved replacement/quit guards remain explicit.
- [ ] #5 Running native application evidence covers full shogi journeys at supported sizes and shogi capacity/defaults. Unsupported chess Full or timed restoration is rejected clearly,not silently relabeled or downgraded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
