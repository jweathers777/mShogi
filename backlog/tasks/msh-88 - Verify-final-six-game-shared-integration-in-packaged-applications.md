---
id: MSH-88
title: Verify final six-game shared integration in packaged applications
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-41
  - MSH-42
  - MSH-43
  - MSH-44
  - MSH-45
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-76
  - MSH-77
  - MSH-78
  - MSH-79
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 88000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Actual final integrated native CI and three-platform packaged journeys; reuse unchanged M3 evidence only for unaffected behavior,record new game/shared changes independently.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All six real game profiles run through shared library/console/engine/session/record/desktop boundaries with complete game-specific actions/history and honest capability/limit metadata.
- [ ] #2 Game switching,cancellation/stale results,analysis/outcomes/recovery,save/reopen/history and per-profile capacity admission have cross-game regression evidence including preserved chess/M2 modes and old documents.
- [ ] #3 Actual installed or packaged GUI+bundled-engine journeys on Windows,macOS and Linux cover real games and readable keyboard/accessibility interactions; build-only,protocol-only,browser-only or synthetic evidence is not substituted.
- [ ] #4 Exact revisions/packages/environments/commands and failures are durable,normal distributables exclude instrumentation,and ownership/coverage maps reconcile every original MSH46AC1-5 without weakening game scopes.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
