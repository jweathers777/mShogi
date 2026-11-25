---
id: MSH-255
title: One Console Session Shell
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
labels:
  - console
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
crates/mshogi-console has one interactive session per game. tori_session.rs, chu_session.rs and tenjiku_session.rs (410, 450 and 441 lines) share most of their code (Chu and Tenjiku about 350 of 440 normalized lines): input handling, position, legal, move, undo, record, analyze, submit, restore, board printing and helpers. shogi_session.rs and xiangqi_session.rs use the same skeleton and have drifted: Debug-format errors ({e:?}) instead of the bounded diagnostic text, different whitespace trimming, different draw command names (offer/accept/decline against offer-draw), different hand printing, and Shogi move does not check that the record can still be exported (the others check it in two different ways). Shared items (MAX_LINE_BYTES, Next, OperationalStop) live in the Chess session.rs, and main.rs has six identical dispatch arms. Core API names also differ (parse_action against parse_local_action, royal_squares against royals). Do this after MSH-254 (One Perft for All Games).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic console session serves Tori, Chu and Tenjiku, and Shogi and Xiangqi where the skeleton fits; per-game code holds only notation, board drawing, hands, draw and profile commands and other real differences.
- [ ] #2 All sessions use one error text style, one draw command vocabulary and one export check; shared items live in a shell module; main.rs dispatches once.
- [ ] #3 Console tests pass, updated for unified command names; production line counts before and after are recorded.
<!-- AC:END -->
