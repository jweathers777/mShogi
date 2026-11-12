# MSH-161 (Competitive Gameplay Diagnosis): Batch 03 follow-up, pair 06 game 0 p22

The new forced 20M **g3 (`g2g3`)** query closes the alternative-coverage gap against played **hxg5 (`h4g5`)**. The played query was reused through its recorded success and artifact hashes; it was not run again. Both queries match the original native history, fixed reference identity and 20M work. Their selected, last-two-depth and final-report continuations passed legal replay (214 PV plies). This review used completed local evidence only.

The comparison remains **unresolved**. Played hxg5 has observed E range [.236, .287]; g3 has [.4255, .4525]. The observed minimum-alternative minus maximum-played margin is .1385 and the selected-score difference is .1655. However, both last-two-depth spreads exceed .025 (.051 and .027). Thus neither the .05 label nor a severity label has stable support. Stale-final-report flags also remain visible. These are finite reference estimates, not measured win probabilities.

The legal g3 line begins `g3 a5 hxg5 hxg5 gxf4 g4 Ne5 Ra6 Be3 f5`. It contests the pawn front before the exchange. The played line begins `hxg5 hxg5 g3 g4 Ne5 f3 d5 a5`, leaving Black the further advanced f-pawn. These lines explain the defensive-choice question; they do not prove a mechanism or establish an absolute first error. The later p24 Kh1 versus g3 comparison remains separately supported at its own root. This unresolved p22 comparison limits a complete account of the earlier defense.

No additional query is needed to complete discovery coverage. Do not repeat searches to obtain a stable label. The actual game remains a candidate loss by mate. No new PV-claim chain or final engineering priority is proposed.

`batch-03-adjudication/final-dispositions-05-09.json` preserves the archived pre-follow-up ledger by reference and hash, then applies this one-root overlay. Its 230 normalized selected rows contain:

- 53 supported finite comparisons.
- 41 supported subthreshold comparisons.
- 75 same-choice controls.
- 60 unresolved comparisons, including the earlier threshold-straddling root and this follow-up.
- 1 claim-policy-limited comparison: pair 07/game 0/p82 Rf8. The legal reference capture was not played; native Black claimed threefold instead.

No row is tagged secondary under the conservative numeric rule used here: a supported finite comparison with alternative upper E ≤ .10. This does not establish that all supported rows are independent primary causes. Their linked episode limits remain in the review and original ledger. Pair 06/game 1/p109 retains an explicit note about the later threefold claim. Draws alone remain no evidence of weakness, and the four native draws are unchanged.
