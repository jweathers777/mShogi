---
id: MSH-239
title: Uniform Desktop Operations and Board Components
status: Done
assignee:
  - '@claude'
created_date: '2026-11-25T08:51:00Z'
updated_date: '2026-11-26T08:17:00Z'
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
- [x] #1 One play, one game-command and one preferences operation serve all six games in the session and the frontend; the per-game variants and the duplicate wire enum are removed, and game data is only in the action or settings payload.
- [x] #2 One piece component with per-game face tables and one large-board component for Chu and Tenjiku; per-game frontend code holds only real differences (artwork, hands, intersection board, Chess claims).
- [x] #3 Settings have the same shape in every game; production line counts before and after are recorded; workspace, session and frontend tests pass.
- [x] #4 One review by a fresh agent session.
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

Line counts (production, main f01c74c -> branch): desktop-session src without *tests.rs 12,550 -> 11,882; Tauri main.rs 1,499 unchanged; frontend ts/tsx without tests, fixtures and src/testing 2,428 lines / 189,924 bytes -> 2,310 / 173,499 (frontend lines are long; bytes -8.6%); frontend CSS 1,002 -> 953. Total -835 lines.
User-visible changes (accidental drift unified to the most common form): (1) Shogi and Tori promotion choice is the shared modal dialog (eyebrow COMPLETE YOUR MOVE, 'Choose promotion', capitalized button text) instead of the inline panel; Xiangqi uses the same dialog (no Xiangqi action has completions). (2) Shogi, Tori and Xiangqi board help is one help line; route controls use the shared 'Selected route: ... No action submitted.' wording (only Chu and Tenjiku have routes). (3) Chu, Tori and Tenjiku documents remember the last analysis limits like Chess, Shogi and Xiangqi (settings have analysis_defaults; initial values = the play limits shown before). (4) Shogi analysis form minimum nodes is 1 like the session already required. (5) Xiangqi selected-piece status uses the lowercase native label ('Selected a0, Red rook'), as its square labels already did. (6) Shogi pieces and hand pieces get the native label as tooltip like Tori. (7) Chu pieces fill the square like Tenjiku (no 94% scale); Tori two-glyph faces 2 px lower; Shogi promoted lettering #8b2d25 like the other games; Chu coordinates 34 px like Tenjiku; one completed-move colour for Chu and Tenjiku. (8) Chess pieces use the native owner colour classes (same colours). No change of what a player can do; the session now refuses a command for a side that no local human owns (the frontend never sent one).
Checks 2026-11-26: fmt; clippy workspace and desktop-session all features; workspace tests 1,156 passed (10 ignored); desktop-session all features 197 passed; frontend tsc, 575 vitest passed (was 586: piece tests merged into Piece.test, dead NativeBoard adapter tests removed; their behaviors stay covered by ShogiBoard/SquareBoard tests), normal and verification builds; desktop scripts 177 Python tests passed. Browser check (Vite + mocked Tauri invoke, real fixtures): Chess, Shogi (promotion dialog), Tori, Xiangqi, Chu (Lion route overlay and footer), Tenjiku (all 44 faces) render; flip sends one preferences operation; no console errors. Packaged GUI not run.

Review (fresh subagent, 2026-11-26, given the diff and the task): no defects. It confirmed that actor derivation is stricter than before (Tori, Chu and Tenjiku took a client actor without an ownership check), that paused/historical checks, analysis bounds, duplicate-field refusal, settings defaults and frontend payloads match the Rust serde shapes. Fixed from its findings: gui-smoke Tenjiku evidence read dataset.face; dead .piece.white/.black CSS selectors; chess glyphs passed only for Chess; Chess play parse error uses Display; one viewed-history exclusion list; ActiveDocument::kind uses profile().game(); Xiangqi label computes the kind once; a duplicate actor-parser test removed; stale ToriBirdPiece comment. Accepted risks, noted: (a) the generic Command path also admits Chess OfferDraw/AcceptDraw/DeclineDraw for the local player (native admission decides; the frontend sends only claims and resign); (b) a malformed play/command/preferences payload is refused at admission instead of at parse time, like an illegal move already was; (c) additional visible change: the selected-route overlay now also draws on the synthetic verification board. Left for MSH-264 (One Frontend Game Table and Wire Types): SyntheticApp re-derives files/ranks like TestBoard's displayedSquares; NativeBoardSnapshot and nativeSnapshotIsNewer still live in NativeBoard.tsx (the component is gone). Remaining per-game dispatch chains in shared_inspection.rs and shared_transaction.rs and the per-variant ActiveDocument accessors can use with_game!; recorded here as a follow-up candidate.

Final checks after merging main (b594c80; main changed only a task file): fmt; clippy workspace and desktop-session all features; workspace tests 1,155 passed (10 ignored); desktop-session all features 196 passed; frontend tsc, 575 vitest, normal and verification builds; desktop scripts 177 Python tests. CI: Rust workspace passed on the branch; Desktop host checks (frontend check/test, Tauri fmt/clippy/test incl. desktop-verification) passed on Linux, macOS and Windows (runs 45778186673 and 45779854867). Packaged GUI journey not run (release-only). No cloud job or spending.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
One operation shape and shared board components for all six games. The session operation is game-neutral: Play (the opaque action of a presented choice), Command (a native command payload) and Preferences (document settings), plus the common controls; the session derives the actor and refuses a command for a side that no local human owns. The per-game variants, the OperationWire duplicate, GameOperation and the custom deserializer are gone; duplicate JSON fields are refused in SharedDesktopCommand::from_json. One GameSettings {play_mode, orientation, analysis_defaults} replaces DesktopSettings, ShogiSettings, XiangqiSettings, the Tori/Chu/Tenjiku aliases and NativeDesktopSettings; limits, new records and record conversion moved to NativeGame and one with_game! dispatch replaces the per-game if-chains. The frontend has the same three operations and one settings shape; one Piece component with per-game face tables replaces four piece components; one SquareBoard serves Chess, Chu and Tenjiku (ChuBoard, TenjikuBoard and the NativeBoard adapter are gone); Board, ShogiBoard and XiangqiBoard share one footer, completion dialog and square label. Shogi and Xiangqi native labels now carry the wording the frontend used. Fixtures were regenerated (only settings and these labels change); GUI scripts, their tests and documents were updated. Production lines -835 (session 12,550 -> 11,882; frontend ts/tsx 2,428 -> 2,310 lines, -8.6% bytes; CSS 1,002 -> 953). User-visible changes are listed in the notes (modal promotion dialog for Shogi/Tori, uniform help line, analysis limits remembered in every game, small artwork drift). Checks, three-platform desktop host CI and a fresh-agent review (no defects; nits fixed) pass. Follow-up candidates are in the notes for MSH-264 (One Frontend Game Table and Wire Types) and for the remaining per-game dispatch in the session.
<!-- SECTION:FINAL_SUMMARY:END -->
