---
id: MSH-236
title: Chu Native Long-Game Capacity
status: Ready
assignee: []
created_date: '2026-11-23T22:01:00Z'
labels:
  - chu
  - native
milestone: m-5
dependencies: []
references:
  - gs://mshogi-experiments-216675633801/runs/msh-213/pilot-2/tasks/0/control/
priority: high
ordinal: 243000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
MSH-213 standard-opening self-play reaches the native ReplayWork guard after 218 applied engine moves (226 board plies including the opening), before a game result. The existing per-move n*n replay charge reaches MAX_REPLAY_WORK=4000000; the guard is independent of the new incremental transport. Restore useful long-game capacity while preserving exact history, lossless saved-record admission and bounded replay. Any product capacity policy change needs John approval; do not turn refusal into a draw.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A fixed long standard-root regression can continue beyond the recorded MSH-213 failure with exact replay and history semantics
- [ ] #2 Capacity accounting has a measured bound; refusal remains atomic and accepted games remain closable and replayable
- [ ] #3 Saved-format compatibility and sourced rules tests pass; any product policy change has recorded John approval
<!-- AC:END -->
