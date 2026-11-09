# MSH-139 (Chess Strength Assessment): findings

**Current working estimate: about 1900 on a conditional local CCRL-referenced
index, with a rough working range of 1700–2200.** This describes equal-one-second
matches on this Mac, not an official CCRL or human Elo rating. The range summarizes
separate opponent estimates and prior sensitivity; it is **not a joint 95%
confidence interval** or a bound on transfer to CCRL's conditions.

Match work stopped after 24 complete games. Fresh results against two independent
nearby families agree sufficiently to inform the next development decision.
Further games would mainly refine the uncertainty. No strength target has been
selected, and assessment completion does not authorize cross-game transfer.

## Actual results and uncertainty

W–D–L is from mShogi's perspective. Scout and confirmation openings came from
disjoint, frozen halves of the generic 8moves_v3 book. Each opening was played
with reversed colors; the opening pair is the statistical unit.

| Full-strength reference | Published anchor | Scouts, 4 games | Fresh confirmation, 8 games | Fresh score | Conditional index median | 95% credible interval |
| --- | ---: | --- | --- | ---: | ---: | --- |
| Oxidation 0.6.0 | 1943 ±25 | 2–1–1 | 1–4–3 | 37.5% | 1883 | 1750–2047 |
| Goldfish 1.13.0 | 1982 ±18 | 0–1–3 | 2–3–3 | 43.75% | 1952 | 1756–2160 |

The anchors come from one [CCRL 40/15 snapshot, November 16, 2024](https://www.computerchess.org/rating_list_all.html).
Across all 24 games the descriptive total is **5 wins, 9 draws and 10 losses**.
All conclusions were native checkmates or native draws. No scored game failed,
hit a technical cap, or used evaluation adjudication. All twelve scored pairs
passed authoritative native replay and process cleanup.

Primary inference uses the frozen pentanomial Dirichlet(.5 per bin) model,
separately for each exact release and phase. With .2/1 priors, confirmation
medians are 1862/1901 against Oxidation and 1941/1961 against Goldfish. Their
95% credible bands span approximately 1716–2185 across those sensitivity checks.
This supports the rounded working range, without pooling away family differences.
Supplemental anytime-valid score bounds remain broad; the Bayesian intervals do
not acquire frequentist coverage from those bounds. Four fresh pairs per family
remain a small sample, and tail precision is limited.

Scouting initially gave Goldfish only one draw and three losses. Fresh games
included two draws and a two-win pair. Retaining those swings prevented an early
mismatch judgment. Broad higher/lower references were available; observed scores
and uncertainty kept measurement near these informative opponents. A further
reference build was unnecessary for the present decision.

## What the games and diagnostics reveal

The frozen 12-puzzle Lichess sample produced eleven published first moves and
one equivalent **Re6#**, independently confirmed by native checkmate. This is a
small diagnostic result, not a tactical rating. A brief Arasan handshake may have
overlapped early tactical queries; their timing is not used for throughput
estimates. In eight tablebase positions,
seven choices preserved WDL; four positions were nondiscriminating because every
legal move had the same outcome class. The selected queen/rook mate-in-one moves were handled.

The discriminating failure was concrete: with Black king h1/rook d1 against White
king c6/pawn e4, **...Re1 is the only winning move**. mShogi chose **...Rc1+**, which
draws. All tablebase alternatives were checked against the native legal move set.
A drawn opposition position received a +260 one-second search score; a theoretical
rook-versus-pawn draw received +303. These are search scores, not static-evaluator
measurements. They expose endgame judgment/calibration limits without changing
native game outcomes.

An actual Goldfish loss shows why the puzzle result does not establish broad
tactical reliability. At **39...Qxa5**, mShogi reported +233 at completed depth2.
A bounded Stockfish19 analysis assessed the position near equality before that
capture and roughly −648 afterward, from Black's perspective. Later, **41...Kh5**
received +139 at depth1 despite 295,113 visits, permitting **42.Qg4+ Kh6 43.Qg6#**;
Kh6 is the sole reply to Qg4+. By move41 the reference already found forced mate
against Black, so Kh5 must not be described as the first avoidable error.

In an Oxidation loss, **45.Rf1** traded into a bad pawn ending. The candidate's
score was about −49; the reference assessed about −432 before the move and −1091
afterward. These three-second reference assessments are diagnostic opinions,
not proven outcomes or match adjudication. The later pawn capture was already in
a losing position and was also the reference's preferred move; blaming it as the
initial error would be misleading.

## Search limits and realistic conventional opportunities

On CP-scored turns, mShogi's median completed nominal depth was7, with roughly
169,000–191,000 reported visits per request. Oxidation's median scored nominal
depth was9 with about1.19million reported nodes; Goldfish's was10 with about
2.90million. Reference root updates need not represent completed iterations.
The extractor preserves scored reports separately from final work-only reports.
Node definitions differ: mShogi's visits omit significant native candidate,
preview, exchange and threat-query work. These figures are not direct CPU-speed
ratios.

Historical MSH-137 (Chess Search Refinement) counters separately show substantial
quiescence work and limited exact-context TT cutoffs. They are not new CPU profiles
from this assessment. Merely increasing table capacity previously showed no
meaningful gain. Current evidence favors investigating search-frontier efficiency,
quiet-attack reliability, redundant native/SEE/threat work and useful safe cache or
ordering reuse, while retaining native rules and history guarantees.

Specialized endgame evaluation and scaling are also established opportunities:
current evaluation uses generic material/positional terms without the specialized
KPK/KRKP knowledge found in classical [Stockfish11 endgame code](https://github.com/official-stockfish/Stockfish/blob/sf_11/src/endgame.cpp).
Such evaluation knowledge must guide search while native Game remains the outcome
authority. Coefficient calibration and king-safety judgment remain plausible work.
The evidence does not establish diminishing returns. It cannot predict a rating
gain, development cost or final attainable strength; those need a chosen next
objective and focused engineering evidence.

## Conditions, reproducibility and handoff

Candidate and runner were built release/locked from
`9a509f077460e07341d2bccf830e4b02fcb0e95e`; production remained unchanged throughout.
All participants were native ARM64, one search worker, no pondering or strength
handicaps, `go movetime 1000`, and no external tablebase files. Reference defaults
were Oxidation64MB Hash and Goldfish16MB; mShogi retained its production cache
budgets. Games ran sequentially on the shared M1 Pro/32GiB Mac, macOS27.0.1;
hardware/load were not isolated. An observed power snapshot was AC/charged.
CCRL's longer calibrated Intel time control, tablebases, compiler and pool
conditions differ. Published anchor errors and these transfer effects are separate
from the conditional credible intervals.

Scored runs took **48.6 minutes** elapsed; unscored protocol smokes and recorded
diagnostics bring measured execution to about50minutes. Build/preparation work is
separate in the preserved logs. No paid compute or fixed campaign-hour budget.
Per-response, process, stream, report and pair watchdogs remained active. Simbelmyne
was excluded before scoring because its fixed-time implementation stopped almost
immediately. Setup failures, earlier helper versions/corrections, all smoke caps
and the unused Arasan probe's verified resource cleanup are retained.

The [method](chess-strength-assessment.md) and single [verification record](evidence/msh139/verification.json)
identify source/binary hashes, opening lineage, every raw compressed report,
commands, tests, diagnostics and independent review. Twelve focused offline
instrumentation tests passed on macOS27.0.1/Python3.11. Unchanged production CI and
GUI evidence from MSH-138 (Desktop Search Limits) is reused; this offline assessment
does not claim fresh Linux/Windows executions or another GUI campaign.

**Next decision belongs to John after reviewing these findings.** MSH-50
(Conventional Shogi) and MSH-54 (Conventional Xiangqi) remain To Do. No final target,
further production improvement, learned evaluator or cross-game implementation
was selected or begun here.
