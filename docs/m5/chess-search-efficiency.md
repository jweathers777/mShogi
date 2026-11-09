# MSH-143 (Chess Search Efficiency)

This increment removes measured redundant native work and fixes a demonstrated
quiet-frontier defensive error. Native rules/history admission, search limits,
draw claims and cancellation remain unchanged. It makes no playing-strength claim.

## Measured work and correction

A five-second frozen-source sample of five representative searches found whole
board structural validation in about11.7% of inclusive sampled stacks:263 of334
such samples came from temporary quiescence projections,43 from exchange
estimation. Categories overlap and the machine had correctness-test contention;
these are qualitative CPU attribution, not additive timings or throughput ratios.

Temporary search projections used public `Position::apply`, constructing public
undo data and rechecking complete structural invariants before discarding both.
The native Game already has a checked transition that resolves identical effects,
checks king safety and preserves identical errors without that redundant scan.
The new read-only `Position::project` exposes that same path. Quiescence and SEE
use it; actual search moves still enter the native cursor with complete history.
Exhaustive coordinate/promotion checks compare projection effects and errors to
full public apply, including castling, EP, pins and counter overflow.

In three sequential projection-only fixed-work comparisons, all16 positions
retained identical result/PV, visits and work counters. Aggregate times were
1116.84/1119.98/1128.74ms before and1086.39/1041.07/1028.13ms after. The first
15-position exploratory comparison was faster still; it is retained, not used
as a stronger repeatable speed claim. Different host scheduling or positions
may produce different changes.

Current SEE sampling was about3.2%, not the historical40% from an older engine.
Exact key comparison was about0.3%; genuine main and keyed-frontier TT cutoffs
were observed. These measurements did not justify a new SEE algorithm or TT
identity redesign. Recursive q branches remain unkeyed, preserving complete
history correctness. Further reuse needs a measured benefit/cost argument.

## Defensive frontier reliability

Independent position `r6k/8/5K2/8/6Q1/8/8/8 b - - 0 1` has exactly three moves
preventing every immediate White mate: quiet Ra7, Ra6+ and Rf8+. At the last
quiet-threat defensive layer, the old search instead selected Rg8, overlooking
non-contact Qh5/Qh4/Qh3 mate beyond its quiet-check cutoff.

After a defence at that final layer, search now checks the immediate native
replies for mate before accepting the resulting quiet position. It does not
extend recursive quiet-threat search. A discovered mating reply is still pushed
through the native cursor and scored by native terminal handling. Checks occur
after native claims/outcomes, poll cancellation for each candidate, preserve
the existing board-ply bound and occur only in an unkeyed q child. There is no
new board-only transposition bound or hidden time/depth limit.

The failed boundary case now passes, as do existing quiet attacks, three distinct
proved sacrifices, check evasions and root-restoration tests. Targeted tests cover
cancellation during the new reply scan and automatic fifty-move claim precedence.
This bounded safeguard does not make selective quiescence complete minimax.

## Practical measurements and acceptance

Three combined fixed-work runs took1069.38/1080.07/1050.99ms, roughly5% less in
aggregate than before. In the sequential one-second sample, completed depth and
scores were retained, with one additional completed depth on the rook-pawn and
KRKP examples. More visits reflect reduced overhead, not stronger play by itself.
The historical unique ...Re1 miss is not claimed solved by completing depth8.

The [single verification record](evidence/msh143/verification.json) retains frozen
binaries, inputs, raw profiles, actual tests, independent review and combined
MSH-142/MSH-143 platform receipts. MSH-142's separate endgame record preserves its
own source and measurements; both use the final combined native integration gate.
No opponent tournament was run during either engineering increment.

The next adaptive assessment uses fresh openings and independent rated opponents
to test the combined practical effect. KRKP, richer pawn-ending judgment and
broader classical evaluation remain substantial candidates depending on findings.
Neither this checkpoint nor algorithm presence establishes conventional maturity
or authorizes learned evaluation or transfer to Shogi/Xiangqi.
