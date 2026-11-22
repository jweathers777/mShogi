---
id: MSH-226
title: Xiangqi on the Generic Document and Session
status: Done
assignee:
  - '@claude'
created_date: '2026-11-22T11:33:00Z'
updated_date: '2026-11-22T16:01:00Z'
labels: []
milestone: m-6
dependencies:
  - MSH-224
priority: medium
ordinal: 233000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-223 and MSH-224 put Tori and Shogi on the generic document (ValidatedGameDocument) and session (NativeSession). Xiangqi (shared_xiangqi.rs, shared_xiangqi_archive.rs) still has its own session and archive in the old journal design. Xiangqi needs more extension points than Shogi: one command can produce several events (Undo and take back work at command boundaries); viewing an event inside an atomic command shows a history snapshot, not a game; command actors depend on the command (draw offer and reply); exact-endgame evidence is saved beside the observations; presentation has count advice, terminal actions and pending adjudication. John (2026-11-22): saved-format compatibility does not matter before release.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Xiangqi uses the generic document and session; game-specific code holds only real differences (multi-event commands, draw actors, exact evidence, presentation)
- [x] #2 The old Xiangqi session and archive code and the old admission rollback in SharedState are removed; production line counts before and after are recorded
- [x] #3 Desktop-session and workspace tests pass; Xiangqi fixtures regenerated; frontend tests pass; one fresh-agent review
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Generic extensions: NativeGame hooks for multi-event commands (rewind/undo/take-back targets at command starts); per-command actor hook (draw offer/reply actors); a per-game evidence ledger in the payload for Xiangqi exact-endgame evidence (shares the 256 observation slots); Xiangqi presentation hooks (count advice, terminal actions, pending adjudication, intersections board).
2. History view: depends on John's decision (see notes).
3. Xiangqi NativeGame/SessionGame impls; remove XiangqiSession/XiangqiArchive/XiangqiDraft/XiangqiWork and the SharedState rollback; collapse lib.rs per-variant arms into one helper.
4. Tests, fixtures (write_frontend_fixtures Xiangqi set, already stale per MSH-221), frontend tests, measured work limit, review, merge.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Question for John (2026-11-22): one Xiangqi command can produce several events (for example a move, then an automatic warning or conclusion). Today the history view can stop between those events (it then shows a HistorySnapshot and refuses analysis there). The generic session views only command boundaries (a full game). Options: (A, recommended) view only at command boundaries: the board between those events is the same position, so little is lost and Xiangqi uses the generic view unchanged; (B) keep views inside a command: the generic session needs a second view type through all presentation hooks.

John (2026-11-22, in conversation): priority is one unified document and session for all games; standard file formats (PGN, KIF/CSA, WXF) are later adapter tasks, not now. History view question: he never wanted internal events shown as steps, so Xiangqi history stops at command boundaries (option A).

Checkpoint 2026-11-22: library builds (all features, clippy clean). Xiangqi runs on ValidatedGameDocument/NativeSession (shared_xiangqi.rs; shared_xiangqi_archive.rs removed; SharedState rollback removed). Generic additions: evidence ledger (NativeGame::Evidence, GameControl::Evidence, counts with observations), last_command_start, SessionGame take_back_target/command_actor/finish_snapshot, TelemetryView status label, internal SessionOperation::XiangqiCommand. History and presented events are per command. Next: tests, fixtures, measurement of XIANGQI_DOCUMENT_WORK (placeholder 4,194,304), review.

Review (fresh subagent, 2026-11-22): no unintended Xiangqi rules change. Fixed: (1) Moves-list entries now open at their command's end (moves.rs xiangqi_projection), test added; (2) pending adjudication counts as concluded for the session (no Resign probe in reserve, no engine start), so such games save, open and admit their final move; (3) saved evidence is read raw, counted with observations before parsing, parsed one entry at a time with stop checks and charged (NativeGame::evidence_work); (4) the evidence field is optional on read (serde default), so committed Chu/Tenjiku/Tori/Shogi GUI documents still open; new test opens every committed .mshogi fixture; (5) History-target evidence at the live length is accepted; (6) Undo capability uses the last command start; start-position events (a root conclusion) are listed; slot refusal reports CombinedObservations when evidence exists. Not changed: XiangqiPresentation.canonical_context_scope and selected_canonical are now constant but the frontend reads them (remove with a frontend change later); Xiangqi label helpers could use xiangqi_side (cosmetic).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Xiangqi now uses the generic document (ValidatedGameDocument) and session (NativeSession), so all five non-Chess games share one document and session. The old Xiangqi session, archive (1,773 lines), drafts, worker jobs and the SharedState admission rollback are removed; the seven touched modules went from 8,375 to about 6,960 lines (native_game.rs includes about 200 lines of measurement tests). Generic additions for the real Xiangqi differences: an evidence ledger for exact-endgame reports (shares the 256 observation slots), command boundaries for Undo and history (John: history stops only at command boundaries; a move and its warning or conclusion are one entry), command-specific actors (draw offer and reply), a snapshot finish hook (intersection board, Xiangqi presentation), and per-game telemetry status labels. Shared behavior now for every game: commands that would change the game are refused at once while history is viewed; bad analysis requests are refused at once. Xiangqi terminal commands now work while paused, as for the other games. Xiangqi work model measured (about 593 us per move at 511 moves) and the document limit set to 8,388,608 units (largest valid 5.5 s, forged refused 6.3 s); slow native replay is MSH-227. Checks: fmt, clippy (also all features), all workspace tests, 609 frontend tests and tsc with regenerated Xiangqi fixtures; new test opens every committed GUI .mshogi document; fresh-agent review findings fixed. Follow-ups: MSH-225 (Chess), MSH-227 (Xiangqi replay speed).
<!-- SECTION:FINAL_SUMMARY:END -->
