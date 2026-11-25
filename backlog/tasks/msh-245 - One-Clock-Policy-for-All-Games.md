---
id: MSH-245
title: One Clock Policy for All Games
status: Done
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T15:40:00Z'
labels:
  - cleanup
  - runner
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 3200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John decided on 2026-11-25 (MSH-209 decision 2): every game uses the normal share of the remaining clock, today ClockAllocation::Nominal (TimeBudget soft). The experiment runner also has a Hard policy (three times the share) and a --clock-allocation flag. Places found: crates/mshogi-runner/src/experiments/mod.rs (FischerClock.allocation, ClockAllocation, clock_work, the shared-policy check and the watchdog bound that uses the hard budget), experiments/tests.rs, crates/mshogi-runner/examples/support/native_match_config.rs (default Hard and the flag; five *_match_config examples use it), tools/strength/native_match.py and tools/strength/tests/test_chu_harness.py, docs/m5/strength-roadmap.md (Clock allocation section) and docs/m5/chu-strength-harness.md. Earlier Hard results are not comparable.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ClockAllocation, the Hard path, the allocation field of the clock configuration and the --clock-allocation flag are removed; the runner gives the engine the normal share in every game.
- [x] #2 The watchdog validation uses the bound of the one remaining policy, or the notes record why a larger bound stays.
- [x] #3 docs/m5/strength-roadmap.md and docs/m5/chu-strength-harness.md describe the one policy and say that earlier Hard results are not comparable; Rust and Python tests pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.

Found in the new search, in addition to the listed places: the ignored measurement test record_timed_native_games (crates/mshogi-engine/tests/all_games_timing.rs) made its move time from the hard budget. It now uses the normal share. The engine's own protocol time management (soft and hard budgets for UCI and USI clocks) is not part of this policy and did not change. No open task plans to use the removed code.

Watchdog: validation now requires watchdog_ms to be at least the largest normal share (at most 60 s). No larger bound stays. The test checks the exact limit (19,998 ms).

Checks (2026-11-25, macOS): cargo fmt --check passed; clippy with -D warnings passed; cargo test --workspace --locked passed (1185 passed, 0 failed, 11 ignored); tools/strength Python tests passed (62 run, 1 skipped). The changed ignored test, run with a release build: the first run failed on ply 1 (452 ms against a 406 ms share) with a newly built binary; the second run passed. The probable cause is the first launch of the new binary; this is not proved.

Review by a fresh subagent (the saved experiment config and report format changed): no defects; one documentation wording point fixed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Every game in the native experiment runner now uses the normal share of the remaining clock (John's decision, MSH-209 decision 2). Removed: ClockAllocation, the Hard path, the allocation field of FischerClock, the shared-policy check, the --clock-allocation flag and its use in native_match.py. One function gives the share to the move time and to the watchdog check. docs/m5/strength-roadmap.md and chu-strength-harness.md describe the one policy; earlier Hard results are not comparable. Experiment configs and reports that have the allocation field no longer decode. Required checks pass. No follow-up tasks.
<!-- SECTION:FINAL_SUMMARY:END -->
