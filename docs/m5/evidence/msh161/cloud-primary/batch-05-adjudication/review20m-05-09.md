# MSH-161 (Competitive Gameplay Diagnosis): Batch 05 20M review, pairs 05–08 and unaided

## Coverage and limits

All 234 frozen selected roots have completed reference evidence. Offline checks verified 363 query receipts against the original native full histories and legally replayed 38,017 PV plies across selected, last-two-depth and final reports. No search, retrieval or cloud job ran during review. Native results remain **6W3D1L**, split into prepared 4W3D1L and unaided 2W0D0L. All three draws remain automatic threefold claims under the original policy.

`dispositions20m-05-09.json` contains all 234 normalized rows with candidate identity, source/plan/artifact bindings, status, margin, severity, secondary context, threshold sensitivity, legal continuations and earlier uncertainty. Counts are **44 supported finite comparisons, 49 subthreshold comparisons, 106 same-choice controls and 35 unresolved roots**. The finite comparisons divide into 20 candidate and 24 reference choices, with 25 major, 10 serious and 9 significant margins. Sensitivity counts at .04/.05/.06 are 49/44/40. These are finite comparison counts, not independent causes or population frequencies.

The 20M result supersedes 5M only at each tested root. Unselected earlier uncertainty and screening-only rows remain. E is a reference self-play proxy, not a probability against Oxidation. Observed ranges are not confidence bounds; legal PVs are not forced outcomes. No absolute first/largest error is claimed. Five subthreshold rows have conservative secondary-context tags because both tested upper E values are ≤ .10: pair 06/game 1/p75; pair 07/game 0/p19 and p43; pair 07/game 1/p74; unaided/game 1/p42. That operational tag does not prove a forced loss.

Three threshold-straddling roots normalize to unresolved: pair 05/game 1/p29 (.0485), pair 05/game 1/p43 (literal .04999999999999999), and unaided/game 1/p35 (.034). Their selected-score differences exceed .05 but the observed margins do not. No rounding silently changes their labels.

## Discovery and linked-reply closure

All ten unrestricted discoveries returned the played move:

| Root | Returned move | Final control status |
|---|---|---|
| 05 / game 0 / p26 | Qxh5 | same-choice |
| 05 / game 1 / p17 | ...fxe6 | same-choice |
| 07 / game 0 / p34 | f4 | same-choice |
| 07 / game 0 / p48 | bxa5 | same-choice |
| 07 / game 0 / p68 | Qh5 | same-choice |
| 07 / game 0 / p72 | h3 | same-choice |
| 08 / game 1 / p46 | cxb3 | unresolved |
| unaided / game 0 / p33 | ...gxh6 | same-choice |
| unaided / game 1 / p27 | ...dxe4 | unresolved |
| unaided / game 1 / p33 | ...Nxe7 | unresolved |

The unrestricted cxb3 report is unstable despite a stable forced played control, so that root remains unresolved. All discovery flags remain visible. The other two unresolved rows also retain instability in the available evidence. A same-choice control does not establish unique best play.

Three new linked PV reply names were checked: ...Rf7 at pair 05/game 1/p101; Ne2 at pair 06/game 0/p24; and Rd3 at unaided/game 1/p42. No new superiority claim is made for these names. The p101 claim compares the completed forced ...Rc4 and ...Rb4+ queries, not the parent's ...Rf7 line. The p24 comparison remains unresolved. The p42 comparison is subthreshold and secondary. Thus no new child-root query is needed to support the actual claims in this review.

`followup-selection-05-09.json` is the frozen empty list. There is no distinct discovery alternative, no required additional linked comparison, and no request to repeat an unstable search.

## Pair 05

**Game 0, candidate win.** Candidate p16 a3 versus g4 remains barely significant (.0505), sensitive to .06. Reference p19 ...Be7 versus ...c5 retains .076 support. Candidate p20 g3 loses stable support because the played range is unstable and has a conflicting bound. Reference p21 ...Nxf4 versus ...c5 now has .1825 support; reference p23 ...g5 versus ...Bd6 retains .2605. These support concrete errors in the exchange/central-play episode without proving the exact best timing of White's pawn advance.

Reference p25 ...Nxh5 loses stable support; its alternative ...g4 is unstable and conflicted. The Qxh5 discovery at p26 returns the played move with stable evidence. Actual White opens the g-file, plays Rg1+, e5 and Qxe5+, and later exchanges queens. The native Rxf6/Nxf6 sequence, b-pawn advance, promotion and mate remain constructive successes. They do not turn the unresolved p20 and p25 choices into confirmed events.

**Game 1, threefold draw.** Reference p16 dxe6 (.2185), p18 Bd3 (.1205) and p20 Nge2 (.218) have support against their matched alternatives. Candidate p21 ...Nxf4 versus ...c5 remains major (.4575): the legal ...c5 g4 Nxf4 Nxf4 Bd6 line contests the center before completing exchanges. Candidate p27 ...Bxa4 now has an unstable reverse comparison; it is not dropped. Candidate p29 ...Be7 remains threshold-straddling. Reference p30 Rhf1 versus Bc4 (.1035) and candidate p31 ...O-O versus ...c6 (.112) remain supported. Candidate p43 ...Rac8 straddles the threshold, while reference p44 Rff3 versus h4 now has .059 support and is sensitive to .06.

The linked exchange episode remains supported at all four main roots: reference p52 Nd4 versus exd6 (.5), candidate p53 ...Bxd4+ versus ...Nc3 (.5015), reference p58 Rfe3 versus Qe4 (.5), and candidate p59 ...Nxe3 versus ...Qc7 (.5). The ...Nc3 Nxe6 Nxe4 Nxd8 Rxd8 line is a concrete intermediate-move resource. The ...Qc7 alternative maintains the attack instead of immediately taking the rook. These are correlated choices within one episode, not four independent causal failures.

Reference p82 Qxb4 remains unresolved. Reference p100 Qe8 versus Qd8 and candidate p101 ...Rc4 versus ...Rb4+ retain .5 support. The matched ...Rb4+ Kc3 Rh4 line uses checks and rook coordination; it is distinct from the parent's untested ...Rf7 reply. Native White claims threefold after p119 with intended Qd7+ unplayed, version 105→106. The finite late opportunities and the actual draw stay separate; no alternate-policy conversion is asserted.

## Pair 06

**Game 0, candidate win.** Reference p23 ...Ng4 versus ...Qe7 now has .3555 support. The legal ...Qe7 h3 Rfd8 Ne2 Ba3 line develops without the immediate sacrifice. Candidate p24 Nb5 versus Bb1 loses stable support; its played range and final bound remain uncertain. Reference p25 ...Nxe3 versus ...Be7 is now bound-conflicted despite a large observed margin. Thus the subsequent ...Nxe3 fxe3 Bxe3+ Kh1 Bxc1 material transition cannot be called a fully confirmed sacrifice error.

The actual queen exchange, active minor pieces, capture of Black's central pawns, and king advance remain the native story. White stops the f-pawn with Bc6+ and Rxf2, advances connected queenside pawns, and uses Bd5 before ...Rxd5+ Kxd5. Promotion on a8 is mate. This long successful conversion is contrary evidence, but does not resolve the earlier p24/p25 alternatives.

**Game 1, candidate loss.** Candidate p29 ...Bd6 versus ...Qe5 retains .5 support. The new matched ...Qe5 line includes Ne4 Qxb2 Nxf6+ Qxf6 Qxf6 gxf6, offering an active simplification rather than allowing Na4 against the queen. Reference p32 Qxf4 versus g3 retains .362 support, but candidate p33 ...Bxf4 is unstable. Therefore a complete renewed-opportunity account must retain this linked uncertainty.

Candidate p43 ...Nd5 versus ...Re2 retains .493 support. The legal ...Re2 Nc3 Rc2 Rd1 h5 line keeps the rook active; actual ...Nd5 Rc5 Rd8 Rd1 leads to a more passive defense. Later White wins material, exchanges rooks, and advances the a-pawn while allowing the knight to be taken on g8. Two promotions and mate are native outcomes. The earlier uncertain recapture prevents an absolute single-cause verdict.

## Pair 07

**Game 0, candidate win.** Reference p17 ...Bxh6 versus ...e5 retains .427 support. Reference p19 ...Rc8 becomes subthreshold (.0485), with both tested moves in an already unfavorable range. Candidate p24 Rhg1 versus e5 retains .1945 support while the played position remains favorable. Reference p31 ...Qe7 remains bound-conflicted. Candidate p32 a3 versus f4 now has .171 support; reference p33 ...Rce8 remains unresolved.

Candidate p44 fxe5, reference p45 ...Qd5, candidate p46 Re3 and reference p47 ...a5 remain unresolved. Their flags still block a settled missed-attack chain. Candidate p50 Rc3 versus Qh4 now has .294 support: Qh4 c5 dxc5 Qxc5 Rd1 keeps more pressure than the tested rook move. Reference p57 ...Nd5 becomes subthreshold (.0245); p59 ...Nb4 remains unstable. Reference p61 ...Na6 loses stable support.

Candidate p70 Kc1 versus Rxg5+ retains .462 support. The legal Rxg5+ Kf8 Rg6 Ke7 Rxh6 line keeps the initiative. Reference p73 ...Qxa5 versus ...Nb4 now has .457 support, candidate p74 Rb3 versus hxg4 retains .493, and reference p75 ...Qa1+ versus ...Nc5 retains .5. This is a supported sequence of alternating opportunities; it is not an uninterrupted advantage or three independent causes. Discovery controls at f4, bxa5, Qh5 and h3 returned the played moves without blocking flags.

The actual g-pawn advance, rook transfer to h1, queen checks and capture of both Black rooks remain constructive play. The queen exchange permits g8=Q+ and the eventual mate on b8. The successful finish coexists with the unresolved middle attack.

**Game 1, candidate win.** Reference p22 Qe3 remains unstable; p24 Ne2 versus f4 retains .4775 support. Black's actual central exchanges, queenside rook activity and knight on f4 remain observations, not proof that every later move is forced.

Candidate p75 ...Rca8 instead of ...Qd8 and reference p76 Bc4 instead of Qxd6 both retain .5 support. The matched Qxd6 Nd3 Kc2 Nxf2 Qe6+ line gives White a concrete defensive resource. Actual Bc4 returns the strong finite opportunity to Black. The later h-pawn promotion and ...Rxb3+/...Qxh1 sequence are successful native tactics. Black eventually exchanges queens, promotes two further pawns and mates. The unselected late audit uncertainty remains in the original ledger; no flawless-conversion claim is made.

## Pair 08

**Game 0, threefold draw.** Selected quiet controls contain no supported significant event. The legal queen exchange and rook/minor-piece activity remain useful contrary evidence, not proof of perfect play. Native Black claims threefold after p106, intended ...Ke6 unplayed, version 92→93.

**Game 1, threefold draw.** Reference p50 Bxc7 versus Kc3 loses stable support because the Kc3 range is unstable. The played move remains near .5. The p46 cxb3 discovery returns the played move but has unstable unrestricted reports, so this control is also unresolved. Actual bishop captures and the opposite-colored-bishop ending remain the native evidence. White claims threefold after p75, intended Kf2 unplayed, version 61→62.

Neither draw itself is a weakness. No supported significant comparison is the immediately terminated final move in these three drawn games, so none is separately counted as a claim-policy-limited event. The policy still limits any conversion interpretation of the earlier finite opportunities.

## Unaided starts

**Game 0, candidate win.** Reference p9 ...Bd6 retains .128 support; candidate p10 Nxf6+ versus O-O now has .160 support. Reference p27 ...Kh8 becomes significant at .0565, sensitive to .06. Reference p31 ...c5 remains unstable. Candidate p36 bxc5 versus Nh4 now has .4065 support, but reference p37 ...Bxc5 remains bound-conflicted. The legal Nh4 f5 Qg6+ Kh8 Nxf5 line shows an attacking alternative; the uncertainty still blocks a fully confirmed renewed-opportunity account. The ...gxh6 discovery remains a stable same-choice control.

The actual later line differs from earlier unaided games: White attacks the queen with f4/Rxf4, exchanges queens through Rxf6/Rxg4, invades the seventh rank with both rooks, and promotes the h- and then g-pawns. This is successful native conversion. The repeated early trajectory remains correlated evidence and is not counted as a fresh independent opening sample.

**Game 1, candidate win.** Candidate p1 ...Nc6 and p13 ...O-O remain unresolved at 20M. Candidate p3 ...e6 (.0665), p5 ...Bb4 (.0585), and reference p2 Nc3 (.0655) have support; reference p6 Be2 remains unresolved. Reference p14 O-O becomes subthreshold (.04 observed to floating-point precision), so its earlier significant label is not retained.

Reference p26 Bc5 (.1285), candidate p29 ...exf3 (.087), and reference p32 Qxe2 (.1585) have support. Same-move discoveries at p27 ...dxe4 and p33 ...Nxe7 remain unstable. Candidate p35 ...c6 straddles .05. Reference p38 c5 versus Qxc5 retains .4025 support. Candidate p41 ...Nfd5 versus ...Ned5 now has .079 support while the played position remains strongly favorable. Reference p42 Qe1 is now a stable reverse/subthreshold comparison with both tested moves below .10; its former threshold concern is not retained as a significant event.

Actual Black coordinates rooks and knights, wins a rook with ...Nxd3, and uses checks plus the bishop attack to force the queen exchange before mate. The successful tactics do not clear the unstable recapture controls or establish a technical mechanism.

## Closure

All ten discoveries and required matched comparisons are covered. The empty follow-up selection is final for this scope. Thirty-five selected roots remain unresolved, including the three threshold-straddlers and retained discovery uncertainty. No repeat search or recursive PV chain is requested. The coordinator can close this review while preserving the native outcomes, linked uncertainty and claim-policy limits. No final engineering priorities or production changes are proposed.
