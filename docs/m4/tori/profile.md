# Selected modern Tori Shogi profile

**Identity: `mshogi.tori.modern.v1`, edition MSH81-MODERN-2026-11-01.**
This is the complete planning contract for future implementation, selected under
John's 2026-11-01 modern-playing direction. It is not a claim that native Tori
support already exists. The earlier reserved `mshogi.tori.bunsei11.v1` was never
adopted and is not an alias. Preserve the [historical checkpoint](archive/2026-10-31/profile.md);
its acquisition questions no longer gate this profile.

## Basis and decision authority

Start from [Wikipedia revision1373875456](https://en.wikipedia.org/w/index.php?title=Tori_shogi&oldid=1373875456)
(2026-09-08), cross-checked against actual Hamburg2019 and Kanare2022 sheets,
Kanare's publisher explanation, and the Evans/Bodlaender modern rules page.
See [source ledger](sources.md) for exact copies/read limits. Wikipedia is a
modern candidate description, not an association code. Neither a historical
edition nor an engine silently governs missing clauses.

John explicitly permits autonomous coherent software conventions for rare edge
cases. Clauses marked **S** below exercise that authority; they are not attributed
to the sources. There is no unresolved category-A ordinary-play choice. Material
future contradictions must return to the owner rather than change this identity's
meaning. A rule change requires a new profile version and fixture review.

## T01 — board, sides and initial state

Seven files × seven ranks of squares. Files7→1 run left→right when viewed from
Sente's side; ranks a→g run top→bottom. Sente moves first, toward decreasing rank;
Gote faces the opposite way. Names are native Tori sides, not Chess colors.
Each side has Phoenix1, Falcon1, Crane2, Pheasant2, LeftQuail1, RightQuail1,
Swallow8. Hands start empty. Lower home rank7g→1g is LQ,Pt,Cr,Ph,Cr,Pt,RQ;
Falcon4f; Swallows1e..7e and3d. Rotate the complete arrangement180° and exchange
owner for Gote (extra Swallow5d). Thirty-two pieces total. T-M5 Setup, T-M3p2,
T-M4 diagram agree; T-H1img4 is supporting evidence only.

## T02 — native identities and movement

Use immutable kind names Phoenix鵬, Falcon鷹, Eagle鵰, Crane鶴, Pheasant雉,
LeftQuail左鶉, RightQuail右鶉, Swallow燕, Goose鴈 (historical鳫 is a display alias).
Only Falcon/Eagle and Swallow/Goose are promotion pairs. Quail handedness is
neither ownership nor promotion and survives capture.

Local offsets `(x,f)` mean mover-right and mover-forward; rotate180° for Gote.
Steps listed are exact endpoints. A ray stops before a friendly blocker or at its
first enemy capture. A jump ignores intermediate occupancy without capturing it.
No action captures more than its destination victim. Eagle's two-step diagonal
is a limited ray, never a jump or a two-capture route.

| Kind | Steps | Jumps | Rays |
| --- | --- | --- | --- |
| Phoenix | all8 adjacent | none | none |
| Crane | (−1,±1),(0,±1),(1,±1) | none | none |
| Pheasant | (±1,−1) | (0,2) | none |
| LeftQuail | (−1,−1) | none | (0,1),(1,−1) |
| RightQuail | (1,−1) | none | (0,1),(−1,−1) |
| Falcon | all adjacent except(0,−1) | none | none |
| Swallow | (0,1) | none | none |
| Eagle | (0,1),(±1,0) | none | (±1,1),(0,−1); limited(±1,−1) length1..2 |
| Goose | none | (±2,2),(0,−2) | none |

T-M5 Individual pieces, T-M3p2, T-M4 pieces/publisher prose. The Kanare quail
panel's opposite-handed picture does not override its prose and the explicitly
labelled pair in the other sources. No optional Tori+ pieces.

## T03 — complete actions, capture and promotion

One turn is one ordinary move or one drop, never pass. A move supplies origin,
destination and the resulting face required by this profile (the Rust spelling
may derive the forced face). Native authority determines capture/effects; reject
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
Swallow4b→4a becomes Goose. There is no hidden next-turn promotion timer.
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

## T05 — royal safety and native termination

Exactly one Phoenix per side in an admitted live diagram. No move may leave its
own Phoenix attacked. An attack means capture geometry and blockers of the
attacking face, including Phoenix adjacency; do not require that attacker's own
Phoenix could safely make that hypothetical capture (**S**, explicit threat-map
convention). Actual Phoenix captures are not admitted board actions: stop at
checkmate rather than ask a user to play a suicide or capture the royal.

On root installation and after each board action: evaluate board-legal replies
before repetition. An imported mate/stalemate diagram installs its immediate
conclusion; its root counts as occurrence1 and creates no inferred earlier cycle. Check
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
never a native win, draw or proof of mating possibility.

## T06 — repetition: explicit departure convention (S)

Modern sources say third occurrence and initiator varies, but do not supply a
machine-complete responsibility algorithm. This profile chooses **the player
moving out of the repeated position**, not its previous creator. This is one
of the documented modern interpretations; do not claim universal engine parity.

- Repetition key: every board square's owner/native face, both hands by distinct
  unpromoted kind/count, and side to move. Exclude counters, observations and
  runtime identity. Saved context still includes all those additional semantics.
- Count the admitted root as occurrence1; each accepted ordinary move/drop adds
  one position. Non-board events never add occurrences. Imported diagrams begin
  known history, with earlier history explicitly unknown; infer no repetitions.
- On a key's third or later occurrence, take its **latest three occurrence
  indices** `(i,j,k)`. The side to move there is the initiator/responsible side.
  Record an obligation at k: on this turn choose a complete action different
  from each actual first departure at i and j. Action equality includes the
  exact kind/drop, origin, destination and forced result face; identical hand
  copies are not distinct choices. This handles unequal and overlapping cycles
  without guessing intent or retrospectively punishing the previous creator.
- The arrival at k is accepted. An attempted forbidden departure is rejected
  without changing history or producing an illegal-move loss. A legal different
  departure consumes this obligation. Subsequent repeated keys may create new
  obligations; they are derived from full known board history, not a hash alone.
- If the responsible side has safety/drop-legal actions but **none** is an allowed
  variation, conclude repetition-forfeit loss immediately at k. If it has no
  board-legal reply, T05 mate/stalemate takes precedence. No forced self-check.
- No separate perpetual-check precedence: the same responsibility rule applies
  whether checking, evading or quiet. A continuous checker is not automatically
  blamed if the selected departure interpretation makes the evader responsible.
  A forced evader can lose by repetition forfeit. This is explicit software
  enforcement of the modern description's evader-may-have-to-stop case, not a
  claim that every tournament/engine uses this interpretation.
- Do not forbid a move merely because it creates the third occurrence. Do not
  import fourth-occurrence CSA draw, WXF warning/claim penalties, chess claims,
  check-evasion exemptions, or Fairy-Stockfish's repetition score.

Canonical records retain the occurrence indices, prior departures, obligation
and conclusion as replay-checked derived facts. History selection restores the
exact prefix; semantic undo restores occurrence lists and obligation exactly.
A side cannot evade responsibility by saving/reopening or adding observations.

## T07 — admission, roots, records and exact reversal (S)

A structurally admissible root has49 unique squares, exactly one Phoenix each,
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
prior moves, a repetition occurrence count or a move-clock adjudication. Native
record schema and transport version are separate explicit identities, chosen by
the native consumer owner, not dictated by a Shogi SFEN-shaped string.

Canonical native record requirements: game=tori; immutable profile/edition;
native schema; standard or explicit diagram root; root provenance (including
missing earlier history/counter meaning); complete ordered board and match events;
actor, complete action, resulting effects and derived obligations/conclusion;
current exact event prefix and board-ply mapping. Recompute effects/conclusions
from native replay and compare any serialized copies, never trust them. Resign
and administrative events do not masquerade as board moves or repetitions.
Strict outer dispatch must retain duplicate fields for native rejection; reject
unknown/conflicting profiles, schema, fields, malformed actions, unavailable hand
pieces, illegal geometry/safety, stale identity and resource violations atomically.

Apply/undo must restore board, hands, face, side, declared metadata, event/board
indices, occurrence ledger, obligation, conclusion and incremental caches exactly.
Undo/history/reopen cannot revive retired execution identities. Saved observations
retain exact target history, purpose, limits, native score perspective and proof
provenance where relevant; they are historical data, not pending work. Capacity
admission precedes every commit with actual encoded bytes/replay work and close/
stop reservation, preserving the existing complete-saveability guarantee.

## T08 — notation and consumer boundaries (S)

Native complete actions are authority. A future human adapter may use explicit
`Fa4c-4b+`, `LQ4d-2f`, capture `x`, drop `Sw*4b`, with side labels and exact source
square where needed. Coordinates are7×7 and quails remain distinct; no generic
9×9 SFEN/USI compatibility claim. Complete native record, not the display string,
is replay authority. Two-column Moves contains board actions only; associate
obligations/conclusions with their exact selected event prefix.

Initial functional native search uses the full selected history/obligations and
reports real results, incomplete/cancelled/failure unscored. No new exact graph
analysis/dead-position proof is required for this profile. Functional M4 engine
and measured M5 strength remain distinct. See [handoff](handoff.md) for actual
R1–R6 consumers and minimum required dispatch extensions.

## Disagreement and resolved gate ledger

| Earlier gate / category | Selected resolution and distinguishing fixture |
| --- | --- |
| P01 / C historical edition | Modern identity;1828/1833 publication relation not a gameplay dependency. Prior research preserved, no unread text represented as read. |
| P02 / apparent A wording | Explicit Wikipedia compulsory origin-or-destination-zone rule, corroborated contemporary compulsory descriptions; Hamburg ambiguous verb does not establish a coherent contrary optional rule. TORI-P-ENTRY/DROP-EXIT/DROP-OTHER/HEAD. |
| P03 / B rare cycles | T06 departure responsibility/count/sanction/forced-cycle convention explicit; not Fairy-Stockfish/Shokidoki equivalence. TORI-H-COUNT/INITIATOR/CHECK and forced/overlap tests. |
| P04 / B edge endings/digital policy | T05 mate stop, quiet immobility loss, no impasse/agreement, native resignation and unscored administration. TORI-O-* tests. |
| P05 / B software boundary | T07/T08 root/counter/strict canonical semantics, no reachability or protocol fiction. TORI-C-* and rejection/replay tests. |
| P06 / ordinary modern agreement | T02/T03 demotion/owner/handed-kind and nonjumping Eagle. TORI-CAPTURE-* and geometry fixtures. |

No remaining material owner decision or source acquisition blocks this complete
planning profile. Implementation, actual native conformance, engine execution,
archive capacity measurements and packaged integration remain future obligations.
