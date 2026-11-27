---
id: MSH-263
title: Shared Experiment Dispatch and Operator Budgets
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-27T00:00:00Z'
labels:
  - runner
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies:
  - MSH-261
documentation:
  - docs/m5/chu-strength-harness.md
priority: medium
ordinal: 7650
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
One Per-Game Table for Native Experiments:
crates/mshogi-runner/src/experiments/native.rs repeats one match arm per game in from_root, root, turn, outcome and admit; the Tori, Chu and Tenjiku admit arms are the same shape; Shogi and Xiangqi build actor and command twice each; the Xiangqi score copies the one in xiangqi.rs. experiments/mod.rs has further per-game tables (valid_work, first_side), the test PROFILES list copies the one in mod.rs, and run.rs repeats the player and step decision logic of the report validation in mod.rs.

Remove the fixed ten-minute experiment cap:
MSH-52.3 truncated seven of eight 40s+0.4s versus 80s+0.8s color pairs at the fixed 600000ms experiment ceiling. Both the native runner and native_match reject a larger caller-specified budget, preventing longer measurements even within the approved cloud budget.

Runner Game Length Limits:
John decided on 2026-11-26: the developer runners keep the stop settings that the operator chooses (max_steps, max_plies, max_actions) and the 128 MiB report budget as experiment budgets. Their fixed upper bounds and the byte caps that grow with game length are removed, as MSH-268 (Remove Game Size and Engine Request Limits) did for the product. Places found on 2026-11-26 (check them again): in native experiments, the cap of 4096 total scheduled steps (so one game is at most 2048 steps), the 8 MiB report reserve per step under the 128 MiB budget, and the 1 MiB experiment configuration cap, which also counts opening records; in the external-engine match runner, max_plies at most 512, max_actions at most 32, command_bytes 4096 on the position command, the transcript bytes per game and the 1 MiB game entry check in report replay.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Per-game experiment behavior comes from one trait or table; arms of the same shape are merged; the run and the validation share one decision function.
- [ ] #2 Experiment tests and recorded-report replay pass; production line counts before and after are recorded.
- [ ] #3 The native experiment runner and match driver accept positive representable caller-chosen durations above ten minutes without another arbitrary duration ceiling.
- [ ] #4 The configured deadline still stops work and records unfinished games as unscored; game rules and product search behavior are unchanged.
- [ ] #5 Current callers, tests and measurement documentation use the same duration contract; required checks pass.
- [ ] #6 max_steps, max_plies, max_actions and the 128 MiB report budget stay as experiment budgets; max_steps, max_plies and max_actions have no fixed upper bound, and no fixed schedule cap limits the length of one game.
- [ ] #7 No runner refuses or stops a game because of a byte cap that grows with game length: the position command, the transcript, the report replay entry and the opening records; the 128 MiB report budget is the one exception.
- [ ] #8 Tests and documents match; the documents say which experiment budgets stay and why; workspace and Python checks pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Consolidate native game dispatch, command admission, player selection and run/replay decisions. 2. Remove length-dependent limits from configuration, transport, report replay and current drivers. Charge native evidence against the encoded report budget. 3. Add boundary and interruption tests, update the measurement guides, run workspace/Python and affected platform checks, get a fresh review and integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-222 (2026-11-26), finding in this scope: EngineResult::validate (line replay), EngineGame::command and EngineGame::submit now serve Shogi, Xiangqi, Tori, Chu and Tenjiku, so the five admit arms in experiments/native.rs can be one generic function (validate, then the command of the choice; Finished; Incomplete or Unresolved). experiments/mod.rs valid_work can read EngineGame::MAX_DEPTH and MAX_NODES.

Context carried from MSH-238 (Remove the fixed ten-minute experiment cap); historical observations, not a replacement for the current criteria and dependency fields:
John, 2026-11-25: runs after the MSH-209 follow-ups and before MSH-243, which uses the runner.

Context carried from MSH-273 (Runner Game Length Limits); historical observations, not a replacement for the current criteria and dependency fields:
Question for John: do the runner limits above fall under 'no limit on game size'? Recommendation: keep the per-experiment stop settings that the operator chooses (max_steps, max_plies, max_actions) and the 128 MiB report budget as experiment budgets, but remove their fixed upper bounds and the byte caps that grow with game length (command_bytes 4096 on position commands, the 1 MiB replay entry check, the 4096 total-step schedule cap as a per-game limit), and do not count opening records against the 1 MiB configuration cap.

John, 2026-11-26, answer to the question above: Yes, as recommended.

John approved consolidation on 2026-11-26: this task retains the full scope of MSH-263 (One Per-Game Table for Native Experiments), MSH-238 (Remove the fixed ten-minute experiment cap), MSH-273 (Runner Game Length Limits). Experiment dispatch, configuration, execution, replay and operator-selected budgets overlap; apply the existing approved budget decisions together. Agent labels are model:large, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed by Codex on codex/msh-263-experiment-budgets from eb3f697. Next: inventory dispatch and budgets, implement, verify and integrate.

Checkpoint on codex/msh-263-experiment-budgets: dispatch and replay now share admission and player selection. Removed game-length and pair-duration ceilings, updated CLI/driver entry points and guides. Runner all-target compilation passed before final boundary tests. Next: finish review, resolve findings, verify platform and workspace checks, then integrate.

Independent review of f530814 found four issues: replay applied the stdout limit to position commands; replay did not enforce per-exchange evidence; external runs could accumulate oversized reports; native sizing omitted long escaped stop reasons. Corrected all four. External traffic and completed games now consume a shared report allowance, and native transitions account for the encoded final outcome/stop and cleanup. Regression coverage includes a 104-action opening and an exchange reset. All runner suites passed after the fixes; final workspace and platform verification in progress.
<!-- SECTION:NOTES:END -->
