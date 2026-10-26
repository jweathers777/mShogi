---
id: MSH-68
title: Deliver shogi console notation and canonical record adapters
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
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 68000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Real console process tests, malformed/oversized input atomicity, notation/record fixtures and preserved chess command regression evidence.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The standalone console explicitly selects shogi and can inspect position,hands,turn,complete actions/history/outcomes,apply/undo and run legal/perft/divide diagnostics; existing chess shell/UCI behavior remains compatible.
- [ ] #2 SFEN and USI move/drop/promotion text plus the selected human-readable notation preserve complete native choices. Any additional import/export adapter documents losses and rejects unsupported semantics instead of becoming the canonical record.
- [ ] #3 Canonical game/match load/save/replay retains all applicable profile/policy/event semantics and bounded hostile-input behavior; diagnostic stdout never contaminates machine protocol output.
- [ ] #4 Actual console subprocess journeys cover drops,optional/mandatory promotion,rejection,history,conclusions and record roundtrips with independently expected outputs.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
