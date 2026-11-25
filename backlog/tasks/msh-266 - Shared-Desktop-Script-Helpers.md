---
id: MSH-266
title: Shared Desktop Script Helpers
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
labels:
  - desktop
  - scripts
  - refactor
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 7800
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Desktop scripts repeat code. desktop/src/assets/fonts/subset-chu.py, subset-tenjiku.py and subset-tori.py (73 lines each) differ only in constants, and subset.py and subset-xiangqi.py (35 each) lack their newer outline and name checks. desktop/scripts/gui-smoke.py (3,749 lines) repeats per-game helpers: history-count waits with three different selectors for the same root, analysis, new game, fixture loading, engine-failure recovery, and five inline copies of the visibility check. macos-verification.py copies the fixture list by hand; gui_linux.py and gui_windows.py record events in different shapes; six test files repeat the same module-loading code for hyphenated file names.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One font subset script with a table of fonts makes the same font files as before, or the changed font files are regenerated and checked.
- [ ] #2 gui-smoke.py and the platform drivers use shared helpers that take the game as a parameter, one event shape and one fixture list; the desktop script tests pass; line counts before and after are recorded.
<!-- AC:END -->
