---
id: MSH-221
title: Desktop GUI Fixture Generators
status: To Do
assignee: []
created_date: '2026-11-21T23:10:00Z'
updated_date: '2026-11-26T11:47:00Z'
labels:
  - desktop
  - fixtures
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: low
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: make the committed desktop GUI fixtures reproducible again. On main c0f67d7, the Chu generator (cargo run --release -p mshogi-desktop-session --example write_chu_gui_documents -- desktop/scripts/fixtures) stops with an assertion failure (left 9, right 1). The committed Chu and Tenjiku .mshogi fixtures were written before the play_mode setting, so regeneration changes every Tenjiku file (the fixtures still load). Found in MSH-217 (Shared Game Session).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The Chu GUI document generator runs to completion on main, and the cause of the 9 vs 1 assertion is fixed or the assertion is corrected with a recorded reason
- [ ] #2 The committed Chu and Tenjiku fixtures are regenerated, and a second run makes no change
- [ ] #3 desktop/scripts/fixtures/CHU.md and the Tenjiku notes still describe the generators correctly
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Found in MSH-224 (2026-11-22): regenerating with write_frontend_fixtures changes Chess (shared-chess-analysis-moves, shared-chess-capacity) and all Xiangqi frontend fixtures because of earlier engine changes (play limits, selective_depth, saved play_mode). Only the Shogi fixtures were updated in MSH-224.

2026-11-25: this task runs after the cleanup tasks (MSH-237 to MSH-238), which change saved formats and engine output again. Regenerate the committed fixtures of all six games once here, not only Chu and Tenjiku.

John approved consolidation on 2026-11-26. Entire implementation scope, acceptance criteria (updated only for superseded requirements), references and relevant context are now owned by MSH-266. This record is archived as superseded, not Done. No implementation or acceptance check was completed by consolidation. Use the surviving task for future work.
<!-- SECTION:NOTES:END -->
