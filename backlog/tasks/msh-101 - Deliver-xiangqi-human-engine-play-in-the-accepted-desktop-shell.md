---
id: MSH-101
title: Deliver xiangqi human-engine play in the accepted desktop shell
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-76
  - MSH-97
  - MSH-96
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 101000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Implement actual untimed human-versus-own-engine Xiangqi through the shared native session in the accepted MSH-74 dark shell. Application header/theme/document controls/live heading/board-sidebar grammar and identical analysis-history-actions-technical order remain stable across games.
- [ ] #2 Provide a deliberate intersection board with river/palaces and Xiangqi-specific native pieces/Red/Black labels, complete board visible without scroll or zoom at supported normal and minimum desktop sizes. No generic square-board visual assumption, shogi hands or promotion UI leaks into this game.
- [ ] #3 Complete click/keyboard/orientation/action confirmation and native read-only opponent inspection with non-actionable markers; retain common live/historical footer, engine state and actual play limits. Keep raw diagnostics in details and instrumentation out of normal packages.
- [ ] #4 Focused frontend/native regressions and actual running packaged play/inspection at same sizes beside chess/shogi screenshots support independent interaction/visual review. No claim of final game delivery before remaining journeys and review.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
