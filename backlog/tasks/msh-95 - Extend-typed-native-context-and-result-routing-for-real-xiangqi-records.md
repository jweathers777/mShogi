---
id: MSH-95
title: Extend typed native context and result routing for real xiangqi records
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-94
  - MSH-69
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 95000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Route real authoritative xiangqi canonical payloads through explicit game/profile/edition/native-schema/transport identities with RawValue-preserving strict outer/nested parsing. Reject duplicate/unknown/missing/conflicting versions without fallthrough; existing chess/shogi and historical records retain semantics.
- [ ] #2 Bind request acceptance to purpose, complete known history/root, target position, limits, request/session/process identity; historical selection and semantic undo cannot revive retired work. Preserve native side/score/operation meanings and distinct action/claim/conclusion/incomplete/failure results.
- [ ] #3 Exercise real native records including same board/different history and non-ply terminal events. Unsupported xiangqi execution stays explicit/unscored and unadvertised until the real engine consumer lands; no borrowed Chess Full operation.
- [ ] #4 Preserve bounded actual encoded envelopes/diagnostics/queues and supervised cancel/cleanup. Run strict-parsing, race/replacement/cancellation and existing actual chess/shogi subprocess regressions; record independent boundary review.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
