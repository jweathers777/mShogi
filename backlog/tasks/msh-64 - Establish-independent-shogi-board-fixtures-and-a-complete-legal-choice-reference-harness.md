---
id: MSH-64
title: >-
  Establish independent shogi board fixtures and a complete-legal-choice
  reference harness
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T09:49:00Z'
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
- [x] #1 Pinned cshogi revision6985fe5b516c34072e27c63cf429847f03193522 is admitted by an actual isolated build/run with toolchain/flags/binary hashes and audited source/license provenance; it is not bundled into the product.
- [x] #2 Independent concrete board-rule fixtures cover promotion/nonpromotion, captures/demotion/hands, nifu, dead drops, uchifuzume, check evasions and pins for both sides, with expected facts established separately from mShogi and reference output.
- [x] #3 The harness enumerates LegalAll complete move/drop/promotion USI choices, normalizes exact semantics, and refuses to use search-pruned Legal, parser acceptance, is_legal, is_draw or is_game_over as a full rules oracle.
- [x] #4 Sentinels verify audited semantic gaps; deterministic roots/seeds/history and mismatch-minimization evidence are retained. Outcome-dependent fixtures remain explicitly owned by the later history task after SHOGI-P1.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Build the pinned cshogi source in an isolated development environment and retain provenance. Instantiate independently reasoned board fixtures and LegalAll-only harness, run admission sentinels plus useful failure/mismatch diagnostics, obtain independent fixture/harness review, and integrate evidence. History and outcome cases remain MSH-67.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-26: Authorized bounded batch MSH-63–66. Primary integrator Codex, branch codex/shogi-board, existing next-task worktree. Owner explicitly approved option A SHOGI-P1 mshogi.shogi.csa36.v1 against integration 6db8500e92129b95e502e7b4d5c4a26072e38652.

Actual isolated unmodified pinned cshogi build and runtime admitted on macOS arm64, using temporary LLVM19 after recorded AppleClang failures. docs/m4/reference retains source/license/compiler/flags/dependency/binary/rule-byte provenance. 101 independently reasoned board fixtures PASS;4 API-gap sentinels and4 injected failure/mismatch prefix-minimization demonstrations PASS. Staged-dirty-source admission correctly rejected. Fresh independent AI fixture/source review and independent actual admission/rerun PASS, docs/m4/reference-review.md; no human review claimed. Python compilation/diff checks PASS. Native differential comparisons belong66; history/outcomes/protocol/archive/UI inventory cases remain their existing owners. Native code unchanged fromce42d06; reuse three-platform native CI40300895588.
<!-- SECTION:NOTES:END -->
