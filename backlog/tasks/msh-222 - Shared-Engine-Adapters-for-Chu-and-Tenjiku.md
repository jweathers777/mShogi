---
id: MSH-222
title: Shared Engine Adapters for Chu and Tenjiku
status: To Do
assignee: []
created_date: '2026-11-21T23:10:00Z'
labels:
  - engine
  - refactor
milestone: m-6
dependencies: []
priority: low
ordinal: 229000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: remove the copied per-game engine adapter modules in crates/mshogi-engine (chu.rs and tenjiku.rs have the same structure: result, telemetry, validation, limits), with one generic implementation behind a per-game trait, in the same way as MSH-217 (Shared Game Session) did for the desktop session. Engine protocol and saved observation formats do not change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One generic adapter serves Chu and Tenjiku; per-game code holds only game-specific behavior; production line counts before and after are recorded
- [ ] #2 Engine protocol tests, saved observations of both games and the desktop-session tests pass unchanged
- [ ] #3 One review by a fresh agent session
<!-- AC:END -->
