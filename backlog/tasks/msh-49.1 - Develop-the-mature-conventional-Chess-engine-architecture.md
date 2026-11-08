---
id: MSH-49.1
title: Develop the mature conventional Chess engine architecture
status: Ready
assignee: []
created_date: '2026-11-08T13:07:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-25
  - MSH-109
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
parent_task_id: MSH-49
priority: high
ordinal: 135000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner direction2026-11-08 replaces weak-baseline acceptance chasing with development of a strong practical conventional engine. Existing7818c33 alpha-beta/positional work is a foundation, not a substantially complete modern engine. Deliver a coherent native search/evaluation/time-management architecture in reviewable engineering increments, researched from mature primary implementations. Preserve native Game ownership and accepted execution/application boundaries; do not introduce a second rules engine or speculative universal variant framework. Routine correctness/tactical tests and bounded profiling accompany coherent increments. Aggregate routine local engine-match work is limited to15minutes per task unless John explicitly authorizes more; no per-technique substantial statistical studies or multi-hour benchmarks. Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The integrated conventional search package includes iterative deepening/PVS, aspiration re-search, history-safe bounded transposition reuse, staged TT/capture/quiet ordering with SEE/history heuristics, and bounded quiescence with native legal check evasions; exact diagnostic search remains available for independently solvable correctness comparisons.
- [ ] #2 Guarded selective search is integrated coherently: verified null-move/zugzwang safeguards, late-move reductions with re-search, conservative futility/SEE/late-move pruning and bounded extensions. The researched inclusion or exclusion of more aggressive techniques such as ProbCut/singular extensions is justified by concrete correctness or profiling evidence, without per-technique formal strength campaigns.
- [ ] #3 Traditional material/positional evaluation covers tapered piece placement, pawn structure/passed pawns, mobility/activity, king shelter/attack pressure, threats and endgame conversion with inspectable terms; independent arithmetic/position/metamorphic tests and measured incremental-state restoration support it. No trained evaluator is introduced.
- [ ] #4 Efficient native search-state access, exact make/unmake and hashing/evaluation maintenance are profiled and verified; repetition, claims/counters, mate-distance scores, TT history/context separation and cancellation restore state exactly. Search-only hypothetical/null branches never become legal game actions, played history or native outcomes.
- [ ] #5 Real UCI and supervised product execution provide reliable bounded thinking-time allocation, hard-stop responsiveness and truthful completed/selective-depth/PV/work telemetry through the existing protocol family with explicit compatibility handling. Stale observations, process failures and incomplete work retain existing honest semantics.
- [ ] #6 Independent review and appropriate actual native platform/affected-consumer checks approve a substantially complete engine checkpoint. Correctness tests, tactical/endgame regressions, short profiling and cumulative match-time accounting address the changed risks; no strength-release claim is inferred from those checks. The checkpoint is ready for the separate approved unrestricted/equal-time milestone.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
