---
id: MSH-64
title: >-
  Establish independent shogi board fixtures and a complete-legal-choice
  reference harness
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T10:23:00Z'
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
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Build the pinned cshogi source in an isolated development environment and retain provenance. Instantiate independently reasoned board fixtures and LegalAll-only harness, run admission sentinels plus useful failure/mismatch diagnostics, obtain independent fixture/harness review, and integrate evidence. History and outcome cases remain MSH-67.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-26: Authorized bounded batch MSH-63–66. Primary integrator Codex, branch codex/shogi-board, existing next-task worktree. Owner explicitly approved option A SHOGI-P1 mshogi.shogi.csa36.v1 against integration 6db8500e92129b95e502e7b4d5c4a26072e38652.

Actual isolated unmodified pinned cshogi build and runtime admitted on macOS arm64, using temporary LLVM19 after recorded AppleClang failures. docs/m4/reference retains source/license/compiler/flags/dependency/binary/rule-byte provenance. 101 independently reasoned board fixtures PASS;4 API-gap sentinels and4 injected failure/mismatch prefix-minimization demonstrations PASS. Staged-dirty-source admission correctly rejected. Fresh independent AI fixture/source review and independent actual admission/rerun PASS, docs/m4/reference-review.md; no human review claimed. Python compilation/diff checks PASS. Native differential comparisons belong66; history/outcomes/protocol/archive/UI inventory cases remain their existing owners. Native code unchanged fromce42d06; reuse three-platform native CI40300895588.

Reviewed source368c613 integrated and pushed via codex/rewrite444107cdadfda873d993828942f3018b45d59531. Complete execution/provenance docs/m4/reference/README.md; finalfixtureSHA579842bc0be0ab74567e1501100ef589be160c140c9371373768dc9fc43b6738. Native source unchanged from passedce42d06CI; normal integration CI triggers and final66CI will exercise corpus natively.

MSH66 longer differential/perft execution exposed two F022 bishop-evasion roots already attacking the nonmoving king (BB4d→WK1a), outside cshogi incremental-check/king-capture invariants. Original101 direct fixture pass remains a true observed result but did not establish safe replay domain. Reopened64 for focused correction: retain v1 fixtures/reports, independently reviewed v2 roots WK1b/mirrorBK9h with unchanged promotion expectations, audited geometric reference-domain guard, all-root audit, original-root refusal sentinel and fresh follow-up review/reintegration. Native behavior must not be changed to match unsupported reference output.

Focused correction complete: preserved original v1 fixture/report/manifest; two bishop-evasion diagrams version2 relocate far king to unattacked square with unchanged independently reviewed promotion facts. Audited raw geometric reference-root admission now rejects prior nonmoving-king check and unsupported local/counter domains before enumeration/push;13 actual rejection sentinels PASS. Independent AI follow-up review PASS docs/m4/reference-domain-review.md; independent all-root geometry audit and actual current101/13domain/4gap/4diagnostic rerun byte-identical. Sourcepin/compiledbinary unchanged; original direct-pass evidence remains historical, not a safe-replay-domain certificate.

Correction source2c25d90 integrated/pushed via codex/rewrite9f9f609c37320863e0fcf803c0fb5d1e834bf584. All current fixture/domain/gap/diagnostic checks PASS and independent domain review resolved; full history of the initial admission and late finding preserved. Native source unchanged from passedce42d06CI; final66source/nativeCI covers current fixtures.

Final current corpus/harness also covered by MSH66 integration6f6301e: native CI40305615224 and desktop regression CI40305615506 PASS allthree platforms. Current101corrected board cases plus68completenesscases have native fixture/differential/perft evidence; reference execution itself is localmacOS only. V1original evidence preserved and explicitly superseded for unsafe-root admission.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Admitted exact isolated cshogi6985fe5 build with compiler/source/license/binary provenance and complete LegalAll-only harness. Current101 reviewed board cases,13 domain rejections,4 API-gap sentinels and4 injected mismatch/failure diagnostics PASS; independent AI reviewer reran them. Longer66 checks exposed two unsupported v1 roots; preserved original evidence, corrected v2 diagrams through independent geometry and added audited pre-replay domain guards. Correction integrated/pushed9f9f609. No native/game/history/UI delivery claim by this fixture ticket.
<!-- SECTION:FINAL_SUMMARY:END -->
