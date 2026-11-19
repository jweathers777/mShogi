---
id: MSH-204
title: Tori Strength Harness
status: Ready
assignee: []
created_date: '2026-11-19T19:43:00Z'
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

**Problem.** Tori Shogi searches through the shared core (MSH-165) with a basic evaluation, but has no engine protocol and no match route: it is played only through the interactive shell. Without these, no game test, corpus or measurement is possible.

**Objective.** The harness step of the per-game classical strength program (docs/m5/strength-roadmap.md) for Tori. Per John (2026-11-19) no external opponents are sought for games after Xiangqi.

**Approach.** A USI-style engine protocol for Tori using native notation and the clocked persistent-process host shared with Shogi and Xiangqi; openings, a tactical suite from a native mate prover (Tori has drops), a game-test control, Texel export, and a time-odds measurement of what speed is worth.

**Completion.** The criteria below are met with recorded evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A Tori engine protocol (handshake, options, position with native move history, clocked and fixed-work go, bestmove) with tests; native admission of every reply.
- [ ] #2 A clocked Tori match route on the shared host that plays and replays games natively, with tests.
- [ ] #3 At least 100 balanced openings with source and balance rule; a tactical suite of at least 200 positions with native-prover answers and a held-out part, with the baseline score.
- [ ] #4 A game-test control with median late-game depth of at least 8; Texel export with a split; a time-odds measurement (2x time) recorded.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
