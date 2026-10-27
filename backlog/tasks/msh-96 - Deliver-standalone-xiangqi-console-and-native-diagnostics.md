---
id: MSH-96
title: Deliver standalone xiangqi console and native diagnostics
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-84
  - MSH-95
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 96000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Add explicit xiangqi selection/profile to the existing independently usable console, inspect native board/side/history/outcomes, list/apply complete actions, exact undo and relevant perft/divide diagnostics; preserve common human/machine output and existing games.
- [ ] #2 Use documented game-owned notation/position adapters and canonical envelope save/load with strict failure atomicity; refuse lossy unsupported conversions or incomplete-history restoration explicitly.
- [ ] #3 Actual subprocess tests cover bounded input/errors, terminal commands preserving board ply, history-sensitive results and replay; record focused checks and independent review. Protocol/match consumers remain a separate ticket.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
