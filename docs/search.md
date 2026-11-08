# Chess search

## Active conventional Chess engine

The `SearchCoordinator`, shared by UCI and supervised native execution, uses
`chess_conventional::{search, search_with_stop, search_with_time}`: search identity
`mshogi.chess.conventional.v1`, evaluator
`mshogi.chess.conventional-tapered.v1`. The implementation and bounded engineering
evidence are described in [the conventional engine checkpoint](m5/chess-conventional.md).
This is a classical, selective engine; no playing-strength claim follows from its
correctness tests or profiling.

Iterative deepening combines fail-soft principal-variation search, aspiration
windows, a bounded transposition table, capture/exchange ordering, killers and
quiet/capture/continuation history. Conventional mode adds capture/promotion
quiescence, check extensions, late-move reductions with re-search, guarded
futility/late-move/exchange pruning and verified null-move probes. Native checked
transitions resolve every real action. The private native search cursor retains
history, outcome and claim authority; hypothetical null branches cannot create
real history or native game conclusions. Null cutoffs require verification on the
real node, with low-material, claim, check, counter and mate-band guards.

Native outcomes and claims precede static evaluation. Scores use the root side's
perspective; draws are zero and native mate scores are `±(1_000_000 - ply)`.
Optional zero-valued claims retain tie precedence; `AutoClaim` remains mandatory.
A concluded root is visited at actual ply zero, returns `Finished` and an empty
variation. No static score or search horizon adjudicates a game.

Nominal `completed_depth` records the last completed iterative-deepening target.
`selective_depth` records the deepest actual visited board ply across all attempted
iterations, including an interrupted later iteration. Quiescence normally allows
16 further plies; checked nodes search native evasions beyond that allowance until
the absolute 128-ply ceiling. At that ceiling, native outcomes and claims still
precede an approximate static leaf, even in check. It is a finite-work cap, not a
proof that the position is quiet or a terminal result. A principal variation can
therefore exceed nominal depth. `SearchStats` exposes the main/quiescence visits
and algorithm counters.

Depth, total visits, cancellation and optional soft/hard thinking-time limits bound
work. The soft boundary stops starting new iterations; the hard deadline and stop
predicate are polled cooperatively at nodes and exchange-ordering work. Only a
completed iteration supplies a decision, score and variation. Interruption before
the first completion supplies none; a later interruption preserves the earlier
completed result. No fallback move or terminal result is invented. Native state
and the incremental evaluator unwind on success, interruption and errors. Timed
search explicitly refuses full native mate-reachability mode because its solver
cannot be interrupted by the search deadline; normal timed consumers use the
existing bounded M2 adjudication mode.

The table is fresh per request and capped at 4096 entries. A position hash is only
an index and ordering hint. Bound reuse checks an exact native key containing the
complete root context and action prefix, plus the remaining extension budget.
Different histories do not share score bounds, even if their boards match; this
conservative policy deliberately forgoes compatible transpositions. Synthetic
null probes have no real-history key. Mate scores are normalized on table storage
and retrieval.

`SearchMode::Exact` retains PVS, ordering and full-history-checked table reuse, but
disables quiescence, extensions and selective approximations. It is a diagnostic
fixed-depth backup with static leaves, tested against an independently written
exhaustive oracle. Conventional-mode tests cover tactical recaptures and check
evasions, native terminal/claim precedence, legal exchange estimates, history and
hash-collision separation, guarded null/selective paths, work limits, cancellation,
representation failure and restoration. These establish specific engineering
contracts; they do not establish playing strength.

## Historical positional candidate

The retained `chess_candidate::{search, search_with_stop}` API has search identity
`mshogi.chess.alpha-beta-capture-order.v1` and evaluator
`mshogi.chess.tapered-position.v1`. It is exact fail-soft alpha-beta with native
history, outcomes and claims, capture/promotion priority followed by lexical UCI
ordering, and no transposition table, quiescence or selective pruning. Its evaluator
is described in [evaluation](evaluation.md). Equal backed-up scores keep the first
ordered action, while optional claims retain precedence on zero-score ties.
The earlier formal study was stopped by owner decision; its preserved observations
are [historical evidence](m5/chess-strength.md), not the active engine's acceptance
bar. Both this API and the full-width/material baseline remain reference paths.

## Frozen full-width baseline

MSH-20 exposes `mshogi_search::search::{search, search_with_stop}` over an immutable `AdjudicatedGame`. Search clones the complete rules-owned context, including known history, initial FEN, claim policy and any exercised claim. Branching uses checked live apply/undo on that private clone. It does not cache scores by board identity. Identity: `mshogi.chess.full-width.v1`; evaluator: `mshogi.chess.material.v1`.

The baseline uses iterative deepening and full-width negamax, with no pruning, quiescence or transposition table. Moves are sorted lexicographically by their UCI text; equal scores retain the first move. Every node checks the rules-owned concluded outcome before claim policy and static evaluation. Draws score zero; checkmate wins score `1_000_000 - ply`, losses `-1_000_000 + ply`, where ply is distance from this search root. Other leaves use material centipawns from their local side's perspective, negated on each backup. Returned scores use the root side-to-move perspective. Faster wins and delayed losses are preferred. A zero assessment alone never concludes a draw. The search neither recognizes additional dead positions nor proves mating possibility beyond the rules profile.

An available claim and a concluded result have different return types. With `ExplicitOnly`, a valid current or intended-move claim is a zero-valued choice at every node, including the leaf. Moves scoring above zero can be preferred; ties prefer the first rules-ordered claim. With `AutoClaim`, the first rules-policy claim is required at the turn boundary even when a winning move exists. Search returns that `ClaimRequest`; it does not silently mutate or conclude the caller's game. The policy still requires the controller to exercise the claim. A claim's intended action is not played. A principal variation can end with a claim at its final leaf boundary; that claim is not counted as an extra board ply. A root already concluded by the rules returns `Finished(outcome)` and an empty variation.

`SearchLimits { depth, nodes }` sets a depth of 1–128 plies and an optional total visited-node budget. Node counts include roots, leaves, terminal nodes and repeated iterative-deepening visits. A budget of zero visits nothing. The result publishes only the last fully completed iteration. If a node limit or stop occurs before depth one completes, decision/score are `None`, completed depth is zero and the variation is empty. This means incomplete work, not a terminal game. A protocol that needs an immediate action must choose and label a legal fallback. Invalid depth and rules transition/representation failures (including counter overflow) are explicit errors; no partial score is passed off as an exact result. The caller remains unchanged on success, interruption and error.

`search_with_stop` polls a cheap nonblocking predicate before every node. It supports later cancellation/timed consumers without owning threads or I/O. Latency is bounded by one node's rules work, rather than a fixed wall-time guarantee. Node/depth limits, policy, engine/evaluator revision and initial context determine deterministic reruns. A wall-time predicate may stop on a different node when rerun; preserve its configured limits, timing and observations without promising identical reruns. Request identities, stale-result rejection and supervising worker lifetimes are MSH-21 responsibilities.

```rust
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy};
use mshogi_search::search::{search, SearchLimits};
let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
let result = search(&game, SearchLimits { depth: 1, nodes: Some(1000) })?;
assert_eq!(result.completed_depth, 1);
# Ok::<(), mshogi_search::search::SearchError>(())
```

## Frozen baseline correctness evidence

`cargo test -p mshogi-search --locked` exercises the exact recursive backup against an independently specified synthetic tree: two-ply minimax selects the branch whose worst result is 5 over the branch whose worst result is 3; one-ply static evaluation intentionally chooses the other branch. It also verifies full-width node visits and exact private-state unwind when halted. Chess fixtures derive from hand-solvable diagrams: mate in one for both colors, mate in two (three searched plies), a hanging queen capture, terminal mate/stalemate/dead/75-move outcomes, and score perspective. They replay every returned variation through the public rules boundary.

History regressions start from the same complete current FEN with either recorded knight cycles or imported unknown earlier history. The losing side can claim the third known occurrence; the imported game has no such claim. A fifth occurrence is an automatic result. Further tests distinguish optional claims from mandatory auto-claims even over mate, intended-move claims, policy at leaf boundaries, deterministic completed-iteration retention under work limits, caller immutability and explicit representation failure. These establish bounded search correctness, not playing strength or complete chess adjudication. The transparent evaluator's existing tests remain applicable.

## Historical full-width execution baseline

Measured 2026-10-25 local date on the MSH-20 working diff based on `d7eced4` (integration commit to be recorded in MSH-20). Hardware: Apple M1 Pro, 10 cores (8 performance/2 efficiency), 32 GB RAM, MacBookPro18,1. System: macOS 27.0.1 (26A434), arm64. Compiler: rustc 1.89.0 (29483883e), LLVM 20.1.7; ordinary Cargo `--release`, no custom build flags, one search thread. Command: `cargo run -p mshogi-search --release --locked --example search_baseline`. Settings: depth 3, unlimited nodes, `ExplicitOnly`, material v1, no external stop. Each fixture imports its stated FEN and thus has unknown earlier history. Node totals include iterations 1, 2 and 3.

| Case / initial FEN | Nodes | Elapsed ms | Nodes/second | Root score / choice |
| --- | ---: | ---: | ---: | --- |
| Standard `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1` | 9765 | 1435.762 | 6801 | 0 / a2a3 |
| Mate two `7k/8/4QK2/8/8/8/8/8 w - - 0 1` | 1204 | 18.187 | 66200 | 999997 / e6d7 |
| Capture `7k/8/8/8/8/8/q7/R6K w - - 0 1` | 2328 | 82.063 | 28368 | 500 / a1a2 |

These are single-run elapsed observations, with setup/build time excluded by the example timer. They are neither speed targets nor playing-strength evidence. The implementation intentionally retains straightforward rules queries at every node. Independent review and native platform CI are recorded in the task rather than inferred from this macOS run.
