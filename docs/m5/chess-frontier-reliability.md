# MSH-141 (Chess Frontier Reliability)

The Goldfish incident is reproducible and has two interacting causes: excessive
quiet-threat expansion inside quiescence, and a missed queen mate whose support
is revealed by the queen's own move. This is a general search defect and
selectivity problem, not evidence that conventional Chess has reached diminishing
returns. Native rules, history, outcomes and operational limits are unchanged.

## What the original telemetry means

The preserved MSH-139 (Chess Strength Assessment) request used the complete
81-ply history, ending at
`4r3/5r2/3p2k1/q1pP4/1pQ5/1Kb1N2P/4RP2/6R1 b - - 1 41`.
The candidate built from `9a509f0` reported depth1, seldepth23, 295,113 visits,
score+139, PV `g6h5`, and elapsed1001ms. The assessment's raw report and
`attack-inputs.json` remain unchanged.

`completed_depth` describes only the last fully completed iterative-deepening
iteration. Visits count calls to main search or quiescence across **all** attempted
iterations and re-searches, including interrupted work. Selective depth is the
greatest visited board ply across those attempts. Native legal-action generation,
exchange-estimation recursion, candidate previews and mate-query probes are
additional work, not visits. The UCI report was accurate; its depth and visit
fields describe different scopes.

New bounded iteration snapshots make that distinction inspectable without changing
the result protocol. A controlled full-history run at exactly295,113 visits,
using the original production behavior plus counters, returned the same depth,
score, choice and selective depth:

| Observation | Reproduction |
| --- | ---: |
| Main visits | 5 |
| Quiescence visits | 295,108 |
| Completed depth1 work | 7,734 visits, about71ms |
| Subsequent incomplete depth2 work | 287,379 visits |
| Check / null-threat extensions | 0 / 0 |
| Aspiration retries / PVS re-searches | 0 / 1 |
| Threatened q expansions | 8,615 |
| Immediate contact-mate admissions | 111,485 |
| Exchange probes | 46,967 |
| Recursive q TT cutoffs | 0 |

This allocation is reproduced evidence, **not counters retrospectively invented
for the original timed game**. Three additional uncapped one-second repetitions
all retained depth1/+139. The immediately earlier 39...Qxa5 position also stalled
at depth2/+233, so the problem is not confined to a single turn.

## Established causes and correction

Quiescence admitted quiet queen threats deep into its16-ply capture horizon and
expanded every legal defence whenever a contact-mate threat existed. Defensive
quiet moves could leave further threats, repeatedly branching as if in check.
Recursive q children intentionally have no TT key, so this large tree received
no recursive bound reuse. The counters and controlled changes identify this
as the dominant visit explosion; they do not constitute a CPU-flamegraph breakdown
of every native query.

Separately, `queen_contact_mate` rejected targets without support **before** moving
the queen. In the actual `Kh5 Qg4+ Kh6 Qg6#` sequence, Qg4 blocks Rg1's support
of g6 until she moves. The query now derives the checked resulting position first,
then checks support, check and absence of legal replies. Actual Game admission
still follows before search scores a native mate. Rook, bishop and queen revealed
support, color reflection, an unsupported queen, king escapes and a pinned queen
have independent regressions.

The search correction confines optional quiet checks and quiet threat creation
to the first two q layers, with a third layer to answer a threat created on the
second. Actual check evasions and immediate native contact mates remain available
at every layer, including the exhausted capture horizon. Deeper quiet plans belong
to main iterative search. This follows the established finite quiet-check frontier
approach in classical [Stockfish11 search](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/search.cpp)
and [move selection](https://raw.githubusercontent.com/official-stockfish/Stockfish/sf_11/src/movepick.cpp);
no source, tables or weights were copied. The selected two-layer policy is a
bounded engineering choice, not a claim of optimal tuning or complete tactics.

This changes search selectivity, **not thinking time, visits, nominal depth,
maximum board ply, native rules, repetition rights or result admission**. Existing
q TT identity already includes the remaining q budget, so the new policy has no
unrepresented cache parameter. No board-only bounds or new supervision system
were introduced.

Controlled intermediate runs are retained: the support fix alone expanded more
valid threats and failed to finish depth1 in300,000 visits; restricting only quiet
threat creation still failed. Restricting defensive expansion resolved that
feedback. Review added the final defensive response layer; the earlier first-two-
layers-only result is not the accepted implementation.

## Verification and practical effect

At the same295,113-visit ceiling, corrected search completes depth8 in **12,279
visits**, around46ms, returning Kf6 and a losing mate score with a legally replayed
mate PV. Depth1 completes in307 visits. This does not save an already lost game;
it removes the false optimistic score and allows productive main search. The
earlier Qxa5 error is no longer selected in the compared request.

Three sequential baseline/corrected rounds on the shared M1 Pro, macOS27.0.1,
use actual one-second hard/soft time, depth128 and **no visit ceiling**:

| Position | Original completed depth | Corrected completed depth |
| --- | --- | --- |
| Goldfish move39 | 2, 2, 2 | 7, 7, 7 |
| Goldfish move41 | 1, 1, 1 | 128, 128, 126; stable losing mate score |
| Standard start | 9, 9, 9 | 9, 9, 9 |
| Developed middlegame | 8, 8, 8 | 8, 8, 8 |
| Rook recapture | 8, 8, 8 | 8, 8, 8 |
| Rook/pawn endgame | 7, 7, 7 | 7, 7, 7 |
| Earlier historical queen-grab | 7, 7, 7 | 7, 7, 7 |

High depth after finding a mate mainly reflects mate-distance and transposition
cutoffs; **it does not mean128 ordinary board plies were searched**. The depth8
fixed-work comparison is the more useful incident measurement. These small
engineering samples establish neither an Elo gain nor general strength maturity.
All returned PVs were natively replayed and root state compared unchanged.

Tests retain the prior independently proved quiet mating attack, exact-mode oracle,
history/cache collisions, draw claims, representability failures, cancellation and
incomplete-result semantics. Added independent Qg4/Kh7/Qg7# fixtures in both colors
exercise quiet threat creation and the final defensive response layer. Check
evasions and revealed-support mate detection are tested beyond that boundary.

The single [verification record](evidence/msh141/verification.json) contains source
identities, binary hashes, raw profiles, commands, local and platform results and
independent review. Local engineering results and required CI remain separately
identified; no unperformed platform run is inferred. Existing GUI and game-specific
evidence is preserved; no packaged GUI campaign or opponent tournament is needed
for this search-only boundary correction.

## Continuing conventional Chess

MSH-142 (Classical Endgame Judgment) addresses demonstrated conversion/scaling
weaknesses. MSH-143 (Chess Search Efficiency) investigates remaining measured
native, exchange and safe cache costs. The lack of recursive q TT reuse remains
a possible measured opportunity, not permission to discard history identity.
Fresh adaptive strength reassessment belongs after meaningful coherent progress.
No learned evaluation, numerical strength target or Shogi/Xiangqi transfer is
authorized by this checkpoint.
