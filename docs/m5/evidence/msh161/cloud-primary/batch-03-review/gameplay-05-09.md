# MSH-161 (Competitive Gameplay Diagnosis): primary cloud Batch 03, pairs 05–08 and unaided

## Scope and limits

This review covers ten completed Linux primary games. Native results are **4 wins, 4 draws, 2 losses** for mShogi: prepared games 2W4D2L; unaided games 2W0D0L. The native record remains the outcome authority. All four draws are automatic threefold claims. No draw itself establishes a weakness. These games must not be pooled with exploratory Mac games or treated as ten independent opening samples.

The structured ledger `gameplay-05-09.json` retains full SAN histories, every played decision, both-player reference comparisons, initial flags, later flags, audit membership, legal reference continuations, receipt paths, and native terminal events. There are 1,095 moves including 128 supplied moves; 967 decisions belong to the players. The supplied opening moves are context only. Offline checks verified 2,190 screening receipt hashes and 905 matched 5M receipt hashes. They replayed 40,441 screening selected-PV plies, 24,419 matched selected-PV plies, and 48,540 matched last-two-report PV plies. No engine search, cloud job, or retrieval was started for this review.

Of 589 decisions with matched 5M evidence, 56 have a finite observed margin at least .05, 93 remain unresolved, 2 straddle the significance threshold, 243 are same-choice controls, and 195 are subthreshold or reverse comparisons. The remaining 378 decisions have screening evidence only. These counts are comparison states, **not confirmed errors or independent episodes**. There are 62 audit controls in this scope; none has a supported margin at least .05. Initial uncertainty and negative comparisons remain visible. No audit expansion trigger was found here.

All consequential claims below are provisional pending 20M checks. The expected-score proxy E is a finite Stockfish self-play estimate, not the probability of beating Oxidation. The minimum-alternative minus maximum-played margin uses observed reports, not confidence bounds. Legal PVs show possible continuations, not forced outcomes. Stable stale-final-report flags remain visible; unstable depths and conflicting final bounds block support. Earlier uncertainty prevents an absolute first-error claim. Repeated errors in one attack or ending are linked, not counted as independent causes. Later errors at E ≤ .10 can be secondary to an already unfavorable position; they do not create a fresh primary episode without a confirmed opponent opportunity.

Ply numbers are zero-based from the normal starting position, including supplied prefixes. Game 0 has mShogi White; game 1 has mShogi Black.

## Pair 05: opposite-side kings and central tension

**Game 0, native draw.** At p24, c6 has a .492 provisional margin against Rhe1. The alternative develops the rook while preserving central flexibility: Rhe1 Rxc5 Nd4 Rcc8 f3 b5. The played line c6 bxc6 Qe1 Qc7 allows Black active central play. But Black actually replies Bxc6 at p25, a .463 provisional difference from bxc6. Thus the initial loss of chances and the reply must be judged together.

White p30 a3 versus Kb1 (.194), p34 f5 versus Nxd5 (.249), p36 Qe1 versus f6 (.166), and p40 Nb3 versus Nxd5 (.461) form one central/king-safety episode. Black p31 ...Kh8 and p37 ...Nc4 remain unresolved; p39 ...exf5 provisionally releases a large advantage. At p40, Nxd5 b4 Nb3 Qd8 axb4 Re8 Qg3 supplies a concrete alternative. The repeated central capture is a gameplay observation, not a search or evaluation diagnosis.

The queen exchange sequence at p57–61 is unresolved or threshold-sensitive. White p62 g5 versus Nd6 (.290) and p64 Nc3 versus b3 (.364) concern coordination in the rook/two-knight ending. The b3 line gives the king an escape and keeps the knights connected: b3 Rc7 Kb2 Rc6 Nb4 Re6 Nc2. Black's p107 and p111 choices and White p108 Ne4 occur in an already unfavorable White episode; they must not be promoted to fresh independent primary errors. The native record ends after p112 with Black claiming threefold, intended ...Kb3 (`a3b3`) unplayed, version 98→99. The observed draw limits any conversion conclusion.

**Game 1, candidate loss.** Black's exchange sacrifice ...Rxc3 at p23 and ...Qxa2 at p25 remain unstable; the line ...Rxc3 bxc3 Nc6 f3 Rc8 shows the compensation question but does not settle it. The p29 ...b6 comparison has a conflicting bound. White p28 Rb1 and p30 Rb4 also remain unresolved. Those linked uncertainties prevent a final statement about where the losing course began.

Black p39 ...f6 versus ...b5 has a .266 provisional margin. White's p42 Bf1 is unresolved; Black's p43 ...Qa3 versus ...Be8 has .479 finite support. After ...Be8, the line Reb1 h5 Be2 Na4 Ra1 offers more resistance; after ...Qa3, Rbb1 Be8 Ra1 attacks the exposed queen and queenside. White p44 Bb5 and Black p45 ...a5 are unresolved linked responses. Actual White then doubles rooks, takes b6, and obtains a rook-versus-knight ending. White advances the c-pawn, captures the knight at p120, promotes at p124, and mates at p126. This is actual reference conversion, with earlier cause unresolved.

## Pair 06: King's Gambit structure

**Game 0, candidate loss.** White p20 Re1 versus Qd3 has a .0915 provisional margin. Qd3 a5 Bd2 Ra6 Rae1 O-O develops while maintaining central activity. White p24 Kh1 versus g3 is consequential but unresolved: the alternative has depth instability and a final-bound conflict. The played position has E = 0 in its matched reports; that is finite evidence, not proof of a forced loss. The line g3 g4 Ne5 f3 Kf2 shows a concrete way to contest the pawn front.

The actual Black continuation ...Bg4, ...Ra6, ...Re6, and ...Qd5 coordinates pieces against White's center. Later ...Qxd4 and ...Rdxe4 lead to exchanges and an active rook/bishop attack. White makes checking attempts, but Black advances ...g4 and ...g3+, then mates at p99. The selection includes these early defensive boundaries and Black constructive controls. It does not attribute every late move to a new mistake.

**Game 1, native draw.** White p20 Kh1 has a .443 comparison to Qd3. Black p23 ...Be6 versus ...Bg4 remains unresolved. White p24 hxg5 then provisionally releases renewed chances; Black p27 ...Nc6 versus ...Nd7 (.481) and White p28 Rd1 versus d5 (.482) alternate large missed opportunities. The ...Nd7 line prepares ...Bg4 and rook pressure; the d5 defense displaces Black's central pieces. All these decisions belong to the same opening attack.

Black's p41 ...Qa5 (.088 to ...Qf5) retains a strong finite advantage, so it is not a complete loss of that advantage. At p51 ...Re4 versus ...g3 the .368 margin is more consequential: ...g3 Nf3 Bf2 Be1 Bxe1 Rfxe1 Rxe1+ gives a forcing simplification. Actual ...Re4 Rde1 Rxe1 Bxe1 Re8 Nxg4 allows the pawn threat to disappear. Black p57 ...Kg7 remains unresolved. Much later, p109 ...Ng5+ versus ...Ng7 has a .5 observed margin but a conflicting bound; its interpretation must include the imminent claim. White claims threefold after p111, intended Kd5-e4 (`d5e4`) unplayed, version 97→98. Neither the reference comparison nor the draw proves an actual conversion failure under another claim policy.

## Pair 07: open-center exchange and rook activity

**Game 0, native draw.** White p22 Bxf6 versus Nxc3 has .359 provisional support. The alternative Nxc3 Ng6 Nd5 a6 bxa6 Rxa6 d4 brings pieces into the center before exchanging the bishop. White p26 bxa6 and p28 d4 remain unresolved; Black p25 ...a6 is also unresolved. Black p35 ...Nc6 versus ...c5 provisionally gives White renewed chances (.475). The actual Bc4, O-O and rook activity are linked response controls, not proof of a perfect recovery.

At p82 Rf8 versus Qd1, the finite margin is .506. The legal continuation Rf8 Kxf8 Ng2 f5 explains why the rook move requires review. But actual Black claims threefold instead of making that capture: intended ...Bc4 (`e6c4`) is unplayed, version 68→69. This terminal decision is an important AutoClaim-limited observation. Do not report a lost rook or lost game that did not occur, or call the draw evidence of poor conversion.

**Game 1, native draw.** White p40 Bc1 versus Be3 is unresolved; Black p41 ...Ra2 versus ...Ra1 (.059) still retains a large finite advantage. Black p53 ...Bxc4 versus ...Nc5 has a large observed margin but a conflicting cp bound. The ...Nc5 line retains pressure before exchanging: ...Nc5 Bf3 Bxc4 Rg1 Bb3. Black p67 ...R4a5 versus ...Qb5 (.5) then permits Be3, attacking the loose knight/rook coordination. Actual ...Re8 and ...Rxe3 change the material to two rooks against rook and knight; these are linked consequences, not separate established causes.

The ending has alternating opportunities: White p82 g4 instead of Rc7 (.5), Black p87 ...Kg7 instead of ...g5 (.482), White p90 Rf1 instead of Rd4 (.495), Black p91 ...Re2 instead of ...Rxf1+ (.497), and White p94 Rd5 instead of Rd4 (.499). The defensive ...Rxf1+ Kxf1 Nc5 simplifies; ...Re2 leaves both White rooks active. These require joint 20M review before any recovery/conversion label. Native White eventually claims threefold, intended Kf2 (`e3f2`) unplayed, version 129→130.

## Pair 08: opposite-side Sicilian attacks

**Game 0, candidate win.** White p16 Qg3 versus g4 is near threshold (.056). Black p17 ...O-O (.315 to ...Nbd7), p19 ...Nh5 (.208 to ...Ne8), and p27 ...b4 (.162 to ...Rg8) offer provisional attacking chances. White p18 Bh6, p26 Qxh6 and p28 Bd3 remain unresolved. The actual Qg4 and Qxh5 exchange sequence is a constructive control, not evidence that Bh6 was uniquely best.

White continues e5, exd6, f5 and Nxf5; later Nd6, Qf6+, and Rhe1+ bring the pieces into the attack. The actual finish Rxe5+ Qxe5 Qxe5+ Re6 Qc5+ Ke8 Nxe6 wins material while keeping checks; Qc7# ends the game at p74. This is contrary evidence to a blanket inability to coordinate an attack. Early comparison uncertainty remains.

**Game 1, candidate win.** Black p19 ...O-O versus ...b5 has .467 provisional support. White p20 Ka1 versus g4 is unresolved, so the early missed attack cannot be called confirmed. Black p27 ...b5 (.212 to ...Nb4) and White p30 Rd2 (.489 to Kb1) precede an unstable tactical exchange. Black p31 ...Nc6 versus ...Rfc8, p35 ...Na5 versus ...Nd4, and White p36 b3 versus Bxh6 remain unresolved, despite large observed margins. The ...Rfc8 line includes ...Qa5 and ...Rxc3; White's Bxh6 alternative attacks the king before allowing ...Qc3+.

Black p39 ...Rac8 versus ...Nc4 has a .125 margin but keeps a near-equal finite estimate. White p40 Bd3 versus Qd3 has .503 support. Actual Black responds ...g6, ...f5, ...e4 and ...Bf6, then captures the rook on d4. The linked response controls cover that forcing sequence. Black later exchanges queens and promotes the b-pawn at p101 before mating at p109. Both color wins show successful attacking continuations, with early unresolved mutual chances preserved.

## Unaided starts

**Game 0, candidate win.** Black p9 ...Bd6 versus ...Nxe4 has a .130 provisional margin. White p10 Nxf6+ versus O-O is unresolved. Later Black p31 ...c5 versus ...f6, White p36 bxc5 versus Nh4, and Black p37 ...Bxc5 versus ...Bc7 remain unresolved. White's actual Bxh6, Qxh6+, b4 and bxc5 form the linked attacking sequence. The alternative Nh4 f5 Qg6+ Qg7 Qxd6 shows why the intermediate knight move needs deeper comparison; no causal verdict is assigned from the sacrifice alone.

White brings the rooks to the a- and b-files, advances the c-pawn, promotes at p98, and uses Ne7+ at p102 to recover Black's rook after the promotion exchanges. Mate at p118 is an actual successful continuation. The selection includes the sacrifice boundary, rook activity, promotion and fork as contrary-success controls.

**Game 1, candidate win.** Early ...Nc6 at p1 and ...e6 at p3 have small margins near .055; p5 ...Bb4 remains unresolved. Reference replies are included. These correlated unaided opening choices do not establish a broad opening weakness. The p13 ...O-O literal margin is .04999999999999999 and stays subthreshold; decimal-grid sensitivity does not silently change classification.

White p26 Bc5 creates a provisional opportunity; Black's p29 ...exf3 versus ...Qe8 remains unresolved. The actual ...dxe4, ...exf3, ...fxe2 and ...Nxe7 sequence accepts a queen exchange for coordinated pieces. White p32 Qxe2 is provisionally inferior to Bxf6. Black p35 ...c6 and p37 ...Rfe8 have smaller finite differences, which do not by themselves explain the later result.

White p50 Rde2 is unresolved. Black p55 ...Re6 (.213 to ...g6) and p57 ...g5 (.500 to ...Nfd6) give White potential relief; White p58 Qb7 instead of Rxe4 (.500) returns the opportunity. The Rxe4 Rxe4 Rxe4 Bxe4 line is a concrete simplifying defense. Actual Black plays ...Rae8, ...Nd2 and ...Nf3+, then wins the rook with ...Nxe1+. Later ...Rh8+ and ...Bd5 force the queen exchange; Black mates at p111. This successful tactical continuation coexists with earlier missed chances.

## Frozen next checks

`selection20m-05-09.json` selects 230 decisions. The baseline includes all matched 5M margins at least .04 and consequential unresolved rows with absolute initial screening difference at least .10. Added rows cover linked replies, favorable boundaries, plausible defenses, and constructive play. There is no per-game event cap. Sixteen unresolved same-choice roots request unrestricted discovery with the played singleton retained. A distinct discovered alternative will need its own matched singleton before comparison. The coordinator must preserve the selections and remaining uncertainty; do not retry to obtain a desired conclusion.

Unselected uncertainty and screening-only decisions remain unresolved for final first/largest attribution. No mechanism hypothesis, final weakness ranking, source change, or new engineering priority follows from this batch review.
