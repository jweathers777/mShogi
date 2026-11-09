# MSH-151 (Chess Coherent-Round Assessment): findings

**The fresh comparison does not establish a playing-strength gain.** Oxidation
confirmation is unchanged from MSH-144 (Chess Progress Assessment); Goldfish confirmation improves in raw
score but varies sharply between its two openings. Conditional medians are about
1900 and1981 respectively, with broad uncertainty. This remains consistent with
the earlier roughly1900 working baseline; it neither proves the engineering work
ineffective nor qualifies conventional Chess as mature.

Accepted production source is `639b6465be384330c9fd7d49580c13bb0c314bf2`, integrated
with final engineering evidence at `4837f9d`. The assessment method, binaries and
fresh opening population were reviewed and published before games at `057f5db`.
MSH-147 (Chess Effectiveness Diagnosis) established the wider limitations. The
accepted round comprises MSH-148 (Deferred Frontier Work), MSH-150 (Lazy Native
Move Queries), MSH-145 (Pawn Threat and Exchange Judgment) and MSH-149 (Classical
King Exposure). No production source changed during the comparison.

## Actual results and uncertainty

W–D–L is from mShogi's perspective. Each opening was played with reversed colors.
References were the unchanged full-strength Oxidation0.6.0 and Goldfish1.13.0
binaries/options, at equal one-second thinking time, one worker, no pondering or
external tablebases, sequentially on the same Mac. Explicit memory-policy
differences remain: mShogi4096TT entries, Oxidation64MB, Goldfish16MB. The frozen
[method](chess-coherent-round-assessment.md) records selection and limitations.

| Reference | Scouts, 2 games | Fresh confirmation, 4 games | Confirmation score | Conditional median | 95% credible interval |
| --- | --- | --- | ---: | ---: | --- |
| Oxidation, anchor1943 | 1–1–0 | 1–1–2 | 37.5% | 1900 | 1728–2094 |
| Goldfish, anchor1982 | 1–0–1 | 2–0–2 | 50% | 1981 | 1692–2272 |

Across all **12 games: 5 wins, 2 draws, 5 losses**. All six pairs reached native
conclusions and passed native replay and process cleanup. No failures, technical
limits, exclusions or evaluation adjudications occurred. Every opening was fresh
relative to prior experimental attempts, including smokes and failures. Historical
root normalization excluded19 canonical roots; earlier experiments had no overlap.

Goldfish won both games in one confirmation pair and lost both in the other.
The four Goldfish games are analyzed as two opening pairs, preserving this
opening dependence rather than treating them as four independent observations
or evidence of a stable50% score.
The established pentanomial Dirichlet(.5) posterior is reported separately by
family and phase. No pooled rating or significance gate is introduced.

| Fresh confirmation versus MSH-144 | Conditional change median | 95% credible interval |
| --- | ---: | --- |
| Oxidation | approximately0 | −259 to+259 |
| Goldfish | +86 | −278 to+469 |

Both samples' uncertainty is included using independent posterior draws. Identical
reference source/binary/arguments/options/anchor checks permit these comparisons;
no interval endpoints were subtracted. Neither comparison supports a reliable gain
or regression claim. With .2/1 priors, Oxidation medians range1879–1916; Goldfish
stays near1982 but its .2-prior interval widens to1576–2391. Supplemental
anytime-valid score bounds remain[0,1]. Published anchor uncertainty and transfer
from CCRL conditions to this Mac add limitations outside these conditional ranges.
These are not official CCRL or human ratings. [Raw analysis](evidence/msh151/analysis.json)
retains scouts, confirmation, sensitivity and all historical comparisons.

## What the engineering and games establish

The broad diagnosis showed useful decisions can change with more thinking time,
but overhead and evaluation limitations interact. Deferring unused quiet
projections preserved exact results but did not robustly improve timing. Lazy
native existence queries reduced aggregate fixed-work time by12.26% while
preserving every result and search counter. Broad low-material LMR was rejected:
its additional depth lost an independently winning rook move. Larger TT capacity
provided no extra completed ply in the small tested panel, so it was not promoted
as the next fix. These are bounded observations, not universal conclusions.

Passer-interception and narrow conversion scaling improved selected independently
drawn scores; material-aware king danger now retains supported major-piece threats.
The final103-test search suite and independent endgame/tactical controls pass.
The baseline already chose WDL-preserving moves in those endgames, so improved
calibration is not a newly solved win. Drawn rook positions remain overoptimistic.
Engineering correctness, execution cost and local decisions are distinct from
playing strength; the fresh matches do not close that gap.

Across605 candidate turns,562 centipawn-scored turns completed nominal depths4–13.
There was no depth-one/high-work recurrence. Median completed depths were8 against
Oxidation and7 against Goldfish, versus reference nominal medians9 and10. Candidate
median final work was about231,000 and303,000 visits. Reference nodes have different
definitions, so their larger counts are not direct speed ratios. Reference depth
reports can describe root updates before a full iteration completes. The roots differ
from earlier games; these medians are descriptive, not matched-position gains.
Typical actual candidate responses were about1.002seconds, with maxima near1.0035.

The recorded games expose successes and remaining weaknesses:

- A Goldfish win converted connected passers through `c8Q` and `b8Q#`. Other wins
  reached native checkmate; no artificial evaluation cutoff awarded them.
- Exchanges were often already present in the PV before the evaluation worsened.
  In an Oxidation loss, `57.Rxc5` anticipated `...hxg3`, `Rxc4`, `...Rxf4` and
  `...g2`. In a Goldfish loss, `27.e4` anticipated the rook exchanges and `...d3`.
  This points to continuation/compensation judgment as an investigation area;
  it does not prove missed capture legality or identify the first losing move.
- Quiet invasion and king containment also matter. After `31...Ke8`, the recorded
  reply forecast was `Qa4`; actual `32.Rb8` pinned Bc8 to Ke8 and preceded a large
  score reversal. Independent board replay confirms the pin and absence of legal
  bishop moves, but does not prove that `Ke8` was the first error.
- Three independent tablebase checks prevent overclaiming: the selected
  `50...h2` preserved a draw despite the engine's earlier positive score; the late
  `46...Kg5` and `66.Kd3` positions were already lost with every legal move.
  Their selected moves preserved WDL. Board-tablebase knowledge is separate from
  complete-history native adjudication; these roots do not establish missed wins.

Full histories, exact score kinds, completed depths, work, PVs, material and
independent oracle alternatives are preserved in the single
[evidence record](evidence/msh151/verification.json). Score swings and opponent
opinions remain hypotheses unless supported by stronger evidence.

## Stopping and next work

Active local match work, including replay and cleanup, was **1162.665seconds
(19minutes23seconds)**. The initial900-second guard and its predeclared extension
to1800seconds total are preserved without resetting usage. Selection stopped after
two fresh confirmation openings per family as recorded before the last two pairs,
regardless of their outcomes. No further games or paid compute ran. A much larger
study could narrow the intervals, but another tournament is not the next priority.

MSH-152 (Classical Tactical Forecasting) records one bounded engineering follow-up:
compare forecast and actual continuations around exchanges, quiet attacks and
conversion, using complete histories and independent controls to distinguish
valuation from frontier/selectivity and useful-depth limits before correction.
This does not assume a new bottleneck or special-case the observed games.
Conventional Chess still needs meaningful practical improvement. No learned
evaluation, strength target, maturity declaration or Shogi/Xiangqi transfer is
selected here.

Independent final review reconciled exact sources and binaries, all six raw pair
receipts, native replay/cleanup, resource accounting, posterior uncertainty,
transcript telemetry and independent diagnostic evidence. No material findings
remain. Assessment acceptance records an honest measurement, not a strength
qualification or permission for cross-game transfer.
