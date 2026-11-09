# MSH-152 bounded independent diagnosis proposal

Read-only source/evidence inspection; no engines, builds or matches run.

## Start with three discriminating historical chains

1. **MSH-151 Goldfish pair003 White27.e4 -> ...Re8 ->28.exf5 ...Rxe5 ->29.fxe5.** The d8/+17 PV already includes this exchange, whereas next actual roots report -97/-108. Root `r7/pp1q1r2/1b5k/4Qb2/1P1p1P1P/P3P1P1/1B1P1R2/R5K1 w - - 0 27`. Fullhistory in `docs/m5/evidence/msh151/diagnostic-selected.json.gz`, id `confirmation-003-0-27-e3e4`. Distinguish continuation valuation from an omitted first capture; no known unique best move or first losing move. Preserve the actual alternative ...Rf5 in its forecast versus ...d3 in game. After exchange White2R+B+7P versus BlackQ+R+B+3P. At30.g4 d3 is NOT a passed pawn because White d2 remains.
2. **MSH-151 Oxidation pair004 Black31...Ke8 ->32.Rb8.** Root `2b5/3k1pqr/8/2p1rR2/2PpP1P1/Q4BPp/P6K/1R6 b - - 1 31`, id `confirmation-004-1-31-d7e8`. -136/d6 forecasts Qa4, then actual Rb8 yields -684/d6. Rb8 absolutely pins Bc8 to Ke8 (independently legal-replayed). This rich-material quiet invasion is the best main-selectivity discriminator. Native40.g4# proves eventual loss, not that Ke8 is uniquely bad or that forced mate existed here.
3. **MSH-144 Goldfish White44.Ne5 ->...Bxe5 ->45.Kxe5.** Root `8/p4pp1/1k3b1p/3P3P/4K1P1/3N4/8/8 w - - 2 44`; complete history and both root records `docs/m5/evidence/msh144/diagnostic-positions.json`. Main null/LMR/quiet/SEE omissions are already materially gated off in this ending, so it separates horizon/evaluation/q effects from rich-material pruning. Existing147 time scaling is historical evidence; no need repeat its full matrix at every revision.

Hold **MSH-139 first-loss-v61** (`docs/m5/evidence/msh139/example-inputs.json`, complete history) as the older-game cross-check: +34/d7 forecast Rf7 ...Rxf7 Rxf7 and king/rook play; no independent best-move proof. Use only if an exchange-value explanation survives the first chain, rather than adding another full time matrix now. This ensures all three studies inform selection while prioritizing new causal information.

## Three small independent controls, plus one negative

- `f3apJ` in `docs/m5/evidence/msh139/quality-inputs.json`: `1r6/3n4/3P2pk/N1pNp3/P2nPp1p/5PqP/Q5P1/3R3K b - - 0 32`. Published continuation ...Nf3 gxf3 Qxf3 Qg2 Qxd1, full history retained. Capture sacrifice/recapture frontier control. Expected first move and line are published puzzle evidence, not a proof of uniqueness; native legality replay alone does not establish optimality. Existing147 already solved1s, so retention is a guard, not new improvement.
- `sts-minortrade-001` from `docs/m5/evidence/msh147/baseline-inputs.json.gz`: `1b3rk1/5ppp/2p2rq1/1p1n4/3P2P1/1BPbBP2/1P1N2QP/R3R1K1 w - - 0 1`, published best Bxd5. Use supplied positional points/ranking as independent heuristic evidence, not theorem; static decomposition and competing continuation rank can distinguish positional compensation from tactical correctness. Do not label disagreement a bug.
- `tb-7`: `8/8/2K5/8/4P3/8/8/3r3k b - - 1 1`, independently preserved Syzygy full-move map in139 quality inputs/source. **Only ...Re1 preserves the win**; alternatives draw or lose. This provides a genuine decision-quality metric (WDL regret0), independent of centipawn calibration.
- Nearby negative `tb-6`: `8/8/2K5/4P3/8/8/8/3r3k b - - 1 1` is drawn; retain a drawing move rather than overgeneralizing ...Re1 as a universal pattern. Positive static score here is calibration error, not necessarily move-quality error. Board-WDL excludes whole-history claim authority.

## Three experiments, adaptively bounded

A. **Forecast consistency before optimization.** At the first chain replay the exact fullhistory prefixes, record root-normalized cp or mate distance, static components, completed iteration, PV, main/q visits, pruning/research counters, fullhistory claims. A reroot gets fresh q horizon, extension budget, ordering and table; equal nominal parent/child depths are not equal work. For a decisive consistency check, trace only the relevant line's original remaining main depth, q allowance, extension credits, search window and bound provenance; use an equivalent internal-context full-window probe if needed. Do not assert printedPV endpoint static equals backed-up score: standpat and TT-bound paths can truncate/change the returned line. Stop once the first material divergence is localized to alternative continuation, frontier, or evaluator; don't instrument every node.

B. **Selective versus less-selective at equal completed depth.** On rich-material Ke8/Rb8 only, compare baseline with main LMR+quiet omissions disabled, preserving evaluator and q frontier; isolate LMR versus omissions only if the combined result changes the informative move/value. Add SEE/delta-disabled capture control only if traced capture rejection is implicated. Current `Exact` mode uses STATIC leaves, so it is not a pruning-only comparator. Existing toggles also leave verified null and extensions enabled. If no changed relevant path after two feasible completed depths, stop that branch; do not claim all selectivity exonerated. A no-table exact shallow backup remains a correctness control, not strength evidence.

C. **Horizon versus static compensation.** Increase time only for the first unresolved historical chain (1s then5s;15s or30s only if a concrete unresolved diagnostic could change action). Compare its chosen alternatives under completed depths, not merely greater nodes. At the quiet postexchange point inspect static material/mobility/king/passers and its short search versus the earlier predicted leaf. A deeper correction that also improves independently established tactic/WDL is decision evidence. A more negative score with the same move and unchanged known outcome is calibration evidence. Stable wrong reference preference is not proof of evaluator fault until relevant alternatives receive comparable complete investigation. One finite strong-reference multiPV comparison may establish a competing hypothesis, never replace native or tablebase truth.

## Measurable outcomes and stopping

- For forced-tactic/TB controls: correct root action set, native replay, WDL preserved, mate/distance semantics correct; no regression accepted for a merely higher nominal depth.
- Historical roots without proven optimum: report move ranking/score stability under comparable completed work, identified continuation coverage and evaluator-component changes; require independently validated consequence before calling a choice improved. Reference disagreements remain qualified.
- Initial screen is just the3 history roots and3 independent controls plus draw negative at1s; targeted followups only. No Cartesian product of all flags/depths/times. Stop a cause investigation when a reproducible controlled toggle/context explains the first discrepancy and a nearby independent positive/negative distinguishes it; otherwise record unresolved hypotheses and next discriminating experiment. Stop uninformative time escalation after two successive completed-depth increments leave the same decision and score trajectory without a new distinguishing prediction. This is a resource stop, not proof of diminishing returns.
- Implement only after a general mechanism is supported; then rerun the exact causal comparison and independent controls. A PV score change alone does not establish stronger play. No rating study or match campaign follows automatically.

## Source-specific cautions / conventional references

Current source: `chess_conventional.rs` main() applies check extensions before TT context; ordinary reductions are move-index based and materially guarded; verified null depends on static beta condition. Q standpat is allowed except check/contact-queen threat; quiet checks/threats only first two frontier plies; capture SEE/delta omissions are separate. These are hypotheses to test, not defects inferred from algorithm presence.

Stockfish11 classical primary implementation illustrates search margins/reductions conditioned on improvement/history and selective q capture safeguards, rather than applying every static rule uniformly. It also retains PV/nonPV distinction in qsearch. Its advanced-pawn exception to q futility suggests a specific comparator ONLY if our rejected capture/pawn frontier trace implicates it; do not copy weights or preemptively add it.
https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/search.cpp

Stockfish11 evaluator distinguishes strongly defended targets, safe pawn threats and restricted pieces. These are possible explanatory component comparisons after causal trace, not an instruction to add more heuristics by default.
https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/evaluate.cpp

STS author explains king-activity fixtures as active defence, pawn support, restriction and blockade, supporting broad positional controls rather than interpreting every activity case as attack.
https://sites.google.com/site/strategictestsuite/11-activity-of-the-king
