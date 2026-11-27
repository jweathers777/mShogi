---
id: MSH-282
title: Generate Frontend Wire Types from Rust
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
  - docs/development.md
priority: medium
ordinal: 256000
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
