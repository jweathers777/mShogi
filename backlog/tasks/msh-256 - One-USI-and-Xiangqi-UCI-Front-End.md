---
id: MSH-256
title: One USI and Xiangqi UCI Front End
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
labels:
  - console
  - protocol
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7250
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In crates/mshogi-console, usi.rs (731 lines) and xiangqi_uci.rs (632) share about 480 normalized lines: the search worker and mailbox, retire, isready, quit, stop and go handling, number parsing, report and run. Real dialect differences: USI handshake and usinewgame, gameover, go mate, SFEN, byoyomi and bestmove win or resign; Xiangqi UCI side naming and the Hash option. Differences with no recorded reason: only USI checks time and generation before it accepts a result; a bare go searches 1 s in USI but uses fixed limits in Xiangqi; only Xiangqi reuses a position that the new move list extends; USI has no Hash option; Xiangqi has no movestogo. uci.rs (Chess) has its own line reader (bounded_uci_line, a copy of main.rs bounded_line that also ends a line at CR), and the ASCII and control-byte check exists three times.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic protocol worker and loop serves USI and Xiangqi UCI through a dialect trait that holds only real dialect differences; the Chess UCI front end uses the same bounded line reader and byte check.
- [ ] #2 Each difference without a reason is resolved to one behavior and recorded in the notes; console protocol tests pass.
- [ ] #3 Production line counts before and after are recorded.
<!-- AC:END -->
