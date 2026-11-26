---
id: MSH-235
title: Reconcile and Prune Durable Documentation
status: To Do
assignee: []
created_date: '2026-11-23T17:53:00Z'
updated_date: '2026-11-26T11:46:00Z'
labels:
  - documentation
  - model:medium
  - effort:high
milestone: m-6
dependencies:
  - MSH-266
documentation:
  - docs/desktop/native-application.md
priority: low
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Do this after the cleanup tasks, so the documents describe the final code. docs/desktop/native-application.md still describes saved observations, journals, retained Take back branches and multi-cause resource omissions that the current code no longer has; other desktop protocol and archive documents may also describe removed versions. docs/ also holds material that AGENTS.md excludes: task reviews, verification records, logs and JSON run output (for example 31 JSON or log files under docs/m4). Keep durable knowledge such as rules derivations.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The desktop documents accurately describe the current document, session, Save/Open, Take back, resource diagnostics and protocol; stale claims are removed.
- [ ] #2 docs/ holds only durable knowledge: task reports, reviews, verification records and run output are removed (useful run data is first uploaded to GCS), and links to them are updated.
- [ ] #3 Statements are checked against docs/design.md and the current code; no product behavior or process rule changes.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-209 (2026-11-25): an earlier bulk edit removed the space between a word and the number after it (for example 'exceeds1MiB', 'rank12') in about 190 places; most are in docs/m4. MSH-249 (Restore Spaces Lost Before Numbers) fixes crates/, desktop/ and tools/ only; fix the documents that this task keeps.

Consolidation order approved by John, 2026-11-26: run after MSH-266 (Desktop Scripts and Reproducible Fixtures), which follows MSH-241 and all structural cleanup. Reconcile documents only after final names and generated fixtures are settled.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Status is To Do while an unfinished dependency blocks work; otherwise Ready. No implementation has started.
<!-- SECTION:NOTES:END -->
