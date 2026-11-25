---
id: MSH-245
title: One Clock Policy for All Games
status: Ready
assignee: []
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T12:41:00Z'
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
- [ ] #1 ClockAllocation, the Hard path, the allocation field of the clock configuration and the --clock-allocation flag are removed; the runner gives the engine the normal share in every game.
- [ ] #2 The watchdog validation uses the bound of the one remaining policy, or the notes record why a larger bound stays.
- [ ] #3 docs/m5/strength-roadmap.md and docs/m5/chu-strength-harness.md describe the one policy and say that earlier Hard results are not comparable; Rust and Python tests pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.
<!-- SECTION:NOTES:END -->
