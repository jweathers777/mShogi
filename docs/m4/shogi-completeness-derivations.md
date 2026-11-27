# Shogi completeness fixture derivations

These derivations establish expected actions for the
[completeness fixtures](../../tests/fixtures/shogi/legal-completeness.json).
They use the movements, promotion and king safety rules of the
[selected profile](shogi-profile.md).

## Exact central/edge sets

There are 56 exact sets: eight base kinds and six promoted kinds, central/edge
placement, and both owners. Sente central pieces start at 5e, with K9h and the
opponent K2a; edge pieces start at 1e, with K9h and opponent K8a. King matrices
replace K9h by the test king. Opponent kings are off the initial attack rays and
too far away to constrain any listed king step. This avoids the unsuitable
default K1a diagonal in a central-bishop diagram.

For every non-king matrix, K9h independently contributes exactly
`9h8g 9h8h 9h8i 9h9g 9h9i`. Rotation transforms those to the five K1b choices.
All other moves must originate at the single test piece. The following totals
include those five auxiliary moves, except in the king rows:

| Test kind | Central complete choices | Edge complete choices |
| --- | ---: | ---: |
| King | 8 | 5 |
| Gold | 11 | 9 |
| Silver | 10 | 8 |
| Knight | 9 | 7 |
| Lance | 11 | 11 |
| Pawn | 6 | 6 |
| Rook | 24 | 24 |
| Bishop | 27 | 16 |
| Promoted pawn/lance/knight/silver, each | 11 | 9 |
| Horse | 25 | 16 |
| Dragon | 25 | 23 |

Gold and promoted minor movement use the six side-relative steps; silver uses
forward plus four diagonals; knight jumps two forward and one sideways; pawn
has one forward choice. Lance reaches four forward ranks, with optional
promotion on ranks c/b and compulsory promotion at a. The edge keeps that
forward ray and clips the other pieces' off-board steps without wrapping.

A central rook has 16 geometric ray targets and three extra promotion choices;
the edge rook still has 16 targets and three extra choices. A central bishop
has 16 diagonal targets and six extra promotion choices; the edge bishop has
eight targets and three extra choices. Horse adds orthogonal adjacent steps
without promotion suffixes: four centrally, three at the edge. Dragon adds
diagonal adjacent steps: four centrally, two at the edge. None of those steps
duplicates a ray move. All arrays, including the exact promotion suffixes and
both directions' clipped edge squares, match these independently listed facts.

## Blockers and empty legal sets

The bishop has a friendly P3c on its 4d–3c ray and an opposing S7g on its
6f–7g–8h–9i ray. It can reach 4d but cannot land on/pass P3c; it can capture
S7g after 6f but cannot pass it. Its other diagonal rays stay open. Only
7c/8b/9a permit promotion among those asserted destinations. These are selected
bishop assertions, not an exact whole-position set: the opposing silver also
attacks the auxiliary king's possible 8h destination. The mirrored assertions
preserve those occupancy and promotion facts.

For lance/friendly-pawn positions, the exact set consists of L5e–5d, P5c–5b
with either promotion flag and the auxiliary king's five moves. For the enemy
silver blocker, it consists of L5e–5d, L5e–5c with either flag and those five
king moves. Nothing crosses the blocker. Both rotated eight-choice sets are
correct, including gote promotion at g/h/i.

SH-F031's checked king K5a has four neighboring squares occupied by its own
L4a/L6a/P4b/P6b. Its remaining destination 5b contains the checking pawn and is
protected by G5c. No other defender captures that pawn, lance jumps are
unavailable, pawn advances leave the check intact, adjacent-pawn check cannot
be interposed against, and there is no hand. The legal set is exactly empty
with check true. Rotation establishes the opposite checked side.

SH-F032's K1a is not itself attacked by G2c/N3c. Its only destinations are
1b/2a/2b: the gold attacks 1b/2b and the knight attacks 2a. There are no other
owned pieces or drops, so its exact legal set is empty with check false. The
rotated K9i counterpart has the same proof. These four records assert board
check/choice facts only; they do not assign a CSA result or a chess stalemate
result.

## Independent shallow perft/divide proof

Both start orientations have the exact independently enumerated 30 actions:
nine pawns, two lances, four silvers, six golds, three king steps and six rook
shifts. The complete USI lists and divide keys were checked against those
groups and their rotation.

No first action captures, promotes, gains a hand, or moves an opponent piece
or blocker. Pawn advances end on f; all other sente actions stay on h behind
the pawn line. Thus gote retains exactly its initial movement opportunities.
The only potentially opened long attack is the bishop ray after 7g–7f:
8h–7g–6f–5e–4d–3c–2b. It misses K5a and all three 4b/5b/6b king destinations;
gote's own pawn/bishop occupancy still bounds that ray. The 9g–9f opening goes
to the board edge and cannot attack gote's king or its candidate destinations.
No first move checks or pins that king or invalidates any initial reply, and
none creates a new gote move.

Consequently every one of the 30 first actions has exactly 30 replies: perft
depth 1 is 30, each depth-2 divide entry is 30, and depth 2 is **900**.
Ownership reversal and board rotation prove the white-to-move version with
the same total. No deeper perft total is invented or approved by this report.
