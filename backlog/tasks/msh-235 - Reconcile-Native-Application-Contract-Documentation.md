---
id: MSH-235
title: Reconcile Native Application Contract Documentation
status: Ready
assignee: []
created_date: '2026-11-23T17:53:00Z'
labels:
  - documentation
milestone: m-6
dependencies: []
documentation:
  - docs/desktop/native-application.md
priority: low
ordinal: 242000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
docs/desktop/native-application.md still describes saved observations, journals, retained Take back branches and multi-cause resource omissions that the current minimal document/session implementation no longer has. Reconcile this durable contract with the current design and code so a new worker does not restore retired behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The native application contract accurately describes current document/session ownership, Save/Open, Take back and resource diagnostics, with stale staging claims removed or clearly historical.
- [ ] #2 Statements are checked against docs/design.md, docs/desktop/document-v1.md and current native session/resource code; no product behavior or process rules change.
<!-- AC:END -->
