# M5 opening-knowledge experiment — MSH-104

Owner-approved future scope,2026-10-27, including the dedicated opening-policy
clarification. **To Do; no implementation or shipping decision.** MSH-56 closure
requires completing the experiment; adopting any book or opening model is not
required. Existing six-game strength commitments and MSH-55's required learned
evaluator/improvement remain intact. MSH-59's optional full policy/value engine
research is a different question and is not a prerequisite.

## Per-game arms and purpose

For every target game establish a meaningful, justified subset and record why
an arm is unavailable or impractical. Consider no assistance, licensed human/game
corpus knowledge, offline deep-search-generated exact-position book, self-play
opening tree/statistics, compact learned opening policy, and book/policy hybrids
used by ordinary search. Chess/Shogi/Xiangqi are plausible corpus-rich cases,
subject to actual licensing/edition/provenance review. For Tori/Chu/Tenjiku
explicitly investigate machine-generated knowledge when reliable human corpora
are sparse. No game disappears merely because no corpus is convenient.

A **dedicated compact opening-policy model is a first-class experimental arm**.
Train it, where justified, from expensive offline teacher search and/or self-play
analysis over early-game positions to predict a distribution over promising
**complete native actions**. The teacher may spend substantially more work on
each target than interactive play allows. A policy for openings is distinct from
a learned position evaluator and from a future full policy/value engine.

Compare these uses where practical, documenting concrete resource/coverage
reasons for any omitted comparison:

| Arm | What is measured |
| --- | --- |
| Direct learned opening selection | Policy chooses a legal complete opening action without normal search verification; assess cost/strength and transition beyond its useful region. |
| Policy-only move ordering/prioritization | Same normal native search, policy changes exploration order/priorities; preserve correctness/limits and assess actual work saved. |
| Policy plus exact-position book statistics | Combine sources with explicit weighting/provenance and search use; ablate each contribution. |
| Normal search without learned prior | Baseline with no opening assistance, plus conventional/generated-book comparison when applicable. |

The intended experiment does not presume direct model play is preferable.
For **Chu and Tenjiku**, explicitly test whether search distillation discovers
useful early-game structure without a large historical game corpus. Native
route/complete-action identity and history semantics govern targets and inference;
do not force a chess-style move encoding or collapse distinct compound routes.

## Regeneration and comparison evidence

Record teacher engine/search/evaluator and rules-profile versions, executable/
model/configuration identities, per-position work limits, target construction
(including distribution transforms, weights/temperature if used), self-play
settings/seeds, source licensing/provenance, dataset/split hashes, training and
inference/export settings. Preserve native context needed by each target. Prevent
train/test leakage. Version the knowledge against its generating engine and
document how to regenerate or invalidate it when search/evaluation/actions change.

Use the reproducible MSH-48 harness and actual relevant functional/strength
baselines. Measure controlled-match strength with uncertainty, early-game work
or compute saved, opening/game diversity, robustness immediately after known
positions end, sensitivity to knowledge depth, storage footprint, generation/
training cost, inference overhead, and regeneration/versioning/maintenance cost.
Distinguish equal-work and interactive-budget comparisons where they answer
different questions. Training loss alone establishes neither strength nor savings.

The reviewed conclusion may favor ordinary search, a conventional/generated
book, a hybrid, or no special mechanism for a game. The compact model need not
ship. Completing this research does not replace MSH-55's improvement requirement
or waive a game-specific strength target. Python/training infrastructure is not
added to the product runtime by creating the task; any eventual native inference
must preserve rules, model/action identities, execution limits and save semantics.

## Dependency/ownership boundary

MSH-104 depends on shared experiment infrastructure48, not evaluator55 or optional
policy/value59. Before substantial per-game experiments, refine against relevant
actual game delivery/strength baselines; no artificial all-game delivery chain
or guessed model architecture is created now. MSH-56 retains all original47–55
dependencies and adds104; its original criteria remain, with additive experiment
completion acceptance. The existing M5 learned-evaluator requirement stays separate.
