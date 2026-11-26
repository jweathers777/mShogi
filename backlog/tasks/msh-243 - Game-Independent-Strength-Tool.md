---
id: MSH-243
title: Game-Independent Strength Tool
status: Ready
assignee: []
created_date: '2026-11-25T11:47:00Z'
updated_date: '2026-11-26T11:55:00Z'
labels:
  - cleanup
  - tooling
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-253
  - MSH-255
  - MSH-256
  - MSH-258
  - MSH-259
  - MSH-263
  - MSH-264
  - MSH-265
  - MSH-267
priority: medium
ordinal: 7930
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John (2026-11-25): consolidate the strength tools into one reusable, game-independent tool, usable for Tenjiku and for later experiments with the other games. Each game supplies only its real differences. MSH-240 removed per-game tools instead of consolidating them; read the removed versions in Git at commit 52f39b8 (git show 52f39b8:tools/strength/<file>) and the tools that remain in tools/strength before writing code.

Includes the scope of the former MSH-242 (folded in by John, 2026-11-25): the Rust strength examples and example modes in crates/mshogi-runner/examples and crates/mshogi-search/examples, some of which lost their only Python consumer in MSH-240 (for example shogi_tsume, chu_judge, and the judge and mining modes of tori_strength and xiangqi_strength). Consolidate them with the tool; do not remove one that the tool needs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One game-independent tool covers the strength program steps that need Python tooling: balanced openings, tactical suites with held-out parts, predecessor matches, self-play data export and split, time-odds measurement, deeper-search move-quality judging, and cost-bounded cloud submission.
- [ ] #2 Per-game code holds only real differences; duplicate drivers and helpers in the remaining tools/strength scripts are merged into the tool and removed.
- [ ] #3 Tenjiku can use it without new game-specific copies; each other game is supported or has a recorded reason; Python tests and a short local run pass.
- [ ] #4 One review by a fresh agent session.
- [ ] #5 Each Rust strength example is used by the tool, merged into one game-independent implementation, or removed with its support files and tests because nothing in the tool or an open task needs it; the reason is recorded, documents are updated, rules, search and evaluation are unchanged and the workspace checks pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: part of the cleanup; no longer waits for the Tenjiku release. Run after the MSH-209 follow-ups, MSH-222 and MSH-238, and before MSH-241. This task also reviews the strength tooling, which MSH-209 leaves out. MSH-240 task notes list known duplicates.

Consolidation order approved by John, 2026-11-26: dependency fields record the structural cleanup prerequisites. MSH-263 now includes the duration and game-length budget work. MSH-266 now includes final fixture regeneration and therefore follows MSH-241, rather than blocking this strength-tool task. This supersedes the earlier blanket instruction to wait for every MSH-209 follow-up; no strength-tool scope is removed.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.
<!-- SECTION:NOTES:END -->
