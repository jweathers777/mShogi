---
id: MSH-98
title: Extend native session and semantic presentation for xiangqi consumers
status: In Progress
assignee:
  - '@codex-session'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-28T12:09:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-84
  - MSH-95
  - MSH-97
  - MSH-72
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 98000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Authoritative serialized native session consumes real xiangqi rules/engine contexts and owns command admission, result acceptance and all actions; preserve exact history/purpose/request/session/generation/limits and cancel/replacement/undo/historical-selection protection.
- [ ] #2 Expose native semantic Red/Black pieces/labels, intersection topology, river/palaces, orientation, complete choices, outcomes/claims and analysis provenance without forcing chess squares or shogi promotion/hands/declaration. Extend only fields required by this consumer.
- [ ] #3 Provide native read-only opponent movement inspection against the exact live or selected history prefix; no TypeScript movement rules or actionable enemy markers. Distinguish view changes from live mutation; requests/late observations never migrate to another selected target.
- [ ] #4 Run concrete native session/presentation/race fixtures and shared chess/shogi regressions with independent review. A diagnostic harness is separate from finished product UI and excluded from normal distributables.
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
1. Add a concrete native Xiangqi session adapter to existing explicit shared desktop dispatch, with native record/context/action/claim/outcome ownership, bounded actual engine requests and read-only result acceptance; do not implement product UI or desktop archive99.
2. Present Red/Black semantic identities and explicit intersection/river/palace/orientation topology, complete choices, native warnings/count advice/conclusions/pending and fast/exact provenance; preserve existing Chess/Shogi serialized shapes where unaffected.
3. Add native read-only hypothetical-owner movement inspection, live/history-prefix selection and controller token/request/session/process/limits guards. Keep all real transitions in native Game and retire work on replacement/undo/view/cancel/retry; explicit exact execution stays bounded and separate if exposed.
4. Exercise native session/real child/race/inspection/strict-command/capacity fixtures and existing shared regressions; independent review, relevant CI, source integration only after both98/100 reviews, one root integrator. Diagnostic constructors only in test/verification builds.
5. Keep standard product new-game/file paths unavailable for Xiangqi until their existing owners; expose the native contract for the next actual UI/archive consumer without claiming completed product delivery.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Parallel ownership audit before source:98 deps84/95/97/72 allDone;100 deps96/97 allDone, no mutual edge.98 owns coreXQread-onlyinspection additions, desktop-session/shareddispatch/semanticcontract, tests/nativeexamples and docs/m4/xiangqi/session*.100 owns consoleexternalprotocol and runner modules/tests/docs/m4/xiangqi/protocol*. Both consume unchanged completed97engine/84nativecontracts; neither owns sharedengine redesign. Root sole integrator and98builder;100 isolatedworker. Primarye923cfd5712698d4a8b059ffb3b37b0e31de8f8b clean/refreshedorigin equal; baselineCLI1.55.1 pinned. Appmanagedworktree98 /Users/johnweathers/.codex/worktrees/xiangqi-session98/mShogi,100 /Users/johnweathers/.codex/worktrees/xiangqi-protocol100/mShogi. Globalhandoff/coverage, taskrecords/finalCI and any incidentaloverlap reconciled onlybyroot. No dependency changes; no laterUI/archive/architecturework.
<!-- SECTION:NOTES:END -->
