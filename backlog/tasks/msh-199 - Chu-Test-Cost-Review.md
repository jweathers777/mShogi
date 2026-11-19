---
id: MSH-199
title: Chu Test Cost Review
status: Ready
assignee: []
created_date: '2026-11-19T06:19:00Z'
labels:
  - handle:chu-test-cost-review
  - chu
  - testing
milestone: m-5
dependencies: []
references:
  - docs/testing/verification-levels.md
ordinal: 206000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Chu Test Cost Review

**Problem.** In debug builds some Chu tests that run the real engine take more than 25 minutes or hit the engine watchdog. Examples: crates/mshogi-desktop-session/src/chu_document/execution_tests.rs (real_engine_standard_progression_measures_targets_pv_and_retired_branch_capacity) and crates/mshogi-console/tests/chu_internal.rs (four tests that search Chu with default play limits, 300,000 nodes). The same tests pass in release, which CI uses. A plain cargo test --workspace is therefore not usable, and it is not known whether each heavy test still checks something that a cheaper test does not (observed in MSH-194, Xiangqi Strength Harness, 2026-11-18).

**Objective.** Make each Chu test that runs the real engine a good test at proportionate cost, so that the debug workspace run is practical again.

**Approach.** Time every Chu test in debug and release. For each slow test, record what it checks and whether the large search budget is needed for that check, or whether accumulated, overlapping checks or default play limits cause the cost. Reduce limits, split, merge or remove tests only where the checked behaviour stays covered. Never weaken a rules or contract check to save time. Mark a test that must stay heavy as release-only, and give the reason.

**Completion.** The criteria below are met, with timings before and after recorded in the task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Debug and release timings of all Chu-related tests in mshogi-console, mshogi-desktop-session, mshogi-engine and mshogi-search are recorded before and after the change.
- [ ] #2 Each slow test has a recorded statement of what it checks and why its budget is needed; redundant or overlapping checks are removed or merged without loss of coverage, as shown by the recorded mapping.
- [ ] #3 cargo test --workspace (debug) completes without Chu watchdog failures, and no single Chu test takes more than about 60 s in debug, or a recorded reason marks it release-only.
- [ ] #4 Release CI suites still pass; independent review confirms that no rules or contract check was weakened.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
