---
id: MSH-226
title: Xiangqi on the Generic Document and Session
status: To Do
assignee: []
created_date: '2026-11-22T11:33:00Z'
labels:
  - desktop
  - refactor
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
