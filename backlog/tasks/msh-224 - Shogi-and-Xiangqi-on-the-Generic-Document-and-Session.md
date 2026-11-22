---
id: MSH-224
title: Shogi and Xiangqi on the Generic Document and Session
status: To Do
assignee: []
created_date: '2026-11-22T09:57:00Z'
updated_date: '2026-11-22T10:24:00Z'
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
- [ ] #1 Shogi and Xiangqi use the generic document and session; game-specific code holds only real differences (for example Shogi declarations, Xiangqi exact-endgame evidence)
- [ ] #2 Old per-game session and archive code is removed; production line counts before and after are recorded
- [ ] #3 Desktop-session and workspace tests pass; committed Shogi and Xiangqi fixtures are regenerated or re-expressed; one fresh-agent review
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Design notes from the MSH-223 session (2026-11-22): the Shogi core fits NativeGame with two hooks: (a) a game without a Stop command (engine_failure_stop becomes optional; reserve skips it), (b) a desktop work model like Tori (no native replay work). Shogi engine choices can be a formal declaration, so the trait should map an engine result to a complete command (engine_command) instead of engine_choice + play. Shogi snapshot adds declaration advice and the declaration capability. Xiangqi adds saved exact-endgame evidence and a native undo list; check whether exact evidence can be a per-game extension of the generic payload.
<!-- SECTION:NOTES:END -->
