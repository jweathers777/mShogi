---
id: MSH-252
title: Shared Game History and Xiangqi Replay
status: To Do
assignee: []
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T11:46:00Z'
labels:
  - core
  - refactor
  - performance
  - model:large
  - effort:xhigh
milestone: m-6
dependencies:
  - MSH-251
priority: medium
ordinal: 6900
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Tori, Chu and Tenjiku still repeat history ownership, prefix selection, undo, retirement, capacity accounting and commit/rollback code. MSH-250 already introduced the shared GameContext token, removed RuntimeToken and the per-game NEXT_INSTANCE statics, and made fork infallible; build on that state. Remaining drift includes Chu checkpoints, prefix-length checks, separate history-node counters, counter-overflow refusals and Shogi retirement. Consolidate the remaining shell while preserving per-game rules and snapshot data.

Xiangqi still clones commands, events, snapshots, steps and identities in transaction_copy. Measure replay after MSH-251 before changing it, then address any remaining cost in this history work. Preserve atomic command admission and saved-game safety. The old record byte limits, desktop work model and document work limit have been removed; do not restore them. The goal remains measured Xiangqi replay improvement, with the cause and any unmet speed target explained.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic history owner in mshogi-core serves Tori, Chu and Tenjiku; each game supplies only its rules step (legality, effects, repetition rule, conclusions) and its snapshot data; Xiangqi and Shogi use it too, or the notes record why not.
- [ ] #2 A game is copied the same way in every game; u64 counter checks and Resource::Counter are removed where overflow cannot occur.
- [ ] #3 Rules, perft and record tests and the committed record fixtures are unchanged; production line counts before and after are recorded.
- [ ] #4 One review by a fresh agent session.
- [ ] #5 A measurement of replay time per move at full length, before and after, with the cause named
- [ ] #6 Replay is at least twice as fast, or the task records why not; Xiangqi rules tests and perft are unchanged
- [ ] #7 The final desktop Xiangqi record-open and validation paths are measured again with long records; record the results without restoring the removed desktop work model or document limit.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-250 (Token-Only Native Request Context), 2026-11-26, changed the shared shell: one GameContext token (instance, revision) in crates/mshogi-core/src/native/context.rs replaces RuntimeToken, the per-game GameContext structs and the four NEXT_INSTANCE statics; all five games re-export it. fresh() is infallible; advanced() still returns Option so that the existing Capacity(Counter) refusals and the Tenjiku ending reserve (next_context(reserve)) stay unchanged for this task to remove. fork() of Xiangqi/Tori/Chu/Tenjiku and EngineGame::fork are infallible. Shogi now has the token: Game::clone gives a new instance (fresh token, node 0) and Game equality ignores the token. Undo binding still differs: Shogi, Xiangqi and Tori use node = revision of the last command (copies start at node 0); Chu and Tenjiku use a separate next_node counter; Tori and Tenjiku also check a prefix length, Chu keeps a checkpoint in the token, Shogi and Xiangqi consume the token by value. Shogi still has no retire_context (desktop shared_shogi retire is a no-op).

Context carried from MSH-227 (Xiangqi Record Replay Speed); historical observations, not a replacement for the current criteria and dependency fields:
Cause analysis (2026-11-22, code reading, not yet profiled): Game::submit (xiangqi/game.rs) (1) serializes the complete record to JSON on every command (GameRecord::encoded_candidate, only to check MAX_GAME_RECORD_BYTES), so replay is quadratic in game length; Tori removed the same cost in MSH-204 by caching per-event encoded lengths; (2) clones the whole game per command (transaction_copy: steps with before/after positions, events, commands, lineage), also linear per command; (3) runs assess_natural_win per board action. Expected order: (1) dominates (about 150 KB of JSON per command near 500 moves). Fix (1) the MSH-204 way first, then measure.

MSH-229 inventory entries 5/9/10 identify this task as the existing Xiangqi implementation work for per-command full-record encoding and transaction/history copies. MSH-232 covers the separate Tori/Chu/Tenjiku resource-admission cuts; MSH-231 covers redundant record metadata. Preserve atomic native command admission and saved-game safety, and measure after the MSH-228/225 desktop simplification.

MSH-209 (2026-11-25): two cross-game follow-ups touch the same Xiangqi code. MSH-250 (Token-Only Native Request Context): each request copies and compares the full command history (game.context() and submit), which makes replay quadratic in all games. MSH-251 (One Repetition Index for All Games): xiangqi/game.rs occurrences scans all earlier identities after each command. If this task changes either in Xiangqi, record it here so that those tasks leave Xiangqi as it is.

John, 2026-11-25: run after MSH-250 and MSH-251, which change the same replay cost. Measure again first; they may already meet this goal.

MSH-250 (Token-Only Native Request Context), 2026-11-26, changed Xiangqi: requests now carry only the shared GameContext token (crates/mshogi-core/src/native/context.rs), so Game::context() no longer copies root, policy and the command ledger and submit no longer compares them. GameUndo is bound by instance and history node (O(1) check); the per-submit copies in GameUndo (after checkpoint, after_commands, after_lineage) and the lineage vector are gone; fork() is infallible. Not changed (left for this task): transaction_copy (whole-game clone per submit), occurrences scan (MSH-251), assess_natural_win per board action. Measured with the extended resource_admission example (now has shogi and xiangqi), Apple M1 Pro release, target/release/examples/resource_admission xiangqi 1000: replay 160.6 ms before, 154-155 ms after (about 155 us per event); admission 91.8 -> 84-91 us; undo 2.2 -> 0.04 us. So the context copy was not the main cost; transaction_copy dominates. Measure again from this state.

John approved consolidation on 2026-11-26: this task retains the full scope of MSH-252 (One Native Game History Owner), MSH-227 (Xiangqi Record Replay Speed). History ownership and Xiangqi replay copies change the same mechanism; retain before-and-after replay evidence in the shared refactor. Agent labels are model:large, effort:xhigh for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Status is To Do while an unfinished dependency blocks work; otherwise Ready. No implementation has started.
<!-- SECTION:NOTES:END -->
