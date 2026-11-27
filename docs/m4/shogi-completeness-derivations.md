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

## Selected board-rule fixtures

These cases are in the [board-rule corpus](../../tests/fixtures/shogi/board-rules.json).

- The standard start's 30 choices follow its blocked placement: nine pawn,
  two lance, four silver, six gold, three king and six rook moves. Knights and
  bishop have no available destination.
- Side-relative movement and promotion assertions use sente's a–c and gote's
  g–i camps. Entry, movement within and exit permit both flags where the piece
  remains live; pawn/lance on the last rank and knight on the last two require
  promotion. The lance rank-three/rank-two nonpromotion and P/L/N/B/R check
  evasions remain complete choices. Moves wholly outside camp cannot promote;
  promoted P/L/N/S use gold movement and cannot promote again.
- Rook rays stop before the friendly pawn and on the capturable opposing
  silver. Knight jumps survive intervening own pieces. Mirrored cases reverse
  forward direction, ownership and camp without changing these facts.
- Capturing each of the six promoted kinds with a promoting rook leaves a
  dragon on the destination and exactly that captured base kind in the correct
  hand. Every listed drop creates an unpromoted owned piece and spends one
  hand piece; two identical rooks do not create two distinct USI choices.
  Empty final-rank pawn/lance and final-two-rank knight exclusions cover both
  sides. Occupied targets, absent resources, wrong-owner moves and same-square
  moves are absent. Those membership checks do not establish atomic
  rejection or state preservation for invalid product requests.
- Nifu uses an own unpromoted pawn on that file. An own tokin and an opposing
  pawn do not trigger it. Both orientations' asserted alternatives are safe
  from unrelated king checks.
- In SH-F015, dropping P5b checks K5a; its four other adjacent destinations are
  occupied, G5c protects 5b, and no defending piece can capture that pawn.
  Removing L6a permits king escape; removing G5c permits king capture; replacing
  P4b by G4b permits a defending capture. Adding B3c pins that G4b to K5a, so
  its proposed capture exposes the king and no longer saves the pawn drop.
  Existing-pawn advance and gold-drop mates remain legal board actions. The
  rotated gote constructions preserve each escape/capture proof.
- SH-F018 permits a pinned gold to stay on the checking rook's file and
  excludes leaving it. Single-check cases separately admit king escape,
  checker capture with either promotion flag, interposition and a blocking
  drop; irrelevant actions remain absent. Double-check interpositions still
  leave the other line open. Capturing a bishop-defended rook with the king is
  forbidden.
- BOARD-PIN's origin x-ray is explicit: with K5h/R5a, moving K5h to 5i empties
  the origin and leaves the complete rook ray open, while 6i is off it. In the
  pinned-attacker contrast, S4c geometrically attacks 5d although moving that
  silver would expose its K4a to R4i; K5e may therefore not enter 5d under the
  selected geometric-influence contract. K5e–6e is safe. This is the profile's
  stated operational interpretation supported by independently checked
  geometry, not a claim that JSA explicitly names this rare configuration.
- The nested sequence captures a tokin into a base-pawn hand, shuffles the
  opposite king, spends the pawn on a drop, then moves the already promoted
  rook without demoting it. Both final SFEN strings and next-ply 6 were checked
  independently. No earlier history or outcome follows from the supplied
  counter.
