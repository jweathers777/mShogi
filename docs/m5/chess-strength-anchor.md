# MSH-185 (Chess Strength Anchor)

**Draft checkpoint: bracketing complete; main results pending.** This report does not claim a final rating or task completion. It records the fixed method and the first 56 games. The first main batch, b08, stopped after a Mr Bob 0.8.0 process failure. Its scored results and failed attempts are under review; all cloud resources from that batch are absent.

The study estimates current Chess strength against independent rated engines after the classical search and evaluation work. It has no strength target. It does not qualify the engine for release or block Shogi work. The result will be a conditional CCRL-referenced index, **not an official CCRL rating or a human rating**.

## Final result — pending

| Required result | Status |
| --- | --- |
| Combined estimate and approximate 95% interval | Pending main results and final fit |
| Total scored games, complete pairs and singletons | Pending; bracket has 56 games in 28 complete pairs |
| Final W/D/L and per-release scores | Pending |
| Main-family estimates and spread | Pending; report any spread greater than 150 points |
| Prior sensitivity and stopping decision | Pending final fit |
| Final wall time, VM-hours and estimated compute | Pending |
| Final job, VM and disk cleanup | Pending; all seven bracket batches are cleared |
| Independent final report review | Pending |

Do not use the bracket estimate below as the final study result.

## Frozen engine and test conditions

Candidate and native runner source: `f433ea13ef0c6682d7c84388142dc9c8238e4e16`, task-start `main`. No production engine or game-rule change is part of this study.

- Candidate Linux binary SHA-256: `d9ea49d457b2865830a85e470fd4d9183ecef1f33261b9c92cbee3d210c440da`.
- Native runner SHA-256: `0807bb820f5f43889c2368c453e4cfaf03aa09dade1180182f43366ba6a14c2d`.
- Build: release, locked dependencies, Rust 1.99.1, portable x86-64 target. Official opponent sources use the recorded GCC/G++ 14.2 builds. Exact commands, source commits, binary hashes and failed build attempts are in the [build evidence](evidence/msh185/opponent-builds-final/README.md) and [deployed pool](evidence/msh185/build-pool.json).

| Condition | Fixed setting or observed bracket environment |
| --- | --- |
| Time control | 60 seconds plus 0.6 seconds Fischer increment for both engines |
| Search limits | UCI clocks and increments; no fixed move time, node limit or depth limit |
| Engine settings | Full strength, one search thread, Hash 64 MB, ponder and own book off |
| Tablebases | No external tablebase files |
| Execution | Google Cloud Batch; at most eight workers; one game at a time on each VM |
| Machine | c3-standard-4 for every game |
| Region and observed zone | us-central1; bracket workers in us-central1-a |
| Observed CPU | Intel Sapphire Rapids; Xeon Platinum 8481C at 2.70 GHz |
| Observed OS | Linux 6.6.159+, x86-64, glibc 2.41 |
| Boot image | batch-cos-stable-official-20261110-00-p01 |

Each worker checks the binary hashes. Options are sent only when advertised. Releases without a Threads option use one search worker. The actual handshakes and options are retained in the build evidence. The cloud x86-64 requirement supersedes the task's earlier generic ARM64 wording. Local Docker was used for builds and non-search checks; no match games ran on the Mac.

The initial worker image was `us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:6f050b230db16669df5df3aba93e94d5cad60ad8e35dfb49c7e1ffc7fddf73f1`. From b02, the deployed pool image was `us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:8290f64d7051548c5e86f62929c8f00e1504ceb77836c474832756224016b627`. Native engine, worker method, machine type and clocks remain fixed; the pool image adds opponents. Batch manifests bind the actual image and settings.

## Anchors and openings

All anchors come from the [CCRL Blitz complete list computed November 14, 2026](https://computerchess.org.uk/404/rating_list_all.html). The [archived HTML](evidence/msh185/ccrl-blitz-2026-11-14.html.gz) is frozen. Only single-CPU, 64-bit rows are used. No ratings from another list or multi-CPU row are mixed into the fit.

| Release | Anchor | Use at this checkpoint | Official source |
| --- | ---: | --- | --- |
| Stash 20.0.1 | 2511 | Initial bracket | [Stash](https://github.com/mhouppin/stash-bot) |
| Stash 21.0 | 2713 | Bracket and selected main family | [Stash](https://github.com/mhouppin/stash-bot) |
| Weiss 0.10 | 2846 | Bracket | [Weiss](https://github.com/TerjeKir/weiss) |
| Weiss 0.9 | 2649 | Selected main family | [Weiss](https://github.com/TerjeKir/weiss) |
| Mr Bob 0.8.0 | 2638 | Selected main family | [Mr Bob](https://github.com/bobsquared/Mr_Bob_Chess) |

The [candidate record](evidence/msh185/opponent-candidates.md) lists the wider pool and exact release commits. Multiple releases of one engine remain one family. Rated engines are not weakened or patched. Build-only compiler compatibility adjustments are recorded. Raven 0.80 was excluded before play for a verified upstream out-of-bounds write; it was not assigned a game loss.

Openings use the CC0 Stockfish [8moves_v3 book](https://github.com/official-stockfish/books), source commit `a804a97e709f40aa75a00f9d19e84915b7a3a45c`. Archive SHA-256: `7e1e9dd118b4bb97d8a8b5b8a790c86e21f8509d59a27d2883767d94477be02e`. The existing parser supplies 34,699 distinct valid 16-ply roots after one terminal root was excluded before play. A uniform shuffle without replacement, seed `msh185-anchor-2026-11-16-v1`, fixes 200 opening pairs in [openings.json](evidence/msh185/openings.json). Each pair uses one fresh root with reversed colors. All prefix moves are preserved. The book is generic; individual roots are not certified balanced.

## Native results and statistical method

The native rules decide legality, history, claims and outcomes, consistent with design boundaries A-01 and A-03. The runner uses automatic draw claims. There is no evaluation adjudication. Each scored result must pass native replay. Process failures, technical caps, incomplete evidence and harness-caused time losses remain unscored. Native clock losses require a separate timing review before admission. Retain every attempt and any valid singleton whose partner failed.

The [method fixed before play](evidence/msh185/method.md) fits one logistic expected-score Elo parameter to **all scored games**, including the bracket. Opponent ratings are fixed. Draws count as half a point. The prior is Normal(2500, 500²), with sensitivity checks at prior standard deviations 250 and 1000.

The opening pair is the uncertainty unit. The estimator uses a cluster sandwich variance and a conservative tabulated Student-t critical value. Before 30 pairs, or when all pair scores are identical, it uses the wider envelope with a pair-level profile approximation. The reported 95% interval is approximate. It is neither an exact posterior interval nor an anytime-valid confidence sequence. The [independent statistics review](evidence/msh185/statistics-review.md) records the formulas, tests and limits.

Anchor uncertainty, hardware and time-control differences, family style and opening selection add systematic error outside this interval. A narrow statistical interval does not remove those limits. Final family fits and prior sensitivity must remain visible beside the combined number.

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

All 56 games remain in the fit. They form 28 complete pairs: 20 wins, 17 draws and 19 losses. No bracket game failed, reached a technical cap, or needed clock-loss review. The provisional estimate is **2728.68**, with interval **2592.57–2860.37**. It is not the final result.

Bracketing stopped after **56.48 minutes** because another round was expected to exceed the one-hour phase limit. The +/-100 guide was not reached; the maximum half-width was 136.11. Seven rounds were needed instead of the usual three or four. The [independent bracket review](evidence/msh185/bracket-review.md) verifies the selection path, retained results and frozen openings.

The first main batch selects Stash 21.0, Weiss 0.9 and Mr Bob 0.8.0. Their anchors are 15.68, 79.68 and 90.68 points from the bracket estimate. They are the three closest **admitted/deployed families**, not the closest engines in the entire CCRL list or prepared reserve roster. b08 schedules 24 fresh pairs, eight per family, with opponents rotated across eight workers. See [main-selection.json](evidence/msh185/main-selection.json).

## Main results and changes — pending

The first main batch stopped at Mr Bob 0.8.0 with a SIGSEGV while the runner waited for `bestmove`. This is a technical failure, not a scored loss. Retain completed valid games and all failure records. The controller is reviewing the exact counts. The selected replacement is the prebuilt Supernova 2.3 reserve, anchor 2644; its admission and image digest must be recorded before further play. No engine patch or failed-game rerun is part of this replacement.

The unchanged Supernova 2.3 binary is recorded in [build-pool-v2.json](evidence/msh185/build-pool-v2.json), source `ffd139d1f3e56d39af41cb7cca78a5212bc96a88`. The replacement image digest is `sha256:72b4a8072b2d3b1db36b21183f2f51476108d408256f531b141dcfa8d37959ee`. Its saved handshake confirms Hash 64 MB and Ponder false. Source review confirms both UCI clocks and increments, one search thread, and a fresh history index for each position command (`Main.c:301,322,330`). This avoids the accumulated-history defect found in Mr Bob 0.8.0. It does not prove that every search path is free of defects; no extra search was run for this review.

| Main family | Final releases used | Main pairs | All-study W/D/L | Family estimate and interval |
| --- | --- | --- | --- | --- |
| Stash | Pending; starts with 21.0 | Pending | Pending | Pending |
| Weiss | Pending; starts with 0.9 | Pending | Pending | Pending |
| Mr Bob | Pending; starts with 0.8.0 | Pending | Pending | Pending |

**Pending:** insert each later batch result, release replacement and reason; include all original games. Report the final family spread and state explicitly whether it exceeds 150 points. Add final prior sensitivity and any scored singleton or excluded result with its reason.

After each main batch, refit all results and inspect release scores. Replace an opponent when its running score leaves about 25–75%. The precision stop requires half-width no more than 40 points, at least 30 pairs overall, and at least 20 main pairs in each main family. The study aims for about 300–400 games and cannot exceed 400 scored games. Resource limits can stop it earlier; incomplete precision or coverage must be reported plainly.

## Resources, failures and cleanup

The original campaign clock starts at the first cloud submission and never resets. The hard limits are **six hours of wall time** and **US$15 estimated compute**, including smoke work, failures and reruns. At most eight C3 workers are allowed. The controller reserves time for deletion; workers also enforce the absolute deadline. No automatic retry is enabled.

| Resource measure | Completed bracket only | Final campaign |
| --- | ---: | --- |
| Scored games | 56 | Pending |
| Phase elapsed time | 56.48 minutes | Pending |
| VM lifecycle hours | 3.661717 | Pending |
| Estimated VM compute | US$0.738231 | Pending |
| Observed workers per batch | 8 in each round | Pending |
| Failed VM insertions | 0 | Pending |
| Unscored game attempts | 0 | Pending |
| Job/VM/disk cleanup | All seven bracket batches cleared | Pending |

VM-hours run from insert request to successful delete completion, including startup, idle time and cleanup. They are separate from engine CPU time and the controller's conservative upper bound. Estimated compute uses US$0.201608 per VM-hour. It excludes storage, registry, network and other charges; actual billed cost is unknown. The [resource review](evidence/msh185/resource-review.md) verifies this distinction.

b02 first reported a controller cleanup timeout because the deleted VM's Batch job record remained in deletion. A later exact-job check proved the job, VMs and disks absent. The original failure is retained in [b02 closure](evidence/msh185/b02/closure.md). No game was discarded or repeated. The controller's bounded deletion-verification window was extended; native engines, worker and scientific method were unchanged.

**Pending:** add final wall time and lifecycle totals, allocation failures, unscored outcomes, any harness timing decisions, and final absence proof. Keep all original failure records.

## Comparison with earlier work

MSH-139 (Chess Strength Assessment) reported about 1900 on a conditional local CCRL-referenced index, with an approximate 1700–2200 range, at one second per move. It used an older engine and a November 2024 CCRL 40/15 anchor list. See the [assessment method and evidence](chess-strength-assessment.md).

MSH-161 (Competitive Gameplay Diagnosis) scored **52.5%** against Oxidation 0.6.0 on 180 prepared-opening games at five seconds per move. That opponent's recorded CCRL 40/15 anchor was 1943. The diagnosis was not an Elo study; its prepared and unaided opening groups were separate. See the [diagnosis](competitive-gameplay-diagnosis.md).

MSH-185 uses a later engine, a different dated rating list, Fischer clocks, cloud hardware and a different opening sample. Subtracting 1900 from its final estimate would not measure an Elo gain. The frozen-predecessor gains from MSH-164 (Shared Search Core), MSH-166 (Search Selectivity Completion), MSH-167 (Engine Time Management) and MSH-168 (Classical Endgame and King Evaluation) cannot be added to create an external rating.

**Pending:** state the final supported comparison after the main-family results are available. Separate evidence of stronger external play from any claim about the size or cause of the gain. Do not infer strength in Shogi or another target game from this Chess study.

## Evidence and completion — pending

[Evidence index](evidence/msh185/README.md), [method](evidence/msh185/method.md), [worker review](evidence/msh185/worker-review.md), [statistics review](evidence/msh185/statistics-review.md), [bracket review](evidence/msh185/bracket-review.md), and compact per-batch records preserve the measurement. Raw games, logs and immutable manifests are stored under `gs://mshogi-experiments-216675633801/msh185/`, with hashes in each batch's retrieval record.

Verification scope is measurement infrastructure and actual Linux cloud play. No new production-engine, GUI or three-platform release claim is made. Final independent review, the strength-roadmap result, task acceptance and integration remain pending. MSH-186 (Shogi Strength Harness) and MSH-187 (Shogi Strength Round) remain separate next work items.
