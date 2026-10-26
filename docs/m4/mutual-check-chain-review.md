# Independent complete-chain mutual-check review

Reviewed 2026-10-26 by the fresh `proof_chain_review` AI research session for
MSH-67. **Accepted:** no nonempty closed sequence of legal ordinary shogi
board actions can give check on every action by both sides in the live-root
domain below. This accepts the reconstructed argument here, including the
supplement, rather than treating Nanka's published proof or earlier AI reviews
as authority. It does not certify the implementation or close MSH-67 by itself.
The original incomplete review remains historical evidence.

## Domain and inspected identities

The domain is a 9 by 9 ordinary shogi board with exactly one king per side,
standard combined board/hand base-piece stock bounds (in particular at most
two bishops and two rooks), and legal complete moves/drops with ordinary
promotion, capture/demotion and geometric king safety. A live root's nonmoving
king is unattacked; its moving king may be checked. Missing material and
unknown prehistory are allowed. No initial-placement reachability assumption
is used. The broader structural analysis-position API is outside this claim.

The source revision reviewed was
`0d1631db8fe4adac3ac0a9e5aa6956a863a3ba1c`, preserving product implementation
`f259bb23834d6156f21aeec53db9ee00ef7242e4`. Inspected requirements were
`AGENTS.md`, `docs/design.md`, `docs/m4/shogi-profile.md`, the current MSH-67
record and the owner's supplied continuation instruction. Mathematical inputs:

- Nanka, third edition dated 2024-03-09, *将棋およびチェスにおいて双方連続王手の千日手が成立しないことの証明*,
  [author-linked PDF](https://drive.google.com/file/d/112O0pL7sObJCkxaviVEEHNerLvTwpXCD/view).
  Inspected `/tmp/mshogi-mutual-check-proof.pdf`, SHA-256
  `cd5f987133d89a32bd6de8391015354b16d066a97cea7f719b67a5089275e065`.
  All rendered pages 2-11, covering sections 2-5 and Table 1, were visually read;
  the incomplete Japanese text extraction was not substituted for those pages.
- `mutual-check-proof-review.md`: historical incomplete result, including its
  concrete mixed-radius candidate and failed source-inventory arguments.
- `mutual-check-supplement/mixed-radius-lemma.md`, SHA-256
  `6654d34fb114a97b625c47fb59ac51d9abd023c0803c3e30bccdcfabb069e7cf`;
  `case-accounting.md`, SHA-256
  `4f62fff2e7c73986912233d4754c8bbf2a4172b2d36d6343be57e74f5554fc88`.
- The two independent geometry implementations, `directed_geometry.py`
  (`d9cb38bb360ede68a8424e1c8036180f1b0ece79ce25712d60bf4bd7560c69cc`)
  and `independent_geometry.py`
  (`218e13fcd6578725a65a3f8edfbe98809d2044bd10f5bd8f6d55cb7f91eba4d2`),
  and their comparison script `verify_geometry.py`
  (`599dc02322a15ee45769695430628939395a4acfdbee3ef00c488b09c838221b`).

Method: reconstruct each implication from movement and occupancy, inspect the
published diagrams and case table, check the supplement line by line, inspect
both scripts' filters and cycle handling, run their comparison, and audit the
live-root/replay assumption boundary in the preserved source. No engine's
repetition heuristic, attachment review assertion or game-state search was
used as mathematical authority.

## Closed interval and fixed kings

Suppose the claimed history exists. Consecutive equal repetition positions
bound a nonempty closed sequence preserving board ownership/promotion, both
hands and turn. Its every action checks. The action sequence can be continued
periodically, because its semantic position repeats; this is a proof device,
not an assertion about unknown play before an imported root.

A king cannot directly give check legally. A checking king move must uncover
a sliding check, so the two kings before that move lie on one orthogonal or
diagonal line. The evading king must leave this line to uncover it. It cannot
land on a different orthogonal/diagonal line through the opposing king in one
step unless the kings become adjacent (the intersections within one king step
are near the opposing king). Such adjacency is illegal. Afterwards neither
king can make another checking move: that would again require alignment.
Their original positions could never be restored. Thus both kings are fixed.

Every position in this closed sequence has exactly one active checker. There
is at least one because every action checks, including the final action into
the initial semantic position. With kings fixed, no legal next action can
evade two checks: distinct rays toward the king meet only at the king; a
checker on another checker’s ray would already block it; a knight check must
be captured at its separate square. One nonking move/drop cannot capture or
interpose both. Therefore a double check cannot occur in the presumed cycle.

A nonking evasion consequently either captures this checker or interposes
strictly on its ray. Capturing an unrelated enemy piece cannot interpose:
that piece would already have blocked the alleged incoming ray. Drops are
interpositions, and placing a piece cannot uncover a friendly attack, so each
checking drop is a direct check.

## Gaze, persistent discovery blockers and sources

Use the PDF's definition: a piece gazes at the opposing king if it attacks
after removing every other movable piece but retaining that piece and all
pieces that neither move nor are captured in the chosen cycle. Equivalently the
retained pieces are the immobile obstacles; an actual checker also gazes.
This is geometry relative to the entire hypothetical sequence, not a new
product API.

A nongazing destination cannot give a direct check, so moving there must
discover one. Conversely, a gazing piece cannot discover a check when it
moves. It was not checking the nonmoving enemy king before the move, so a
movable obstacle lay between it and that king. That obstacle would still
block any sliding source behind its departure square. Non-ray gaze (knight
or adjacent step) cannot provide another discovered line through that square.
This also handles a promotion on the move: if the destination could not
check, neither direct nor discovered check would be available.

Thus a gazing piece cannot move to a nongazing square, and a drop cannot
create a nongazing piece. The number of nongazing on-board pieces cannot
increase. Moving one to a gazing square or capturing one would decrease it.
The repeated semantic board restores this count, so neither can occur.
Nongazing pieces are consequently uncapturable, remain on board with fixed
ownership, and every move of one is a discovery. Their total promoted count
cannot decrease (demotion requires capture), so none can promote in this
closed sequence either. Interchangeable physical labels need not return to
individual starting squares; these arguments use counts.

At each square occupied by a moving discovery blocker, some own sliding
source must already gaze through that square toward the enemy king. To see
this, follow the periodic continuation until that square is next vacated:
the departure must discover a check. A source could not have been introduced
while the blocker stayed there: arrival by move/drop would have to check,
yet the blocker would obstruct it. Nor could an established source depart
while the blocker stayed and be replaced before that departure. The source
therefore exists when the blocker arrives and remains while it stays. It is
not captured then because the blocker prevents it from being the checker.
In particular, for a bishop blocking an orthogonal ray, the own rook/dragon
count behind it cannot increase while it remains: own move/drop arrival there
would have to directly check but the bishop blocks it, and opposing actions
cannot create own ownership there. The source needed at its next departure
must therefore already be present on arrival. This supplies the simultaneous
inventory argument used below; it is not an assumption of permanent ownership.
The departing and arriving rays of a discovery move have distinct sources
simultaneously; a piece cannot move through its own source or the fixed king
to discover a check on the same ray. Static obstacles only restrict these
rays further.

## King line, half-plane invariant and discovery necessity

A discovery blocker's square is never on the line joining the kings. If it
were, gaze by its own source and a possible evasion against the other king
would put the square between the kings. Its own source, between that square
and its own king, would already block the incoming check that arrival there
purports to evade. This is PDF proposition 3.5's occupancy contradiction.

An active checker is also never on that line. An evasion cannot be a discovery
landing there by the preceding result. A direct interposition between checker
and checked king could not check back through the old checker or a fixed
king. Only capture at the checker square could check back; repeating the
reasoning forces successive captures on that same square and strictly
decreases board material, incompatible with a closed sequence. If the fixed
enemy king intervenes, even that capture response cannot check back.

Let H be either open half-plane bounded by the king line. Count all moving
persistent discovery blockers in H plus the active checker if it is in H.
This count is invariant. A response lands in the incoming checker's
half-plane (on the capture square or its ray to the checked king). A direct
response replaces the old checker by the new one in that half-plane. A
discovery response uncovers a source in the half-plane of the blocker’s
departure: crossing into H adds one blocker and removes the checking unit
from H, and crossing out does the reverse. Same-half-plane moves change
neither total. A direct drop replaces rather than supplements the prior
checking contribution. Captures remove only the old checker, not a persistent
blocker. This proves the invariant with captures/drops/source changes.

Each side must make a discovery. Suppose T never does, and examine S's moves
with enemy king K and own king L. Measure the unsigned angle at K from KL
to the active checker (strictly between 0 and 180 degrees by the king-line
result). T's direct response lies on the old S checker's ray from K, so it
preserves that angle. For S's evasion landing at C, write its checker-angle
change as `ang = iang + dang`, where `iang` is angle(C)-angle(old checker)
and `dang` is angle(new checker)-angle(C). Capture gives `iang = 0`;
strict interposition gives `iang < 0`. A direct move/drop has `dang = 0`.
A discovery moving a blocker from D to C has
`dang = angle(D)-angle(C)`, because its new source lies beyond D on the
enemy-king ray. No on-board origin for a drop is required; assigning D=C
gives the same zero direct term.

The checker-angle changes telescope to zero around the cycle. For discovery
moves, the sum of `dang` is the decrease of the sum of angles of S's complete
nongazing-piece multiset. That multiset is restored by the repeated semantic
board, so this sum is zero. Individual labeled pieces need not return.
Meanwhile S cannot use captures only: each would increase its hand material,
and restoring the hand requires a drop, hence an interposition. Therefore
`sum(iang) < 0`, a contradiction. Exchange sides to obtain a discovery on
both sides. Checking sources may change freely in this argument.

## Piece-stock reduction

An uncapturable persistent blocker must participate in a closed on-board
walk. Pawns, lances and unpromoted knights move only forward, so cannot do so;
their promoted forms have gold movement. Intersecting the permitted sliding
source rays with nongazing movement gives these possibilities (S forward is
positive y and the opposing king is the origin):

- Gold movement admits only the nongazing transition from `(±1,1)` to `(0,2)`;
  no return to the earlier ray is available. Thus no gold or promoted minor
  piece is a discovery blocker.
- Silver movement can cycle only on edges `(1,0) <-> (0,1)` or its horizontal
  reflection. These form a tree; any closed walk using an edge traverses both
  directions. The king must be attackable from both endpoints by enemy
  geometry and must pass the evasion/path and king-line constraints. The only
  nonadjacent viable own-king locations are `(2,1)` and `(1,2)`. For example
  the additional knight-geometry candidate `(0,-2)` puts endpoint `(0,1)` on
  the king line and is excluded by the proved king-line lemma. At `(2,1)`, the
  move `(1,0) -> (0,1)` requires two S rook/dragon sources plus an incoming
  T rook/dragon simultaneously. At `(1,2)`, use the reverse move. Required
  vertical directions exclude a substituting lance. This violates two-rook
  stock; horizontal reflection covers the other edge.
- Rook/dragon nongazing discovery moves connect diagonal corners
  `(±t,±t)` at a fixed positive radius. A dragon's extra step cannot add a
  different usable ray transition; adjacent diagonals already gaze. At such
  a transition two distinct own bishops/horses must simultaneously supply
  the two diagonal sources. The opposing discovery blocker cannot be a
  bishop/horse: those persistent nongazing pieces cannot coincide with these
  gazing sources. With gold/silver excluded it too would be a rook/dragon,
  and even while stationary it needs an opposing bishop/horse source at its
  current square. That requires three bishops simultaneously, impossible.

Only bishops/horses remain. Each side needs one; standard stock gives exactly
these two persistent discovery blockers and no additional bishop/horse source.
Reduced material either lacks them and contradicts discovery necessity sooner,
or obeys the same two-bishop/two-rook restrictions. No lifetime allocation of
capturable gazing pieces is assumed.

## Conditional finite geometry and complete case inventory

Without an available bishop source, a discovery bishop's departure and arrival
must be on orthogonal source rays about the enemy king. Bishop movement
therefore joins adjacent axis corners at one fixed radius; opposite corners
would require an orthogonal move. A horse's extra orthogonal step cannot
switch those rays; radius-one horse locations gaze and are disallowed. A
destination evades check only by capturing a nonbishop checker or interposing
its ray. The necessary union is orthogonal alignment, adjacent diagonal
displacement, or the correctly oriented knight displacement. The scripts
deliberately admit extra impossible minor-piece attacks within that union.

Both implementations inspect all king displacements and radii up to eight,
all 9 by 9 translations via bounding-box spans, king separation, forbidden
king-line endpoints, and path obstructions by the fixed own king. The
directed version additionally excludes departing blockers that would already
obstruct the incoming evasion ray, and enumerates directed simple cycles.
Neither excludes shared endpoints merely because their occupancy would need
scheduling. The comparison returns exactly **23 horizontally normalized
pairings at 20 king offsets**, or **46 including horizontal reflections**.

A closed walk contains a simple cycle; this alone would not justify claiming
the whole walk has only two endpoints. Here every surviving pair has two
endpoints per bishop and there is exactly one viable pair for each retained
king-offset/radius pair. Any additional used cycle at the same radii would
also jointly fit the actual board with the opposing bishop's used cycle and
would appear as another tuple. None does. Thus multiple two-cycles cannot
silently combine into an omitted larger walk. Discarded translations or
actual horse/minor restrictions can only shrink the candidate set.

The inventory is:

| King L, with K=(0,0) | S radius, T radius | Exclusion |
| --- | --- | --- |
| `(1,-3)` | `1,1` | Half-plane invariant |
| `(1,-2)` | `1,1` and `2,2` | Half-plane invariant |
| `(2,-1)` | `1,1` | Half-plane invariant |
| `(3,-2)` | `2,2` | Half-plane invariant |
| `(1,-2)` | `1,2` and `2,1` | Mixed-radius lemma below |
| `(1,2)` and `(2,1)` | `1,1` | Three simultaneous rooks |
| `(t,t)` and `(t,-t)`, `2 <= t <= 8` | `t,t` | Locked identical endpoints |

The positive-offset cases are excluded at a single transition, just like the
silver stock argument: at L=(2,1), S's `(1,0)->(0,1)` needs two S rook sources
and a T horizontal rook checker; at L=(1,2), use the reverse move and the
upward T checker, which cannot be a negative-y T lance. No ownership across
different phases is presumed. For the fourteen diagonal-offset pairs both
bishops share the same two endpoints, occupy opposite endpoints and cannot
capture one another. Neither can move; direct checks cannot free an endpoint.

## The five half-plane contradictions

This table supplies the actual endpoint assignments used to check the PDF's
argument. H contains A and B; the other half-plane contains A' and B'.

| L | A | A' | B | B' |
| --- | --- | --- | --- | --- |
| `(2,-1)` | `(1,0)` | `(0,-1)` | `(2,0)` | `(1,-1)` |
| `(3,-2)` | `(2,0)` | `(0,-2)` | `(3,0)` | `(1,-2)` |
| `(1,-2)` | `(0,-1)` | `(1,0)` | `(0,-2)` | `(1,-1)` |
| `(1,-3)` | `(0,-1)` | `(1,0)` | `(0,-3)` | `(1,-2)` |
| `(1,-2)` | `(2,0)` | `(0,-2)` | `(1,0)` | `(-1,-2)` |

In the first four rows A is strictly between B and K, and B' is strictly
between A' and L. When T's bishop arrives at B, S's bishop must be at A':
otherwise A already obstructs the incoming S check that T's arrival purports
to evade. The new T source lies beyond B' from L, outside H. Thus `disc H=1`.
When S's bishop arrives at A', T's bishop must instead be at B, for the same
obstruction reason at B'. The new S source lies beyond A from K, in H, so
`disc H=2`. Both arrivals occur because each persistent bishop moves between
its two endpoints in a closed sequence. Arbitrary direct checks between those
arrivals preserve the invariant and do not remove the contradiction.

In the fifth row B lies strictly between A and K, and A' lies strictly between
B' and L. S's A->A' requires T at B'; afterwards both blockers are outside H
and S's checker is in H, giving `disc H=1`. T's B'->B requires S at A;
afterwards both blockers are in H and T's checker is outside H, giving
`disc H=2`. This covers the fifth old case without assuming adjacent
discovery moves in time or permanently assigned checking sources.

## Mixed-radius exclusion checked independently

Use L=(1,-2), K=(0,0), S endpoints a=(1,0), c=(0,-1), and T endpoints
b=(-1,-2), a. S's radius-one blocker is an unpromoted bishop: a horse there
would gaze at adjacent K. T's closed walk includes a->b. Since b is on no
orthogonal/diagonal line from K, that move must capture the checker at b.
Only an unpromoted S knight checks K from b, fixing S's forward direction to
positive y. If the forward direction were opposite, this move is already
impossible.

That capture adds one knight to T's hand. Restoring the knight hand count
requires a T knight drop at some later or earlier point of the cyclic
interval; additional captures cannot reduce the count. Every drop directly
checks, and T's negative-y checking-drop squares against L are K itself and
n=(2,0). K is occupied, so the drop must be at n. To evade check on fixed K,
this drop interposes an S rook/dragon at (x,0), x>2.

S must capture the dropped knight because a knight check cannot be
interposed. Its persistent bishop cannot move from either endpoint to n.
A direct checker at n must be an S rook/dragon. Its possible origins are:
horizontal rook origins; vertical rook origins; and, for a dragon, adjacent
diagonals `(1,±1)` and `(3,±1)`. Negative-x horizontal origins are blocked by
K; `(1,0)` and `(1,±1)` already check K and the drop at n cannot evade them;
vertical origins and `(3,±1)` do not gaze at K, so cannot be moving direct
pieces by the proved classification. Thus the capturing rook/dragon comes
from the outer ray R={(x,0):x>2}, decreasing its S rook-base count.

No S evasion can end on R. From any q=(x,0), x>2, the displacement to L is
neither orthogonal nor an eligible knight or adjacent-step attack. Only x=3
is diagonal, and that would need a checking bishop/horse; both bishops are
the persistent nongazing blockers. Interposition at q likewise requires a
long diagonal source only at x=3, and no such source exists. This excludes
moves, captures and drops onto R. Opposing moves cannot increase the number
of S-owned rook base pieces there; promotion on R does not change that count.
It never increases and the forced knight episode decreases it, contradicting
the closed board. This explicitly accommodates source capture/replacement,
hands and promotions without tracking individual rooks.

Horizontal reflection preserves forward directions. Exchanging sides and
applying `(x,y)->(1-x,-2-y)` produces S radius-two endpoints `(2,0),(0,-2)`
and T radius-one endpoints `(0,-2),(1,-1)`, while preserving the required
opposite forward directions. This is precisely the other mixed pairing.

## Boundary audit, executed checks and conclusion

At the inspected preserved revision, `Position::validate` requires one king
each, valid promotion and combined standard stock bounds; it permits reduced
material. `GameRoot::import` checks the nonmoving king is unattacked, permits
the moving king to be checked, records full/reduced imported categories and
explicit counter provenance, and does not assert reachability. `GameRecord`
rebuilds such roots through this admission function and replays commands
through authoritative game operations. Local legality uses geometric king
safety (including pinned attackers), consistent with this argument. This was
a source-boundary audit, not fresh implementation testing or a replacement
for final code review. Structural diagrams with an attacked nonmoving king
remain available for analysis and are not required to obey this theorem as
live histories.

Executed in the macOS worktree at the revision above:

- `shasum -a 256 /tmp/mshogi-mutual-check-proof.pdf`: matched the exact PDF
  identity recorded above.
- `shasum -a 256 -c SHA256SUMS` in the supplement directory: all seven entries
  passed.
- `python3 docs/m4/mutual-check-supplement/verify_geometry.py`: passed with
  agreement, 23 normalized pairings, 20 offsets, 46 reflected pairings,
  two-endpoint cycles and verified mixed-case side exchange.
- Rendered PDF pages 2-11 and both enumerator sources: inspected manually;
  all five old contradictions and both new mixed cases reconstructed above.

No unresolved failed mathematical implication or counterexample was found.
The result excludes the mutual-check history pattern for every admitted live
root, including roots with unknown prehistory. Fourfold repetition would
contain the prohibited closed all-checking interval, so the HIST-INTERVAL
impossibility obligation is discharged by this reviewed chain. The finite
calculation covers conditional geometry, **not all legal shogi states**.
This is an independently performed AI mathematical review, not formal proof
assistant certification or human expert review. No additional game outcome,
tournament penalty, root reachability requirement or rules-profile change is
introduced. Implementation invariant diagnostics and their focused tests
still require the separate implementation review and integration evidence.
