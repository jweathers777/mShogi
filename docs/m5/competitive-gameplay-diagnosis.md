# MSH-161 (Competitive Gameplay Diagnosis)

The 200-game primary cloud study is complete. Independent Astra High synthesis and challenge identify recurring weaknesses in consequential continuation choices: exchange/capture order, queen and piece mobility, quiet defense, and the timing of pawn, king and rook resources. These observations support a focused investigation order. They do not establish a search or evaluation mechanism, an Elo rating, or a guaranteed rating gain from a proposed correction.

**Stop for John's review before engineering.** Original AutoClaim, mShogi, Oxidation and worker identities were preserved throughout the primary sample. No Batch 11, additional engine search, opponent patch, production correction or cross-game work follows from this report.

## Outcomes and scientific coverage

| Stratum | Games | mShogi wins | Draws | Losses | Score |
|---|---:|---:|---:|---:|---:|
| Prepared master-supported opening pairs | 180 | 67 | 55 | 58 | 52.5% |
| Unaided starts, separate correlated stratum | 20 | 15 | 3 | 2 | 82.5% |
| All native outcomes, descriptive total | 200 | 82 | 58 | 60 | 55.5% |

Ninety prepared opening pairs span 22 frozen source families. The family-stratified whole-pair bootstrap gives a **conditional 95% interval of 47.22–57.78%** for prepared score. Leave-one-family-out score is 51.45–53.53%. This conditions on purposive observed families; it does not quantify all-Chess population selection uncertainty. Cells have only four or five games per family/color. The result does not establish Oxidation's superiority: it was a suitable fixed competitive reference, provisionally selected before the main study, rather than a proven stronger opponent.

Unaided play is not 20 independent opening observations. All ten candidate-Black games share the first 16 plies; seven share 32. Two unaided games share as many as 70 initial plies, despite every complete trajectory being unique. Keep this stratum outside the prepared interval and avoid general opening-strength claims from its high score.

All **24,939 played decisions of both engines** have fixed Stockfish screening. Another 2,880 supplied-prefix decisions are context only. Played-root evidence tiers are 9,711 screening-only, 9,087 matched 5M, 6,139 selected 20M and two admitted direct-20M expansion roots. The latter retain mixed-budget audit interpretation. Full histories, native replay, source hashes, flags, legal alternatives and original failures are preserved. There are 6,141 selected deep roots across both players; candidate dispositions are 589 supported finite comparisons, 1,013 subthreshold, 1,781 same-choice, 657 unresolved and 20 claim-limited. These are correlated selected comparisons, not independent loss causes or all-move error rates.

## Observed behaviors and investigation order

The practical order below follows concrete consequential examples and breadth, not a numeric category league table.

1. **Resolve consequential continuation choices involving exchanges, quiet defense and coordination.** Exact examples join capture order, queen restriction, defensive resources and attacking preparation. The synthesis's initial seven anchors span six prepared families, both colors and wins, draws and losses. Examples include `main-06-08/g0/p44` Qxe2 versus Nxd4, the queen-capture/rook-entry sequence in `main-08-01/g0/p28`, and `main-10-01/g0/p72` Bxf5 versus Nd7. A legal alternative PV shows the consequence under reference analysis; it does not show why mShogi selected its move.
2. **Resolve linked pawn, king, rook and liquidation timing in endings.** Cases require different remedies: immediate promotion in some positions, support, king activation or retained defenses in others. Repeated b1=Q opportunities in `main-02-00/g1` are one motif, not three independent failed wins. The h6 versus Ng3 comparison at `main-05-04/g0/p84` illustrates why a blanket passed-pawn bonus would be unjustified. Ending anchors span five families and both colors.
3. **Integrate early central/development choices into the same continuation diagnosis.** At `main-10-04/g0/p22`, Bh6 is inferior to e5 in the retained comparison. This supports studying the timing of central play and deployment, not selecting an opening book or family blacklist. MSH-160 (Chess Opening Quality Assessment) uses these same frozen histories, prepared-root screening and separate unaided evidence; no competing opening corpus was created.

For the first investigation, distinguish a continuation that is not admitted, is cut off, is returned incorrectly, or is reached and valued incorrectly. Trace actual interior admission, windows, TT use, reductions, frontier replies and returned values under comparable context. A separate child-root search, nominal depth, faster visits or an attractive PV does not establish the distinction. Only then select a supported general search/value-propagation correction or classical judgment change. No single mechanism has been diagnosed here.

For endings, contrast immediate-push/promotion cases with support, king-route, blocker and liquidation controls. Test intermediate-position rankings and concrete consequences while preserving histories and claim policy. Opening choices should enter that same contrasted failure/success analysis before a separate subsystem is proposed. Any later bounded engineering assignment needs observable decision outcomes, success controls and stopping criteria. These exposed study positions are not a fresh held-out test. A substantial post-correction competitive campaign requires separate approval.

## Descriptive category accounting

The corrected table reconciles all 589 supported candidate roots across 175 games. Eleven additive correction rows were applied without changing original packets or numerical support. Categories overlap. Reviewer granularity differs: the first packet retained 82 unknowns, later packets three and one. The table is therefore a **coder-dependent descriptive summary**, not a precise frequency ranking of mechanisms or defects. Zero separately coded planning events means no defensible distinct label was assigned, not that planning is sound. Endgame decisions is a contextual behavior label, not every mechanically tagged ending error.

| Category | Supported moves | Affected games | Prepared families |
|---|---:|---:|---:|
| advantage preservation/conversion | 3 | 1 | 1 |
| coordination/activity | 252 | 117 | 22 |
| endgame decisions | 118 | 62 | 21 |
| exchanges | 100 | 80 | 22 |
| king safety | 40 | 32 | 19 |
| opening/development | 71 | 52 | 20 |
| opportunity response | 37 | 23 | 11 |
| pawn structures/breaks | 110 | 78 | 22 |
| planning | 0 | 0 | 0 |
| quiet defense | 154 | 86 | 22 |
| tactical calculation | 127 | 83 | 22 |
| unknown | 86 | 50 | 19 |

Severity across the 589 comparisons is 419 major, 64 serious and 106 significant under the frozen finite-E convention. The source expectation is not a calibrated win probability against Oxidation. Secondary contexts remain separate; an absent old secondary tag does not establish a fresh primary cause.

Mechanical phase counts for supported candidate roots are 105 opening, 366 middlegame and 118 endgame. Moving resolved opening boundaries by two plies changes 17 or 21 assignments. Material cutoffs 24/28 instead of 26 yield 97/121 supported ending roots. No opening boundary was unresolved in these complete histories. These conventions describe phases, not position quality or causal categories.

`behavior-aggregate.json` supplies category severity, family, phase, color, native-result and prepared/unaided cross-tabs, phase sensitivity and leave-one-family-out breadth. For example, coordination/activity affects 102 prepared games and 15 unaided games; after dropping each family in turn, affected prepared-game share ranges 55.2–58.7%. Quiet defense affects 82 prepared and four unaided games, with corresponding range 44.2–47.1%. These conditional descriptive shares are not error probabilities.

Literal retained-range support at thresholds .04/.05/.06 is **614/589/555** candidate comparisons. Stable straddlers, unstable/bound-conflicted roots, same-choice and native claim limits remain separate. Uniform 1e-9 arithmetic gives 615/590/556 only as a numerical-boundary sensitivity; it does not change the source 589-event baseline. Fifty-seven nonfinite roots straddle a threshold under that sensitivity; one historical boundary record is retained separately. Unresolved cases can add or alter behaviors, but cannot be allocated to invented categories or an exact category upper bound. Removing broad activity labels weakens prevalence claims while leaving the concrete exchange-order, mobility, pawn-support and promotion examples.

## Linked episodes and contrary successes

Three bounded narratives were reviewed; they are illustrative, not an exhaustive episode or conversion census.

- **Favorable segment, `main-06-02/g0/p73–80`:** after the reference's ...Nc7, successive candidate turns Rb8 and Ra8 have stable played E at least .75, satisfying the frozen favorable-segment gate. R8a7 at p78 loses that finite advantage versus Rxc8. The reference then concedes a drawing course. Ra8 is actually played at p80; Black subsequently claims threefold with intended ...Nc7 unplayed. All twelve relevant queries and the native endpoint were independently checked. Earlier uncertainty remains; margins are not added and no counterfactual win after the claim is assigned.
- **Promotion motif, `main-02-00/g1/p81,91,113`:** three choices miss b1=Q; repeated board arrangements retain different clocks/history. Count one linked resource motif, not three lost wins. The native game ends by threefold.
- **Missed attacking resources inside a win, `main-02-05/g0/p46–62`:** six selected candidate roots prefer Bxh7, intertwined with reference errors and uncertainty. The candidate later promotes twice and mates. This is neither six lost games nor a certified uninterrupted favorable episode.

Contrary evidence includes completed bishop-and-knight mate in `main-01-08/g0`, promotion and mate in `main-09-02/g0`, successful rook defenses, blockade and capture of promoted pawns into dead positions. Stable same-choice and withdrawn concerns remain. The final synthesis retains tactical controls even in eventual losses. Candidate wins can coexist with earlier supported inferior decisions, just as draws can coexist with successful defense.

Of 58 draws, 43 are threefold claims, four fifty-move claims and eleven dead positions. Original AutoClaim can limit observation of advantage conversion. Analyze decisions leading to claims without treating the draws themselves as proof of weakness or a reference continuation after termination as a native event. This report estimates no alternate-policy results or global failed-conversion rate.

## Resources, verification and evidence

The completed primary study used **30,889 scientific jobs, 86,581 reference queries and 164.657894 measured VM-hours**, priced at **US$33.196349 estimated compute**. Cloud submission-to-final-task stage durations sum to 28.173053 hours; they are not total human/model review duration. Reused played queries are counted once. Actual billing, ancillary charges and model usage costs have not been fully reconciled. Mac, qualification/pilots and abandoned-transition work remain separate. Earlier allocation failures and actual concurrency are preserved, including the constrained Batch 04 and Batch 07 stages; a request for eight workers is not assumed to mean eight were delivered in every stage. Final stages reached eight and all their workers were deleted.

Verification is scoped to scientific analysis and execution provenance, with targeted process/cloud evidence rather than product-platform claims. Native outcome/history checks, plan-aware query verification, legal-line replay, independent statistics, complete played-root coverage, phase sensitivity and behavioral corrections passed. Fresh explicitly configured Astra High synthesis and challenge are recorded. The challenger independently checked all 589 supported roots, 1,216 query bindings and legal lines (39,624 replayed plies), plus the favorable-segment evidence. No production source changed and no unperformed product/GUI/three-platform tests are claimed.

Primary entry points:

- [Independent review handoff](evidence/msh161/cloud-primary/independent-review-handoff.md): neutral raw-evidence map and blind-review instructions; no private chat history needed.
- [Scientific synthesis](evidence/msh161/cloud-primary/final-analysis/scientific-synthesis-astra.md) and [independent challenge](evidence/msh161/cloud-primary/final-analysis/scientific-challenge-astra.md).
- [Cumulative source ledger](evidence/msh161/cloud-primary/primary-cumulative.json), [native statistics](evidence/msh161/cloud-primary/final-analysis/native-statistics.json), [corrected behavior tables](evidence/msh161/cloud-primary/final-analysis/behavior-aggregate.json), [threshold audit](evidence/msh161/cloud-primary/final-analysis/audit-threshold-normalization.json), and [final-analysis procedure](evidence/msh161/cloud-primary/final-analysis/procedure.md).
- Raw store `gs://mshogi-experiments-216675633801/msh161-primary-v1`, mirrored in ignored `cloud-primary/local-cache/`; Git contains references and derived evidence, not the full raw archive.
- [Final batch closure](evidence/msh161/cloud-primary/batch-10-review/closure.md), per-stage execution reviews and frozen manifests. The prior 180-game archive has a documented misleading through-batch10 filename; its exact bytes/hashes are correct.
- [Updated roadmap](strength-roadmap.md). [Original preparation record](competitive-gameplay-diagnosis-preparation.md) is historical; frozen method and actual receipts govern results.

MSH-153 (Classical Compensation Calibration) remains held. No new engineering task, learned evaluation, Shogi/Xiangqi transfer or rating claim is selected automatically. The completed diagnosis is ready for John's decision on the proposed investigation order.
