---
id: MSH-76
title: Extend shared contracts for real xiangqi intersection-board consumers
status: To Do
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-27T15:04:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-84
  - MSH-95
  - MSH-98
  - MSH-99
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/xiangqi/handoff.md
type: task
ordinal: 76000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Before activation refine against the accepted game profile and concrete native handoff; verify codecs,context races,records and shared UI behavior with independently expected real-game fixtures. No dependency on the aggregate game delivery or MSH46 final completion.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The real game-owned native handoff is consumed through explicit typed profile/record/session/engine dispatch and supported capabilities; existing chess/shogi contexts remain compatible and stale/cancelled results cannot cross profiles.
- [ ] #2 Shared mechanisms accommodate 9x10 intersections,palace/river markers,Red/Black sides and native complete history/action/outcome payloads through native metadata and complete choices,with no frontend legality or mandatory common board representation.
- [ ] #3 Canonical envelope/history/admission supports the concrete game payload and documented resource policy without losing prior accepted data; unsupported profiles/conversions remain explicit.
- [ ] #4 Real authoritative game-generated fixtures and interaction tests establish the extension; synthetic M3 fixtures alone do not prove real-game support. Required cross-game regression and ownership boundaries are recorded.
- [ ] #5 The semantic presentation contract can feed the accepted common dark shell with stable document/live/board/sidebar slots and native intersection/river/palace/piece metadata. Native read-only enemy inspection binds exact selected history; no TypeScript movement rules, required hands/promotion/declaration, scrolling or zooming assumption. Actual product rendering and same-size comparison are verified by MSH-101/102, avoiding a prerequisite cycle.
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
MSH-80 refinement2026-10-27: original4AC/DoD and prior dependencies retained. Shared ownership is95 typed native routing,98 authoritative session/semantic intersection presentation/native read-only inspection,99 archive/admission. This aggregate accepts those real boundaries; it does not own another rules kernel, engine or finished Xiangqi UI. Product rendering/journeys are101/102 after76. Native84 never depends on76. Accepted MSH-74 shell, full-board visibility and MSH-61 saveability remain product requirements.
<!-- SECTION:NOTES:END -->
