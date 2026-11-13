# MSH-161 (Competitive Gameplay Diagnosis): Batch 06 follow-up, pairs 00–04

All sixteen selected singleton obligations are complete. This review used the new alternatives, exact reused played 20M evidence, native-history decision data and archived pre-follow-up ledger. No engine, retrieval, cloud job, repeated unstable query or new PV investigation was run. The archived original remains unchanged.

All new artifact, reused played artifact and reused success hashes match their bindings. Each comparison has identical full-history decision data and 20M requested nodes. All 32 selected PV instances were replayed legally. `review-00-04.json` records exact query, plan, completion, verification and archived-ledger hashes.

## Sixteen exact comparisons

Plies are zero-based. Ranges are alternative minimum minus played maximum through alternative maximum minus played minimum across the last two distinct depths. They are finite sensitivity measures, not statistical bounds or exact values.

| Pair/game/ply | Played / new alternative | Margin range | Disposition |
| --- | --- | --- | --- |
| 00 / 0 / 30 | f5 / Ne2 | 0.3860–0.4365 | Unresolved: alternative depth instability |
| 00 / 0 / 35 | ...e6 / ...Rd7 | 0.0535–0.0535 | Supported finite significant |
| 00 / 0 / 36 | fxe6 / Ne2 | 0.0220–0.0220 | Supported subthreshold |
| 00 / 0 / 37 | ...fxe6 / ...b6 | 0.0530–0.2180 | Unresolved: reused played depth instability |
| 00 / 0 / 89 | ...Kc7 / ...Bf6 | 0.4985–0.4995 | Supported finite major |
| 01 / 1 / 28 | Rac1 / Nd2 | 0.0655–0.1360 | Unresolved: reused played depth instability |
| 01 / 1 / 29 | ...b6 / ...Qf5 | 0.0810–0.1270 | Unresolved: alternative depth instability and expected-score bound conflict |
| 01 / 1 / 106 | Rb3 / Rc5 | 0.1340–0.2330 | Unresolved: alternative depth instability and expected-score bound conflict |
| 01 / 1 / 107 | ...Ba1 / ...Ra7 | 0.4915–0.4940 | Supported finite major |
| 01 / 1 / 108 | Ke6 / Rb8 | 0.1990–0.2605 | Unresolved: reused played depth instability |
| 02 / 0 / 17 | ...O-O / ...dxe4 | 0.4860–0.5175 | Unresolved: reused played depth instability |
| 02 / 0 / 67 | ...Rc6 / ...Qg8 | 0.0865–0.1810 | Unresolved: reused played depth instability |
| 02 / 0 / 69 | ...Rd7 / ...Kg8 | −0.0130–0.1800 | Unresolved: both reports have depth instability |
| 02 / 0 / 70 | Qf5+ / Rb8 | 0.0240–0.0565 | Unresolved: alternative depth instability |
| 02 / 1 / 32 | Ne5 / Qe6 | 0.4875–0.5055 | Supported finite major |
| 04 / 1 / 44 | Kh1 / Re1 | 0.2735–0.4320 | Unresolved: alternative depth instability |

There are four supported finite comparisons, one subthreshold comparison and eleven unresolved comparisons. All four supported comparisons concern roots with existing supported events, so no new event root is added. The new ...Ra7 comparison strengthens the already supported late defensive comparison at pair 01 game 1 ply 107. Qe6 strengthens the existing comparison at pair 02 game 1 ply 32. The ...Rd7 comparison at pair 00 game 0 ply 35 supports significance at 0.04 and 0.05 but remains below 0.06.

Discovery ...Kg8 at pair 02 game 0 ply 69 now has its required singleton. Its instability and range crossing zero leave the comparison unresolved; the unrestricted best-move label does not resolve that uncertainty. No repeated search is requested. The other large unresolved margins likewise do not override failed report conditions.

## Preserved and changed root dispositions

All old and new comparisons remain separate in the final ledger. Existing supported events at the four supported new roots remain supported. The new uncertain alternatives do not erase a supported comparison against another move.

Candidate fxe6 at pair 00 game 0 ply 36 changes from a same-choice control to supported subthreshold after the new Ne2 comparison. Opponent Rb3 at pair 01 game 1 ply 106 changes from supported subthreshold to unresolved because the new Rc5 alternative remains uncertain and could be consequential. Its original stable subthreshold Rb4 comparison remains recorded. Candidate Qf5+ at pair 02 game 0 ply 70 changes from a same-choice control to unresolved because of the new uncertain Rb8 comparison. These changes do not create finite error counts.

Earlier unselected and unresolved evidence remains unchanged. No first-error or largest-error claim follows from completing the query obligations. No new PV-derived obligations are introduced.

## Legal lines and qualitative examples

The new ...Rd7 line at pair 00 game 0 ply 35 starts ...Rd7 Nb5 Nf6 Bd4 e6. It supports a concrete defensive rook-placement comparison. The new ...Bf6 line at ply 89 starts ...Bf6 Ncb5 Bxd4 Nxd4 Rc7, illustrating an active defense against the passed pawn and knights.

The new ...Ra7 line at pair 01 game 1 ply 107 starts ...Ra7 Rb4 Ne7+ Ke6 Nxf5 Kxf5. This supports a finite alternative in the late rook-and-minor-piece defense. The Qe6 line at pair 02 game 1 ply 32 starts Qe6 Ngf6 Ne5 Qe8 f5, giving a concrete queen-activity response. These are legally replayed examples, not forced proofs or engineering explanations.

The unresolved Rc5 line at pair 01 game 1 ply 106 starts Rc5 Ra7 Rc4 Rd7+ Ke4. The unresolved ...Kg8 line at pair 02 game 0 ply 69 starts ...Kg8 Nh4 Rf7 Qg6+ Kh8. Their legality does not establish their value. They are retained as explanatory evidence without another chain of searches.

## Five terminal claim controls

The final ledger preserves `claim_policy_limited` for pair 00 game 1 ply 131; pair 02 game 0 ply 80; pair 02 game 1 ply 75; pair 03 game 0 ply 96; and pair 03 game 1 ply 109. Underlying finite statuses remain explicit: four same-choice controls and one subthreshold comparison. They were not relabeled as unresolved or counted as primary causes during normalization.

Original native automatic claims end those games. A reference line continuing beyond a terminal action is not observed gameplay. Draws themselves are not weaknesses. No policy change is proposed. Native results remain two candidate wins, three losses and five automatic threefold draws.

## Final normalized ledger and closure

`batch-06-adjudication/final-dispositions-00-04.json` contains all 301 normalized roots under `decisions`, with separate old/new comparisons, exact sources, conservative margins, severity, secondary tags, threshold sensitivity and retained earlier uncertainty.

| Root status | Candidate | Opponent | Total |
| --- | ---: | ---: | ---: |
| `supported_finite` | 35 | 33 | 68 |
| `supported_subthreshold` | 56 | 10 | 66 |
| `same_choice_control` | 79 | 2 | 81 |
| `unresolved` | 49 | 32 | 81 |
| `claim_policy_limited` | 5 | 0 | 5 |

One supported finite event, candidate h4 at pair 04 game 0 ply 56, remains secondary. Primary finite event roots therefore remain 67: 34 candidate and 33 opponent. Twenty-six roots across statuses carry secondary tags. Finite support counts at 0.04 / 0.05 / 0.06 remain 68 / 68 / 64. These are selected comparisons, not independent episodes or all-game error rates.

All sixteen obligations are closed. No new query obligation remains for this half-batch review. Eighty-one roots retain scientific uncertainty; query completion does not mean every question is resolved. The files are ready for coordinator integration and independent challenge, with no final priority, technical mechanism or production recommendation.
