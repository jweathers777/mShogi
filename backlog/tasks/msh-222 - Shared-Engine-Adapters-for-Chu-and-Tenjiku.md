---
id: MSH-222
title: Shared Engine Adapters
status: Ready
assignee: []
created_date: '2026-11-21T23:10:00Z'
updated_date: '2026-11-25T08:51:00Z'
labels:
  - engine
  - refactor
milestone: m-6
dependencies: []
priority: medium
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The per-game adapter modules in crates/mshogi-engine (shogi.rs, xiangqi.rs, tori.rs, chu.rs, tenjiku.rs; about 2,060 lines) repeat result, telemetry, validation and limits code; chu.rs and tenjiku.rs have the same structure. Replace them with one generic implementation behind a per-game trait, as MSH-217 (Shared Game Session) did for the desktop session. Start with Chu and Tenjiku, then include every other game whose adapter has the same structure. Protocols may change (A-09); update callers and fixtures.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic adapter serves Chu, Tenjiku and every other game with the same adapter structure; per-game code holds only game-specific behavior; production line counts before and after are recorded.
- [ ] #2 Engine protocol tests, desktop-session tests and the workspace checks pass; changed fixtures are regenerated.
- [ ] #3 One review by a fresh agent session.
<!-- AC:END -->
