---
id: MSH-188
title: Shogi Evaluation Round
status: Done
assignee:
  - '@claude'
created_date: '2026-11-17T08:58:00Z'
updated_date: '2026-11-18T05:48:00Z'
labels:
  - handle:shogi-evaluation-round
milestone: m-5
dependencies:
  - MSH-187
  - MSH-189
  - MSH-190
documentation:
  - docs/m5/shogi-strength-round.md
  - docs/m5/strength-roadmap.md
priority: high
ordinal: 195000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Evaluation Round

## Problem
MSH-187 (Shogi Strength Round) improves search speed, exact history matching and timed defaults. The handwritten Shogi evaluation remains basic and untuned. The small data generation demonstration is not an independent training or validation corpus.

## Objective
Improve conventional Shogi decisions with classical evaluation and measured benefit against a frozen predecessor. Support MSH-50 (Conventional Shogi) without claiming an external rating.

## Approach
Choose one bounded evaluation set after inspecting the completed MSH-187 evidence. Establish a useful native-validated corpus and separate training from held-out games and openings before Texel tuning. Native rules remain authoritative. Record baseline, intended benefit, comparison method, resource cap and checkpoint before implementation or data generation. Use the existing harness and cloud path for long work. Neural evaluation, external rating studies and substantial-cost campaigns remain outside standing authorization.

## Completion
Retained changes pass native perft and differential checks, the development tactical suite and a winning sequential fast game test against the frozen task-start predecessor. Record held-out decision evidence, depth and speed tradeoffs, actual compute, independent review and limitations. Finish one coherent set without an open-ended tuning campaign.

## Additions (John, 2026-11-17)
- Order: start after MSH-189 (Shogi Request Overhead) and MSH-190 (Shogi Selectivity Round). MSH-190 owns the quiescence check stage, full SEE, margins and drop ordering; this task does not reopen them.
- Corpus: the MSH-186 export (497 positions) is a demonstration only. Generate at least about 500,000 labeled quiet positions from self-play games of the current engine (cloud, under the existing path and caps), from many openings, with game outcome labels. Split by game, not by position: about 90% training, 10% validation, plus held-out games from openings not used in training. Record generation conditions, size, duplicates and label balance.
- Terms: Shogi-meaningful handwritten terms (piece and promoted-piece values, hand-piece values by count, king safety against attackers and possible drops near the king, castle shapes, piece-square terms, mobility). Tune weights jointly by Texel regression; report validation loss before and after.
- Tests: reuse the MSH-190 opening set (at least 100 openings), tactical suite and game-test control (median late-game depth at least 8). Use H1 about 10 Elo for evaluation changes (docs/m5/strength-roadmap.md).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Before work, record a frozen baseline, intended benefit, corpus provenance and held-out split, comparison method, resource cap and decision checkpoint.
- [x] #2 A labeled corpus of at least about 500,000 quiet positions from self-play exists with a game-level training/validation split and held-out games from unused openings; its generation conditions and statistics are recorded.
- [x] #3 Retained changes pass Shogi native perft and differential checks and the development tactical suite; shared consumers remain correct.
- [x] #4 Record validation loss, held-out decision results and depth and speed tradeoffs; retain changes only after a winning sequential game test against the frozen predecessor using the MSH-190 opening set and control.
- [x] #5 Record actual compute, failed attempts, independent review, limitations and current conventional-engine documentation.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Owner @claude (Claude Code), branch codex/msh-188-shogi-evaluation in the primary checkout.
1. Shared Texel tool: move the Chess fitter's procedure into examples/support/texel.rs, generic over mshogi_search::texel::TexelModel; Chess and Shogi supply models. Corpus rows {position field, result (first player's native outcome), split, game_id}.
2. Shogi evaluation: linear in 133 integer weights (board values per face, hand values per piece and first-piece bonus, rank tables for 9 piece groups, king rank/file, slider mobility, king safety: own pieces adjacent, attacked adjacent squares, enemy pieces within 2, gold/silver defenders within 2, enemy non-pawn hand x empty adjacent squares, tempo). INITIAL reproduces the old evaluation exactly; static terms incremental.
3. Corpus: self-play of the frozen predecessor (main 578748d6, engine source a9188ebe; Linux build sha256 a29f5bd7...) on Google Cloud Batch, fixed node budgets per player drawn per pair from 24k-48k, all 315 MSH-190 openings, every native event exported with Sente outcome after native replay; quiet = not in check and no capture/promotion with SEE > 0; split by opening family hash (tools.strength.texel_data: 10% held-out openings, 10% validation, 80% train), cross-split duplicates removed.
4. Fit on train, select on validation, then held-out check once.
5. Verification: perft/differential and evaluation-identity tests, tactical suite, bench speed/depth, sequential game test.
6. Review, docs, integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Shared-core rule (John, 2026-11-17): evaluation terms belong to the Shogi position (game data); the Texel tuning tool and corpus format stay shared across games. Do not add search changes in this task.

2026-11-18 baseline and protocol (before generation), @claude.
- Frozen predecessor: main 578748d6 (target/strength/engines/578748d6bf65/mshogi); engine source identical to a9188ebe. Predecessor bench 2,400,987 nodes, 1.14M nps (M1 Pro); INITIAL-weight build reproduces exactly 2,400,987 nodes but at 0.87M nps (per-node mobility and king-safety features: -24% speed; recorded tradeoff).
- Intended benefit: >= 10 Elo from tuned classical terms; validation loss below INITIAL.
- Corpus provenance: project self-play only (no imported data), BSD project terms; generation conditions in the manifest; target about 500,000 quiet unique positions (local probe: ~41 quiet unique positions per game, so about 7,000 pairs). Fixed budgets below ~15k nodes sometimes completed no iteration (MSH-190 depth-1 cost in drop-rich positions); 24k-48k chosen; unfinished games give no labels.
- Comparison: candidate vs frozen predecessor, MSH-190 catalog (315 openings, pair N = opening N-1), 10 s + 0.5 s, SPRT H0=0 H1=10 alpha=beta=.05, min 30, max 315 pairs; plus the 413-position tactical suite at 1 s (no material regression) and bench depth/speed.
- Resource caps: corpus generation one Batch job, <=8 c3-standard-4 VMs, 4 concurrent pairs per VM, task bound 3 h, campaign window 4 h, conservative cap US$8 (expected ~US$3); game test as MSH-190 (window 2 h, US$4).
- Checkpoint: if validation loss does not fall, stop and report; if the game test reaches 315 pairs without decision or accepts H0, ablate the costly per-node terms once (speed vs. value), then record a final decision; no open-ended tuning.

2026-11-18 corpus batch 1 (msh188-corpus, image sha256:1866aec2...): 5,622 of 6,400 pairs completed (778 skipped at the 3 h task bound), 11,244 games (461 incomplete:NodeLimit, unlabeled), 3.47 h wall, conservative 22.78 VM-h / US$4.59, cleanup verified. Only ~188k unique quiet positions: deterministic fixed-node self-play repeated games from the same opening (616k within-partition duplicates of 1.27M rows). Shortfall against ~500k target.
Batch 2 (msh188-corpus2, image sha256:e84da71f..., source 69651fca): 4,400 pairs, nodes 20k-40k, plus two seeded random legal plies after each catalog prefix so repeated openings diverge; whole corpus re-split by catalog opening id (held-out openings unused in training). Task bound 2.5 h; expected conservative ~US$3.3, keeping total corpus generation within the recorded US$8 cap.

2026-11-18 corpus complete and fit (@claude).
- Corpus: batches msh188-corpus (5,622 pairs) and msh188-corpus2 (4,400 pairs, two seeded random plies): 20,044 games (877 incomplete:NodeLimit, unlabeled), 2,216,012 exported rows; split by catalog opening id (252 train / 35 validation / 28 held-out openings), 32,295 cross-split rows and 723,177 within-split duplicates removed. Quiet unique positions: train 403,044 (11,929 games), validation 54,953 (1,637), held-out 43,009 (1,253); total 501,006. Labels (Sente loss/draw/win): train 188,289/8,701/206,054. Compute: conservative 39.23 VM-h / US$7.91 (cap US$8), cleanup verified for both jobs.
- Fit (shared tool, K=0.13 calibrated on INITIAL): train MSE 0.22412 -> 0.21733, validation 0.22421 -> 0.21809 (11 sweeps, validation patience). One bounded revision with wider bounds for the 32 weights at bounds gave validation 0.21832 (worse) and was rejected. Held-out (read once): 0.21891 -> 0.21398; by hand pieces 0-2: 0.23059 -> 0.23141 (slightly worse), 3-7: 0.21663 -> 0.20908, >=8: 0.19129 -> 0.17535.
- Local (M1): bench nodes 2,400,987 -> 1,113,070; nps 1.22M -> 1.09M; 1000 ms depth equal or +1..+5 on all nine roots. Preliminary local smoke (batch-1-only weights): 13 W 3 L of 16.
- Tests: two search and one USI toy fixtures moved the Sente King to its castled square (tuned King terms prefer a corner King step to a depth-1 promotion there); Shogi fingerprint updated; 857 consumer + 19 native pass; clippy clean.
- Game test submitted: candidate da198205 vs predecessor (engine a9188ebe = main 578748d6), image sha256:72310483..., H1=10.

2026-11-18 final (@claude).
- Game test (experiment msh188-eval, candidate da198205 vs predecessor engine a9188ebe): H1 after 64 pairs, 107 W 0 D 21 L, pentanomial [2,0,17,0,45], LLR 4.096, +283 Elo (209..387), late-game median depth 10/10, no failures, all 128 games host-replayed; conservative 2.34 VM-h / US$0.47; jobs verified absent. Task total conservative 41.57 VM-h / US$8.38.
- Tactical suite at 1 s: 386/413 (development 271, held-out 115) vs predecessor 382 (271/111).
- Independent review (fresh-context subagent): no correctness bugs. Applied: fixed 16-chunk loss summation (refit gives identical weights), evaluation identity asserted also in check, source revisions in evidence, predecessor tactical link, explicit King-placement risk; checkpoint added to MSH-191. Recorded: evaluator identity unchanged (consumers only classify score bounds), Chess K grid may differ in last bits, Chess weights not refitted.
- Checks at final source: workspace clippy clean; 857 consumer tests pass (14 ignored); native Shogi 19; search lib tests and both Texel example tests pass.

Integrated into main by merge 20cdace1 (branch codex/msh-188-shogi-evaluation); pushed to origin/main.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
MSH-188 (Shogi Evaluation Round) replaces the basic Shogi evaluation with a Texel-tuned linear classical evaluation (133 weights: board and hand values with first-piece bonus, nine rank tables, King rank/file, slider mobility, King safety, tempo). INITIAL reproduces the old evaluation exactly (identical bench nodes); static terms are incremental, mobility and King safety per node via a king-zone attack map (about -10% nps). The Chess fitter became a shared Texel tool (TexelModel trait + examples/support/texel.rs) used by Chess and Shogi.
Corpus: cloud self-play of the frozen predecessor, 20,044 games, 501,006 unique quiet positions split by catalog opening (403k train / 55k validation / 43k held-out from unused openings); a second batch added seeded random plies after the first showed repeated deterministic games. Fit K=0.13; validation 0.22421 -> 0.21809, held-out 0.21891 -> 0.21398 (slightly worse with 0-2 hand pieces). A wider-bounds revision was rejected on validation.
Result: bench nodes -54%, timed depth equal or +1..+5; tactical suite 386 vs 382; sequential test vs predecessor H1 after 64 pairs (107-0-21, +283 Elo). Compute: conservative 41.57 VM-h / US$8.38. Risk: extreme fitted King placement (corner penalty, advanced-King bonus) shared by both test engines; checkpoint recorded in MSH-191. Docs: docs/m5/shogi-evaluation-round.md, evidence docs/m5/evidence/msh188. Integrated into main.
<!-- SECTION:FINAL_SUMMARY:END -->
