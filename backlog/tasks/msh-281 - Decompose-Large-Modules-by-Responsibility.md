---
id: MSH-281
title: Decompose Large Modules by Responsibility
status: Ready
assignee: []
created_date: '2026-11-27T12:34:00Z'
updated_date: '2026-11-28T20:55:00Z'
labels:
  - technical-debt
  - model:large
  - effort:high
dependencies: []
documentation:
  - docs/design.md
priority: medium
ordinal: 250700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Review large production modules including desktop-session game_session.rs and lib.rs, Tauri main.rs, search alphabeta/mod.rs, console protocol.rs, and engine adapter.rs. Extract responsibilities that have clear ownership and simplify the remaining structure.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Identify coherent responsibilities and extract cohesive, reusable modules with narrow interfaces and clear ownership where useful.
- [ ] #2 Keep interfaces as private as practical. Avoid size targets and relocations that leave the same complexity.
- [ ] #3 Review traits, macros, and abstraction layers for unnecessary indirection, coupling, or superficial sharing. Simplify or remove abstractions that increase maintenance cost.
- [ ] #4 Preserve behavior, performance, and applicable test coverage.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Re-inventory the named modules after MSH-278, MSH-280 and MSH-279. Chess consolidation can remove work in the engine adapter, console protocol, search core and desktop session before decomposition. Extract responsibilities from the resulting code and leave wire generation to MSH-282.
<!-- SECTION:NOTES:END -->
