---
id: MSH-54
title: Establish and meet measured xiangqi engine strength criteria
status: Done
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-25T07:39:00Z'
labels:
  - handle:conventional-xiangqi
milestone: m-5
dependencies:
  - MSH-181
  - MSH-198
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
  - docs/m5/chess-search-costs.md
ordinal: 179000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Bring the Xiangqi engine to its classical baseline through the per-game strength program in docs/m5/strength-roadmap.md (D-09). John decides when the baseline is reached. The baseline is the start point for learned evaluation (MSH-55, MSH-135).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The program steps in docs/m5/strength-roadmap.md are done, or skipped with a recorded reason.
- [x] #2 John's stop decision is recorded, with the accepted baseline revision.
- [x] #3 The evidence and its limits are recorded in the step tasks; no target or rating is invented.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner follow-on2026-11-08: after initial Chess diagnostic49.2, prioritize conventional Shogi/Xiangqi development rather than repeated small Chess benchmarking. Adapt49.1 techniques to native tactics/history/outcomes; do not copy Chess constants or pruning assumptions. Routine algorithm/verification choices are autonomous, with meaningful correctness/tactical tests, profiling and15minutes cumulative routine matches/task unless explicitlyextended. Material release targets or substantial formal studies return toowner whenappropriate; they do not block ordinary conventional engineering. Preserve selectednativeprofiles andallfiveAC. No implementation ofthisfuturetask starts in49.2.

Owner requests bounded Chess engineering-performance checkpoint136 beforecross-game transfer. No additionalChessmatches. Carry measurednative query/transition,SEE/quiescence andhistory-safe TT lessons into conventionalXiangqi;136 dependency prospectivelyadded, not reopening49/49.2.

136 transfer lesson: native subset queries before expensive validation, exact cached turn state and deferred copies are practical patterns; do not transfer Chess rates/constants/SEE or board-only TT scores. Xiangqi already uses native checkpoint transactions and pending/no-win/natural-count guards; preserve these before search shortcuts. Measure native transitions/query/quiescence work and TT position/context/depth eligibility separately. Dependency136 still awaits3OS acceptance; no Xiangqi implementation started.

MSH-136 is Done. Corrected Chess source d9147713aa5ad5044a4e25a41d5142f6834a2685 passed actual Linux, macOS 15 and Windows native CI 42519269011 and independent final review. This task is dependency-ready, with Shogi first in the roadmap. Carry forward native subset queries, cached turn reuse and staged history-safe TT counters while preserving Xiangqi pending/no-win/natural-count guards; see docs/m5/chess-search-costs.md and its canonical verification record. No additional Chess match studies; routine local engine-match experiments retain the 15-minute cumulative task limit.

Owner correction after MSH-136: wait for continued conventional Chess engineering and practical-maturity review in MSH-137. Algorithm presence and the initial Stockfish diagnostic are foundations, not a maturity verdict. Preserve prior completed evidence; this supersedes the immediate transfer handoff. No Xiangqi work starts until MSH-137 is accepted.

MSH-137 reviewed production integrated/pushed at f1484b15f44817e5469a849af28796d844393223; required native3OSCI still pending, dependency not released yet. Transfer lessons: measure native query/transition work, derive caches from canonical state, defer discarded-child work via checked native projections, prove exact history context before TT bounds. Do not copy Chess irreversibility guard: Shogi captures/drops can restore material; Xiangqi lateral pawn/chase/pending/no-win history needs its own native proof. No new Chess opponent studies; conventional-game engineering precedes learned research.

Before cross-game transfer, finish MSH-138 desktop play-budget calibration so ordinary product play can use the refined engine proportionately. This is a scheduling/product-default gate, not a newly discovered search-architecture defect; existing admission ceilings and no-new-Chess-study instruction remain.

Stable task handle: Conventional Xiangqi. Use MSH-54 (Conventional Xiangqi) in new conversational reports and development summaries. Historical titles/evidence/status unchanged.

Owner consolidation2026-11-09: MSH-139 (Chess Strength Qualification) is a mandatory prerequisite. MSH-138 (Desktop Search Limits) and MSH-137 (Chess Search Refinement) engineering evidence alone cannot release cross-game transfer. Qualification requires approved target/protocol/budget and demonstrated agreed strength, not just a completed proposal. Preserve existing acceptance/evidence.

Owner revision2026-11-09: MSH-139 (Chess Strength Assessment) is discovery, not qualification. Even when that assessment isDone, do notactivate MSH-54 (Conventional Xiangqi) untilJohn reviews findings and explicitly chooses the next objective/transfer direction. Retainfull dependencies and acceptedcriteria. StatusToDo records this unresolvedownerdirection; a completed assessmentalone cannotrelease transfer.

Status check, 2026-11-15: the 2026-11-09 note above asks for John's choice of the transfer direction after MSH-139 (Chess Strength Assessment). That direction was chosen on 2026-11-13: one shared search core, with Tori as the second game (done in MSH-165). When this game starts is still John's decision; this task stays To Do until then.

Decision, 2026-11-15: John set the order of games: Shogi and Xiangqi move to the shared core first, then Chu Shogi and Tenjiku Shogi. When this task starts is still John's selection.

Status check, 2026-11-15: the move of this game to the shared search core is now the separate task MSH-181. This task keeps the strength criteria, opponent pool and measurement, and depends on MSH-181.

Direction, 2026-11-18 (John): Xiangqi follows the per-game classical strength program in docs/m5/strength-roadmap.md (the same standards as Chess and Shogi). The work is MSH-194 (Xiangqi Strength Harness), MSH-195 (Xiangqi Selectivity Round), MSH-196 (Xiangqi Evaluation Round), MSH-197 (Xiangqi Strength Anchor, needs approval with caps) and MSH-198 (Xiangqi Blunder Screen). This task keeps the final criteria and verdict.

2026-11-19: Xiangqi program steps done: MSH-194 harness, MSH-195 selectivity (+166 Elo vs predecessor), MSH-196 evaluation (+221 confirmation), MSH-197 ladder (mShogi +18 vs Pikafish 1,000 nodes; about 110 below ElephantEye 3.31, 270 below classical Fairy-Stockfish), MSH-198 blunder screen (no gross single weakness; 42% of errors avoided with 10 s). Awaiting John's stop-or-continue decision (program step 6).

Decision, 2026-11-19 (John, verbatim: "Accept baseline"): the Xiangqi classical engine after MSH-194..198 and MSH-203 (main 8c7f487d; ladder MSH-197 measured before MSH-203: +18 Elo relative to Pikafish at 1,000 nodes; MSH-203 then added about +41 Elo against its predecessor) is accepted as the Xiangqi baseline for later learned evaluation (MSH-55, MSH-135). No further Xiangqi classical round is planned unless John asks. This task keeps the formal criteria and final verdict for M5 closure.

2026-11-25 (John): the October criteria (independent opponent pool, release targets met in matches) are replaced by the strength program criteria above, as done earlier for MSH-49 (Chess). This task is closed at the accepted baseline.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Baseline accepted by John on 2026-11-19 ("Accept baseline"): main 8c7f487d.
- Steps: harness MSH-194; selectivity MSH-195 (+166 Elo against the predecessor); evaluation MSH-196 (+221); reference ladder MSH-197; blunder screen MSH-198; speed MSH-203 (+41 against the predecessor, after the ladder).
- Ladder (relative Elo, before MSH-203): +18 against Pikafish at 1,000 nodes, about 110 below ElephantEye 3.31, about 270 below classical Fairy-Stockfish.
- Blunder screen: no single large weakness; a 10 s search avoids 42% of the errors.
- Further strength work goes to learned evaluation (MSH-55, MSH-135).
- Review: no separate review applies to this decision record; the step tasks hold their own checks.
<!-- SECTION:FINAL_SUMMARY:END -->
