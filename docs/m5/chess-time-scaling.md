# Chess time scaling: findings and engineering implications

MSH-155 (Chess Time Scaling), 2026-11-10. **More time repairs two important selected
mShogi decisions, but this experiment does not establish longer-time playing strength.**
The evidence rejects treating the one-second conditional local index as a general
rating or a fixed ceiling. It also does not establish that the engine is broadly
strong at longer controls, or that execution overhead alone explains its limits.

Production source remains `d53f34413d808dc2b727a06a770b92f24191eabf`; the repository
baseline was `dcc17e1`. No engine implementation changed. John broadly approved
MSH-154 (Chess Adversarial Assessment) and requested this investigation before
substantial implementation. The next engineering decision still awaits his review.

## How decisions change with time

A fresh independent selector froze eight roots before new engine answers: two per
primary tactical, positional, defensive and endgame category, with one fresh root
per category. Historical roots retain complete histories. The fresh mate also has
its published full-game prefix; the EPD/studies explicitly start independent contexts.

All three engines ran every root at 1, 5 and 15 seconds. The table shows **moves at
1 → 5 → 15 seconds**, not an aggregate accuracy score. A dash in the final column
means 30 seconds was not run. Current-source identities and every raw result are
linked in the [verification record](evidence/msh155/verification.json).

| Root and outcome authority | mShogi | Goldfish 1.13 | Oxidation 0.6 | Selected 30s result |
| --- | --- | --- | --- | --- |
| Historical 22.Nf5; finite-reference tactical/coordination judgment | Nf5 → Bf5 → Bc1 | b3 → Rab1 → Bc1 | Rab1 throughout | — |
| Fresh SfkRC; proved mate within five plies | Qf2+ throughout | Qf2+ throughout | Qf2+ throughout | — |
| Historical 24.h4; finite-reference initiative judgment | h4 throughout | Qb4 → Qb4 → h4 | h4 throughout | — |
| Fresh STS Undermine.004; published preference plus inspected reference continuations | b5 throughout | b5 throughout | Ne5 throughout | Same three choices |
| Historical 23...Rf6; finite-reference defensive judgment | Rf6 → Rad8 → Rad8 | Rf6 throughout | Rad8 throughout | — |
| Fresh Réti; exact draw-preserving move | Kg7 throughout | Kg7 throughout | Kg7 throughout | — |
| Historical rook ending; finite-reference favorable choices | Rf4 → g4 → g4 | b5 → g4 → Ra6 | Ra6 → f4 → f4 | — |
| Fresh Dedrle; exact win-preserving move | Kb1 throughout | Kc3 throughout | Kc3 throughout | Same three choices |

**mShogi's two known weaknesses improve at five seconds.** Fresh bound-aware
Stockfish checks support Bf5/Bc1/Rab1 over Nf5 and Rad8/Ne4 over immediate Rf6.
The 15-second change from Bf5 to previously unclassified Bc1 was investigated:
Bc1 remains near equality in the reference continuation, so it is not an established
regression merely because it differs from the earlier answer. These two roots were
already known to be time-sensitive; their improvement cannot estimate general frequency.

Four additional mShogi-only samples at **2 and 3 seconds** still chose Nf5/Rf6.
Five seconds is therefore the earliest **sampled** successful allowance on both,
not a guaranteed threshold, monotonic law, or achievable speedup multiplier.
The samples argue against assuming a small microoptimization will automatically
bring those decisions inside one second. Larger exact-cost reductions and more
useful search coverage deserve complementary investigation.

**The fresh controls were already successful at one second.** mShogi finds the
proved mating move and preserves both tablebase outcomes at every tested allowance;
it also retains the published STS preference through 30 seconds. These are strong
local controls but a ceiling-limited test of further improvement. The exact endgame
claim concerns the root choice, not demonstrated complete-game conversion. Once these
positions guide development they are regressions, not fresh holdouts.

On the other historical controls, h4 remains supported and all observed rook moves
retain substantial favorable finite-reference assessments. No supported mShogi
regression appears in this panel. That is not proof of monotonic improvement in
Chess generally. Additional time changes some scores and PVs without changing the
outcome, and not every move change is either progress or failure.

## What the reference comparisons establish

The references do not scale uniformly. Goldfish repairs its Nf5-root choice by five
seconds and chooses the stronger h4 initiative at fifteen seconds, but retains the
Rf6 weakness through the tested fifteen-second limit. Oxidation already chooses
supported defenses on both historical weakness roots at one second. mShogi therefore
closes those particular gaps to Oxidation by five seconds; it does not gain five
times more chess ability merely because it receives five times the time.

On Dedrle, both references choose Kc3 through thirty seconds, **surrendering the win
to a draw**, while mShogi chooses the only win-preserving Kb1. This is exact tablebase
evidence at the stated counters. On STS, Oxidation's Ne5 remains disfavored through
thirty seconds: it vacates Nf3's control of h4, enabling queen-entry/checking activity.
mShogi and Goldfish keep b5, preserving that control while starting the queenside lever.
The STS outcome is a finite-reference-supported positional/tactical judgment, not
proved WDL or mere published top-move agreement.

Thirty-second selection followed the frozen rule: Dedrle ranked first by exact
WDL evidence; STS and Rf6 then tied on consequential finite-reference evidence,
and the stable-ID tie-break selected STS. Rf6 ranked third and was not run at thirty
seconds. This is a selected follow-up, not an eight-position thirty-second curve.

These results show **different matched-time decision-quality trajectories** and
some selected-position convergence/advantages. They do **not** establish improved
relative match competitiveness at longer controls. No games were played, the roots
are purposively selected, and opposing mistakes, openings, conversion and time
allocation across a whole game are unmeasured. No new rating or mixed-time estimate
is calculated. The roughly 1900 historical index remains conditional on its original
one-second experiment and opponent/pool assumptions.

## Evidence behind the judgments

The [stable benchmark](chess-decision-benchmark.md) keeps proof, tablebase outcomes,
published preferences and finite opinions separate. Three fresh hard objectives are
reported individually, not averaged with uncertain positional scores. All exact
mate root moves were independently enumerated; both tablebase maps cover every legal
move with child-to-root perspective inversion and explicit counter assumptions.
An unlisted move remains unresolved until independently assessed.

Twenty-three fresh Stockfish 19 queries used a common three-second allowance,
Threads 1, Hash 16 MB, cold processes, full supplied histories and no tablebases.
Each sampled candidate and an independent anchor was tested from the same parent.
Raw atomic UCI events retain bounds, PVs and final-bestmove alignment; all selected
non-bound events aligned, all PVs were legal, and last-two-depth values varied by at
most 21 cp. Differing search horizons still make these finite assessments, not exact
centipawn losses or proofs of optimality.

Concrete continuations were inspected. Nf5 allows queen liquidation followed by
loose queenside material; Bf5/Bc1 preserve coordination. Immediate Rf6 permits Rh4
and an unfavorable queen/minor liquidation; Rad8 first permits ...Qc6+ and recapture
in the later defense. Qb4 gives Black time for a defensive ...g5 break that h4/Ng5+
restricts. STS Ne5 permits ...Qh4 and checks on h2/h1/h3. All sampled rook choices
remain favorable; differing winning margins are not labeled lost wins.
The [adjudication report](evidence/msh155/adjudication/findings.md) preserves these
lines, contrary cases and uncertainties. Tablebase/proof objectives were never
regraded to agree with a reference engine.

## Search work, useful outcomes and causal limits

The [machine-readable trajectories](evidence/msh155/runs/all-summary.json) retain
actual elapsed time, main/q work, PV, score, native completed/incomplete iterations
and available reference telemetry. On mShogi's Nf5 root, completed depth progresses
5 → 7 → 8 at 1/5/15 seconds, with approximately 0.24 → 1.22 → 3.76 million visits.
On Rf6 it progresses 7 → 8 → 9, with 0.21 → 1.09 → 3.29 million visits. The useful
observations are their supported decision changes, not depth increments themselves.

Across non-mating native samples, quiescence is roughly 55–95% of counted work.
Between about 6% and 72% of allotted search time occurs after the last completed
iteration, depending on the root/budget. This is **unfinished work, not proved waste**;
it does not justify returning partial-root scores. Dedrle stays at completed depth
15 between 15 and 30 seconds while retaining the correct move. The mating control
finishes early at the safeguard depth 128; that nominal number is not useful depth.

Reference UCI depth is explicitly weaker evidence: its last matching non-bound
score is not proof that a complete iteration finished, and reference incomplete
iteration fractions are unavailable. Node definitions differ, so raw NPS/depth
ratios are not normalized work or strength comparisons. Equal nominal time also
retains the prior unequal memory policies: 4096 native TT entries, Goldfish 16 MB,
Oxidation 64 MB. All engines were single-threaded, cold per cell and sequential on
the same unisolated Mac; no statistical timing-repeat claim is made.

The results support **available search work interacting with coverage and valuation**,
not an isolated cause. More time can reach the better decision, so current behavior
is not uniformly incapable of it. But the experiment did not vary execution,
selectivity and evaluation independently; neither static evaluation nor pruning is
exonerated. MSH-154's profiles remain relevant, including substantial native work
outside counted visits. Its obsolete-history defect remains real and narrow and
does not explain these two selected weakness roots. A large representation rewrite
or isolated compensation bonus is not established as the best remedy.

## Priorities and decisions

The [revised roadmap](strength-roadmap.md) makes native-cost reduction and
continuation/evaluation diagnosis complementary within one coherent round:

1. Establish which candidate defenses and consequences distinguish supported choices,
   with matched-horizon, bound-aware comparisons and fresh controls. Improve useful
   coverage while retaining the exact outcomes and successful decisions.
2. In parallel, prototype measured exact native reuse: checked transition effects,
   changed-cell updates and efficient history/claim summaries. Preserve one native
   authority, complete histories, claims, restoration and cancellation. Measure
   total-cost gains and decision non-regression; do not promise that a small speedup
   buys the five-second decisions or precommit a wholesale rewrite.
3. After meaningful combined progress, propose a controlled predecessor/current
   comparison with common openings/opponents and a sample designed for the effect
   of interest. Serious qualification should consider conventional clocks with
   increments and declared rating-pool conditions, including hardware differences.
   Separate time controls unless a justified model supports combining them.

MSH-153 (Classical Compensation Calibration) remains held: endpoint valuation must
be distinguished from insufficient/selective coverage before tuning. If a coherent
round fails its decision/cost goals, reconsider the causal hypothesis and strategy
rather than automatically generating more isolated optimization tickets.

The consequential decision is approval or revision of that coherent engineering
round and its practical thinking-time objectives. No additional resources are
needed to finish this diagnosis. A substantial opponent study still requires a
separate approved design and budget. MSH-140 (Player Thinking Time) remains separate
product work; this experiment changes no desktop default. No learned evaluation or
Shogi/Xiangqi development is authorized.

## Verification and boundaries

The [verification record](evidence/msh155/verification.json) covers 72 matched baseline
cells, six selected thirty-second cells, four intermediate mShogi samples, three
smokes and 23 successful independent adjudications. Active engine/search charge is
723.175 seconds, plus 0.132 seconds for independent mate proof; below the declared
900-second operational ceiling. No matches, production changes, hosted platform
campaign or packaged GUI campaign occurred. All source/binary fingerprints, full
histories, run failures, actual timings and assertion results are retained.

One Stockfish initialization failed before `go` under the inherited file-size cap.
The failure and explicit reviewed retry are preserved and charged; the exact file
triggering the signal is unknown. A higher inherited cap with bounded controller
logs resolved startup. No failed query supplied chess evidence. The independent
methodology reviewer challenged design, driver, escalation and final conclusions.

The eight roots and single timed samples are the main limit. Fresh controls show
local capability, while previously selected weaknesses favor observing time gains.
The evidence supports neither a blanket claim of intrinsic weakness nor a claim
of broadly strong longer-time play. It establishes specific capabilities, limits
and an evidence-based next engineering question, then stops for owner review.
