# MSH-144 (Chess Progress Assessment): findings

**This reassessment does not establish a playing-strength gain.** Fresh confirmation
against both independent families still gives conditional local medians around
1900. Uncertainty permits improvement or regression; neither is proved. These
are local CCRL-referenced estimates under the recorded conditions, not official
CCRL or human ratings, and not evidence that conventional Chess is mature.

The accepted engineering source is `f9747769c6e0b4f08a2cb893bdaa696b1317faee`.
MSH-141 (Chess Frontier Reliability), MSH-142 (Classical Endgame Judgment) and
MSH-143 (Chess Search Efficiency) are complete. All three required native CI
platform jobs passed at the combined source; the separate MSH-141 macOS gate
also passed. [Engineering evidence](evidence/msh143/verification.json) retains
exact sources, independent review and historical infrastructure failures.

## Actual results

W–D–L is from mShogi's perspective. Each opening was played with reversed colors.
Scout and confirmation roots came from separate frozen partitions, excluding
every historical attempted root before sampling. Full-strength reference binaries,
options and rated anchors are unchanged from MSH-139. Thinking time was one second
per move, one worker, sequential local execution, with the same explicit memory
policies and native automatic draw-claim policy.

| Reference | Scouts, 2 games | Fresh confirmation, 4 games | Fresh score | Conditional median | 95% credible interval |
| --- | --- | --- | ---: | ---: | --- |
| Oxidation 0.6.0, anchor 1943 | 0–1–1 | 1–1–2 | 37.5% | 1900 | 1728–2094 |
| Goldfish 1.13.0, anchor 1982 | 2–0–0 | 1–0–3 | 25% | 1901 | 1644–2115 |

Across all **12 games: 4 wins, 2 draws, 6 losses**. Every game reached a native
conclusion. All six pairs passed native replay and process cleanup; none failed,
hit a technical limit, was excluded or used evaluation adjudication.

The initial Goldfish sweep did not repeat on fresh openings. Competitive results
and their uncertainty kept selection near the two informative independent families;
there was no convincing reason to move to a much stronger or weaker reference.

## Comparison and uncertainty

The established paired pentanomial Dirichlet(.5 per bin) model is applied separately
by family and phase. Historical and fresh posteriors are sampled independently;
automatic change estimates require matching reference source, binary, arguments,
options and anchor. Both samples' uncertainty is included.

| Confirmation comparison with MSH-139 | Conditional change median | 95% credible interval |
| --- | ---: | --- |
| Oxidation | +15 | −223 to +253 |
| Goldfish | −56 | −377 to +238 |

These intervals do not support a gain or a regression claim. Oxidation's raw
percentage is unchanged; its small positive median difference reflects unequal
sample sizes and prior shrinkage, not additional observed wins. With .2/1 priors,
current medians span 1879–1916 against Oxidation and 1859–1930 against Goldfish.
Goldfish's .2-prior interval widens to 1499–2099. Its small-sample lower tail is
particularly uncertain. Supplemental anytime-valid score bounds remain broad.

Published anchor margins and transfer between CCRL conditions and this Mac are
additional limitations, not included in those conditional intervals. No pooled
joint interval, official rating, significance threshold or predetermined gain is
claimed. [Raw analysis](evidence/msh144/analysis.json) preserves every phase and
sensitivity result, including the favorable scouts.

## What improved, and what remains

The engineering changes have verified benefits: bounded quiet-threat processing,
correct mating-threat and final-defence handling, exact advisory KPK knowledge,
narrow fortress scaling and conversion guidance, and about 5% lower aggregate
time in the combined fixed-work comparison. These are correctness and
performance results, not substitutes for playing-strength evidence.

In the retained independent eight-position endgame sample, the final one-second
choices preserve WDL in all eight positions; four are nondiscriminating. The
unique winning rook move ...Re1 was selected in one final depth-8 sample where
an earlier depth-7 sample chose a draw. Stable general rook-versus-pawn conversion
is unproved, and another theoretically drawn rook ending still scores +303.

No depth-1/high-work recurrence appeared in the 682 candidate turns. On
centipawn-scored turns, median completed depth was 7 against Oxidation and 8 against
Goldfish, with median final work about 188,000 and 230,000 visits. Reference nominal
depths were 9 and 10. Their roughly 1.09 million and 2.75 million reported nodes
use different definitions; these are not direct CPU-speed ratios or matched-root
before/after throughput measurements.

The games show both successful conversion and remaining problems:

- One Goldfish win converted connected passers into two queens and checkmate.
- Several losses involved advancing passed pawns and exchanges into difficult
  endings. In the final Goldfish loss, **44.Ne5** reported −18 at depth 7 and
  explicitly anticipated the bishop exchange, but its continuation missed a king
  interception found after **45.Kxe5**, at depth 11 and −227. Full histories and
  telemetry are retained in [diagnostic positions](evidence/msh144/diagnostic-positions.json).
- Another Goldfish loss featured queen/rook activity and king danger after
  `Ra7`, `Nxa5` and `Nxc6`. Recorded score disagreement identifies an investigation
  area, not a proved uniquely losing move.

These are actual sequences and search predictions. They do not establish the
first losing move or attribute individual wins/losses to a particular increment.
An apparently suspicious rook underpromotion was checked separately: every legal
promotion permitted the same king capture, so the underpromotion alone is not
proof of an error. Its 14-piece position was outside the small tablebase domain.

## Stopping and next work

Active match work, including replay and cleanup, was **1,313.607 seconds
(21 minutes 54 seconds)**. The initial 900-second tranche and its freeze are
preserved. Proportionate confirmation under the owner-requested reassessment used
a 1,800-second aggregate operational ceiling, without resetting prior usage or
changing the statistical/game conditions. No paid compute or further games ran.

The results remain consistent with the previous working baseline around 1900.
They are inconclusive about modest changes, but useful for the next engineering
decision: concrete threat, exchange and conversion weaknesses remain. A larger
study would be needed to estimate a small gain precisely; it is not being started.

MSH-145 (Pawn Threat and Exchange Judgment) records the next focused investigation,
with general independent fixtures and full-history reproductions before correction.
Queen/rook activity and king-safety judgment remain additional opportunities.
No diminishing-return, strength-target, learned-evaluation or cross-game-transfer
decision is made here. The [single assessment ledger](evidence/msh144/verification.json)
contains sources, methods, resource history, raw receipts and final review.

Independent final review reconciled every pair receipt, transcript-derived metric,
full diagnostic history, source identity, artifact hash and resource total. No
material findings remain. Assessment completion records an honest inconclusive
comparison of modest strength changes; it does not qualify Chess for transfer.
