---
id: MSH-280
title: Reduce Unjustified Chess Specializations
status: Ready
assignee: []
created_date: '2026-11-27T12:34:00Z'
updated_date: '2026-11-28T20:54:00Z'
labels:
  - technical-debt
  - model:large
  - effort:xhigh
dependencies: []
documentation:
  - docs/design.md
  - docs/m5/shared-search-core.md
priority: medium
ordinal: 250500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Review Chess exceptions across rules history, search roots, execution adapters, search coordination, protocols, limits, timing, and experiments. Build on MSH-252 and MSH-258 while retaining justified Chess behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Inventory significant Chess-specific implementations and classify each by rules, protocol, measured performance, or another concrete technical constraint.
- [ ] #2 Consolidate unjustified differences into existing shared implementations. Retain necessary semantics and performance optimizations and avoid generic complexity introduced only for symmetry.
- [ ] #3 Verify correctness, supported protocol behavior, and playing strength with the applicable checks and measurements.
- [ ] #4 Record the justification for remaining significant differences.
<!-- AC:END -->
