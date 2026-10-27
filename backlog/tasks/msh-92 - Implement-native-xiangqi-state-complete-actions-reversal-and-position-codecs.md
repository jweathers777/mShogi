---
id: MSH-92
title: Implement native xiangqi state complete actions reversal and position codecs
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T18:11:00Z'
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
  - docs/m4/xiangqi/owner-decisions.md
  - docs/m4/xiangqi/coverage.md
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Root@codex sole native builder/integrator in primarycodex/rewrite at53e90a9; parallel91 fixture/reference files isolated. 1.Add native xiangqi module with Red/Black, General/Advisor/Elephant/Horse/Rook/Cannon/Soldier,90bounded intersections/river/palace/forward geometry, complete from/to actions, structuraldiagnostic states and explicit declaredcounter/rootprovenance limits. 2.Document strict bounded position/coordinate codec with native semantics, duplicate/unknown/version rejection for itsposition envelope, notcanonical fullgame orsharedrouting. 3.Implement private/explicitlimited structural capture/turn/counter transitions and exact poststate-checked undo; no completelegalclaim until93. 4.Focusednative structural/codec/atomicity/reversal tests, independent review, normalintegration/push beforeDone; existingChess/Shogi untouched. No history/outcome decisions orsharedconsumer scaffolding.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
XIANGQI-P1 selected2026-10-27. This independently settled board/state scope is unaffected by G-CYCLE/G-NOWIN later history/proof questions or missing complete AXF/CXA corpora. Ready status will be set only after80 reviewed planning integration; no execution in80 batch. Preserve44-action opening set/F002 seeds; actual reference build/admission remains91, not80.

MSH80 planning completed under XIANGQI-P1 with reviewed integrated/pushed evidence412bbe743f78cef47a931854eef3cdd39783a83b. This task is now Ready and dependency80 satisfied. No implementation/reference build/tests performed for this future ticket, no ACchecked, no assignee/implementationplan. Root stops at implementation boundary;91+92 then93 is first bounded queue. G-CYCLE/G-NOWIN remain affected94/105gates, not this scope.

Ownerauthorized batch91+92 then93 only. Startup clean primarycodex/rewrite53e90a9, one worktree, nootheractiveBacklogownership. Backlog1.55.1 pinned mutationbinary resolvesPATH1.55.0difference.80Done selectedXIANGQI-P1 governs. Root sole durabletaskowner/integrator; isolatedindependentfixture/reference contributors will be recorded; no taskDone until own evidence/review/integration. G-CYCLE/G-NOWIN stay94/105 and must not affectlocal boardlegality.

Native state/action/private transition foundation implemented, strict scoped FEN/position JSON and immutable root provenance documented. 12 focused native tests PASS, core all-target clippy -Dwarnings and workspace fmt PASS. Independent source/boundary reviewer separately ran checks: PASS; initial FEN and missing nullable-field findings resolved with tests. Evidence docs/m4/xiangqi/board-foundation-evidence.md, position-api.md, reviews/state-review.md and reviewed hashes. Complete legality remains93; history94/105 untouched.
<!-- SECTION:NOTES:END -->
