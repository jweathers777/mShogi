---
id: MSH-236
title: Remove Remaining History-Work Limits
status: In Progress
assignee:
  - codex
created_date: '2026-11-23T22:01:00Z'
updated_date: '2026-11-24T06:44:00Z'
labels:
  - chu
  - native
milestone: m-5
dependencies: []
references:
  - gs://mshogi-experiments-216675633801/runs/msh-213/pilot-2/tasks/0/control/
priority: high
ordinal: 243000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Remove the remaining artificial history-work limits and all runtime consumers across all six games. John explicitly authorized this on 2026-11-24 after the surviving Chu limit blocked MSH-213. Keep real cancellation, process timeouts, game rules and saved-record compatibility; do not introduce another work model.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 No artificial history-work counter, quota or validation consumer remains in any game
- [ ] #2 Long-game regressions pass beyond the former work cutoff; saved records and rules behavior remain correct
- [ ] #3 Required checks and fresh review pass; MSH-213 resumes with the removal integrated
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Remove counters and validation budgets, retain direct native validation and cancellation, check old record readers, test long games, review and integrate. Then resume Chu measurements.
<!-- SECTION:PLAN:END -->
