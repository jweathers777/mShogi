# MSH-161 (Competitive Gameplay Diagnosis): Batch 03 follow-up, pairs 00–04

All six selected new alternatives have completed matched 20M review. The six existing played receipts were reused. No engine, retrieval, cloud job, policy change or further query was run by this reviewer. The pre-follow-up ledger remains unchanged in `batch-03-adjudication/dispositions20m-00-04.json.gz`.

Each new artifact and reused played artifact passed its SHA-256 binding. Reused success receipts also matched their bound hashes. Each comparison has identical full-history decision data and 20M requested nodes. All twelve selected PV instances were replayed legally. Exact source, plan, completion, verification and original-ledger hashes are in `review-00-04.json`.

## Six exact dispositions

Plies are zero-based. Margins are minimum alternative E minus maximum played E across the observed last two depths. They are finite sensitivity measures, not statistical bounds or exact values.

| Pair/game/ply | Played / new alternative | Margin range | Disposition |
| --- | --- | --- | --- |
| 01 / 1 / 33 | ...Qc7 / ...Qd8 | 0.189–0.190 | Supported finite serious event |
| 01 / 1 / 67 | ...Bf8 / ...Kh7 | 0.497–0.497 | Supported finite major event |
| 01 / 1 / 69 | ...Be7 / ...Kh7 | 0.1015–0.137 | Unresolved: reused played report has depth instability and a final bound conflict |
| 02 / 0 / 18 | Nxf6+ / Nf3 | 0.1445–0.193 | Unresolved: played bound conflict and alternative depth instability |
| 02 / 0 / 21 | ...a5 / ...b5 | 0.0825–0.132 | Unresolved: alternative depth instability and final bound conflict |
| 02 / 0 / 33 | ...Qd6 / ...Rb6 | 0.5585–0.594 | Unresolved: reused played depth instability and final bound conflict |

The first two comparisons support their proposed alternatives. They do not add new event roots: stronger stable alternatives already supported those roots. At pair 01 game 1 ply 33, the original ...Qc6 comparison retains the larger supported margin. At ply 67, the original ...Ba7 comparison also retains the larger supported margin. At pair 02 game 0 ply 21, ...Qd5 still supplies stable significant support even though the new ...b5 comparison is unresolved. Keep both conclusions instead of letting one erase the other.

The four unresolved new comparisons remain unresolved regardless of their large apparent margins. Their query obligations are closed; this review does not request repeated searches to obtain a preferred conclusion. It does not create another chain of PV-based claims.

## Legal continuations and limits

The new ...Qd8 line at pair 01 game 1 ply 33 starts ...Qd8 Rfb1 Rf7 Rb3 Rc8 Rab1. The ...Kh7 line at ply 67 starts ...Kh7 Qd1 Ba7 c3 Qa4. Both are legal explanatory continuations with stable matched support.

The unresolved ...Rb6 line at pair 02 game 0 ply 33 starts ...Rb6 Qd3 b4 axb4 Bxb4 Nf3. It remains an illustrative legal line, not confirmed punishment of the preceding f4. Likewise, the legal Nf3 line at ply 18 and ...b5 line at ply 21 do not clear their report conflicts. Full selected lines and retained flags are in the JSON.

Earlier unresolved and unselected decisions remain unchanged. No first-error, largest-error, forced-line, or complete favorable-episode claim follows from these six comparisons. Original AutoClaim continues to limit conversion observation in pair 04 game 1. Its native draw is unchanged. This follow-up makes no policy or engineering recommendation.

## Final normalized selected-root ledger

`batch-03-adjudication/final-dispositions-00-04.json` contains one normalized row for each of the 167 selected roots, plus all original and new comparison details. Each row includes candidate identity, played and selected alternative moves, status, conservative margin, severity, secondary flag and exact sources. A stable supported alternative can establish a finite event while another alternative at the same root remains unresolved; both records remain available.

| Final status | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| `supported_finite` | 33 | 32 | 65 |
| `supported_subthreshold` | 21 | 11 | 32 |
| `same_choice_control` | 24 | 2 | 26 |
| `unresolved` | 26 | 18 | 44 |
| `claim_policy_limited` | 0 | 0 | 0 |

Three of the 65 finite events are already-lost secondary comparisons: pair 00 game 1 ply 31, pair 01 game 1 ply 178, and pair 03 game 1 ply 46. They are **not fresh primary causes**. Thus primary finite event roots remain 62: 32 candidate and 30 opponent. Fourteen total roots carry `secondary: true`, including subthreshold and same-choice controls. This is separate from the comparison threshold classification.

No decision comparison was relabeled `claim_policy_limited` merely because its game drew. The original automatic-claim limit is retained as explicit metadata on the affected game's rows. Native outcomes remain five candidate wins, four losses and one draw. These selected-root counts are not all-game error rates, independent episode counts or final study priorities.
