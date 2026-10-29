---
id: MSH-102
title: Complete xiangqi analysis outcomes recovery and document journeys
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-29T12:55:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-101
  - MSH-99
  - MSH-97
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 102000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Deliver native bounded read-only analysis with explicit target/history/purpose/perspective/limits, cancellation and replacement; live turn acceptance cannot apply an analysis action/claim. Preserve native operation meanings and show real observations only; no invented Chess Full capability.
- [x] #2 User-facing claims/warnings/refusal, win/loss/draw/agreement/resignation, unknown/unscored assessment, cancelled/incomplete work and process failures have distinct meanings. Recovery/retry preserve authoritative history and stop stale work.
- [x] #3 Actual Save/Open/close/history/Return to live/Resume and unsaved-change journeys preserve complete native records/policies/settings/observations, exact prefix provenance and fresh runtime identities. Capacity failure occurs before destructive acceptance and offers a valid save path; failed operations leave current document intact.
- [x] #4 Verify full running packaged journeys including hard states through separately labeled instrumentation if needed, plus focused tests, accessibility/focus/status and independent review. Preserve normal-package exclusion and evidence provenance; three-platform all-game closure remains MSH-88.
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
1. Root Codex is sole primary builder/integrator on clean primary codex/rewrite at095f27a; dependencies101/99/97 are Done, no active worker owns102, origin refreshed equal. PinnedBacklog1.55.1 replaces incompatiblePATH1.55.0 for all mutations. Preserve approved101 design/evidence and stop after102.
2. Reuse the existing game session, ProductApp document/quit guards, Dialog, ApplicationShell, native inspection, archive admission and supervised purpose-specific execution. Expose native Xiangqi facts/terminal capabilities without frontend rules/history reconstruction; add a cheap native draw-offer advisory reused by formal admission, readable native concluded/pending labels and enable existing strict Xiangqi product Open. Preserve profiles, counters, canonical schemas, capacity guarantees and complete event-prefix distinction (partial atomic prefixes viewable but not analyzable).
3. Build the missing Xiangqi analysis/outcome/claims/agreement/recovery/history/document UI in stable shell slots. Small scope-justified sharing of native search observation, analysis/history/control presentation and focus/error/capacity behavior with Shogi where identical; do not refactor Chess architecture or force native-specific semantics. Native limits depth1–3/visits1–512; normal analysis is read-only with one resource owner. Preserve historical targets, cancelled/incomplete/failure/unscored distinctions, stopped reopen/ReturnLive and explicitResume, exact-token confirmations, settings and unsaved guards. No invented agreement policy or ChessFull/Xiangqi exact execution capability.
4. Add authoritative native-generated fixtures and clearly feature-gated Xiangqi verification states (warning/fourth-loss/count/offer/concluded/failure/capacity as actual records permit); normal packages must exclude instrumentation. Focused tests cover analysis cancellation/replacement/staleness, actions/advice/terminal semantics, exact event navigation, file failures/capacity and cross-game shared changes. Existing99/101/76 evidence reused where unchanged. Record any defensive pending state verified only by presentation test honestly, not as executed native GUI.
5. Build/run actual packaged macOS ordinary journeys through play, bounded analysis, cancellation/replacement, Save/Open/close, selected history, ReturnLive/Resume and unsaved protection; use separately labeled native verification packages for hard outcomes/failure/refusal/capacity states and preserve package/revision/source provenance. Obtain independent review, resolve findings, run relevant static/native/frontend checks and applicableCI; distinguish actualGUI from package/CI and platform limitations.
6. Record concrete remaining duplication for106 without starting it. Finalize102 throughCLI only with all originalAC/DoD supported, integrate/push normally, no ownworktree remains, leaveprimaryrewrite and stop. Do not start103/45/106 or closeM4.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner continuation attachment3cb0d087 authorizes102completion with acceptedChess/Shogi behavior as product references and only small justified sharing. Native read-only audit found existing exact-purpose/history execution/archive controls sufficient; productOpen gate and frontend facts/commands remain missing. Draw-offer eligibility uses private last_offer_ply and must be native queried/reused, not inferred inTS. Explicit no automatic engine agreement policy is added; native engine choices remain board/claim. Normal depth1–3/1–512 analysis remains sound_fast_only; partial intra-command history stays inspectable but cannot analyze. Any absent native pending fixture is recorded honestly. Root ownsprimary integration; isolated bounded native implementation/reviews may be delegated without sharedfile competition.

Implementation integrated/pushed6009b25; reviewed SaveAs name fixf8149ce; GUI/review feedback fixes6dec5ab+a1f8234(final production source). Authoritative native draw advice reuses admission; normal strict productOpen enabled. Shared native panels avoid another independent copy of analysis/history/recovery. Final local197frontend,TS,29independentfocused,132desktop-session(allfeatures),16corehistory,5nativehost andclippy/fmt pass; exactcommands/revisions and GUI package provenance retained docs/m4/evidence/msh102/completion-2026-10-29. Actual Mac normal+separately labeledQA journeys complete includingplay/read-onlyanalysis,cancel/replace,incomplete,warning/fourth/count/agreement/resign,missingengineRetry,exactprefixhistory/stoppedreopen/ReturnLiveResume,atomicfailedSave/badOpen,unsavedquit/keyboardfocus andreal256→257admission+successfulSave/reopen. Reviewer xiangqi102_boundary_review independent source/tests/artifacts found2P2presentation issues, both fixed/retested/recaptured; nooutstandingmaterialfindings. NoindependentGUIexecutionclaimed. CI atfinalsourcea1f pending; task remainsInReview withoriginalAC/DoDunchecked untilrequiredchecksfinish. Nativef8149ceCI40872721376 all3PASS; unchangednativeevidence reusable. MSH-103/45/106notstarted;M4open.

Final completion evidence: docs/m4/evidence/msh102/completion-2026-10-29/README.md maps all four original criteria to native, frontend, actual packaged Mac and review evidence. Final production source a1f8234e6d82add1c485e7e2537e414cffd4d49d is integrated and pushed on codex/rewrite. Rust CI40874262186 and Desktop CI40874262241 both passed Windows, macOS and Linux. Desktop Windows/Linux GUI results are existing Chess/Shogi regressions, not new Xiangqi GUI acceptance; MSH-88 retains all-game platform closure. MSH-111 failure history is preserved. Intermediate Linux runner failure40874033735 at6dec5ab is retained verbatim, with no runner change or claimed fix; new unassigned MSH-112 To Do tracks the distinct early-exit classification investigation without gates or implementation. Independent review of source, independently executed focused tests, final PNG/AX/package hashes and failure qualifications has no outstanding material findings; it was artifact review of root-controlled GUI, not independent GUI execution. Relevant handoffs now point to the current journeys while preserving their earlier gates as historical records. No archive/schema/limit/profile change; no accepted observation/history discarded. Source and documentation identify concrete future MSH-106 diagnostic inputs without starting it. Primary checkout remains codex/rewrite; no new task worktree was created. MSH-103 is the next dependency-satisfied product review but remains To Do; do not start it automatically.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed Xiangqi bounded read-only analysis, native claims/warnings/terminal actions and recovery, exact-event historical inspection, explicit Return to live/Resume, and normal strict Save/Open/close/unsaved journeys in the accepted shell. Small shared native panels consolidate identical Shogi/Xiangqi interactions; native rules, complete contexts, stale-result safety, archive v2 and capacity admission remain authoritative. Actual normal and labeled QA macOS journeys, 197 frontend tests, 132 all-feature session tests, 16 core history tests, 5 host tests, independent source/test/artifact review and final three-platform Rust/Desktop CI support all criteria. Reviewed feedback fixes preserve failed Save visibility, suppress live obligations after conclusion, and keep accurate observation-limit feedback across Save. Final production source is a1f8234e6d82add1c485e7e2537e414cffd4d49d on codex/rewrite; exact packages, hashes, screenshots, commands and CI are linked in the completion evidence. Untimed limits and separate exact execution remain; defensive pending GUI reachability and Windows/Linux Xiangqi GUI are not claimed. MSH-112 preserves an intermediate unchanged-runner CI mismatch for future investigation. MSH-103/45/106 are unstarted and M4 remains open.
<!-- SECTION:FINAL_SUMMARY:END -->
