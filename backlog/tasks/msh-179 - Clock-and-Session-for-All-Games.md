---
id: MSH-179
title: Clock and Session for All Games
status: Ready
assignee: []
created_date: '2026-11-15T22:31:00Z'
labels:
  - handle:all-games-time-state
milestone: m-5
dependencies: []
priority: medium
ordinal: 186000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Clock and Session for All Games.

**Problem:** Only Chess has a time budget per move (MSH-167 (Engine Time Management)) and engine state that lasts between the moves of a game (MSH-169 (Engine Session Persistence)). The other games search with fixed depth and node limits, the internal execution protocol rejects `time_ms` for them, and their searches start empty at each move. Tori already uses the shared core (MSH-165 (Tori Search Core Adoption)), and the other games will follow.

**Objective:** Each game on the shared core can search with a time budget and keep its transposition table and statistics between the moves of one game, through the same mechanism as Chess.

**Approach:** Generalize the Chess mechanisms instead of copying them for each game: time allocation in the core, the session of the engine process (`crates/mshogi-engine/src/server.rs`) and the `time_ms` admission in the protocol. Keep fixed depth and node limits reproducible, as Chess does. Do not change desktop defaults; MSH-140 (Player Thinking Time) owns player choices. A game that moves to the core later uses this mechanism through its adapter. This changes protocol and process behavior, so it is Level 2: select the platforms from the actual risk.

**Completion:** Tori, and each other game already on the core, plays under a time budget and keeps engine state between moves, with Chess behavior unchanged.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every game on the shared core accepts a time budget through the internal protocol, and the engine never exceeds the hard limit in a recorded set of games under a clock
- [ ] #2 Engine state lasts between the moves of one game for every game on the core, and a new, loaded or taken-back game starts empty, as for Chess
- [ ] #3 Requests with fixed depth or node limits stay reproducible, and saved documents with earlier observations still open
- [ ] #4 The mechanism is shared: no per-game copy of time allocation or session handling, and Chess behavior and tests are unchanged
- [ ] #5 Level 2 checks on the selected platforms, and independent review, are recorded
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
