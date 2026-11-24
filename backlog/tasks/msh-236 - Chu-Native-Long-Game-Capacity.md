---
id: MSH-236
title: Remove Remaining History-Work Limits
status: Done
assignee:
  - codex
created_date: '2026-11-23T22:01:00Z'
updated_date: '2026-11-24T07:37:00Z'
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
Remove the remaining artificial history-work limits and all runtime consumers across all six games, including command/event history quotas. John explicitly authorized removal after these limits blocked MSH-213. Keep real cancellation, process timeouts and game rules; do not introduce another work model. This unreleased application has no backward-compatibility obligations unless John explicitly names an exception.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 No artificial history-work counter, quota or validation consumer remains in any game
- [x] #2 Long-game regressions pass beyond the former work cutoff; saved records and rules behavior remain correct
- [x] #3 Required checks and fresh review pass; MSH-213 resumes with the removal integrated
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

The larger cloud sample exposed a second leftover: the separate Chu512-event quota. Removing artificial command/event history quotas and their consumers across all six games as part of the same cleanup. Shogi native320-board-action ending remains unchanged; actual record/document byte bounds, cancellation and elapsed deadlines remain. New long-history regressions cross the former event quotas. No replacement work model or higher history allowance is introduced. Initial work-only sample remains diagnostic, not the final baseline.

Checkpoint: removed the remaining command/event quotas across all games; retained actual encoded-byte bounds and the Shogi 320-board-action rule. Console candidates now validate their full encoded record before publication. Final tests and review are in progress. The work-only sample failed on the old event quota; all eight cloud tasks and their VMs/disks are verified absent. Next: commit reviewed removal, rebuild current binaries and run the final MSH-213 sample. Branch codex/msh-236-remove-history-work.

Source4f5e2ed removes the remaining event/command quotas. Fresh review approved actual-byte console publication checks and preserved cancellation rollback. fmt and Clippy pass; frontend595 tests pass; desktop-verification200 pass; workspace tests and Linux/macOS/Windows persistence CI45347831998 remain running. Final cloud build build-msh236-final is active; MSH-213 final sample will use that source. Work-only failed sample cleanup is verified; conservative compute upper bound US$0.356.

Final persistence CI45347831998 passed on Linux/macOS/Windows. Final build source4f5e2ed succeeded in151.3s on c3-standard-4; build job/VM/disk cleanup verified, conservative compute upper bound US$0.123. Last full workspace run exposed one obsolete Xiangqi test expecting the removed511-command search failure; faef59f replaces it with600-command successful native-PV replay and unchanged-source checks. Complete workspace rerun is active; Clippy remains clean. Final MSH-213 cloud sample is active at runs/msh-213/final-sample.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed synthetic history-work accounting and command/event quotas across all six games, plus obsolete Tori readers and old-child protocol fallback. AGENTS.md now states zero backward compatibility unless John names an exception. Native rules, cancellation and actual encoded-byte checks remain. Final checks: fmt/Clippy pass; workspace1187 passed,12 ignored; frontend595 and desktop-verification200 pass; fresh review accepted; Linux/macOS/Windows persistence CI45347831998 passed. MSH-213 measurements resumed with source4f5e2ed. No replacement work model.
<!-- SECTION:FINAL_SUMMARY:END -->
