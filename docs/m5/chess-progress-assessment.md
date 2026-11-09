# MSH-144 (Chess Progress Assessment)

John requested a fresh adaptive reassessment after MSH-142 (Classical Endgame
Judgment) and MSH-143 (Chess Search Efficiency). The question is whether the
combined conventional improvements produce meaningfully stronger practical play
than the conditional local index around 1900 from MSH-139 (Chess Strength
Assessment). There is no target, minimum gain, maturity or cross-game-transfer gate.

Reuse the [established assessment method](chess-strength-assessment.md), native
runner, frozen rated reference releases and actual one-second-per-move conditions.
Native outcomes alone score games; no evaluation adjudication. Preserve failures,
incomplete pairs, exact binaries/options, process cleanup and full native replay.
This is a new ledger; historical MSH-139 results remain unchanged.

The initial tranche stays within **900 seconds cumulative active local match work**,
including failures, cleanup and replay. Start with one fresh reversed-color pair
against each of Oxidation 0.6.0 and Goldfish 1.13.0, the informative independent
families near the old estimate. Complete engineering/platform acceptance before
launching matches. Retain full-strength defaults, one worker, no pondering,
no external tablebases, no hidden move/depth handicap and no concurrent heavy
local work. The hard resource guard can leave an interrupted pair unscored;
never truncate a game into a draw or replace a failed pair silently.

Use the frozen opening population and its discovery/confirmation partitions.
Exclude every root used in historical attempts, including smoke, before new
task-seeded draws. Remaining draws are with replacement, including any duplicate
draws; record them rather than choosing favorable openings. Inspect complete
paired results before another batch. Competitive results favor fresh confirmation
against both families; clear mismatches may justify another eligible rated anchor.

Use the existing paired pentanomial Dirichlet(.5) score posterior, with .2 and 1
prior sensitivity, separately by family and phase. Conditional reference-index
mapping is not an official CCRL or human rating. For change estimates, draw the
old and new posteriors independently and propagate both uncertainties. Automatic
change estimates require the same reference source, executable, arguments, options
and rating anchor; disclose differences and omit the delta otherwise. Do not
infer gain from point estimates alone, subtract interval endpoints, pool family
disagreement or claim useful precision from a few games. Original discovery and
fresh confirmation evidence remain distinguishable.

Two initial pairs are expected to take roughly 8 minutes based on prior observations,
but this is not a guarantee. If they leave the development question unresolved,
use fresh confirmation under the owner-requested adaptive reassessment. After
the initial two pairs, a proportionate operational extension permits at most
1,800 seconds total active local work, including the first tranche; this is not
a fresh allowance. Stop earlier when evidence is useful. A substantial further
expanded campaign requires a concrete resource proposal. There is no automatic
repetition of the old 48-minute study.
Routine opponent selection, analysis and correctness work need no owner checkpoint.

Report actual scores, conditional ranges and their sensitivity, representative
mistakes and endgame/search limitations. An inconclusive initial result is useful
if stated honestly. No learned evaluation, final strength target or Shogi/Xiangqi
transfer is authorized by this assessment.
