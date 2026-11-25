---
id: MSH-260
title: Incremental Engine Sessions for Every Game
status: Ready
assignee: []
created_date: '2026-11-25T12:22:00Z'
labels:
  - engine
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 7500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The engine keeps a retained native game between turns (incremental session: mshogi-core native/incremental.rs and mshogi-engine incremental.rs) only for Shogi (MSH-189), Tori (MSH-204) and Chu (MSH-213); each was added during the strength work of that game. Xiangqi, Tenjiku and Chess replay the full record on each request. The Tenjiku harness (MSH-214) will play long games through the same route. Do this after MSH-222 (Shared Engine Adapters) and MSH-253 (One Compact Record Codec), which give the generic adapter and a generic record suffix.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Incremental sessions are a capability of the generic engine adapter; each game that has a record suffix uses them without per-game session code; a game left out has a recorded reason.
- [ ] #2 Engine protocol and incremental tests cover each game; the replay cost per request at full length is measured before and after for Xiangqi and Tenjiku.
<!-- AC:END -->
