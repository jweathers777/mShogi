---
id: MSH-100
title: Deliver xiangqi engine protocol adapters and bounded match evidence
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T15:09:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-96
  - MSH-97
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 100000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Choose and document the minimum real external dialect(s) from admitted opponents; pin rank/alphabet/FEN/options differences (Pikafish UCI0-9 versus Fairy generic UCI1-10/UCCI0-9). Preserve structured native functionality and do not advertise unsupported conversions.
- [ ] #2 Actual supervised handshake/position/go/stop/quit and bounded runner uses native match authority and complete known history. Explicitly report invalid replies, unsupported adjudication policy, missing responses, timeout/process exit and unscored unresolved work separately from native game outcomes.
- [ ] #3 Record own-engine self-play and a bounded match against an independently built/admitted Xiangqi engine with exact source/binary/network/options/work policy and replayable commands/moves/events. Deterministic replay is required; rerun equality only under documented deterministic settings/limits, never promised for timed observations.
- [ ] #4 Execute protocol-negative/cancel/cleanup/replay tests and independent review. Audit/licensing/source facts alone do not admit an opponent or establish playing strength.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
