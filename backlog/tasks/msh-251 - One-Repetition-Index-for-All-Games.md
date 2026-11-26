---
id: MSH-251
title: One Repetition Index for All Games
status: Done
assignee:
  - '@codex'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T14:51:00Z'
labels:
  - core
  - simplification
  - performance
  - model:large
  - effort:high
milestone: m-6
dependencies:
  - MSH-250
priority: medium
ordinal: 6800
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each native game finds earlier occurrences of a position in its own way: Chess keeps a hash-map count (chess/game.rs), Tori a digest index (tori/game.rs, MSH-208, the only measured one), while Shogi (shogi/game.rs occurrences) and Xiangqi (xiangqi/game.rs occurrences) scan all earlier identities after each command, Chu scans all earlier keys (chu/game.rs current_occurrences), and Tenjiku scans all earlier keys for each candidate action in legal_actions_with_stop and in bare-royal adjudication (tenjiku/game.rs). The repetition rules differ by game; the lookup does not need to differ. Do this after MSH-250 (Token-Only Native Request Context), which changes the same functions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 One generic repetition index in mshogi-core (position key to occurrence indices, with append and truncate) serves all six games; per-game code keeps only its key type and its rule.
- [x] #2 No full-history scan per command or per candidate action remains in Shogi, Xiangqi, Chu or Tenjiku.
- [x] #3 Rules, perft and record tests pass unchanged; replay and legal-action time at full length are measured before and after.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Measure release replay and legal choices on the same deterministic full-length histories, including Chess and both Tenjiku profiles. 2. Introduce one exact-key index with ordered occurrences and truncation. Use it for all six game histories, including Chu cycle anchors and Tenjiku admission. 3. Check unchanged rule, perft and record fixtures, compare timings, review the diff, integrate and clean up.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measurement tools that exist (MSH-209): mshogi-core examples record_replay, resource_admission and tenjiku_rules_work.

MSH-250 (Token-Only Native Request Context), 2026-11-26: did not change any occurrences or repetition function. Chu replay at 1000 events is still about 615 us per event after MSH-250 (was 674); its repetition scans dominate. Baseline for this task: target/release/examples/resource_admission GAME N (shogi and xiangqi added), numbers in MSH-250 notes.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed by Codex on codex/msh-251-repetition-index. Next: measure the baseline and implement the shared index.

Checkpoint on codex/msh-251-repetition-index: all six games use the shared index. It stores keys in order and links matching hash buckets by board ply, with full equality checks on collision. Chu tries only indexed cycle midpoints, Tori tracks the earliest doubled key for each turn, and Tenjiku checks the index in admission and bare-royal replies. First design used an allocation per key and slowed Xiangqi copies; the replacement uses contiguous vectors. Baseline comparisons cover all six games and both Tenjiku profiles. Next: run interleaved final measurements, independent review, shared checks and deep perft, then integrate.

Fresh independent review of fecbcf5..179087e found no rule, collision, scan, rollback or undo defect. Fixed the benchmark finding: control walks now time the same native legal-choice query as board walks, separately from the scripted control command. A 1000-event Tenjiku control walk completed. Board-record lengths and bytes matched in all three interleaved baseline/candidate runs for each profile. First integration check stopped on a Clippy question-mark suggestion in the Tori guard; fixed it. Next: complete checks, upload timings and integrate codex/msh-251-repetition-index.

Full-length timings, Apple M1 Pro, Rust 1.99.1 release, single process, fecbcf5 versus 179087e with the same final resource_admission example from 2ea7376 (only candidate product edit in 2ea7376 uses the equivalent Option guard). Command: target/release/examples/resource_admission GAME N. Three interleaved baseline/candidate pairs; median of each run’s 5 replay or 11 legal-choice samples. N=1000, except Shogi 318 near its 320-ply limit. Chess uses native analysis-history replay and locally legal actions. Results below give complete replay in ms, then legal choices in microseconds, before -> after.
Chess: 0.478 -> 0.564, 0.291 -> 0.334. Shogi: 13.03 -> 13.31, 28.00 -> 27.63. Xiangqi: 153.10 -> 152.44, 1.625 -> 1.666. Tori: 23.13 -> 26.28, 23.25 -> 28.25; undo 154.08 -> 1.46 us. Chu: 592.76 -> 593.47, 1.459 -> 1.500. Tenjiku Adams: 7.015 -> 5.806, 161.42 -> 42.29. Tenjiku Muller: 8.237 -> 5.532, 83.08 -> 12.46. Chu full replay was effectively flat after removing the two scans. Chess and Tori replay were slower. No expected rule, perft or record test changed. Timing data and comparison script: gs://mshogi-experiments-216675633801/runs/msh-251/. No cloud compute or VMs; storage/requests estimated below US/bin/zsh.01.

Integrated implementation and measurements by fast-forward of main at 5fcb379. Required checks on 2ea7376 against fecbcf5 passed: python3 tools/checks.py run --base origin/main (format, workspace/all-target Clippy, 1158 Rust tests passed with 10 ignored by default, TypeScript, 575 frontend tests). The six independently expected Chess perft cases passed in release including the deep case, and the native rules, replay, record, collision/truncation and fixed-search baseline tests passed. Node 24.19.0 and Rust 1.99.1. No unresolved check. No new follow-up task.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
All six games use one index for exact repetition lookup with append and truncation. Chu checks only candidate cycle midpoints, and Tenjiku checks recurrence directly for each action. Native rules, records and perft pass with their existing expectations. Full-length Tenjiku legal choices improved 3.8x (Adams) and 6.7x (Muller); Tori undo improved about 106x. Tori and Chess replay took longer, while Chu replay was flat. Review and all required checks passed. MSH-252 (Shared Game History and Xiangqi Replay) is now eligible.
<!-- SECTION:FINAL_SUMMARY:END -->
