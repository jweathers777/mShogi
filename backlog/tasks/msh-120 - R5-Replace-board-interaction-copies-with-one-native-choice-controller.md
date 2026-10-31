---
id: MSH-120
title: 'R5: Replace board interaction copies with one native-choice controller'
status: Done
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-31T10:11:00Z'
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
- [x] #1 One interaction owner replaces selection/inspection state in Chess/synthetic Board, ShogiBoard and XiangqiBoard. It consumes native opaque origins/action identities, ordered route steps and completion alternatives; it owns selection/prefix/candidates, terminal-versus-continuation, completion, cancellation, non-actionable inspection and board focus lifetime.
- [x] #2 Specialized renderers supply board topology/geometry/coordinates/decorations, piece art/orientation, hands/board-adjacent native content, focus geometry and promotion/compound-route completion views. They compose within R4 shared product sections, do not recreate app layouts/panels/guards, and never compute legality/effects.
- [x] #3 Complete native promotion/decline/forced choices, drops, equal-endpoint distinct routes, terminal prefixes with continuation, capture-before-enemy-inspection precedence and cancellation remain exact; partial selection never commits and Xiangqi steps[0] is not generalized into a universal action model.
- [x] #4 Inspection uses authoritative native selected-history context and shared freshness, remains visibly non-actionable and retires on document/history/orientation/capability/unmount changes. Preserve Inspecting {side} {piece} · Esc clears, distinct markers, accessible announcements, Escape and keyboard/pointer focus recovery.
- [x] #5 Delete replaced interaction implementations, obsolete controller adapters/imports and development >9 scrolling assumptions from production paths. Preserve game-specific sizing and full-board visibility without required scrolling/zooming at supported sizes; the labeled 16x16 compound fixture uses the same controller and stays verification-only.
- [x] #6 Existing independently meaningful BoardRoutes/native generated choices/hands/promotion/inspection/stale tests run through the current controller. Actual packaged all-three-game pointer/keyboard/orientation/inspection/completion/focus plus separately labeled synthetic route journeys and independent interaction review support the change; normal packages exclude instrumentation.
- [x] #7 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
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

Packaged checkpoint at7ebab: normalmacOS build/package-smoke PASS, actual CUA Chess/Shogi/Xiangqi keyboardplay+realengine replies, native readonlyinspection andflip, historyReturnlive, Takeback/Resume/nativeSave/quitguard supported. Independent95frontend+56GUI-driver checks PASS. InitialDesktopCI41189536158 Linux14/15GUIphasesPASS; synthetic post-completion probe FAIL: routecontrol focus scrolled the stacked fixture page416px, clippingboard. Keptprobeunchanged; fixture-onlyCSS39ab0c1 uses bounded viewport/remaining-height board and independentlyscrolling diagnostics.23targetedtestsPASS; actualmac normal/min,bothorientations,partial/complete/resized whole16x16boardPASS, independent6capturevisualreviewPASS. NewDesktop41190406764/Rust41190406750 at39ab pending. NoACchecked until final package/CI audit. Historical6509 nativeLinuxfailure matches unchangedMSH112 process missing-response/Brokenpipe race; no runner code changed.

Final acceptance 2026-10-31: production7ebab4648ade6d6be6da6220aefc0948558a23a8 and fixture-only39ab0c1b0a145a97c3bd980a2b216ba743cb8f0c are integrated/pushed to codex/rewrite. One shared native-choice owner replaces all three renderer state machines and the query helper lifecycle; native complete choices, history identity, inspection and focus remain authoritative. All7AC and4DoD now supported by docs/review/native-choice-controller-120.md and durable evidence/msh120.252 frontend/strictTS,8 native inspection,69 external-driver checks pass;23 focused checks after fixture correction pass. Separate reviewer independently ran95 frontend and56 driver checks; completed source/semantic/deletion, actual interaction and direct screenshot review PASS, no material finding remains. Actual normal macOS three-game native play/inspection/focus/Save/Takeback/guards and separate promotion/drop/compound QA documented with bundle hashes. Final Desktop41190406764 at39ab all3platforms PASS; Linux/Windows15/15 actual GUI phases each, verified package/engine identities, native cleanup, normal instrumentation exclusion,7 strict synthetic fitting probes and exact native routes. Original Linux page-scroll and Windows2.78px SVG failures retained, resolved by bounded fixture-only CSS. Native41189535914 at7ebab all3PASS reused for identical native source; duplicate39ab41190406750 mac/WindowsPASS, Linux known MSH112 Brokenpipe/MissingResponse raceFAIL retained. Proposed retry not issued after independent proportionality assessment; no race fix or all39 workflows green claim. Backlog doctor, evidence JSON/screenshot checksums and diff-check pass. No temporary R5 worktree created; unrelated worktrees preserved.123 readonly completed geometry handoff documented without implementing Moves/cues;123/124/125/121 remain unstarted, M4 open.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed R5 with one useNativeChoiceController used by Chess/synthetic Board, the NativeBoard adapter, ShogiBoard and XiangqiBoard. Removed independent selection, route, completion, cancellation, inspection and focus machinery, duplicate inspection lifecycle, and obsolete large-board scrolling assumptions. Specialized geometry, art, hands and native complete-action semantics remain authoritative.

All seven acceptance criteria and four completion items are supported by docs/review/native-choice-controller-120.md and its durable evidence. Verification passed: strict TypeScript, 252 frontend tests, eight native inspection tests, 69 verifier tests, and focused correction tests. A separate reviewer independently passed 95 frontend and 56 verifier checks and completed source, interaction and visual review without unresolved material findings. Actual macOS journeys and Linux/Windows 15-phase GUI campaigns identify packages, sources, native cleanup and instrumentation exclusion.

Integrated source: 7ebab4648ade6d6be6da6220aefc0948558a23a8; verification-only fixture correction: 39ab0c1b0a145a97c3bd980a2b216ba743cb8f0c, normally pushed to codex/rewrite. Desktop CI at 39ab passed all three systems. Reused unchanged native CI at 7ebab passed all three systems; the duplicate 39ab Linux MSH-112 race failure remains recorded and is not claimed fixed. Initial fixture failures are preserved with their verified correction.

MSH-123 receives separate readonly completed-action geometry and owns Moves/history notation and last-move cues. It is now dependency-ready alongside MSH-124 and MSH-125; none was started. M4 remains open. No temporary R5 worktree was created; unrelated worktrees remain untouched.
<!-- SECTION:FINAL_SUMMARY:END -->
