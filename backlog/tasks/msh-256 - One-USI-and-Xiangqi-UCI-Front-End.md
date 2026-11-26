---
id: MSH-256
title: One USI and Xiangqi UCI Front End
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:22:00Z'
updated_date: '2026-11-26T18:22:00Z'
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
In crates/mshogi-console, usi.rs (731 lines) and xiangqi_uci.rs (632) share about 480 normalized lines: the search worker and mailbox, retire, isready, quit, stop and go handling, number parsing, report and run. Real dialect differences: USI handshake and usinewgame, gameover, go mate, SFEN, byoyomi and bestmove win or resign; Xiangqi UCI side naming and the Hash option. Differences with no recorded reason: only USI checks time and generation before it accepts a result; a bare go searches 1 s in USI but uses fixed limits in Xiangqi; only Xiangqi reuses a position that the new move list extends; USI has no Hash option; Xiangqi has no movestogo. uci.rs (Chess) has its own line reader (a line reader copied from main.rs that also ends a line at CR), and the ASCII and control-byte check exists three times.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic protocol worker and loop serves USI and Xiangqi UCI through a dialect trait that holds only real dialect differences; the Chess UCI front end uses the same line reader and byte check, without restoring the removed input line cap for records and position commands.
- [ ] #2 Each difference without a reason is resolved to one behavior and recorded in the notes; console protocol tests pass.
- [ ] #3 Production line counts before and after are recorded.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: MSH-268 removes the input line cap for game records and position commands; do not add it back in the shared front end.

MSH-222 (2026-11-26), finding in this scope: ShogiSearchSession and XiangqiSearchSession now live in mshogi-engine/src/server.rs. They could be one SearchSession<G: EngineGame> that uses EngineGame::extends; the Xiangqi session compares events directly, which is cheaper than a committed_prefix replay, so measure before merging.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-256-protocol at 655e63b. Next: compare the protocol paths and record the implementation plan.
<!-- SECTION:NOTES:END -->
