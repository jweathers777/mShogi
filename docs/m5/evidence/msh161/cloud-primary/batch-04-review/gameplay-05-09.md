# MSH-161 (Competitive Gameplay Diagnosis): Batch 04, pairs 05–08 and unaided

## Scope and limits

This independent offline review covers ten fresh primary cloud games under the frozen method and original AutoClaim policy. Native results are **3 wins, 4 draws, 3 losses** for mShogi: prepared 1W4D3L; unaided 2W0D0L. Two draws are automatic threefold claims and two are native dead positions. A draw itself is not evidence of weakness. No Mac game is pooled with these results, and repeated unaided openings remain correlated observations.

`gameplay-05-09.json` contains full legal SAN histories, native terminal events, all 1,325 player decisions, original screening flags, matched 5M evidence, audit membership, and receipt references. The 128 supplied moves are context and are not attributed to a player. All 2,906 screening query hashes and 1,266 matched 5M query hashes passed offline checks. Legal replay covered 53,535 screening selected-PV plies, 32,902 matched selected-PV plies, and 65,228 matched last-two-report PV plies. No engine search, retrieval, or cloud job ran during this review.

There are 850 decisions with matched 5M evidence: 67 provisional finite comparisons at .05 (35 candidate, 32 reference), 118 unresolved, 388 same-choice controls, and 277 subthreshold/reverse comparisons. Another 475 decisions have screening evidence only. The 80 fixed audit controls contain no supported missed-error trigger. Three audits remain unresolved: pair 05/game 1/p92, pair 06/game 0/p118, and unaided/game 1/p67. The last is included as a tactical continuation control; the first two remain explicit uncertainties.

All consequential labels below remain **provisional pending 20M checks**. The observed margin is minimum-alternative E minus maximum-played E across retained reports, not a confidence bound. E is a finite Stockfish self-play proxy, not a probability against Oxidation. Stale-final flags remain visible; conflicting bounds and unstable report ranges block support. Legal PVs show possible continuations, not forced results. Earlier uncertainty blocks absolute first-error claims, and linked moves are not independent causes. A later adverse choice in an already unfavorable episode is secondary unless an opponent decision restores chances.

Ply numbers are zero-based from the normal starting position, including supplied moves. Game 0 has candidate White; game 1 has candidate Black.

## Pair 05: opposite-side kings and exchanges

**Game 0, candidate loss.** White p16 cxd5 versus c5 has a .0935 provisional margin. The ensuing actual queen exchange is concrete: Bxa7, ...Nxc3, Rxe5, ...Nxd1, Rxd1. These moves are selected as context controls, not assumed errors. White p32 a3 versus Bf2 is a large but bound-conflicted comparison. The Bf2 line permits Bxa2 Rb5 Rxd3 Rxd3 and keeps rooks active; the played a3 allows ...Bd6 and a queenside net around the bishop.

Black p35 ...Rd7 versus ...b6 (.498) gives a provisional defensive opportunity; White p36 g3 instead of Bf2 returns it (.498). The alternative Bf2 c6 Ra4 Kc7 Bc2 lets the bishop escape while the rooks coordinate. Black p43 ...Ra8 versus ...g6 remains bound-conflicted, so a later renewed-chance claim is not settled. White p46 fxe6 versus Bb5 has a .196 margin; p48 Rxc5 versus Kf1 is unresolved. Actual ...Bc5+ Rxc5 Rd1+ Kf2 bxc5 explains the forcing exchange sequence, but the instability prevents a final account of the best defense.

Black then coordinates both rooks, advances the c-pawn and invades the second rank. White eventually loses the rook and bishop and is mated at p109. That actual finish does not make each earlier adverse comparison a separate primary cause. The p32 and p43 uncertainties remain material.

**Game 1, native threefold draw.** The same supplied root produces the same p16 cxd5 comparison for the reference player. White p18 f4 and Black p21 ...Be6 remain unresolved. White p22 Nxd5 versus Ne4 has .2975 support. Black p25 ...g6 versus ...Bd6 (.115) still retains a large finite advantage; p27 ...Re8 versus ...Bd6 (.2965) releases more of it. White p28 Qd2 instead of Bf2 (.4095), Black p33 ...c6 instead of ...g5 (.4065), and Black p37 ...Qxf6 instead of ...Bxf4 (.492) are linked provisional opportunities. White p32 f5 and p34 f6 remain unresolved. The ...Bxf4 Rxf4 Qg5 line keeps the queen and attack; actual ...Qxf6 Be5 Bxe5 Rxf6 Bxf6 changes the material to queen versus rooks with other pieces.

White p48 Qd3 versus Qg3+ has .4645 support. Black develops a strong rook attack and later p75 ...b6 versus ...Rfg2+ has .498 support: actual Rxb6+ axb6 Qxb6+ opens checking routes. White p88 Qxg5 versus Qe5+ remains bound-conflicted. Black p95 ...Rxb2 versus ...f5 has .449 support; White p100 Qa8+ and Black p107 ...Kb6 remain unresolved, with the latter showing a reverse comparison that must not be discarded. Much later White p148 Qf8+ versus Qe7+ (.5) and Black p149 ...Kc4 versus ...Kb3 (.499) form another linked checking episode.

After p241, White claims threefold with intended Qf3+ (`f4f3`) **unplayed**, version 227→228. The long actual checking sequence and final draw limit conversion conclusions. A finite missed opportunity is not a claim that this fixed-policy game would have continued without the automatic claim.

## Pair 06: Sicilian structure and opposite-side attacks

**Game 0, candidate win.** Reference p19 ...Ng4 versus ...e5 is threshold-sensitive (.0605). Reference p23 ...Bd7 versus ...c5 has .3085 support. Candidate p26 Be2 versus Bd3 is bound-conflicted; p28 h4 versus Nd6 has .1855 support. The Nd6 line establishes central activity before the pawn advance. Reference p27 ...c5 remains unresolved. Reference p31 ...f6 versus ...Bc6 (.5) gives a strong provisional opportunity; White p32 exf6 versus h6 (.108) retains a favorable position while using a less forcing order.

White p36 Kb1, Black p37 ...b5, and White p38 c4 remain unresolved. Black p39 ...f5 instead of ...e5 (.506) gives another provisional chance; White p40 Nc3 versus Qg3 (.305) returns part of it. Qg3 Rg8 Qc3+ e5 fxe5 supplies a legal attacking line. Black p41 ...Bd6 versus ...Bf6 is unresolved. Actual White later plays Rxd6, Rd1 and Rxd7 to remove defenders. White p60 Bc4 versus Qd6 remains unstable, so the subsequent favorable course does not certify that choice.

The actual ending is a useful success: White restrains Black's f-pawn with Bf1, exchanges queens, advances the a-pawn, captures f2, and promotes at p106 after ...Rxb7. White then wins the rook with Ng5+ and Qxg5. Black promotes the h-pawn, but White mates immediately at p136. These legal native continuations are contrary evidence to a blanket inability to coordinate passed pawns or convert. Earlier mutual errors remain separate.

**Game 1, candidate loss.** Reference p30 Qc3 remains unresolved. Black p31 ...Rab8 versus ...Qb7 has .0735 support; reference p34 Qb3 versus Qd2 has .4655 support. Black's actual ...Rb4, ...exd4 and ...dxe3 create advanced central pawns while the rooks and bishops remain active. At p55, ...Bxd4 versus ...Qa7 has a very large observed margin (.9125) but unstable reports and a conflicting bound. It is **not confirmed**. The ...Qa7 Rxb4 Bxb4 Rxc8+ Bxc8 line preserves the queen and changes the tactical exchange; actual ...Bxd4 Rxc7 Rxc7 Qxb4 trades the queen for different material.

Black p59 ...Bc5 versus ...Bf6 has .064 support, but even the alternative has E no higher than .089: this is a secondary comparison in an already unfavorable finite episode, not a fresh primary loss. Reference p64 Qd8+ versus Bb5 remains bound-conflicted. The actual Qd8+ Bf8 Qxd7 Rxa6 creates another material transition; a recovery verdict needs this linked uncertainty resolved. Black p77 ...f6 versus ...Rc7+ has .5 support. The checking defense keeps the rook active; actual ...f6 gxf6 gxf6 e5 opens more targets. White later promotes the b-pawn and mates at p140. The large p55 uncertainty and p64 resource prevent a settled single-cause account.

## Pair 07: London structure, attack and defense

**Game 0, native threefold draw.** Reference p29 ...Kh8 (.0775), candidate p30 Ra3 (.0765), reference p31 ...Ng4 (.3555), and p33 ...Nxf2 (.1945) have provisional support against their matched alternatives. Actual Bxh7, Kxf2, Ng5+, Qg4 and Qh7+ form a forcing king attack. They are selected as constructive controls; seeing the sequence does not prove that each move was uniquely best.

White p50 cxb4 instead of Qh4 (.5) reduces the finite advantage; p52 Rb1 instead of Re2 (.5005) allows ...Qe3+ and ...d4. Reference p57 ...Rc1+ instead of ...Qxd4 (.5) offers relief, but White p58 Kg2 instead of Rxc1 (.5) does not use that tested defense. Reference p59 ...Qxa3 instead of ...Qxd4 (.5) then returns the opportunity. The legal Rxc1 Qxc1+ Kg2 Qd2+ Kh3 line makes the checking resource explicit. These are one linked tactical episode, not six independent causes.

White p66 Ng5 versus Qg7 has .5005 support in the reference comparison. But native Black immediately claims threefold with intended ...Kf6 (`e7f6`) unplayed, version 52→53. The alternative reference line after Ng5 begins ...Qa2+, but that move was **not played**. Preserve this as an AutoClaim-limited terminal comparison; it is not an observed lost game or failed conversion.

**Game 1, native dead-position draw.** Black p23 ...h5 remains unstable. White p24 Nxd7 versus b4 (.0885), White p30 Qg4 versus Nf3 (.115), and Black p33 ...Kf8 versus ...e5 (.068) have provisional support. White p46 Re2 versus Qe2 (.497) allows a central knight tactic; Black p47 ...Qxa2 instead of ...Nc6 (.497) does not take the tested opportunity. The ...Nc6 Rb1 Qc3 Qe3 Nd4 line targets the loose coordination before taking a pawn.

The actual game reaches rook versus two knights with pawns. White p80 Ba4 and p98 g4 remain unresolved. Black p113 ...Kd5 versus ...Nd4 is bound-conflicted; White p116 Rd8+ instead of Rxg5 (.5) and Black p137 ...Ne5 instead of ...Ne6 (.5) have provisional support. The ...Ne6 line keeps the knight structure and pawn protected; the actual ...Ne5 permits Rxg5. These finite opportunities do not imply that the final draw is itself an error.

At the end White gives the rook for the f-pawn: Rxf2 Nxf2, then g7 Nxg7 Kxf2. Native White's final p184 Kxf2 creates the dead position, version 169→170. This actual defensive resource is contrary evidence to a blanket inability to defend an ending. Earlier unresolved alternatives still limit the episode analysis.

## Pair 08: queen's-pawn structure and endings

**Game 0, native dead-position draw.** No matched 5M comparison has a margin at least .05. This is not proof of perfect play, but it is an important quiet control game. Both sides exchange central pieces, queens and eventually rooks. White's active knight exchanges, capture on a4, queen exchange at p56, capture of the a-pawn at p66, and the final rook/pawn sequence are selected as representative controls.

Actual Rxg3 hxg3+ leads to a king-and-pawn position in which White reaches g3. Native p106 Kxg3 removes the last pawn and records a dead position, version 91→92. The draw comes from an actual material-ending state, not an automatic repetition claim. Do not assign a weakness merely because neither player won.

**Game 1, candidate loss.** Black p17 ...Nc6 instead of ...b5 has .0955 support. Black p21 ...b5 instead of ...Qxc4 has .4405 support. The queen exchange alternative ...Qxc4 Nxc4 Bd7 develops without the immediate queenside pawn commitment. The actual tactical sequence ...Nxd4, Qxd7, ...Ne2+, ...Nxd7 and Rxc7 is preserved in the selected context. It does not by itself establish the cause of the later loss.

Reference p56 Rxc3 versus Rb1 is bound-conflicted. White p60 Ng5 versus a4 has .094 support; actual ...Nxa3 Nxe6 reaches a knight ending with Black's a-pawn racing. These later resources show why the early disadvantage cannot be treated as an unbroken forced loss. Black promotes at p99 and White captures the new queen with Nxa1. At p101, ...Ne4+ instead of ...Nxe6 has .5 provisional support. The alternative ...Nxe6 fxe6 Kxe6 removes the advanced e-pawn; actual ...Ne4+ Kd4 Nf2 leaves White connected central threats. White later uses f6, Nxf6 and king activity, promotes two pawns, and mates at p154. The ending event needs 20M alongside the unresolved earlier exchange before a final recovery/loss account.

## Unaided games

**Game 0, candidate win.** Early reference ...Bd6 at p11 (.2205) and ...Na5 at p13 (.2125) give provisional chances; White p12 O-O instead of Rg1 (.147) and p14 Bd5 instead of Ba2 (.13) reduce them. Black p15 ...Kh8, White p16 b4/p18 Be3, and Black p21 ...Nxd5 remain unresolved. White p24 Bxd4 instead of Ne4 (.3345) is supported: Ne4 Nxf3+ Qxf3 Be7 d6 keeps a strong central initiative. The actual Bxd4 exd4 changes the center before the later pawn exchanges.

Reference p27 ...c6 (.1515), candidate p28 dxc6 (.1355), reference p37 ...Qa5 (.264), reference p39 ...Bg4 (.4885), candidate p40 h3 (.154), and reference p49 ...Rc8 (.487) have provisional support. Candidate p38 c4 and p42 g3 remain unresolved. In particular, the alternative Ne5 at p38 gives a concrete central attack, but its bound conflict prevents a settled missed-opportunity label. The later matched ...Bxa3 defense at p49 must be checked before attributing the actual winning attack solely to White's play.

Actual White then coordinates both rooks and knights: Nf5, Nd6, Rb7, Rxf7, Nxf7, Nh6+ and Nxf6+ lead to winning the rook on a8. White exchanges the remaining rook for Black's queen and promotes the c-pawn before mate at p110. These are successful legal continuations despite the earlier uncertain choices.

**Game 1, candidate win.** The repeated unaided prefix again has small ...Nc6 and ...e6 margins near .055, unresolved ...Bb4 and reference Be2, and literal-subthreshold ...O-O at .04999999999999999. These are correlated observations. They are not independent proof of broad opening failure, and the decimal-grid boundary is not silently rounded into support.

Reference p40 g4 instead of Be3 (.418) gives a provisional opportunity. Reference p46 Qc7, candidate p47 ...Rfc8 and p53 ...Ra8 remain unresolved. Candidate p55 ...Ng6 instead of ...Bf7 (.3) permits a strong White position; reference p56 Be3 instead of Bg3 (.367) gives some chances back. Candidate p59 ...Nh4, reference p60 f5/p62 b6, and candidate p63 ...Raa2 remain unresolved. The p63 reverse comparison is retained. Reference p64 Rb1 instead of Rd2 has .5825 support and is the central tactical boundary in the selected panel.

Actual Black responds ...Nf3, ...exf3, ...Rxe2, ...Rh2+ and ...Rag2+. After the checking sequence Black captures the rook on b1; later the queens are exchanged. This is a constructive tactical sequence, not proof that all earlier moves were sound. Black then restricts the king, advances the d-pawn, and eventually forks the rook with ...Nxe3+ before mating at p197. The win is contrary-success evidence while the p46–63 uncertainties remain.

## Frozen 20M selection

`selection-05-09.json` contains **285 unique roots**: 121 from the unchanged both-player baseline (matched 5M margin at least .04, or unresolved absolute initial screening difference at least .10), plus 164 linked responses, episode boundaries and constructive/quiet controls. No event cap excludes consequential baseline rows. Twenty-five unresolved same-choice roots request unrestricted discovery as `[played, null]`. A distinct discovered move will require its own forced 20M singleton versus the hash-bound played result; discovery alone cannot certify an event.

Forty-six unselected matched uncertainties and all screening-only rows remain visible in the ledger. They are not cleared by this panel. The comparison at pair 06/game 1/p59 is explicitly secondary under the finite already-unfavorable marker. Native claim-limited endings and native dead positions remain separate. Freeze these checks before execution; do not repeat searches to obtain a desired label. No mechanism, final priority ranking, new corpus or production change is proposed.
