# MSH-161 (Competitive Gameplay Diagnosis): Batch 04 20M review, pairs 05–08 and unaided

## Scope and disposition

The 285 frozen roots in this scope have 460 completed query receipts. Offline verification matched each query to its original native full history and replayed 51,061 PV plies across selected, last-two-depth and final reports. No new search, retrieval or cloud job ran. Native results remain 3W4D3L: prepared 1W4D3L and unaided 2W0D0L. Two automatic threefold draws remain separate from two native dead-position draws.

The normalized ledger `dispositions20m-05-09.json` contains all 285 rows, candidate identity, source/plan/artifact bindings, legal continuations, severity, threshold sensitivity, and retained earlier/linked uncertainty. It records **70 supported finite comparisons, 40 subthreshold comparisons, 113 same-choice controls, 61 unresolved roots and 1 claim-policy-limited root**. The supported comparisons divide into 34 candidate and 36 reference choices: 48 major, 9 serious and 13 significant margins. Supported sensitivity counts at .04/.05/.06 are 74/70/64, excluding the claim-policy-limited root.

These are finite comparisons, not independent causes or population frequencies. E remains a reference self-play proxy, not an opponent win probability. The margin uses observed ranges, not confidence bounds. Threshold-straddling and pending discovery comparisons normalize to unresolved. Unselected earlier uncertainty and screening-only evidence remain in the original ledger. No absolute first/largest error is claimed.

Five subthreshold rows retain a secondary-context tag because both tested moves have stable observed upper E ≤ .10: pair 05/game 0/p38; pair 05/game 1/p26; pair 06/game 1/p52 and p59; pair 08/game 1/p29. This operational tag does not prove forced loss. It does not imply that all other rows are independent primary causes.

## Discovery coverage and bounded follow-up

All 25 unrestricted discovery results were inspected. Five chose a distinct alternative and need one forced 20M singleton each, against their existing hash-bound played result:

| Pair / game / ply | Played | New alternative |
|---|---|---|
| 05 / 1 / 103 | ...Kc6 | ...Kd8 (`c7d8`) |
| 05 / 1 / 108 | Kg1 | a5+ (`a4a5`) |
| 06 / 0 / 25 | ...O-O | ...c5 (`c6c5`) |
| unaided / 0 / 23 | ...Nd4 | ...Ne7 (`c6e7`) |
| unaided / 1 / 41 | ...Be6 | ...Bg6 (`f5g6`) |

`followup-selection-05-09.json` contains exactly these five roots, with no duplicate request and an exact reused played job for each. Discovery reports at 05/game 1/p108 and unaided/game 0/p23 retain a conflicting bound and instability respectively. A singleton closes equal-work alternative coverage; it does not promise a stable result. No repeat is requested merely to clear uncertainty.

The other 20 discoveries returned the played move. Forced and unrestricted query flags remain visible. Nine of these roots remain unresolved: 05/game 1/p96; 06/game 0/p62; 06/game 1/p56 and p57; 07/game 0/p32; 08/game 1/p18; unaided/game 0/p25 and p41; unaided/game 1/p57. The other eleven are same-choice controls. In particular, stable forced played results at 06/game 1/p56 and 07/game 0/p32 do not clear the blocking flags on their unrestricted discoveries.

Linked PV replies were checked. Untested names include Bc4 at 05/game 1/p22; ...Rfg2+ at 05/game 1/p89; ...Bc6 at 06/game 0/p29; Qe2 at 07/game 0/p30; a6 at 07/game 0/p32; Ng5 at unaided/game 0/p16; Rac1 at unaided/game 0/p50; and ...d5 at unaided/game 1/p47. No new superiority claim is made for these reply names. The reported margins compare only the matched root alternatives. Existing uncertainty remains, and same-choice controls do not prove unique best play. The linked ...Bg6 at unaided/game 1/p41 is already one of the five discovery follow-ups. No further PV chain is opened.

## Pair 05

**Game 0, candidate loss.** The opening cxd5/c5 comparison remains significant (.0695). White p32 a3 versus Bf2 now has stable major support (.497). The Bf2 line gives the bishop an exit while active rooks permit exchanges. Reference p35 ...Rd7 versus ...b6 (.499) creates a supported renewed chance; candidate p36 g3 versus Bf2 (.497) does not preserve it. Reference p43 ...Ra8 versus ...g6 now has .4815 support, so another near-equal finite opportunity exists later in the same episode.

Candidate p46 fxe6 is now subthreshold (.0405 to Bb5); its earlier .196 label is not retained. Reference p47 ...Bc5+ remains unstable. Candidate p48 Rxc5 versus Kf1 now has .111 support: Rxc5 Rd1+ Kf2 bxc5 gives a concrete forcing exchange, while Kf1 Rd4 Rxd4 Bxd4 Bc4 keeps more resistance. The actual reference rook activity and eventual mate remain unchanged. The unstable p47 boundary limits the detailed transition account.

**Game 1, threefold draw.** Support now covers reference p18 f4 (.3135 to Nc3), candidate p21 ...Be6 (.294 to ...Bc6), and reference p22 Nxd5 (.2765 to Ne4). Candidate p25 ...g6 becomes subthreshold at .0495. Candidate p27 ...Re8 (.4085 to ...Bd6), reference p28 Qd2 (.407 to Bf2), candidate p29 ...Qg4 (.079 to ...Bd6), and p31 ...Bd6 (.134 to ...c6) show repeated changes in the attack. The p29 comparison retains a strong played position; it is not a complete loss of the advantage.

Reference p32 f5 (.307 to Qa5), candidate p33 ...c6 (.43 to ...g5), and candidate p37 ...Qxf6 (.4975 to ...Bxf4) are supported. Reference p34 f6 remains bound-conflicted. The matched ...Bxf4 Rxf4 Qg5 line preserves the queen and attack; the actual queen exchange changes the material. Reference p48 Qd3 (.479 to Qg3+) and candidate p75 ...b6 (.4995 to ...Rfg2+) remain supported. The latter permits actual Rxb6+ axb6 Qxb6+ and checking routes.

Reference p88 Qxg5 now has .5 support against Qe5+. Candidate p89 ...Rhg2+ remains unresolved, with a reverse comparison; do not discard it. Candidate p95 ...Rxb2 loses stable support. Reference p100 Qa8+ versus Qc6+ now has .4015 support, but p99 ...Kc8 and p96 Qe5+ remain unresolved. New discoveries at p103 and p108 must be matched before closing this checking episode. The late p148 Qf8+ and p149 ...Kc4 comparisons remain bound-conflicted at 20M, so their former finite labels are not retained.

Native White claims threefold after p241, intended Qf3+ unplayed, version 227→228. The result remains a draw under the fixed claim policy. It does not establish that the finite opportunities would convert under another policy.

## Pair 06

**Game 0, candidate win.** Reference p19 ...Ng4 remains just significant (.0505), sensitive to .06. Reference p23 ...Bd7 now has .404 support. The p25 ...c5 discovery is pending and is an earlier boundary before candidate p26 Be2 versus Bd3, now supported at .445. The Bd3 Rfd8 h4 Be8 Nd2 line develops and preserves the attack. Candidate p28 h4 (.104 to Nd6), reference p31 ...f6 (.5 to ...Bc6), and candidate p32 exf6 (.134 to h6) remain supported.

Candidate p34 h6 has an unstable reverse comparison, and p36 Kb1 plus reference p37 ...b5 remain unresolved. Candidate p38 c4 versus Rhe1 now has .4725 support: Rhe1 Bc8 Rxd8 Rxd8 a3 keeps the active pieces and pressure. Reference p39 ...f5 versus ...e5 remains supported at .5065. Candidate p40 Nc3 and reference p41 ...Bd6 remain unstable; no settled loss/recovery claim is assigned across them. Candidate p60 Bc4 remains bound-conflicted and p62 Qd6 is unresolved despite same-move discovery.

The native successful finish remains contrary evidence: White restrains the f-pawn, exchanges queens, advances and promotes the a-pawn, then wins the rook and mates immediately after Black's h-pawn promotes. These actual actions do not erase earlier unstable choices.

**Game 1, candidate loss.** Reference p30 Qc3 versus f4 now has .1025 support; candidate p31 ...Rab8 loses stable support. Reference p34 Qb3 versus Qd2 remains major (.4855). Candidate p55 ...Bxd4 versus ...Qa7 still has a large observed margin but unstable/conflicting evidence; it remains unresolved. The p56 and p57 recaptures also retain discovery uncertainty. Thus the queen-versus-pieces transition cannot be given a settled single cause.

Candidate p59 ...Bc5 versus ...Bf6 is now subthreshold (.0325), with both moves in a very unfavorable range. Reference p64 Qd8+ versus Bb5 now has .4985 support: the matched Bb5 Bf8 Bxc6 Bxc6 line differs from the actual Qd8+ Bf8 Be2 continuation. Candidate p77 ...f6 versus ...Rc7+ retains .5 support. These later defensive resources are linked to the unresolved earlier material transition. Native reference promotion and mate remain the observed result.

## Pair 07

**Game 0, threefold draw.** Reference p29 ...Kh8 remains supported at .067; candidate p30 Ra3 becomes subthreshold (.036). Reference p31 ...Ng4 remains major (.3955), but p33 ...Nxf2 loses stable support. The Bxh7 discovery at p32 returns the played move with a conflicting bound, so the sacrifice remains an unresolved control rather than proof of perfect attack.

Candidate p50 cxb4 is now bound-conflicted. Candidate p52 Rb1 versus Re2 (.4995), reference p57 ...Rc1+ versus ...Qxd4 (.5), candidate p58 Kg2 versus Rxc1 (.5), and reference p59 ...Qxa3 versus ...Qxd4 (.5) remain supported. The legal Rxc1 Qxc1+ Kg2 line supplies a concrete defense within this one tactical episode.

At p66 Ng5 versus Qg7, the finite .5005 comparison is **claim-policy-limited**. Native Black immediately claims threefold, intended ...Kf6 unplayed, version 52→53. The reference ...Qa2+ line was not played. This row is excluded from the 70 supported-event count and cannot be called an actual loss.

**Game 1, dead-position draw.** Candidate p23 ...h5 (.0555), reference p24 Nxd7 (.053), reference p30 Qg4 (.0505), and candidate p33 ...Kf8 (.072) have support; the first three are sensitive to .06. Reference p46 Re2 is now bound-conflicted, while candidate p47 ...Qxa2 instead of ...Nc6 retains .4985 support. The tested knight move addresses central coordination before taking the pawn; the preceding reference decision remains uncertain.

Reference p80 Ba4 becomes subthreshold (.0485). Reference p98 g4 versus Rd6 now has .4875 support. The Rd6 Kf7 Rxe6 Kxe6 Re1+ line gives a concrete simplifying defense; played g4 permits ...f4 and restriction. Candidate p113 ...Kd5 and reference p116 Rd8+ remain bound-conflicted. Candidate p137 ...Ne5 versus ...Ne6 retains .5 support: ...Ne6 keeps the knight structure and g-pawn, while actual ...Ne5 permits Rxg5.

The actual Rxf2 Nxf2, g7 Nxg7 Kxf2 sequence creates a native dead position. That material result is contrary evidence to a blanket inability to defend, not an automatic repetition termination or proof that all earlier choices were sound.

## Pair 08

**Game 0, dead-position draw.** The selected quiet controls show no supported significant event. This does not prove perfect play. The original legal exchanges and final Kxg3 remove the last pawn and create the native dead position. Keep this quiet game as contrary evidence alongside the tactical games, without inventing a weakness from the draw.

**Game 1, candidate loss.** Candidate p17 ...Nc6 loses stable support; p21 ...b5 versus ...Qxc4 remains supported at .4135. Reference p56 Rxc3 versus Rb1 now has .356 support, with the Rb1 h6 Rb8+ line preserving rook activity rather than entering the knight ending. Reference p60 Ng5 versus a4 remains supported at .1005. These later reference choices prevent treating the early Black disadvantage as an unbroken forced loss.

Candidate p101 ...Ne4+ versus ...Nxe6 retains .5 support after the actual promotion and Nxa1. The matched ...Nxe6 fxe6 Kxe6 removes White's advanced pawn; actual ...Ne4+ Kd4 Nf2 leaves central threats. White's later king activity, two promotions and mate are actual outcomes. The unresolved early p17/p18 choices still limit first-error attribution.

## Unaided starts

**Game 0, candidate win.** Reference p11 ...Bd6 and candidate p12 O-O now have conflicting bounds. Reference p13 ...Na5 remains supported (.203), candidate p14 Bd5 becomes unstable, and reference p15 ...Kh8 now has .3955 support. Candidate p16 b4 stays bound-conflicted; p18 Be3 versus Ba2 now has .1325 support. Reference p21 ...Nxd5 remains unstable. The distinct p23 ...Ne7 discovery must be matched before the central episode can close. Candidate p24 Bxd4 is now unstable, not a confirmed missed central opportunity.

Reference p27 ...c6 loses stable support. Candidate p28 dxc6 (.1255), reference p37 ...Qa5 (.255), candidate p38 c4 (.256), reference p39 ...Bg4 (.4935), candidate p40 h3 (.3615), candidate p42 g3 (.058), and reference p49 ...Rc8 (.4875) have support. Reference p35 ...a4 straddles .05, with observed margin .0445, and remains unresolved. Reference p41 ...Bh5 retains discovery instability. The legal Ne5 c5 Nc4 line at p38 explains a central alternative; it is not a unique-best claim.

The actual rook/knight attack, capture of the rook on a8, queen-winning exchange and c-pawn promotion remain successful continuations. Their success does not clear the earlier sacrifice and central uncertainties.

**Game 1, candidate win.** Candidate p1 ...Nc6 and p13 ...O-O are unresolved. Candidate p3 ...e6 (.0665), p5 ...Bb4 (.0585), and reference p2 Nc3 (.0655) have support, while reference p6 Be2 remains unresolved. These repeated starts remain correlated opening observations.

Reference p40 g4 versus Be3 now has .4395 support; the linked new ...Bg6 discovery at candidate p41 awaits its singleton. Reference p46 Qc7 (.478), candidate p47 ...Rfc8 (.3685 to ...Bxg4), candidate p53 ...Ra8 (.2055), candidate p55 ...Ng6 (.168), and reference p56 Be3 (.2815) have support. Candidate p57 ...Rxc2 retains discovery uncertainty; reference p58 f4 has an unstable reverse comparison. Candidate p59 ...Nh4 and reference p60 f5 remain unresolved. Reference p62 b6 is now a stable reverse/subthreshold comparison, so its former adverse interpretation is not retained.

Reference p64 Rb1 versus Rd2 remains supported at .521. Actual ...Nf3, ...exf3 and rook checks are constructive continuations; the matched Rd2 Rxd2 Bxd2 Qf6 line keeps more resistance. The later queen exchange, passed d-pawn and rook fork lead to native mate. Earlier uncertainty still prevents a complete account of the attack as one uninterrupted advantage.

## Closure status

Five distinct discovered alternatives need the frozen singleton follow-up. All other completed results retain their flags without repeat searches. Follow-up completion can close alternative coverage at those roots; it will not clear unselected uncertainty, prove a technical cause, or justify final engineering priorities. No additional PV chain or production change is proposed.
