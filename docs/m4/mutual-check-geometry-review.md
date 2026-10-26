# Independent review of the mutual-check geometry supplement

Reviewed 2026-10-26 by the independent AI `geometry_supplement_review`
session for MSH-67, against source revision
`0d1631db8fe4adac3ac0a9e5aa6956a863a3ba1c`. The reviewer did not author the
supplied supplement and did not accept its prior AI reviews as authority.

**Verdict:** the conditional necessary-geometry calculation, revised case
accounting and mixed-radius exclusion are accepted. No mathematical correction
to the supplied mixed-radius lemma is required. The positive-offset inventory
argument needs the explicit source-persistence justification below; merely
assuming that a future discovery source is present at an earlier transition
would be insufficient. This document supplies that justification. Acceptance
here remains conditional on the earlier fixed-king, gaze, discovery-necessity
and piece-stock reductions, which are the subject of the separate full-chain
review. This is neither an exhaustive search of legal shogi states nor a
machine-checked theorem or human expert review.

## Method and reviewed boundary

I read the current user continuation instructions, AGENTS.md, the design and
MSH-67 record, the historical incomplete proof review, both geometry programs,
their cross-check and all supplement prose. I reconstructed the reasons for
each filter, checked the five steps of the new lemma directly from movement,
ownership and hand operations, and checked the original table and its
half-plane witnesses against the rendered Nanka v3 PDF pages 10–11. I also
checked an extra graph obligation: two-vertex simple cycles alone would not
exclude a longer walk alternating between several overlapping two-vertex
cycles. The actual candidate graphs have at most one simple cycle per fixed
king pair, side and radius, so that issue does not occur.

The supplied enumerators are two separately authored programs, not two
independent proofs of their shared premises. My review separately establishes
that their filters are necessary under the explicitly preceding reductions.
No mShogi or cshogi move-generator output establishes these expectations.

## Why the conditional geometry filters are necessary

Assume the preceding reductions: kings are fixed; each side has a moving,
uncapturable, always nongazing discovery bishop or horse; these use the whole
two-bishop stock; all other moving pieces make direct checks. Repetition is of
semantic board ownership, promotion, hands and turn, without physical labels.

* A discovery blocker must leave a ray from its opposing king. Only an
  orthogonal source remains available, since both bishop base pieces already
  have the persistent blocker roles. A blocker therefore occupies an axis
  corner relative to the opposing king whenever it moves. A bishop move
  between different axes has equal absolute coordinate displacements and
  hence preserves radius. Opposite-axis corners cannot be joined by a bishop
  move. A horse's orthogonal step cannot switch axes without landing on the
  king, and a step along the same ray does not uncover it. Adjacent horse
  squares that already attack the king are incompatible with nongazing.
* A bishop arrival must capture the checker or interpose its check on the
  fixed own king. Another persistent bishop cannot be that checker. Without
  an additional bishop, all possible checker/interposition geometries lie in
  the scripts' union: orthogonal line, adjacent diagonal, or the correctly
  oriented unpromoted-knight displacement. Orthogonal attacks are deliberately
  overapproximated without minor-piece direction restrictions. This can
  retain impossible cases; it removes no required geometry.
* A bishop cannot cross its own king. The directed program additionally
  excludes an origin strictly between the own king and destination: that
  origin would already have blocked any ray check that arrival could evade.
  The second program omits this extra directional pruning, yet its paired
  results agree exactly. Neither program presumes reversibility of an actual
  legal move; the directed program explicitly finds cycles.
* A bishop endpoint cannot lie on the line of the kings. On an orthogonal
  king line, the source behind that endpoint and the checker whose check its
  arrival evades cannot coexist with clear required paths: either the own
  king blocks the discovery ray, the opposing king blocks the incoming ray,
  or the future friendly discovery source blocks the incoming checker.
  The source cannot first be supplied later while its own bishop blocks the
  ray, for the source-persistence reason in the next section. This is a
  necessary exclusion, rather than an arbitrary geometric simplification.
* King differences and bishop radii range up to eight because the board is
  9 by 9. A bounding box of span at most eight on each axis is equivalent to
  the existence of a translation fitting every relevant square. Requiring
  all endpoints to fit the same board is necessary even if they are visited
  at different times. Coincident or adjacent fixed kings cannot occur in the
  live domain. Horizontal reflection preserves forward direction. No
  unchecked vertical-reflection symmetry is used.

The directed program's simple-cycle calculation is sufficient for arbitrary
closed bishop walks here. A closed walk uses only edges lying on directed
cycles. The additional check reproduced below finds 84 cyclic fixed-king,
side/radius graphs before pairing, each with exactly one distinct simple
cycle. Those cycles have two vertices. A longer closed walk therefore cannot
combine different edges to escape the two indicated endpoints. This reasoning
uses no assumption about individually labeled identical pieces returning.

## Source persistence and the two positive-offset cases

Here is the temporal justification needed for simultaneous source counting.
After a persistent S bishop arrives at a square p, consider the interval until
its next departure. A discovery source must exist strictly beyond p on the
ray from the opposing king K immediately before that departure. While the
bishop stays at p, no other S move, capture or drop can newly put an S source
on that outer ray: the bishop would block the resulting piece's direct check
on K, and all those other moving S pieces must make direct checks. Opponent
moves cannot increase the number of S-owned rook base pieces there. Capturing
an S piece can only decrease that count; moving, dropping or promoting a
T-owned piece does not create an S-owned one. Unmasking an already present S
rook merely selects a source already counted. Thus at least one source needed
at the next departure was already present when the bishop arrived. No fixed
ownership of particular checking sources over the entire cycle is assumed.

At L=(2,1), S's move (1,0) -> (0,1) requires an S rook/dragon behind the
vacated horizontal endpoint. Its eventual return requires a source above
(0,1), already present at arrival by the preceding argument. An S lance
cannot check downward from that ray because S advances in positive y, so this
is a second S rook/dragon. Arrival at (0,1) must also evade a horizontal check
on L, supplied by a T rook/dragon. All three base rooks are present just before
this move, including if the move captures the incoming T checker. The stock
bound is two.

At L=(1,2), use S's move (0,1) -> (1,0). The vacated vertical source attacks
downward and cannot be an S lance; the destination needs a distinct horizontal
source. The incoming vertical check on L comes from below it and attacks
upward, so it cannot be a T lance, since T advances in negative y. Again the
two S sources and the T incoming source are simultaneous. These arguments
remain valid with arbitrary earlier/later source captures, drops and ownership
changes.

## Inventory and five existing half-plane contradictions

Both programs and the cross-check produce 23 horizontally normalized pairings
over 20 king offsets, or 46 including reflections. These are conditional
candidate geometries, not 46 legal positions or histories. The disposition is
five existing half-plane cases, two mixed-radius cases, two positive-offset
stock contradictions, and fourteen identical-endpoint locks.

For clarity the half-plane witnesses can be stated without relying on Table 1
being complete. Let H be the open half-plane of the king line containing A and
B. In the first four rows below, A lies strictly between K and B, and B'
lies strictly between L and A'. A T arrival at B consequently requires S at
A'; an S arrival at A' requires T at B. The newly discovered checker lies on
the ray of the vacated endpoint. The first transition therefore gives
`disc H = 1`, and the second gives `disc H = 2`, contrary to the established
invariance. The source of either discovery may change between transitions;
its half-plane is still fixed by the vacated endpoint.

| L | A | A' | B | B' |
| --- | --- | --- | --- | --- |
| (2,-1) | (1,0) | (0,-1) | (2,0) | (1,-1) |
| (3,-2) | (2,0) | (0,-2) | (3,0) | (1,-2) |
| (1,-2), radii (1,1) | (0,-1) | (1,0) | (0,-2) | (1,-1) |
| (1,-3) | (0,-1) | (1,0) | (0,-3) | (1,-2) |
| (1,-2), radii (2,2) | (2,0) | (0,-2) | (1,0) | (-1,-2) |

In the last row B is strictly between K and A, and A' is strictly between L
and B'. S's move A -> A' requires T at B', giving `disc H = 1`; T's move
B' -> B requires S at A, giving `disc H = 2`. The active checker lies in H
after the former move and outside H after the latter. These give the fifth
existing contradiction. This step explicitly depends on the full-chain
review's justification of the single active checker and the `disc H`
invariant, including interposition drops.

For L=(t,t) and (t,-t), 2 <= t <= 8, the bishops have identical two-endpoint
sets. Both are persistent and uncapturable. Each occupies the other's only
possible destination, so neither can make its required discovery move.
Intervening direct checks cannot free an endpoint. This accounts for all
fourteen locks, including the positive diagonal family omitted by the earlier
published table. The remaining two pairings are the mixed radii (1,2) and
(2,1) at L=(1,-2).

## Independent check of the new mixed-radius lemma

Use the supplement's K=(0,0), L=(1,-2), a=(1,0), c=(0,-1), b=(-1,-2).

1. T's moving blocker has only a,b, so a closed walk containing its moves
   includes a -> b. The displacement b to K is (1,2), neither a sliding line
   nor a step attack. Arrival cannot interpose; it must capture the active
   checker. Only an unpromoted S knight with forward direction +y checks K
   from b. Its capture increases T's knight hand count.
2. Restoring T's knight hand requires a T knight drop. Capturing promoted
   knights also increases this same hand count; promotion or ownership change
   cannot consume a hand knight. Drops cannot discover checks. With T forward
   -y, its only checking-drop squares against L are K and n=(2,0). K is
   occupied, so n is forced.
3. The drop must evade the incoming check on K by interposition. Only an S
   rook/dragon beyond n on y=0 supplies that ray. S must then capture the
   checking knight, since fixed kings cannot move and knight checks cannot be
   interposed. S's persistent bishop is unpromoted (a horse at a or c would
   attack adjacent K) and cannot reach n from either endpoint. Every other
   moving S piece is direct-checking, so the capturer at n must be a
   rook/dragon.
4. I checked every rook/dragon origin for that capture. Negative horizontal
   origins cannot cross K. Origin (1,0) already checks K and is not neutralized
   by a drop at n. Vertical rook origins do not gaze at K. The extra dragon
   origins (1,+/-1) already check K adjacently, while (3,+/-1) do not gaze at
   K. Thus the capturing rook/dragon necessarily leaves the outer ray
   R={(x,0):x>2}.
5. No S checking evasion can end on R. A non-bishop checker at q in R cannot
   attack L: the displacement is neither orthogonal, adjacent nor a correctly
   oriented knight displacement. Its only possible diagonal is x=3. An
   interposition likewise needs an orthogonal or diagonal line, and only x=3
   is diagonal. Both diagonal alternatives need an additional active bishop
   or horse, which the two persistent blockers exclude. Therefore S cannot
   move, capture or drop onto R.

Consequently the number of **S-owned rook base pieces** on R strictly decreases
at the forced response and never increases. This count includes both rooks
and dragons. Promotion preserves the base kind; T captures can only decrease
the count; T movements, drops or promotions do not add S-owned pieces; and
S's only means of newly acquiring an on-board piece at q would itself end
at q and is excluded. A rook in hand is not counted. This covers opponent
actions and ownership changes as well as S moves. The semantic repeated board
must restore the count, giving the contradiction without tracking physical
piece identities.

Exchanging sides and applying (x,y) -> (1-x,-2-y) maps the radius-(1,2)
endpoints to radius-(2,1) endpoints and rotates both forward directions with
their exchanged sides. It is a valid shogi symmetry; a bare rotation without
side exchange would not be. Horizontal reflection and translations also
preserve the argument. Board edges can disallow required squares and thus
remove candidates, but cannot create a missing replenishment move.

## Execution evidence and fingerprints

Environment: macOS 27.0.1 (26A434), arm64; Python 3.14.7. Commands were run in
the restored `codex/shogi-history` worktree at the revision above.

* `python3 docs/m4/mutual-check-supplement/verify_geometry.py`: PASS; agreement,
  20 offsets, 23 normalized pairings, 46 reflected pairings, cycle sizes [2,2],
  and side-exchange symmetry.
* `python3 docs/m4/mutual-check-supplement/directed_geometry.py`: PASS; inspected
  the complete displayed normalized inventory against the case dispositions.
* The following additional Python audit: PASS, output
  `Cyclic fixed king/radius graphs by number of distinct simple cycles: {1: 84}`.

```python
import sys, itertools, collections
sys.path.insert(0, 'docs/m4/mutual-check-supplement')
import directed_geometry as d
hist = collections.Counter()
for x, y in itertools.product(range(-8, 9), repeat=2):
    if max(abs(x), abs(y)) <= 1:
        continue
    for own, enemy, forward in (((x,y),(0,0),-1), ((0,0),(x,y),1)):
        counts = collections.Counter(
            r for r, cycle in d.candidate_components(own, enemy, forward))
        hist.update(counts.values())
        assert all(n == 1 for n in counts.values())
print('Cyclic fixed king/radius graphs by number of distinct simple cycles:',
      dict(hist))
```

Reviewed SHA-256 fingerprints (the package's `SHA256SUMS` records the remaining
supplement files):

| Artifact | SHA-256 |
| --- | --- |
| Nanka v3 PDF | `cd5f987133d89a32bd6de8391015354b16d066a97cea7f719b67a5089275e065` |
| Historical incomplete proof review | `5fae20f53a07cdb2a47501d6495fe7362dda7251148efeccb0c1cd38c5546039` |
| directed_geometry.py | `d9cb38bb360ede68a8424e1c8036180f1b0ece79ce25712d60bf4bd7560c69cc` |
| independent_geometry.py | `218e13fcd6578725a65a3f8edfbe98809d2044bd10f5bd8f6d55cb7f91eba4d2` |
| verify_geometry.py | `599dc02322a15ee45769695430628939395a4acfdbee3ef00c488b09c838221b` |
| case-accounting.md | `4f62fff2e7c73986912233d4754c8bbf2a4172b2d36d6343be57e74f5554fc88` |
| mixed-radius-lemma.md | `6654d34fb114a97b625c47fb59ac51d9abd023c0803c3e30bccdcfabb069e7cf` |

No product code, fixtures, Backlog records or supplied supplement files were
changed by this review. No complete-chain acceptance, new rules outcome,
implementation verification or MSH-67 completion is inferred from this bounded
verdict.
