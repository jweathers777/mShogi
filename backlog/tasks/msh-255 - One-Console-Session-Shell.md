---
id: MSH-255
title: Shared Console Sessions and Test Support
status: Done
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T17:50:00Z'
labels:
  - console
  - refactor
  - tests
  - model:large
  - effort:high
milestone: m-6
dependencies:
  - MSH-254
priority: medium
ordinal: 7200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
One Console Session Shell:
crates/mshogi-console has one interactive session per game. tori_session.rs, chu_session.rs and tenjiku_session.rs (410, 450 and 441 lines) share most of their code (Chu and Tenjiku about 350 of 440 normalized lines): input handling, position, legal, move, undo, record, analyze, submit, restore, board printing and helpers. shogi_session.rs and xiangqi_session.rs use the same skeleton and have drifted: Debug-format errors ({e:?}) instead of the bounded diagnostic text, different whitespace trimming, different draw command names (offer/accept/decline against offer-draw), different hand printing, and Shogi move does not check that the record can still be exported (the others check it in two different ways). Shared items (MAX_LINE_BYTES, Next, OperationalStop) live in the Chess session.rs, and main.rs has six identical dispatch arms. Core API names also differ (parse_action against parse_local_action, royal_squares against royals).

Shared Console Test Helpers:
crates/mshogi-console/tests repeat helpers: the subprocess run helper six times (tori_cli, chu_cli, tenjiku_cli, xiangqi_cli, common_console, shogi_cli), a temporary-directory guard six times (with desktop_persistence), the protocol harness of tests/usi.rs and tests/xiangqi_uci.rs (identical except one message, about 80 lines; tests/uci.rs has a variant), bridge, wait and stop helpers four times, desktop send, command and started helpers three or four times, and about 75 lines of Chess snapshot helpers three times (desktop_analysis, desktop_play, desktop_persistence). tori_selfplay.rs and tenjiku_selfplay.rs share about 180 lines. The success helper requires empty stderr in some files and not in others.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 One generic console session serves Tori, Chu and Tenjiku, and Shogi and Xiangqi where the skeleton fits; per-game code holds only notation, board drawing, hands, draw and profile commands and other real differences.
- [x] #2 All sessions use one error text style, one draw command vocabulary and one export check; shared items live in a shell module; main.rs dispatches once.
- [x] #3 Console tests pass, updated for unified command names; production line counts before and after are recorded.
- [x] #4 One tests/support module holds the shared helpers and the test files use it; one success rule (stderr checked or not) applies to all files.
- [x] #5 Test coverage is unchanged (same tests and assertions); console tests pass; test line counts before and after are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Use a shared line parser, command session, record transaction and export check, with native adapters for game-specific commands and presentation. 2. Extract the test process, protocol, bridge, desktop, record and self-play helpers, preserving all tests and assertions. 3. Compare test inventories and line counts, run the console and workspace checks and affected CI, fix independent review findings, then integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John approved consolidation on 2026-11-26: this task retains the full scope of MSH-255 (One Console Session Shell), MSH-257 (Shared Console Test Helpers). Session consolidation and test-support consolidation touch the same console tests; retain all assertions. Agent labels are model:large, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-255-console-sessions from c7e99bb. Next: consolidate the shell and test helpers, compare coverage, review, and run checks.

Baseline c7e99bb has 4,680 Rust source lines under crates/mshogi-console/src and 9,345 under tests including support.

Checkpoint on codex/msh-255-console-sessions: all six shells share line parsing and record dispatch. Five games share the candidate, move batch, undo and command session; Chess uses its own history and the common export check. All seven focused shell test binaries passed. Next: finish test support, compare coverage, fresh review, workspace and targeted process CI.

Shared test support now holds console execution and success checks, protocol children, temporary directories, desktop/bridge helpers, Chess snapshots and both self-play harnesses. Focused Clippy passes. Next: inspect preserved test coverage, finish independent review and run workspace and platform checks on codex/msh-255-console-sessions.

Independent review found repeated Shogi history replay, an inconsistent record separator, three remaining success checks, two duplicate desktop waits and stale Xiangqi help documentation. Fixed all findings. Native undo tokens now roll back only an uncommitted batch, including export failures. First 90 plies of msh187-round2-failed-0002.usi as individual human shell moves on the Mac, debug: reviewer measured 3.70 s before the fix. Median of three after the fix was 0.092 s versus 0.048 s on the original binary, with exactly equal final records. New code checks export on each move. Source lines 4680 to 4144 and test lines 9345 to 8423 (all .rs under src and tests). All 164 original integration and five unit tests retain their names, including the release-only Chu test. Clippy passes. Next: tracked workspace checks and three-platform process CI, integrate, close and clean.

Final Mac check on 0577740: CARGO_BUILD_JOBS=2 RUST_TEST_THREADS=2 python3 tools/checks.py run --base origin/main passed workspace formatting, Clippy and all default tests (1,162 passed, 10 pre-existing ignored). Console accounted for 168 passes and one debug-only skip of a release test. Manual duplicate rollback checks for Chess, Shogi and Xiangqi passed. Backlog doctor, guide links and git diff --check passed. Waiting for GitHub release process suite run 45868580340 on Ubuntu 24.04, macOS 15 and Windows 2022, then integrate codex/msh-255-console-sessions and clean.

Verified code 0577740, integrated through 371c84e on main. GitHub Actions https://github.com/jweathers777/mShogi/actions/runs/45868580340 ran cargo test --release --locked -p mshogi-engine -p mshogi-runner -p mshogi-console --tests on Ubuntu 24.04, macOS 15 and Windows 2022. All three passed, including the production Chu case skipped in debug. No Cloud Batch or Cloud Build jobs, no GCP cost and no GCS run artifacts.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
All six games share shell input, record handling and export checks. Five games use one session for atomic moves and undo. Xiangqi uses offer-draw, accept-draw and decline-draw. Console tests share process, protocol, desktop and self-play helpers. Rust source fell from 4,680 to 4,144 lines and test code from 9,345 to 8,423. All 169 original console test cases remain. Workspace checks passed locally. Release process tests passed on Linux, macOS and Windows. Independent review findings are fixed. MSH-256 (One USI and Xiangqi UCI Front End) is first in the eligible Ready queue. MSH-243 has other open prerequisites.
<!-- SECTION:FINAL_SUMMARY:END -->
