# MSH-161 (Competitive Gameplay Diagnosis): independent evidence handoff

This is a neutral entry point for another chat or model. No private conversation history is required. All 200 primary cloud games and required reference searches are complete. The final interpretation, corrected category tables and independent challenge are complete; exact acceptance hashes are in `final-analysis/final-integration-review.json`. John has not authorized follow-on engineering.

## Question and conditions

Identify recurring observed decisions limiting mShogi's competitive Chess effectiveness, then distinguish possible mechanisms and discriminating investigations. This is not an Elo/human-rating measurement, a proof of code mechanisms or authorization to engineer changes. Inspect gameplay without prior engineering explanations before proposing mechanisms.

The primary cohort contains 90 master-supported opening pairs (180 games, swapped engine colors) across 22 source families, plus ten unaided starting-position pairs (20 games). Both engines receive the frozen practical five-second-per-move conditions. Pair/family selection is purposive, not a random Chess population. Unaided starts share substantial prefixes and stay separate. Fixed opponent is Oxidation 0.6.0. Qualification selected it provisionally; main results do not establish that it is stronger.

All primary games used original AutoClaim with unchanged original mShogi/Oxidation binaries and worker image. Optional threefold/fifty-move claims terminate games under this policy. Preserve actual native outcomes; intended claim moves are unplayed. Claims censor observations about continued conversion. A draw itself is not weakness evidence. An unsuccessful automatic-threshold-only policy transition is separate evidence and supplied no primary games. Do not patch the opponent or repeat games to homogenize conditions.

Native results: 82 wins / 58 draws / 60 losses for mShogi. Prepared: 67/55/58; unaided: 15/3/2. Native results are authoritative, not Stockfish estimates. Forty-seven draws are optional claims (43 threefold, four fifty-move); eleven are dead positions. All played decisions of both engines have fixed Stockfish screening, with selected matched 5M and consequential matched 20M work. Finite WDL-derived expectation and report ranges are neither calibrated opponent win probabilities nor exact chess proofs. Selected/correlated comparisons are not unbiased error frequencies or independent loss causes.

## Read first: protocol and frozen inputs

Paths below are relative to this directory unless prefixed `../`.

- `../owner-request.txt`: original scientific objective and authorized scope.
- `../main-method.md`: frozen scientific protocol, thresholds, phases, audit, episodes and inference.
- `../main-freeze.json`, `cohort-freeze.json`, `cloud-config.json`, `batch-01-manifest.json` through `batch-10-manifest.json`: exact frozen inputs, conditions, worker image and hashes.
- `autoclaim-restoration.json`: authoritative original-policy/binary/image identities; transition abandoned.
- `../sources/main-selection.json`: 90 opening IDs, families, source games and selection metadata; source/history files are bound by the freeze.
- `batch-09-review/method-amendment-v1.1.json`, `audit-expansion-admission.json`, `audit-expansion-notation-erratum.json`: admitted narrow audit expansion. Preserve original 5M trigger and 20M evidence. The 21-root cell has 19 reused 5M roots plus two new 20M roots/three searches; mixed budgets do not yield a uniform cell error rate. Correct SAN is Bf8 for UCI g7f8.
- `final-analysis/procedure.md` and `procedure-admission.json`: final descriptive statistics/coding/inference procedure, with independent reviews. Use whole opening pairs, family-stratified bootstrap; unaided repeats are separate.

The high-level `../../../competitive-gameplay-diagnosis.md` now contains the integrated final report. Original preparation text is preserved separately as `../../../competitive-gameplay-diagnosis-preparation.md`. Frozen method and actual primary receipts take precedence over old planning/status prose.

## Raw evidence and how to locate it

The durable raw result store is:

`gs://mshogi-experiments-216675633801/msh161-primary-v1`

The existing local, Git-ignored mirror is `local-cache/`. Store object keys map directly beneath that directory. **Git alone does not contain all raw evidence.** A different chat on this Mac can read the mirror. A different computer/model needs a read-only export or existing authorized GCS read access. Do not change bucket/security settings or launch new computation to obtain access.

`primary-cumulative.json` contains `native_games`: every entry gives pair/game IDs, actual outcome and the native report object key/SHA256. Native pair reports contain complete canonical histories, accepted commands, claims, native conclusions, UCI transcripts and process/identity evidence. Many reports are gzip wrappers containing `report_json`; `../analysis.py` provides the bounded decoder and accepted-move extractor.

For each batch, `batch-NN-review/review-index.json.gz` (or plain `.json` in earlier batches) maps native pair reports, all-move screening and matched 5M success/artifact objects. `remote-inventory` and retrieval/verification records preserve store identities. Query artifacts retain exact initial FEN, complete history, mover, played/forced move, Stockfish identity/options/node budget, reports/PVs/WDL, stability and bound flags, and process receipts. Native replay and legal-PV checks are retained in result artifacts/reviews. Preserve original source rows rather than using only prose summaries.

`batch-NN-adjudication/plan.json` and its `results/verification.json` bind consequential 20M selections and successful objects. `batch-NN-followup/plan.json`, where present, binds only new alternatives and hash-bound reuse of existing played 20M results. Do not count reuse as new searches. Source overlays preserve earlier comparisons, unresolved dependencies and secondary contexts. Original pre-followup ledgers remain intact.

There is no Batch 11. Local Mac exploratory games, qualification/pilots and failed policy-transition validation are separate cohorts under the parent evidence tree. Do not pool them with primary results.

## Convenient structured derived datasets

These accelerate review but do not replace raw evidence:

- `primary-cumulative.json`: all 200 native outcomes, 6,141 selected roots, exact final-source pointers, original claims, audit amendment and source hashes. Contains 589 supported candidate comparisons; uncertain/control/subthreshold/claim-limited statuses remain separate.
- `final-analysis/played-coverage.json.gz`: all 24,939 unique played roots of both engines, plus 2,880 separate supplied-prefix context roots; evidence tiers, verbatim base statuses/flag paths, final-source row hashes. Screening-only is unresolved, not confirmed non-error.
- `final-analysis/native-statistics.json`, `native-results-review.md/json`: native outcomes, pair/family/color tables, whole-pair conditional bootstrap and source bindings. Prepared score 52.5%, conditional 95% interval 47.22–57.78%; not an Elo interval or population guarantee.
- `final-analysis/phase-summary.json.gz`, `phase-result-review.md/json`: frozen phase tags and material/boundary sensitivity for every played root.
- `final-analysis/context-tables.json`: family-by-color outcomes and repeated unaided prefixes. Unique full trajectories do not establish independence.
- `batch-10-followup/forecast-update.json` and runtime reviews: final primary 30,889 jobs / 86,581 queries / 164.657894 VM-hours / US$33.196349 estimated compute. Billing/ancillary/model/pilot/Mac/transition costs are not included or silently reconciled. Earlier resource-pool failures and actual concurrency remain in per-stage records.

A previous cumulative archive is misleadingly named `batch-10-review/primary-cumulative-through-batch10.json.gz`: its exact contents are the preserved 180-game record through Batch 09. Hashes/content are correct; see `native-results-review.md`. Do not reinterpret the filename as final coverage.

## Blind analysis versus reviewing existing interpretations

For a fresh independent gameplay diagnosis, begin with the protocol, frozen inputs, raw native games and query evidence. Delay reading category narratives, roadmap and synthesis until you have your own observations. Use native complete histories, not isolated FENs.

To audit existing interpretations, additionally read the twenty batch gameplay panels, deep/followup reviews and final overlays; then `final-analysis/behavior-coding-01-04.*`, `behavior-coding-05-07.*`, `behavior-coding-08-10.*` and the additive `behavior-coding-challenge.*` / `audit-*` records. Coding granularity differs across reviewers, with explicit unknowns; category counts overlap and are descriptive, not a precise league table of failure rates. Additive corrections retain source packets. The provisional `threshold-summary.json` has a known early-schema omission; use independently corrected `audit-threshold-normalization.json` instead. Source .05 dispositions and floating-boundary sensitivities are distinct.

The `scientific-synthesis-astra.*` and `scientific-challenge-astra.*` files are the completed synthesis and independent challenge. Final exact-file acceptance is in `final-integration-review.*`; `completion-verification.json` links checks and hashes. Requested agent models/effort are recorded in `model-assignment.json`. Their interpretation limits remain part of acceptance.

## Suggested instruction for another chat

> Independently analyze MSH-161 using cloud-primary/independent-review-handoff.md. Work from saved evidence only; do not run engines, new searches/games or modify production. First inspect the frozen method and raw primary gameplay/reference evidence without earlier engineering explanations. Keep prepared opening pairs/families and correlated unaided starts separate. Preserve native AutoClaim outcomes, full histories, uncertainty, secondary contexts and admitted audit limits. Identify recurring supported behaviors with affected-game/move breadth, bounded linked episodes, contrary successes and legal alternatives. Separate observations, possible mechanisms and discriminating investigations. Challenge reference suitability and category/inference limits. Compare with existing synthesis only after forming independent conclusions. Report evidence paths and unresolved questions; stop before engineering.
