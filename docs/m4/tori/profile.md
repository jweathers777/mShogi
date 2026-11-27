# Selected modern Tori Shogi profile

**Identity: `mshogi.tori.modern.v1`, edition MSH208-MODERN-2026-11-20.** The
earlier edition MSH81-MODERN-2026-11-01 differed only in T06 (see T06).
This is the complete rules contract selected by the recorded 2026-11-01
modern-playing decision. Support and delivered consumers are recorded in
the delivery review. The earlier reserved `mshogi.tori.bunsei11.v1` was never
adopted and is not an alias. Preserve the historical checkpoint;
its acquisition questions no longer gate this profile.

## Basis and decision authority

Start from [Wikipedia revision 1373875456](https://en.wikipedia.org/w/index.php?title=Tori_shogi&oldid=1373875456)
(2026-09-08), cross-checked against actual Hamburg2019 and Kanare2022 sheets,
Kanare's publisher explanation, and the Evans/Bodlaender modern rules page.
See [source ledger](sources.md) for exact copies/read limits. Wikipedia is a
modern candidate description, not an association code. Neither a historical
edition nor an engine silently governs missing clauses.

The approved scope permits autonomous coherent software conventions for rare edge
cases. Clauses marked **S** below exercise that authority; they are not attributed
to the sources. There is no unresolved category-A ordinary-play choice. Material
future contradictions must require an explicit decision rather than change this identity's
meaning. A rule change requires a new profile version and fixture review.

## T01 — board, sides and initial state

Seven files × seven ranks of squares. Files7→1 run left→right when viewed from
Sente's side; ranks a→g run top→bottom. Sente moves first, toward decreasing rank;
Gote faces the opposite way. Names are Tori sides, not Chess colors.
Each side has Phoenix 1, Falcon 1, Crane 2, Pheasant 2, Left Quail 1, Right Quail 1,
Swallow 8. Hands start empty. Lower home rank 7g→1g is LQ,Pt,Cr,Ph,Cr,Pt,RQ;
Falcon 4f; Swallows 1e..7e and 3d. Rotate the complete arrangement 180° and exchange
owner for Gote (extra Swallow 5d). Thirty-two pieces total. T-M5 Setup, T-M3p2,
T-M4 diagram agree; T-H1img4 is supporting evidence only.

## T02 — identities and movement

Use immutable kind names Phoenix鵬, Falcon鷹, Eagle鵰, Crane鶴, Pheasant雉,
LeftQuail左鶉, RightQuail右鶉, Swallow燕, Goose鴈 (historical鳫 is a display alias).
Only Falcon/Eagle and Swallow/Goose are promotion pairs. Quail handedness is
neither ownership nor promotion and survives capture.

Local offsets `(x,f)` mean mover-right and mover-forward; rotate 180° for Gote.
Steps listed are exact endpoints. A ray stops before a friendly blocker or at its
first enemy capture. A jump ignores intermediate occupancy without capturing it.
No action captures more than its destination victim. Eagle's two-step diagonal
is a limited ray, never a jump or a two-capture route.

| Kind | Steps | Jumps | Rays |
| --- | --- | --- | --- |
| Phoenix | all 8 adjacent | none | none |
| Crane | (−1,±1),(0,±1),(1,±1) | none | none |
| Pheasant | (±1,−1) | (0,2) | none |
| LeftQuail | (−1,−1) | none | (0,1),(1,−1) |
| RightQuail | (1,−1) | none | (0,1),(−1,−1) |
| Falcon | all adjacent except(0,−1) | none | none |
| Swallow | (0,1) | none | none |
| Eagle | (0,1),(±1,0) | none | (±1,1),(0,−1); limited(±1,−1) length 1..2 |
| Goose | none | (±2,2),(0,−2) | none |

T-M5 Individual pieces, T-M3p2, T-M4 pieces/publisher prose. The Kanare quail
panel's opposite-handed picture does not override its prose and the explicitly
labelled pair in the other sources. No optional Tori+ pieces.

## T03 — complete actions, capture and promotion

One turn is one ordinary move or one drop, never pass. A move supplies origin,
destination and the resulting face required by this profile (the Rust spelling
may derive the forced face). Session authority determines capture/effects; reject
invented victims, extra routes, optional nonpromotion or promotion of other kinds.
A capture transfers the victim into the mover's hand: Eagle→Falcon, Goose→Swallow,
all other nonroyal kinds unchanged, including LeftQuail/RightQuail. Ownership
changes; handed movement rotates with the new owner. Phoenix never enters a hand.

Promotion zone is the farthest two ranks: a/b for Sente, f/g for Gote. An
unpromoted Falcon or Swallow **must** promote after a board move whose origin OR
destination is in the zone. Resolve geometry using the face before moving,
then capture, then forced promotion, then assess the resulting attacks. No
out-of-zone capture or 'in front of Phoenix' promotion privilege. Drops never
promote. A zone-dropped Falcon remains Falcon while some other piece moves;
on its own next move within or out of the zone it becomes Eagle. A zone-dropped
Swallow 4b→4a becomes Goose. There is no hidden next-turn promotion timer.
T-M5 Promotion/Drops is explicit on these cases; T-M4 compulsory entry and
T-M6 wholly/partly-zone language corroborate. Hamburg's *kann* is ambiguous
wording, not the selected optional-promotion rule.

## T04 — hands and drops

Drop one available unpromoted nonroyal hand piece on an empty in-board square,
remove one from that owner's hand and end the turn. No capture, promoted drop or
owner swap supplied by a caller. Falcon and other non-Swallow pieces may drop on
the last rank if royal safety holds; there is no generic pawn/lance/knight rule.
Swallow drops additionally forbid:

1. the mover's farthest rank;
2. a file already holding two of the mover's unpromoted Swallows (enemy Swallows
   and friendly Geese do not count);
3. immediate mate delivered by the dropped Swallow's own attack.

For(3), test the resulting direct Swallow check and all otherwise legal defender
board actions, including captures, king moves and counter-drops. If none evades
check, reject the drop atomically. A discovered mate from a nonchecking Swallow
drop is not this prohibition. Defender Swallow counter-drop must itself satisfy
the same direct-check drop-mate restriction; a direct adjacent Swallow check
cannot be interposed, avoiding circular mate-test assumptions. Repetition
obligations do not turn an otherwise evadable check into prohibited drop-mate:
use royal-safety/drop legal replies before history enforcement (**S**, separation
of board legality from repetition sanction). Mate by another dropped kind is
legal. T-M5 Drops; T-M3p1; T-M4 rules; T-M2 narrow two/three corroboration.

## T05 — royal safety and termination

Exactly one Phoenix per side in an admitted live diagram. No move may leave its
own Phoenix attacked. An attack means capture geometry and blockers of the
attacking face, including Phoenix adjacency; do not require that attacker's own
Phoenix could safely make that hypothetical capture (**S**, explicit threat-map
convention). Actual Phoenix captures are not admitted board actions: stop at
checkmate rather than ask a user to play a suicide or capture the royal.

On root installation and after each board action: evaluate board-legal replies
before repetition. An imported mate/stalemate diagram installs its immediate
conclusion; its root counts as occurrence 1 and creates no inferred earlier cycle. Check
with no safety/drop-legal reply is checkmate, a win for the mover. No check and
no such reply is **stalemate loss for the immobile side (S)**, not Chess draw.
Checkmate terminology T-M5/T-M3/T-M4 supports stopping at the inevitable royal
loss; atomic digital rejection replaces physical illegal-move forfeiture.
Mate by moving a Swallow must use its resulting promoted face, not pretend the
unpromoted attack survives promotion.

**S exceptional endings:** entering both enemy camps does not end play; there is
no impasse point count/declaration, material-only/dead-position adjudication,
move-count draw, evaluation resignation or agreement draw. Agreement commands
are unsupported (Hamburg expressly disallows them). A side may resign, including
out of turn, while live; resignation ends the game as the opponent's win without
incrementing board ply. Once concluded, further actions/terminal commands reject
atomically. No timed games in initial delivery. Administrative stop, resource
exhaustion, engine failure or exhausted experiment limit is incomplete/unscored,
never a win, draw or proof of mating possibility.

## T06 — repetition: third occurrence refused (decided)

**Decision (John, 2026-11-20, MSH-208 (Tori Repetition Rule)).** The rule works
like the Shogi repetition rule, with two differences: three occurrences instead
of four, and the result is not a draw — the move that would create the third
occurrence is not allowed and a different move must be chosen. Sources named by
John: a 2022 Japanese rule book (同じ手が3度繰り返された場合、その動きを始めた側の
プレイヤーが手を変える必要があります), the PyChess Tori page (same position three
times with the same player to move; the player starting the sequence must vary;
pieces in hand are part of the position; not a draw as in Shogi), and Okano,
禽将棋 (2008, Osaka University of Commerce bulletin 10), cited by the Japanese
Abstract Games Museum as "third occurrence; change the repeating move".

- Repetition key: every board square's owner/face, both hands by distinct
  unpromoted kind/count, and side to move. Exclude counters, observations and
  runtime identity. Saved context still includes all those additional semantics.
- The positions compared are all positions of the known history, wherever they
  occurred (as in Shogi), not only consecutive moves. Count the admitted root as
  occurrence 1; each accepted move/drop adds one position; non-board events add
  none. An imported diagram begins known history; no earlier repetition is inferred.
- An action whose resulting key already occurred twice is refused without
  changing state (`ThirdOccurrence`); it is not an illegal-move loss. A key
  therefore occurs at most twice.
- Who must break the loop (John, 2026-11-20): "Find the earliest position in
  the loop that later comes back. The player whose move first produced it
  started the repetition, and that player must break the loop." A position
  always has the same side to move, so the player who produces it is always the
  other side, and the earliest position of a loop is the first to come back
  each time around. The refused action above is therefore always the starter's.
  A loop that begins at the root (the standard start or a set-up diagram),
  which no move produced, counts the root as produced by the player who is not
  to move (John, 2026-11-20).
- If the side to move has board-legal actions but every one is refused by this
  rule, that side loses at once (`repetition_forfeit`). If it has no board-legal
  action, T05 checkmate or quiet-immobility loss takes precedence. There is no
  separate perpetual-check rule; check and quiet cycles are treated the same.
- Records store the occurrence list of each event (replay-checked). The set of
  refused actions is derived from history on replay and is not stored.
- The shared repetition index locates earlier positions by hash and checks the
  full Tori key on each match.

The earlier edition (MSH81-MODERN-2026-11-01, schema 1) let the move that created
the third occurrence stand and obliged the other side to depart differently; an
earlier session derived that convention, it was never decided by John. That development-era format is no longer supported.

## T07 — admission, roots, records and exact reversal (S)

A structurally admissible root has 49 unique squares, exactly one Phoenix each,
valid owners/kinds, nonnegative hand counts of base nonroyal kinds, at most the
initial total of each base kind across board+hands (Ph2,Fa2,Cr4,Pt4,LQ2,RQ2,Sw16),
no unpromoted Swallow stranded on its last rank or more than two own unpromoted
Swallows on a file, nonadjacent Phoenixes, and no attack on the nonmoving side's
Phoenix. Missing material is permitted only in explicitly marked diagram/custom
roots, not standard-start records. Unpromoted zone-dropped pieces are admissible;
a diagram alone does not prove they were dropped or historically reachable.
Exactly two sides; odd custom roots may start Gote with explicit provenance.

Declared completed-ply counters are immutable nonnegative root metadata. Track
accepted board plies separately from zero; current declared total, if displayed,
is root counter plus accepted board plies. Terminal/control events add neither.
These counters are not proof of
prior moves, a repetition occurrence count or a move-clock adjudication. Game
record schema and transport version are separate explicit identities, chosen by
the consumer owner, not dictated by a Shogi SFEN-shaped string.

Canonical game record requirements: game=tori; immutable profile/edition;
record schema; standard or explicit diagram root; root provenance (including
missing earlier history/counter meaning); complete ordered board and match events;
actor and complete action. Schema 4 reconstructs effects, conclusions, event
prefixes and board-ply mapping by game replay instead of storing copies. Resign
does not masquerade as a board move or repetition. Operational interruptions
stay in session memory and add no game event.
Strict outer dispatch must retain duplicate fields for rejection; reject
unknown/conflicting profiles, schema, fields, malformed actions, unavailable hand
pieces, illegal geometry/safety, stale identity and resource violations atomically.

Apply/undo must restore board, hands, face, side, declared metadata, event/board
indices, occurrence ledger, obligation, conclusion and incremental caches exactly.
Undo/history/reopen cannot revive retired execution identities. Saved observations
retain exact target history, purpose, limits, reported score perspective and proof
provenance where relevant; they are historical data, not pending work. Records
have no byte or event limit (MSH-268); compact root and command sizes stay
bounded, so every event prefix stays saveable.

## T08 — notation and consumer boundaries (S)

Complete actions are authority. A future human adapter may use explicit
`Fa4c-4b+`, `LQ4d-2f`, capture `x`, drop `Sw*4b`, with side labels and exact source
square where needed. Coordinates are 7×7 and quails remain distinct; no generic
9×9 SFEN/USI compatibility claim. Complete game record, not the display string,
is replay authority. Two-column Moves contains board actions only; associate
obligations/conclusions with their exact selected event prefix.

Initial functional search uses the full selected history/obligations and
reports real results, incomplete/cancelled/failure unscored. No new exact graph
analysis/dead-position proof is required for this profile. Functional M4 engine
and measured M5 strength remain distinct. See handoff for actual
R1–R6 consumers and minimum required dispatch extensions.

## Disagreement and resolved gate ledger

| Earlier gate / category | Selected resolution and distinguishing fixture |
| --- | --- |
| P01 / C historical edition | Modern identity;1828/1833 publication relation not a gameplay dependency. Prior research preserved, no unread text represented as read. |
| P02 / apparent A wording | Explicit Wikipedia compulsory origin-or-destination-zone rule, corroborated contemporary compulsory descriptions; Hamburg ambiguous verb does not establish a coherent contrary optional rule. TORI-P-ENTRY/DROP-EXIT/DROP-OTHER/HEAD. |
| P03 / B rare cycles | T06 departure responsibility/count/sanction/forced-cycle convention explicit; not Fairy-Stockfish/Shokidoki equivalence. TORI-H-COUNT/INITIATOR/CHECK and forced/overlap tests. |
| P04 / B edge endings/digital policy | T05 mate stop, quiet immobility loss, no impasse/agreement, resignation and unscored administration. TORI-O-* tests. |
| P05 / B software boundary | T07/T08 root/counter/strict canonical semantics, no reachability or protocol fiction. TORI-C-* and rejection/replay tests. |
| P06 / ordinary modern agreement | T02/T03 demotion/owner/handed-kind and nonjumping Eagle. TORI-CAPTURE-* and geometry fixtures. |

No remaining material project decision or source acquisition blocks this complete
planning profile. Implementation, actual rules conformance, engine execution,
archive capacity measurements and packaged integration remain future obligations.
