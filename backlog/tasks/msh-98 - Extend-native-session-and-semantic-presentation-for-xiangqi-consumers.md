---
id: MSH-98
title: Extend native session and semantic presentation for xiangqi consumers
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
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
