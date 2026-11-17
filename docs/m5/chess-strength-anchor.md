# MSH-185 (Chess Strength Anchor)

The final conditional CCRL-referenced estimate is **2750.51**, with an **approximate 95% interval of 2714.87–2786.15** at 60 seconds plus 0.6 seconds per move. This is not an official CCRL rating or a human rating.

The study retained **228 scored games in 114 complete opening pairs: 98 wins, 76 draws and 54 losses**. It stopped at the fixed precision rule: half-width 35.64 points, below 40, with 28, 30 and 28 main pairs for Stash, Weiss and Supernova. No scored singletons or rerun games were needed. The usual 300–400-game target was not needed because precision and family coverage were met earlier.

All campaign jobs, VMs and disks are absent. Final independent report review and task integration remain pending; see the planned [final review](evidence/msh185/final-review.md). The result has no strength target or release qualification threshold and does not block Shogi work.

## Frozen engine and test conditions

Candidate and native runner source: `f433ea13ef0c6682d7c84388142dc9c8238e4e16`, task-start `main`. No production engine or game-rule change is part of this study.

- Candidate Linux binary SHA-256: `d9ea49d457b2865830a85e470fd4d9183ecef1f33261b9c92cbee3d210c440da`.
- Native runner SHA-256: `0807bb820f5f43889c2368c453e4cfaf03aa09dade1180182f43366ba6a14c2d`.
- Build: release, locked dependencies, Rust 1.99.1, portable x86-64 target. Official opponent sources use the recorded GCC/G++ 14.2 builds. Exact commands, source commits, binary hashes and failed build attempts are in the [build evidence](evidence/msh185/opponent-builds-final/README.md) and [deployed pool](evidence/msh185/build-pool.json).

| Condition | Fixed setting or observed environment |
| --- | --- |
| Time control | 60 seconds plus 0.6 seconds Fischer increment for both engines |
| Search limits | UCI clocks and increments; no fixed move time, node limit or depth limit |
| Engine settings | Full strength, one search thread, Hash 64 MB, ponder and own book off |
| Tablebases | No external tablebase files |
| Execution | Google Cloud Batch; at most eight workers; one game at a time on each VM |
| Machine | c3-standard-4 for every game |
| Region and observed zone | us-central1; us-central1-a and us-central1-b; b07 and the main batches used zone b |
| Observed CPU | Intel Sapphire Rapids; Xeon Platinum 8481C at 2.70 GHz |
| Observed OS | Linux 6.6.159+, x86-64, glibc 2.41 |
| Boot image | batch-cos-stable-official-20261110-00-p01 |

Each worker checks the binary hashes. Options are sent only when advertised. Releases without a Threads option use one search worker. The actual handshakes and options are retained in the build evidence. The cloud x86-64 requirement supersedes the task's earlier generic ARM64 wording. Local Docker was used for builds and non-search checks; no match games ran on the Mac.

The initial worker image was `us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:6f050b230db16669df5df3aba93e94d5cad60ad8e35dfb49c7e1ffc7fddf73f1`. From b02, the deployed pool image was `us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:8290f64d7051548c5e86f62929c8f00e1504ceb77836c474832756224016b627`. This pool change added opponents without changing the native engine, worker method, machine type or clocks. The later classification-only worker change is recorded below. Batch manifests bind the actual image and settings.

## Anchors and openings

All anchors come from the [CCRL Blitz complete list computed November 14, 2026](https://computerchess.org.uk/404/rating_list_all.html). The [archived HTML](evidence/msh185/ccrl-blitz-2026-11-14.html.gz) is frozen. Only single-CPU, 64-bit rows are used. No ratings from another list or multi-CPU row are mixed into the fit.

| Release | Anchor | Use in this study | Official source |
| --- | ---: | --- | --- |
| Stash 20.0.1 | 2511 | Initial bracket | [Stash](https://github.com/mhouppin/stash-bot) |
| Stash 21.0 | 2713 | Bracket and selected main family | [Stash](https://github.com/mhouppin/stash-bot) |
| Weiss 0.10 | 2846 | Bracket | [Weiss](https://github.com/TerjeKir/weiss) |
| Weiss 0.9 | 2649 | Selected main family | [Weiss](https://github.com/TerjeKir/weiss) |
| Mr Bob 0.8.0 | 2638 | Initial main selection; technical crashes, no scored games | [Mr Bob](https://github.com/bobsquared/Mr_Bob_Chess) |
| Supernova 2.3 | 2644 | Replacement main family from b09 | [Supernova](https://github.com/MichaeltheCoder7/Supernova) |

The [candidate record](evidence/msh185/opponent-candidates.md) lists the wider pool and exact release commits. Multiple releases of one engine remain one family. Rated engines are not weakened or patched. Build-only compiler compatibility adjustments are recorded. Raven 0.80 was excluded before play for a verified upstream out-of-bounds write; it was not assigned a game loss.

Openings use the CC0 Stockfish [8moves_v3 book](https://github.com/official-stockfish/books), source commit `a804a97e709f40aa75a00f9d19e84915b7a3a45c`. Archive SHA-256: `7e1e9dd118b4bb97d8a8b5b8a790c86e21f8509d59a27d2883767d94477be02e`. The existing parser supplies 34,699 distinct valid 16-ply roots after one terminal root was excluded before play. A uniform shuffle without replacement, seed `msh185-anchor-2026-11-16-v1`, fixes 200 opening pairs in [openings.json](evidence/msh185/openings.json). Each pair uses one fresh root with reversed colors. All prefix moves are preserved. The book is generic; individual roots are not certified balanced.

## Native results and statistical method

The native rules decide legality, history, claims and outcomes, consistent with design boundaries A-01 and A-03. The runner uses automatic draw claims. There is no evaluation adjudication. Each scored result must pass native replay. In-play process failures, technical caps, incomplete evidence and harness-caused time losses remain unscored. Normal bounded post-game termination is distinguished below. Native clock losses require a separate timing review before admission. Retain every attempt and any valid singleton whose partner failed.

The [method fixed before play](evidence/msh185/method.md) fits one logistic expected-score Elo parameter to **all scored games**, including the bracket. Opponent ratings are fixed. Draws count as half a point. The prior is Normal(2500, 500²), with sensitivity checks at prior standard deviations 250 and 1000.

The opening pair is the uncertainty unit. The estimator uses a cluster sandwich variance and a conservative tabulated Student-t critical value. Before 30 pairs, or when all pair scores are identical, it uses the wider envelope with a pair-level profile approximation. The reported 95% interval is approximate. It is neither an exact posterior interval nor an anytime-valid confidence sequence. The [independent statistics review](evidence/msh185/statistics-review.md) records the formulas, tests and limits.

Anchor uncertainty, hardware and time-control differences, family style and opening selection add systematic error outside this interval. A narrow statistical interval does not remove those limits. The final family fits and prior sensitivity below accompany the combined number.

## Bracketing result and main selection

The initial Stash 20.0.1 choice near 2500 was recorded before play for its full increment handling. Subsequent rounds selected the nearest release in the admitted image pool after each cumulative fit.

| Round | Opponent and anchor | W/D/L | Cumulative estimate | Approximate interval |
| --- | --- | --- | ---: | --- |
| b01 | Stash 20.0.1, 2511 | 5/3/0 | 2742.66 | 2384.43–3191.01 |
| b02 | Stash 21.0, 2713 | 4/4/0 | 2834.42 | 2564.39–3140.01 |
| b03 | Weiss 0.10, 2846 | 3/1/4 | 2821.33 | 2604.74–3041.56 |
| b04 | Weiss 0.10, 2846 | 0/1/7 | 2741.95 | 2555.31–2919.15 |
| b05 | Stash 21.0, 2713 | 1/2/5 | 2697.16 | 2530.61–2853.99 |
| b06 | Stash 21.0, 2713 | 3/3/2 | 2707.94 | 2558.60–2850.34 |
| b07 | Stash 21.0, 2713 | 4/3/1 | 2728.68 | 2592.57–2860.37 |

All 56 games remain in the fit. They form 28 complete pairs: 20 wins, 17 draws and 19 losses. No bracket game failed, reached a technical cap, or needed clock-loss review. The bracket estimate was 2728.68, with interval 2592.57–2860.37.

Bracketing stopped after **56.48 minutes** because another round was expected to exceed the one-hour phase limit. The +/-100 guide was not reached; the maximum half-width was 136.11. Seven rounds were needed instead of the usual three or four. The [independent bracket review](evidence/msh185/bracket-review.md) verifies the selection path, retained results and frozen openings.

The first main batch selected Stash 21.0, Weiss 0.9 and Mr Bob 0.8.0. Their anchors are 15.68, 79.68 and 90.68 points from the bracket estimate. They are the three closest **admitted/deployed families**, not the closest engines in the entire CCRL list or prepared reserve roster. b08 scheduled 24 fresh pairs, eight per family, with opponents rotated across eight workers. See [main-selection.json](evidence/msh185/main-selection.json).

## Main results and final fit

All bracket and valid main games contribute to [b12-fit.json](evidence/msh185/b12-fit.json). A family remains one family when different releases were used.

| Family | Scored releases | Main pairs | All-study W/D/L | Estimate | Approximate interval |
| --- | --- | ---: | --- | ---: | --- |
| Stash | 20.0.1, 21.0 | 28 | 38/34/24 | 2748.29 | 2694.24–2802.34 |
| Weiss | 0.10, 0.9 | 30 | 34/18/24 | 2735.30 | 2666.44–2804.17 |
| Supernova | 2.3 | 28 | 26/24/6 | 2771.13 | 2637.09–2912.04 |

The family estimates span **35.83 points**, below the 150-point disagreement threshold. Supernova has only 28 pair clusters, so its reported interval uses the broader small-sample routing envelope. Its narrower sandwich interval, 2702.67–2839.59, is not substituted for that conservative interval. The combined fit has 114 clusters and uses the pair-cluster sandwich interval. Family spread alone does not measure all systematic uncertainty.

| Main batch | Scored games | W/D/L | Disposition |
| --- | ---: | --- | --- |
| b08 | 10 | 6/1/3 | Eight Mr Bob crashes excluded; 30 assignments never started |
| b09 | 18 | 10/4/4 | Includes all eight reviewed Supernova conclusions; 30 assignments never started |
| b10 | 48 | 22/13/13 | Complete |
| b11 | 48 | 20/23/5 | Complete |
| b12 | 48 | 20/18/10 | Complete; precision stop |

Per-release scores retain the bracket contributions and show the main opponents
within the 25–75% score guide:

| Release | W/D/L | Score | Release-only estimate | Approximate interval |
| --- | --- | ---: | ---: | --- |
| Stash 20.0.1 | 5/3/0 | 81.25% | 2742.66 | 2384.43–3191.01 |
| Stash 21.0 | 33/31/24 | 55.11% | 2747.29 | 2690.45–2804.12 |
| Weiss 0.10 | 3/2/11 | 25.00% | 2649.10 | 2323.46–2916.91 |
| Weiss 0.9 | 31/16/13 | 65.00% | 2754.29 | 2680.77–2827.82 |
| Supernova 2.3 | 26/24/6 | 67.86% | 2771.13 | 2637.09–2912.04 |

Prior sensitivity is small relative to the reported interval:

| Prior standard deviation | Estimate | Approximate interval |
| ---: | ---: | --- |
| 250 | 2748.83 | 2713.49–2784.16 |
| 500, primary | 2750.51 | 2714.87–2786.15 |
| 1000 | 2750.94 | 2715.22–2786.65 |

After each main batch, all results were refitted and release scores inspected against the 25–75% guide. The stop required at least 30 pairs overall and 20 main pairs per family, in addition to half-width no more than 40. The campaign remained below its 400-scored-game maximum.

## Technical failures and classification

Eight Mr Bob 0.8.0 games crashed with SIGSEGV during play and remain unscored. Source review found that repeated FEN-plus-history commands accumulated history in a fixed-size array. There is no Mr Bob performance estimate. The **60 never-started assignments** across b08 and b09 are distinct from these eight attempted failures. All valid results, failure records and skipped opening IDs remain recorded; no failed game was rerun.

The prebuilt Supernova 2.3 reserve, anchor 2644, replaced Mr Bob. Its source and binary were frozen before the failure. [build-pool-v2.json](evidence/msh185/build-pool-v2.json) and the [replacement review](evidence/msh185/replacement-review.md) record its Hash, clock, increment and history-reset checks. The admission image was `sha256:72b4a8072b2d3b1db36b21183f2f51476108d408256f531b141dcfa8d37959ee`.

The original b09 worker rejected eight Supernova games after native conclusions and successful replay because cleanup ended with SIGKILL. Supernova waits one second after `quit`; the frozen runner allows only 100 milliseconds before bounded termination. Independent [shutdown classification review](evidence/msh185/shutdown-classification-review.md) accepted a narrow measurement correction for the exact engine and runner identities. It requires a valid native conclusion and replay, a successful post-conclusion quit write, signal 9, no runner/diagnostic/cleanup failure, no outer timeout, and a settled process group. All eight b09 games passed, including five wins, one draw and two losses. Attribution rests on the full evidence and verified cleanup path; there is no direct signal-origin field.

Original reports, rejected receipts and the original fit remain unchanged. The [review mapping](evidence/msh185/b09-shutdown-reclassifications.json) binds each derived classification to original hashes; [b09-corrected-fit.json](evidence/msh185/b09-corrected-fit.json) retains 84 games. This changes neither native results nor in-play crash exclusions.

Before b10, worker measurement instrumentation changed to implement that predicate. The [image record](evidence/msh185/shutdown-image.json) pins digest `sha256:60163e4fb2f8f88a28320614b7c8106a3de9c37c5ba3cff5f20d00f64eb3e0cb` and exact worker, collector and test source hashes. Candidate, native runner, opponent binaries, clocks, openings and rating method remained unchanged. Worker instrumentation was not byte-identical across this transition.

## Resources and cleanup

The campaign used **13.183893 VM lifecycle hours**, with **US$2.657978 estimated compute**, including failed batches and cleanup. Wall time from first submission through final batch cleanup was **9571.96 seconds, about 2 hours 39 minutes 32 seconds**. This stayed within the original six-hour and US$15 compute caps. The campaign clock never reset. At most eight c3-standard-4 workers ran at once; no automatic retry was enabled. All 12 batches had eight observed workers and no failed VM insertions.

VM-hours run from insert request through successful delete completion, including startup, idle time and cleanup. They are separate from engine CPU time and controller allocation upper bounds. Compute uses US$0.201608 per VM-hour and excludes storage, registry, network and other charges. Actual billed cost is unknown. See the [resource review](evidence/msh185/resource-review.md) and each batch's lifecycle receipts.

b02 initially reported a controller cleanup timeout while its Batch job record remained in deletion. Later exact-job checks proved the job, VMs and disks absent. The original failure is retained in [b02 closure](evidence/msh185/b02/closure.md); no game was discarded or repeated. The deletion-verification window was extended without changing native play. The final live [cleanup check](evidence/msh185/final-cleanup.json) lists no campaign jobs, labeled VMs or labeled disks.

## Comparison with earlier work

MSH-139 (Chess Strength Assessment) reported about 1900 on a conditional local CCRL-referenced index, with an approximate 1700–2200 range, at one second per move. It used an older engine and a November 2024 CCRL 40/15 anchor list. See the [assessment method and evidence](chess-strength-assessment.md).

MSH-161 (Competitive Gameplay Diagnosis) scored **52.5%** against Oxidation 0.6.0 on 180 prepared-opening games at five seconds per move. That opponent's recorded CCRL 40/15 anchor was 1943. The diagnosis was not an Elo study; its prepared and unaided opening groups were separate. See the [diagnosis](competitive-gameplay-diagnosis.md).

MSH-185 uses a later engine, a different dated rating list, Fischer clocks, cloud hardware and a different opening sample. Subtracting 1900 from its final estimate would not measure an Elo gain. The frozen-predecessor gains from MSH-164 (Shared Search Core), MSH-166 (Search Selectivity Completion), MSH-167 (Engine Time Management) and MSH-168 (Classical Endgame and King Evaluation) cannot be added to create an external rating.

The result supports stronger external Chess play under this study's conditions. It does not establish a numerical Elo gain from the earlier studies or isolate which engineering change caused the improvement. It makes no strength claim for Shogi or another target game.

## Verification and review

All 96 strength-tool tests passed. The prior 28 Google Cloud runner tests also passed. The shutdown implementation review checked 27 focused worker/collector tests, negative classifications, all eight affected reports, and independent collection of the corrected fit. [Worker](evidence/msh185/worker-review.md), [statistics](evidence/msh185/statistics-review.md), [bracket](evidence/msh185/bracket-review.md), resource and shutdown reviews record their source scope and limits. Final independent report review remains pending; numerical results are final.

The [evidence index](evidence/msh185/README.md) links compact records. Raw games, logs and immutable manifests remain under `gs://mshogi-experiments-216675633801/msh185/`, with hashes in retrieval inventories. This establishes measurement infrastructure checks and Linux cloud play, not a new GUI or three-platform release qualification. MSH-186 (Shogi Strength Harness) and MSH-187 (Shogi Strength Round) remain separate next work items.
