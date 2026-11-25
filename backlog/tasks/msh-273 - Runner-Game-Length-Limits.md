---
id: MSH-273
title: Runner Game Length Limits
status: To Do
assignee: []
created_date: '2026-11-25T22:13:00Z'
labels:
  - runner
  - needs-john
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: low
ordinal: 250000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-268 (Remove Game Size and Engine Request Limits) removed the product caps on game size and engine requests. The developer runners still have limits that can depend on game length: native experiments cap total scheduled steps at 4096 (so one game is at most 2048 steps) and stop a game with artifact_limit inside the 128 MiB report budget (8 MiB reserve per step); the 1 MiB experiment configuration cap also limits the size of opening records; the external-engine match runner caps max_plies at 512, max_actions at 32, command_bytes at 4096 (the position ... moves command), transcript bytes per game and a 1 MiB game entry in report replay. John must decide whether his decision covers these developer tools.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 John's decision for each listed runner limit is recorded in the notes.
- [ ] #2 Each limit that John removes is removed with its tests and docs; each kept limit has its reason in the docs.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John: do the runner limits above fall under 'no limit on game size'? Recommendation: keep the per-experiment stop settings that the operator chooses (max_steps, max_plies, max_actions) and the 128 MiB report budget as experiment budgets, but remove their fixed upper bounds and the byte caps that grow with game length (command_bytes 4096 on position commands, the 1 MiB replay entry check, the 4096 total-step schedule cap as a per-game limit), and do not count opening records against the 1 MiB configuration cap.
<!-- SECTION:NOTES:END -->
