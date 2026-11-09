---
id: MSH-140
title: Player Thinking Time
status: To Do
assignee: []
created_date: '2026-11-09T11:26:00Z'
labels:
  - handle:player-thinking-time
milestone: m-5
dependencies:
  - MSH-138
priority: medium
ordinal: 145000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Player Thinking Time. Owner consolidation2026-11-09 wants the player eventually to choose faster responses or stronger searches. This is future product work after MSH-138 (Desktop Search Limits), not a reason to expand the current one-second default change. Start with ordinary Chess per-move time; do not imply timed support for other games before their engines support it. Align exposed choices with bounded supervision/resource policy and existing document/execution contracts.
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
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
