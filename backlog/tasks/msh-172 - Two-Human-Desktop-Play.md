---
id: MSH-172
title: Two-Human Desktop Play
status: Ready
assignee: []
created_date: '2026-11-14T12:40:00Z'
labels:
  - handle:two-human-desktop-play
dependencies: []
references:
  - docs/design.md
  - desktop/src/product/ProductApp.tsx
  - backlog/tasks/msh-171 - Rewrite-Main-Promotion.md
priority: medium
type: feature
ordinal: 176000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Two-Human Desktop Play.

**Problem**
The old C++ desktop allows two human players on one computer. The rewrite requires one human side against mShogi. John requested this feature on 2026-11-14 after the branch comparison in MSH-171 (Rewrite Main Promotion).

**Objective**
Allow two people to play locally in the normal desktop application for all six supported games and both Tenjiku profiles. This is a separate feature and does not block MSH-171 (Rewrite Main Promotion).

**Approach**
Extend the shared desktop setup and native participant/session ownership. Reuse native legality, complete-action entry, outcomes, history, persistence and shared controls. Preserve human-versus-engine play. Keep this task local and untimed; online play, engine-versus-engine setup and clock features are outside scope. The implementer records a current plan and selects risk-based checks after inspecting the code.

**Completion**
The normal desktop supports both local human turns, preserves the selected play mode through Save/Open, and has appropriate native/frontend regressions, actual desktop verification and independent review. Documentation states how to select and use the mode.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 New game offers local two-human play through the shared setup for Chess, Shogi, Xiangqi, Tori, Chu and both Tenjiku profiles; existing human-versus-engine setup still works.
- [ ] #2 Each human can play the side to move using native legal complete actions, including applicable drops, promotions and compound routes; turn and player labels are correct, and no automatic engine turn runs in two-human mode.
- [ ] #3 Save/Open preserves the local play mode and complete native history; stopped reopening, history inspection, Take back, native outcomes and resignation have explicit correct behavior for two human sides.
- [ ] #4 Optional read-only analysis remains explicitly requested and cannot apply a move or change participant ownership; switching or opening games retires stale engine work safely.
- [ ] #5 Relevant native and frontend regressions, actual normal desktop journeys and independent review cover both modes, all supported game/profile paths and persistence; verification records the exact source, environment and risk-based platform scope.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
