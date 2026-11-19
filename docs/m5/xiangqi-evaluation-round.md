# MSH-196 (Xiangqi Evaluation Round)

The evaluation step of the [per-game classical strength program](strength-roadmap.md#per-game-classical-strength-program-john-2026-11-18) for Xiangqi. **Result: a Texel-tuned evaluation of static terms (material and placement tables) passed its sequential test against the frozen predecessor after 40 pairs (H1); with a 40-pair confirmation batch, about +241 Elo (approx. 95% 187–308) at 5 s + 0.1 s. Two fits that also used per-node terms (mobility, piece value by phase, General-file pressure) lost their tests (about −33 Elo each) and were rejected.** Evidence: [evidence/msh196](evidence/msh196/).

## Baseline and plan (recorded before the work)

Predecessor: main `61d42fb9` (the MSH-195 engine). Its evaluation was material plus hand-set placement formulas for Soldier, Horse, Chariot and Cannon, and a tempo of 10. Test: sequential pentanomial test H0 = 0, H1 = 10 Elo, at most 600 pairs, 5 s + 0.1 s, xiangqi-openings-v1, eight concurrent games locally. Caps: cloud US$5 for the task.

## Evaluation

Linear in 80 integer weights ([`xiangqi/eval.rs`](../../crates/mshogi-search/src/xiangqi/eval.rs)), each feature the side to move's value minus the opponent's. Ranks count from the owner's back rank (0–9), files by distance from the centre file (0–4).

| Group | Weights | Final state |
| --- | ---: | --- |
| Material: Advisor, Elephant, Horse, Chariot, Cannon, Soldier | 6 | tuned |
| Rank (10) and file (5) tables for Soldier (file after crossing), Horse, Chariot, Cannon | 60 | tuned |
| General on rank 1 / rank 2 / off the centre file; central Advisor; central Elephant | 5 | tuned |
| Mobility of Horse, Chariot, Cannon; Horse and Cannon times major pieces; enemy Chariots and Cannons on the General's file; attacked palace squares next to the General | 8 | pinned at 0 (see below) |
| Tempo | 1 | tuned (fitted to 0) |

`INITIAL` reproduces the predecessor's evaluation exactly (identical fixed-depth moves and node counts verified). Static terms are kept incrementally per piece and square; the per-node terms are computed only when a compiled weight is non-zero, so the final engine does no per-node evaluation work. Tests: evaluation equals the feature sum on random games (also against a fresh position), and the Texel score equals the search evaluation.

## Corpus

Clocked self-play of the frozen predecessor on Google Cloud Batch ([`xiangqi_selfplay.py`](../../tools/strength/xiangqi_selfplay.py), worker image `msh196-61d42fb9`), 2 s + 0.05 s, 3,200 pairs (6,400 games, all 112 openings), every position exported with the native Red result. c3-standard-4 × 8: 55 minutes, 5.91 conservative VM-hours, **US$1.19** (smoke run of 16 pairs: US$0.08); cleanup verified. All 6,400 games ended by native conclusion. Split by catalog opening: 1,123,632 rows; after duplicate removal and the quiet filter (not in check, no capture that wins material by SEE), **631,802 unique quiet positions**: training 528,821, validation 46,516, held-out 56,465. About two thirds of the labels are draws (mostly natural-count draws). Provenance: [generation.json](evidence/msh196/corpus/generation.json).

## Fits and game tests

The shared Texel tool (`xiangqi_texel`), K calibrated on the initial weights (K = 0.25, inside the range 0.01–0.71; the first fit used the default range, whose lower end was 0.25), validation selection.

| Fit | Change | Validation MSE | Held-out MSE | Weights on a bound | Game test |
| --- | --- | ---: | ---: | ---: | --- |
| initial | — | 0.05881 | 0.06162 | — | — |
| fit1 | all terms | 0.05127 | 0.05180 | 16 | **H0** at 200 pairs, −33 Elo, [10, 63, 88, 33, 6] |
| fit2 | per-node terms skipped where zero; Advisor/Elephant lower bound 150 | 0.05225 | 0.05252 | 15 | **H0** at 200 pairs, −33 Elo, [12, 56, 93, 36, 3] |
| fit3 | as fit2, per-node terms pinned at 0 | 0.05201 | 0.05259 | 11 | **H1** at 40 pairs, +262 Elo, [0, 1, 8, 10, 21] |
| fit3 confirmation | pairs 41–80 | | | | +221 Elo (144–325), [1, 2, 9, 7, 21] |

Held-out loss improved in every phase bucket for every fit (fit3: 0.0434 → 0.0370 with 0–4 major pieces, 0.0991 → 0.0837 with 5–8, 0.1332 → 0.1176 with 9–12). The held-out file was read after each fit; selection used validation only.

**Decision path.** fit1 lowered the losses but lost its game test. fit2 was the one bounded revision: it removed the per-node cost where weights were zero and raised the Advisor and Elephant lower bound (fit1 had put both at 100, against 528 for a Horse, below classical relative values), and it also lost. Two failures of the same approach called for a controlled experiment rather than another adjustment: fit3 differs from fit2 only in pinning the per-node terms at zero. In fit1 and fit2 those terms had reached their bounds (Chariot mobility 30 per square, Horse and Cannon phase terms +10 per major piece) and added about 20% search cost. **Descriptive conclusion:** as fitted on this corpus, the per-node terms make play clearly worse; the cause is not established (candidates: large evaluation swings against Soldier-scaled pruning margins, correlation-driven fits on a draw-heavy self-play corpus, the speed cost). They stay in the code at weight 0, skipped by the search, for a later, better-designed attempt.

Notable fit3 values: Chariot 1,440 (at its upper bound), Horse 512, Cannon 498, Soldier 108, Advisor and Elephant 150 (at their lower bound); advanced Chariots and crossed Soldiers valued highly; a General off its home rank or file and central Advisor/Elephant slightly negative; tempo 0. Eleven weights end on a bound.

## Measurements of the final engine

| Measure | Predecessor | Final |
| --- | ---: | ---: |
| Late-game fresh depth at 100 ms, median / 10th percentile | 10 / 9 | 11 / 9 |
| Middlegame search, 4 s | depth 16 | depth 17 (same node rate) |
| Tactical suite at 200 ms | 373 / 373 | 373 / 373 |
| Tactical suite at 5 ms | 359–361 (two runs) | 347 |

The final build differs from the tested candidate only by skipping zero-weight work; fixed-depth moves and node counts are identical. Total compute: US$1.27 cloud (corpus and smoke) and about 1.2 hours of local game tests.

## Verification

Release suites for search, engine, runner, console and desktop session pass; Clippy and formatting are clean. The Xiangqi entry of the all-games fixed fingerprint changed to move `b0c2`, 217 nodes, score 0 (the other games are unchanged). Perft and native differential tests are unchanged and pass.

## Limits

The corpus is self-play of one engine at a fast control and is draw-heavy; the fitted values (for example the Chariot at its bound) may carry its style. The result is measured against this predecessor only, at one control; it is not a rating. The game tests at 5 s + 0.1 s ran on one machine (macOS arm64, eight concurrent games).
