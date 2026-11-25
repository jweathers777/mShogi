---
id: MSH-253
title: One Compact Record Codec
status: Ready
assignee: []
created_date: '2026-11-25T12:21:00Z'
labels:
  - core
  - persistence
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
tori/record.rs, chu/record.rs and tenjiku/record.rs in mshogi-core repeat the same codec: strict() (identical in all three), replay of the commands with stop checks, the re-encode-and-compare check, from_json_with_stop, to_game, to_game_with_undo, GameDelta suffixes (Tori and Chu), Serialize and Deserialize, and a RecordError enum that native.rs maps three times (Stopped, Capacity). Xiangqi and Shogi records follow a similar pattern. Only the root data and the command type differ by game. Do this after MSH-252 (One Native Game History Owner).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic compact record codec in mshogi-core serves Tori, Chu and Tenjiku, including a suffix delta for each; per-game code holds only its root data and command type; Xiangqi and Shogi use it too, or the notes record why not.
- [ ] #2 native.rs maps one record error type; committed records, fixtures and envelope test outputs are byte-identical before and after.
- [ ] #3 Production line counts before and after are recorded; workspace checks pass.
<!-- AC:END -->
