---
id: MSH-66
title: Complete shogi legality diagnostics and independent conformance comparisons
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
  - MSH-64
  - MSH-65
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 66000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Board fixture corpus, independent expected totals where established, bounded differential runs with seeds/prefixes and full undo checks. No invented perft counts.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The native library enumerates every complete legal board choice including legal nonpromotions and drop categories, and validates king safety,nifu,dead drops and pawn-drop mate with legal defenses.
- [x] #2 Legal-action acceptance is atomic; attacks,pseudo-legal candidates and legal choices are separate. King capture is absent and no chess castling/en-passant/pawn-double-step rule leaks in.
- [x] #3 Perft/divide and deterministic sequence diagnostics compare complete LegalAll sets/transitions against the admitted reference while independently sourced expectations settle disagreements.
- [x] #4 Both sides and promotion/check-evasion edge cases have retained fixtures; apply/undo and exact-state/repetition identity restoration are verified without treating reference history helpers as outcomes.
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
Prerequisites64 and65 are integrated/Done. Add distinct geometric attacks, structural candidate enumeration, full king-safe legal choices and checked public apply/undo. Implement pawn-drop-mate through exhaustive legal defenses without suppressing optional promotions. Add counter-independent repetition identity and perft/divide diagnostics, JSONL verification-only probe, independent corpus tests and deterministic exact-restoration checks. Run admitted-reference comparisons with retained roots/seeds/prefixes and perft expectations only where independently derived, broaden existing regressions/native CI, obtain fresh independent review, integrate/push. No history adjudication, engine, console or desktop implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-26: Primary integrator Codex codex/shogi-board. Prerequisites MSH64 source368c613/integration444107c and MSH65 source826cd2f/integrationce42d06 Done. Selected profile SHOGI-P1 mshogi.shogi.csa36.v1.

Paused further conformance acceptance while correcting newly exposed MSH64 reference-domain issue. Native legal source passes initial4groups; perft comparison correctly reported54vs18 in two unsupported reference roots. Independent geometry identifies old opponent-check root invalid for reference, rather than product-rule disagreement. Original diagrams remain native-only structural regressions; no outcome inferred.

MSH64 correction now re-integrated/Done at9f9f609. Native comparisons after corrected domain PASS167sequences/3850plies/4017prefixes;169depth2 perft/divide cases PASS including independently expected initial900/divide30 bothsides. Native7groups and wholeworkspace260tests PASS (2existing deep chess tests ignored), fmt/clippywarningsdeniedPASS; fresh independent AI code/fixture reviews PASS. Source/harness findings resolved without changing native rules to match unsupported reference output.

Review-ready final nativeAPI/evidence docs/m4/legality-validation.md and.json; actual source fingerprints match all final files and compared releaseprobe. Independent AI code review docs/m4/legality-review.md and independent fixture review docs/m4/completeness-fixture-review.md PASS, no material native finding. Perft harness duplicate-expectation and reference counter-headroom findings resolved with actual negative sentinels. Reference-domain correction independently re-reviewed/reintegrated MSH64; original unsupported diagram remains native-only expected regression. Backlogdoctor and diffchecks PASS. Final integration/native CI pending beforeDone.

Final native source0845392 integrated/pushed codex/rewrite6f6301e5a6b90a1dca986e88c6ce6e022f07e5d7. Source fingerprints and report bindings independently checked against integrated Git tree. Normal native CI40305615224 PASS Ubuntu24.04/macOS15/Windows2022; Desktop regression CI40305615506 PASS all3. URLs and exact evidence docs/m4/legality-validation.md/.json. Completion bookkeeping only reuses this verified source. Existing chess consumers, M2/M3, MSH61 guarantee and MSH40 correction unchanged; MSH41–47 remainToDo. MSH67 becomes eligible but NOT started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed native local shogi board legality for mshogi.shogi.csa36.v1: complete promotions/nonpromotions/drops, king safety/pins/nifu/dead ranks/uchifuzume defenses, atomic apply/exact undo, repetition identity and counter-independent perft/divide. Seven focused groups and wholeworkspace260tests PASS (2existingdeepignored);169 reviewed cases,167 deterministic reference runs/4017prefixes and169perft/divide comparisons PASS. Independent AI code and fixture reviews PASS; verification-tool findings and unsafe reference roots resolved with preserved historical evidence, not altered product rules. Integrated source6f6301e, normal native+desktop regression CIall3PASS. Game/history/outcomes/declarations/search/console/UI remain later tickets; stop before67.
<!-- SECTION:FINAL_SUMMARY:END -->
