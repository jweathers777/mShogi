---
id: MSH-226
title: Xiangqi on the Generic Document and Session
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-22T11:33:00Z'
updated_date: '2026-11-22T11:51:00Z'
labels:
  - needs-john
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
- [ ] #1 Xiangqi uses the generic document and session; game-specific code holds only real differences (multi-event commands, draw actors, exact evidence, presentation)
- [ ] #2 The old Xiangqi session and archive code and the old admission rollback in SharedState are removed; production line counts before and after are recorded
- [ ] #3 Desktop-session and workspace tests pass; Xiangqi fixtures regenerated; frontend tests pass; one fresh-agent review
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
<!-- SECTION:NOTES:END -->
