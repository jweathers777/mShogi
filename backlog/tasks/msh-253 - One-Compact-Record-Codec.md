---
id: MSH-253
title: Shared Record Codec and Incremental Engine Sessions
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T15:42:00Z'
labels:
  - core
  - persistence
  - refactor
  - engine
  - model:large
  - effort:high
milestone: m-6
dependencies:
  - MSH-252
priority: medium
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
One Compact Record Codec:
tori/record.rs, chu/record.rs and tenjiku/record.rs in mshogi-core repeat the same codec: strict() (identical in all three), replay of the commands with stop checks, the re-encode-and-compare check, from_json_with_stop, to_game, to_game_with_undo, GameDelta suffixes (Tori and Chu), Serialize and Deserialize, and a RecordError enum that native.rs maps three times (Stopped, Capacity). Xiangqi and Shogi records follow a similar pattern. Only the root data and the command type differ by game.

Incremental Engine Sessions for Every Game:
The engine keeps a retained native game between turns (incremental session: mshogi-core native/incremental.rs and mshogi-engine incremental.rs) only for Shogi (MSH-189), Tori (MSH-204) and Chu (MSH-213); each was added during the strength work of that game. Xiangqi, Tenjiku and Chess replay the full record on each request. The Tenjiku harness (MSH-214) will play long games through the same route.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic compact record codec in mshogi-core serves Tori, Chu and Tenjiku, including a suffix delta for each; per-game code holds only its root data and command type; Xiangqi and Shogi use it too, or the notes record why not.
- [ ] #2 native.rs maps one record error type; committed records, fixtures and envelope test outputs are byte-identical before and after.
- [ ] #3 Production line counts before and after are recorded; workspace checks pass.
- [ ] #4 Incremental sessions are a capability of the generic engine adapter; each game that has a record suffix uses them without per-game session code; a game left out has a recorded reason.
- [ ] #5 Engine protocol and incremental tests cover each game; the replay cost per request at full length is measured before and after for Xiangqi and Tenjiku.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Context carried from MSH-260 (Incremental Engine Sessions for Every Game); historical observations, not a replacement for the current criteria and dependency fields:
MSH-222 (2026-11-26), finding in this scope: EngineGame (mshogi-engine adapter.rs) has KIND, record, extends (record prefix) and valid_limits. The incremental Adapter impls (ShogiAdapter, turn_adapter for Tori and Chu) can become one impl over EngineGame where Record: IncrementalRecord<Game = Game>; server.rs dispatch_request still has three per-game arms and execute_retained::<G> is already generic.

John approved consolidation on 2026-11-26: this task retains the full scope of MSH-253 (One Compact Record Codec), MSH-260 (Incremental Engine Sessions for Every Game). The shared codec provides the record suffix used by incremental engine sessions; verify both ends together. Agent labels are model:large, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-253-record-codec at a63fa47. Next: baseline evidence, codec and retained adapter, then checks and review.
<!-- SECTION:NOTES:END -->
