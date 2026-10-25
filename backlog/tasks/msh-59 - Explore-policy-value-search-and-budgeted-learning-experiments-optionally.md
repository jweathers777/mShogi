---
id: MSH-59
title: Explore policy value search and budgeted learning experiments optionally
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
labels:
  - optional-research
milestone: m-6
dependencies:
  - MSH-48
  - MSH-55
documentation:
  - docs/design.md
ordinal: 59000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The design keeps policy/value models and MCTS available as a later research path, and M6 allows budgeted learning exploration. These are optional experiments, distinct from required six-game functional delivery, M5 strength/initial learned evaluation and M6 product quality. This task is not a prerequisite for MSH-60 or a useful product release.

Open research questions (D-09): whether to run a pilot at all, game choice, research question, policy/value representation, search method, architecture, compute/data budget and stopping/evaluation criteria. Python experiments must reuse the Rust rules. No MCTS adoption, distributed trainer, model scale or favorable result is decided here.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Before any authorized experiment, a bounded research question, selected game/baseline, provenance, compute budget, stopping criteria and reproducible comparison are recorded; speculative implementation plans are not preapproved by this scope.
- [ ] #2 Any completed pilot reports actual method, rules/model identities, cost, uncertainty and positive/negative results against the M5 baseline; it preserves authoritative complete actions/history. A decision not to pursue the path is documented honestly rather than presented as an engine improvement.
- [ ] #3 Adoption into the shipped product, if justified, requires separately scoped integration and verification; optional research status never hides or blocks required M4/M5/M6 delivery and does not require Python in the shipped application.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
