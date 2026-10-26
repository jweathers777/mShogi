---
id: MSH-66
title: Complete shogi legality diagnostics and independent conformance comparisons
status: Ready
assignee: []
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T07:44:00Z'
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
- [ ] #1 The native library enumerates every complete legal board choice including legal nonpromotions and drop categories, and validates king safety,nifu,dead drops and pawn-drop mate with legal defenses.
- [ ] #2 Legal-action acceptance is atomic; attacks,pseudo-legal candidates and legal choices are separate. King capture is absent and no chess castling/en-passant/pawn-double-step rule leaks in.
- [ ] #3 Perft/divide and deterministic sequence diagnostics compare complete LegalAll sets/transitions against the admitted reference while independently sourced expectations settle disagreements.
- [ ] #4 Both sides and promotion/check-evasion edge cases have retained fixtures; apply/undo and exact-state/repetition identity restoration are verified without treating reference history helpers as outcomes.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
