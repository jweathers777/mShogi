---
id: MSH-227
title: Xiangqi Record Replay Speed
status: Ready
assignee: []
created_date: '2026-11-22T15:18:00Z'
updated_date: '2026-11-26T08:57:00Z'
labels:
  - core
  - performance
  - model:medium
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 6850
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Found in MSH-226 (2026-11-22): replaying a native Xiangqi record costs about 0.6 ms per move at 511 moves (303 ms in release on macOS arm64), about four times Shogi and Tori. The desktop document replays each record several times when it opens or changes, so a document with one long game takes about 2 s to validate and the Xiangqi document work limit had to be 8,388,608 units (largest valid document 5.5 s). Find the cost in mshogi-core xiangqi (Game::submit, repetition history) and reduce it without changing rules.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A measurement of replay time per move at full length, before and after, with the cause named
- [ ] #2 Replay is at least twice as fast, or the task records why not; Xiangqi rules tests and perft are unchanged
- [ ] #3 The desktop Xiangqi work model and document limit are measured again and updated
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Cause analysis (2026-11-22, code reading, not yet profiled): Game::submit (xiangqi/game.rs) (1) serializes the complete record to JSON on every command (GameRecord::encoded_candidate, only to check MAX_GAME_RECORD_BYTES), so replay is quadratic in game length; Tori removed the same cost in MSH-204 by caching per-event encoded lengths; (2) clones the whole game per command (transaction_copy: steps with before/after positions, events, commands, lineage), also linear per command; (3) runs assess_natural_win per board action. Expected order: (1) dominates (about 150 KB of JSON per command near 500 moves). Fix (1) the MSH-204 way first, then measure.

MSH-229 inventory entries 5/9/10 identify this task as the existing Xiangqi implementation work for per-command full-record encoding and transaction/history copies. MSH-232 covers the separate Tori/Chu/Tenjiku resource-admission cuts; MSH-231 covers redundant record metadata. Preserve atomic native command admission and saved-game safety, and measure after the MSH-228/225 desktop simplification.

MSH-209 (2026-11-25): two cross-game follow-ups touch the same Xiangqi code. MSH-250 (Token-Only Native Request Context): each request copies and compares the full command history (game.context() and submit), which makes replay quadratic in all games. MSH-251 (One Repetition Index for All Games): xiangqi/game.rs occurrences scans all earlier identities after each command. If this task changes either in Xiangqi, record it here so that those tasks leave Xiangqi as it is.

John, 2026-11-25: run after MSH-250 and MSH-251, which change the same replay cost. Measure again first; they may already meet this goal.

MSH-250 (Token-Only Native Request Context), 2026-11-26, changed Xiangqi: requests now carry only the shared GameContext token (crates/mshogi-core/src/native/context.rs), so Game::context() no longer copies root, policy and the command ledger and submit no longer compares them. GameUndo is bound by instance and history node (O(1) check); the per-submit copies in GameUndo (after checkpoint, after_commands, after_lineage) and the lineage vector are gone; fork() is infallible. Not changed (left for this task): transaction_copy (whole-game clone per submit), occurrences scan (MSH-251), assess_natural_win per board action. Measured with the extended resource_admission example (now has shogi and xiangqi), Apple M1 Pro release, target/release/examples/resource_admission xiangqi 1000: replay 160.6 ms before, 154-155 ms after (about 155 us per event); admission 91.8 -> 84-91 us; undo 2.2 -> 0.04 us. So the context copy was not the main cost; transaction_copy dominates. Measure again from this state.
<!-- SECTION:NOTES:END -->
