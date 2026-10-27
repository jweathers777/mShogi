---
id: MSH-105
title: Deliver bounded exact xiangqi impossible-to-win verification
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T16:10:00Z'
updated_date: '2026-10-27T21:30:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-94
  - MSH-95
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/coverage.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/exact-verification.md
ordinal: 105000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
XIANGQI-P1 owner decision retains WXF3.2A as required M4 scope. MSH-94 supplies native history/adjudication records and the fast sound recognizer/assessment contract; this separately bounded ticket supplies the exact path beyond it. Do not import a chess-specific algorithm. This is future intent/acceptance, not an implementation plan or permission to start. Resolve the named G-NOWIN goal/domain/proof fixtures before affected implementation; missing proofs cannot silently become positive capability claims. Native handoff84 and complete-game review103/45 must wait for this delivery.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Define and independently review the native WXF3.2A verification goal/domain and sound proof obligations with source/position fixtures before implementation, distinguishing existence of a possible native board win from forced-win evaluation, voluntary resignation/agreement or administrative failure. Any material unresolved semantic case is presented to the owner before affected behavior; no chess mating-only goal or comparator material shortcut is silently imported.
- [x] #2 Integrate and independently verify the MSH-94 native assessment contract and sound fast recognizer, and implement the separately bounded or independently terminable exact verification path beyond it. Preserve exact selected native rules/history and proof-relevant context; Xiangqi algorithm/representation must be justified rather than copied from Chess. Native bounded execution must not depend on later aggregate84 or ordinary engine97; advertise only actually implemented operations.
- [x] #3 Expose distinct proven-impossible-for-both, proven-at-least-one-side-can-win with witness/proof, and unknown/incomplete outcomes. Failure/cancellation/exhaustion to prove impossibility is never proof that a win is possible; an experiment whose adjudication depends on unresolved exact work remains unresolved and unscored.
- [x] #4 Bound expensive reconstruction, replay, graph expansion, memory and diagnostics; cancellation/resource exhaustion/worker failure preserve the authoritative game and complete save path. Actual modes/limits/request-purpose/target/history/identity remain explicit, unsupported restores reject clearly and retired work cannot return after selection/undo/replacement.
- [x] #5 Independently expected positive/negative/unknown/resource/cancel fixtures and exact replay records establish the recognizer and verifier, including positions where stalemate is a loss and histories that affect permitted continuation. Actual bounded native execution evidence, independent correctness review and relevant regressions are required before this task or native handoff84 is Done; MSH-103 explicitly checks this obligation before MSH-45 closure.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Independently review the owner-selected natural-board-win goal, finite graph state and small proof/witness fixtures against the MSH-94 contract and existing G-NOWIN research before affected implementation.
2. Implement a native bounded cancellable exact reachability backend, with explicit proof/witness/unknown/failure results, bounded storage and useful work diagnostics; justify Xiangqi state identity and algorithm from measured alternatives.
3. Connect read-only context-bound exact assessment to the native game boundary, preserving history, request/selection/undo retirement, canonical fast-mode compatibility and complete saveability; no engine/console/desktop capability.
4. Verify independent fixtures, a feasible exhaustive small universe, strict records/context and all limits/cancellation; benchmark representative material classes and alternative graph strategies within bounded workloads.
5. Obtain separate completed-boundary correctness review, resolve findings, run relevant workspace/native CI, document actual feasibility/Unknown policy, integrate and push only MSH-105.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner G-NOWIN resolution during MSH94 selects mshogi.xiangqi.natural-board-win.exists.v1: existential ordinary board-action continuation to WXF3.1A checkmate or stalemate loss only, not minimax and not repetition/conduct/resignation/agreement/admin/time/invalid-claim awards. Full known history governs actual match outcomes but cannot create a positive natural-win witness through penalties. See docs/m4/xiangqi/owner-decisions.md and history-contract.md. This resolves the prior goal semantics gate; exact algorithm, independently reviewed proof/unknown fixtures, bounded/terminable execution and feasibility remain this ticket required scope. No implementation started or dependency changed;94 and95 must both be Done.

MSH94 is now Done with tested source integration a822450d1618810e0ef37efb9ef05035939c5746 and native/package CI PASS. This task remains To Do, unassigned and unstarted: MSH95 is the remaining unmet direct dependency. No graph change or exact-backend/feasibility work occurred.

MSH95 is now Done: tested routing source69838067abfdd98771d5b1fdd19984d18fb15b55, native CI40578289309 and package CI40578289335 PASS all3 systems. Both direct dependencies94/95 are Done, so105 is dependency-ready. It remains To Do, unassigned and unstarted; no implementation, ownership, acceptance criterion or dependency was changed in the95 batch. Preserve the selected existential natural-board-win goal and the required bounded/exact verification obligations. Existing graph continues105→84→96/97;95 did not add evaluator/search/engine execution.

Primary builder/integrator @codex in primary /Users/johnweathers/Development/Personal/mShogi on codex/rewrite at3c426636ef31b0a5f8c459571c35a6590bbebaa3. Clean checkout, sole worktree, refreshed origin matches; no active task owners. Dependencies94/95 Done. Pinned Backlog1.55.1 used for mutations (PATH1.55.0 read-only). Independent preimplementation semantic/fixture reviewer /root/nowin_goal_review; no competing builder. MSH84/96/97 and future architecture reviews remain unstarted.

Preimplementation separate /root/nowin_goal_review accepted intrinsic placement+turn quotient and independent P1/P2/108+1020-state fixtures before code; retained exact-reviews/goal-and-fixtures.md. Implemented compact full-key BFS/DFS exact closure/witness backend, bounded cancellable native read-only ExactAnalysis resource and native assessment conversion; no live command/result mutation or new restored mode. Six focused tests PASS, independent universe/model and F002 actual histories included. Release20k/100k two-order experiments and all positive witness replays retained in exact-evidence; exact-verification.md records domain, owned capacity versus RSS, alternate algorithms, measured Unknown classes and safe unscored policy. Workspace test484PASS/2existingignored; strict clippy passed after fixing a test-only int-plus-one lint. Separate completed-boundary reviewer /root/nowin_boundary_review assessing final docs/code. Native CI and final integration evidence pending.

Separate completed-boundary review PASS with no blocking findings; actual source inspection, independent6test execution, release31-observation probe and witness replays recorded exact-reviews/completed-boundary.md. P1/H1 coverage and vocabulary/documentation findings resolved. No owner semantic decision needed; profile unchanged. Pending source commit/nativeCI and completion checklist.

Tested/integrated production source601399502486a9143488cafe9bfa21dc3d24976d pushed normally to origin/codex/rewrite. Native CI40584949290 PASS on macos-15/ubuntu-24.04/windows-2022: workspace484/484/481passed,2existingignored deep perft each; isolated desktopQA93each; fmt/clippy/build/snapshot/console checks passed. Local workspace484passed/2ignored; desktopQA93passed; native frontend fixture regeneration unchanged. Earlier clippy test-style finding was fixed before final checks. Automatically triggered packageCI40584949296 supporting regressions still running; no new Xiangqi GUI/protocol capability or platform performance claim.
Acceptance mapping: AC1 controlling intrinsic goal/domain and independently frozen fixtures reviewed before code (exact-reviews/goal-and-fixtures.md). AC2 compact native exact closure/witness backend plus ExactAnalysis and assessment conversion, old sound fast scope and strict fast-only records preserved. AC3 distinct exact proof/Unknown/failure, actual witnesses and unresolved/unscored policy documented; no ordinary-game completeness claimed. AC4 state/edge/time/owned-memory/witness ceilings, cancellation/selection/replacement/undo/reopen guards, immutable sourceGame/save record, explicit purpose/context/limits/runtime identities; no child process or expensive exact restore. AC5 six focused tests, independent108+1020static-state universe and reachable component/edge checks, actual F002 histories, endgame/overflow/native-side/resource/cancel fixtures, retained31x2bounded measurements/witness replay and separate completed-boundary review PASS. Full graph109unique/resolved/no cycles and doctor/diff checks PASS.
No temporary worktree was needed; primary remains codex/rewrite. No MSH84/96/97, architecture review, game-profile change, engine or console product work started. M4/45 remain open. MSH84 has only105unfinished and becomes dependency-ready when105isDone; retain its To Do/unassigned state until separately authorized.

Final supporting packageCI40584949296 PASS all3 at exact source601399502486a9143488cafe9bfa21dc3d24976d: normal packages/bundled-engine inspections, strict frontend/native checks and existing actual Linux/Windows Chess/Shogi GUI journeys passed; macOS result is packaging/engine evidence, not a new macOS GUI run. Required nativeCI40584949290 also PASS all3. All5AC/4DoD supported; no required check unavailable or failed. Final completion change is Backlog-only; tested production source remains6013995. Normal completion integration/push uses codex/rewrite; no disposable worktree exists to remove. MSH84 now dependency-ready but remains To Do/unassigned/unstarted;96/97 remain blocked by84 and unstarted. M4 remains open.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered the owner-selected intrinsic WXF3.2A natural-board-win proof boundary at tested/integrated source601399502486a9143488cafe9bfa21dc3d24976d on codex/rewrite. Native compact exact-key BFS/DFS establishes negative reachable closure or retains a checked finite checkmate/stalemate witness; cancelled/exhausted work is Unknown, execution/input failure separate. Board/turn is the independently reviewed proof quotient; complete known history/root/policy and monotonic runtime identity remain authoritative observation/admission context. No penalty/claim/voluntary result supplies a witness.

ExactAnalysis is explicitly read-only; native assessment conversion preserves fast miss versus incomplete exact work. Existing fast recognition, strict fast-only canonical records, unsupported restoration and complete save path remain unchanged. Later engine/console/desktop consumers were not started and must retain unresolved/unscored treatment when exact adjudication is needed.

Evidence: independent preimplementation domain/fixture review and separate completed-boundary review PASS;6focused tests,108+1020independently modeled static states with complete choice/successor comparisons and reachable component checks, actual F002 history/selection/undo/reopen/resource/cancel fixtures. Retained31observations each at20k/100k with witness replays, M1Pro32GiB hardware and actual metrics in docs/m4/xiangqi/exact-verification.md. Small palace/Cannon roots close; larger four-defender samples remain Unknown. Defaults20kstates/200kedges/250ms/4MiB/4096witnessplies; configurable ceilings and actual cancellation documented. This is not universal ordinary-play WXF completeness or playing strength.

Local workspace484PASS/2existingignored, desktopQA93PASS; nativeCI40584949290PASS macOS/Linux/Windows (484/484/481plus93QAeach); packageCI40584949296PASS all3 including existing Linux/Windows GUI regressions. Doctor/full109-taskgraph/diff/fmt/clippy checks PASS. MSH84 is now dependency-ready and remains unstarted; M4/45open,84/96/97not implemented in this batch.
<!-- SECTION:FINAL_SUMMARY:END -->
