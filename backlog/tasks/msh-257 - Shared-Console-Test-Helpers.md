---
id: MSH-257
title: Shared Console Test Helpers
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
labels:
  - console
  - tests
  - refactor
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 7300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
crates/mshogi-console/tests repeat helpers: the subprocess run helper six times (tori_cli, chu_cli, tenjiku_cli, xiangqi_cli, common_console, shogi_cli), a temporary-directory guard six times (with desktop_persistence), the protocol harness of tests/usi.rs and tests/xiangqi_uci.rs (identical except one message, about 80 lines; tests/uci.rs has a variant), bridge, wait and stop helpers four times, desktop send, command and started helpers three or four times, and about 75 lines of Chess snapshot helpers three times (desktop_analysis, desktop_play, desktop_persistence). tori_selfplay.rs and tenjiku_selfplay.rs share about 180 lines. The success helper requires empty stderr in some files and not in others.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One tests/support module holds the shared helpers and the test files use it; one success rule (stderr checked or not) applies to all files.
- [ ] #2 Test coverage is unchanged (same tests and assertions); console tests pass; test line counts before and after are recorded.
<!-- AC:END -->
