---
id: MSH-273
title: Runner Game Length Limits
status: Ready
assignee: []
created_date: '2026-11-25T22:13:00Z'
updated_date: '2026-11-26T07:44:00Z'
labels:
  - runner
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: low
ordinal: 250000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John decided on 2026-11-26: the developer runners keep the stop settings that the operator chooses (max_steps, max_plies, max_actions) and the 128 MiB report budget as experiment budgets. Their fixed upper bounds and the byte caps that grow with game length are removed, as MSH-268 (Remove Game Size and Engine Request Limits) did for the product. Places found on 2026-11-26 (check them again): in native experiments, the cap of 4096 total scheduled steps (so one game is at most 2048 steps), the 8 MiB report reserve per step under the 128 MiB budget, and the 1 MiB experiment configuration cap, which also counts opening records; in the external-engine match runner, max_plies at most 512, max_actions at most 32, command_bytes 4096 on the position command, the transcript bytes per game and the 1 MiB game entry check in report replay.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 max_steps, max_plies, max_actions and the 128 MiB report budget stay as experiment budgets; max_steps, max_plies and max_actions have no fixed upper bound, and no fixed schedule cap limits the length of one game.
- [ ] #2 No runner refuses or stops a game because of a byte cap that grows with game length: the position command, the transcript, the report replay entry and the opening records; the 128 MiB report budget is the one exception.
- [ ] #3 Tests and documents match; the documents say which experiment budgets stay and why; workspace and Python checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John: do the runner limits above fall under 'no limit on game size'? Recommendation: keep the per-experiment stop settings that the operator chooses (max_steps, max_plies, max_actions) and the 128 MiB report budget as experiment budgets, but remove their fixed upper bounds and the byte caps that grow with game length (command_bytes 4096 on position commands, the 1 MiB replay entry check, the 4096 total-step schedule cap as a per-game limit), and do not count opening records against the 1 MiB configuration cap.

John, 2026-11-26, answer to the question above: Yes, as recommended.
<!-- SECTION:NOTES:END -->
