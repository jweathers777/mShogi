---
id: MSH-235
title: Reconcile and Prune Durable Documentation
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-23T17:53:00Z'
updated_date: '2026-11-27T07:18:00Z'
labels:
  - documentation
  - model:medium
  - effort:high
milestone: m-6
dependencies:
  - MSH-266
documentation:
  - docs/desktop/native-application.md
  - docs/desktop/document-v1.md
  - docs/desktop/internal-execution-v4.md
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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Verify the six-game desktop session, document, file, history and engine contracts against the current source and design. Consolidate superseded desktop documents.
2. Inventory historical records. Preserve useful run output in the project GCS bucket before removal. Preserve active fixture inputs and independent derivations. Update references.
3. Restore corrupt number spacing in retained prose. Check documentation links, unchanged fixture data, and Backlog structure before integration.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-209 (2026-11-25): an earlier bulk edit removed the space between a word and the number after it (for example 'exceeds1MiB', 'rank12') in about 190 places; most are in docs/m4. MSH-249 (Restore Spaces Lost Before Numbers) fixes crates/, desktop/ and tools/ only; fix the documents that this task keeps.

Consolidation order approved by John, 2026-11-26: run after MSH-266 (Desktop Scripts and Reproducible Fixtures), which follows MSH-241 and all structural cleanup. Reconcile documents only after final names and generated fixtures are settled.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Claimed on codex/msh-235-durable-docs. Next: verify desktop contracts against the code and inventory evidence under docs/.

Initial inventory: current runner replay tests include 15 run reports by path. Keep their exact bytes as runner test inputs outside docs/ and update the test paths. The Shogi mutual-check chain contains a durable proof. Preserve the proof, and archive generated results and verification narratives.

The retired files and original run narratives are archived under gs://mshogi-experiments-216675633801/runs/msh-235/. Before pruning, verified downloaded SHA-256: retired-docs-5b5c35f.tar.gz 4804ab1f731419ee83a6b6099becc4245dbf3c1f6e3ec7075cfaf96148b6bb5d; retired-run-narratives-5b5c35f.tar.gz b7a0a1c4d7dce7d992712f1139bdeb7d3bda2a247678cc4ab101631f5b1bf386. Source 5b5c35f retains the original paths in Git history.

Rewrote current desktop contracts and kept the independent Shogi derivations. Moved 15 required replay inputs and two reference manifests byte for byte. All four runner replay tests and the repository Markdown link scan passed. Next on codex/msh-235-durable-docs: finish the shared check run and content audit, then integrate.

Final prose sweep removed dated search result tables and retained the current design notes, independent rules derivations and reproducible methods. The original search material is in retired-search-results-5b5c35f.tar.gz under the same GCS path, downloaded SHA-256 38d398da129a6c45c2ce7560238209a22b20b1af59f69d0bed42d69dc2390580. The Chu data validator and Shogi geometry comparison passed. The optional standalone Tenjiku validator fails identically against unmodified main 5b5c35f; its projection bookkeeping defect is MSH-276.
<!-- SECTION:NOTES:END -->
