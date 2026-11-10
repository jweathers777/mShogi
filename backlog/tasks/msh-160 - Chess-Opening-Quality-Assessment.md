---
id: MSH-160
title: Chess Opening Quality Assessment
status: To Do
assignee: []
created_date: '2026-11-10T18:28:00Z'
labels:
  - handle:chess-opening-quality-assessment
milestone: m-5
dependencies:
  - MSH-158
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
- [ ] #1 Before execution, establish a modest aggregate resource budget, process/artifact bounds, stopping rules and frozen protocol with independent review; finish current classical evaluation/correction work and update dependencies.
- [ ] #2 Verify and record source terms, licenses, attribution, access limits, provenance and data quality before use.
- [ ] #3 Use approximately 100–150 reproducibly sampled positions across diverse opening families, both colors and several development stages; preserve available full histories and label unknown history.
- [ ] #4 Compare unaided one/five/selected fifteen-second decisions using independent continuation evidence and the four soundness classes; report master-choice frequency separately and do not penalize theory absence alone.
- [ ] #5 Report development, center, king safety, threat responses, family difficulties, timing effects and middlegame soundness with uncertainty; any optional sequences continue legally and independently soundly after deviations.
- [ ] #6 Reuse existing infrastructure and native rules/history without a new framework, opening book or production changes; preserve reproducible results, actual resource use and independent outcome review.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
