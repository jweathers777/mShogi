---
id: MSH-53.1
title: Tenjiku Selectivity Round
status: Ready
assignee: []
created_date: '2026-11-27T15:26:00Z'
labels:
  - tenjiku
  - search
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-202
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
parent_task_id: MSH-53
ordinal: 258000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Run the classical selectivity round for both accepted Tenjiku profiles using the MSH-214 harness and the MSH-202 search baseline. Preserve the rules and use the shared strength tool. Evaluation, later strength measurement and the baseline stop decision are separate steps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Quiescence checks, full SEE, game-scaled margins, move ordering and shared df-pn mate search each have a measured result or a concrete reason they do not apply, for both profiles as applicable.
- [ ] #2 Freeze the predecessor and record the test method, reachable depth, caps and decision threshold before trials. Kept changes pass applicable perft, differential and tactical checks and qualify in a clocked predecessor test when search behavior changes. Record rejected or inconclusive trials and unscored games.
- [ ] #3 Required checks pass, affected durable documentation is current, and task notes record revisions, settings, measurements, costs, run paths and cleanup.
<!-- AC:END -->
