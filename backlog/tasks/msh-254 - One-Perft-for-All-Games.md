---
id: MSH-254
title: One Perft for All Games
status: Ready
assignee: []
created_date: '2026-11-25T12:21:00Z'
labels:
  - core
  - console
  - refactor
  - model:medium
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each game has its own perft and divide API in mshogi-core: chess/perft.rs (u32 depth, no budget), shogi/perft.rs, xiangqi/perft.rs (u8 depth, maximum 3), tori/perft.rs (visit budget 1,000,000 and report types) and tenjiku/perft.rs (budget 100,000 and stop). Chu has none, so the console (chu_session.rs, about 58 lines) and chu/conformance.rs write their own recursion. The console also copies the private chess perft order_key (session.rs). The console visit limit is 1,000,000 for Tori and 100,000 for the others, and tenjiku_session.rs writes 100_000 instead of the constant. Perft is rules evidence, so the counts must not change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic perft and divide with depth limit, visit budget and stop serves all six games in mshogi-core; per-game perft code keeps only what the game needs, such as its node order.
- [ ] #2 Console perft and divide use it for every game with one budget rule; the console Chu recursion and the copied order_key are removed.
- [ ] #3 All perft expected values and conformance results are unchanged; workspace checks pass.
<!-- AC:END -->
