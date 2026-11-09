# MSH-147 (Chess Effectiveness Diagnosis)

This investigation separates decision quality, useful search depth and execution
cost. It is not an opponent match study, rating estimate or declaration that
conventional Chess is complete. The accepted MSH-144 assessment remains the
playing-strength baseline until a substantial coherent round is reassessed.

## Method and boundaries

Twelve positions were selected before this run: three published tactical lines,
three first positions from independent STS themes, two independently adjudicated
rook-versus-pawn neighbors, two neutral computational controls and two full-history
Goldfish positions. Each receives a fresh 4,096-entry table at 1, 5, 15 and
30 seconds, depth ceiling 128 and no visit ceiling. A solved search may finish
before its allowance. Searches run sequentially on the same Mac, without concurrent
builds, tests or matches. This is a small diagnostic panel, not a representative
estimate of all Chess positions.

The [frozen panel](evidence/msh147/positions.json) retains provenance and expectation
strength. STS preferred moves are strategic judgments, not proof that alternatives
lose. The preserved Syzygy responses classify every legal child from the child's
perspective; those results must be inverted when judging the root choice. Published
tactical lines are natively replayed, not assumed to be exhaustive unique-best
proofs. Historical positions retain their entire real move prefix. Independent
FEN diagrams deliberately start fresh history with the stated counters.

The existing diagnostic example checks exact reconstructed root FEN, root
restoration, native PV admission and supplied-line admission. Scores use the root
side's perspective. Completed depth and its PV come only from a completed
iteration; visits and selective depth include interrupted work. Iteration counters
are cumulative and must be differenced. Node visits do not include every native
query, exchange recursion or candidate projection. Static evaluation terms at a
PV leaf are heuristic observations, not native outcomes or necessarily the backed-up
search value.

## Discriminating controls

The first pass holds production behavior fixed. Selected follow-up controls vary
TT capacity or one search mechanism while preserving quiescence, native history,
claims and cancellation. The old `Exact` mode is unsuitable as a one-factor pruning
control because it also removes quiescence and check extensions. CPU samples are
separate from the time-scaling runs; overlapping stack categories are not additive
percentages of exclusive costs.

Classical [Stockfish 11 search](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/search.cpp)
separates null-move conditions from late-move reduction decisions, verifies
promising reductions and varies reductions with search context/history. Its
[staged move picker](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/movepick.cpp)
defers later move classes. These motivate controlled comparisons; they do not prove
that its constants or published strength transfer to mShogi.

Current source inspection identifies a concrete coupling: the requirement that
both players retain at least 600 non-pawn material gates null passing, late-move
reductions and shallow omissions. This excludes many ordinary endgames from all
three mechanisms. Its practical effect must be measured; removing a null-move
zugzwang safeguard is not the proposed experiment. Mating-threat protection also
shares this gate and must accompany any broader reduction eligibility.

## Evidence and disposition

Measurements and independent reconciliation are in progress. The canonical
[verification record](evidence/msh147/verification.json) identifies the exact
instrumentation, production behavior, inputs, environment and actual checks.
MSH-145 (Pawn Threat and Exchange Judgment) remains a candidate correction until
these findings establish its priority. No learned evaluation or cross-game
transfer is authorized. Routine portable work follows Level 1 local verification;
full platform and packaged-product campaigns remain later integration obligations.
