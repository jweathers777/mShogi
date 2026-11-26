---
id: MSH-255
title: Shared Console Sessions and Test Support
status: To Do
assignee: []
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T11:46:00Z'
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

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John approved consolidation on 2026-11-26: this task retains the full scope of MSH-255 (One Console Session Shell), MSH-257 (Shared Console Test Helpers). Session consolidation and test-support consolidation touch the same console tests; retain all assertions. Agent labels are model:large, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Status is To Do while an unfinished dependency blocks work; otherwise Ready. No implementation has started.
<!-- SECTION:NOTES:END -->
