---
id: MSH-63
title: Ratify the standard-shogi profile and software match conventions
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T09:33:00Z'
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
  - docs/m4/shogi-profile.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 63000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Primary-source clause comparison and owner decision evidence; independent profile review. This is a decision/profile task, not a rules implementation.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Owner selection SHOGI-P1 is recorded explicitly against the sourced proposal, including selected profile identity/version and untimed software adaptations; no unselected event convention is silently combined.
- [x] #2 The selected outcome contract covers fourfold/perpetual-check intervals and precedence, no-legal-move loss, entering-king advisory versus committed declarations, resignation, length counting/import provenance, and game result versus score/replay policy.
- [x] #3 Every remaining material rule ambiguity has primary-source resolution or a named blocking fixture/research item before affected implementation; source editions and conditional JSA alternatives remain traceable.
- [x] #4 An independently reviewed profile and clause-to-fixture ledger are accepted without claiming implementation or completing MSH41.
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
Record the explicit SHOGI-P1 approval against 6db8500 and reconcile the selected profile, design decision and clause-to-fixture ledger. Resolve board interpretation through concrete fixtures; identify exact deferred history research. Obtain fresh independent profile review, then integrate decision evidence.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-26: Authorized bounded batch MSH-63–66. Primary integrator Codex, branch codex/shogi-board, existing next-task worktree. Owner explicitly approved option A SHOGI-P1 mshogi.shogi.csa36.v1 against integration 6db8500e92129b95e502e7b4d5c4a26072e38652.

Selected profile, D-13, current M4 index/delivery/verification and clause-to-fixture ledger reconciled. Original MSH-62 planning/review records preserved. Fresh independent AI-agent profile review in progress; executed implementation fixtures are not claimed by this decision ticket.

Focused independent review PASS: separate AI reviewer selected_profile_review, author-independent source/clause/document inspection recorded docs/m4/profile-selection-review.md. No material findings; historical MSH-62 reviews reused only as source/planning evidence. git diff --check passed. Documentation-only decision work runs no product tests; executed board fixtures belong64–66.

Integrated source bd374dd via codex/rewrite614d2b465533c285b9e4dbed7421f204c98efc69 and pushed. Documentation-only commit uses skip-ci; no source behavior changed, existing M1/M2/M3 evidence reused. Follow-up implementation/native CI belongs64–66.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Ratified actual owner selection SHOGI-P1 option A mshogi.shogi.csa36.v1, reconciled current design/profile and clause-to-fixture owners, preserved historical proposals/reviews and unselected alternatives. Independent AI profile review PASS (docs/m4/profile-selection-review.md), no material findings. Named MSH-67 history/declaration/import gates remain explicit; no runtime conformance or complete MSH-41 delivery claimed. Integrated/pushed614d2b4.
<!-- SECTION:FINAL_SUMMARY:END -->
