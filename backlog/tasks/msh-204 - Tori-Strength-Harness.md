---
id: MSH-204
title: Tori Strength Harness
status: Ready
assignee: []
created_date: '2026-11-19T19:43:00Z'
updated_date: '2026-11-19T19:45:00Z'
labels:
  - handle:tori-strength-harness
  - tori
  - strength
milestone: m-5
dependencies: []
references:
  - docs/m5/strength-roadmap.md
ordinal: 211000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Tori Strength Harness

**Problem.** Tori Shogi searches through the shared core (MSH-165) with a basic evaluation. It has no openings catalog, tactical suite, chosen game-test control or Texel export of the program standard. Tori has no USI-style text protocol, but it does not need one: the native experiment runner already plays clocked Tori games through the structured internal protocol (MSH-179), and no external Tori engines are used (John, 2026-11-19).

**Objective.** The harness step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Tori.

**Approach.** Reuse the clocked native experiment route; openings; a tactical suite from a native mate prover (Tori has drops); a game-test control; Texel export; a time-odds measurement of what speed is worth.

**Completion.** The criteria below are met with recorded evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 At least 100 balanced openings with source and balance rule; a tactical suite of at least 200 positions with native-prover answers and a held-out part, with the baseline score.
- [ ] #2 A game-test control with median late-game depth of at least 8; Texel export with a split; a time-odds measurement (2x time) recorded.
- [ ] #3 Tori games use the existing clocked native experiment route (mshogi-match experiment-run with a Fischer clock, structured internal protocol, MSH-179); any gap for this harness (for example per-player clocks for time odds, or opening catalogs) is closed with tests; no text engine protocol is added, since no external Tori engines are used.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
