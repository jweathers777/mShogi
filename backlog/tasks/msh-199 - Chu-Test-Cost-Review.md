---
id: MSH-199
title: Chu Test Cost Review
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-19T06:19:00Z'
updated_date: '2026-11-23T20:22:00Z'
labels:
  - handle:chu-test-cost-review
  - chu
  - testing
milestone: m-5
dependencies: []
references:
  - AGENTS.md
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inventory current Chu tests and shared tests that include Chu in console, desktop-session, engine and search. Time each independently in debug/release with two build workers and one active test; cap baseline cases at 65 s and record timeouts honestly. 2. Map slow tests to contract assertions and overlapping coverage. Use measured bounded work for protocol/history checks; retain any test of production default limits in release with an explicit reason if needed. Preserve rules, assertions and actual subprocess coverage. 3. Repeat timings, obtain independent review of the coverage mapping and diff, run required workspace checks and targeted process/release CI, record evidence in task/GCS, integrate and clean.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Observed again during MSH-228 on 2026-11-23, Apple Silicon Mac: cargo test --workspace --locked -j 2 -- --test-threads=2 stopped in chu_internal (2 passed, 4 failed, 30.48 s). Failures: actual_child_executes_complete_chu_choices_for_full_known_history_and_purpose (Timeout: internal engine response/exit watchdog expired at default depth64/300000 nodes); actual_controller_readonly_analysis_save_reopen_and_stale_identity_retirement (no score); actual_engine_turns_for_both_player_assignments_admit_observation_then_action and real_historical_analysis_saves_exact_prefix_and_reopens_stopped_without_future (only one board ply, expected two). No test budgets/assertions or watchdog changed in MSH-228. Removed document-capacity measurement tests no longer apply after minimal-document refactor.

Debug failure log: gs://mshogi-experiments-216675633801/runs/msh-228/workspace-debug.log (60-day lifecycle). Source commit 6102123; subsequent changes were GUI observer assertions, comments and task notes.

MSH-230 (source 1b75415), Apple Silicon Mac with 2 build jobs/2 test threads: same four chu_internal watchdog failures reproduced (30.49 s), while all 1182 workspace release tests pass. An additional debug --no-fail-fast run reached mshogi-engine/tests/chu_protocol.rs::bounded_standard_progression_has_both_sides_real_legal_choices_and_exact_records, which searches twelve standard-position turns using CHU_PLAY_LIMITS. It ran beyond 60 s and was stopped as extra debug coverage; this test also belongs in the cost review. No limits, assertions or rules changed. Logs: gs://mshogi-experiments-216675633801/runs/msh-230/ (60-day lifecycle).

Started from main 7754591. Measure current Chu test coverage and debug/release costs before changing budgets. The referenced docs/testing/verification-levels.md is absent; inspect its removal and current AGENTS.md for applicable verification rules. Branch codex/msh-199-chu-test-cost.

Checkpoint: baseline timing driver runs selected test binaries serially with a 65 s process-group timeout. Scope includes all desktop-session tests and engine/console unit tests conservatively, dedicated Chu integration/search tests, and shared multi-game timing/experiment/protocol tests that include Chu. Original debug console failures measured individually at 15.02-15.10 s (child watchdog), while current desktop tests completed below 60 s; the old Chu capacity test was removed by prior tasks. No product/test changes yet. Source 7754591; logs/timing script in runs/msh-199. Next finish debug/release baseline, then compare bounded contract budgets with retaining normal default Resume coverage in release. Current verification rules are AGENTS.md; deleted verification-levels.md was read from 4752743 parent and its removal confirmed as MSH-215 cleanup.

Completed before-debug inventory on Darwin arm64 (single active test): 299 selected tests, 291 passed, 4 failed, 3 pre-existing ignored evidence/benchmark generators, 1 timeout at 65 s. Dedicated Chu search tests all completed below 60 s; no search/rules change is needed. The sole timeout is engine chu_protocol::bounded_standard_progression_has_both_sides_real_legal_choices_and_exact_records (12 searches at depth64/300000 nodes). Standard-root internal-search probes at depth1 with 256/512/1024-node caps all completed depth1 in 38 nodes. Proposed 512-node contract budget will be validated against unchanged assertions across all tested histories; retain default Resume path in a release-only test, plus a bounded debug version sharing its assertion body. Release baseline building; no test edits yet.

Coverage mapping prepared (test-only diff): (1) console actual_child... retains both purposes, offered/declined draw history, full context equality, native legality, completed iteration and score; budget depth1/512 replaces unrelated playing depth. (2) readonly analysis retains all seven stale-identity probes, read-only result, save/open, selection/undo/cancel/cross-game retirement; only completed-search budget changes. (3) historical analysis retains two real board actions, exact historical target and live prefix, save/open stopped with no results, stale rejection; its duplicated default-budget setup uses bounded work. (4) original both-player-assignment Resume test retains every assertion and normal production budget in release, with an added submitted-limit assertion; a bounded debug test calls the same assertion body. The production budget is intrinsically what that one test checks, so it remains release-only. (5) inline twelve-ply engine progression retains all twelve real searches, alternating sides, native legality/result validation, complete records, conclusion and observation JSON roundtrip; depth1/512 suffices for this contract, while default-budget completion is covered by (4). (6) repeated file wait loops now use one helper with the same completion/failure/deadline assertions. All search tests, native fixtures, rule assertions, process watchdogs and product defaults stay unchanged.

Fresh required review (review_199, diff vs7754591 plus task/mapping) found no actionable defects: completed native-search, identity, legality, history/persistence/replay assertions retained; original production Resume path intact in release; bounded debug path shares the assertion body; wait helper preserves success/failure/deadline checks; no rules/product code changed. Early after-debug timings: actual child 0.085 s, readonly analysis 0.146 s, historical analysis 0.192 s (each formerly failed at15 s); bounded both-player assignment 0.139 s. Full after timing and workspace verification still running.

After-debug complete at e2e0da3: 300 selected tests, 296 passed, 4 ignored, no failures/timeouts. Fourth ignore is the explicitly release-only normal Resume test; bounded both-side test passed. Engine twelve-ply progression: >65 s timeout ->0.705 s; actual child history/purpose:15.024 s failure ->0.085 s; readonly analysis:15.050 s failure ->0.146 s; historical analysis:15.098 s failure ->0.192 s; bounded turn assignment0.139 s. Slowest selected debug test18.917 s (unchanged Xiangqi fixture, included conservatively in desktop inventory). After-release timing underway. Required targeted process CI: https://github.com/jweathers777/mShogi/actions/runs/45258165311 .

All four timing inventories complete; full per-test table is runs/msh-199/timing-comparison.json, to be uploaded at the task GCS path. Before debug:299 selected/291 pass/4 fail/1 timeout/3 ignored; before release:299/296 pass/3 ignored. After debug:300/296 pass/4 ignored; after release:300/297 pass/3 ignored. Existing ignores are manual frozen-executable capture, timed-game evidence capture, and representation benchmark, not ordinary workspace checks. New debug-only ignore is the normal production-budget Resume test; release time4.353 ->4.292 s, assertions preserved. Other release changes: child purpose/history4.434 ->0.024 s; readonly analysis2.325 ->0.049 s; historical analysis4.368 ->0.074 s; twelve-ply progression28.831 ->0.138 s. Bounded both-side test0.139 s debug/0.081 s release. All active measured tests pass after the change; maximum measured after-debug18.917 s and after-release4.292 s. Full workspace debug/release, fmt and Clippy now running. No cloud compute/VMs used; GCP compute cost $0.
<!-- SECTION:NOTES:END -->
