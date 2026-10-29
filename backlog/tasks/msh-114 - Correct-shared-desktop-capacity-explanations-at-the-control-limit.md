---
id: MSH-114
title: Correct shared desktop capacity explanations at the control limit
status: Done
assignee:
  - '@codex'
created_date: '2026-10-29T20:22:00Z'
updated_date: '2026-10-29T21:54:00Z'
labels:
  - m4
  - desktop
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-99
  - MSH-102
  - MSH-113
references:
  - desktop/src/product/NativeSessionPanels.tsx
  - crates/mshogi-desktop-session/src/shared_xiangqi_archive.rs
  - crates/mshogi-desktop-session/src/shared_archive.rs
documentation:
  - docs/m4/xiangqi/delivery-review.md
  - docs/desktop/evidence/msh114/README.md
priority: medium
type: bug
ordinal: 114000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-103 fresh independent review found that NativeRecovery prioritizes any diagnostic containing 256 as an observation-count limit. Xiangqi check_capacity and Shogi counts emit combined diagnostics naming both 256 observations and 8192 controls. A control-only refusal with zero observations therefore tells the player that 256 observations were recorded. Native admission and complete saveability remain correct. Correct this shared user-facing resource diagnosis without changing accepted rules, archive compatibility, limits, retained history or MSH-61 guarantees. This is the bounded corrective prerequisite to final MSH-103 acceptance; no broader UI redesign or architecture review is authorized.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 At a real control-only refusal, including an active document with 8191 controls and one reserved stop/close slot but zero observations, both Xiangqi and Shogi show a truthful control/application-limit explanation and never claim that 256 observations were recorded.
- [x] #2 The genuine 256-to-257 observation boundary remains accurately explained, including Xiangqi ordinary plus exact observations. Byte, native-event and replay-work resource failures are not mislabeled as observation/control exhaustion, illegal actions or game outcomes.
- [x] #3 Current complete history and accepted observations remain saveable without discard or raised constants. Save and Save As remain available where native admission permits them; Save/Open does not reset structural limits and the user is not told to retry an unchanged impossible operation.
- [x] #4 Focused tests exercise actual native diagnostic payloads through shared presentation for control-only, observation-only and ambiguous combined text in both consumers, with current native capacity/stop-reserve regressions reused or rerun as applicable. Verify the user interaction in a running packaged app using separately labeled instrumentation if necessary, excluded from normal packages.
- [x] #5 Record focused checks and exact source/package evidence, independent review and resolved findings, applicable CI and normal integration/push. Reconcile the MSH-103 matrix and acceptance blocker after correction without closing MSH-45 or starting MSH-106.
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
1. Own this correction in primary codex/rewrite at e1bba329f01f9f4c23f628c4efd82a7f4dc9776f; dependencies99/102/113 Done, no competing In Progress owner. Use pinned Backlog1.55.1.
2. Replace numeric-substring resource guesses with truthful shared wording: only use a specific observation count when native retention refusal and authoritative snapshot accounting support it; otherwise concise generic capacity/saveability language. Preserve technical diagnostics and all native admission/archive contracts.
3. Add focused Shogi/Xiangqi regressions using real native diagnostic payloads, including ambiguous count/control, bytes/events/replay and ordinary+exact observations; verify Save/Save As and independent file errors. Add separately gated real control-boundary QA fixtures if needed for packaged interaction.
4. Run focused frontend/native admission checks, obtain independent source/test/artifact review, build normal and labeled QA packages and exercise actual packaged user interaction. Record exact source/package/platform/CI evidence; verify normal instrumentation exclusion.
5. Reconcile MSH103 F01 with an appended correction record preserving original failed evidence and unfinished review status, finalize114 only when supported, integrate and push normally. Stop after114; do not finalize103 or start45/106.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-29 Codex primary builder/integrator; clean startup at e1bba32 on codex/rewrite. All three dependencies remain Done;103 In Review ownership retained as review handoff, no competing implementation. Global Backlog1.55.0 is incompatible; pinned1.55.1 used for every mutation. Unrelated three worktrees preserved.

Bounded correction: shared recovery no longer interprets numeric limits in combined/archive diagnostics. Only known native observation-retention preflight prefixes select observation wording, with authoritative ordinary+exact snapshot count; all ambiguous/byte/event/replay reasons use truthful generic saveability language. Both technical disclosures retain capacity and retained error. No native product admission/schema/limit changes. Feature-gated control-only fixtures journal8191 real preferences+reservedstop and verify actual refusal, complete Save/Open/SaveAs. New27-case frontend suite and58 focused tests/TypeScript pass; native fixture test passes, independently rerun1/1. Independent reviewer capacity114_review approves current source/test boundary without material findings. PackagedGUI and CI pending; initial new-test compile error used incorrect context.game accessor, corrected to canonical context equality before passing execution.

Source1c6b97e2bf7ecf560be1902e01b4e95038ee3482 integrated/pushed normally. All235 frontend tests/25files pass (27new), strict TS/fmt/allfeature clippy pass. Native new control Save/Open test plus Xq256→257, mixedordinary/exact and Shogi256→257 tests pass. Normal and separately labeled QA macOS packages built and normal instrumentation/bundled-engine smoke passed. Actual CuaGUI: both0-observation control refusals generic/saveable; SaveAs/Save success; XqOpen/Resume stillstructurallyrefused; genuine256thengineobservation retained,257threfusedaccurately,savedall256; normalappOpen8192controlarchive→Resume generic/Save succeeds; originalChess0document restoredclean/stopped.9PNG/AX+exactpackage/archivehashes in docs/desktop/evidence/msh114. Fresh reviewer capacity114_review independently passed27frontend+1native, inspectedall9captures/manifest, no materialfindings. Native/Desktop CI40959072793/40959072831 pendingfinalplatformcompletion; MSH103addendum supplied but103remainsInReview/uncheckedremainingcriteria.

Final independent source/test/artifact/documentation review PASS, no material findings. All three nativeCI jobs at1c6b97e PASS; DesktopMac/Linux PASS and Windows installed GUI regression stillrunning.9screenshots and8packagefilehashes independently matched. Currentnormalproduct also opens8192control0observationXqarchive→Resume correctlyrefusesgeneric→Save succeeds. Source unchanged since1c6b97e. Remaining completion check: finalWindowsDesktopCI.103 remainsInReview and only its additive114correction/statusdelta is being reconciled.

Final exact-sourceCI allPASS at1c6b97e2bf7ecf560be1902e01b4e95038ee3482: native40959072793 andDesktop40959072831, Windows2022/macOS15/Ubuntu24.04 all6jobscompletedSUCCESS. Includes nativeworkspace/allfeatureconstructor/regressions, frontend235, host/package/bundledengine and existingWindows/LinuxChess/ShogiGUI campaigns. Actual newresource-limitGUI ismacOS only. Olde1 docs-headUbuntu runnerfailure40949231746 preserved asMSH112sameunchangedMissingResponsevsUnexpectedExit;111driverfailureunchanged. Source/productcode unchanged after1c. Documentationlinks/evidencehashes/JSON/doctor/diffchecksverified. AllAC/DoD supported; sourcealreadyintegratedandnormallypushed oncodex/rewrite; evidenceclosure follows. No temporaryworktree/newtaskimplementation.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Corrected shared Shogi/Xiangqi capacity explanations at 1c6b97e2bf7ecf560be1902e01b4e95038ee3482. Combined diagnostics no longer identify a resource merely by mentioning its numeric bound. Ambiguous, control, byte, event and replay refusals use truthful generic saveability language; known native observation-retention refusals retain accurate counts, including Xiangqi ordinary plus exact observations. Technical diagnostics, file-error feedback, Save/Save As/Open, complete history and native stop reservation remain intact. Verification: 235 frontend tests (27 new), TypeScript, formatting, clippy, four focused native tests, actual normal and labeled QA macOS interaction, package instrumentation exclusion, and fresh independent source/test/screenshot/manifest review passed. Native CI 40959072793 and Desktop CI 40959072831 passed all six platform jobs. Evidence: docs/desktop/evidence/msh114/README.md. F01 is resolved and the MSH-103 corrective dependency is satisfied; its original assessment and failed probe are preserved. MSH-103 remains In Review for separate resumed acceptance/finalization. No MSH-45 or MSH-106 work started.
<!-- SECTION:FINAL_SUMMARY:END -->
