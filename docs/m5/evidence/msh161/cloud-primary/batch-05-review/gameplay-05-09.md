# MSH-161 (Competitive Gameplay Diagnosis): Batch 05, pairs 05–08 and unaided

## Scope and evidence

This independent offline review covers ten completed primary cloud games under the frozen method and original AutoClaim policy. Native results are **6 wins, 3 draws, 1 loss** for mShogi: prepared 4W3D1L; unaided 2W0D0L. All three draws are native automatic threefold claims. A draw itself is not a weakness. Mac evidence is not pooled with these games, and repeated unaided trajectories remain correlated observations.

`gameplay-05-09.json` retains full legal SAN histories, native terminal events, every player decision, initial flags, matched 5M evidence, audit membership and receipt references. All 1,298 moves were replayed, including 128 supplied moves that are context only. There are 1,170 player decisions: 558 with matched 5M evidence and 612 with screening evidence only. The matched ledger has 41 provisional finite comparisons at .05 (17 candidate, 24 reference), 72 unresolved, 3 threshold-straddling, 249 same-choice controls and 193 subthreshold/reverse comparisons.

Offline checks verified 2,596 screening query hashes and 844 matched 5M query hashes. Legal replay covered 46,738 screening selected-PV plies, 21,663 matched selected-PV plies and 43,008 matched last-two-report PV plies. The 93 fixed audit controls contain no supported missed-error trigger. Two remain unresolved: pair 07/game 0/p68 and pair 07/game 1/p187. The first is selected as an attack boundary; the second remains visible and unselected. No engine, retrieval or cloud job ran during review.

All consequential labels below remain **provisional pending 20M**. E is a finite reference self-play proxy, not a probability against Oxidation. Minimum-alternative minus maximum-played margins are observed-report sensitivity, not confidence bounds. Stale-final flags remain visible; instability and conflicting bounds prevent support. Legal PVs are possible continuations, not proved outcomes. Earlier uncertainty blocks absolute first-error claims. Linked moves are not independent causes; late choices in an already unfavorable episode are secondary unless the opponent supplies renewed chances.

Ply indices are zero-based from the normal starting position, including supplied moves. Candidate is White in game 0 and Black in game 1.

## Pair 05: opposite-side attacks and exchanges

**Game 0, candidate win.** Candidate p16 a3 versus g4 has a threshold-sensitive .052 margin. Reference p19 ...Be7 versus ...c5 (.127), candidate p20 g3 versus h4 (.2805), reference p23 ...g5 versus ...Bd6 (.2835), and reference p25 ...Nxh5 versus ...g4 (.1995) are provisionally consequential. Reference p21 ...Nxf4 remains bound-conflicted. These decisions concern the order of central play, pawn advances and exchanges around the kings; they do not establish a search or evaluation cause.

The legal h4 c5 d6 Bf8 Bg3 line at p20 illustrates keeping space before exchanging. Actual White instead recaptures on f4, plays Nh5 and Qxh5, then opens the h- and g-files. Rg1+, e5 and Qxe5+ bring the queen into the attack. The p26 Qxh5 boundary is selected for discovery because its same-choice evidence is unresolved. Later White exchanges queens, advances the b-pawn, and uses Rxf6/Nxf6 to coordinate the minor pieces with the rooks. White's h-pawn advance draws Black's rook and king aside; the b-pawn then promotes and mate follows at p98. These are actual constructive continuations, not proof that every earlier choice was best.

**Game 1, native threefold draw.** Reference p16 dxe6 versus g4 (.231) and p20 Nge2 versus Qe3 (.227) have provisional support; p18 Bd3 remains unresolved. Candidate p21 ...Nxf4 instead of ...c5 has .451 support. The ...c5 line challenges the center before exchanging: ...c5 g4 Nxf4 Nxf4 Bd6. The immediate recapture at p17 is a discovery control. Candidate p29 ...Be7 straddles .05 and is included without being called significant. Reference p30 Rhf1 (.0945 to Bc4) and candidate p31 ...O-O (.075 to ...c6) have finite support. Candidate p43 ...Rac8 and reference p44 Rff3 remain unresolved.

The tactical exchange episode contains four linked .5 comparisons: reference p52 Nd4 instead of exd6, candidate p53 ...Bxd4+ instead of ...Nc3, reference p58 Rfe3 instead of Qe4, and candidate p59 ...Nxe3 instead of ...Qc7. At p53, ...Nc3 Nxe6 Nxe4 Nxd8 Rxd8 gives a concrete intermediate-move resource. At p59, ...Qc7 keeps pressure rather than immediately taking the rook. Actual ...Nxe3 Rxd8+ Rxd8 leads to queen versus two rooks with pawns. These alternatives must be checked together before assigning a missed opportunity or recovery.

Reference p82 Qxb4 remains bound-conflicted. At p100 Qe8 versus Qd8 and p101 ...Rc4 versus ...Rb4+, both margins are .5. The legal ...Rb4+ Kc3 Rh4 line uses checks and rook coordination; actual play continues with queen checks. White eventually claims threefold after p119, intended Qd7+ (`d5d7`) unplayed, version 105→106. Preserve the draw as the actual outcome. A finite late opportunity is not an observed conversion under another claim policy.

## Pair 06: tactical exchanges and activity

**Game 0, candidate win.** Reference p23 ...Ng4 versus ...Qe7 is unstable. Candidate p24 Nb5 versus Bb1 has .112 support, while reference p25 ...Nxe3 versus ...Be7 has .363. The alternative ...Be7 h3 Nf6 Nbd4 maintains more resistance. Actual ...Nxe3 fxe3 Bxe3+ Kh1 Bxc1 Bxc1 creates a material imbalance; that capture sequence is retained as linked context rather than judged from material totals alone.

White later exchanges queens through Nxb4 Qxd2 Nxd2 axb4 Bxb4 and keeps active minor pieces. The long conversion includes Nxc6, Bxe5, capture of Black's c- and d-pawns, and king activity. White stops the f-pawn with Bc6+ and Rxf2, advances connected queenside pawns, and uses Bd5 before ...Rxd5+ Kxd5. Promotion on a8 is mate at p154. These successful tactical simplifications and pawn conversion are contrary evidence to a blanket inability to convert. The earlier ...Ng4 uncertainty remains.

**Game 1, candidate loss.** Candidate p29 ...Bd6 versus ...Qe5 has .4975 support. The ...Qe5 Qg3 Qxg3 hxg3 line offers a different queen exchange and active pieces. Reference p32 Qxf4 instead of g3 has .4365 support and gives Black renewed finite defensive chances. These matched comparisons require 20M before calling a confirmed recovery.

After the actual minor-piece exchanges, candidate p43 ...Nd5 instead of ...Re2 has .488 support. The legal ...Re2 Nc3 Rc2 Rd1 h5 keeps the rook active; actual ...Nd5 Rc5 Rd8 Rd1 ties Black's pieces to defense. White exchanges rooks, wins the a-pawn, then captures the knight on g2. A later rook exchange leaves White a knight and several pawns against Black's king and pawns. White can give up the knight on g8 while the a-pawn promotes, then promotes the h-pawn and mates at p118. The selected controls cover the material transitions and promotion; no position-specific mechanism is inferred.

## Pair 07: king attacks with repeated changes in chances

**Game 0, candidate win.** Reference p17 ...Bxh6 instead of ...e5 has .4075 support. Reference p19 ...Rc8 versus ...Kh8 has .0835 support, but even the alternative has E .0835: this is secondary context in an already unfavorable finite episode. Candidate p24 Rhg1 instead of e5 has .1645 support while keeping a strong finite advantage.

The middle attack remains uncertain at several roots: candidate p32 a3, reference p33 ...Rce8, candidate p44 fxe5, reference p45 ...Qd5, candidate p46 Re3 and p50 Rc3. Reference p47 ...a5 has a reverse, unstable comparison that is retained. The alternative Rxg6 at p44 and Rg3 at p46 give legal attacking resources, but the flags prevent a settled missed-attack verdict. For example, Rxg6 Rf6 Rg2 exf4 Reg1 brings both rooks into play. Reference p57 ...Nd5 and p59 ...Nb4 also remain unstable; these rows stay in the selected panel where consequential.

Reference p61 ...Na6 versus ...Nd5 has .258 support. Candidate p70 Kc1 instead of Rxg5+ has .4225 support; the legal Rxg5+ Kf8 Rg6 Ke7 Rxh6 line keeps the initiative. Reference p73 ...Qxa5 remains unstable. Candidate p74 Rb3 instead of hxg4 (.4665) gives up part of a favorable episode; reference p75 ...Qa1+ instead of ...Nc5 (.5) provisionally restores it. Those linked opportunities must be adjudicated jointly.

Actual White then advances the g-pawn, brings a rook to h1, and combines queen checks with rook captures to remove both Black rooks. The queen exchange at p116 permits g8=Q+; the remaining rook finally mates on b8 at p124. This is a successful actual attack after several uncertain and supported adverse choices, not a claim of one uninterrupted advantage.

**Game 1, candidate win.** Reference p22 Qe3 versus f4 is unstable. Reference p24 Ne2 versus f4 has .464 support; actual ...Nxe2+ Bxe2 cxd5 clears the central files. Black coordinates the rooks on the queenside and places the knight on f4. The initial finite disadvantage for White does not prove every later move forced.

At p75, candidate ...Rca8 instead of ...Qd8 has .5 support. Reference p76 Bc4 instead of Qxd6 also has .5 support. The legal Qxd6 Nd3 Kc2 Nxf2 line shows a defensive tactical resource; actual Bc4 allows renewed Black pressure. These are one linked opportunity episode. Black later advances the h-pawn and promotes at p113, then uses ...Rxb3+ and ...Qxh1 in the ensuing exchanges. After a long queen-and-knight phase, Black advances the g- and f-pawns, exchanges queens through ...Nxd6, promotes twice and mates at p217. This actual long conversion is contrary evidence to a general conversion failure. The unselected p187 audit uncertainty remains visible and blocks any blanket claim of flawless play.

## Pair 08: quiet control games and automatic claims

**Game 0, native threefold draw.** No matched 5M comparison has a margin at least .05. The actual game exchanges queens and reaches rook/knight versus rook/bishop with pawns. White's rook activity, minor-piece exchanges, and eventual capture on c6 are retained as quiet controls. No absence of a detected event proves perfect play.

Black claims threefold after p106, intended ...Ke6 (`e5e6`) unplayed, version 92→93. The near-equal finite comparisons and draw are separate observations. Do not turn this control game into a weakness merely because neither side won.

**Game 1, native threefold draw.** Reference p50 Bxc7 versus Kc3 has .0945 support, but the played range is .5: this is loss of a small finite advantage, not a proved loss or complete collapse. The legal Kc3 b5 cxb6 cxb6 Bc7 line keeps the king active before taking material. Actual Bxc7 Bxb3 Bxa5 reaches an opposite-colored-bishop ending with pawns.

White claims threefold after p75, intended Kf2 (`g2f2`) unplayed, version 61→62. The selected queen exchange, pawn captures and bishop checks are controls. A finite alternative cannot establish that an ending would convert under another claim policy.

## Unaided starts

**Game 0, candidate win.** Reference p9 ...Bd6 versus ...Nxe4 has .13 support. Candidate p10 Nxf6+ versus O-O remains unstable. Reference p27 ...Kh8 straddles the significance threshold. The familiar attack episode again has unresolved reference p31 ...c5, candidate p36 bxc5 and reference p37 ...Bxc5 comparisons. Candidate Bxh6 and Qxh6+ are actual moves, but their presence does not establish the best timing or a confirmed opponent error. The p33 ...gxh6 recapture is selected for discovery.

This game later diverges from earlier unaided trajectories. White plays Ra3 and Nh4, wins the a-pawn, then uses f4 and Rxf4 to attack the queen on f6. Actual Rxf6 Rxg4 exchanges queens while leaving active rooks. White's knight captures on c4, both rooks invade the seventh rank, and the h-pawn eventually promotes. After the promotion exchange, White promotes the g-pawn and mates at p150. Those are successful native continuations, with the earlier sacrifice uncertainty retained. Repeated early moves are correlated, not fresh independent opening evidence.

**Game 1, candidate win.** The repeated prefix again includes candidate ...Nc6 and ...e6 margins near .055, unresolved ...Bb4 and reference Be2, and literal-subthreshold ...O-O at .04999999999999999. Do not round that value into support or treat repeated prefixes as independent samples.

Reference p26 Bc5 versus exd5 has .189 support. Candidate p29 ...exf3 remains unstable; reference p32 Qxe2 versus Bxf6 has .103 support. The actual ...dxe4, ...exf3, ...fxe2 and ...Nxe7 sequence is preserved with discovery controls at p27 and p33. Candidate p35 ...c6 versus ...Rfc8 has .059 support. Reference p38 c5 instead of Qxc5 has .3955 support. Candidate p41 ...Nfd5 versus ...Ned5 is unstable; reference p42 Qe1 versus a4 straddles .05, and even the alternative remains below .10. These later rows need secondary/threshold context, not a fresh primary-error label.

Black's actual rook activity on the a- and c-files, knight route through f5, d6 and c4, and ...Nf4/ ...Bd5 restrict White's pieces. Later ...Ne2 and ...Nxd3 win a rook; Black uses checks and a bishop attack to force a queen-for-pawn exchange and mates with ...Bxf3# at p127. The result is a successful tactical conversion, while the p29 and p41 uncertainties remain.

## Frozen selection

`selection-05-09.json` contains **234 unique roots**: 71 from the unchanged both-player baseline (matched margin at least .04, or unresolved absolute initial screening difference at least .10), plus 163 linked responses, episode boundaries and constructive/quiet controls. No per-game cap excludes baseline consequential rows. Ten unresolved same-choice roots use `[played, null]` for unrestricted discovery. A distinct discovered alternative will require its own forced 20M singleton against the reused played result.

Thirty-one unselected matched uncertainties and all screening-only rows remain in the ledger. Three threshold-straddling roots are included as linked context. All three native automatic claims remain explicit, and no draw alone is called a weakness. Freeze the selected checks before execution; do not repeat searches to obtain a desired label. No technical cause, final weakness ranking or production change is proposed.
