---
id: MSH-69
title: Extend shared native context and routing for real chess and shogi consumers
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-67
  - MSH-62
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 69000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Strict serialization/protocol fixtures, actual chess subprocess regression and concrete shogi record roundtrips; purpose/history/generation replacement cases.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A small typed game/profile/version and canonical-record envelope supports actual chess and shogi payloads without universal rules traits,plugin ABI or common board storage.
- [ ] #2 Internal engine/library/console routing preserves complete action versus declaration/claim versus concluded/incomplete/failure results and exact purpose,history,request,session/process-generation and limit acceptance.
- [ ] #3 Existing chess protocol/record behavior has an explicit compatible adapter or documented version rejection; strict payload parsing preserves duplicate-field rejection and profile/schema mismatches cannot dispatch to another game.
- [ ] #4 Roundtrip actual native chess/shogi contexts and reject cross-game/profile,wrong-purpose,stale and unknown envelopes. Unsupported shogi execution capabilities stay unadvertised until its engine consumer lands.
- [ ] #5 This ticket solely owns common dispatch/wire evolution; MSH41 owns shogi rules/search/notation behavior. No dependency on MSH41 or MSH46 final completion is introduced.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
