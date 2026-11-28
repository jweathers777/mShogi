---
id: MSH-289
title: Refresh completed Tenjiku desktop snapshots
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:46:00Z'
updated_date: '2026-11-28T20:59:00Z'
labels:
  - tenjiku
  - desktop
  - model:small
  - effort:low
dependencies: []
references:
  - MSH-285
priority: high
type: chore
ordinal: 250200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Four completed Tenjiku frontend snapshots, the analysis and engine-played snapshots for Adams and Muller, still reflect search output older than the accepted MSH-53.2 evaluation. Regeneration from main at 2ccc003 and from the MSH-285 candidate produces the same current results for these four snapshots.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The four completed Tenjiku snapshots match the current generator output.
- [ ] #2 The affected frontend tests and desktop fixture regeneration check pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Regenerate the desktop fixtures and retain the four completed Tenjiku snapshots. 2. Run the fixture regeneration check, affected frontend tests, and shared checks; integrate and clean up.
<!-- SECTION:PLAN:END -->
