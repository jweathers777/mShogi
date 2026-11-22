---
id: MSH-228
title: Minimal Saved Game Document
status: To Do
assignee: []
created_date: '2026-11-22T16:43:00Z'
labels:
  - desktop
  - refactor
  - simplification
milestone: m-6
dependencies:
  - MSH-226
priority: high
ordinal: 235000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The shared desktop document (game_document.rs) stores a full audit trail: a journal of every change (settings, rewinds, closes), a redundant current record, all branches removed by Take back and Undo, engine observations each with a full copy of the game up to that point, Xiangqi exact evidence, a runtime execution floor and the viewed history position. The UI uses almost none of it after reopening, and it drives most of the capacity reservations and work limits. John (2026-11-22) approved reducing the document to what a game file needs and changing the design accordingly (A-09 'Take back lineage' and the audit-trail wording).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The saved document holds only: game kind, rules profile, start position, human side, play mode, orientation, the game's commands, and optional per-move engine notes (score, line, depth, nodes); opening checks it by one replay of the commands
- [ ] #2 Removed: the controls journal, current, initial_settings, retired branches, observation contexts, Xiangqi evidence ledger (or reduced to a short note), retired_execution_floor, selected_event, Close entries, mode, and the reservation and work-limit machinery that only these needed; line counts before and after recorded
- [ ] #3 Take back and Undo cut the move list; reopening a file restores the game, sides and settings; docs/design.md is updated to the new rules (A-09 and audit wording)
- [ ] #4 Desktop-session and workspace tests pass; fixtures regenerated; frontend tests pass; one fresh-agent review
<!-- AC:END -->
