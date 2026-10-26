---
id: MSH-73
title: >-
  Extend shared canonical archives compatibility and lossless admission for
  chess and shogi
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-72
  - MSH-69
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 73000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Reuse M3 canonical corpus; strict old/new format,capability mismatch,concurrent IO/candidate races and each exact resource boundary,including256→257 when that policy is selected.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A versioned tagged desktop envelope preserves complete native game/match payloads,settings,commands/events,observations and original context/modes/limits for chess and shogi; old chess archivev1 remains strictly readable without semantic downgrade.
- [ ] #2 Open/replay/history workers dispatch to authoritative per-game validators,preserve exact prefixes and install fresh runtime execution identities with play stopped; saved observations never resume pending work.
- [ ] #3 Every accepted candidate remains completely saveable under documented per-profile capacity/work policies. Existing MSH61 admission,byte/event/control/observation boundaries,shutdown/identity headroom and stable receipts are preserved,not replaced by truncation or raised constants.
- [ ] #4 Atomic failed/interrupted Save/SaveAs retains prior valid data and dirty state; corrupt/unsupported/resource-limited/stale Open preserves the current live game and unsaved-change protections.
- [ ] #5 Unchanged chess archives including claims,conclusions,advanced observations and unsupported restoration cases plus concrete shogi records pass strict roundtrips. Shogi archive/search limits are evaluated from real record/observation sizes and bounded reconstruction,not assumed universal chess limits.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
