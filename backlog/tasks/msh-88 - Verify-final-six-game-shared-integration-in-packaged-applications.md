---
id: MSH-88
title: Verify final six-game shared integration in packaged applications
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-05T13:54:00Z'
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
  - docs/m4/shared-integration/README.md
  - docs/m4/shared-integration/verification.json
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
- [x] #1 All six real game profiles run through shared library/console/engine/session/record/desktop boundaries with complete game-specific actions/history and honest capability/limit metadata.
- [x] #2 Game switching,cancellation/stale results,analysis/outcomes/recovery,save/reopen/history and per-profile capacity admission have cross-game regression evidence including preserved chess/M2 modes and old documents.
- [x] #3 Actual installed or packaged GUI+bundled-engine journeys on Windows,macOS and Linux cover real games and readable keyboard/accessibility interactions; build-only,protocol-only,browser-only or synthetic evidence is not substituted.
- [x] #4 Exact revisions/packages/environments/commands and failures are durable,normal distributables exclude instrumentation,and ownership/coverage maps reconcile every original MSH46AC1-5 without weakening game scopes.
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
Primary builder/integrator: Codex in the primary codex/rewrite checkout; clean origin-aligned start c1aee938e8057c5a2c1ec48e86ad87716f18c811, accepted production eca9f9cfd43cf83ff4e3489292f6c25f8951bb30. Five unrelated worktrees and MSH133 ownership are preserved. A. Audit accepted per-game/current shared contracts and the47-phase packaged baseline against original88AC1–4 and MSH46AC1–5; record one ownership/evidence matrix and explicit gaps, reusing unchanged conformance/resource evidence. B. Add only missing integrated verification: actual same-process cross-game/seven-profile New/Load and outstanding-work transitions, strict current-envelope/native-history/saveability/refusal checks, complete-action/accessible UI coverage and fixed-slot actual macOS six-game journeys. Preserve rules, architecture, resource budgets and minimum760x650. C. Run focused native/frontend/observer checks and fresh independent review early; freeze coherent source, run required Rust+Desktop ubuntu24.04/macOS15/windows2022 plus actual packaged GUI on each platform, bothTenjiku profiles, under one completion-aware monitor. Preserve all failed/superseded attempts and do not treat an unchanged retry as fixing an existing race. D. Reconcile exact source/package/environment/actions in one structured shared evidence record, fresh independent final review and all original88AC/DoD; update46/89 handoffs, integrate/push normally, mark88Done only if supported. Do not begin89/46/108/47/M5 or closeM4.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup2026-11-05: CLI1.55.1 matches recorded baseline. c1aee938 HEAD/origin/FETCH_HEAD aligned; no dirty or untracked work. MSH88 Ready/unassigned/all dependenciesDone; no competing active implementer. Codex claims88 only in primaryrewrite; MSH133 remains independentlyInReview and its worktree plusfour other unrelated worktrees untouched. Owner request114b0252 requires integrated acceptance, actual all-six-game GUI on eachOS, two immutableTenjiku profiles, original46AC1–5 traceability and later89/47 ownership. Accepted44productionECA/6CI/47GUIbaseline reused only for unchanged scope; cross-game and all-six normalmacOS gaps to audit before implementation.

Focused integrated checkpoint: added one same-controller seven-profile New/reverse-current-Load ring, stale command/inspection/replacement/Open/execution rejection and malformed-Load authority/prior-file preservation (manual execution, explicitly not subprocess evidence); default and all-feature focused native test each passed. Added actual normal same-host seven-profile New/play/Save/reverse-Load/history/Resume/Take back journey and separately labeled delayed real-engine cross-game/profile New/Load retirement journey. Fresh independent source review approved after correcting verifier process classification to exclude persistent WebKit children; negative observer regression added. Final actual platform evidence remains pending; no production rules, architecture, supported formats, budgets or UI minimum changed.

Pre-campaign verifier correction: source9dd173f runs Rust42028250037 and Desktop42028250101 were proactively cancelled after code inspection found repeated fresh-document Save targets (Adams/Chess) could trigger an unhandled GTK overwrite confirmation. All six jobs had runners and ended cancelled; preserve raw receipts, not passes, in final structured record. Corrected distinct per-visit Save/capture names and used existing real keyboard activation for nested Moves controls; no product fix or unrelated race correction is claimed. Fresh independent review approved the correction; 176 external verifier tests pass. Both superseded runs reached terminal cancellation before starting a replacement campaign.

Actual local macOS27 normal package verified in one hostPID50108 across all6games/7profiles: native keyboard humanmoves+real replies, normal New and Adams↔Muller fresh-profile choices, native seven-file cross-game Load stopped2plies→flip→Resume→root history→Return→Take back0paused→complete retained-branch Save, nativeAdamsResign and dirtyQuitCancel→Save→cleanQuit. Rebuilt bc92bfe normal bundle matches all4 actual ECA bundle files byte-for-byte. Fixed QA identity rebuilt/staged once: genuine bounded analysis Replace/Cancel, profile replacements, fresh1depth/1024node read-only Adams result at0plies, real observation Save→ToriLoad→AdamsLoad stopped. Separate OS process receipt confirms no hosts/gates/engines. Local qa18–20 helperbinding attempt reached unintended normal app; artifacts explicitly excluded and corrected explicit-QA helpers used. Current structured record docs/m4/shared-integration/verification.json and original46AC1–5 ownership matrix are drafts, acceptancefalse pending remaining exact-source CI/actualWindowsLinux GUI receipts and fresh independent final reconciliation. No production semantics/limits/minimum or acceptance requirement changed.

Reviewed sourcebc92bfe5e3b20e9e999df12cbf54b91682c71b8d: Rust42028972679 passed Ubuntu24/macOS15/Windows2022 jobs127250225725/127250225640/127250225944. Desktop42028972586 macOS15 job127250226038 passed; Windows job127250225967 passed actual49/49 GUI phases with0failures, original reportSHA81e919cedfe2cbdf0c7a6bf580fc072fb9431f4e61339c68eb1f6feec2df9917, normal/QA producer inventories verified against downloaded files. Windows includes both new single-host cross-game phases and bothTenjiku profiles. Linux job127250225410 has finished its long successful dependency step and is executing actualGUI; required receipt and independent final platform verdict remain pending. No unchanged-source retry, required-check/test change or product fix claimed. Status InReview; originalAC/DoD remain unchecked until final reconciliation.

Fresh independent finalMSH88review approved: all6 rawjob/step receipts success exactbc92bfe, sixCIlogs and default/allfeature newnative sevenprofiletest passes independently verified; Linux+Windows49/49 actualGUI reports/rawhashes and normal/QA inventories audited. Linux normalPID173199 and pendingPID260519 prove9New/7profiles/7stoppedLoad contexts+realobservations/retainedTakeback and8exact retired gates with no postgate authority/observation mutation; Windows equivalentPID1940/8008 independentlyapproved. Mac71capture references/eightactualarchives/normal4file byteequivalence/unchangedsealedQA identity validated. No material blocker. OriginalAC1–4 andDoD1–3 now supported; DoD4 intentionally waits for normal evidence commit/push, then final closure. Currentcompiled production/test inputs unchanged from fullytestedbc source; no new campaign for documentation/evidence bookkeeping.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Verified all six delivered games and both immutable Tenjiku profiles through the existing shared owners. Added a same-controller seven-profile stale-authority/document regression and two actual packaged cross-game GUI journeys. Production rules, search, architecture, budgets and the 760×650 minimum are unchanged.

Exact tested source bc92bfe5e3b20e9e999df12cbf54b91682c71b8d passed all six required Rust/Desktop platform jobs (Rust 42028972679; Desktop 42028972586). Windows and Linux each passed 49/49 actual GUI phases. Local macOS27 seven-profile normal/QA play and document journeys are explicitly separated from hosted macOS15 CI; the rebuilt normal bundle matches all four actual GUI bundle files byte-for-byte. Accepted unchanged game conformance/resource measurements are reused. Complete actions, real engine observations, stopped Load, retained branches, cancellation, stale authority, native outcomes and atomic refusal have evidence.

Fresh independent final review approved all four original acceptance criteria and applicable completion items. The complete evidence packet was normally committed and pushed to codex/rewrite at e68e8e3d1787f2d029e3601259070807bdbab180 before the final DoD4/Done closure. docs/m4/shared-integration/verification.json is the single exact-source record; README.md maps every original MSH-46 AC1–5 and preserves AC6 ownership. Failed, cancelled and excluded observer attempts remain explicit. Dense Tenjiku text limitations remain documented; no complete screen-reader accessibility or competitive-strength claim is made.

MSH-89 is the next dependency-ready M4 task and has not started. MSH-46/108/47 and M4 remain open. The primary checkout was reused; this assignment created or removed no worktrees.
<!-- SECTION:FINAL_SUMMARY:END -->
