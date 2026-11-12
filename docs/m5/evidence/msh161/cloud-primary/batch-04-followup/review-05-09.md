# MSH-161 (Competitive Gameplay Diagnosis): Batch 04 follow-up, pairs 05–08 and unaided

## Coverage and checks

All five requested new alternatives now have forced 20M queries against their exact hash-bound reused played queries. The played queries were not run again. The ten receipts match original native full histories and the fixed reference identity. Local artifact verification passed. Selected, last-two-depth and final-report continuations passed legal replay, covering 1,227 PV plies. This review used completed evidence only; it started no search, retrieval or cloud job.

`review-05-09.json` records five separate new comparisons, both query bindings, the reused played success/artifact hashes, native histories, legal lines, and all retained flags. The original compressed pre-follow-up ledger is unchanged and hash-linked. `batch-04-adjudication/final-dispositions-05-09.json` applies only these five root overlays to the 285 selected rows. It preserves earlier/linked uncertainty and keeps the claim-policy-limited root separate.

## Results

| Root | Matched comparison | Observed ranges: played / alternative | Margin | Disposition |
|---|---|---|---|---|
| 05 / game 1 / p103 | ...Kc6 / ...Kd8 | [.9635, .9665] / [.932, .932] | -.0345 | supported subthreshold/reverse |
| 05 / game 1 / p108 | Kg1 / a5+ | [0, 0] / [.076, .076] | .076 | unresolved: conflicting final bounds |
| 06 / game 0 / p25 | ...O-O / ...c5 | [.0155, .0155] / [.0155, .0155] | 0 | supported subthreshold |
| unaided / game 0 / p23 | ...Nd4 / ...Ne7 | [.01, .01] / [.0575, .0585] | .0475 | supported subthreshold; .04 sensitivity |
| unaided / game 1 / p41 | ...Be6 / ...Bg6 | [.7315, .7775] / [.897, .918] | .1195 | unresolved: unstable reports and conflicting bound |

E is a finite reference self-play proxy, not an opponent win probability. Observed ranges are report sensitivity ranges, not confidence bounds. No new supported significant event is added. Prior unrestricted discovery flags remain recorded as history; the new classifications use the matched forced pair and retain its current uncertainty.

**Pair 05, game 1: king routes under checks.** The p103 comparison does not support an error in ...Kc6. The played range is higher than the new ...Kd8 range. Legal lines include `Kc6 Qc5+ Kb7 Qd5+ Ka6` and `Kd8 Qa5+ Kc8 Qc5+ Kb7`. These are checking-route alternatives, not proof of a forced escape. At p108, a5+ remains an unconfirmed defensive resource because final bounds conflict with the selected reports. The legal line `a5+ Ka6 Qc4+ Kxa5 Qc5+` keeps checking while giving up the pawn. Earlier uncertainty at p95, p96 and p99 remains; the later native threefold claim and draw remain unchanged. No missed-conversion verdict follows from the final draw.

**Pair 06, game 0: central counterplay before the attack.** At p25, both ...O-O and ...c5 have equally unfavorable finite ranges for the reference player. The earlier discovery does not establish that castling caused the disadvantage. The legal ...c5 line begins `c5 Nd6+ Bxd6 Rxd6 c4 c3 Bc6`; the played line begins `O-O Bd3 Rfd8 Nf6+ Bxf6 exf6`. This is secondary context under the operational E ≤ .10 marker, not a new primary event. The already reviewed candidate Be2/Bd3 comparison at p26 retains its own evidence. Native White eventually wins; no inference of perfect attacking play is added.

**Unaided game 0: knight placement in an unfavorable center.** At p23, ...Ne7 has a .0475 observed margin over ...Nd4. That is below the frozen .05 significance threshold, though it passes .04 sensitivity. Both alternatives remain below .10, so this is secondary context. The legal lines `Nd4 Ne4 Nxf3+ Qxf3 Be7 d6` and `Ne7 b6 f5 Nb5 Nxd5 Bg5` show different central resistance. They do not settle the unstable earlier p21 or later p24 comparisons. The actual candidate win and its later constructive rook/knight play remain unchanged.

**Unaided game 1: bishop placement and opportunity response.** The p41 ...Be6/...Bg6 comparison remains unresolved. The played reports are unstable and the alternative has a conflicting bound. The legal ...Bg6 line begins `Bg6 Qb3 Kh7 Rde1 Rae8 Qc3`, while the played line begins `Be6 c5 Ra2 g5 Nd5`. These show a concrete choice in piece placement and central response, but do not prove that ...Be6 lost a winning opportunity. The earlier reference p40 g4 comparison and later supported tactical events keep their own status. No new child-root superiority claim is made from either PV.

## Final overlay and closure

The 285 selected normalized rows now contain:

- 70 supported finite comparisons.
- 43 supported subthreshold comparisons.
- 113 same-choice controls.
- 58 unresolved comparisons.
- 1 claim-policy-limited comparison: pair 07/game 0/p66, followed by a native automatic threefold claim rather than the reference continuation.

There are seven secondary-context tags under the documented conservative rule; the two added tags are p25 and p23 above. They are subthreshold rows. These tags do not prove forced loss or make the other rows independent causes.

The qualitative examples remain gameplay observations: king routes under checks, central counterplay, knight placement, bishop placement, and response to an opponent opportunity. They are not technical mechanisms or final engineering priorities. The two native dead-position draws remain separate from automatic threefold claims; a draw alone remains no evidence of weakness.

**Alternative coverage is closed for this review scope.** All five distinct discoveries have matched singleton evidence. The two unstable comparisons stay unresolved. No new query, repeated unstable search or PV chain is requested. The coordinator can close the batch review while preserving the recorded uncertainty and native outcomes.
