# Conventional decision-quality investigation: bounded findings

**Disposition: stop this attribution experiment without a speculative coverage or evaluation change.** Retain the separately verified exact native history-cost correction as an execution improvement, then evaluate that frozen candidate against the predecessor on the full development collection and untouched held-out collection. These findings do not promise broad decision improvement from the native correction. Do not use held-out results to choose another change in this round.

The development baseline has all32 exact tablebase root outcomes preserved and its one immediate-mate objective met;60 of63 published puzzle preferences selected; STS17 maximum-point choices,11 listed lower-point choices and12 unlisted alternatives;17 of24 finite-reference preferred choices. The latter tiers are not a common accuracy scale. No exact failure was available for causal diagnosis. Three puzzle mismatches plus five lower-point strategic choices were selected after the complete160-root pass, with two successful controls. No held-out data was accessed.

## General weaknesses supported by independent continuations

Bound-aware same-parent Stockfish19 analysis at common cold3s supports consequential deficits on three development puzzles:

- **Pawn-ending king access and distant conversion (0UOyM):** native Kg6 permits a blockade; exf3+ liquidates the central pawns and opens ...Kh6–g5 and a distant passed-pawn race. Finite root-side opinions are0 versus+699cp. The engine retains Kg6 even at15s. Ordinary LMR/null activity is negligible; removing SEE or delta pruning at5s changes neither move nor completed depth13. This is a quiet horizon/valuation limitation, not an established faulty pruning condition.
- **Timing a king penetration and bishop blockade (0m5mI):** immediate Kc3 can answer ...f4/exf4/...e3 using Bc1–d2 while the king reaches b4/c5; Bd6 allows the opposing king to blockade. Finite opinions are0 versus+548cp. Fifteen seconds changes the choice to Kc3 at depth11, but its reported continuation still differs from the strongest reference defense and scores only+8. More search reaches the better root choice without demonstrating fully correct consequence valuation. There are no baseline LMR/null events; SEE/delta ablations retain Bd6 at depth10.
- **Quiet queen coordination and defensive recognition (28pJK):** Qc8 prepares ...Rd2/...Qa6+ and an invasion; Qf4 permits Qc1 and consolidation. Finite opinions are−438 versus+287cp. Native5s uses Qf4 and forecasts Bd3 instead of the reference Qc1 defense. Fifteen seconds reaches Qc8 at depth9 and includes the rook/queen invasion in its PV. Its negative score still differs from the finite reference, so static evaluation and horizon remain confounded.

These are concrete mechanisms rather than an assumption that every published alternative is correct. The strong reference remains finite opinion, not exact WDL or centipawn-loss proof. All16 initial reference queries had bestmove-aligned nonbound events and legal PVs; last-two-depth values differed by at most14cp. Raw data and opposite evidence are in `msh157-reference-findings.json` and the reference ledger.

The strategic cases are less decisive. Both Rg5 and h4 in Undermine.037 retain substantial finite advantages (+471/+668); Rg5 is not established as a lost win. Simplification, AT, Recapturing and Knight-Outposts show modest finite preferences (+53–99cp gaps), with specific alternative liquidation/defense sequences. They justify inspection, not an evaluation bonus or a blanket failure count.

## Controlled comparisons reject a simple pruning diagnosis

The main ablations retain the conventional quiescent frontier. Exact SearchMode was not used as a pruning-only control. All six matched-depth runs completed their requested iteration within the5s hard cap.

| Root and matched depth | Default | No LMR | What follows |
| --- | --- | --- | --- |
| AT.065, depth8 | Qe7,+43 | Qd6,+33 | Selectivity changes the choice at the same nominal horizon. However Qe7 has not been independently condemned; published5 versus10 points do not prove a general defect. The original Qxa5 appears at deeper ordinary iterations, so the initial apparent improvement also had a horizon component. |
| Recapturing.003, depth7 | Rxe4,+51 | Rxe4,+57 | The apparent no-LMR improvement at5s is explained by the earlier ordinary iteration choosing the same move. Deeper default search later selects Rxd8+. A reduced fail-low defect is not established. |
| 28pJK, depth6 | Qf4,−204 | Qf6,−193 | Selectivity changes a choice, but neither is the supported Qc8 and Qf6 remains unadjudicated. This does not demonstrate better tactical decisions from disabling reductions. |

Removing main quiet omissions also changes some choices while reducing completed horizons. In Undermine.037 it chooses Rc2 with a native repetition PV/score0 instead of the strongly favorable baseline Rg5, providing a warning against blanket removal. The two controls retain expected move/outcome. Removing SEE pruning does not repair either difficult ending or the queen-coordination puzzle; the queen selects another unresolved Qf6. Removing delta pruning leaves all three puzzle choices unchanged, again with controls retained. There is no evidence here for turning either omission off globally.

Equal nominal depths are themselves selective and depend on ordering/history context; these comparisons are controlled diagnostic evidence, not exact minimax equivalence. Seeing a relevant move in a PV does not establish that every defense or endpoint was properly valued. No individual reduced/omitted continuation was causally localized sufficiently to warrant a targeted production change.

## Stopping rule and next interpretation

The declared question has discriminating results: two important choices improve with time, one persists; a plausible LMR explanation is partly falsified by matched-depth controls; broad omission removals fail to produce reliable supported benefits; exact controls remain intact. Further parameter sweeps would spend the reserve without a sufficiently specific new hypothesis. Do not invent an outpost, compensation or king-activity bonus to make these examples agree.

Forty native diagnostic attempts consumed **241.674175835 active seconds**, below the300s diagnostic allocation, all successful. This includes six15s searches, eight no-LMR, eight noquiet, seven noSEE, five nodelta and six matched-depth searches; requested caps sum260s, with matched-depth completion returning early. Sixteen reference queries are separately charged within the160s reference reservation. No new match was played. Detailed structured trajectories: `/tmp/msh157-development-probes-summary.json`.

The justified correction remains the independently established native obsolete-history cost defect, whose scope and performance evidence belong to Workstream A. Its practical quality effect must be measured rather than inferred from a3.9–4.1× affected-root speedup. If the coherent checkpoint produces no broad decision benefit, report that outcome, retain the exact-cost improvement on its own merits, and reconsider the diagnosis before another engineering series. A substantial competitive study is not justified merely by these diagnostic results or microbenchmark gains.
