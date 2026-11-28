---
id: MSH-53.4
title: Tenjiku Time-Odds Scaling
status: Ready
assignee: []
created_date: '2026-11-28T10:02:00Z'
updated_date: '2026-11-28T10:19:00Z'
labels:
  - tenjiku
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies:
  - MSH-285
  - MSH-286
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/strength-harness.md
parent_task_id: MSH-53
priority: high
ordinal: 264000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replaces the open scaling criterion of MSH-53.3, whose fixed-node ladder and shared-wall clock pilot gave no estimate. Measure Tenjiku scaling the same way as the accepted Tori and Chu baselines: the same accepted engine at 2x against 1x clock, both colors, for Adams and Muller. Estimate game length from the existing MSH-53.2 reports first; no extra pilot is needed unless that data is missing.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Before play, a short feasibility check from earlier data shows that each game can finish within its clock, ply, wall and job limits, and gives the projected cost
- [ ] #2 120+3 vs 60+1.5 (optionally also 60+1.5 vs 30+0.75), 24 to 32 opening pairs per profile, fixed maximum; report W/D/L, unscored games with reasons, pentanomial Elo estimate with 95% interval per profile
- [ ] #3 Cost (inside the US$10 cap), commands, revisions and GCS path recorded; roadmap Tenjiku row updated; baseline decision left to John
<!-- AC:END -->
