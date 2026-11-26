---
id: MSH-267
title: Shared Rules-Check Python Helpers
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-26T11:55:00Z'
labels:
  - tools
  - refactor
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 7850
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The rules-check tools outside tools/strength repeat helpers. tools/check_chess_*.py (seven scripts, 477 lines) each repeat require, the oracle version and hash check (differential_chess.py has the hash hard-coded), the fixture header checks, the per-case checks and the exit wrapper; perft exists three times (check_chess_corpus.py, check_chess_divide.py, differential_chess.py) and the classification map twice. tools/shogi and tools/xiangqi repeat sha, git, strict-JSON and probe-launch helpers. tools/shogi/compare_perft.py has no documented use. tools/xiangqi/test_reference.py does not run in CI. tools/experiments helpers shared with tools/strength are MSH-243 (Game-Independent Strength Tool) work; leave them.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One shared helper module serves the chess checks (one script with subcommands, or shared functions), and tools/shogi and tools/xiangqi use shared helpers; each check gives the same result on the committed fixtures.
- [ ] #2 compare_perft.py is removed or documented; the Python tests of these tools run in CI, or the notes record why not.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.
<!-- SECTION:NOTES:END -->
