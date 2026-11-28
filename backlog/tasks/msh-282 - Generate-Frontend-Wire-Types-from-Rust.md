---
id: MSH-282
title: Generate Frontend Wire Types from Rust
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
  - docs/development.md
priority: medium
ordinal: 250800
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Build on the shared frontend types from MSH-264 by making authoritative Rust definitions the source of generated TypeScript for the desktop serialization contract.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Generate TypeScript from authoritative Rust definitions wherever practical, covering serialized commands, snapshots, execution results, enums, game identities, and rules-profile identifiers as applicable.
- [ ] #2 Generated types correctly represent Serde tagging, naming, optional fields, and game-specific variants.
- [ ] #3 Frontend presentation types stay separate. Remove redundant handwritten wire definitions.
- [ ] #4 Generation is deterministic and reproducible, and CI detects stale generated types.
- [ ] #5 Use a maintainable generation mechanism and retain the existing serialization and frontend contract tests.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Generate wire types from the Rust definitions and module boundaries left by MSH-280 and MSH-281. Reuse the simplified square-board consumers from MSH-279 and the check infrastructure from MSH-277. Keep presentation types in the frontend.
<!-- SECTION:NOTES:END -->
