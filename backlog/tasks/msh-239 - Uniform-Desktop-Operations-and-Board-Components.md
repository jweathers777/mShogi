---
id: MSH-239
title: Uniform Desktop Operations and Board Components
status: Ready
assignee: []
created_date: '2026-11-25T08:51:00Z'
updated_date: '2026-11-25T09:15:00Z'
labels:
  - desktop
  - refactor
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The desktop session behind the UI is generic (MSH-217, MSH-223 to MSH-226), but the layer above it is not. Make one operation shape and shared board and piece components for all six games, so each game supplies only its real differences.

Found 2026-11-25:
- SessionOperation and its wire enum (crates/mshogi-desktop-session/src/shared.rs) have per-game variants: ChessPlay, ChessCommand, ChessClaim, Xiangqi, XiangqiCommand, Tori, Chu, Tenjiku, Shogi, Play, and six *Preferences variants. desktop/src/product/native.ts copies them.
- GameBoard.tsx builds the play command differently for each game (some parse the action JSON, some carry the actor, Shogi uses plain play). Xiangqi settings are nested differently from the other games. Chu and Tenjiku use the ToriSide type.
- ShogiPiece, ToriPiece, ChuPiece and TenjikuPiece draw the same piece shape with their own face tables. ChuBoard and TenjikuBoard are near copies.
Saved formats and protocols may change (A-09); update callers and fixtures.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One play, one game-command and one preferences operation serve all six games in the session and the frontend; the per-game variants and the duplicate wire enum are removed, and game data is only in the action or settings payload.
- [ ] #2 One piece component with per-game face tables and one large-board component for Chu and Tenjiku; per-game frontend code holds only real differences (artwork, hands, intersection board, Chess claims).
- [ ] #3 Settings have the same shape in every game; production line counts before and after are recorded; workspace, session and frontend tests pass.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->
