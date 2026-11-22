---
id: MSH-224
title: Shogi on the Generic Document and Session
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-22T09:57:00Z'
updated_date: '2026-11-22T11:33:00Z'
labels:
  - desktop
  - refactor
milestone: m-6
dependencies:
  - MSH-223
priority: medium
ordinal: 231000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-217 (Shared Game Session) put Chu and Tenjiku on one generic document (ValidatedGameDocument) and session (NativeSession). Shogi (shared.rs, shared_archive.rs) and Xiangqi (shared_xiangqi.rs, shared_xiangqi_archive.rs) still have their own session and archive code, and their session structs are near copies of each other. John (2026-11-22): saved-format compatibility does not matter before release (A-09), so their formats can change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Shogi uses the generic document and session; game-specific code holds only real differences (declarations, settings with analysis defaults, presentation)
- [ ] #2 Old Shogi session and archive code is removed; production line counts before and after are recorded
- [ ] #3 Desktop-session and workspace tests pass; committed Shogi fixtures are regenerated; frontend tests pass; one fresh-agent review
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Shogi first, then Xiangqi, each its own commit on claude/msh-224-shogi-xiangqi-generic.
2. Generic extensions (from the real differences): per-game settings type in NativeGame (Shogi/Xiangqi keep analysis_defaults); optional engine Stop command (Shogi has none); engine_command(result) -> complete command (Shogi engine may declare); GameOperation::Local {command} so the frontend wires SessionOperation::Play{action}/Declare{} stay unchanged; SessionGame hooks for declaration advice and the declaration capability.
3. Shogi NativeGame impl with a modelled work count (like Tori); Shogi undo handle is by value (clone in impl).
4. Replace ShogiSession/ShogiArchive/NativeWork::Admit/Save/ShogiSeed and shogi_snapshot with the generic ones; keep Shogi presentation (pieces, hands, choices with promote completion, events, status, declaration advice) as SessionGame hooks.
5. Regenerate Shogi fixtures (write_frontend_fixtures / write_document_fixtures), run frontend tests.
6. Xiangqi: same, plus exact-endgame evidence (per-game payload extension) and its own presentation.
7. Checks, line counts, review, merge.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Design notes from the MSH-223 session (2026-11-22): the Shogi core fits NativeGame with two hooks: (a) a game without a Stop command (engine_failure_stop becomes optional; reserve skips it), (b) a desktop work model like Tori (no native replay work). Shogi engine choices can be a formal declaration, so the trait should map an engine result to a complete command (engine_command) instead of engine_choice + play. Shogi snapshot adds declaration advice and the declaration capability. Xiangqi adds saved exact-endgame evidence and a native undo list; check whether exact evidence can be a per-game extension of the generic payload.

Shogi behavior differences found (shared.rs, shared_archive.rs on main 12bb952), to unify on the generic behavior unless they are rules: (a) Take back/Undo drop the abandoned branch (generic keeps it); (b) an inadmissible engine choice is saved as an observation with application_error and the game unchanged (generic refuses the observation); (c) Analyze also saves the limits as settings.analysis_defaults; (d) a human Play needs the session resumed in every play mode (generic: only two-humans); after a human move Shogi sets paused=false; (e) engine-turn limits are checked against a fixed list of past limits (compatibility only; drop); (f) board_action capability needs !paused; engine_turn capability differs. Shogi has no Stop command and no runtime context; Game::submit clones the game; Game::prefix replays.

Checkpoint 2026-11-22 (branch claude/msh-224-shogi-xiangqi-generic, commit after 5ca6d2c): library done for Shogi: NativeGame/SessionGame for Shogi (shared_shogi.rs), DocumentSettings trait (per-game settings; Shogi keeps analysis_defaults, validated by a hook), engine_command (Shogi may declare), optional engine Stop, GameOperation::Local (frontend Play/Declare unchanged) and SessionOperation::Shogi {actor, command}; one boxed NativeWork::Game job and NativeCompleted::Runtime for all generic games; generic refuse_completed_work; old ShogiSession/ShogiArchive/Admit/Save removed. Shogi work unit 200 (measured 144 us per command at 319 commands). Next: fix desktop-session tests (shared_capacity_tests, shared_archive_tests, shared_tests, inspection tests), regenerate Shogi fixtures (write_frontend_fixtures, write_document_fixtures), frontend tests, then Xiangqi.

Checkpoint 2026-11-22 (branch claude/msh-224-shogi-xiangqi-generic, pushed): SHOGI DONE. Shogi runs on ValidatedGameDocument/NativeSession (shared_shogi.rs). Lines shared.rs 2378->1712, shared_archive.rs 1834->654 (+ shared_shogi 289; generic modules grew ~460). shared_capacity_tests.rs removed (old Shogi capacity model; generic capacity has Chu/Tenjiku/Tori tests). Shared-session changes taken from Shogi, now for all generic games: preference changes keep current work and version; Resume sets Idle and refuses a concluded game; full observation ledger refuses Analyze/Resume at once; take back after a formal declaration (CommandKind::Terminal); an approved replacement retires a pending admission; engine failure text keeps the engine message; save clamps an exhausted identity floor; analysis target side is checked against the prefix. Shogi-specific now generic hooks: DocumentSettings (validate, with_analysis_defaults: Shogi remembers analysis limits), declaration advice/capability, local Play/Declare operations, optional Stop, engine_command. Accepted differences: Shogi Take back/Undo keep retired branches; an inadmissible engine choice refuses the observation (no application_error); Shogi analysis needs nodes >= 1; Save waits for a pending admission. Shogi document limit 4,194,304 measured (largest valid 2,362,756 units, 1.6 s; forged refused 3.2 s). Checks: desktop-session 311 pass, workspace tests pass, clippy clean, frontend 609 pass with regenerated Shogi fixtures. Found: Chess and Xiangqi frontend fixtures are stale from earlier engine changes (not regenerated here; belongs to MSH-221). NEXT: Xiangqi (shared_xiangqi.rs, shared_xiangqi_archive.rs, shared_xiangqi_verification.rs; exact-endgame evidence; Xiangqi rollback in SharedState).

Scope split (2026-11-22): Xiangqi moved to MSH-226 because it needs more generic extension points (multi-event commands, snapshot views, draw actors, exact evidence).
<!-- SECTION:NOTES:END -->
