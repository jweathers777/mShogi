---
id: MSH-94
title: Implement selected xiangqi history adjudication and canonical native records
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-80
  - MSH-93
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 94000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Implement the ratified complete history/event policy with exact known prefixes and persistent relevant piece identity. Cover all applicable WXF19/20 and chapter5 classification/exception/precedence families, repetition warnings/refusal, natural-count claimant-specific checks/capture boundaries and native terminal commands; resolve R-NOWIN and any required semantic decisions before affected implementation.
- [ ] #2 Keep advisory opportunities, committed claims, warnings, refusal, board conclusions, resignation/agreement, unresolved unscored assessments, illegal requests and resource/process failures distinct. Native adapters own actor/turn admission; terminal events may preserve board ply. Missing pre-root history cannot supply inferred occurrences/checks.
- [ ] #3 Canonical game-owned records preserve immutable profile/edition/schema, root provenance/declared counters, all complete actions/commands/events/policies and exact replay/undo semantics. Strict bounded parsing rejects duplicate/unknown/missing/conflicting identities atomically; saved/repetition/search identities are distinct.
- [ ] #4 Run independently reviewed legal history fixtures including same board/different history, published diagram boundaries, alternating versus simultaneous threats, roots/pins/victims, natural-count10/11/100 and neither-can-win proof/unknown cases. Retain exact replay/reversal and independent review evidence; no reference-engine outcome shortcut or required rule deferred to M5.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
