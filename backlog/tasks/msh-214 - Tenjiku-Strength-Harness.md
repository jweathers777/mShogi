---
id: MSH-214
title: Tenjiku Strength Harness
status: To Do
assignee: []
created_date: '2026-11-20T22:51:00Z'
updated_date: '2026-11-25T11:47:00Z'
labels:
  - tenjiku
  - search
  - model:large
  - effort:high
milestone: m-5
dependencies:
  - MSH-243
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: the harness step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Tenjiku, modelled on MSH-204 (Tori Strength Harness). It gives the later speed, selectivity and evaluation tasks a way to show that a change makes play better. Use the existing clocked native experiment route (mshogi-match experiment-run, structured internal protocol); add no text engine protocol. No external opponents (John, 2026-11-19). Fire Demon burning, range-jumping generals and the Lion moves must be covered by the tactical suite. The board is large and search is slow, so the depth target may need a recorded adjustment.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 At least 100 balanced openings, with their source and the balance rule fixed before screening
- [ ] #2 A tactical suite of at least 200 positions with independently established answers (for example from a native mate prover) and a held-out part, with the baseline score
- [ ] #3 A game-test control at which the median late-game depth is at least 8, or a lower target with its reason and cost recorded; a game test against a frozen predecessor runs end to end in the cloud
- [ ] #4 A self-play export for Texel data with a split, and a time-odds measurement (2x time) recorded
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-11-25 (John): Tenjiku comes last, after the codebase cleanup (MSH-237 to MSH-235). Set to Ready when John releases Tenjiku.

John, 2026-11-25: use the game-independent strength tool of MSH-243 (Game-Independent Strength Tool); do not write new Tenjiku-only copies. MSH-240 removed the earlier per-game tools; MSH-243 records where to read them.
<!-- SECTION:NOTES:END -->
