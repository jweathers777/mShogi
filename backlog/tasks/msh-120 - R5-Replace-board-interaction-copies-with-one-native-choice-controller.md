---
id: MSH-120
title: 'R5: Replace board interaction copies with one native-choice controller'
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-31T09:38:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-119
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
  - docs/desktop/post-r4-ux.md
  - docs/review/native-choice-controller-120.md
priority: medium
type: task
ordinal: 120000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One interaction owner replaces selection/inspection state in Chess/synthetic Board, ShogiBoard and XiangqiBoard. It consumes native opaque origins/action identities, ordered route steps and completion alternatives; it owns selection/prefix/candidates, terminal-versus-continuation, completion, cancellation, non-actionable inspection and board focus lifetime.
- [ ] #2 Specialized renderers supply board topology/geometry/coordinates/decorations, piece art/orientation, hands/board-adjacent native content, focus geometry and promotion/compound-route completion views. They compose within R4 shared product sections, do not recreate app layouts/panels/guards, and never compute legality/effects.
- [ ] #3 Complete native promotion/decline/forced choices, drops, equal-endpoint distinct routes, terminal prefixes with continuation, capture-before-enemy-inspection precedence and cancellation remain exact; partial selection never commits and Xiangqi steps[0] is not generalized into a universal action model.
- [ ] #4 Inspection uses authoritative native selected-history context and shared freshness, remains visibly non-actionable and retires on document/history/orientation/capability/unmount changes. Preserve Inspecting {side} {piece} · Esc clears, distinct markers, accessible announcements, Escape and keyboard/pointer focus recovery.
- [ ] #5 Delete replaced interaction implementations, obsolete controller adapters/imports and development >9 scrolling assumptions from production paths. Preserve game-specific sizing and full-board visibility without required scrolling/zooming at supported sizes; the labeled 16x16 compound fixture uses the same controller and stays verification-only.
- [ ] #6 Existing independently meaningful BoardRoutes/native generated choices/hands/promotion/inspection/stale tests run through the current controller. Actual packaged all-three-game pointer/keyboard/orientation/inspection/completion/focus plus separately labeled synthetic route journeys and independent interaction review support the change; normal packages exclude instrumentation.
- [ ] #7 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
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
1. Establish shared native-choice/presentation types and one controller for selection, ordered prefix/candidates, terminal/continuing paths, alternative completion, inspection retirement, complete-action submission and focus/keyboard lifetime. Keep native query validation separate from selection but retire duplicated request owners. 2. Migrate actual Chess/synthetic Board, ShogiBoard and XiangqiBoard consumers; preserve specialized geometry/art/hands/completion presentation and common shell, delete old independent handlers/state and large-board scrolling heuristics. Supply a separate read-only completed-geometry extension boundary for123 without deriving or rendering its new cues. 3. Port independently meaningful native-generated choice/route/promotion/drop/inspection tests and add common-controller context, stale, double-submit, focus and historical regressions. 4. Verify focused frontend/native consumers/static checks, actual normal/min packaged all-three-game interaction and separately labeled synthetic routes; independent source/interaction review, applicable CI, exact evidence, normal integration/push and120completion only.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner-approved planning coordination2026-10-31: original R5 criteria and direct dependency119 preserved. New123 owns completed last-move notation/history/geometry presentation after120; R5 remains controller extraction plus specialized rendering integration, not this product feature.124/125 own shell and lifecycle changes,121 waits for allthree. New read-only cue context must remain separate from selection/inspection/focus state;123 consumes the surviving boundary rather than adding a competing controller. No R5 implementation or detailed future plan begins.

Startup2026-10-31: root Codex owns120 in primary codex/rewrite at6509dd3ea7db60a10f95e5fc50768668596f39f3; clean/fetched aligned, no InProgress/InReview task owner, unrelated worktrees retained. GlobalBacklog1.55.0 mismatch resolved with pinned1.55.1 and overview/execution read.119Done dependency/evidence preserved;123/124/125/121 remainunstarted. Read current request/design A09-A11, migration/source review/post-R4 planning and actual Board/ShogiBoard/XiangqiBoard/query/fixtures. Existing three controllers own parallel route/inspection/focus; Board hardcodes >9 scrolling and Xiangqi steps[0]. Root owns shared contract/controller/Chess migration and integration; any renderer/test helper work is isolated behind that contract, separate independent reviewer owns no implementation. No native rules/archive/engine/profile/limits change planned.

R5 source boundary ready: one useNativeChoiceController used by Board/NativeBoard/synthetic/Shogi/Xiangqi; removed renderer state/query/focus handlers and >9scrollheuristics, current presentationtypes migrated. Stateless native inspection validator retains exactidentity; shared controller owns lifecycle and sync stalecompletion/doublesubmit latch.252frontend+strictTS PASS;8focusednativeinspection PASS;69externaldriverchecks PASS, actualpackage/CIpending. Independent bounded review resolvedoldprioritydivergence: markedreadonlyroute destination previewsbeforefriendlyswitch, regularnativecapturepriorityunchanged; explicitChesssyntheticassertionported, actualXqexpectedcaseunchanged. GenericforcedShogi singleaction nowautosubmits likeproduct; AppFocus remountidentityassertionportedtofocus+retirement. CompletedActionGeometry readonlypropsonly;123notimplemented. Fullindependentboundaryreview ongoing, package/CI notyetacceptanceevidence; noACchecked.
<!-- SECTION:NOTES:END -->
