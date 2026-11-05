---
id: MSH-46
title: Deliver shared six-game integration across console engine records and desktop
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-05T21:12:00Z'
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

MSH88 final shared packaged integration accepted2026-11-05, evidence e68e8e3d1787f2d029e3601259070807bdbab180 pushed to codex/rewrite; tested codebc92bfe5e3b20e9e999df12cbf54b91682c71b8d, all6 required platform jobs pass and Windows/Linux49/49 actualGUI phases plus separately qualified localmacOS27 actualall6/seven-profile journeys. Current ownership/originalAC1–5 mapping: docs/m4/shared-integration/README.md; single exact-source record verification.json in that directory. All current native/library/console/engine/session/document/frontend owners are exercised together; no production rules/limits/architecture or minimum changed, no retired reader revived. Fresh88independent review approved functional/platform evidence. Original46AC6 remains MSH89; no46criteria are checked or narrowed by this handoff,46 remains ToDo/unassigned, and108/47/M4 are not closed.89 is dependency-ready but not begun.

MSH-89 independent contract reconciliation approved 2026-11-05; publication/integration and89DoD4 closure follow. Report docs/m4/shared-integration/contract-review-89.md and compact review index contract-review-89.json independently support every original46AC1–6. Reviewed integration3bcaf013b7e9ea7d37a5a542700de8a1ea041b2b, reused exact tested runtime/platform sourcebc92bfe5e3b20e9e999df12cbf54b91682c71b8d and unchanged88record; two fresh source/evidence reviewers plus distinct findings/DAG reviewer approved. All original46sixcriteria and16dependencies including90 remain intact and unchecked; no46activation or owner claim. Current nativeChessM2/Full/console/live-desktop scopes and deliberately retired development formats are distinguished; all6games/two immutableTenjiku profiles use existing owners, game-specific resource accounting and actualplatform evidence. No material blocker remains for shared acceptance. Only stale rustdoc/Load-install wording corrected.112 remains a separately owned unfixed functional diagnostic race, not M5 optimization or repaired by later retries. After89Done,46 is the next dependency-satisfied M4 task and still requires its own activation, criteria/DoD reconciliation and normal integration;108/47/M4 are not completed here.

MSH-89 is Done. Independent original46AC1–6 contract reconciliation/report and compact review index were normally integrated/pushed on codex/rewrite at22b4f409f738cd7dec45a78ab7ecc4cf9661322b before final89closure. All six obligations have supporting evidence, with no material shared-contract blocker; current source/compatibility/resource/platform qualifications and the separate unfixed112 disposition remain explicit. Original46criteria/dependencies, ToDo status, unassigned ownership and unchecked items are preserved. With89Done all16dependencies are satisfied;46 is the next M4 task and still requires its own authorized activation, final criteria/DoD reconciliation and normal completion.108/47/M4 remain open; none started here.
<!-- SECTION:NOTES:END -->
