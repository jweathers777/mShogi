---
id: MSH-105
title: Deliver bounded exact xiangqi impossible-to-win verification
status: To Do
assignee: []
created_date: '2026-10-27T16:10:00Z'
updated_date: '2026-10-27T16:18:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-94
  - MSH-95
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/coverage.md
  - docs/m4/xiangqi/handoff.md
ordinal: 105000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
XIANGQI-P1 owner decision retains WXF3.2A as required M4 scope. MSH-94 supplies native history/adjudication records and the fast sound recognizer/assessment contract; this separately bounded ticket supplies the exact path beyond it. Do not import a chess-specific algorithm. This is future intent/acceptance, not an implementation plan or permission to start. Resolve the named G-NOWIN goal/domain/proof fixtures before affected implementation; missing proofs cannot silently become positive capability claims. Native handoff84 and complete-game review103/45 must wait for this delivery.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Define and independently review the native WXF3.2A verification goal/domain and sound proof obligations with source/position fixtures before implementation, distinguishing existence of a possible native board win from forced-win evaluation, voluntary resignation/agreement or administrative failure. Any material unresolved semantic case is presented to the owner before affected behavior; no chess mating-only goal or comparator material shortcut is silently imported.
- [ ] #2 Integrate and independently verify the MSH-94 native assessment contract and sound fast recognizer, and implement the separately bounded or independently terminable exact verification path beyond it. Preserve exact selected native rules/history and proof-relevant context; Xiangqi algorithm/representation must be justified rather than copied from Chess. Native bounded execution must not depend on later aggregate84 or ordinary engine97; advertise only actually implemented operations.
- [ ] #3 Expose distinct proven-impossible-for-both, proven-at-least-one-side-can-win with witness/proof, and unknown/incomplete outcomes. Failure/cancellation/exhaustion to prove impossibility is never proof that a win is possible; an experiment whose adjudication depends on unresolved exact work remains unresolved and unscored.
- [ ] #4 Bound expensive reconstruction, replay, graph expansion, memory and diagnostics; cancellation/resource exhaustion/worker failure preserve the authoritative game and complete save path. Actual modes/limits/request-purpose/target/history/identity remain explicit, unsupported restores reject clearly and retired work cannot return after selection/undo/replacement.
- [ ] #5 Independently expected positive/negative/unknown/resource/cancel fixtures and exact replay records establish the recognizer and verifier, including positions where stalemate is a loss and histories that affect permitted continuation. Actual bounded native execution evidence, independent correctness review and relevant regressions are required before this task or native handoff84 is Done; MSH-103 explicitly checks this obligation before MSH-45 closure.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
