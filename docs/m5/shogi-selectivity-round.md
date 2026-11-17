# Shogi selectivity round

MSH-190 (Shogi Selectivity Round) adds standard selectivity and a mate search to the shared search core and feeds them with Shogi data. The frozen predecessor is task-start main `f70febd23088fc3af661fce756bb8ecbe38f7f55`. **The set (`15bbf1a0`) passed the sequential predecessor test after 64 pairs: 95 wins, 3 draws and 30 losses. The final source (`a9188ebe`), with review fixes to the mate search, passed the same test again after 32 pairs: 49 wins, 2 draws and 13 losses.** Every game passed native replay. Evidence is in [evidence/msh190](evidence/msh190/).

Native rules remain the authority for real moves, histories, claims and outcomes. A proven mate is private analysis; the adapter still admits the chosen move through the native rules.

## Measurement additions

**Openings.** [`shogi-openings-v2.json`](../../tools/strength/data/shogi-openings-v2.json) has 315 distinct openings in 21 families. They are original CC0 USI transcriptions of conventional structures (the same rights basis as MSH-186). Each opening combines one conventional trunk with one common development plan per side; they are not copied from professional games or books, so "established line" here means established structure, not a quoted game record. [`make_shogi_openings.py`](../../tools/strength/make_shogi_openings.py) extends 22 trunks by four sente and four gote development plans: 352 candidates, 348 legal after one corrected plan. The admission rule was fixed before screening: the predecessor's depth-9 score must be within ±150. All 16 side-pawn-capture candidates failed (+580 to +2022); 315 were admitted and shuffled with seed 190. Pair N plays opening N−1, so a campaign of at most 315 pairs repeats no opening. Screen scores: [opening-screen.json](evidence/msh190/opening-screen.json).

**Tactical suite.** [`shogi-tsume-v1.json`](../../tools/strength/data/shogi-tsume-v1.json) has 413 unique forced-mate positions (mate in 3 to 11 plies) from 123 decisive games in 68 native-replayed MSH-187 reports ([`shogi_tsume mine`](../../crates/mshogi-runner/examples/shogi_tsume.rs)). Each position is a diagram without earlier history. Its answer set is every checking first move that mates within the shortest mate length. A memoized AND-OR prover over the native `Position` ([`shogi_mate_proof.rs`](../../crates/mshogi-runner/examples/support/shogi_mate_proof.rs)) establishes the answers; it does not use the search crate. Native re-validation of all 413 answer sets passed ([receipt](evidence/msh190/tsume-validation.json)). The split is by source game: 283 development and 130 held-out positions. A minimal mate of at most 11 plies cannot contain a fourfold repetition, so repetition does not change these answers. The suite measures checking mates only; quiet winning moves are outside the metric.

**Control.** A local pilot at 10 s + 0.5 s gave median depth 12 for both engines in the second half of each side's turns. In the cloud test the same measure was 10 (candidate) and 11 (predecessor), above the required 8.

## The selectivity set

All techniques are in the shared core (`crates/mshogi-search/src/alphabeta`). Shogi supplies only data through `SearchPosition`. Other games keep their behaviour: the cross-game fixed-search fingerprint is unchanged except Shogi (121 → 122 nodes, same move and score).

| Item | Core | Shogi data | Outcome |
| --- | --- | --- | --- |
| Scaled margins | `MARGIN_UNIT` scales futility, reverse futility, razoring, null-move, singular, aspiration and quiescence-delta margins (100 = previous Chess values) | 200: the smallest capture moves two Pawn values (board and hand) | Retained in the set |
| Quiescence checks | At the first quiescence ply only, after the tactical moves: quiet checks from `generate_checks` with exchange ≥ 0, at most `QUIESCENCE_CHECKS` tried, verified with `make`/`in_check`, not delta-pruned; the reply is an ordinary in-check node, so the stage never recurs | Cap 8; complete check generator including drops, promotions and discovered checks | Retained in the set |
| Full SEE | `exchange_at_least` orders good captures first, defers losing ones and filters quiescence captures and checks | Swap-list SEE with board and hand values, least valuable attacker, promotion on recapture, x-rays, king last | Retained in the set |
| Drop and promotion ordering | `quiet_order` adds a game bonus to the history score | Direct check +6000, drop within two squares of the enemy king +2000, non-promotion of Pawn, Bishop or Rook in the zone −20000 | Retained in the set |
| Evasion drops | — | In check, `generate_search` keeps only drops between the king and a single sliding checker (legal set unchanged) | Retained (speed only) |
| Mate search | `alphabeta/mate.rs`: df-pn with 1+ε thresholds over game-supplied checks and ordinary evasions | `MATE_SEARCH = true` | Root use retained; in-search probe rejected |

The earlier MSH-187 failure came from an unbounded check stage. This stage is bounded by construction: one quiescence ply, a fixed cap and no recursion. No iteration failed to complete in the 128 cloud games.

### Mate search

The df-pn search plays only checking moves for the attacker and every legal reply for the defender; the defender is mated when it is in check with no legal move. `make` decides legality, so the Pawn-drop-mate rule holds by construction. A child that a game rule decides (`rule_score`: repetition, perpetual check, move limit) is not a mate. That child is not stored, but a parent's stored disproof can depend on it; such disproofs can only hide a mate, and the table is cleared before each root search so they do not carry into later moves. The table has buckets of four; a solved entry is not evicted by unsolved work. A parent uses each child's returned numbers, so a dropped table entry cannot make it select the same child again without progress. A proof line follows strictly decreasing stored proof lengths (shortest attacker move, longest defence), so it cannot cycle.

At the root the search uses at most 1/8 of the soft time or node budget (131,072 nodes without either). A proven line is kept, the iterative deepening still runs, and its mate is preferred when it is no longer: df-pn proofs are not shortest. The first candidate returned the df-pn line at once and played longer mates (mate-in-3 cases fell from 117 to 93); this change fixed it.

Alone, on all 413 suite positions with 100,000 nodes each and a fresh 2^20-entry table, the final df-pn (`a9188ebe`) finds a forced mate in **413** cases; every line replays as checkmate under the native rules, with no false mate. Median time is 0.56 ms, maximum 56 ms. In 343 cases the first move is in the shortest-mate answer set; the others are longer forced mates. Under table pressure, one 2^18 table shared by all positions also solves 413, and one shared 2^12 table solves 378. In the other 35 the root was already proven from earlier positions' entries, but evictions broke the line, so the result is Unknown (never a false mate). Play clears the table before each root search, so this case does not arise there. See [df-pn receipts](evidence/msh190/).

**In-search probe, rejected.** A 128-node df-pn at non-PV nodes of depth ≤ 4 raised fixed-depth bench time about five times and lowered timed depth by 1 to 4 plies. The suite fell from 380 to 376 (development 269 → 259, held-out 111 → 117). The quiescence check stage already finds short mates at the horizon. The experiment is preserved as [a diff](evidence/msh190/mate-probe-experiment.diff); it is not in the source.

## Results

Local measurements: Apple M1 Pro, arm64 macOS, Rust 1.99.1 release, one fresh process per position.

| Tactical suite, 1000 ms | Predecessor | Game-tested set | Final source |
| --- | ---: | ---: | ---: |
| All 413 | 362 | 380 | 382 |
| Development 283 | 251 | 269 | 271 |
| Held-out 130 | 111 | 111 | 111 |
| Mate in 3 / 5 / 7 | 117 / 106 / 86 | 118 / 107 / 86 | 118 / 109 / 86 |
| Mate in 9 / 11 | 35 / 18 | 46 / 23 | 45 / 24 |

The gain is in the development part; the held-out result is unchanged (111 of 130 for all three). Most remaining misses of the set are forced mates that are longer than the shortest one. A secondary native check proves whether a missed checking move still forces mate within 11 plies: with it, 375 (predecessor) against 391 (set) positions are won by force. Quiet moves cannot be checked this way.

The fixed-depth bench rose from 1,594,093 to 2,418,072 nodes at equal speed (1.14 million nodes per second), because the Shogi margins are wider and quiescence includes checks. Timed depth at 1000 ms is about one ply lower on several roots (for example central rook 11 → 10, active rooks with hands 7 → 6). The game test shows that the selectivity and tactics outweigh this cost.

## Game test

Protocol, recorded before submission: 10 s + 0.5 s Fischer clocks, 315-opening catalog with colours reversed in each pair, sequential pentanomial test H0 = 0, H1 = 15 Elo, α = β = 0.05, minimum 30 and maximum 315 pairs, decisions only after complete batches in seed order. An unfinished pair stops the campaign and is never scored.

Execution: [`shogi_campaign.py`](../../tools/strength/shogi_campaign.py) on the existing Google Cloud Batch path, `us-central1`, c3-standard-4 Intel Sapphire Rapids, eight VMs with two concurrent pairs each, no retries. Linux x86-64 binaries were built from clean clones in the pinned MSH-187 compiler image with `-C target-cpu=x86-64`. Image `worker@sha256:0becf2e3afe19cc6187ae0eb2d55f105bf797d8c36646dcce56ddc76cc14ca72`, worker tools `1ebe7de9`.

| Batch | Pairs | Wall | Cumulative W–D–L | LLR |
| --- | --- | ---: | --- | ---: |
| b01 | 1–32 | 678 s | 46–2–16 | 1.28 |
| b02 | 33–64 | 727 s | 95–3–30 | **3.37 (H1)** |

Pentanomial [5, 0, 20, 3, 36]; Elo estimate +194 (95% interval 128 to 277). The host replayed all 128 games through the native authority. The conservative compute upper bound is 3.02 VM-hours and US$0.61 (eight VMs over each batch interval at US$0.201608 per VM-hour), within the recorded US$4 cap. Owned jobs are verified absent. The result shows improvement against the frozen predecessor under this control and catalog; it is not an external rating.

**Final source.** The independent review found mate-search defects after this test: proof-line extraction could follow a table proof into a repetition; replace-always storage could evict part of a proof tree; and when the table dropped a result, a parent could select the same child again until its budget ended. The final source `a9188ebe2923e6b91e5434f3baaaf693b6309f94` stores proof lengths, uses buckets that keep solved entries, uses each child's returned numbers and clears the proof table before each root search. Because these changes alter play, the final source was tested again under the same frozen protocol as a separate experiment (`msh190-final`, openings again from pair 1):

| Batch | Pairs | Wall | W–D–L | LLR |
| --- | --- | ---: | --- | ---: |
| f01 | 1–32 | 739 s | 49–2–13 | **2.98 (H1)** |

Pentanomial [0, 0, 13, 2, 17]; Elo estimate +221 (95% interval 143 to 327); cloud late-game median depth 10 / 11; no failures; all 64 games replayed on the host. Conservative compute 1.59 VM-hours and US$0.32; jobs verified absent. Both experiments together used a conservative 4.61 VM-hours and US$0.93. Linux candidate SHA-256 `a29f5bd7c2c2ac88d1a43f615819fd88486889ad7f818a2254784612401c3014`, image `worker@sha256:2a0627af684ac480060a2f3b9e2f1bd4804e005874ff3483db0f8a6a390c91be`. Receipts: [cloud](evidence/msh190/cloud/).

## Verification

Level 1 (private search and evaluation), with the Linux worker run as targeted platform evidence for the measurement only. At `a9188ebe`: workspace Clippy with warnings denied passed; release tests for search, engine, console, desktop session and runner passed 857 with 14 ignored; native Shogi core tests passed 19; Python strength tools passed 138. New tests: check-generator completeness and evasion-drop equivalence in the random-walk differential, fixtures for discovered, promotion, single-slider, adjacent and double checks, hand-computed SEE cases, df-pn on the MSH-186 tsume with native replay, Pawn-drop mate and perpetual-check cycles, df-pn progress with a 16-entry shared table (fails before the review fix), and the cloud manifest policy. The cross-game fixed-search fingerprint changes only for Shogi (121 → 122 nodes, same move and score). Independent review: a fresh-context reviewer read the source diff, tools and evidence; its three findings (df-pn non-progress when the table drops a result, the ply-bound variant of it, stale df-pn evidence) are fixed above, and its risks are recorded in the task. Windows and packaged desktop checks were not run.

## Follow-up

MSH-188 (Shogi Evaluation Round) uses these openings and this suite and starts from this engine.
