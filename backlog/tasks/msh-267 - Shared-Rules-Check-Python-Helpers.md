---
id: MSH-267
title: Shared Rules-Check Python Helpers
status: Done
assignee:
  - '@codex'
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-27T01:02:00Z'
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
- [x] #1 One shared helper module serves the chess checks (one script with subcommands, or shared functions), and tools/shogi and tools/xiangqi use shared helpers; each check gives the same result on the committed fixtures.
- [x] #2 compare_perft.py is removed or documented; the Python tests of these tools run in CI, or the notes record why not.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Share Chess fixture and oracle checks while preserving each fixture family’s assertions and command.
2. Share Shogi and Xiangqi developer probe helpers while preserving game admission and process limits. Document the Shogi perft command and run the dependency-free Xiangqi sentinels in CI.
3. Compare before and after results on committed fixtures with the pinned oracles and run integration checks.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.

Branch codex/msh-267-rules-check-helpers. Claim pushed. Next establish reference runs, extract the common code, and verify parity. No fixture or rules changes planned.

Shared helpers, documentation, and CI selection are implemented on codex/msh-267-rules-check-helpers. Baseline and refactor agree for all seven Chess fixture scripts and the 15,008-ply differential, and for 105 Xiangqi fixture entries and 34 walk positions. All 15 Xiangqi sentinels passed. Local integration checks passed with the installed nvm Node/npm pair because Homebrew Node cannot load libsimdjson. Next finish Shogi and Xiangqi history parity, confirm CI, then integrate.

Pinned cshogi and Pikafish rebuilt and locally admitted. Before/after payloads match exactly for 101 Shogi board fixtures, 99 seeded Shogi sequences, 13 admission plus 4 API-gap sentinels, 4 diagnostic injections, 101 depth-2 Shogi divides, Xiangqi board and walks, and 17 Xiangqi histories (492 prefixes). GitHub run 45919886214 passed Routine and tooling on Linux, macOS, and Windows. Its Routine log confirms eight local Xiangqi sentinels. The seven upstream Xiangqi sentinels run manually with the admitted binary per conformance policy. All code and validation are on codex/msh-267-rules-check-helpers. Next verify CI on the final code checkpoint and integrate.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Shared Chess oracle, fixture, classification, perft, and command helpers, plus Shogi/Xiangqi hash, Git, strict JSON, and probe request helpers. Documented Shogi perft.
Baseline and refactor observations match on the committed fixtures. Chess compared 15,008 plies. Shogi checked 101 fixture positions and depth-2 divides. Xiangqi checked 105 board entries and 492 history prefixes.
Local integration passed. GitHub Actions run 45920116159 passed Routine checks and tooling on Linux, macOS, and Windows. CI runs eight local Xiangqi sentinels. The seven upstream-dependent tests passed locally with the admitted binary.
Code on main at b70aeef. Next is MSH-243 (Game-Independent Strength Tool).
<!-- SECTION:FINAL_SUMMARY:END -->
