---
id: MSH-53.2
title: Tenjiku Evaluation Round
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-27T16:53:00Z'
updated_date: '2026-11-27T16:54:00Z'
labels:
  - tenjiku
  - evaluation
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-53.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
parent_task_id: MSH-53
ordinal: 259000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Run the classical evaluation round for both accepted Tenjiku profiles using the shared Texel fitter, MSH-214 harness and accepted MSH-202 engine. Preserve the game rules. Later strength measurement and baseline acceptance are separate steps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Use about 500,000 or more quiet self-play positions with rules-adjudicated outcomes, duplicate control and a split by game or opening. Record held-out loss before and after fitting with coverage of both profiles and report any shortfall.
- [ ] #2 Evaluation features agree with search scoring through moves, captures, promotion and undo, including applicable compound effects. Required correctness and tactical checks pass.
- [ ] #3 Keep weights only with qualifying clocked evidence against the frozen predecessor, using the predeclared method and H1 around 10 Elo. Record incomplete games and unsupported candidates and revert unqualified weights.
- [ ] #4 Required checks and applicable review pass. Update relevant durable documentation and record revisions, measurement settings, costs, run paths and cleanup in Backlog.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Claimed on codex/msh-53.2-tenjiku-evaluation from b0ab5ac. The frozen predecessor is the accepted MSH-202 product at d6274cb. Next: freeze the sampling, fitting and qualification gates before looking at pilot outcomes.
<!-- SECTION:NOTES:END -->
