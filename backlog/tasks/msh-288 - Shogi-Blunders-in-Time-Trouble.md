---
id: MSH-288
title: Shogi Blunders in Time Trouble
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:14:00Z'
updated_date: '2026-11-28T18:55:00Z'
labels:
  - shogi
  - time-management
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies:
  - MSH-287
documentation:
  - docs/m5/shogi-blunder-screen.md
  - docs/m5/strength-roadmap.md
priority: medium
ordinal: 266000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In the MSH-193 Shogi blunder screen (304 MSH-191 games at 60 s + 1 s against external engines), 28% of the 226 clear blunders were in time trouble, and a 10 s rerun avoided 28% of all errors. The old 30% single-mechanism fix rule blocked a fix round. Find why the engine reaches time trouble (allocation early in the game, increment handling, overhead) and fix it in the shared clock policy where the cause is not Shogi-specific. Coordinate with the Xiangqi clock task so that only one shared allocation change is made.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 From the MSH-191 game records, measure clock use per move and identify where the time-trouble positions come from
- [ ] #2 A fix for the measured cause wins an SPRT against the predecessor in Shogi at 60 s + 1 s, or the task records why no candidate won
- [ ] #3 Results, cost and GCS path recorded; shared clock documentation updated if the policy changes
<!-- AC:END -->
