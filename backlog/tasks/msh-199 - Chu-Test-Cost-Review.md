---
id: MSH-199
title: Chu Test Cost Review
status: Ready
assignee: []
created_date: '2026-11-19T06:19:00Z'
updated_date: '2026-11-23T12:58:00Z'
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

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Observed again during MSH-228 on 2026-11-23, Apple Silicon Mac: cargo test --workspace --locked -j 2 -- --test-threads=2 stopped in chu_internal (2 passed, 4 failed, 30.48 s). Failures: actual_child_executes_complete_chu_choices_for_full_known_history_and_purpose (Timeout: internal engine response/exit watchdog expired at default depth64/300000 nodes); actual_controller_readonly_analysis_save_reopen_and_stale_identity_retirement (no score); actual_engine_turns_for_both_player_assignments_admit_observation_then_action and real_historical_analysis_saves_exact_prefix_and_reopens_stopped_without_future (only one board ply, expected two). No test budgets/assertions or watchdog changed in MSH-228. Removed document-capacity measurement tests no longer apply after minimal-document refactor.

Debug failure log: gs://mshogi-experiments-216675633801/runs/msh-228/workspace-debug.log (60-day lifecycle). Source commit 6102123; subsequent changes were GUI observer assertions, comments and task notes.

MSH-230 (source 1b75415), Apple Silicon Mac with 2 build jobs/2 test threads: same four chu_internal watchdog failures reproduced (30.49 s), while all 1182 workspace release tests pass. An additional debug --no-fail-fast run reached mshogi-engine/tests/chu_protocol.rs::bounded_standard_progression_has_both_sides_real_legal_choices_and_exact_records, which searches twelve standard-position turns using CHU_PLAY_LIMITS. It ran beyond 60 s and was stopped as extra debug coverage; this test also belongs in the cost review. No limits, assertions or rules changed. Logs: gs://mshogi-experiments-216675633801/runs/msh-230/ (60-day lifecycle).
<!-- SECTION:NOTES:END -->
