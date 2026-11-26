---
id: MSH-255
title: Shared Console Sessions and Test Support
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T17:27:00Z'
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
- [ ] #1 One generic console session serves Tori, Chu and Tenjiku, and Shogi and Xiangqi where the skeleton fits; per-game code holds only notation, board drawing, hands, draw and profile commands and other real differences.
- [ ] #2 All sessions use one error text style, one draw command vocabulary and one export check; shared items live in a shell module; main.rs dispatches once.
- [ ] #3 Console tests pass, updated for unified command names; production line counts before and after are recorded.
- [ ] #4 One tests/support module holds the shared helpers and the test files use it; one success rule (stderr checked or not) applies to all files.
- [ ] #5 Test coverage is unchanged (same tests and assertions); console tests pass; test line counts before and after are recorded.
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
<!-- SECTION:NOTES:END -->
