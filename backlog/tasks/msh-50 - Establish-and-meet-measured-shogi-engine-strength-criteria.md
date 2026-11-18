---
id: MSH-50
title: Establish and meet measured shogi engine strength criteria
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-18T18:15:00Z'
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
  - MSH-180
  - MSH-186
  - MSH-187
  - MSH-188
  - MSH-189
  - MSH-190
  - MSH-191
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
  - docs/m5/chess-search-costs.md
ordinal: 175000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Functional shogi search/play does not satisfy P-02. This ticket owns the shogi-specific opponent pool, transparent baseline, release-strength criteria, search/evaluation improvement and measured verdict, using MSH-48 shared infrastructure. An initial learned evaluator improvement is separately owned by MSH-55; no requirement for an identical neural architecture in every game is inferred.

Open D-09 questions: establish meaningful shogi opponents and starting positions, resource/fairness conditions, release-strength targets and uncertainty criteria from baseline/pilot evidence; record owner decisions on material targets before claiming success. Search tuning, evaluator choices and compute budgets remain empirical, not resolved by this planning ticket.

Project direction 2026-11-13 (AGENTS.md, "Current Chess strength direction"): Shogi moves to the one shared search core and supplies its own search position, evaluation and special rules; do not build a separate search for this game. docs/m5/shared-search-core.md records what Shogi needs from the core. Project decision 2026-11-15: Shogi and Xiangqi move to the shared core first, then Chu Shogi and Tenjiku Shogi. Transfer useful techniques, not Chess constants, pruning assumptions, royal semantics or repetition identities; preserve the accepted native profile(s). Routine correctness/tactical tests and bounded profiling accompany coherent increments. Game tests follow "Game tests in this direction" in docs/m5/strength-roadmap.md; the earlier 15-minute match allowance no longer applies (revised 2026-11-15). Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reproducible shogi baseline and suitable independent opponent pool are established, with profile compatibility, limits, openings/colors, failures and executable identities recorded. Unavailable external opponents remain explicit measurement limitations; self-play alone is not independent strength evidence.
- [ ] #2 Game-specific shogi strength targets and comparison/uncertainty criteria are explicitly decided and documented from baseline evidence before a release-strength verdict. Numeric targets are not fabricated from other games or nodes-per-second results.
- [ ] #3 The mShogi shogi engine has measured search/evaluation improvements and meets the agreed criteria in reproducible matches against the defined pool. Search correctness, legal complete actions, history-aware terminal behavior and regression evidence remain intact; speedups alone do not establish stronger play.
- [ ] #4 The final shogi report records the tested revision, all experimental conditions and uncertainty, actual successes/failures and independent review. Open required targets block completion and M5 closure rather than being absorbed into optional M6 research.
- [ ] #5 A substantially complete conventional engine for this game, on the shared search core with its own search position, evaluation and special rules, is independently verified before the major strength study; established techniques are adapted to native tactics, history and outcomes, and development game tests follow docs/m5/strength-roadmap.md without per-optimization formal studies.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
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

Status check, 2026-11-15: the 2026-11-09 note above asks for John's choice of the transfer direction after MSH-139 (Chess Strength Assessment). That direction was chosen on 2026-11-13: one shared search core, with Tori as the second game (done in MSH-165). When this game starts is still John's decision; this task stays To Do until then.

Decision, 2026-11-15: John set the order of games: Shogi and Xiangqi move to the shared core first, then Chu Shogi and Tenjiku Shogi. When this task starts is still John's selection.

Status check, 2026-11-15: the move of this game to the shared search core is now the separate task MSH-180. This task keeps the strength criteria, opponent pool and measurement, and depends on MSH-180.

Owner direction, 2026-11-16: after MSH-185 (Chess Strength Anchor), Shogi strength work starts with MSH-186 (Shogi Strength Harness), then MSH-187 (Shogi Strength Round). MSH-186 proposes the external opponent pool; John approves the pool, targets and baseline protocol (AC 1-2) before any external rating games. MSH-187 supports AC 5. This task keeps the strength criteria and the final measurement.

MSH-187 (Shogi Strength Round) isolates a coherent speed/history/time set; see docs/m5/shogi-strength-round.md for final-source evidence and comparison status. MSH-188 (Shogi Evaluation Round) records the next classical evaluation set and depends on MSH-187. This does not complete conventional-engine maturity or approve an external rating campaign.

Owner direction, 2026-11-17: after MSH-187 (Shogi Strength Round), the Shogi order is MSH-189 (Shogi Request Overhead), MSH-190 (Shogi Selectivity Round), then MSH-188 (Shogi Evaluation Round).

Owner direction, 2026-11-17: MSH-191 (Shogi Strength Anchor) follows MSH-188 and gives the first floodgate-referenced Shogi strength estimate. Strength targets for this task stay John's decision after that result.

From MSH-191 (Shogi Strength Anchor), 2026-11-18: conditional floodgate-referenced index 2765 (approx. 95% 2723-2807) at 60 s + 1 s, 304 games, engine main de5ae4ae (search/eval) with USI clocks. Gikou family 2772; YaneuraOu/elmo 1000k 0/24 (family spread 374 > 150). Not an official floodgate rating; report docs/m5/shogi-strength-anchor.md.

Correction (MSH-191 review): the 374-point family spread is a mechanical flag; the elmo-only value is prior-dominated and the difference is weakly supported (0/24, p = 0.06 at 2765). elmo was dropped after m01, a recorded deviation that moves the index toward the Gikou family.

Owner target (John, 2026-11-18): mShogi should play competent Shogi with no obvious blunders, as far as non-NNUE (classical) techniques allow; move it out of strong-amateur into solid professional level - it need not match the Chess engine or beat most professionals, but its ranking should not be below professional strength if that is achievable without NNUE. Limit effort: this engine is a solid foundation for later NN-evaluation experiments and less-known games (for example Chu Shogi), not a large standalone investment. Measurable proxy (floodgate-referenced level and a blunder criterion) to be proposed by the agent and confirmed by John before it is used as an acceptance threshold.

Proposal for John (agent, 2026-11-18) - measurable proxy for the owner target and a bounded path. NOT an accepted threshold until John confirms.
Proxy:
(1) Level: conditional floodgate-referenced index >= 3000 at 60 s + 1 s, measured as in MSH-191 against at least two families with the family check reported. Rationale: no calibrated floodgate-to-professional mapping exists; the classical engines that reached and passed professional level around 2013-2017 appear on floodgate at about 2900-3300 (Yss1000k 2893, elmo_WCSC27_479_1000k 3131, gikou2_1c 3300). 3000 is about +235 over today's 2765 and is an assumption, not a measured pro rating.
(2) Blunders: (a) the 413-position tsume suite at 1 s >= 95% (today 386/413 = 93.5%); (b) a blunder screen of mShogi's anchor games with a stronger reference engine used only as a measurement tool: rate of moves that turn a non-losing position into a clearly lost one, target and threshold to be fixed after one baseline measurement.
Path (classical, non-NNUE, bounded): the main remaining lever is evaluation quality. Bonanza-style piece-pair evaluation (KPP/KKP linear tables, incremental update) trained by regression on a larger self-play corpus is what moved 2006-2017 engines from strong amateur to professional strength without neural networks. AGENTS.md authorizes Texel tuning of handwritten terms and reserves 'learned evaluation' for neural networks; piece-pair tables are linear but very large, so John should confirm they count as classical. Budget proposal: at most two rounds (piece-pair evaluation with a larger corpus; then one search/time round), each with a predecessor test and a short anchor recheck, roughly US$10-25 cloud each; stop and report if the index stays below 3000 after both.

Owner decision (John, 2026-11-18): piece-pair (KPP) evaluation is not pursued because it does not transfer to larger variants. The current classical Shogi engine (MSH-191 index about 2765) is the accepted baseline for later learned-evaluation experiments; further Shogi strength work is deferred to learned evaluation (MSH-55). Before freezing, one bounded blunder screen (MSH-193) documents the baseline's weaknesses and may fix a mechanism that explains a large share of clear blunders.

MSH-193 (Shogi Blunder Screen), 2026-11-18: 226 clear blunders in 15,805 baseline moves (1.43/100), none in a mechanism >= 30% (time trouble 28.3%, capture refutation 27.0%, drop refutation 23.5%); 10 s search avoids only 28%; elmo losses are gradual (1 clear blunder in 24 games). No fix round. Blunder suite tools/strength/data/shogi-blunders-v1.json; baseline avoids 73/226 at 1 s.
<!-- SECTION:NOTES:END -->
