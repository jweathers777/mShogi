---
id: MSH-286
title: Independent Color Games in the Strength Harness
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-28T10:01:00Z'
updated_date: '2026-11-28T12:33:00Z'
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
- [ ] #1 Each color game runs with its own wall limit; a pair is formed only at scoring; an unscored game is reported but does not stop the campaign
- [ ] #2 Default per-game wall comes from the clock: base + (max plies / 2) * increment for each side plus a margin; default Batch max_run_seconds comes from the projected campaign need
- [ ] #3 Strength tests pass; docs/m5/strength-harness.md describes the new behavior and drops the shared-wall caveat
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Also covers archived MSH-238 (Remove the fixed ten-minute experiment cap): crates/mshogi-runner/src/protocol.rs still limits max_game_ms to 600000, while shogi_match.rs and xiangqi_match.rs allow 21600000.

Also remove the fixed job length: tools/strength/batch.py sets SECONDS = 1800 for every match manifest, and anchor_cloud.py starts cleanup 300 s before it, so gcp.py max_run_seconds has no effect. Found in MSH-285, where 4 of 16 Tenjiku 60+1.5 pairs hit the shared wall.

Checkpoint (codex/msh-286-color-games): Independent one-color reports, per-game clock wall defaults, projected Batch runtimes, and updated guides are implemented. The initial strength suite passed. Next: finish shared checks, verify the CLI with native engine processes, run affected-OS CI, obtain review, and integrate.

Review checkpoint (codex/msh-286-color-games): Fresh review found missing catalog diagnostics, a successful exit when every game failed to replay, and evidence upload that could outlive a short Batch task watchdog. Fixed all three and added regressions. Native Tenjiku calibration replayed both colors; two native Chess pairs retained all four wall stops and continued. Next: verify the reviewed candidate on Linux, macOS, and Windows, then integrate.

Validation checkpoint (codex/msh-286-color-games): The shared check passed with Node 24.19.0, including 1,176 Rust tests, 578 frontend tests, 57 strength tests and 25 tooling tests. Linux and macOS process checks passed on 5f2a173. Windows is running in GitHub Actions 46143993677. Updated the Xiangqi protocol guide for the six-hour limit. Next: verify Windows, then integrate.
<!-- SECTION:NOTES:END -->
