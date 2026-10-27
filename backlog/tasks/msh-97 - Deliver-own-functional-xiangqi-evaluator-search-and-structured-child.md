---
id: MSH-97
title: Deliver own functional xiangqi evaluator search and structured child
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
ordinal: 97000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Provide transparent native Red/Black evaluation and functional bounded search/play using authoritative full known game context and selected native adjudication, complete legal actions and claim policy; retain native terminal, unresolved, incomplete and failure distinctions.
- [ ] #2 Implement the real structured child consumer, truthful execution capabilities and purpose-specific read-only analysis acceptance. Bind full target/prefix/identities/limits; enforce cancellation, replacement, bounded queues/diagnostics/process cleanup and stale results across games.
- [ ] #3 Run independent tactical/terminal/history correctness fixtures, deterministic work-limit searches, actual child entry-point and cancellation/replacement/resource tests; actual observations only, perspective and position provenance explicit. Expensive native assessment uses bounded or independently terminable work and returns unknown on exhaustion.
- [ ] #4 Independently review evaluator/search/consumer evidence and run relevant chess/shogi regressions. M4 establishes functional legality and execution, not competitive strength; MSH-54 retains measured strength/tuning.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
