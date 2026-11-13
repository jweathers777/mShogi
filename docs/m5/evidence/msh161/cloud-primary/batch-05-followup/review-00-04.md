# MSH-161 (Competitive Gameplay Diagnosis): Batch 05 follow-up, pairs 00–04

All eleven selected singleton obligations are complete. This review used the new alternatives, exact reused played 20M evidence, native-history decision data and the archived pre-follow-up ledger. No engine, retrieval, cloud job, repeated unstable query or new PV investigation was run. The original ledger remains unchanged.

All new artifact, reused played artifact and reused success hashes match their bindings. Each comparison has identical full-history decision data and 20M requested nodes. All 22 selected PV instances were replayed legally. `review-00-04.json` retains exact query, plan, completion, verification and archived-ledger hashes.

## Eleven comparison dispositions

Plies are zero-based. The margin range is alternative minimum minus played maximum through alternative maximum minus played minimum over the last two distinct depths. These are finite sensitivity measures, not statistical confidence intervals or exact values.

| Pair/game/ply | Played / new alternative | Margin range | Disposition |
| --- | --- | --- | --- |
| 00 / 0 / 28 | Rd1 / Ba3 | 0.0090–0.0195 | Supported finite subthreshold |
| 00 / 1 / 26 | Rad1 / Rfd1 | 0.1855–0.1900 | Supported finite serious event |
| 01 / 0 / 60 | Rbd2 / Kh2 | 0.2150–0.4610 | Unresolved: alternative depth instability |
| 01 / 0 / 65 | ...a4 / ...Bf7 | 0.1725–0.1835 | Supported finite serious event |
| 01 / 1 / 36 | f4 / Kf1 | 0.4715–0.4745 | Supported finite major event |
| 01 / 1 / 84 | Be3 / Bc7 | 0.3860–0.3885 | Unresolved: alternative cp and expected-score bound conflicts |
| 03 / 0 / 21 | ...Na5 / ...Qc8 | 0.1640–0.1845 | Unresolved: reused played bound conflict |
| 04 / 1 / 34 | Bb7 / Na7 | 0.0515–0.0530 | Supported finite significant comparison; secondary context |
| 04 / 1 / 40 | Rad1 / Rfd1 | 0.0175–0.0545 | Unresolved: alternative depth instability; secondary context |
| 04 / 1 / 98 | Bc3 / Bb4 | 0.4675–0.4710 | Supported finite major event |
| 04 / 1 / 103 | ...Qc5+ / ...Qe8 | 0.4995–0.5000 | Unresolved: alternative cp-bound conflict |

The discovery alternative Ba3 at pair 00 game 0 ply 28 now has equal-work singleton evidence. The difference is below the significance thresholds; it is not an error claim or proof of unique best play. Pair 00 game 1 ply 26 already had supported significance against another move; Rfd1 strengthens its conservative severity to serious rather than adding a new event root.

Three new primary event roots gain support: opponent ...a4 at pair 01 game 0 ply 65, opponent f4 at pair 01 game 1 ply 36, and opponent Bc3 at pair 04 game 1 ply 98. The Bb7/Na7 comparison at pair 04 game 1 ply 34 gains finite significance but remains secondary under the low-value marker. It is not a fresh primary cause. The correct legal SAN is Na7, without capture; the earlier narrative's Nxa7 label was a notation error. The exact UCI request and history were unchanged.

Five new comparisons remain unresolved. Their apparent margins do not override report failures. In particular, the near-half-point ...Qe8 comparison remains unresolved because of its cp-bound conflict. No repeat search is requested. A completed query obligation does not imply a resolved scientific conclusion.

## Old support and new uncertainty

All old and new comparisons remain separate in the final ledger. The unresolved Kh2 comparison at pair 01 game 0 ply 60 does not erase the previously supported major comparison against Qf3. The unresolved Bc7 comparison at pair 01 game 1 ply 84 likewise does not erase existing supported major evidence against Kf2.

The new unresolved comparisons at pair 04 game 1 plies 40 and 103 change those root dispositions from supported subthreshold to unresolved; they do not become finite event counts. The prior subthreshold comparisons remain recorded. The discovery root at pair 00 game 0 ply 28 changes from unresolved to supported subthreshold. Earlier unselected and unresolved evidence remains intact. No first-error or largest-error claim is made.

## Legal explanatory lines and qualitative examples

The ...Bf7 line at pair 01 game 0 ply 65 starts ...Bf7 Rf2 Bh4 Rff1 Bc4. It supplies finite support for an active-piece defense during the mutual attack. The Kf1 line at pair 01 game 1 ply 36 starts Kf1 Rc6 Ba3 Rhc8 Bxd6+ Rxd6; this supports a concrete defensive alternative before the pawn advance. The Bb4 line at pair 04 game 1 ply 98 starts Bb4 Qg6 Qh4 Qf7 Kg1 Qf3. These are board-level examples of defense and piece coordination. They do not establish technical causes or final study priorities.

The Rfd1 line at pair 00 game 1 ply 26 starts Rfd1 h5 Bd2 Nc5 Rac1. Its supported comparison is a development and rook-placement example, not a defect inferred from the subsequent draw. The Na7 line at pair 04 game 1 ply 34 starts Na7 Rb8 f5 exf5 Bxb4; its limited finite chances remain secondary. All selected lines were legally replayed, but are not forced proofs. No new PV-based obligations are created from them.

## Final selected-root ledger

`batch-05-adjudication/final-dispositions-00-04.json` contains all 219 normalized rows under `decisions`, with separate comparisons, exact sources, conservative margins, severity, secondary tags, sensitivity and retained earlier uncertainty.

| Final root status | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| `supported_finite` | 29 | 25 | 54 |
| `supported_subthreshold` | 28 | 7 | 35 |
| `same_choice_control` | 69 | 10 | 79 |
| `unresolved` | 29 | 21 | 50 |
| `claim_policy_limited` | 1 | 0 | 1 |

One of the 54 finite event roots is secondary, leaving 53 primary finite event roots: 29 candidate and 24 opponent. Ten total roots carry secondary tags across statuses. These counts describe selected roots, not independent episodes or all-game error rates.

Terminal pair 03 game 0 ply 84 remains `claim_policy_limited`. Native Rd2 produces the automatic threefold claim; the reference's ...Rxc7 continuation is not observed gameplay. Its finite comparison remains recorded but is excluded from primary error causes. Both original automatic threefold draws and the dead-position draw remain unchanged. No draw is treated as weakness by itself, and no policy work is proposed.

Finite comparison support at thresholds 0.04 / 0.05 / 0.06 is 59 / 55 / 52. These figures include the terminal claim-limited comparison and any secondary comparison, so they are not primary event counts. At the primary threshold, four roots straddle significance and 46 retain report/discovery uncertainty, totaling 50 unresolved roots.

All eleven obligations are closed. No further query obligation is requested for this half-batch review. The files are ready for coordinator integration and independent challenge, with all remaining uncertainty preserved. No final weakness ranking, engineering explanation or production recommendation is made.
