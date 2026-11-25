---
id: MSH-140
title: Player Thinking Time
status: To Do
assignee: []
created_date: '2026-11-09T11:26:00Z'
updated_date: '2026-11-25T09:15:00Z'
labels:
  - handle:player-thinking-time
  - model:medium
  - effort:high
milestone: m-6
dependencies:
  - MSH-138
priority: medium
ordinal: 173000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Let the player choose faster responses or stronger searches in desktop play. Desktop play now uses 1 s per move for Chess and Shogi and node limits for Xiangqi, Tori, Chu and Tenjiku, although all six engines accept a per-move time limit through the shared core (MSH-167). Start with the games that already play by time; offer a choice for another game only after its desktop play uses time. Keep the choices inside the existing process-supervision limits and document/execution contracts.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The player can select clearly described supported Chess thinking times, with exact effective limits reflected in actual desktop engine requests and ordinary play.
- [ ] #2 Saving/reopening, explicit Resume, historical observations, cancellation and stale-result rejection preserve complete identities and compatible documents; time choices and process-supervision deadlines agree without unbounded execution.
- [ ] #3 Focused actual desktop/native execution checks and independent review support responsiveness and honest last-completed-result/incomplete behavior; other-game capabilities and resource protections remain explicit.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Status check, 2026-11-15: MSH-167 (Engine Time Management) added clock-based time allocation to the shared core and the Chess engine; the desktop default stayed fixed time per move. This task can build the player choice on that mechanism. Tori and the other games still use node limits only.

2026-11-25: moved from M5 (strength) to M6 (product polish) and the description updated to the current desktop limits (John approved the ticket review).
<!-- SECTION:NOTES:END -->
