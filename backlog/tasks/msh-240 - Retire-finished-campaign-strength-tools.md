---
id: MSH-240
title: Retire finished-campaign strength tools
status: Ready
assignee: []
created_date: '2026-11-25T09:51:00Z'
updated_date: '2026-11-25T10:20:00Z'
labels:
  - cleanup
  - model:medium
  - effort:medium
dependencies: []
priority: high
ordinal: 2500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Some tools/strength scripts are kept unchanged only to re-verify evidence of finished campaigns (for example shogi_cloud.py, kept by shogi_campaign.py for frozen evidence). Remove the tools and tests that only support earlier development versions (AGENTS.md zero backward compatibility).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each tools/strength script that only re-verifies finished-campaign evidence is removed, with its tests, or has a recorded current use.
- [ ] #2 Current match, screening and cloud tools still work and their Python tests pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: run before MSH-209, so the review has less code to read.
<!-- SECTION:NOTES:END -->
