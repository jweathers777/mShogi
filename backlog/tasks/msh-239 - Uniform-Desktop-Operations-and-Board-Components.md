---
id: MSH-239
title: Uniform Desktop Operations and Board Components
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T08:51:00Z'
updated_date: '2026-11-26T07:42:00Z'
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Checked 2026-11-26 on main f01c74c: findings still apply (MSH-222 changed only engine glue).
Production lines before (src, no *tests.rs / *.test.*, no fixtures): desktop-session 12,550; Tauri main.rs 1,499; frontend ts/tsx 2,428 lines (189,924 bytes; lines are long); frontend CSS 1,002.
1. Session: SessionOperation keeps only game-neutral kinds: Play {action: opaque choice id}, Command {command: native command JSON}, Preferences {settings JSON}, plus Resign, Undo, TakeBack, Inspect, ReturnLive, Analyze, Cancel, Resume, Retry and the Rust-only Replace. The session parses the payload for the active game (SessionGame::play), derives the actor (local actor, or the game's command_actor for draw responses) and checks one ownership rule for all games. Remove the per-game variants, GameOperation, the OperationWire duplicate and the custom deserializer; duplicate JSON fields are refused in SharedDesktopCommand::from_json.
2. Settings: one GameSettings<Side> {play_mode, orientation, analysis_defaults} for all six games (DocumentSettings trait, ShogiSettings, XiangqiSettings, DesktopSettings and the per-game aliases go); the snapshot has document.settings with the same shape in every game (NativeDesktopSettings goes). Tori, Chu and Tenjiku now remember analysis limits like the other games.
3. Frontend: Operation has the same three game-neutral kinds; GameBoard, GameActions and flip build them once. One Piece component with per-game face tables (Shogi, Tori, Chu, Tenjiku); one LargeBoard for Chu and Tenjiku; merged piece and large-board CSS.
4. Frontend board controls: one footer (help line, route controls, live status), one completion dialog and one square-label helper for Board, ShogiBoard and XiangqiBoard; Chess uses the native piece path; remove the Board hands group, the NativeBoard guards that only tests reach and the help portal.
5. Update tests (keep checks of kept behavior), console tests, examples, fixtures with their generators, GUI scripts and their Python tests, docs.
6. Checks (fmt, clippy incl. desktop-session all features, workspace and session tests, frontend tsc/test/build, script tests), browser check of the six boards, CI on affected OS, line counts after, fresh-agent review, integrate.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-209 (2026-11-25), findings that fall under AC2 and AC3 (no scope change): Board.tsx (47-127), product/ShogiBoard.tsx (36-114) and product/XiangqiBoard.tsx (20-108) repeat the same board controls (square list, help text, route controls, square labels, inspection rings, completion dialog) and have drifted (modal or inline dialog, label wording, help through a portal or inline). Chess still goes through the old colour-piece path of Board.tsx (GameBoard.tsx converts pieces); the hands group of Board.tsx and the Xiangqi guard and Tori hand filter of NativeBoard.tsx have only test callers. In desktop-session, ResourceScope (resource.rs) repeats GameKind plus Application. MSH-264 (One Frontend Game Table and Wire Types) runs after this task and covers game tables, wire types and test helpers.

Checkpoint 2026-11-26 (branch claude/msh-239-uniform-desktop): Rust session part done (one SessionOperation shape, GameSettings for all games, NativeGame limits/new_record/record_of, with_game! dispatch, chu/tori/tenjiku_document alias modules removed). Session tests compile and pass except the committed GUI fixture documents (need regeneration: Tori/Chu/Tenjiku settings now have analysis_defaults). Next: frontend operations and settings, piece/board components, fixtures, scripts, docs.

Checkpoint 2026-11-26 (ed8860c, branch claude/msh-239-uniform-desktop): session operations/settings done; Shogi/Xiangqi native piece labels use frontend wording; GUI documents and frontend fixtures regenerated (only settings and these labels change, checked by JSON comparison). Frontend done: Operation = play/command/preferences + common controls; GameSettings one shape; Piece.tsx (one tile, per-game face tables) replaces ShogiPiece/ToriPiece/ChuPiece/TenjikuPiece; SquareBoard (Chess, Chu, Tenjiku) replaces ChuBoard/TenjikuBoard/NativeBoard adapter; BoardControls (footer, completion dialog, point label) shared by Board, ShogiBoard, XiangqiBoard. Frontend tsc, 575 vitest tests and both builds pass. Next: gui-smoke.py/test_gui_smoke.py selectors and settings shape, docs, Tauri build, full checks, browser check of six boards, CI, line counts, review, integrate.
<!-- SECTION:NOTES:END -->
