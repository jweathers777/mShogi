# Shogi mutual-check geometry

The [full derivation](../mutual-check-derivation.md) reduces mutual check
to a finite set of bishop configurations. This supplement describes the
case accounting and mixed-radius exclusion.

- [Case accounting](case-accounting.md) gives the candidate inventory and exclusions.
- [Mixed-radius lemma](mixed-radius-lemma.md) proves the exclusion using
  knight hand balance and a decreasing count of owned rooks on one ray.
- `directed_geometry.py` enumerates necessary candidate walks and cycles.
- `independent_geometry.py` independently enumerates necessary edges.
- `verify_geometry.py` compares the two enumerations and checks the case count.

From this directory, run `python3 verify_geometry.py`. The conditional
geometry gives 23 horizontally normalized pairings at 20 king offsets,
or 46 pairings with reflections. Each retained cycle has two endpoints for
each bishop.

## How the missing case is excluded

Use the review's coordinates. Side T's king is K=(0,0), side S's king is
L=(1,-2), S's discovery bishop moves between a=(1,0) and c=(0,-1), and T's
discovery bishop moves between b=(-1,-2) and a. S moves forward in positive y.

1. T's return from a to b must capture a checking S knight. The square b lies on
   no sliding line through K, and only an unpromoted knight checks K from b.
2. This capture increases T's knight hand count. A closed cycle restores both
   hands, so T must eventually drop a knight.
3. Every move checks. T's checking knight drop can only go to n=(2,0); its other
   possible checking square is occupied by T's own king.
4. That drop interposes an S rook or dragon's check. S must capture the knight,
   taking a rook or dragon off the ray R={(x,0) where x>2}.
5. No S checking evasion can ever arrive on R. Its squares cannot capture or
   interpose a checking non-bishop piece relative to L. The only possible long
   diagonal would require an active bishop or horse, while both bishops are
   already the persistent discovery blockers.

Thus the number of S rooks or dragons on R never increases and must decrease.
It cannot return to its original value. The proposed closed cycle is impossible.
The full lemma treats all alternative dragon origins and the required earlier
gaze-classification assumptions explicitly.
