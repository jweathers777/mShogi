---
id: MSH-134
title: Synchronize Chess history-view assertions with actual view completion
status: Done
assignee:
  - '@codex'
created_date: '2026-11-06T09:06:00Z'
updated_date: '2026-11-07T21:36:00Z'
labels: []
dependencies: []
references:
  - >-
    https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127801682025
  - crates/mshogi-desktop-session/src/shared_moves_tests.rs
documentation:
  - docs/review/history-view-134.md
  - docs/review/evidence/msh134/verification.json
type: bug
ordinal: 134000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-112 native Windows CI at unchanged desktop source44c86b01428ca5f0de56fdb96cbe7e5b013677ea failed in shared_moves_chess_real_admission_reopen_and_takeback_do_not_fabricate_cues at shared_moves_tests.rs174 (selected_root.historical and last_action none). Run42190693190 job127801682025. Read-only independent review found Chess Inspect queues asynchronous Work::View without an admission receipt; shared_command_with_receipt returns an immediately completed receipt while I/O can still be Viewing. The helper may assert before poll_files installs Completed::Viewed. This predates112 and has no runner dependency. Preserve the strict historical/action assertions and accepted production semantics; determine the correct completion boundary before changing tests. A successful unchanged-source retry is not a correction of this issue. No implementation is authorized as part of112.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 History-view assertions wait for the exact requested view completion and retain their original historical, action-cue and known-row semantics.
- [x] #2 Controlled scheduling coverage distinguishes command admission from asynchronous view completion and demonstrates the original race without accepting a stale snapshot.
- [x] #3 Relevant native controller tests pass on Linux, macOS and Windows, with failed and retry receipts preserved and independent read-only review recorded.
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
Primary builder/integrator Codex, clean/origin-aligned primary codex/rewrite at816f670d1bacc95c73eae7c62dfa7b1e3b132edb; preserve unrelated133 review and historical-access worktree. (1) Trace exact existing shared Moves helper, Chess Inspect queue_view/currentIO/result admission and native assertions. Fix only test synchronization: distinguish stable admission receipt from exact requested view IO completion, preserve strict root/cue/row/native-history assertions and bounded wait/failure. (2) Add cfg(test)-only result-observation hold at existing FileWorker poll boundary for deterministic delayed completion; exercise actual native/controller/worker path and old-helper negative control, fast/delayed observations, exact IO/target and refusal of superseded/failed/stale completion as appropriate. No fake native results, added sleeps as proof or product behavior changes. (3) Focused shared Moves and relevant persistence/controller checks, fmt/clippy and fresh independent read-only review; preserve original Windows127801682025 failure and successful unchanged retry127812835546/source44c86b0 as historical, not a fix. (4) Freeze narrow source, required existing Rust ubuntu24.04/macOS15/windows2022 jobs on identical source, one completion-aware monitor, cancel automatic unrelatedDesktop with provenance; no broad GUI campaign. One structured verification record and concise report with actual commands/OS/source/control outcomes/review/platformreceipts. Evaluate original3AC/4DoD only with evidence, normal integrate/push/Done, no M5 or othertask implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Private shared Moves helper now waits admission plus exact requested Chess View I/O id/Complete/target and rejects failed or superseded work under unchanged20s total helper deadline. Original strict3Moves regressions untouched; cfg(test)-only result observation hold controls actual FileWorker publication without changing execution/native payload. Four new controls cover live/priorhistorical delays1/4, same-root newIO, actual supersession and real tokenFailed. Admission-only mutation of same helper fails deterministically cargo101 Viewing vsComplete (0pass1fail), exact source restored. Corrected local checks: Moves7/nativeprojection4/persistence19/workflow11=41passed; Clippyalltargets/allfeatures/fmt passed. Initial new-test compile DTO equality and wrong prior-prefix ReturnLive expectation corrected, logs qualified as development failures. Fresh independent read-only msh134_review accepts source/no materialfinding; exact2file hashes/mutation source recorded. Current tests/CI pending freeze/3OS Rust; no AC/DoD markedcomplete from localMac alone. Preserve Windows127801682025 failed and unchangedretry127812835546 at44 as history, notfix. No product/rules/M4acceptance/M5change.

Independent source and preliminary evidence review accepted with no materialfindings. Frozen f426c67e73af53b40aee6b9491a2d8cbde55bfb5 two source hashes/diff match; all local41logs and expectedmutation101 verified, historicalfailed/retrylogs reread, automaticDesktop42413689587 terminalcancelledreceipt retained. AC1/2 now supported andchecked; AC3/allDoD remainpending requiredactual3OS nativeCI and finalpublication. Rust42413689537 Ubuntu128527069550/macOS15 128527069747/Windows128527069788 allrunnersacquired/Test; one completionmonitor71273, no dirty source or newcampaign. No successinferredfromlocalMac.

Final independent read-only msh134_review ACCEPT atf426c67e73af53b40aee6b9491a2d8cbde55bfb5,no materialfindings. Actual Rust42413689537 attempt1 Linux128527069550/macOS15128527069747/Windows128527069788 all8requiredsteps pass; all5criticalcases exactly2passes ineachrawlog(default/allfeature). Reviewer independently verifies2source/14evidence hashes, originalfailed/retryrecords, mutation101, qualifieddraftfailures, twofiletest-onlyscope and datedM4handoff preservation. All3originalAC/DoD1–3 nowchecked. DoD4/statusDone awaitnormalfinal evidencepublication/push; codef426alreadyintegrated. No retry,newtests,campaign,productbehavior,M4reopening orM5work. Structuredrecord/docs retainactualIDs/source; current M4handoff annotations supersedeopen134snapshotwithoutrewriting47verdict.

Independent post-publication reconciliation ACCEPT,no remainingfindings. Actualpublished evidence89abb3f HEAD/origin equal,Done134/all3AC4DoD; original7criteriontexts/sourceeqf426/all2source14evidencehashes unchanged. Finaldiff onlytask/report/index/3datedcurrent-handoffcompletion metadata;47task/milestone/acceptedindex unchanged,M4stillCompleted77/77. Ready--readyempty,48ToDo/unstarted. Normalfinalmetadatacommitpush follows,no moretestsorotherwork.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
MSH-134 complete on codex/rewrite. Corrected only the private shared Moves test helper: Chess Inspect admission is distinct from asynchronous View completion, so the helper now waits for the returned exact I/O id, Complete and requested native event, and refuses failed/superseded work under the unchanged20-second deadline. Original historical-root/cue/known-row assertions remain strict. A cfg(test)-only FileWorker result-observation hold leaves actual native replay/worker execution and normal product behavior unchanged.

Four controlled cases exercise delayed1/4 observations from live/prior historical prefixes, repeated root/new I/O, actual supersession and real token-invalidated completion. Disabling only the new wait reproduces the old helper failure deterministically (cargo101,Viewing vsComplete); restored exactsource passes. Local focused Moves7/projection4/persistence19/workflow11=41tests pass, all-target/all-featureClippy and formatting pass. Initial new-test compile/expected-context errors are preserved as development observations, distinct from final success.

Exact tested/integrated sourcef426c67e73af53b40aee6b9491a2d8cbde55bfb5; actual Rust42413689537 attempt1 Linux128527069550/macOS15 job128527069747/Windows128527069788 all8requiredsteps PASS,no retry. Each5criticalcases passed twice perplatform(default andallfeature). Original Windows127801682025 failure and unchangedretry127812835546 at44c86b0 remain preserved as historical, notthefix; unrelatedautomaticDesktop42413689587 intentionallycancelled withreceipt. Fresh independent read-only source+finalevidence review approves all3originalAC/DoD1–3,no materialfindings,verifies2source/14evidencehashes. No timingsoruniversalflake-freepromise follows from observation control.

Approved evidence published normally at89abb3fe36f57cbe2d48f60beb830f0cb149bf5d and origin-aligned beforeDone/DoD4. All3AC/4DoD are supported. Report docs/review/history-view-134.md and single index docs/review/evidence/msh134/verification.json retain commands, negative proof and exactsource/platformreceipts. Current M4handoff notes distinguish this completed quality correction from47's earlier open134snapshot; M4remainsCompleted77/77, no requirements/runtime/rules/search/protocol/schema/budget change. Unrelated133/historical-accessworktree preserved; none created. MSH-48 remains nextplanned M5item,unstarted; no othertask implemented.
<!-- SECTION:FINAL_SUMMARY:END -->
