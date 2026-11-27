---
id: MSH-53.3
title: Tenjiku Reference Ladder and Blunder Screen
status: Ready
assignee: []
created_date: '2026-11-27T21:36:00Z'
labels:
  - tenjiku
  - strength
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-53.2
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
  - docs/m5/strength-harness.md
parent_task_id: MSH-53
ordinal: 260000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Measure how the accepted classical Tenjiku engine scales with search resources and screen its moves with a substantially deeper mShogi search. Cover Adams and Muller independently with the shared strength tools. John authorized both measurements; no external opponents or judges, architecture work, or baseline stop decision in this step.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Freeze the accepted revision, resource rungs, paired sampling and scoring method, and time and dollar limits before the campaign. Report the same-family scaling curve separately for Adams and Muller, with uncertainty, incomplete results and limitations.
- [ ] #2 Freeze the sampling, deeper judge protocol, blunder rule and classes before inspecting results. Report blunders per 100 moves and mean win-probability loss for each profile with coverage and limits; preserve reproducible blunder positions and apply the roadmap follow-up rule.
- [ ] #3 Record exact commands, revisions, inputs, run evidence, costs, and cleanup. Relevant checks pass and the parent task and roadmap reflect the evidence. Leave the baseline stop decision to John.
<!-- AC:END -->
