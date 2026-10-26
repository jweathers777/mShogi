---
id: MSH-64
title: >-
  Establish independent shogi board fixtures and a complete-legal-choice
  reference harness
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-62
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/shogi-verification.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 64000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Run pinned reference sentinels, fixture provenance review and harness failure/mismatch cases; no unsupported runtime or full-match correctness claim.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Pinned cshogi revision6985fe5b516c34072e27c63cf429847f03193522 is admitted by an actual isolated build/run with toolchain/flags/binary hashes and audited source/license provenance; it is not bundled into the product.
- [ ] #2 Independent concrete board-rule fixtures cover promotion/nonpromotion, captures/demotion/hands, nifu, dead drops, uchifuzume, check evasions and pins for both sides, with expected facts established separately from mShogi and reference output.
- [ ] #3 The harness enumerates LegalAll complete move/drop/promotion USI choices, normalizes exact semantics, and refuses to use search-pruned Legal, parser acceptance, is_legal, is_draw or is_game_over as a full rules oracle.
- [ ] #4 Sentinels verify audited semantic gaps; deterministic roots/seeds/history and mismatch-minimization evidence are retained. Outcome-dependent fixtures remain explicitly owned by the later history task after SHOGI-P1.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
