---
id: MSH-237
title: Remove obsolete development-version compatibility code
status: Ready
assignee: []
created_date: '2026-11-24T20:31:00Z'
updated_date: '2026-11-25T08:51:00Z'
labels:
  - cleanup
dependencies: []
references:
  - crates/mshogi-engine/src/chu.rs
  - crates/mshogi-search/src/chu.rs
  - crates/mshogi-engine/tests/all_games_timing.rs
documentation:
  - AGENTS.md
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The unreleased application still accepts historical evaluator/search identities, and an ignored timing test converts old protocol-v4 replies. These paths conflict with the zero-backward-compatibility rule in AGENTS.md. Remove obsolete development-version support while preserving current games, rules profiles and useful current regression coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Product readers, identity tables and callers use current development interfaces without obsolete version adapters or discarded fields.
- [ ] #2 Tests and fixtures describe current behavior; obsolete protocol capture/conversion paths are removed.
- [ ] #3 All six games and their selected current rules profiles remain supported, and required checks pass.
<!-- AC:END -->
