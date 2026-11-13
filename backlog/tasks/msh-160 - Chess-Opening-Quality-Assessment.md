---
id: MSH-160
title: Chess Opening Quality Assessment
status: To Do
assignee: []
created_date: '2026-11-10T18:28:00Z'
updated_date: '2026-11-13T20:00:00Z'
labels:
  - handle:chess-opening-quality-assessment
milestone: m-5
dependencies:
  - MSH-158
  - MSH-159
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/experiments.md
ordinal: 164000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chess Opening Quality Assessment.

## Problem
Recent human play suggests that mShogi, which has no opening book, finds sound opening moves through conventional search and evaluation. This observation needs independent testing; popularity or theory membership alone cannot establish decision quality.

## Objective
Assess sound opening decisions, agreement with master-game choices, sound unusual alternatives, piece development, central control, king safety, responses to common threats, difficult opening families, thinking-time sensitivity and sound middlegame entry. Measure decisions rather than memorized theory; do not infer general strength from this sample.

## Approach
Future bounded research: propose approximately 100–150 balanced opening decision positions across families, both colors and early/intermediate/late development. Preserve full move histories where available and label unknown history. Compare one and five seconds, with selected fifteen-second probes. Predeclare sampling, independent continuation evidence, classifications and stopping rules. Classify established and sound, unusual but sound, questionable, or clearly poor; preserve uncertainty. Absence from theory is never proof of error. Report master-choice frequency separately from soundness and stratify by family, color, stage and time.

Candidate public resources, not yet cleared for use: Lichess Opening Explorer/master-game data and API documentation (https://lichess.org/api and https://github.com/lichess-org/lila-openingexplorer); Lichess Chess Openings/ECO (https://github.com/lichess-org/chess-openings); Lichess Position Evaluations (https://database.lichess.org/#evals); independent Stockfish analysis of selected alternatives (https://stockfishchess.org/ and https://github.com/official-stockfish/Stockfish). Verify current source terms, licenses, attribution/redistribution obligations, access/rate limits and data quality before acquisition/use. Record provenance, versions, access dates, coverage/selection bias, sparse master samples, transposition/ECO limitations and evaluation engine/depth/work conditions. Cached evaluations support investigation, not unquestionable truth.

Reuse existing experiment and native history/replay infrastructure in docs/m5/experiments.md and current decision-assessment tools; no new framework or second rules authority. Optionally include a small number of book-guided opponent sequences to inspect middlegame entry; mShogi always chooses unaided. After deviation, the opponent must continue legally with independently sound choices, never force an invalid or inappropriate book line. Do not introduce an opening book or modify production search/evaluation.

## Completion
Deferred To Do in existing M5. Depends on MSH-158 (Chess Decision Corrections), the recorded classical-work checkpoint, already completed at recording. Before activation, verify any then-active classical evaluation/correction assignment is finished and record its dependency. This assessment must not interrupt or block corrections or MSH-153 (Classical Compensation Calibration), which remains held. Separate from MSH-104 (per-game opening-knowledge research); no learned-evaluation prerequisite or assistance adoption is implied.

Before future execution, establish a modest aggregate engine/reference compute and artifact budget, individual process limits, stopping criteria and frozen source/procedure, with independent protocol review. Substantial resource scope requires owner approval under repository policy. Future completion requires reproducible evidence, actual resource use, limitations/uncertainty and independent outcome review. Recording this task authorizes no engines, large downloads, benchmark construction, experiments or production changes now. Planning preservation completes when this record and roadmap reference are verified and integrated into codex/rewrite; the research task stays unfinished.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Coordinate opening assessment within MSH-161 complete-game study; freeze reviewed method, sources, process bounds and actual resource accounting under the new owner authorization.
- [ ] #2 Verify source terms, provenance, attribution, access limits and data quality before use.
- [ ] #3 Measure unaided opening decisions separately from diverse prepared-position quality; preserve complete histories, family/color coverage and phase-transition definitions.
- [ ] #4 Distinguish established and sound, unusual but sound, questionable and clearly poor moves using independent continuation evidence; report master frequency separately.
- [ ] #5 Report demonstrated development/center/king-safety consequences and middlegame entry with uncertainty, without blaming candidate for supplied disadvantage.
- [ ] #6 Reuse native authority and existing infrastructure; no production/book changes; preserve reproducible complete-game analysis and independent review.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner supersession 2026-11-10: opening questions are now integrated into MSH-161 (Competitive Gameplay Diagnosis), whose full request authorizes broad complete-game study and necessary Stockfish analysis. Earlier proposed separate100–150-position sample,1/5/15s matrix and modest aggregate cap are superseded for this assignment. Use separate unaided start games plus prepared-position baseline analysis, master provenance and consistent phase boundaries. Preserve historical description; do not run competing corpus or introduce book. Dependency MSH-159 (Classical Evaluation Decisions) completed; opening deliverable closes with reviewed study evidence.

MSH-161 (Competitive Gameplay Diagnosis) completed the integrated opening questions without a competing corpus: 90 master-supported color pairs across 22 families plus ten separate unaided pairs, independent balance screening, full histories and consistent phase sensitivity. Prepared 67W/55D/58L versus unaided 15W/3D/2L; strong unaided dependence (all candidate-Black starts share 16 plies) prevents broad opening-strength inference. Supported early central/development choices enter the same continuation-quality investigation; no opening book, family blacklist or new campaign is selected. See docs/m5/competitive-gameplay-diagnosis.md and cloud-primary/independent-review-handoff.md. This preserves evidence for the existing task and does not activate its separate proposed study. Stop for John before engineering.
<!-- SECTION:NOTES:END -->
