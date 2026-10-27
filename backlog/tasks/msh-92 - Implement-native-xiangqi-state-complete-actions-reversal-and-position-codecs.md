---
id: MSH-92
title: Implement native xiangqi state complete actions reversal and position codecs
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-80
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 92000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Implement game-owned Red/Black state and intersection geometry, all seven native piece identities, complete origin/destination actions, capture/derived effects and exact checked transitions/reversal under the selected profile; no promotion/hands/declaration or chess representation default.
- [ ] #2 Distinguish strict structurally admissible diagrams, declared counters, known history and proven historical reachability. Specify and test scoped position/coordinate codecs, alphabet/rank/side conventions, bounds and malformed/duplicate rejection; a FEN-like position is not a canonical full game.
- [ ] #3 Keep structural or transition helpers private or explicitly limited until the local-legality ticket establishes the complete legal-action contract. Test atomic rejection and exact restoration including side/material/counters and derived state.
- [ ] #4 Record focused native tests and independent boundary review; preserve existing chess/shogi APIs and fixtures. No history conclusion or complete-game claim is inferred from this state slice.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
