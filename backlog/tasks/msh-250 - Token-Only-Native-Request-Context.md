---
id: MSH-250
title: Token-Only Native Request Context
status: Ready
assignee: []
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-26T08:22:00Z'
labels:
  - core
  - simplification
  - performance
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 6700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Each native command request carries a full GameContext: the root and the complete event or command history, plus a runtime token in Xiangqi, Tori, Chu and Tenjiku. Callers build it with game.context(), which copies the whole history (about 150 call sites outside tests), and submit compares it field by field. Record replay does this for each event, so replay is quadratic in game length. Shogi submit also clones the whole Game three times per command (its GameUndo keeps full before and after copies). The runtime token alone already changes at each accepted command and at each retirement. This task includes Xiangqi and runs before MSH-227 (Xiangqi Record Replay Speed), as John decided on 2026-11-25; record in MSH-227 what changes for Xiangqi.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Shogi, Xiangqi, Tori, Chu and Tenjiku requests are bound by one small token type without a copy of the history, with the same stale-request refusals as before; Chess keeps its version admission or uses the same token.
- [ ] #2 Shogi admission and undo no longer copy the whole game per command.
- [ ] #3 Rules, perft and record tests pass unchanged; replay time per move at full length is measured before and after for each changed game.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measurement tools that exist (MSH-209): mshogi-core examples record_replay and resource_admission (replay and admission time on generated walks), tenjiku_rules_work (Tenjiku modes play, controls, replay, undo), mshogi-engine example execution_cost.
<!-- SECTION:NOTES:END -->
