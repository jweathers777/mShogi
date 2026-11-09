---
id: MSH-50
title: Establish and meet measured shogi engine strength criteria
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-09T13:28:00Z'
labels:
  - handle:conventional-shogi
milestone: m-5
dependencies:
  - MSH-48
  - MSH-41
  - MSH-109
  - MSH-49
  - MSH-136
  - MSH-137
  - MSH-138
  - MSH-139
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
  - docs/m5/chess-search-costs.md
ordinal: 50000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Functional shogi search/play does not satisfy P-02. This ticket owns the shogi-specific opponent pool, transparent baseline, release-strength criteria, search/evaluation improvement and measured verdict, using MSH-48 shared infrastructure. An initial learned evaluator improvement is separately owned by MSH-55; no requirement for an identical neural architecture in every game is inferred.

Open D-09 questions: establish meaningful shogi opponents and starting positions, resource/fairness conditions, release-strength targets and uncertainty criteria from baseline/pilot evidence; record owner decisions on material targets before claiming success. Search tuning, evaluator choices and compute budgets remain empirical, not resolved by this planning ticket.

Owner sequence2026-11-08: after the mature Chess milestone, adapt and develop established conventional search/evaluation as a coherent Shogi engine before substantial formal strength evaluation. Shogi/Xiangqi precede the less-studied variants. Transfer useful techniques, not Chess constants, pruning assumptions, royal semantics or repetition identities; preserve the accepted native profile(s). Routine correctness/tactical tests and bounded profiling accompany coherent increments. Aggregate routine local engine-match work is limited to15minutes per task unless John explicitly authorizes more; no per-technique substantial statistical studies or multi-hour benchmarks. Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reproducible shogi baseline and suitable independent opponent pool are established, with profile compatibility, limits, openings/colors, failures and executable identities recorded. Unavailable external opponents remain explicit measurement limitations; self-play alone is not independent strength evidence.
- [ ] #2 Game-specific shogi strength targets and comparison/uncertainty criteria are explicitly decided and documented from baseline evidence before a release-strength verdict. Numeric targets are not fabricated from other games or nodes-per-second results.
- [ ] #3 The mShogi shogi engine has measured search/evaluation improvements and meets the agreed criteria in reproducible matches against the defined pool. Search correctness, legal complete actions, history-aware terminal behavior and regression evidence remain intact; speedups alone do not establish stronger play.
- [ ] #4 The final shogi report records the tested revision, all experimental conditions and uncertainty, actual successes/failures and independent review. Open required targets block completion and M5 closure rather than being absorbed into optional M6 research.
- [ ] #5 A substantially complete game-specific conventional search/evaluation/execution engine is independently verified before the major strength study; established techniques are adapted to native tactics/history/outcomes and routine development stays within the15minute cumulative match allowance without per-optimization formal studies.
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
Owner follow-on2026-11-08: after initial Chess diagnostic49.2, prioritize conventional Shogi/Xiangqi development rather than repeated small Chess benchmarking. Adapt49.1 techniques to native tactics/history/outcomes; do not copy Chess constants or pruning assumptions. Routine algorithm/verification choices are autonomous, with meaningful correctness/tactical tests, profiling and15minutes cumulative routine matches/task unless explicitlyextended. Material release targets or substantial formal studies return toowner whenappropriate; they do not block ordinary conventional engineering. Preserve selectednativeprofiles andallfiveAC. No implementation ofthisfuturetask starts in49.2.

Owner requests bounded Chess engineering-performance checkpoint136 beforecross-game transfer. No additionalChessmatches. Carry measurednative query/transition,SEE/quiescence andhistory-safe TT lessons into conventionalShogi;136 dependency is prospectively added, not reopening49/49.2.

136 transfer lesson: native destination projection and deferred copies removed measured redundant Chess work without changing policy/history; gains are not a Shogi prediction. Current Shogi NativeTree submits/undoes through Game; GameUndo owns before/after Games and submit clones for atomic admission. Measure native transaction/history/query cost early before scaling conventional search, preserving CSA36 declaration/repetition/drop legality and exact restoration. This is current source inspection, not an already-measured Shogi bottleneck or authorization for unchecked moves. Keep per-game semantic TT identity and stage-specific reuse counters. Dependency136 still awaits3OS acceptance.

MSH-136 is Done. Corrected Chess source d9147713aa5ad5044a4e25a41d5142f6834a2685 passed actual Linux, macOS 15 and Windows native CI 42519269011 and independent final review. This task is dependency-ready. Carry forward native subset queries, cached turn reuse, early native transaction/undo cost measurement and staged history-safe TT counters; see docs/m5/chess-search-costs.md and its canonical verification record. No additional Chess match studies; routine local engine-match experiments retain the 15-minute cumulative task limit.

Owner correction after MSH-136: wait for continued conventional Chess engineering and practical-maturity review in MSH-137. Algorithm presence and the initial Stockfish diagnostic are foundations, not a maturity verdict. Preserve prior completed evidence; this supersedes the immediate transfer handoff. No Shogi work starts until MSH-137 is accepted.

MSH-137 reviewed production integrated/pushed at f1484b15f44817e5469a849af28796d844393223; required native3OSCI still pending, dependency not released yet. Transfer lessons: measure native query/transition work, derive caches from canonical state, defer discarded-child work via checked native projections, prove exact history context before TT bounds. Do not copy Chess irreversibility guard: Shogi captures/drops can restore material; Xiangqi lateral pawn/chase/pending/no-win history needs its own native proof. No new Chess opponent studies; conventional-game engineering precedes learned research.

Before cross-game transfer, finish MSH-138 desktop play-budget calibration so ordinary product play can use the refined engine proportionately. This is a scheduling/product-default gate, not a newly discovered search-architecture defect; existing admission ceilings and no-new-Chess-study instruction remain.

Stable task handle: Conventional Shogi. Use MSH-50 (Conventional Shogi) in new conversational reports and development summaries. Historical titles/evidence/status unchanged.

Owner consolidation2026-11-09: MSH-139 (Chess Strength Qualification) is a mandatory prerequisite. MSH-138 (Desktop Search Limits) and MSH-137 (Chess Search Refinement) engineering evidence alone cannot release cross-game transfer. Qualification requires approved target/protocol/budget and demonstrated agreed strength, not just a completed proposal. Preserve existing acceptance/evidence.

Owner revision2026-11-09: MSH-139 (Chess Strength Assessment) is discovery, not qualification. Even when that assessment isDone, do notactivate MSH-50 (Conventional Shogi) untilJohn reviews findings and explicitly chooses the next objective/transfer direction. Retainfull dependencies and acceptedcriteria. StatusToDo records this unresolvedownerdirection; a completed assessmentalone cannotrelease transfer.
<!-- SECTION:NOTES:END -->
