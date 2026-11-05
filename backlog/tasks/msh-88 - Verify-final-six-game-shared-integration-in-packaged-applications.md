---
id: MSH-88
title: Verify final six-game shared integration in packaged applications
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-05T12:39:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Primary builder/integrator: Codex in the primary codex/rewrite checkout; clean origin-aligned start c1aee938e8057c5a2c1ec48e86ad87716f18c811, accepted production eca9f9cfd43cf83ff4e3489292f6c25f8951bb30. Five unrelated worktrees and MSH133 ownership are preserved. A. Audit accepted per-game/current shared contracts and the47-phase packaged baseline against original88AC1–4 and MSH46AC1–5; record one ownership/evidence matrix and explicit gaps, reusing unchanged conformance/resource evidence. B. Add only missing integrated verification: actual same-process cross-game/seven-profile New/Load and outstanding-work transitions, strict current-envelope/native-history/saveability/refusal checks, complete-action/accessible UI coverage and fixed-slot actual macOS six-game journeys. Preserve rules, architecture, resource budgets and minimum760x650. C. Run focused native/frontend/observer checks and fresh independent review early; freeze coherent source, run required Rust+Desktop ubuntu24.04/macOS15/windows2022 plus actual packaged GUI on each platform, bothTenjiku profiles, under one completion-aware monitor. Preserve all failed/superseded attempts and do not treat an unchanged retry as fixing an existing race. D. Reconcile exact source/package/environment/actions in one structured shared evidence record, fresh independent final review and all original88AC/DoD; update46/89 handoffs, integrate/push normally, mark88Done only if supported. Do not begin89/46/108/47/M5 or closeM4.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup2026-11-05: CLI1.55.1 matches recorded baseline. c1aee938 HEAD/origin/FETCH_HEAD aligned; no dirty or untracked work. MSH88 Ready/unassigned/all dependenciesDone; no competing active implementer. Codex claims88 only in primaryrewrite; MSH133 remains independentlyInReview and its worktree plusfour other unrelated worktrees untouched. Owner request114b0252 requires integrated acceptance, actual all-six-game GUI on eachOS, two immutableTenjiku profiles, original46AC1–5 traceability and later89/47 ownership. Accepted44productionECA/6CI/47GUIbaseline reused only for unchanged scope; cross-game and all-six normalmacOS gaps to audit before implementation.

Focused integrated checkpoint: added one same-controller seven-profile New/reverse-current-Load ring, stale command/inspection/replacement/Open/execution rejection and malformed-Load authority/prior-file preservation (manual execution, explicitly not subprocess evidence); default and all-feature focused native test each passed. Added actual normal same-host seven-profile New/play/Save/reverse-Load/history/Resume/Take back journey and separately labeled delayed real-engine cross-game/profile New/Load retirement journey. Fresh independent source review approved after correcting verifier process classification to exclude persistent WebKit children; negative observer regression added. Final actual platform evidence remains pending; no production rules, architecture, supported formats, budgets or UI minimum changed.

Pre-campaign verifier correction: source9dd173f runs Rust42028250037 and Desktop42028250101 were proactively cancelled after code inspection found repeated fresh-document Save targets (Adams/Chess) could trigger an unhandled GTK overwrite confirmation. All six jobs had runners and ended cancelled; preserve raw receipts, not passes, in final structured record. Corrected distinct per-visit Save/capture names and used existing real keyboard activation for nested Moves controls; no product fix or unrelated race correction is claimed. Fresh independent review approved the correction; 176 external verifier tests pass. Both superseded runs reached terminal cancellation before starting a replacement campaign.
<!-- SECTION:NOTES:END -->
