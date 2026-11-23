---
id: MSH-213
title: Chu Strength Harness
status: In Progress
assignee:
  - codex
created_date: '2026-11-20T22:51:00Z'
updated_date: '2026-11-23T20:59:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies: []
ordinal: 220000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: the harness step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Chu, modelled on MSH-204 (Tori Strength Harness). It gives the later speed, selectivity and evaluation tasks a way to show that a change makes play better. Use the existing clocked native experiment route (mshogi-match experiment-run, structured internal protocol); add no text engine protocol. No external opponents (John, 2026-11-19). Lion double moves, igui and the Lion-capture rules must be covered by the tactical suite.
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
Started from main 30e08e4. Freeze this predecessor for Chu harness measurements; reuse native clocked experiments and shared harness infrastructure. No game-rule, search-strength or evaluation change is planned. Budget cap US$10; heavy runs use Google Cloud Batch, one machine type per comparison. Branch codex/msh-213-chu-strength-harness.
<!-- SECTION:NOTES:END -->
