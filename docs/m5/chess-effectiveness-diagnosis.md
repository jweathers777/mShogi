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

## Time scaling

| Position | 1s move / score / depth | 30s move / score / depth |
| --- | --- | --- |
| MO2mm | d8d5 / 999995 / 128 | d8d5 / 999995 / 128 |
| f3apJ | d4f3 / 433 / 9 | d4f3 / 591 / 12 |
| CNWwY | c1c8 / 752 / 8 | c1c8 / 960 / 11 |
| tb-6 | d1c1 / 303 / 7 | d1e1 / 272 / 10 |
| tb-7 | d1e1 / 376 / 8 | d1e1 / 358 / 10 |
| sts-openfiles-001 | e5e3 / 106 / 5 | e5e3 / 160 / 10 |
| sts-minortrade-001 | b3d5 / 133 / 8 | b3d5 / 168 / 12 |
| sts-kingactivity-001 | c5e7 / 121 / 6 | b8c8 / 159 / 11 |
| standard-control | b1c3 / 18 / 9 | b1c3 / 21 / 13 |
| closed-minors-control | d3d2 / 131 / 9 | c4c5 / 222 / 12 |
| goldfish-pawn-exchange-44 | e4f5 / -65 / 8 | e4f5 / -132 / 10 |
| goldfish-queen-rook-31 | b7a7 / 162 / 8 | b3c5 / -88 / 11 |

Scores are root-side centipawns except 999995, which is the native mate-distance
score for mate in five plies. The mate case finishes early; its depth ceiling is
not comparable to useful nominal depth in unresolved positions. The complete
5s/15s trajectories, PVs and cumulative work are retained in the raw receipts.

All three published tactical first moves are found at every budget. The unique
winning rook move is also retained in every baseline condition. The drawn KRKP
neighbor remains optimistically scored after thirty seconds, supporting missing
conversion/scaling knowledge rather than an illegal-move or adjudication defect.
The queen/rook historical score changes from +162 to -88 and its move changes;
extra computation materially changes that judgment. The pawn-exchange root
already avoids Ne5 in this local one-second sample at depth 8 (the historical
match completed depth 7); its score becomes more pessimistic with deeper search.
Neither observation proves which historical move first lost the game.

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

## What the controls establish

- **TT reuse helps; capacity is not the leading explanation.** Disabling the
  table loses one completed ply in three of four five-second cases. Increasing
  it from 4,096 to 16,384 entries gains no completed ply in those cases. Choices
  remain the same. This does not justify dropping history identity or simply
  allocating a larger table.
- **More selective depth can be worse.** Experimental low-material LMR gains
  two plies in the pawn-exchange case and three in the closed-minor control.
  But in the independently adjudicated rook ending it changes the unique
  winning ...Re1 to drawing ...Rc1, despite increasing completed depth 9 to11.
  This broadening is rejected as a production change. Null passing and shallow
  omissions remained guarded, so this is specifically an LMR quality warning.
- **Selectivity affects decisions, not only speed.** Removing quiet omissions
  selects the STS-preferred Kc7 in the king-activity case at depth 7; the default
  selects Bxe7 at depth 9. Removing LMR, SEE or delta produces other trajectories.
  A separate completed depth 7 comparison returns Bxe7/+106 by default versus
  Kc7/+110 without quiet omissions (208,175 versus572,841 visits), confirming
  that selectivity itself changes this choice at equal nominal depth.
  These equal-time comparisons mix depth and pruning effects; they do not prove
  that the preferred STS move wins or identify a universally better setting.
  In the queen/rook case, disabling delta alone reaches depth 10/-16 in five
  seconds versus default depth 9/+23. That is a useful targeted lead, not grounds
  to disable every safeguard.
- **Avoided work is a safer immediate lever.** In the tactical, king-activity
  and queen/rook controls, quiescence projects roughly 1.9/2.7/4.1 million quiet
  candidates per five seconds. About 95–96% of move-loop q cutoffs occur on the
  first actually searched move (stand-pat and TT exits are excluded). Current
  code performs every quiet projection before searching good captures. Deferring
  those projections until their ordered moves are reached can save work without
  adding pruning or changing relative move order.
- **Evaluation limitations extend beyond one pawn race.** Six supplementary,
  independently queried endgames all preserve WDL in the one-second choices,
  but theoretical KQ-versus-bishop-pawn and KNNK draws score +714 and +827.
  The corresponding winning central-pawn and KBN cases score +729 and +806.
  A drawn rook ending scores +203. These expose weak conversion discrimination,
  not failed native adjudication. The generic evaluator has no material scaling
  stage and gives king danger zero endgame weight. Its passed-pawn runway uses
  current attacks rather than future king interception; those general evaluation
  questions remain relevant after computational waste is removed.

## CPU attribution and next engineering order

Three separate four-second owned-process samples cover open lines, the pawn
exchange and queen/rook play. Exclusive stack attribution assigns 32–51% to other
native work,14–22% to evaluation including its native calls, 12–13% to allocation,
and1–3% to SEE including native calls. Explicit table/key-comparison symbols are
below 2%; repetition identity and position-hash maintenance are separate costs.
The parser subtracts immediate children, counts recursive ancestry once and
reconciles every residual against the profiler's independent collapsed totals.
Inlining and short samples limit precision. Quiescence ancestry spans 37–81%, but
those overlapping figures cannot be added to the exclusive categories. No exact
CPU percentage or anticipated speedup is assigned to quiet projections alone.

The initial sandboxed samples could not inspect their owned child (exit255).
Three bounded retries outside that sandbox succeeded without sudo or changing
security settings. Both failed attempts and successful samples are preserved.

The evidence-led order is:

1. **MSH-148 (Deferred Frontier Work):** remove unused quiet projections while
   keeping the admitted move order, native checks and existing search policy.
   Verify identical controlled fixed-work decisions and actual saved cost.
2. **MSH-145 (Pawn Threat and Exchange Judgment):** address general conversion,
   scaling and king-interception limitations using the broader independent
   counterexamples, rather than special-casing Ne5. Do not adopt unrestricted
   endgame LMR or inflate the TT as a substitute for quality.
3. **MSH-149 (Classical King Exposure):** investigate material-aware queen/rook
   king-exposure corrections as a separate coherent evaluation increment. Reassess with fresh adaptive independent
   opponents after the coherent round, not after each technique.

This identifies several substantial remaining opportunities, not a single proved
explanation of a conditional 1900 rating. Native query/allocation costs, evaluation
quality and selective horizons interact. Further optimizations should follow
measured costs; neither nodes, nominal depth, STS agreement nor faster execution
alone establishes playing strength.

## Evidence and disposition

The baseline and controlled measurements are complete and independently approved. Source/input/artifact hashes, native assertions, rejected counterexamples and CPU attribution reconcile. The canonical
[verification record](evidence/msh147/verification.json) identifies the exact
instrumentation, production behavior, inputs, environment and actual checks.
MSH-145 (Pawn Threat and Exchange Judgment) remains a candidate correction until
these findings establish its priority. No learned evaluation or cross-game
transfer is authorized. Routine portable work follows Level 1 local verification;
full platform and packaged-product campaigns remain later integration obligations.
