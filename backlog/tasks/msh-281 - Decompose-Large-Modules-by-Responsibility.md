---
id: MSH-281
title: Decompose Large Modules by Responsibility
status: To Do
assignee: []
created_date: '2026-11-27T12:34:00Z'
labels:
  - technical-debt
  - model:large
  - effort:high
dependencies: []
documentation:
  - docs/design.md
priority: low
ordinal: 255000
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
