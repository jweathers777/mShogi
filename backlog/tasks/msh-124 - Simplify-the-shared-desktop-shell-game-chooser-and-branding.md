---
id: MSH-124
title: Simplify the shared desktop shell game chooser and branding
status: Ready
assignee: []
created_date: '2026-10-31T09:02:00Z'
updated_date: '2026-10-31T12:33:00Z'
labels:
  - desktop
  - owner-approved-ux
milestone: m-4
dependencies:
  - MSH-119
documentation:
  - docs/design.md
  - docs/desktop/post-r4-ux.md
  - docs/architecture-migration-plan.md
priority: medium
type: enhancement
ordinal: 124000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved post-MSH-119 product presentation refinement (2026-10-31), preserving the accepted common dark shell and single composition rather than redesigning it. Own section order, compact implemented-game selector, existing side-button presentation, branding and removal of implementation narration from ordinary play. Moves notation/history geometry and document/Resign/Return behavior have separate owners. Retain meaningful native uncertainty, proof and recovery information and legally required notices; this is not permission to delete evidence or native capabilities. Future work only: unassigned and not an implementation plan.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Chess/Shogi/Xiangqi retain one accepted dark application composition, stable document/player/status/board slots and identical sidebar order Analysis, Game Actions, Moves. Remove the ordinary About & technical details sidebar section; no game-specific alternative page or displaced common controls. All boards stay fully visible at supported normal/minimum sizes with existing art, Shogi hands and Xiangqi intersections/palaces/river intact.
- [ ] #2 New Game uses a compact accessible game dropdown listing only currently implemented playable Chess, Shogi and Xiangqi. Future or unsupported variants are absent, including disabled entries; layout can accommodate six when actually delivered without promising them now. Replace only game-description cards. Preserve the existing side-selection button visual treatment and keyboard/radio behavior with White/Black, Sente/Gote, Red/Black and Random, plus native actor/orientation semantics.
- [ ] #3 The application brand uses the existing Shogi selector character instead of the Chess rook. Remove the game-description and untimed/start-at-bottom narration and user-facing reconstruction/native-implementation explanations identified in post-r4-ux.md. Keep understandable game status, uncertainty/unscored outcomes, capacity/saveability, bounded analysis and inspection language; do not conceal errors or invent results.
- [ ] #4 Provide a nonintrusive shared support/developer/About entry outside the gameplay sidebar, accessible by keyboard, retaining exact native diagnostics/provenance/resource facts and bundled license/notices. Keep actual Full/exact result meanings and relevant warnings available to players in Analysis/status/position context, not solely buried as raw diagnostics. Quit remains discoverable through the normal native/application mechanism.
- [ ] #5 Focused frontend tests cover supported-only dropdown contents, absence of unsupported entries, unchanged side labels/button treatment, shared order/copy/brand and support/license availability across all three games. Actual identified packaged normal/minimum three-game comparison captures and pointer/keyboard/focus journeys confirm the refinement without full-board clipping or shell divergence. Separately labeled verification instrumentation is excluded from normal packages.
- [ ] #6 Proportionate independent source/visual/accessibility review resolves material findings. Record exact revisions/packages/platforms, applicable focused checks/CI and normal integration/push, preserving historical MSH-119 approval/evidence and native semantics. Coordinate shared ProductApp/ApplicationView files with the workflow task under one integrator rather than competing edits; no R5 controller expansion or later work starts automatically.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-123 final platform visual handoff (planning note only; task remains Ready/unstarted): at760x650 the lower edge of the focused Moves button gold outline can clip at the scrollable sidebar viewport in Windows all3 and Linux Xiangqi. Top/sides, label and selected-position state remain clear; actual Tab/Enter succeeds and independent123review classifies this nonmaterial. Consider edge spacing/scroll reveal during this existing shell-layout/accessibility scope. Exact unchanged captures and reviewer rationale: docs/review/evidence/msh123/final-platform-visual.md (recorded on123finalization). No dependency/AC/ownership change or implementation begins.
<!-- SECTION:NOTES:END -->
