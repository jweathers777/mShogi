---
id: MSH-266
title: Desktop Scripts and Reproducible Fixtures
status: To Do
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-26T11:46:00Z'
labels:
  - desktop
  - scripts
  - refactor
  - fixtures
  - model:medium
  - effort:high
milestone: m-6
dependencies:
  - MSH-241
priority: medium
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Desktop scripts repeat code. desktop/src/assets/fonts/subset-chu.py, subset-tenjiku.py and subset-tori.py (73 lines each) differ only in constants, and subset.py and subset-xiangqi.py (35 each) lack their newer outline and name checks. desktop/scripts/gui-smoke.py (3,749 lines) repeats per-game helpers: history-count waits with three different selectors for the same root, analysis, new game, fixture loading, engine-failure recovery, and five inline copies of the visibility check. macos-verification.py copies the fixture list by hand; gui_linux.py and gui_windows.py record events in different shapes; six test files repeat the same module-loading code for hyphenated file names.

Make all six games' committed frontend and GUI fixtures reproducible with the final code and names. MSH-221 found a Chu generator assertion (9 versus 1), old Chu/Tenjiku play_mode output, and stale Chess/Xiangqi frontend fixtures. Recheck those findings, fix generator defects, regenerate all six games and verify that a second run makes no change. Keep CHU.md and the Tenjiku generator notes accurate. This combined task runs after MSH-241 so final fixture verification does not precede the naming change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One font subset script with a table of fonts makes the same font files as before, or the changed font files are regenerated and checked.
- [ ] #2 gui-smoke.py and the platform drivers use shared helpers that take the game as a parameter, one event shape and one fixture list; the desktop script tests pass; line counts before and after are recorded.
- [ ] #3 The Chu GUI document generator runs to completion with the current code, and the cause of the 9 vs 1 assertion is fixed or the assertion is corrected with a recorded reason.
- [ ] #4 The committed frontend and GUI fixtures for all six games are regenerated with the final code and names; a second run makes no change.
- [ ] #5 desktop/scripts/fixtures/CHU.md and the Tenjiku notes still describe the generators correctly
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Context carried from MSH-221 (Desktop GUI Fixture Generators); historical observations, not a replacement for the current criteria and dependency fields:
Found in MSH-224 (2026-11-22): regenerating with write_frontend_fixtures changes Chess (shared-chess-analysis-moves, shared-chess-capacity) and all Xiangqi frontend fixtures because of earlier engine changes (play limits, selective_depth, saved play_mode). Only the Shogi fixtures were updated in MSH-224.

2026-11-25: this task runs after the cleanup tasks (MSH-237 to MSH-238), which change saved formats and engine output again. Regenerate the committed fixtures of all six games once here, not only Chu and Tenjiku.

John approved consolidation on 2026-11-26: this task retains the full scope of MSH-266 (Shared Desktop Script Helpers), MSH-221 (Desktop GUI Fixture Generators). Script helpers and fixture lists must be verified with working generators and reproducible outputs; final regeneration follows the naming cleanup. Agent labels are model:medium, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Status is To Do while an unfinished dependency blocks work; otherwise Ready. No implementation has started.
<!-- SECTION:NOTES:END -->
