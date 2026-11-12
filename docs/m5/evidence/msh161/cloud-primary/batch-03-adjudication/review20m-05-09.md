# MSH-161 (Competitive Gameplay Diagnosis): Batch 03, 20M review of pairs 05–08 and unaided

## Evidence and interpretation

This is an offline overlay on the independent initial gameplay review. The frozen plan has 230 selected roots in this scope. All 386 query receipts passed local artifact verification and matched their original native full-history decisions. Selected, last-two-depth and final-report PVs were legally replayed: 43,319 plies. No engine, retrieval, or cloud job ran during this review. `dispositions20m-05-09.json` records every selected root, query artifact, flags, observed ranges, legal lines, sensitivity, and earlier uncertainty. Native outcomes remain 4W4D2L, split into prepared 2W4D2L and unaided 2W0D0L.

There are 54 supported finite comparisons at .05: 29 candidate and 25 reference, with 33 major, 13 serious and 8 significant margins. There are also 58 unresolved roots, 1 threshold-straddling root, 41 supported subthreshold roots, 75 same-choice controls, and 1 discovery requiring a new matched singleton. Supported counts at .04/.05/.06 are 55/54/51. These are finite comparisons, not independent causal errors or population frequencies. Same-choice controls establish only the tested played move's evidence; they do not establish unique best play.

The 20M evidence supersedes 5M only at these exact roots. Earlier and linked uncertainty, unselected rows, and screening-only rows remain. No absolute first/largest claim is made. Later losses of E in an already unfavorable episode are secondary until a separately supported opponent opportunity restores chances. E is a finite reference self-play proxy, not a measured probability against Oxidation. Stable stale-final flags remain visible; conflicting bounds and unstable ranges block support. No extra search is requested merely to clear those flags.

## Discovery coverage and follow-up

All 16 unrestricted discovery roots were checked against their existing forced 20M played queries. Fifteen returned the played move; no new singleton is needed at those roots:

| Pair / game / ply | Discovered move | Forced-played disposition |
|---|---|---|
| 05 / 1 / 24 | bxc3 | unresolved |
| 05 / 1 / 26 | Kd2 | same-choice control |
| 05 / 1 / 41 | ...Nc5 | unresolved |
| 06 / 0 / 23 | ...hxg5 | same-choice control |
| 06 / 1 / 45 | Qxg1+ | same-choice control |
| 06 / 1 / 111 | Ne6 | same-choice control |
| 07 / 0 / 24 | Nxc3 | unresolved |
| 07 / 0 / 38 | O-O | same-choice control |
| 07 / 1 / 39 | ...Nd7 | same-choice control |
| 07 / 1 / 51 | ...c4 | same-choice control |
| 08 / 0 / 30 | e5 | same-choice control |
| 08 / 0 / 32 | exd6 | same-choice control |
| unaided / 0 / 33 | ...gxh6 | same-choice control |
| unaided / 1 / 27 | ...dxe4 | unresolved |
| unaided / 1 / 33 | ...Nxe7 | unresolved |

At **main-03-06, game 0, p22**, unrestricted discovery selects **g3 (`g2g3`)**, different from played **hxg5 (`h4g5`)**. `followup-selection-05-09.json` requests exactly one new forced 20M g3 query and names the completed played job for hash-bound reuse. Discovery alone is not a matched event. This is an earlier defensive boundary before p24 Kh1 and must remain open in the episode account.

Linked PV replies were also checked. New reply names occur at pair 05/game 0/p26 (a3), pair 05/game 1/p30 (g4), p36 (Rbb1), p40 (Qg1), and pair 08/game 1/p31 (...Qa5). These are legal line resources, not newly confirmed superior root choices. No new superiority or missed-response claim is made for them. The existing matched alternatives remain the scope of each reported margin. In particular, p36 stays unresolved; p26 and p40 controls do not prove uniqueness. The p31 ...Rfc8 comparison already has its own forced 20M evidence. This avoids treating a parent's PV reply as an adjudicated child comparison.

## Pair 05

**Game 0, draw.** The early linked pair is supported: p24 c6 versus Rhe1 has margin .4875; reference p25 ...Bxc6 versus ...bxc6 has .479. The latter restores a near-equal finite position after the unfavorable pawn advance. The Rhe1 line develops and permits central exchanges; ...bxc6 instead supports ...Qc7 and ...c5. White p30 a3 versus Kb1 now has .317; Black p31 ...Kh8 versus ...Ne8 has .214. These support a repeated change in chances, not a single unbroken decline.

The initial p34 f5, p36 Qe1 and p40 Nb3 findings no longer have stable 20M support; they remain unresolved. Black p37 ...Nc4 (.1565 to ...b4), p39 ...exf5 (.481 to ...b4), and p57 ...Qc4 (.3525 to ...Qd8) are supported. White p58 Rf4 and p62 g5, and the ending choice p64 Nc3, remain unresolved. The 20M ...Qc4 Qxc4 Rxc4 Ne3 line explains the queen-exchange resource, but the independent p58 comparison remains unstable. Do not promote that line to a confirmed missed recovery. Late p108 Ne4 is unresolved and p111 ...Ka3 is subthreshold; the previous late provisional labels are not retained as settled errors.

Black claims threefold after p112, intended ...Kb3 unplayed, version 98→99. Thus the draw does not establish a failed Black conversion or a successful White rescue under a different claim policy.

**Game 1, loss.** The exchange-sacrifice decisions p23 ...Rxc3 and p25 ...Qxa2 remain unresolved. Support now exists for White p28 Rb1 versus f3 (.1835), Black p29 ...b6 versus ...Qc7 (.2025), White p30 Rb4 versus f3 (.334), and Black p35 ...Rc8 versus ...Nd7 (.1255). The p35 alternatives are both unfavorable for Black; its classification must be linked to the earlier unresolved sacrifice, not counted as a fresh original loss.

Black p39 ...f6 versus ...b5 has .2855; p43 ...Qa3 versus ...Be8 has .478. After ...Qa3, the legal Rbb1 a5 Rxb6 line gives concrete queen/rook pressure; after ...Be8, Reb1 h5 Bb5 Bf7 keeps more resistance. White p42 Bf1, p44 Bb5, and Black p45 ...a5 remain unresolved. The actual reference rook activity, c-pawn promotion and mate are unchanged. Earlier uncertainty prevents complete attribution to the queen move alone.

## Pair 06

**Game 0, loss.** White p20 Re1 versus Qd3 is supported at .092. White p24 Kh1 versus g3 now has stable .386 support: g3 g4 Nh2 f3 Bh6 Re8 Re5 contests the pawn front, while Kh1 allows ...f6 and sustained central pressure. However, newly discovered p22 g3 requires its singleton before the earlier defensive sequence can be closed. The actual ...Ra6/...Re6 coordination and eventual mate remain observations. They do not prove the exact move at which the loss became inevitable.

**Game 1, draw.** Reference p20 Kh1 (.472 to Qd3) and p24 hxg5 (.4355 to Qe2) are supported. Candidate p23 ...Be6 remains unresolved. The earlier large p27 ...Nc6 and p28 Rd1 labels lose stable support at 20M; both are unresolved. Candidate p51 ...Re4 versus ...g3 also becomes unresolved. Its legal ...g3 Nf3 Bf2 line remains explanatory only; it does not establish a confirmed missed conversion. Candidate p57 ...Kg7 versus ...Re2 now has .1315 support, with ...Re2 c3 Bg7 a4 maintaining more activity.

At p109, ...Ng5+ versus ...Ng7 now has stable .5 support. The ...Ng7 Bxf6+ Rxf6 Rxf6 Kxf6 line shows a defensive simplification. The actual game instead ends after p111 with White claiming threefold, intended Kd5-e4 unplayed, version 97→98. Report the finite late comparison separately from the actual draw.

## Pair 07

**Game 0, draw.** White p22 Bxf6 now has a conflicting bound, so the 5M label is unresolved. Black p25 ...a6 remains unresolved. Black p35 ...Nc6 versus ...c5 remains supported at .467; actual White Bc4 and castling are constructive responses, not proof that all preceding choices were accurate.

At p82, Rf8 versus Qd1 remains supported at .508. Rf8 Kxf8 is a legal reference line, but **Black did not capture the rook**: native Black claimed threefold, intended ...Bc4 unplayed, version 68→69. This is an AutoClaim-limited terminal choice, not an observed lost rook or lost game.

**Game 1, draw.** White p40 Bc1 versus Be3 now has .472 support. Black p53 ...Bxc4 versus ...Nc5 now has .5 support; ...Nc5 Bf3 Bxc4 Rg1 Bb3 keeps pressure before exchanging. Black p67 ...R4a5 versus ...Qb5 remains supported at .501. After ...R4a5, Be3 attacks the loose coordination; the actual exchange sequence changes the material to two rooks against rook and knight.

The ending's alternating chances remain supported: White p82 g4 (.5), Black p87 ...Kg7 (.496), White p90 Rf1 (.5), Black p91 ...Re2 (.498), and White p94 Rd5 (.4935). The matched ...Rxf1+ Kxf1 Nc5 line at p91 gives a concrete simplification. Treat these as linked ending decisions, not five independent causes. White ultimately claims threefold, intended Kf2 unplayed, version 129→130. No completed conversion was observed.

## Pair 08

**Game 0, win.** White p16 Qg3 is now subthreshold (.018), so the initial near-threshold label is not retained. Black p17 ...O-O and p19 ...Nh5 are unresolved at 20M. White p18 Bh6 versus f4 now has .126 support; Black p25 ...b5 versus ...Nd7 has .0835. White p26 Qxh6, Black p27 ...b4, White p28 Bd3 and Black p35 ...Bb7 remain unresolved. The actual e5/exd6 and later rook/queen attack, ending in mate, are contrary-success evidence. They do not erase the unresolved early opportunities.

**Game 1, win.** Candidate p19 ...O-O versus ...b5 (.451) and reference p20 Ka1 versus g4 (.473) are both supported. Candidate p21 ...e5 is just above threshold (.0525), sensitive to .06. Candidate p27 ...b5 (.1185), reference p30 Rd2 (.4845), candidate p31 ...Nc6 (.639), reference p32 Nd5 (.1115), and candidate p35 ...Na5 (.4495) have support. Reference p28 Be2 and p36 b3 remain unresolved. Thus the successful final attack came after repeated mutual opportunities; it was not a consistently favorable course.

The matched p31 ...Rfc8 line includes Kb1 Qa5 a3 Rxc3 and shows concrete queenside pressure. Candidate p39 ...Rac8 versus ...Nc4 has .1195; reference p40 Bd3 versus Qd3 has .5. Actual ...g6, ...f5, ...e4 and ...Bf6 win material. Subsequent queen exchange, b-pawn promotion and mate remain successful native continuations, with linked p36 uncertainty preserved.

## Unaided games

**Game 0, win.** Reference p9 ...Bd6 (.128) and candidate p10 Nxf6+ (.160 to O-O) are now supported. Reference p27 ...Kh8 is significant at .0565 but not .06. Reference p31 ...c5 remains unresolved. Candidate p36 bxc5 versus Nh4 now has .4065 support; the Nh4 f5 Qg6+ Kh8 Nxf5 line gives a concrete attacking alternative. Reference p37 ...Bxc5 remains bound-conflicted, so a fully confirmed renewed-opportunity claim is premature. The actual rook activity, c-pawn promotion, knight fork and mate remain contrary successes.

**Game 1, win.** Candidate p1 ...Nc6 and p13 ...O-O are now unresolved; neither retains its earlier threshold label. Candidate p3 ...e6 (.0665) and p5 ...Bb4 (.0585) are supported; reference p2 Nc3 (.0655) is supported and p6 Be2 remains unresolved. Candidate p5 is sensitive to .06. These are correlated unaided opening choices, not a broad population finding.

Reference p26 Bc5 (.1285), candidate p29 ...exf3 (.087), reference p32 Qxe2 (.1585), and candidate p37 ...Rfe8 (.117) have support. Candidate p35 ...c6 straddles the threshold: observed margin .034, selected-score difference .063. Candidate p33 ...Nxe7 remains unstable despite discovery returning the same move. Reference p50 Rde2 remains bound-conflicted. The earlier candidate p55 ...Re6 deterioration is not retained: its new observed margin is only .01 and a bound conflict remains.

Candidate p57 ...g5 (.4995) and reference p58 Qb7 (.4995) remain a supported linked missed-defense episode. The matched Rxe4 Rxe4 Rxe4 Bxe4 line gives White a concrete simplifying resource. Actual Black instead obtains ...Nd2, ...Nf3+ and ...Nxe1+, later forces the queen exchange and mates. This is successful tactical continuation after an earlier released opportunity.

## Remaining work

Run only the one distinct discovery singleton in the saved follow-up selection, reusing the hash-bound played query. Preserve all other unresolved results without repeated searches to seek a label. The follow-up can close alternative coverage at p22, not establish an absolute first error or a technical cause. No final priority, mechanism hypothesis, or production change is proposed.
