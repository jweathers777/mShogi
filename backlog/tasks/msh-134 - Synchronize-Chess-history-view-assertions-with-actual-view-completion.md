---
id: MSH-134
title: Synchronize Chess history-view assertions with actual view completion
status: In Review
assignee:
  - '@codex'
created_date: '2026-11-06T09:06:00Z'
updated_date: '2026-11-07T20:58:00Z'
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
- [ ] #1 History-view assertions wait for the exact requested view completion and retain their original historical, action-cue and known-row semantics.
- [ ] #2 Controlled scheduling coverage distinguishes command admission from asynchronous view completion and demonstrates the original race without accepting a stale snapshot.
- [ ] #3 Relevant native controller tests pass on Linux, macOS and Windows, with failed and retry receipts preserved and independent read-only review recorded.
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
Primary builder/integrator Codex, clean/origin-aligned primary codex/rewrite at816f670d1bacc95c73eae7c62dfa7b1e3b132edb; preserve unrelated133 review and historical-access worktree. (1) Trace exact existing shared Moves helper, Chess Inspect queue_view/currentIO/result admission and native assertions. Fix only test synchronization: distinguish stable admission receipt from exact requested view IO completion, preserve strict root/cue/row/native-history assertions and bounded wait/failure. (2) Add cfg(test)-only result-observation hold at existing FileWorker poll boundary for deterministic delayed completion; exercise actual native/controller/worker path and old-helper negative control, fast/delayed observations, exact IO/target and refusal of superseded/failed/stale completion as appropriate. No fake native results, added sleeps as proof or product behavior changes. (3) Focused shared Moves and relevant persistence/controller checks, fmt/clippy and fresh independent read-only review; preserve original Windows127801682025 failure and successful unchanged retry127812835546/source44c86b0 as historical, not a fix. (4) Freeze narrow source, required existing Rust ubuntu24.04/macOS15/windows2022 jobs on identical source, one completion-aware monitor, cancel automatic unrelatedDesktop with provenance; no broad GUI campaign. One structured verification record and concise report with actual commands/OS/source/control outcomes/review/platformreceipts. Evaluate original3AC/4DoD only with evidence, normal integrate/push/Done, no M5 or othertask implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Private shared Moves helper now waits admission plus exact requested Chess View I/O id/Complete/target and rejects failed or superseded work under unchanged20s total helper deadline. Original strict3Moves regressions untouched; cfg(test)-only result observation hold controls actual FileWorker publication without changing execution/native payload. Four new controls cover live/priorhistorical delays1/4, same-root newIO, actual supersession and real tokenFailed. Admission-only mutation of same helper fails deterministically cargo101 Viewing vsComplete (0pass1fail), exact source restored. Corrected local checks: Moves7/nativeprojection4/persistence19/workflow11=41passed; Clippyalltargets/allfeatures/fmt passed. Initial new-test compile DTO equality and wrong prior-prefix ReturnLive expectation corrected, logs qualified as development failures. Fresh independent read-only msh134_review accepts source/no materialfinding; exact2file hashes/mutation source recorded. Current tests/CI pending freeze/3OS Rust; no AC/DoD markedcomplete from localMac alone. Preserve Windows127801682025 failed and unchangedretry127812835546 at44 as history, notfix. No product/rules/M4acceptance/M5change.
<!-- SECTION:NOTES:END -->
