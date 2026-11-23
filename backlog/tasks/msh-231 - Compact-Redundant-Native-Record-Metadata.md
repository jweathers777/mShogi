---
id: MSH-231
title: Compact Redundant Native Record Metadata
status: Ready
assignee: []
created_date: '2026-11-23T11:05:00Z'
labels:
  - simplification
  - persistence
milestone: m-6
dependencies: []
priority: high
ordinal: 238000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Split from MSH-229: native.rs plus per-game record codecs repeatedly store game/profile/edition/schema identities, policy strings and replay-derived current positions, effects and conclusions. A saved game needs an unambiguous profile, a complete root and player history; repeated summaries are currently rechecked during every decode. Cut redundant encoded data and reconciliation where no real consumer needs it. Preserve root history/provenance that affects rules and explicitly recorded format compatibility (including the Tori v1 reader); coordinate native consistency findings in MSH-209.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 For each game, identify essential root/command data versus derived or repeated metadata, and remove redundant data in separate reviewed steps
- [ ] #2 New records round-trip all supported profiles, claims, offers and conclusions through native rules; recorded compatibility and independent rules tests remain valid
- [ ] #3 Record encoded size, replay time and source-line changes; pass required Rust/frontend/affected-platform persistence checks after each step
<!-- AC:END -->
