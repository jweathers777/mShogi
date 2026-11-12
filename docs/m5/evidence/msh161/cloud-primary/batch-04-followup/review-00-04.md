# MSH-161 (Competitive Gameplay Diagnosis): Batch 04 follow-up, pairs 00–04

All five selected linked-alternative obligations are complete. This review used the new singleton artifacts, hash-bound reused played 20M evidence, exact native-history decision data and the preserved pre-follow-up ledger. No engine, retrieval, cloud job, repeated unstable query or new PV investigation was run.

The five new artifact hashes, reused played artifact hashes and reused success hashes match their bindings. Each comparison has identical full-history decision data and 20M requested nodes. All ten selected PV instances were replayed legally. `review-00-04.json` records exact query, plan, completion, verification and original-ledger source hashes. The archived `batch-04-adjudication/dispositions20m-00-04.json.gz` remains unchanged.

## Five exact dispositions

Plies are zero-based. Margin ranges use alternative minimum minus played maximum, and alternative maximum minus played minimum, across the last two distinct depths. These are finite sensitivity measures, not statistical intervals or exact values.

| Pair/game/ply | Played / new alternative | Margin range | Disposition |
| --- | --- | --- | --- |
| 02 / 1 / 26 | Bb3 / Nh4 | −0.1420 to −0.1135 | Unresolved: reused played report remains unstable |
| 03 / 1 / 46 | Bb7 / Ba4 | 0.4345 to 0.4715 | Unresolved: reused played report remains unstable |
| 04 / 1 / 44 | Nh4 / Rc5 | 0.3825 to 0.4080 | Supported finite major event |
| 04 / 1 / 55 | ...g5 / ...Qg4 | 0.4655 to 0.4665 | Supported finite major event |
| 04 / 1 / 59 | ...f6 / ...Nf8 | 0.4895 to 0.4975 | Supported finite major event |

The negative Nh4 comparison is not evidence that Bb3 is a mistake. Its played uncertainty also prevents a firm reverse ranking. The Ba4 comparison has a large apparent loss but cannot clear the unstable played evidence. Both obligations are complete and both scientific conclusions remain unresolved. No repeat search is requested.

The three supported new alternatives concern roots that already had supported finite events. They therefore add no event roots. Rc5 strengthens the supported comparison at ply 44. ...Qg4 supplies another stable alternative to ...g5 at 55. ...Nf8 strengthens the comparison at 59 from a serious conservative margin against the earlier alternative to a major margin. All earlier comparisons remain in the final ledger. A new unresolved alternative does not erase existing support, and a new supported alternative does not retroactively clear another report's uncertainty.

## Legal lines and qualitative examples

The finite Rc5 line at pair 04 game 1 ply 44 starts Rc5 Nd7 Rxb5 Qc7 Kg1 Qa7. It supports a concrete rook-activity and material-response comparison against Nh4 in the actual history. The ...Qg4 line at 55 starts ...Qg4 f3 Qe6 Rc2 Rca8 f4. The ...Nf8 line at 59 starts ...Nf8 Rf2 Ne6 Qb2 Ra7 Kg1. These support board-level examples of active-piece placement and preservation of favorable play. They do not establish technical causes, forced continuations or final study priorities.

The unresolved Ba4 line starts Ba4 Rc7 Rd1 g5 Kg2 g4. It remains an illustrative legal defense, not confirmed recovery. The Nh4 line starts Nh4 b5 Bb3 Kh8 f3 Ng5. It does not justify a fresh error label or another sequence of PV-driven searches. Full selected lines and retained flags are in the JSON.

All three new supported comparisons are in pair 04 game 1. Treat them as related decisions in one game, not three independent games or universal weaknesses. The native game remains an original automatic threefold draw. Finite decisions before the claim can be supported while conversion observation remains limited. Do not infer weakness from the draw itself, replace the result with reference E, or start policy work.

## Final normalized ledger and closure

`batch-04-adjudication/final-dispositions-00-04.json` contains all 195 selected roots under `decisions`. It preserves source bindings, separate old/new comparisons, conservative margins, severity, candidate identity, secondary tags, threshold sensitivity and earlier uncertainty.

| Final status | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| `supported_finite` | 35 | 32 | 67 |
| `supported_subthreshold` | 21 | 14 | 35 |
| `same_choice_control` | 44 | 3 | 47 |
| `unresolved` | 25 | 21 | 46 |

All 67 supported finite events are outside the recorded secondary contexts. Nine other roots retain `secondary: true`; comparison threshold classification stays separate. There are no new finite event roots. Native results remain four candidate wins, four losses, one dead-position draw and one automatic threefold draw.

At thresholds 0.04 / 0.05 / 0.06, supported counts remain 69 / 67 / 64. At the primary threshold, 44 roots have report/discovery uncertainty and two more straddle significance. Those 46 roots remain unresolved. Unselected earlier evidence is unchanged. No unsupported first/largest error claim follows from closure.

The five query obligations are closed. No new query obligation remains for this half-batch review. The evidence is ready for coordinator integration and independent challenge, with all stated uncertainty retained. This is selected-root closure, not a claim that the whole game sample is fully understood.
