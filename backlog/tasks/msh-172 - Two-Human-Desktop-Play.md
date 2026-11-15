---
id: MSH-172
title: Two-Human Desktop Play
status: In Review
assignee:
  - codex
created_date: '2026-11-14T12:40:00Z'
updated_date: '2026-11-15T14:46:00Z'
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
The old C++ desktop allows two human players on one computer. At task creation, the rewrite required one human side against mShogi. The branch comparison in MSH-171 (Rewrite Main Promotion) identified local two-human play as a missing feature.

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

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a shared, durable participant mode and native actor/engine ownership policy. Preserve the existing human-engine mode.
2. Extend New game and shared controls for both local players, explicit draw responses and side-labelled resignation. Use one-action Take back in two-human games.
3. Verify all seven profiles, history and saved-mode round trips, stale execution retirement, frontend controls and the standalone Tauri host. Run targeted normal desktop journeys and independent review.
4. Record evidence, update user documentation and integrate on main.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex. Integration checkout/branch: /Users/johnweathers/Development/Personal/mShogi, main. Start: a0770114. Independent architecture review: /root/two_human_review. Verification Level 2: participant persistence, native command transport and stale execution are affected; rules, search algorithms and packaging are unchanged. Use targeted native/frontend/host checks and normal macOS desktop journeys. A full release/platform campaign remains a later integration checkpoint.

Implemented durable human_engine/two_humans ownership for all seven native profiles. New game, actor-qualified complete actions, participant labels, draw responses, side-named resignation, one-command Take back, saved mode/history and explicit analysis use the existing shared runtime. Existing documents default to human_engine. Display preferences cannot switch mode.
Independent source review: /root/two_human_final_review. Resolved two findings: Tauri requires the camelCase playMode wire argument; paused local Tori/Chu/Tenjiku boards disable action input. Final follow-up review approved local analysis panel and native complete-action tests.
Actual normal macOS desktop: both humans completed two moves in Chess, Shogi, Xiangqi, Tori, Chu, Adams Tenjiku and Muller Tenjiku. Chess Save/Open preserved two-human mode and two plies, reopened stopped; Take back removed only the last action, then required Resume. Resign Black identified Black; native M2 correctly returned unscored unresolved without a mating witness. Existing human-engine Chess produced a real b8c6 reply. Requested Muller Tenjiku analysis completed at event2 without moving the game. Fixed normal bundle identity reused; no approval/security stores changed.
Final verification record is being completed. Full Linux/Windows package or GUI campaigns were not run for this Level2 scope.
<!-- SECTION:NOTES:END -->
