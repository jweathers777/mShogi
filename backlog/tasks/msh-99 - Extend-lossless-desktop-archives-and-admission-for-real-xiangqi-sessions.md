---
id: MSH-99
title: Extend lossless desktop archives and admission for real xiangqi sessions
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-98
  - MSH-95
  - MSH-73
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 99000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Persist complete native xiangqi semantics plus desktop policy/settings/observations with exact game/profile/schema/transport identity, root provenance and commands/events; strict duplicate-preserving dispatch and native replay own validation. Preserve old chess/shogi documents.
- [ ] #2 Measure actual encoded bytes, native events, controls, observations and replay work; enforce native-owned admission before accepting any work that would destroy a complete save path, including reserved stop/close. Cover exact boundaries and observation256-to257; no silent discard or constant-only fix, preserving MSH-61.
- [ ] #3 Atomic failed/interrupted saves preserve prior valid file and dirty state; corrupt/unsupported/resource-limited opens preserve the live game. Fresh execution identities on reopen, historical observations never pending work, untimed engine stopped until explicit Resume; protect unsaved replacement/quit.
- [ ] #4 Run actual native payload roundtrips, malformed/resource/failure boundaries and historical-prefix provenance tests plus current archives regressions; record independent review. Common migration belongs to MSH-46 while native replay remains game-owned.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
