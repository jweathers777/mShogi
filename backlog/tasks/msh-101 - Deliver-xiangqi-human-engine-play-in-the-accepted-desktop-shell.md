---
id: MSH-101
title: Deliver xiangqi human-engine play in the accepted desktop shell
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-28T18:38:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-76
  - MSH-97
  - MSH-96
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/desktop-product.md
ordinal: 101000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Implement actual untimed human-versus-own-engine Xiangqi through the shared native session in the accepted MSH-74 dark shell. Application header/theme/document controls/live heading/board-sidebar grammar and identical analysis-history-actions-technical order remain stable across games.
- [ ] #2 Provide a deliberate intersection board with river/palaces and Xiangqi-specific native pieces/Red/Black labels, complete board visible without scroll or zoom at supported normal and minimum desktop sizes. No generic square-board visual assumption, shogi hands or promotion UI leaks into this game.
- [ ] #3 Complete click/keyboard/orientation/action confirmation and native read-only opponent inspection with non-actionable markers; retain common live/historical footer, engine state and actual play limits. Keep raw diagnostics in details and instrumentation out of normal packages.
- [ ] #4 Focused frontend/native regressions and actual running packaged play/inspection at same sizes beside chess/shogi screenshots support independent interaction/visual review. No claim of final game delivery before remaining journeys and review.
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
1. Preserve the accepted MSH-74 ApplicationShell/GameArea/theme and current native Xiangqi contracts. Root Codex is sole builder/integrator on the clean primary codex/rewrite checkout at f71e149; prerequisites76/97/96 are Done. Use pinned Backlog1.55.1 and current Rust1.99.1. Read-only independent font/native audits inform narrow changes.
2. Implement only the real product integration needed for the mandatory early visual checkpoint: native standard-Xiangqi New Game admission with human-bottom orientation and existing bounded supervised engine, strict complete move and native token-bound inspection; retain archive/capacity/identity protections. Preserve MSH-102 ownership of full analysis/outcome/recovery/document/history journeys.
3. Add a dedicated intersection renderer inside the existing shared player/board/footer slots, using native90-point/river/palace/orientation/piece/complete-choice metadata. Implement approved warm honey board, ivory round upright discs with differentiated Red/Black Chinese glyphs from a separately bundled licensed Traditional Chinese subset; retain semantic accessible names. Reuse native inspection freshness and actionable-versus-hollow inspection visual conventions, capture precedence, pointer/keyboard/Escape and responsive width/height fitting.
4. Run focused native product-admission and frontend interaction/shared-shell regressions, type/format/lint checks, independent bounded implementation and screenshot review. Build and inspect the normal macOS package; exercise actual native play/selection/inspection/orientation and capture normal1180x820, minimum760x650 and intermediate sizing plus matching Chess/Shogi comparisons. Record exact source, package hashes, environment, commands, screenshots and limitations without claiming GUI from unit tests.
5. Commit/push the reviewable checkpoint through the normal integration workflow, keep MSH-101 unfinished with all unproven criteria unchecked, and STOP for explicit owner approval of actual screenshots before remaining implementation/finalization. Do not start102/103/106 or close45/M4.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner attachment2026-10-28 approves board/piece visual direction only and requires an early actual normal packaged visual checkpoint. Surrounding Chess/Shogi dark application shell remains authority; no redesign. Root owns implementation/integration in primary checkout; no new worktree needed. Startup clean f71e149 (newer user toolchain pin1.99.1 preserved), origin refreshed equal, no other active task owners. PATHBacklog1.55.0 is incompatible for mutation; pinned1.55.1 used. Existing98/100/111 worktree cleanup handoffs remain separate. Independent read-only font audit recommends separate TC2.003 OFL subset preserving accepted JP asset; native audit confirms existing strict admission/inspection/archives suffice with narrow New Game/product command gates. Full Xiangqi Open/analysis/history/outcome journeys remain102; no completed-delivery claim at checkpoint.

Early checkpoint source implemented (unfinished task): real product standard-Xiangqi creation goes through existing native archive admission and journaled Resume, human-bottom Red/Black, complete native Play wrapper and token-bound movement inspection. Dedicated SVG90-intersection renderer/river/nativepalaces, upright approved discs/glyphs and separate TC font subset; actual ApplicationShell/GameArea/theme retained. Native flat Xiangqi settings are represented explicitly in a TS discriminated union; no archive/schema normalization. Independent test author supplied18 UI tests using actual Rust-generated snapshots/query replies; caught and corrected flat-settings access and extra game_kind in strict Flip preferences. Root strictTS+182frontend tests,127all-feature native session tests, nativeproduct6tests, root/sessionclippy/fmt and normalmacOS build/package smoke passed (Rust1.99.1; Node24.19 runtime path, PATHnode26 broken dylib was bypassed without system mutation). Independent implementation reviewer read bounded source and ran18tests+TS, no unresolved finding.

Actual normal app first-pass CUA comparison at1180x820 exposed coordinate occlusion under edge discs and XQ empty-slot spacing8/14px drift against Chess/Shogi. Independent visual review confirmed. Corrected art margin90/viewBox980x1080, equalpitch100 andratio49/54; explicit common empty notice-slot spacing preserves accepted existing adapter positions; native history row uses current shared treatment. Separate source re-review no finding; normal app rebuild and fresh actual comparison/selection/minimum captures pending. Initial test failures/setup mistakes remain notes, not product semantic changes. CUA actual drag reached exact760x650 (1520x1300 raw), full board/player/footer fit; no final corrected minimum evidence yet. Owner checkpoint approval remains REQUIRED before remaining101implementation/finalization; all AC/DoD remain unchecked, no102/103/106started.
<!-- SECTION:NOTES:END -->
