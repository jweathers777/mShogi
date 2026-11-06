---
id: MSH-134
title: Synchronize Chess history-view assertions with actual view completion
status: Ready
assignee: []
created_date: '2026-11-06T09:06:00Z'
labels: []
dependencies: []
references:
  - >-
    https://github.com/jweathers777/mShogi/actions/runs/42190693190/job/127801682025
  - crates/mshogi-desktop-session/src/shared_moves_tests.rs
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
