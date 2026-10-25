---
id: MSH-55
title: >-
  Deliver an initial learned evaluator with reproducible training and native
  inference
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
documentation:
  - docs/design.md
ordinal: 55000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The accepted M5 outcome requires a demonstrated reproducible improvement from an initial learned evaluator after a correct rules/search/match baseline. A-08 calls for game-specific evaluators and shared infrastructure; NNUE-style evaluation is an initial candidate, not a resolved architecture or a promise of one shared model for all games. MSH-48 owns common rules bindings/data generation and experiment records; this task owns the initial model lifecycle and deployment evidence.

Open D-09 questions: select the initial game(s) only after their functional and baseline evidence is available; decide feature encoding, architecture/size, quantization, training data provenance, hardware/compute budget, export and native inference representation from pilot measurements. Record decisions and applicable game-task dependencies when scoped; no initial game or research result is selected here.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The selected initial game(s), completed rules/search baseline, data provenance, budget and comparison criteria are documented before substantial training. The relevant game delivery and strength-baseline evidence is linked; unresolved material model/compute choices remain explicit blockers.
- [ ] #2 Reproducible data preparation, Python/PyTorch training, validation, export and Rust/native inference preserve matching rules profile, feature encoding and model identity. Agreement and model compatibility/error handling have independent expected checks; product inference does not require shipping Python.
- [ ] #3 The candidate evaluator is integrated into the native mShogi engine and the actual supported product execution path, with clear model identity/configuration and reproducible comparisons against the transparent baseline. Required rules/search/cancellation and record compatibility remain intact.
- [ ] #4 Controlled recorded matches demonstrate reproducible playing-strength improvement under agreed conditions and uncertainty criteria; training loss, inference speed or self-consistency alone is insufficient. Results, budgets and limitations receive independent review; failure to improve remains unfinished M5 work, not a fabricated research success.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
