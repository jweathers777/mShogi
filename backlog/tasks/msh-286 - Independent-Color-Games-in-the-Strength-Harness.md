---
id: MSH-286
title: Independent Color Games in the Strength Harness
status: Ready
assignee: []
created_date: '2026-11-28T10:01:00Z'
labels:
  - harness
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies: []
documentation:
  - docs/m5/strength-harness.md
priority: high
ordinal: 263000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
tools/strength/internal_match.py runs both color games of a pair in sequence under one shared pair wall. A long first game leaves the swapped game with zero turns (MSH-53.3 clock pilot: 6 of 8 pairs hit the shared wall). Limits were also set by hand: a 1280 s wall for games that can need about 1908 s of charged clock time, and 30-minute Batch jobs although gcp.py allows 6 hours. One unscored pair also stops a campaign.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each color game runs with its own wall limit; a pair is formed only at scoring; an unscored game is reported but does not stop the campaign
- [ ] #2 Default per-game wall comes from the clock: base + (max plies / 2) * increment for each side plus a margin; default Batch max_run_seconds comes from the projected campaign need
- [ ] #3 Strength tests pass; docs/m5/strength-harness.md describes the new behavior and drops the shared-wall caveat
<!-- AC:END -->
