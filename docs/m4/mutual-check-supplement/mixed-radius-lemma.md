# Conditional exclusion of the mixed-radius shared-endpoint case

This is an independently derived repair for the specific missing configuration
in the MSH-67 research review. It is not, by itself, a proof of the entire
mutual-check theorem. The earlier reduction and completeness of the resulting
configuration list remain separate obligations.

## Assumptions supplied by the earlier reduction

Assume a nonempty closed sequence of ordinary legal shogi board moves in which
every move gives check. Board ownership, promotion, hands and turn return to
their initial values. The standard piece-stock bounds apply.

The preceding lemmas establish the following facts for such a sequence.

1. Both kings are fixed.
2. Every moving piece either remains on board, never gazes at the opposing
   king, and makes discovered checks, or gazes at the opposing king whenever
   it is on board and makes direct checks. Here gaze is Nanka's definition,
   retaining the immobile pieces while removing the other movable pieces.
3. Exactly the two bishops or horses provide the two sides' persistent
   discovery blockers. Neither is captured or supplies a direct check.
   There is consequently no additional bishop or horse available as a
   checking piece along a long diagonal.
4. For the case considered here, those blockers have only the two indicated
   possible squares each.

## Coordinates and orientation

Let S's king be L = (1,-2), and T's king be K = (0,0).

S's discovery bishop moves between a = (1,0) and c = (0,-1).
T's discovery bishop or horse moves between b = (-1,-2) and a = (1,0).

The bishops cannot occupy a simultaneously. S's bishop is unpromoted: a horse
on either a or c would directly attack the adjacent K and would not be a
persistent nongazing discovery blocker.

S's forward direction must be positive y in any cycle that contains T's
return a -> b. If it were negative y, the knight-check requirement in the
next paragraph would already make that return impossible. Thus T's forward
direction is negative y.

## Lemma

There is no closed all-checking sequence satisfying these assumptions.

### Step 1. T must capture an S knight

T's bishop is a moving persistent blocker with just the squares a and b.
A closed sequence in which it moves must therefore contain a -> b.

T's king K does not move. The destination b = (-1,-2) lies on none of the
orthogonal or diagonal lines through K. Moving to b cannot interpose against
a sliding check. To evade check, the bishop must capture the checker at b.

The only ordinary shogi piece that can check K from b is an unpromoted
S knight advancing in positive y. Neither a promoted knight nor a bishop,
horse, rook, dragon, lance, pawn, gold or silver checks over that displacement.

Thus a -> b captures an unpromoted S knight and increases T's knight hand
count by one.

### Step 2. A T knight drop is forced by hand balance

The closed sequence restores both hands. Capturing a knight, promoted or
unpromoted, increases the capturer's knight hand count. The only operation
that decreases T's knight hand count is T dropping an unpromoted knight.

Consequently, Step 1 forces at least one T knight drop somewhere in the
closed sequence. Additional captures or promotions cannot avoid this
hand-balance conclusion.

Every drop in an all-checking sequence must directly check the opposing
king, since placing a piece cannot uncover a friendly attack.
With T advancing in negative y and S's king at L = (1,-2), a T knight can
give check only from (0,0) or n = (2,0). The former is occupied by T's own
fixed king K. The required drop is therefore at n = (2,0).

### Step 3. That drop forces an S rook or dragon to leave the outer ray

T is itself evading check when it drops at n. A drop neither captures a
checker nor moves the king, so it must interpose. The incoming check on K
must therefore come from an S rook or dragon at (x,0) with x > 2.

After the drop, the knight checks L. The fixed king cannot move, and a
knight check cannot be interposed. S must capture the knight at n.

The persistent S bishop cannot make this capture: neither a -> n nor
c -> n is a bishop move. All other moving S pieces make direct checks.
The only such piece that can directly check K from n is a rook or dragon.

The capturing rook or dragon must originate on the outer ray
R = {(x,0) : x > 2}:

- A horizontal origin with x < 0 cannot reach n through the occupied K.
- Origin (1,0) would already check the nonmoving K adjacently, and the
  knight drop at (2,0) could not remove that check.
- A vertical rook origin (2,y), y != 0, does not gaze at K, so it cannot
  supply this direct checking move in the closed sequence.
- A dragon's extra diagonal-step origins (1,1) and (1,-1) already check
  K adjacently; a drop at n cannot evade those checks.
- Its other extra origins (3,1) and (3,-1) do not gaze at K and are
  likewise unavailable to a moving direct-check piece.

Hence this response removes an S-owned rook or dragon from R.

### Step 4. No S move or drop can replenish that ray

Consider any S move or drop ending at q = (x,0), x > 2. S must evade a
check on its fixed king L = (1,-2).

If q is the checker square, no non-bishop T piece at q attacks L:
the displacement is too long for a step piece or dragon's extra step,
is neither orthogonal nor a knight displacement, and is diagonal only
when x = 3. A bishop or horse at q cannot be the checker either, because
both bishops are persistent nongazing blockers at their prescribed squares.

If q interposes, it must lie strictly between L and a sliding checker.
The line Lq is neither orthogonal nor diagonal except when x = 3.
For x = 3, a checker beyond q would have to be a bishop or horse giving
a long-diagonal check. Both available bishops are already the persistent
nongazing blockers, so that checker does not exist.

Thus no checking evasion by S can end on R, whether by move, capture or
drop. In particular, the number of S-owned rooks and dragons on R can
never increase during the sequence.

### Step 5. Contradiction

Step 3 strictly decreases that number. Step 4 forbids any increase.
A closed board position must restore it. This is a contradiction.

The argument allows arbitrary intervening direct checks, captures, drops
and promotions. It counts the knight hand and rook base kinds rather than
tracking interchangeable physical piece identities. Finite board edges can
only remove candidate squares and do not create an exception.

## Symmetry and limits

Translations and horizontal reflection preserve the argument. Exchanging
S and T together with a 180-degree board rotation gives the corresponding
opposite-side case. A transformation that changes the forward direction
must be checked explicitly rather than assuming full rotational symmetry
for shogi pieces.

Concretely, exchange the sides and apply (x,y) -> (1-x,-2-y). The new enemy
king is again (0,0), the new own king is again (1,-2), the new own bishop
has radius-two endpoints (2,0) and (0,-2), and the new enemy bishop has
radius-one endpoints (0,-2) and (1,-1). Thus the missing radius-(2,1)
configuration is the same case as radius-(1,2), with the permitted shogi
side/orientation transformation made explicit.

This excludes the stated mixed-radius shared-endpoint configuration under
the earlier reduction. It does not certify that the published table plus
this case exhausts every possible king/blocker configuration, or that all
earlier lemmas have been independently reviewed.

## Earliest-fourfold semantics

Temporal ordering alone does not discharge the mutual-check question.
An imported live root may already have its moving king in check. If a
closed all-checking sequence existed, repeating it would give an interval
whose moves all check for both sides. Choosing the earliest fourth
occurrence does not independently explain which side should lose.
The geometrical impossibility argument, or an authoritative rule for an
actual counterexample, remains necessary for that hypothetical case.
