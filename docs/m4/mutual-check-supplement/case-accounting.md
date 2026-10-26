# Revised necessary-geometry case accounting

## Scope of the finite calculation

The calculation begins after the preceding reductions have established fixed
kings and one persistent, uncapturable, nongazing discovery bishop or horse per
side. These hypotheses must remain explicit in the integrated proof.

Because both bishops are already allocated to those roles, an additional
checking source behind a discovery blocker can only supply an orthogonal ray.
Each moving bishop therefore travels between adjacent axis corners at a fixed
radius around the opposing king. A horse's extra step cannot switch between
such rays without entering an already-gazing adjacent position.

Every destination must also capture a checking piece or interpose a check on
the moving side's fixed king. Without another checking bishop, the necessary
geometries are an orthogonal ray, an adjacent diagonal attack, or a correctly
oriented knight displacement. The scripts deliberately admit the union of
these geometries, even where the actual piece inventory or occupancy would
make the candidate impossible.

The scripts cover king displacements and radii permitted by a 9 by 9 board,
exclude coincident/adjacent kings and forbidden king-line/path cases, and admit
all translations that fit the board. They retain overlapping bishop endpoints.
The directed implementation also checks cyclic walks rather than presuming
that every eligible edge can be reversed. Its surviving cycles all have two
endpoints, agreeing with the independently authored edge calculation.

## Complete retained inventory

Set K=(0,0), put S's king at L, and take S's forward direction as positive y.
Horizontal reflection permits L.x >= 0. A radius is measured from the opposing
king, not from a bishop's own king.

| L | S radius | T radius | Disposition |
| --- | --- | --- | --- |
| (1,-3) | 1 | 1 | Existing half-plane contradiction |
| (1,-2) | 1 | 1 | Existing half-plane contradiction |
| (1,-2) | 1 | 2 | New mixed-radius lemma |
| (1,-2) | 2 | 1 | Side-exchanged 180-degree image of the new lemma |
| (1,-2) | 2 | 2 | Existing half-plane contradiction |
| (2,-1) | 1 | 1 | Existing half-plane contradiction |
| (3,-2) | 2 | 2 | Existing half-plane contradiction |
| (1,2) | 1 | 1 | Three simultaneous rooks required |
| (2,1) | 1 | 1 | Three simultaneous rooks required |
| (t,t), 2 <= t <= 8 | t | t | Both bishops share the same two endpoints and cannot move |
| (t,-t), 2 <= t <= 8 | t | t | Both bishops share the same two endpoints and cannot move |

This is 23 normalized pairings over 20 king offsets. The last two rows each
represent seven pairings. Horizontal reflection gives 46 signed pairings in
the full directed enumeration. No surviving pairing has L.x=0.

## Positive-offset exclusions count sources at one transition

At L=(2,1), S's bishop moves between (1,0) and (0,1). Consider its move from
(1,0) to (0,1). There must be an S rook/dragon supplying the newly uncovered
horizontal check and a distinct S rook/dragon behind the destination's vertical
ray. A lance cannot supply that vertical source because S advances in positive
y while the source must attack downward toward K. The destination must also
evade a horizontal check on L, requiring a T rook/dragon. This needs three
rook base pieces at that specific transition.

At L=(1,2), consider the reverse bishop move from (0,1) to (1,0). Again, S
needs two distinct rook sources. The incoming vertical check on L comes from
below it; T advances in negative y, so a T lance cannot supply that upward
check. A third rook is required.

These contradictions use the standard two-rook stock bound at a specified
simultaneous position. They do not assume that source ownership stays fixed
through arbitrary intervening captures and drops.

## Identical endpoint sets

For L=(t,t) or L=(t,-t), both discovery bishops have the same two possible
endpoints. They occupy different endpoints, and neither can be captured.
Neither can move because its only other endpoint is occupied by the other
persistent bishop. Intervening direct checks cannot free either endpoint.

## Integration checks for the preceding argument

The new lemma and enumeration do not replace the earlier reduction. A fresh
review should explicitly connect them and retain the following proof hygiene.

1. With fixed kings, a legal continuation cannot face an unevadable double
   check. A nonking move cannot capture or interpose two independent checks
   at once. The half-plane argument's single active checker must be justified.
2. Drops are direct checks. They can interpose against the prior checker, but
   the old checker then ceases checking. Counting currently checking pieces
   does not acquire an unexplained extra unit from the drop.
3. In the angle argument, a direct checking drop has zero discovery-angle
   term and a strictly negative interposition term. An on-board origin is
   unnecessary for that term; a virtual origin equal to the destination is
   an equivalent explicit convention.
4. A repeated semantic position need not restore labels attached to identical
   physical pieces. Use the monotone count of nongazing pieces for the role
   classification. For the angle telescoping step, sum over the complete
   nongazing-piece angle multiset, which a repeated board restores.
5. Nongazing discovery pieces cannot be captured or replenished by drops.
   Their promotion state cannot change during a closed sequence without
   an unavailable demotion. Source/inventory arguments must use simultaneous
   presence or a proved monotone quantity, never a permanent ownership guess.
6. The admitted live-root domain includes reduced material and unknown earlier
   history. None of the new arguments assumes reachability from the standard
   initial placement. Structural analysis diagrams with an already attacked
   nonmoving king remain outside the live-root domain.

These are review obligations for the whole proof chain, not new game rules.

## Executed checks and limitations

The two geometry scripts were written independently by the root review and a
separate mathematics reviewer. A third mathematics reviewer derived the new
mixed-radius lemma, and the geometry reviewer checked its written argument.
All reviewers were AI agents. No human expert review is claimed.

`verify_geometry.py` executes both enumerators, compares exact candidate tuples,
checks the inventory above and the mixed-case symmetry, and prints the result.
The retained JSON records the initial successful cross-check. The scripts use
neither mShogi's move generator nor cshogi, so their agreement does not reuse
either product's local-legality assumptions.

The calculation exhausts a small conditional geometric domain. It is not a
search of all legal shogi positions, a formal proof certificate, or evidence
that a mutual-check cycle was found. Human-readable arguments provide the
exclusions after the candidate calculation.

The appropriate acceptance step is a fresh review of the integrated chain,
followed by the existing MSH-67 implementation verification and integration.
