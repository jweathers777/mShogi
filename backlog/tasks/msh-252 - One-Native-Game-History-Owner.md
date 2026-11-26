---
id: MSH-252
title: Shared Game History and Xiangqi Replay
status: Done
assignee:
  - '@codex'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T15:30:00Z'
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
- [x] #1 One generic history owner in mshogi-core serves Tori, Chu and Tenjiku; each game supplies only its rules step (legality, effects, repetition rule, conclusions) and its snapshot data; Xiangqi and Shogi use it too, or the notes record why not.
- [x] #2 A game is copied the same way in every game; u64 counter checks and Resource::Counter are removed where overflow cannot occur.
- [x] #3 Rules, perft and record tests and the committed record fixtures are unchanged; production line counts before and after are recorded.
- [x] #4 One review by a fresh agent session.
- [x] #5 A measurement of replay time per move at full length, before and after, with the cause named
- [x] #6 Replay is at least twice as fast, or the task records why not; Xiangqi rules tests and perft are unchanged
- [x] #7 The final desktop Xiangqi record-open and validation paths are measured again with long records; record the results without restoring the removed desktop work model or document limit.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Measure post-MSH-251 release replay and desktop Open/decode on the same 1,000-move walk. Share the event and snapshot owner, rollback, context, copy, reversal, and retirement. Keep rule data and adjudication in each game. Replace Xiangqi full-history copies with append rollback. Check unchanged rules, perft, records, and fixtures; measure the final code, obtain independent review, then integrate.
<!-- SECTION:PLAN:END -->

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

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-252-shared-history at 23a0fd0. Next: measure the post-MSH-251 replay baseline and inspect history ownership.

Baseline on Apple M1 Pro, 23a0fd0: resource_admission xiangqi 1000 (release, median of five), 1,000 moves/59,375 bytes, 153,474.958 us replay (153.475 us/move), 78.291 us last-command admission. Branch codex/msh-252-shared-history.

Checkpoint on codex/msh-252-shared-history: Tori, Chu and Tenjiku share the history owner, prefix selection, rollback, compact undo and request lifecycle. Shogi and Xiangqi share the transaction and authority but retain their specialized allocations: Shogi replays prefixes without storing board snapshots; Xiangqi has several viewable events per command, including root events and claim audits. All game owners now clone their full admitted state directly. Xiangqi history-copy removal alone brought full replay to 127.745 ms and last admission to 8.500 us. Original 59,593-byte desktop document median of five: decode 157.357 ms, product Open 163.023 ms. Added short-circuit terminal testing to the existing Xiangqi legal-action generator; next are final timings, full checks and independent review.

Independent review cleared dd27c8b after two fixes: Tenjiku search PV validation and full-game perft now clone the admitted game directly, with cooperative stop checks. New Xiangqi regression forces a refusal after both a board action and a multi-event control, checks all ledgers, and uses the previous undo chain. Six release Chess perft cases passed, including deep divides. Initial native replay 28.719 ms after short-circuiting terminal checks. Next on codex/msh-252-shared-history: final workspace checks, interleaved benchmark runs, and integration.

Final release timings on Apple M1 Pro, Rust 1.99.1, 23a0fd0 versus dd27c8b. Three alternating pairs, each reporting the median of five replays or eleven admissions. resource_admission xiangqi 1000 generated the same 1,000 board moves and 59,375 native bytes. Replay fell from 154.510 to 27.668 ms (154.510 to 27.668 us/move, 5.58x). Last-move admission fell from 87.375 to 3.917 us. Append rollback eliminates repeated whole-history clones. The remaining large cost was evaluating one-ply win witnesses: local_status generated every legal reply for each candidate. It now ends the same native iterator at the first legal reply. All witness ordering, declared-counter checks and ordinary move legality stay intact. On one fixed 59,593-byte desktop document with the same 1,000 moves, ValidatedGameDocument::<Xiangqi>::decode fell from 153.594 to 28.352 ms (5.42x). DesktopController::queue_product_open through receipt of its installed snapshot fell from 158.427 to 31.248 ms (5.07x). No document limit or desktop work model was added. Production Rust in all changed source files, counting physical lines with comments/blank lines and excluding test-only files/items: 12,345 before, 11,984 after (361 fewer). No fixture or rule/perft/record expectation changed. Two synthetic Tenjiku runtime-exhaustion tests and their test hook were removed with that obsolete refusal. Declared board counters can reach their limits through imports; those typed refusals and tests stay. Routine CI for dd27c8b and the six release deep/ordinary Chess perft cases passed. Final workspace check running.

The reviewed code is on main at 19072f5. Final python3 tools/checks.py run --base origin/main (base 23a0fd0) passed formatting, workspace/all-target Clippy, 1,157 Rust tests (10 ignored by default), TypeScript and 575 frontend tests. The independent release perft target passed all six tests including deep divides. Native Xiangqi board, perft, record, archive and rollback tests retained their expectations. All committed fixtures are identical. An additional 74,172-byte document with 1,000 board actions and 256 rejected count claims (512 audit events) validated in 171.007 -> 28.444 ms and completed product Open in 175.215 -> 32.531 ms on the same machine. Both inputs, probe, timing output, count script and final local check log are at gs://mshogi-experiments-216675633801/runs/msh-252/. No cloud compute or VMs. Estimated GCS cost below US$0.01. Routine CI passed at https://github.com/jweathers777/mShogi/actions/runs/45847286033. All required checks resolved.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Tori, Chu and Tenjiku use one history owner. Shogi and Xiangqi share command transactions, copy isolation and runtime retirement. Xiangqi now rolls back appended state on failure and stops terminal checks at the first legal reply. Native replay of 1,000 moves was 5.58x faster; desktop Open was 5.07x faster. The full check suite, independent perft and fresh review passed. Next is MSH-253 (Shared Record Codec and Incremental Engine Sessions).
<!-- SECTION:FINAL_SUMMARY:END -->
