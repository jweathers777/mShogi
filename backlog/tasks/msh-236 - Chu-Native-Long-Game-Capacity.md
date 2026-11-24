---
id: MSH-236
title: Remove Remaining History-Work Limits
status: In Progress
assignee:
  - codex
created_date: '2026-11-23T22:01:00Z'
updated_date: '2026-11-24T07:07:00Z'
labels:
  - chu
  - native
milestone: m-5
dependencies: []
references:
  - gs://mshogi-experiments-216675633801/runs/msh-213/pilot-2/tasks/0/control/
priority: high
ordinal: 243000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Remove the remaining artificial history-work limits and all runtime consumers across all six games. John explicitly authorized this on 2026-11-24 after the surviving Chu limit blocked MSH-213. Keep real cancellation, process timeouts, game rules and saved-record compatibility; do not introduce another work model.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 No artificial history-work counter, quota or validation consumer remains in any game
- [x] #2 Long-game regressions pass beyond the former work cutoff; saved records and rules behavior remain correct
- [ ] #3 Required checks and fresh review pass; MSH-213 resumes with the removal integrated
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Remove counters and validation budgets, retain direct native validation and cancellation, check old record readers, test long games, review and integrate. Then resume Chu measurements.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John clarified that this unreleased product has no backward-compatibility obligation. Removed the obsolete Tori schema-1 reader and its envelope path rather than retaining discarded work fields. No replacement work model. Native rule tests remain; compatibility-only tests and old migration fixtures are removed.

John also explicitly authorized AGENTS.md to state ZERO backward compatibility obligations for files, APIs and protocols unless he names an exception. Updated that rule in place. Removed the old-child capability fallback added during MSH-213; measurements must use the current interface, not preserve a product compatibility path for an old test binary.

Removal implemented: Chu/Tenjiku history charges and counters, Tenjiku local query/effect metering, record caches, validation work returns and UI diagnostics removed. Tori obsolete reader/envelope removed under explicit user direction. Real cancellation checks remain. 400-action Chu regression passes with replay and resignation. Fresh review found no product defect; fixture/doc findings fixed. Frontend597 tests and TypeScript pass; final Clippy passes; final workspace tests running. Branch codex/msh-236-remove-history-work contains the pending MSH-213 harness too.

Final native/source4f6f07e checks: fmt/Clippy pass, workspace1190 passed/12 ignored/0 failed; frontend597 tests and TypeScript pass; harness examples6 pass; Python169 run/1 skipped. Final fresh review also approved removal of old Tori migration and old-child fallback. Cloud standard games now complete at416/438/427/114 moves;64 fixed-work results match the former30e08e4 binary. Cloud build completed in153.6s on c3-standard-4 with cleanup verified. Linux process CI45343902716 passed; macOS/Windows still running. Full post-removal game sample active under MSH-213/sample-1. AGENTS.md now explicitly states zero backward compatibility unless John names an exception.
<!-- SECTION:NOTES:END -->
