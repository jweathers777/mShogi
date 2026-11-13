# Batch 07: independent 20M review, pairs 05–08 and unaided

The 362 selected roots have 65 supported finite comparisons (30 candidate, 35 reference), 68 supported subthreshold comparisons, 171 same-choice controls and 58 unresolved roots. One distinct discovered alternative still requires a singleton. Supported severity counts are 48 major, 5 serious and 12 significant. The .04/.05/.06 sensitivity counts are 67/65/62. These are finite report comparisons, not confidence bounds or opponent win probabilities.

The review checked 555 cached query artifacts against result receipts and original native decisions. Legal replay covered 60,953 PV plies across last-two-depth, selected and final reports. All 362 root identities match the frozen selection. The ledger retains exact plan and artifact hashes, job IDs, candidate/engine identity, legal UCI/SAN lines, blocking flags, underlying finite status, sensitivity, secondary tags and earlier uncertainty. No engine, cloud job, search or retrieval ran. The initial independent gameplay review remains the full native-history record.

## Discovery and followup

All 21 discoveries were inspected. Twenty return the played move. Eight are supported same-choice controls; twelve same-move discoveries remain unresolved after retaining unrestricted and played-query flags. A stable played singleton does not erase an unstable discovery. The remaining discovery is pair 08/game 1/p38: unrestricted Qxc5 (`c3c5`) differs from played Bxc5 (`e3c5`). Its apparent margin is -.004, but unrestricted output is not a matched singleton comparison. It remains unresolved pending the required new forced Qxc5 query.

`followup-selection-05-09.json` contains exactly that one alternative and the hash-bound played job `2f74840beab436c87ee9af0f51069b3889cdcc3c858038e790369b7a81d168b1`. Both choices are legal in the native position; the alternative has no existing forced query. No played rerun is requested. Legal PVs below illustrate original root alternatives only. No separate consequential claim is made about an off-trajectory reply, so no new linked-reply query is required.

## Pair 05

Game 0 remains a native threefold draw. Reference p59 ...g6 versus ...Nd7 (.2985), candidate p60 Qf2 versus Rc3 (.08), reference p61 ...Na4 versus ...Nd7 (.47), and candidate p62 g4 versus Kh2 (.4845) retain support. The legal Rc3 Nd7 Re3 line keeps rook activity; the Kh2 line postpones the pawn opening. Candidate p62 moves from an alternative E near .99 to played E .5; this is a finite loss of opportunity, not an observed forced loss.

Reference p77 ...Rc8, candidate p78 Qd5+, reference p79 ...Be6 and candidate p84 Nd7 are unresolved at 20M. Their large margins do not override instability or conflicting bounds. Reference p89 ...Nd1+ versus ...Re2+ and candidate p90 Kg3 versus Kxe1 both retain .5 support. Kxe1 Qe6+ Kxd1 is a legal resource. Actual play instead reaches repeated checks. The p118 terminal control is same-choice at zero margin. Black's intended ...Qf2+ is unplayed because the native claim ends the game. The earlier comparisons are separate from that terminal observation.

Game 1 remains a candidate win. Reference p26 gxf3 versus Qxf3 is supported at .4805. Candidate p27 ...Qg6+ versus ...Rd8 becomes supported at .082, but played E remains .873: this is reduced advantage, not a failed attack. Reference p30 f4 versus Bf4 is supported at .079 and is secondary because both upper E values are at most .10. Candidate p49 ...a5 no longer has a supported significant label. The observed forcing exchanges and two promotions remain contrary success evidence; they do not erase the p27 loss of finite value or p28 discovery uncertainty.

## Pair 06

Game 0 remains a candidate loss. Candidate p38 Bxh6 versus Nf1 retains .4715 support. Nf1 Qd8 Qd2 provides a legal alternative before the bishop sacrifice and rook captures. Reference p69 ...Rxe4 versus ...Qc3 retains a 1.0 margin. Candidate p72 Kf1 versus Rg4 becomes unresolved despite the same 1.0 margin, because a conflicting bound blocks support. The legal Rg4 Rxg4+ Qxg4 line remains illustrative. Thus the late opportunity is not a fully confirmed two-player reversal. The native promotion mate remains unchanged.

Game 1 remains a candidate win with several supported changes in opportunity. Candidate p19 ...h5 versus ...Qf6 (.4305), reference p30 Qb3 versus a4 (.0515), reference p38 Re2 versus Rf3 (.4715), candidate p39 ...Kg8 versus ...Nh6 (.414), and reference p40 a5 versus Qb1 (.4875) retain support. Candidate p23 ...O-O versus ...Nf8 (.06) is a supported secondary comparison: even the alternative has E .06. Candidate p35 ...Nb6 is now subthreshold. Reference p34 Re3 remains reverse and unresolved.

Candidate p45 ...Rb8 versus ...Nf7 (.0895) and reference p46 Rff2 versus Qd1 (.104) have support. Candidate p53 ...b6 versus ...h4 becomes supported at .3395. Reference p50 a6, p54 Qa4 and candidate p55 ...g4 remain unresolved. Reference p64 Nf5 becomes supported at .053, while candidate p67 ...Nfd7 becomes unresolved. Reference p72 Qd2 versus Rh2 retains .5005 support.

After the rook exchanges, candidate p89 ...Qxf5 versus ...Ne5, reference p90 Kg2 versus Bxf3, candidate p91 ...Kg7 versus ...Qg5, and reference p92 Qe3 versus Bxf3 all retain margins near .4995. The Bxf3 Qxf3+ line supplies a legal exchange resource. These four roots are one linked sequence of renewed chances. Black's later queen exchange, king activity and pawn promotion are actual successes, not proof that the whole game was continuously favorable.

## Pair 07

Game 0 remains a candidate win. Reference p17 ...Be7 versus ...Nb4 (.071), candidate p20 a3 versus Bd2 (.1785), reference p21 ...Nh6 versus ...Na5 (.4455), and reference p37 ...h5 versus ...Bg5 (.2645) are supported. Candidate p24 Na4 versus Bxh6 becomes unstable; p28 Bxh6 also remains unresolved. The legal Bxh6 gxh6 line does not justify a confirmed root claim where its evidence is unstable.

Candidate p78 Nd3 versus Nb3 (.4995) and reference p79 ...Re6 versus ...Re8 (.5) retain support. Nb3 Re8 Nxa5 illustrates the alternative knight route, while ...Re8 Ne1 fxe5+ leads to a different defensive exchange. Actual White recovers an opportunity after ...Re6, advances the e-pawn, and wins with rook and knight. Earlier uncertainty remains. Game 1 remains a candidate loss: candidate p17 ...f6 versus ...Nb4 retains .4725 support, while p25 ...Bc5 is subthreshold secondary context. White's native c-file invasion, exchange sequence and promotion remain the observed continuation, not a uniquely forced explanation of the loss.

## Pair 08

Game 0 remains a candidate win after a long rook ending. Reference p39 ...Rfd8 versus ...Qxc6 (.3875), candidate p40 Qxd7 versus Bd4 (.398), and reference p43 ...Rb8 versus ...d4 (.459) are supported. Earlier p33 ...Rac8 and p34 a3 remain unresolved. Candidate p54 Rc5 versus Ra8 becomes supported at .452; reference p55 ...f5 remains unstable. Candidate p56 Bf4 versus Ra5 (.48) and reference p57 ...Bd6 versus ...Bxf4 (.5) retain support. The bishop exchange and rook-ending entry must be interpreted as one linked episode.

Candidate p66 Kf2 versus Rc1 (.4955), reference p67 ...g6 versus ...Ra7 (.4915), candidate p80 h4 versus Rb7+ (.4995), and reference p81 ...Ra6 versus ...d5 (.499) become or remain supported. Reference p65 ...Rb7 is now subthreshold secondary context. Candidate p84 Rg7 remains unresolved despite .5. Reference p89 ...Kc4 versus ...Kc5, candidate p96 Rg8 versus Kxd5 and reference p127 ...Kd3 versus ...Rg3+ each retain .5 support. Legal Kxd5 Rg1 Rb8+ and ...Rg3+ Kh7 Rf3 illustrate capture and checking resources at their own roots.

Actual White eventually gives the rook for the d-pawn, promotes the h-pawn and wins queen versus rook. The long conversion is contrary success evidence. The unselected p179 audit remains unresolved, so no flawless-technique claim follows. Game 1 remains a candidate loss. Its p33 ...b6 comparison no longer has a supported significant label. Discovery p38 Qxc5 versus Bxc5 needs its singleton; no advantage is inferred from its unrestricted output. The native queen exchanges, rook-versus-knight ending and h-pawn promotion remain intact.

## Unaided games

Game 0 remains a candidate win. Reference p11 ...Bd6 and candidate p12 O-O become bound-conflicted. Reference p13 ...Kh8 versus ...Be7 (.4475), candidate p14 Be3 versus Ng5 (.303), reference p17 ...Na5 versus ...Be7 (.278), and candidate p30 Bxh6 versus Bg5 (.3905) are supported. Candidate p22 Nf3, reference p23 ...Nd4, candidate p40 exf5, p44 Rfe1, p46 Qd2 and p50 f3 remain unresolved. Reference p39 ...f5 has a reverse unstable comparison. These uncertainties prevent a settled account of the entire attack.

Reference p51 ...Re8 versus ...Rg8 retains .5 support. The actual Qxe8+ Bxe8 Rxe8+ exchange is legal and observed; it does not prove a win at every later root. Reference p61 ...Qxb6 remains unstable. White's later queen exchange, passed-pawn control and two promotions are observed successful play.

Game 1 remains a candidate win. Reference p2 Nc3 (.0655), candidate p3 ...e6 (.0665) and p5 ...Bb4 (.0585) are supported; the repeated opening remains correlated evidence. Reference p6 Be2 remains unresolved. Reference p26 Bc5 (.1285), candidate p29 ...exf3 (.087), reference p32 Qxe2 (.1585) and candidate p37 ...Rfe8 (.117) are supported. Candidate p35 ...c6 is threshold-straddling: margin .034 with a selected-score difference above .05. It remains unresolved. Reference p50 Rd1 is also unstable.

Reference p52 Re3 versus Rxh4 (.494), candidate p53 ...Nd5 versus ...Bd5 (.488), and reference p54 Re5 versus Rxe6 (.445) retain or gain support. Reference p58 Re4 remains unstable rather than becoming confirmed at its .0675 margin. Candidate p65 ...Nd2 (.4985), reference p72 Kh4 (.4985), candidate p73 ...Ngf3+ versus ...Ndf3+ (.4955), and reference p78 Qb7 versus Rg6+ (.5) are supported. The exact knight identity is important. Legal Rg6+ Kh7 Rh6+ illustrates a checking defense at p78; it is not a new claim about later PV positions. Black's actual rook sacrifice, knight fork, passed-pawn defense and promotion remain contrary tactical and ending successes.

## Limits and readiness

Nineteen rows have secondary tags: two supported significant comparisons and seventeen subthreshold comparisons. Earlier and linked uncertainty is retained in all rows. The sole native draw is a threefold claim after p118 in pair 05/game 0; there is no fifty-move or dead-position result in this panel. Its underlying finite status is same-choice at zero margin. No supported significant terminal comparison conflicts with native termination, so no row requires `claim_policy_limited` here. The native 6W1D3L result remains unchanged.

The review and normalized ledger are frozen. One new singleton is required before final closure; all other roots need no further query. No unstable query is repeated, no recursive PV investigation is requested, and no final engineering priorities are assigned.
