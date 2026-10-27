---
id: MSH-104
title: Evaluate per-game opening-knowledge strategies
status: To Do
assignee: []
created_date: '2026-10-27T16:10:00Z'
updated_date: '2026-10-27T20:26:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-109
documentation:
  - docs/m5/opening-knowledge.md
  - docs/design.md
ordinal: 104000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved future M5 experiment,2026-10-27. Early-game search may benefit from reusable knowledge, but a book or model must earn its generation/training/inference/maintenance cost. This scope compares per-game strategies; it requires evidence, not adoption or shipping. A dedicated opening-policy model is a first-class experimental arm, separate from MSH-55 learned evaluation and optional MSH-59 full policy/value search. No implementation, Python runtime dependency, training pipeline, architecture, fixed budget or strength target is approved by task creation. Refine game-specific prerequisites against actual functional baselines before starting each arm; native complete actions/history remain authoritative.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Before substantial experiments, document a per-game arm/coverage matrix and controlled comparison protocol using MSH-48, actual functional search/rules baselines and relevant MSH-49–54 evidence. Evaluate a justified meaningful subset for every target game; record unsupported arms, concrete acquisition/budget blockers and scope rationale rather than inventing missing corpora or silently omitting a game.
- [ ] #2 Compare appropriate arms including no opening assistance, provenance-cleared human/game-corpus knowledge, offline deep-search-generated exact-position books, self-play opening trees/statistics, compact learned opening policy, and hybrids using book/policy priors with ordinary search. Treat Chess/Shogi/Xiangqi as likely corpus-rich subject to actual licensing/provenance review; explicitly investigate machine-generated knowledge for Tori/Chu/Tenjiku without requiring a large historical corpus.
- [ ] #3 Treat a compact dedicated opening-policy model as a first-class arm that learns a distribution over promising native complete actions from expensive offline teacher search and/or self-play analysis of early-game positions. It is not merely the learned evaluator or a prerequisite for a future full policy/value engine. For Chu and Tenjiku explicitly test whether search distillation discovers useful opening structure despite sparse human records.
- [ ] #4 Where practical compare direct policy opening-move selection, policy-only ordering/prioritization for normal search, policy combined with exact-position book statistics, and normal search with no learned prior. Document any impractical comparison and its measured/resource justification; policy need not bypass search. All accepted moves and targets use authoritative native legality/complete-action encodings, including compound routes and native context.
- [ ] #5 Record reproducible teacher engine/search/evaluator and rules-profile versions, model/options/work limits (which may greatly exceed interactive budgets), target construction/distribution/temperature or weighting, self-play configuration/seeds, training data sources/licensing/provenance, dataset/split identities, training/inference/export configuration and artifact identities. Retain regeneration/version-compatibility procedure when engine/evaluator/action encoding changes; avoid train/test leakage and unsupported history conflation.
- [ ] #6 Measure controlled match strength with uncertainty, early-game compute/work saved, diversity, robustness immediately beyond known positions, sensitivity to knowledge depth, storage footprint, generation/training and inference cost, plus regeneration/versioning/maintenance complexity. Separate work-saving, rule correctness and strength claims; demonstrate whether benefits justify total cost under documented interactive and experimental budgets.
- [ ] #7 Independent review assesses reproducibility, licensing/provenance, all-game arm coverage, ablations and evidence-supported per-game recommendation. Valid completion may recommend ordinary search, generated/conventional book or hybrid and no shipping opening model. M5 closure requires completing this experiment, not adopting any mechanism; preserve all existing MSH-55 evaluator-improvement and six-game strength commitments. Product inference remains native and no Python/training runtime is added merely by this plan.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
