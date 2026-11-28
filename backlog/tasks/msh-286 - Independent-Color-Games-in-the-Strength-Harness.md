---
id: MSH-286
title: Independent Color Games in the Strength Harness
status: Done
assignee:
  - '@codex'
created_date: '2026-11-28T10:01:00Z'
updated_date: '2026-11-28T12:34:00Z'
labels:
  - harness
  - strength
  - model:medium
  - effort:medium
milestone: m-5
dependencies: []
documentation:
  - docs/m5/strength-harness.md
  - docs/m4/xiangqi/protocol.md
priority: high
ordinal: 263000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
tools/strength/internal_match.py runs both color games of a pair in sequence under one shared pair wall. A long first game leaves the swapped game with zero turns (MSH-53.3 clock pilot: 6 of 8 pairs hit the shared wall). Limits were also set by hand: a 1280 s wall for games that can need about 1908 s of charged clock time, and 30-minute Batch jobs although gcp.py allows 6 hours. One unscored pair also stops a campaign.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Each color game runs with its own wall limit; a pair is formed only at scoring; an unscored game is reported but does not stop the campaign
- [x] #2 Default per-game wall comes from the clock: base + (max plies / 2) * increment for each side plus a margin; default Batch max_run_seconds comes from the projected campaign need
- [x] #3 Strength tests pass; docs/m5/strength-harness.md describes the new behavior and drops the shared-wall caveat
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Scope includes MSH-238 and the 1,800-second Batch limit found in MSH-285.

Native Adams calibration replayed both colors. Two native Chess pairs retained four wall stops and continued. The local shared check passed with Node 24.19.0: 1,176 Rust tests passed (10 ignored), plus 578 frontend, 57 strength, and 25 tooling tests. Fresh review found three issues in catalog diagnostics, all-game failures, and the Batch upload reserve. All were fixed and covered. Process CI passed on Linux, macOS, and Windows for 5f2a173 at https://github.com/jweathers777/mShogi/actions/runs/46143993677. Batch timing was verified offline. No live Batch job was needed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Each color game has a separate timer, process, and replay report. Only two verified scores enter paired statistics. An unscored game stays visible and the campaign continues. The default game wall follows the clocks and ply budget. Batch projects its task runtime from the longest command bin. The protocol runner accepts up to six hours. MSH-53.4 (Tenjiku Time-Odds Scaling) and MSH-287 (Xiangqi Engine Uses Too Little of Its Clock) can proceed.
<!-- SECTION:FINAL_SUMMARY:END -->
