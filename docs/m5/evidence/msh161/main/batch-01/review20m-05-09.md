# Batch01 pairs05–08 and unaided:20M review overlay

**Continue the unchanged broad study with batch02.** This bounded review presents
no scientific blocker to its next batch. It does not rank final weaknesses or
propose engineering. Preserve the remaining reference and claim-policy uncertainty
instead of extending this batch until every judgment becomes favorable or exact.

The43 selected decisions contain84 query receipts. Every receipt hash, completed
execution condition, full original history and20M requested work was checked;
all84 selected full PVs replay legally. The frozen method admits a stale-final-
report flag alone, but retains instability or directional bound conflicts.
`adjudicated20m-05-09.json` supersedes5M only at these queried decisions and links
the unchanged earlier1,112-move disposition record. There are28 supported finite
deteriorations across both players,3 subthreshold comparisons,6 unresolved
comparisons,4 stable singleton controls, and2 claim-policy-dependent judgments.
These are selected-decision counts, not prevalence estimates or independent samples.

All scores below are mover-perspective Stockfish WDL expected-score proxies.
“Margin” means the minimum alternative estimate minus maximum played estimate
over the last two distinct-depth reports; it is not a statistical lower bound.
Significant/serious/major thresholds remain.05/.10/.20. A finite supported loss is
not a game-theoretic proof. First/largest indices in the machine overlay refer
only to the queried20M subset; earlier or larger unqueried/unresolved comparisons
prevent unsupported absolute labels.

## Updated game judgments

| Pair/game, candidate color | Supported candidate judgment | What remains or changed |
|---|---|---|
|05/0 White, win|p16 Bxf6 versus f4: .5545→.488, margin.0665; significant contextual exchange/central-play loss.|p18 Ndb5 becomes subthreshold(.001 loss); p30 f5 retains E1.0 and is **not** a supported preservation error. Opponent p17 gxf6 remains conflicting, so do not claim a demonstrated missed opportunity there. Opponent p27 Nb4 loses.4965 and still supports White's subsequent successful attack.|
|05/1 Black, win|No candidate significant loss is resolved among these probes.|p27 ...b6 versus ...e5 still has unstable played estimates [.606,.643], despite an apparent.3595 loss. Opponent p22 Qxd6 is now supported(.376); p30 Rd8+ remains unresolved because its alternative g3 varies [.2405,.361]. Actual Black conversion remains observed, but first/largest candidate error and exact restoration of advantage are unsettled.|
|06/0 White, loss|p20 Bd2 versus e4: .5145→.444, margin.066; p70 hxg5 versus d5:1.0→.514, margin.486; p82 Ke1 versus Be6: .989→0, margin.988.|p82 is the largest supported queried loss and a concrete late tactical/defensive collapse. Opponent p77 Bh2 supplies a supported major chance(.499 loss), but intervening p80 Bc5/p81 ...Qh4 retain5M uncertainty; do not attribute the entire restored advantage to one earlier move without that caveat.|
|06/1 Black, draw|p27 ...Ne5: .468→.001; p81 ...c4 versus ...b4:1.0→.5; discovery at p89 ...Kxh6 now establishes .479 with ...b4 versus0 after capture.|The p89 discovery supplies the previously missing **matched alternative**, not just an isolated bad score. Opponent p42 Ne4, p78 Kh2 and p90 Kf4 have supported major losses, linking separate recovery opportunities. p97 ...Ba8 and p98 Bf3 require the native claim-policy qualification below.|
|07/0 White, loss|p32 b3 versus Bf4: .48→0; p108 Ra7 versus g4: .4445→0; p126 Kf5 versus Ra8+: .496→0, margin.4905.|p32 supports the original threatened-bishop/queen-tempo failure. The later events are distinct failed recovery chances, not independent causes of the original deterioration. Opponent p125 ...Kg8 loses.499, supporting the immediate p126 recovery attribution. Largest supported queried loss is p126, slightly larger than p32.|
|07/1 Black, win|No significant candidate loss among queried comparisons; p29 ...f6 loses only.0315.|Opponent p50 b4 now has supported.4145 damage. Candidate p51 ...Ra2 has a stable singleton E.8745 and the legal invasion line, supporting actual strong rook play without proving optimality. Earlier unqueried uncertainties remain; “no queried loss” does not mean flawless play.|
|08/0 White, win|p34 Qd6 versus Qf6: .996→.5075, margin.4885; supported major loss of attacking advantage.|Opponent p23 ...Bd5(.354 loss) and p35 ...Qxa2(.485 loss) now support the two opportunities. Candidate c4 and later Qf6 are stable positive singleton controls(E.91/.989); the final win includes both an attack-preservation failure and successful exploitation of renewed chances.|
|08/1 Black, loss|p31 ...Bf6 versus ...Rd7: .599→.525, margin.0715; p63 ...e4 versus ...Re8: .993→.5, margin.493.|The earlier Bf6 issue gains support, moving the earliest supported queried deterioration before the earlier5M Qc6 example. p71 ...Qxd1+ remains **unresolved** because of a final directional cp conflict, despite played E0 versus Qg3 E.5. Do not retain its previous5M “largest confirmed error” label. Opponent p34 Kh1 loses.4545 and supports the preceding candidate opportunity.|
|Unaided/0 White, win|p2 e4 versus d4: .5705→.5095, margin.0575; now just significant.|This remains a modest lost opportunity between plausible opening moves, not proof that e4 is unsound or that rarity matters. Opponent p15 ...Qe8 loses.4585; candidate p16 Bxf6 is a stable E1.0 successful attack control. Sound subsequent development/attack and actual conversion remain contrary evidence.|
|Unaided/1 Black, win|p3 ...e6 versus ...d5: .4995→.432, margin.067.|p1 ...Nc6 is now unstable [.4205,.4475] and its central loss estimate falls to.036; it cannot remain a confirmed first significant error. p55 ...Rc6 versus ...Rd2 remains unresolved because the alternative varies [.6845,.7375]. Opponent p56 g3 loses.471, supporting the later practical conversion opportunity.|

## Discovery and the native draw in06/1

At zero-based p89, discovery found **...b4**. Its independently cleared-hash
singleton20M query gives [.4745,.479], while played ...Kxh6 gives[0,0]. The legal
alternative line begins `...b4 axb4 Be6 Bf3 Bf5`; the played line begins
`...Kxh6 Bf3 Kg6 Kg4`. This now supports active pawn counterplay over the capture.
It does not retroactively validate the earlier singleton-only inference.

At p98, discovery found **Bf5**, but matched20M gives E1.0 for **both Bf5 and
played Bf3**. Consequently there is no Stockfish-supported comparative error in
Bf3. The actual native game still draws: after **50.Bf3 Bd5** (p98/p99), the
controller makes White's valid threefold claim with intended **Bf3–g4**. That
intended move is **not played**. The exact claim is in
`games/main-01-06/report.json.gz`, nested
`report_json.games[1].match_record_json.events[-1]`, request version85 and reply86.
The native conclusion is authoritative; all100 played plies and the subsequent
claim remain preserved.

This is a concrete distinction between Stockfish's winning continuations and the
study's preregistered unconditional auto-claim behavior. A winning player in
ordinary Chess can decline a claim; the controller here does not. Therefore p97
...Ba8's finite drawing-to-losing score change and p98 Bf3's finite winning score
must not be converted directly into actual-result failure/recovery labels without
native counterfactual claim-policy analysis. The overlay retains their SF
comparisons and explicitly withholds that attribution. It does not allege a rules
or search defect and does not propose changing the fixed policy mid-study.

## Remaining uncertainty and continuation

The six unresolved20M comparisons are05/0 p17,05/1 p27/p30,08/1 p71, and
unaided/1 p1/p55. They stay unresolved; no favorable-repeat campaign is proposed.
All unqueried5M significant/unstable/threshold cases remain in the per-game
remaining-dependency lists. The earlier category candidates for05/0 p18/p30
must not be counted as confirmed weaknesses after this overlay; contextual
successes and counterexamples remain essential.

The deeper evidence supports several behavioral questions—central breaks, active
defense, exchange/attack preservation—while also removing apparent errors and
retaining strong play. Continue batch02 under the same frozen conditions and
review completed batches. The planned broad coverage, not the appeal of these
individual positions, determines when final priorities can be ranked.
