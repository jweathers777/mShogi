---
id: MSH-46
title: Deliver shared six-game integration across console engine records and desktop
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-10-26T16:47:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-41
  - MSH-42
  - MSH-43
  - MSH-44
  - MSH-45
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-76
  - MSH-77
  - MSH-78
  - MSH-79
  - MSH-88
  - MSH-89
  - MSH-90
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
ordinal: 46000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The five game scopes own their rules and game-specific adapters/tests. This ticket is the single owner of reusable boundary extensions and cross-game integration under A-01 through A-07: game/profile selection and dispatch, complete-action transport, native engine/session coordination, common canonical-record evolution and compatibility, shared board/action presentation and packaging. Do not duplicate those deliverables in each game ticket. Existing chess behavior and MSH-26 evidence remain valid regression obligations.

Delivery may coordinate with game tickets before their completion; dependencies below gate final cross-game acceptance, not permission for speculative framework work. D-08 remains open for concrete public contracts, schema evolution and necessary interoperability adapters. Select protocols only for actual consumers/opponents; no universal DSL, ABI or chess-only record requirement. The current M3 batch is unchanged.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Shared library/console/structured engine/session boundaries select and carry explicit rules profile/version, complete actions and rules-relevant state/history for all six games. They preserve atomicity, claim versus board-action distinctions, identity/cancellation/stale-result safety and bounded reporting without frontend legality or Python/node round trips.
- [ ] #2 Common canonical-record/version compatibility and save/reopen plumbing preserve all six games and reject unsupported or lossy conversions explicitly; game-specific encodings and expectations remain owned by MSH-41 through MSH-45. Previously supported chess records and adjudication modes have regression evidence.
- [ ] #3 Shared desktop mechanisms support the real square/intersection and large-board presentations, multi-stage complete-action choices, promotions/drops or other profile choices, keyboard and readable textual interaction required by the implemented profiles. Partial selection never commits a partial turn.
- [ ] #4 An ownership/coverage matrix ties each shared mechanism to this task and each game-specific adaptation to its game ticket. Cross-game library, console, native engine, saved-record and actual desktop flows run together; chess regressions and transitions between games/profile versions are exercised.
- [ ] #5 Installed packaged applications with their bundled native engines demonstrate the required six-game flows on Windows, macOS and Linux, including cancellation/stale work, persistence, outcomes, recovery and accessibility. Actual revisions, environments, commands and failures are recorded; synthetic fixtures or browser-only tests do not establish this acceptance.
- [ ] #6 Independent integration review resolves material shared-boundary findings and updates public contracts and coverage honestly. Functional delivery makes no playing-strength claim and does not move M5 targets into this task.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH62 planning refinement2026-10-26: original identity,status,AC and DoD remain intact. This complete-delivery record now depends explicitly on its required bounded component/shared work; docs/m4/delivery-plan.md traces each original criterion and single ownership. No game implementation or delivery completion occurred. Earlier M3-context wording remains historical; current authorization is planning-only. Early shared69/72/73 depend on concrete shogi native consumers, not this final record or all five game deliveries. Later game-specific shared extensions76-79 wait on native handoffs84-87 and profile research80-83, not their aggregate delivery. Final integrated packaged acceptance88 and fresh shared review89 retain all original final obligations; MSH47 still gates M4 closure.

MSH-90 is the required bounded common-console follow-up to MSH-68/69/70, owned once under MSH-46. Added to this final acceptance gate without removing existing dependencies or changing criteria. MSH-71/72 remain unstarted.
<!-- SECTION:NOTES:END -->
