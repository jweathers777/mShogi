---
id: MSH-48
title: Establish shared reproducible strength and learning experiment infrastructure
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-25
documentation:
  - docs/design.md
ordinal: 48000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-02/P-05 and A-08 require comparable measurements before optimization or substantial training. Extend the existing M2 search/match evidence into reusable experiment support. This ticket owns shared harnesses, evidence formats, native data generation and rules reuse; six game-specific opponent pools, targets and strength verdicts belong to MSH-49 through MSH-54, and the initial learned evaluator lifecycle belongs to MSH-55.

Open questions (D-09): evaluation conditions, statistically justified decision criteria, compute/data budgets and appropriate independent opponents must be researched against available baselines. Protocol extensions are selected only for real opponents. No throughput target, training scale, chosen model or invented strength target is approved here.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Reproducible experiment tooling records code/executable/model and profile identities, parameters, seeds, limits, hardware/build configuration, starting-position distributions, colors, results and failures; paired colors and multiple baselines are used where possible.
- [ ] #2 Shared native self-play/data generation and any needed Python/PyTorch bindings reuse the authoritative Rust rules and preserve complete actions and history; provenance, model/data identities and replayable regressions are retained without adding Python to the shipped application.
- [ ] #3 The harness supports the six game-specific strength protocols and independently assessed search/evaluation comparisons, including actual opponent adapters where required. Rule correctness, search correctness, execution performance and measured playing strength are reported separately.
- [ ] #4 Budgeted pilot measurements establish usable evidence and expose limitations before substantial training. Unresolved budgets, licensing/provenance or opponent availability are recorded as blockers for affected experiments, with no fabricated successful comparison.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
