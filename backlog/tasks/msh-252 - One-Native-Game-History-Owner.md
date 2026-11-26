---
id: MSH-252
title: One Native Game History Owner
status: Ready
assignee: []
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T08:57:00Z'
labels:
  - core
  - refactor
  - model:large
  - effort:xhigh
milestone: m-6
dependencies: []
priority: medium
ordinal: 6900
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Tori, Chu and Tenjiku game.rs repeat the same history and authority shell: RuntimeToken, GameContext, GameRequest, Resource, CapacityUsage, GameUndo, NEXT_INSTANCE, fork, event_prefix, select_prefix, retire_context, record_prefix, capacity_usage, undo, the commit-and-rollback wrapper, and many u64 counter overflow checks (Resource::Counter) that cannot fail in practice. The copies have drifted: Tenjiku retire_context reserves one more counter value, Chu keeps a checkpoint in each undo token, Tori and Tenjiku keep a prefix length. Xiangqi has its own form of the same shell; Shogi and Chess have no runtime token and use Clone, which is why only Xiangqi, Tori, Chu and Tenjiku have Game::fork. Do this after MSH-250 (Token-Only Native Request Context) and MSH-251 (One Repetition Index for All Games), which change the same code.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic history owner in mshogi-core serves Tori, Chu and Tenjiku; each game supplies only its rules step (legality, effects, repetition rule, conclusions) and its snapshot data; Xiangqi and Shogi use it too, or the notes record why not.
- [ ] #2 A game is copied the same way in every game; u64 counter checks and Resource::Counter are removed where overflow cannot occur.
- [ ] #3 Rules, perft and record tests and the committed record fixtures are unchanged; production line counts before and after are recorded.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-250 (Token-Only Native Request Context), 2026-11-26, changed the shared shell: one GameContext token (instance, revision) in crates/mshogi-core/src/native/context.rs replaces RuntimeToken, the per-game GameContext structs and the four NEXT_INSTANCE statics; all five games re-export it. fresh() is infallible; advanced() still returns Option so that the existing Capacity(Counter) refusals and the Tenjiku ending reserve (next_context(reserve)) stay unchanged for this task to remove. fork() of Xiangqi/Tori/Chu/Tenjiku and EngineGame::fork are infallible. Shogi now has the token: Game::clone gives a new instance (fresh token, node 0) and Game equality ignores the token. Undo binding still differs: Shogi, Xiangqi and Tori use node = revision of the last command (copies start at node 0); Chu and Tenjiku use a separate next_node counter; Tori and Tenjiku also check a prefix length, Chu keeps a checkpoint in the token, Shogi and Xiangqi consume the token by value. Shogi still has no retire_context (desktop shared_shogi retire is a no-op).
<!-- SECTION:NOTES:END -->
