---
id: MSH-55
title: >-
  Deliver an initial learned evaluator with reproducible training and native
  inference
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-08T13:08:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-109
  - MSH-49
  - MSH-50
  - MSH-51
  - MSH-52
  - MSH-53
  - MSH-54
documentation:
  - docs/design.md
  - docs/m5/strength-roadmap.md
ordinal: 55000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner sequence2026-11-08 selects Chess first for the initial learned evaluator, after conventional development across all six games. Research NNUE-style efficiently updated evaluation as a candidate against the mature conventional Chess engine; select features/model/native representation from real requirements rather than prescribing one architecture for every game. Preserve the required reproducible Python/PyTorch training, export, Rust inference, actual product integration and independently demonstrated strength improvement. MSH-135 tracks subsequent other-game adaptation; it is separate from this initial M5 deliverable.

Routine correctness/tactical tests and bounded profiling accompany coherent increments. Aggregate routine local engine-match work is limited to15minutes per task unless John explicitly authorizes more; no per-technique substantial statistical studies or multi-hour benchmarks. Formal studies require owner approval at a substantially complete conventional-engine or new-evaluation milestone. Routine algorithm and implementation choices are autonomous. See AGENTS.md and docs/m5/strength-roadmap.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Chess is the selected initial learned-evaluation game, after all six conventional engines are established. Its completed conventional rules/search/strength baseline, data provenance, substantial training budget and formal comparison criteria are documented before training/studies. Routine feature/model implementation choices are researched by the implementer; material compute/data/product decisions remain explicit.
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
