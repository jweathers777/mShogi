# Shogi evaluation round

MSH-188 (Shogi Evaluation Round) replaces the basic Shogi evaluation with a classical, Texel-tuned linear evaluation. The frozen predecessor is task-start main `578748d6` (engine source `a9188ebe`, the MSH-190 result); the candidate is `da19820583a90e61cb081b5b49cfbc7d31fc4f35`. **The candidate passed the sequential predecessor test after 64 pairs: 107 wins, 0 draws and 21 losses.** Every game passed native replay. Evidence is in [evidence/msh188](evidence/msh188/).

Native rules remain the authority for moves, histories and outcomes. Labels are native game outcomes, never search scores. No search change is part of this task; the evaluator identity `mshogi.shogi.material-placement.v1` is kept so that saved observations stay readable.

## Evaluation

The evaluation is linear in 133 integer weights ([`shogi/eval.rs`](../../crates/mshogi-search/src/shogi/eval.rs)). Each feature is the side to move's value minus the opponent's ([`ShogiPosition::features`](../../crates/mshogi-search/src/shogi/position.rs)):

| Group | Weights |
| --- | --- |
| Board value per face (13) | Rook … tokin |
| Hand value per piece (7) and first-piece bonus (7) | By hand kind |
| Rank tables (81) | Nine groups (Pawn, Lance, Knight, Silver, Gold-like, Bishop, Rook, Horse, Dragon) × nine advanced ranks |
| King placement (14) | Advanced rank; file distance from the centre (castle files) |
| Slider mobility (5) | Lance, Bishop, Rook, Horse, Dragon reachable squares |
| King safety (5) | Own pieces next to the King; attacked squares next to the King; enemy pieces within two squares; own Golds, promoted minors and Silvers within two squares; enemy non-Pawn hand pieces × empty squares next to the King |
| Tempo (1) | Side to move |

`INITIAL` reproduces the earlier evaluation exactly: an engine with `INITIAL` weights searches exactly the predecessor's 2,400,987 benchmark nodes. Board, rank, King-square and hand terms are incremental; mobility and King safety are computed per node with one enemy attack map around the King. A random-walk test checks that the incremental evaluation equals the full feature sum and that the attacked-square count equals the native `is_square_attacked` result.

## Shared Texel tool

The Chess fitter became the shared tool [`examples/support/texel.rs`](../../crates/mshogi-search/examples/support/texel.rs) over the `TexelModel` trait ([`texel.rs`](../../crates/mshogi-search/src/texel.rs)): corpus rows `{position, result, split, game_id}`, K calibration on the initial weights, bounded coordinate descent on training rows, validation selection of completed sweeps, and a separate held-out check. Chess (`chess_texel`) and Shogi (`shogi_texel`) supply only their models; Chess keeps its K range, steps and bounds. The Shogi model skips positions that are not quiet: in check, or with a capture or promotion that wins material by static exchange. `stats` reports quiet counts and label balance.

## Corpus

Self-play of the frozen predecessor on Google Cloud Batch ([`shogi_selfplay.py`](../../tools/strength/shogi_selfplay.py)), c3-standard-4, eight VMs, four pairs per VM, all 315 MSH-190 openings. Each pair draws one fixed node budget per player; each report is replayed natively on the worker and exported with Sente's native outcome for every position.

| Batch | Pairs | Budgets | Wall | Conservative compute |
| --- | ---: | --- | ---: | ---: |
| msh188-corpus | 5,622 of 6,400 (778 skipped at the 3 h task bound) | 24k–48k nodes | 3.47 h | 22.78 VM-h, US$4.59 |
| msh188-corpus2 | 4,400 | 20k–40k nodes, plus two seeded random plies after the catalog prefix | 2.55 h | 16.46 VM-h, US$3.32 |

The first batch alone gave only about 188,000 unique quiet positions: deterministic fixed-node self-play repeated whole games from the same opening. The second batch adds two seeded random legal plies so that repeated openings diverge. The splitting tool groups games by catalog opening id, so held-out games come from openings that training never used.

Totals: 20,044 games (877 unfinished at their node budget and unlabeled), 2,216,012 exported rows; 32,295 cross-split and 723,177 within-split duplicate rows removed. **Unique quiet positions: 501,006**: training 403,044 (11,929 games, 252 openings), validation 54,953 (1,637 games, 35 openings), held-out 43,009 (1,253 games, 28 openings). Training labels (Sente loss/draw/win): 188,289 / 8,701 / 206,054. Raw reports and derived files stay outside Git; [generation.json](evidence/msh188/corpus/generation.json) records the GCS prefixes and file hashes.

Fixed budgets below about 15,000 nodes sometimes completed no iteration in drop-rich positions (the MSH-190 depth-one cost; timed play keeps its legal fallback). The chosen budgets avoid most of these; unfinished games give no labels.

## Fit and held-out result

K = 0.13 (calibrated on the initial weights; the untuned material-heavy evaluation predicts Shogi outcomes weakly). Training MSE 0.22412 → 0.21733; validation 0.22421 → 0.21809, selected after 11 sweeps by validation patience. 32 weights ended on a bound. One bounded revision with wider bounds for those weight types reached validation 0.21832 and was rejected. Held-out (read once): **0.21891 → 0.21398**. By pieces in both hands: 0–2: 0.23059 → 0.23141 (slightly worse); 3–7: 0.21663 → 0.20908; 8 or more: 0.19129 → 0.17535. See [fit](evidence/msh188/fit.json), [rejected fit](evidence/msh188/fit-wide.json) and [held-out check](evidence/msh188/heldout-check.json).

Notable fitted values: hand Pawn below a board Pawn, tokin 404, King safety terms at their bounds (adjacent attacked −80, nearby enemy −80, nearby defenders +80), a King on castle files (distance 3 from the centre) preferred, and a large bonus for a King advanced beyond the fourth rank (entering-king positions in the corpus).

## Measurements

Local, Apple M1 Pro, Rust 1.99.1 release:

| Measure | Predecessor | Candidate |
| --- | ---: | ---: |
| Fixed-depth bench nodes | 2,400,987 | 1,113,070 |
| Nodes per second | 1.22 million | 1.09 million |
| Depth at 1000 ms (nine roots) | 15, 16, 11, 13, 11, 6, 64, 15, 11 | 15, 17, 16, 16, 14, 8, 64, 16, 11 |
| Tactical suite at 1000 ms | 382 / 413 | 386 / 413 |
| — development / held-out | 271 / 111 | 271 / 115 |

The per-node terms cost about 10% speed; the tuned evaluation shrinks fixed-depth trees by 54%, so timed depth rises on every ordinary root.

## Game test

Frozen protocol, as MSH-190: 10 s + 0.5 s, 315-opening catalog (pair N = opening N−1), SPRT H0 = 0, H1 = 10 Elo, α = β = 0.05, minimum 30 and maximum 315 pairs, decisions after complete batches. Google Cloud Batch, c3-standard-4 Sapphire Rapids, eight VMs, two pairs each. Image `worker@sha256:72310483120ea59dac3b4e211053352cceeb50c2f89ff15167a061d34d737520`; candidate SHA-256 `9e9fabd9…`, predecessor `a29f5bd7…`.

| Batch | Pairs | Wall | Cumulative W–D–L | LLR |
| --- | --- | ---: | --- | ---: |
| b01 | 1–32 | 552 s | 56–0–8 | 2.39 |
| b02 | 33–64 | 542 s | 107–0–21 | **4.10 (H1)** |

Pentanomial [2, 0, 17, 0, 45]; Elo estimate +283 (95% interval 209 to 387); cloud late-game median depth 10 for both; no failures; all 128 games replayed on the host. Conservative compute 2.34 VM-h, US$0.47; jobs verified absent. Task total: 41.57 VM-h, US$8.38 conservative. The result shows improvement against the frozen predecessor under this control and catalog; it is not an external rating.

## Verification and limits

Level 1. Workspace Clippy with warnings denied passed; release tests for search, engine, console, desktop session and runner passed 857 with 14 ignored; native Shogi tests passed 19. Three toy fixtures (two search integration tests and one USI test) now place Sente's King on its castled square: the tuned King terms make a corner King step worth more than a depth-one promotion there, which those tests do not test. The cross-game fixed-search fingerprint changes only for Shogi.

Limits: the corpus is self-play of one engine at fixed budgets and may carry its style; the slight loss in hand-poor positions and the advanced-King bonus may need attention in later rounds; weights are untested against external engines (MSH-191, Shogi Strength Anchor).
