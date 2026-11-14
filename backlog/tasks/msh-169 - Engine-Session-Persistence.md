---
id: MSH-169
title: Engine Session Persistence
status: To Do
assignee: []
created_date: '2026-11-14T09:57:00Z'
labels:
  - handle:engine-session-persistence
milestone: m-5
dependencies:
  - MSH-164
references:
  - docs/m5/shared-search-core.md
priority: medium
ordinal: 173000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Engine Session Persistence.

**Problem:** The desktop and the internal execution protocol start one engine process for each search request (`mshogi_engine::serve_once`, `execute_chess` makes a new `SearchCoordinator`). The transposition table and move statistics of MSH-164 (Shared Search Core) therefore start empty at each move on that path. Only the UCI path keeps them for the full game. The independent review of MSH-164 found this.

**Objective:** The engine state lasts for the moves of one game on the desktop path too.

**Approach:** Decide between a persistent engine process for a game session and another way to keep the state, within the single internal execution contract of MSH-116 and the process supervision rules of the desktop. This changes process lifetime, cancellation and shutdown, so it is a Level 2 change. Measure the gain first with the MSH-163 harness (warm table against cold table at equal time); if the gain is small, record that and close without the change.

**Completion:** A measured decision, and if the change is made, the desktop path keeps the engine state between moves with cancellation and shutdown still correct.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The strength difference between a kept table and a new table at each move is measured with the game test and recorded
- [ ] #2 If the change is made: two successive desktop searches of one game use the same engine state, and a new game or a loaded game does not reuse it wrongly
- [ ] #3 If the change is made: cancellation, replacement and shutdown of a search still stop the engine within the present limits on the targeted platforms
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
