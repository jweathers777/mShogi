---
id: MSH-227
title: Xiangqi Record Replay Speed
status: Ready
assignee: []
created_date: '2026-11-22T15:18:00Z'
updated_date: '2026-11-25T07:39:00Z'
labels:
  - core
  - performance
milestone: m-6
dependencies: []
priority: low
ordinal: 234000
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
<!-- SECTION:NOTES:END -->
