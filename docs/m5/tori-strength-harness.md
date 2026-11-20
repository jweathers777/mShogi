# MSH-204 (Tori Strength Harness)

The first step of the [per-game classical strength program](strength-roadmap.md#per-game-classical-strength-program-john-2026-11-18) for Tori Shogi. Tori uses the existing clocked native experiment route; no text protocol and no external engines (John, 2026-11-19). Evidence: [evidence/msh204](evidence/msh204/).

**Results.** 218 balanced openings; a 502-position tactical suite (124 held-out) with native-prover answers; the 5 s + 0.1 s control reaches median late-game depth 10 in real games (fresh search: 8); a doubling of time is worth about **+330 Elo** (approx. 95% 266–418, H1 after 96 pairs); Texel export and split work. Cloud cost about US$0.70.

## Route and infrastructure changes

Smoke games first ended unscored, so several faults were fixed before any measurement:

- **Clock allocation.** The runner gave the engine the 3× hard ceiling as a fixed move time, which spent the base time in about 20 moves. The configuration now records its policy (`clock.allocation`): `nominal` for Tori; absent means the earlier `hard` policy, so committed Shogi reports (MSH-186) replay unchanged ([roadmap note](strength-roadmap.md#clock-allocation-policy-of-the-native-experiment-runner)).
- **Request overhead that grew with game length.** Tori now uses the incremental session protocol (MSH-189), a native `Game::fork` (copy without replay), and admission computes the encoded size from cached event lengths. Experiment reports omit the per-step record that native replay restores, so reports grow linearly (a long pair: 4.6 MB → 40 KB; replay 75 s → 1.75 s).
- **Game length.** The recorded native limit ended about 8% of self-play games unscored at about 360 plies. This led to MSH-208 (Tori Repetition Rule): John's repetition rule, and a limit of 1,024 entries. The runner's step cap is now 1,024 (Tori default 1,000).
- **Length-limit rule**, fixed before any result: a Tori pair in which a game reaches the step limit is excluded from the test and counted; any other unscored stop stops the test; the test stops when more than 10% of at least 20 pairs are excluded (`native_match.py`).
- **Time odds:** an optional per-participant clock in the runner; replay uses it (`--candidate-time-factor`).
- **Cloud build:** Linux binaries are compiled on Google Cloud Batch from a commit archive with vendored dependencies (`cloud_build.py`, about 60 s, US$0.004). Heavy work does not run on the owner's Mac (John, 2026-11-20).

## Openings

[`tori-openings-v1.json`](../../tools/strength/data/tori-openings-v1.json): **218 openings**, project-generated ([writer](../../tools/strength/make_tori_openings.py), [generator](../../crates/mshogi-search/examples/tori_openings.rs)). Rule fixed before screening: 8 plies from the standard root; each ply chooses (seeded) among actions within 50 of the best depth-6 score; distinct final positions; screen at fixed depth 12; admit |score| ≤ 100 Swallow hundredths. 300 candidates, 82 unbalanced; admitted mean +2.9. All positions have Sente to move; games are paired by colour. Not a certified equal-evaluation set.

## Tactical suite

[`tori-tactics-v1.json`](../../tools/strength/data/tori-tactics-v1.json): **502 positions** (378 development, 124 held-out; split by source report, so the two games of an opening pair stay together): 150 with a 3-ply, 152 with a 5-ply and 200 with a 7-ply forced win.

- **Source:** 112 self-play pairs at 1 s + 0.02 s from the opening catalog (224 games, all concluded natively except one at the 1,000-step limit; [source](evidence/msh204/mining-source.json)). Before every move a native prover ([`tori_mate_proof.rs`](../../crates/mshogi-runner/examples/support/tori_mate_proof.rs), local Tori rules, drops included, independent of the search) looked for a forced win by checks of 3–7 plies: 568 rows; at most three per game, longest first, equal positions once; positions with a one-ply win excluded.
- **Answers and scoring:** each row stores every checking first action that wins with a checking attacker. An action outside that list passes only when the native full-width prover (every attacker action, every defender reply) proves that it wins within the mate length. This is checked when a move is played (`tori_strength probe`), not precomputed for every action: the pass rule is the same as with complete answer lists, at a small fraction of the cost. Proofs are stored under (SHA-256 of the canonical diagram, action, mate length, proof bound, prover version) and reused by later runs ([store](../../tools/strength/data/tori-tactics-v1-proofs.json), 200 proofs).
- **Tests:** the prover agrees with plain recursion on generated positions; the per-move proof agrees with a plain full-width recursion on all 624 actions of 12 positions with real three-ply wins, and on every checking action of 3 positions with five-ply wins; a prover reused across depths gives the answers of fresh provers; every checking answer is a full-width win.
- **Selection limit:** mining dropped positions whose checking proof exceeded its bound (400,000 visits) without counting them, so very hard mates are under-represented.

**Baseline** (cloud, c3-standard-4, 4 parallel fresh searches; [results](evidence/msh204/cloud/)):

| Budget | All | Development | Held-out | Proved wins outside the list |
| --- | --- | --- | --- | --- |
| 20 ms | 346 / 502 (69%) | 259 / 378 | 87 / 124 | 26 |
| 200 ms | 484 / 502 (96%) | 362 / 378 | 122 / 124 | 32 |
| 1,000 ms | 501 / 502 (99.8%) | 378 / 378 | 123 / 124 | 34 |

No proof reached its bound. The suite discriminates at 20 ms and is nearly saturated at playing budgets. As for Xiangqi, it is a correctness and regression guard; the judged blunder screen will be the discriminating tactical measure.

## Game-test control and depth

**Control: 5 s + 0.1 s per side** (nominal allocation). 56 self-play pairs on c3-standard-4 (two games per VM at a time): 111 games concluded natively, 1 reached the 1,000-step limit (1 length-limited pair, 1.8%); median 179.5 plies; median 114 ms per move. In these games mShogi reaches **median depth 10 from ply 100 on** (10th percentile 8; 12,499 moves, ply counted within each game after the opening). A fresh search (empty table, 4 parallel) of 948 positions from ply 100 on reaches median depth 8 at 100 ms (10th percentile 7). The control meets the program rule (median late-game depth ≥ 8). A pair takes about one minute.

## Time odds

Same engine, candidate at 10 s + 0.2 s against 5 s + 0.1 s, 96 pairs: pentanomial [1, 0, 23, 0, 72] (Tori has no draws, so pair scores are 0, ½ or 1), **+330 Elo** (approx. 95% 266–418), SPRT (0, 15) H1, evaluated once on the fixed 96-pair sample (not sequentially). All 192 games concluded natively; no pair was excluded. Speed is worth a great deal at this control, much more than in Xiangqi (+209), so the selectivity round (MSH-205) has a large possible benefit.

## Texel export

`tori_strength export` writes native-labelled rows (every 4th position after the opening, Sente result); unscored games keep no samples. `texel_data.py split --game tori --by-opening` (Tori compact diagrams). Check on 256 local games: 12,880 positions; 405 within-partition duplicates removed; train 9,995, validation 383 (only 6 games), held-out 2,097; no cross-partition duplicates ([manifest](evidence/msh204/texel/split-manifest.json)). The tuning corpus is MSH-206 (Tori Evaluation Round) work.

## Compute

Cloud (Google Cloud Batch, us-central1, c3-standard-4, US$0.201608 per VM-hour; [cost](evidence/msh204/cloud/cost.json)): build 1 VM × 70 s; smoke 4 VMs × 348 s; campaign 8 VMs × 1,383 s; about 3.5 VM-hours and **US$0.70** (upper bound, compute only); cleanup verified for each job. The worker image's frozen data are listed file by file in [data-files.json](evidence/msh204/cloud/data-files.json); the depth task used the 16 local control pairs (seeds 113–128) as late-game positions. Local (macOS arm64, before the 2026-11-20 rule): opening generation and screen, the 1 s self-play for mining and the first 16 control pairs, mining, and two prover runs that were stopped as wasteful (about 10 CPU-hours in all, mostly those two runs). Those local runs are not repeated, and later heavy work goes to the cloud.

## Limits

- The suite comes from mShogi's own fast games and is nearly saturated at 200 ms and above.
- Control depth and time odds are self-play measurements on one machine type; they do not give a rating.
- The control pairs are self-play of one binary (pentanomial [7, 0, 34, 0, 14], +44 Elo, interval includes 0): a check of colour balance and of the route, not a strength result.
- One long control game reached 1,000 plies; repetition cannot end such a game, so length-limited pairs stay possible.
