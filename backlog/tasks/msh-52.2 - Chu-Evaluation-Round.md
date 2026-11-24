---
id: MSH-52.2
title: Chu Evaluation Round
status: In Progress
assignee:
  - codex
created_date: '2026-11-24T19:15:00Z'
labels:
  - chu
  - evaluation
milestone: m-5
dependencies:
  - MSH-52.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 245000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace the untuned Chu material and placement weights with a measured classical evaluation, using the shared Texel fitter and native self-play outcomes. This is the evaluation step John requested after the selectivity round; no neural evaluation or external opponents.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 About 500,000 or more quiet self-play positions have native outcome labels, a split by game or opening, duplicate control and recorded held-out loss before and after fitting.
- [ ] #2 Evaluation features agree with the search score through moves, captures, promotion and undo; native correctness and tactical checks pass.
- [ ] #3 Kept weights have supporting predecessor SPRT evidence at H1 about 10; unsupported candidates are reported and reverted.
- [ ] #4 Required checks and review are complete, durable documentation is current, and run identities, costs and resource cleanup are recorded.
<!-- AC:END -->
