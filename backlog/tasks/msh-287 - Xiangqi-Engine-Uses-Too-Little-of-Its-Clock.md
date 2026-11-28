---
id: MSH-287
title: Xiangqi Engine Uses Too Little of Its Clock
status: Ready
assignee: []
created_date: '2026-11-28T10:14:00Z'
updated_date: '2026-11-28T11:55:00Z'
labels:
  - xiangqi
  - time-management
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies:
  - MSH-286
documentation:
  - docs/m5/xiangqi-blunder-screen.md
  - docs/m5/strength-roadmap.md
priority: medium
ordinal: 265000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In the MSH-198 blunder screen (MSH-197 ladder, 60 s + 1 s), mShogi used a median of 1.5 s per move, and a 10 s rerun avoided 42% of all errors and 13 of 26 clear blunders. The old 30% single-mechanism fix rule blocked a fix round. Time allocation is shared across games (MSH-245 One Clock Policy for All Games), so a better allocation should be one shared change.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measure how the engine spends a 60 s + 1 s clock in Xiangqi (time per move by phase, unused clock at game end)
- [ ] #2 A changed shared allocation (for example a larger share, or extra time when the best move or score is unstable) wins an SPRT against the predecessor in Xiangqi at 60 s + 1 s, or the task records why no candidate won
- [ ] #3 A kept change does not lose in a short SPRT in the other games that use the shared policy; results, cost and GCS path recorded
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
The experiment runner uses the same shared allocation (soft budget for 30 moves, docs/m5/strength-roadmap.md 'Clock allocation of the experiment runner'), so a change here also changes the runner policy that John decided on 2026-11-25. Depends on MSH-286 so its matches do not hit the shared pair wall or the fixed 1800 s job.

John, 2026-11-28: MSH-287 may change the shared 30-move clock allocation, including the experiment runner policy he decided on 2026-11-25. Update the 'Clock allocation of the experiment runner' section of docs/m5/strength-roadmap.md if the policy changes.
<!-- SECTION:NOTES:END -->
