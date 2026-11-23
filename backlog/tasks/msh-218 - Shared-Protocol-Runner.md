---
id: MSH-218
title: Shared Protocol Runner
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-20T23:26:00Z'
updated_date: '2026-11-23T18:49:00Z'
labels:
  - runner
  - refactor
milestone: m-6
dependencies: []
priority: medium
ordinal: 225000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: merge the copied Shogi (USI) and Xiangqi (UCI0-9) experiment runners in crates/mshogi-runner into one generic implementation, with a per-game part for only the real differences (protocol dialect, notation, rules types, outcome mapping).

Finding (2026-11-21): with game and protocol names replaced by one placeholder, shogi.rs (1,110 lines) and xiangqi.rs (1,356) differ in 398 lines, so about 85% is the same untimed fresh-child-per-turn runner; shogi_match.rs (504) and xiangqi_match.rs (728) differ in 432 lines. The Chess runner (run.rs) is mostly different and is out of scope unless a real shared part shows up. The Shogi and Xiangqi runner tests repeat the same structure.

Approach: list the real differences, design the boundary from them (A-10; generalize only shared semantics), move the untimed runner first, then the persistent clocked match runner if the boundary holds. External protocol behavior, report formats and recorded-report replay do not change (A-09 allows changing internal APIs).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic runner (and, if it fits, one generic clocked match runner) serves Shogi and Xiangqi; per-game code holds only protocol, notation and rules differences; production line counts before and after are recorded
- [ ] #2 Behavior is unchanged: runner tests pass, written once against the shared code where they were copied, and the recorded Shogi and Xiangqi reports in docs/m4 still replay
- [ ] #3 One review by a fresh agent session of the boundary and the migration
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Record current production line counts and protocol differences; preserve concrete configuration/report schemas and native opening/outcome checks. 2. Extract shared traffic/failure/turn types and a generic fresh-process controller and runner/replay lifecycle, with native adapters for notation, admission, terminal notification, Xiangqi failure messages and transcript checks. Consolidate duplicated behavioral tests around that boundary. 3. Apply the same native adapter to the existing shared clocked host and merge duplicated clocked run/replay logic where semantics match, preserving UCI/UCCI conversion and clock/report contracts. 4. Fresh review, recorded-report replay, required Rust checks and targeted process CI on affected platforms; record production line counts, merge, mark Done and clean task outputs.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main 731a8ff. Trace Shogi/Xiangqi runner semantics and preserve recorded report formats and external protocol behavior. Branch codex/msh-218-shared-protocol-runner.

Checkpoint: shared untimed Controller/run/replay and persistent clocked run/replay are implemented on codex/msh-218-shared-protocol-runner. Concrete report/config schemas and native adapter differences remain. Untimed and recorded-report tests: 29 passed; clocked fixture suites: 10 passed (two optional full-engine tests skipped internally because release mshogi was not built). Copied failure tests now share macros across both adapters. Fresh independent untimed and clocked reviews found no actionable defects. Next: validate final shared configuration helpers and tests, run workspace checks, build release console for actual clocked games, targeted process CI on Linux/macOS/Windows, record line counts, integrate and clean.

Production line count (wc-style including comments/blank lines, excluding the unchanged #[cfg(test)] unit module): before at 731a8ff shogi 1110, xiangqi 1356, shogi_match 504, xiangqi_match 654, clocked 327 = 3951; after at d5fe836 shogi 445, xiangqi 735, shogi_match 303, xiangqi_match 452, clocked 328, protocol 908, protocol_match 354 = 3525 (426 fewer, 10.8%). Whole-file totals including the unchanged 74-line unit module: 4025 -> 3599. Shared native data models and runtime traits preserve serialized config/report shapes; native adapters retain dialect/notation, admission, terminal/pending outcome, and Xiangqi strict transcript differences. Final helper/test delta also reviewed without findings. Formatting and workspace Clippy passed. Product commit d5fe836 is pushed. Targeted process CI: https://github.com/jweathers777/mShogi/actions/runs/45242420199 ; workspace debug tests currently compiling.
<!-- SECTION:NOTES:END -->
