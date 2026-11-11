# Batch01 games05–09: independent gameplay review

This review covers the second five opening pairs: `main-01-05` through
`main-01-08` and `main-01-unaided`, ten games. mShogi scored **6 wins,1 draw,3
losses**. This is a batch description, not a rating or final weakness ranking.
No engine was run and no engineering report was read for this review.

All1,112 played decisions receive an explicit disposition in
`adjudication-05-09.json`. The861 referenced5M query receipts match their hashes,
complete successfully without retained-output truncation, and have independently
legal full selected PVs. The standard stale-final-report flag alone does not
exclude finite support. Other instability/conflict remains unresolved. Values
below are Stockfish's mover-perspective expected-score proxy; a range-stable
comparison is finite support, not a probability of winning against this opponent
or proof of a game-theoretic outcome. Numbers such as p32 are zero-based plies
from the initial position, before the named move.

The reviewed method requires20M scrutiny of first/largest or result-changing
claims. Accordingly **all such labels below are provisional**: stable5M support
is distinguished from unresolved5M comparisons, and none becomes a final ranked
weakness here. `plan20m-05-09.json` contains43 decisions,78 singleton queries plus
two discovery cases; discovery may add its newly found alternative mechanically.
It prioritizes about three candidate decisions per game plus the opponent moves
needed to attribute opportunities/recovery. Unqueued significant comparisons
remain5M provisional; unqueued uncertain rows remain unresolved. No omission
means harmlessness or resolution.

## Pair05: opposite-castled Sicilian

**Game0, mShogi White, win.** The earliest and largest stable5M candidate loss is
9.Bxf6 (p16), E.5715 with f4 versus.4895 after the exchange, margin.082. The
alternative retains central pawn tension; the played exchange allows ...Bxf6
followed by ordinary liquidation. This is a contextual exchange/central-play
judgment, not a rule against exchanging that bishop. Black instead played
9...gxf6 (p17), offering a possible attack; its comparison and White's following
10.Ndb5 versus f4 are unstable, so that missed opportunity is unresolved.
The clear subsequent opportunity is14...Nb4 (p27), E.496 with ...Rd8 versus0
after the knight move. The legal continuation Qg3+ exploits the open king;
White played it and eventually mated. White's16.f5 (p30) reduces1.0 to.941 versus
e5 but preserves a very large advantage. It is a modest failure to maximize the
attack, not a thrown win. Twenty-million probes target p16/18/30 and Black17/27.

**Game1, mShogi Black, win.** Black's forcing knight exchanges culminate in
...Nxe2+ after White's12.Qxd6; the latter error's size is unstable. The largest
apparent candidate loss is14...b6 (p27), E.9535 with ...e5 versus.532 after b6,
but this too is unresolved5M. No stable significant candidate comparison
establishes a first error in this game. White then plays16.Rd8+ (p30), a stable
.2865 loss relative to g3, leaving E.007 for White. Black meets the check with
...Ke7, advances ...e5 and converts to mate. This is evidence of practical
conversion after another opponent error, not proof that b6 preserved the earlier
win. Probe p27 and the opponent p22/p30 sequence.

## Pair06: central pawn tension, later repeated reversals

**Game0, mShogi White, loss.** The first stable significant loss is11.Bd2 (p20),
E.5185 with e4 versus.4065. After Black's unstable ...Qe7 error,13.Ng5 misses a
larger e4 opportunity (.789 versus.4905). These independently legal lines show a
recurring choice between central expansion and a less useful piece maneuver.
Later Black supplies several further chances. White's32.h4 (p62),36.hxg5 (p70)
and38.Qc3 (p74) each surrender a supported large advantage. At p70 the central
break d5 retains E1.0 while hxg5 gives.506, the largest stable candidate loss.
Black's39...Bh2 (p77) then restores a winning chance; White finds40.Ke2, but
41.Bc5 is unstable and42.Ke1 (p82) is the largest *apparent* final reversal,
.728 with Be6 versus0. Its legal ...Bb8 continuation threatens the seventh-rank
rook through the bishop diagonal; the alternative Be6 retains active resistance.
That comparison must be deepened. The very long later defense is not counted as
many independent causes of the original defeat. Probe p20/70/82 and Black77;
p24/62/74 and their other granting moves remain provisional rather than silently
resolved.

**Game1, mShogi Black, draw.** The first stable major loss is14...Ne5 (p27),
.4705 with ...Ng4 versus.002. The exchange sequence permits f4/e5 with strong
pressure against the rook and king. White's22.Ne4 (p42) later returns the game to
a drawing range; Black finds ...Bd5. White's40.Kh2 (p78) then grants a winning
chance, and ...Bd5 keeps E1.0. But41...c4 (p81) chooses the wrong pawn break:
...b4 retains1.0 whereas c4 gives.5. This is the largest stable conversion loss.
A further49...Ba8 (p97) changes.4985 to0 versus ...b4, showing another passive
choice where an active pawn break matters. The actual game still draws by a
native threefold claim, so opponent recovery must be tracked too.

There is an important unresolved middle sequence:45...Kxh6 (p89) has a singleton
5M estimate0 although the preceding positions are near equality. Its comparison
has no distinct alternative, so it **cannot establish a move error**. White's
46.Kf4 (p90) then misses the supported Bf3 win (1.0 versus.504). After p97,
White instead plays50.Bf3; the recorded alternative at the preceding position
had been Bf5, requiring direct discovery at the actual p98 root. The20M plan
includes p27/81/97, discovery at89, and opponent42/78/90 plus discovery98. This
exception to three candidate probes is needed to separate real recovery from
inconsistent adjacent estimates.

## Pair07: queen-side invasion and quiet defense

**Game0, mShogi White, loss.** The first/largest stable loss is17.b3 (p32),
.488 with Bf4 versus0. The bishop on e5 is already attacked by ...f6; moving the
b-pawn removes control of a3 and allows ...Na3 with tempo on Qc2 before ...fxe5.
The legal Bf4 alternative retreats the threatened bishop and retains resistance.
This is a concrete tactical/defensive failure, not a generic pawn-structure
label. White remains in trouble for much of the game, so later mistakes are
secondary. However, late opponent mistakes create distinct new chances:
55.Ra7 (p108) misses the active g4 idea (.374 versus0), and63...Kg8 (p125)
returns the position to approximately a draw. White immediately plays64.Kf5
(p126), .4745 with Ra8+ versus0; ...Rf6+ demonstrates the lost defensive
coordination. Probe p32/108/126 and Black125. Earlier uncertain p104 remains
unresolved and is not called the original losing move.

**Game1, mShogi Black, win.** No candidate comparison meets the stable5M
significant-loss threshold. The largest near-threshold candidate example,
15...f6 (p29), loses.0485 versus immediate ...Nxc3 and warrants a boundary check,
not an automatic mistake label. Black develops pressure and rook activity.
White's26.b4 (p50) is an unstable but large apparent loss; ...Ra2 enters the
second rank, with ...Bxe2 ideas in the legal reference line. Black actually finds
...Ra2, then ...Qe8 and ...Rb2, and converts to mate. Probe the p50/p51 opportunity
and success sequence. The six unresolved5M rows in this game preclude describing
it as error-free; the evidence nevertheless supplies useful contrary examples
of coordinated rook play and conversion.

## Pair08: exchange decisions and preservation of an attack

**Game0, mShogi White, win.** Black's12...Bd5 (p23) offers the tactical c4
opportunity; both its size and White's singleton c4 estimate are unstable, though
the actual legal sequence c4 Nxd4 cxd5 Nxe2+ Qxe2 Bxa1 Rxa1 Qxd5 Qxe7 is preserved.
Black's17...Rae8 then leaves a strongly favorable attack. White's first and
largest stable loss is18.Qd6 (p34), E.9895 with Qf6 versus.516. The ...Rd8 line
allows queen exchanges and relieves the pressure; Qf6 preserves it. Black instead
plays18...Qxa2 (p35), a stable.468 loss, and White now finds Qf6 and eventually
mates. Thus the final win contains a clear failure to preserve the attack plus
successful exploitation of a second chance. Probe candidate24/34/36 and Black23/35.

**Game1, mShogi Black, loss.** An early16...Bf6 (p31) loss is unstable. The
first stable significant candidate comparison is22...Qc6 (p43),1.0 with ...Qe5
versus.9315: it reduces but does not surrender a large advantage. Next25...Bxc3
(p49) loses.2455 versus ...Bc6 while retaining.745. These are earlier preservation
concerns, not independent proofs of a lost game. The decisive-looking sequence
is32...e4 (p63), .9855 with ...Re8 versus.5, followed by36...Qxd1+ (p71), .5
with ...Qg3 versus0. The latter exchanges the queen for material and reaches
an inferior liquidation after Bxd1 Rxd1+ Kh2; ...Qg3 instead retains active
checks and counterplay. This is the largest stable damage and a concrete
exchange/defense question. Black had earlier exploited White's18.Kh1 (p34) and
kept substantial chances for many moves, so the final loss is a conversion and
preservation failure rather than uninterrupted poor play. Probe candidate31/63/71
and opponent34. Earlier p43/49 remain5M provisional; uncertainty at31 means the
first actual significant deterioration is not settled.

## Unaided starts

**Game0, mShogi White, win.** White reaches an Italian/Four-Knights type position
through Nf3,e4,Nc3,Bc4,d3 and castling. The mechanical opening boundary is p14.
The only candidate comparison near significance is2.e4 (p2), .5725 with d4
versus.521, but its conservative observed-range margin is only.043. It is a
threshold-straddling difference between plausible opening moves, not evidence
that e4 is bad or insufficiently popular. Black's8...Qe8 (p15) then permits
Bxf6 and king-side threats: its stable comparison is.4735 with ...h6 versus0.
White finds Bxf6, Qd2 and Qh6, maintains the attack and mates. This is a strong
unaided development/attack and conversion example; it contrasts directly with
the contextually premature Bxf6 in pair05. Probe White2/16 and Black15.

**Game1, mShogi Black, win.** The first stable opening loss is1...Nc6 (p1),
.4805 with ...d5 versus.425;2...e6 (p3) is a second loss, .4995 with ...d5
versus.4265. The latter is the largest stable candidate comparison.3...Bb4
is unstable. These are independent finite quality comparisons, not judgments
based on book rarity; no master-frequency assertion is made. The mechanical
opening boundary is p16. Black later finds active ...Nf4 and ...Rc2. The
largest apparent later loss is28...Rc6 (p55), .698 with ...Rd2 versus.512,
but this is unstable. White's29.g3 (p56) then loses.431, after which Black's
...Ne6, ...Rb6 and eventual pawn advances lead to mate. Some intermediate
candidate advantage estimates remain unstable; actual successful conversion
does not resolve each decision. Probe candidate1/3/55 and opponent56.

## What this batch can support

The provisional behavioral candidates are central pawn-break timing, threatened-
piece defense, exchange decisions, active defensive resources and preservation
or conversion of an advantage. Multiple games also contain successful tactical
exploitation, active rooks and sustained mating attacks. These are observations
to test across the remaining broad survey, not diagnoses of search/evaluation
mechanisms or a priority ordering. Opponent errors and their candidate responses
remain linked; late errors in already poor positions are not independent samples.
