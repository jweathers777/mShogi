---
id: MSH-124
title: Simplify the shared desktop shell game chooser and branding
status: Done
assignee:
  - '@codex'
created_date: '2026-10-31T09:02:00Z'
updated_date: '2026-10-31T14:33:00Z'
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
  - docs/review/shell-124.md
priority: medium
type: enhancement
ordinal: 124000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved post-MSH-119 product presentation refinement (2026-10-31), preserving the accepted common dark shell and single composition rather than redesigning it. Own section order, compact implemented-game selector, existing side-button presentation, branding and removal of implementation narration from ordinary play. Moves notation/history geometry and document/Resign/Return behavior have separate owners. Retain meaningful native uncertainty, proof and recovery information and legally required notices; this is not permission to delete evidence or native capabilities.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Chess/Shogi/Xiangqi retain one accepted dark application composition, stable document/player/status/board slots and identical sidebar order Analysis, Game Actions, Moves. Remove the ordinary About & technical details sidebar section; no game-specific alternative page or displaced common controls. All boards stay fully visible at supported normal/minimum sizes with existing art, Shogi hands and Xiangqi intersections/palaces/river intact.
- [x] #2 New Game uses a compact accessible game dropdown listing only currently implemented playable Chess, Shogi and Xiangqi. Future or unsupported variants are absent, including disabled entries; layout can accommodate six when actually delivered without promising them now. Replace only game-description cards. Preserve the existing side-selection button visual treatment and keyboard/radio behavior with White/Black, Sente/Gote, Red/Black and Random, plus native actor/orientation semantics.
- [x] #3 The application brand uses the existing Shogi selector character instead of the Chess rook. Remove the game-description and untimed/start-at-bottom narration and user-facing reconstruction/native-implementation explanations identified in post-r4-ux.md. Keep understandable game status, uncertainty/unscored outcomes, capacity/saveability, bounded analysis and inspection language; do not conceal errors or invent results.
- [x] #4 Provide a nonintrusive shared support/developer/About entry outside the gameplay sidebar, accessible by keyboard, retaining exact native diagnostics/provenance/resource facts and bundled license/notices. Keep actual Full/exact result meanings and relevant warnings available to players in Analysis/status/position context, not solely buried as raw diagnostics. Quit remains discoverable through the normal native/application mechanism.
- [x] #5 Focused frontend tests cover supported-only dropdown contents, absence of unsupported entries, unchanged side labels/button treatment, shared order/copy/brand and support/license availability across all three games. Actual identified packaged normal/minimum three-game comparison captures and pointer/keyboard/focus journeys confirm the refinement without full-board clipping or shell divergence. Separately labeled verification instrumentation is excluded from normal packages.
- [x] #6 Proportionate independent source/visual/accessibility review resolves material findings. Record exact revisions/packages/platforms, applicable focused checks/CI and normal integration/push, preserving historical MSH-119 approval/evidence and native semantics. Coordinate shared ProductApp/ApplicationView files with the workflow task under one integrator rather than competing edits; no R5 controller expansion or later work starts automatically.
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
1. Keep one primary builder/integrator in the clean primary codex/rewrite checkout at 97b9470. Refine existing ProductApp/ApplicationView/ApplicationShell: one native HTML game select with only implemented games, unchanged side radio buttons and admission/lifecycle callbacks; shared 歩 brand; exact Analysis → Game Actions → Moves order.
2. Move diagnostics, execution provenance, exact historical evidence and all font/license notices into one shared keyboard-accessible About dialog opened outside the sidebar. Retain current native Quit and all player-facing result/warning/Unknown/recovery/capacity distinctions. Simplify ordinary copy without changing native commands, analysis meanings, document controls or R5 selection ownership.
3. Reproduce the MSH-123 minimum-size outline clipping from its actual package captures. Preserve the full focus outline using shared sidebar/inner-scroll gutters and bounded non-board focus reveal, without hiding indicators or altering Moves identities, board geometry or controller state. Add focused accessibility and semantic regressions.
4. Update external packaged GUI verification for the new dropdown, shared section order, About access and truthful copy; prove full board and focused control outline visibility at actual 1180x820 and760x650. Run focused/full frontend and strictTS checks, normal/QA package exclusion and actual three-game package interaction, independent source/visual/accessibility review and applicable CI. Reuse unchanged native correctness evidence; normal integration/push, six AC/four DoD only when supported; stop124.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-123 final platform visual handoff (planning note only; task remains Ready/unstarted): at760x650 the lower edge of the focused Moves button gold outline can clip at the scrollable sidebar viewport in Windows all3 and Linux Xiangqi. Top/sides, label and selected-position state remain clear; actual Tab/Enter succeeds and independent123review classifies this nonmaterial. Consider edge spacing/scroll reveal during this existing shell-layout/accessibility scope. Exact unchanged captures and reviewer rationale: docs/review/evidence/msh123/final-platform-visual.md (recorded on123finalization). No dependency/AC/ownership change or implementation begins.

Startup: root Codex owns MSH-124 in primary codex/rewrite at97b9470943107ae842c8184081c5e8e4cd26ef32; clean checkout, no active In Progress/In Review owner displaced, unrelated worktrees preserved. PATH Backlog1.55.0 differs; pinned project1.55.1 overview/execution guide used for all mutations. MSH-119 dependency and completed120/123 evidence inspected. User attachment requires correction of123focusedge, not optional deferral. ProductApp currently owns chooser/lifecycle; ApplicationShell orders analysis/history/actions/technical; ApplicationView owns diagnostics/capability composition; Dialog already traps select/buttons and restores invoker. No native or125 lifecycle change planned.

Implementation checkpoint: one shared Analysis → Game Actions → Moves composition; supported-only native HTML game selector retains existing side radio buttons and admission/dirty guards. Shared 歩 header About dialog replaces sidebar diagnostics, retaining raw context/provenance/resource facts/licenses/Quit; Xiangqi fast/exact/Unknown/witness meanings remain readable in Analysis. Non-board nested scroll gutters/reveal preserve outward focus rings; R5 and native code unchanged. Focused tests added; independent source review in progress and actual packaged visual/focus verification still required. Initial frontend failures were obsolete copy/order expectations plus a new About trap regression test: jsdom comma-selector ordering placed visible links before summaries; explicit document order now enforced and rerun pending. No failed check treated as passed.

Implementation integrated/pushed checkpoint de5209d3ac0d987d42396a5b2e3bf4325800784d; final production ead30e672f2a2cf1747c02c8daa508babed3b7c2 adds only support paragraph wrapping found by actual GUI/independent visual review. Author full frontend272/25files PASS, strictTS PASS, external GUI-driver71 PASS. Independent source reviewer shell124_review executed141 focused checks +TS PASS; corrected DOM-order About trap finding resolved. Local actual normal all3 at1180 and separately labeled minimum760 package keyboard/history/full-focus checks PASS; final About normal/min wrapping/focus rechecked. Exact manifests, original failing/settling observations and successful captures preserved under docs/review/evidence/msh124/macos. CI and final independent visual/platform audit pending; AC/DoD remain unchecked until evidence complete.

Initial Desktop CI41206493902 atde5209d: macOSpackagePASS; Linux/Windows12of15GUIphasesPASS/3FAIL. Exact report retained. Two failures were obsolete verification copy (historical read-only prose and Recorded native evidence); corrected probes now also require exact historical event/native mode/Return availability and accepted255/256 counts. Real cross-game minimum Analysis height mismatch: LinuxXq221vsChess200, WindowsXq229.046875vsChess200. Shared minimum232px reserves actual content in all3; strict cross-game geometry comparison unchanged. Root external73PASS; independent reviewer checked measured232correction as in-scope. Fresh actualminimumcaptures and CI required before closing finding.

Finalproduction4a624d6c4f129ce774d385192fbbe6a9bdcf547a: freshMac232pxminimum all3captures align; independentreview22MacscreenshotsPASS, externaldriver73independentlyPASS. Finalnormal/QApackagesmoke+exactmanifests recorded. Actual finalMacnativeSELECTtypeahead s→Shogi,x→Xiangqi; retainedsidearrowstoRandom andkeyboardStart admittedBlackorientation/realRedenginefirstturn, no genericactorfallback. NativeCI41207699395 attempt1 separatelyfailed unchangedMac retirement355(LateReplyvsResourceLimit, matching72/76history) andLinuxearlyexit172(MissingResponse/BrokenpipevsUnexpectedExit, existing112). InitialdeNative41206493928all3PASS; noRust/Cargo/tauri diff. BothfocusedtestsPASS locally, notplatformfix. Independentnativefailureassessment preserved; rerun unavailablewhileWindowsmatrixactive, retryoncewhencomplete. RequiredfinalGUI CI41207699334 stillrunning; no prematureDone or nativeassertion/timeoutchanges.

Desktop CI 41207699334 passes all three platforms at 4a624d6; actual Linux/Windows reports each pass 15/15 phases. Artifact audit verifies all 435 Linux and 396 Windows report files, all package-manifest entries, six whole-outline measurements per platform and exact common card rectangles at both sizes. Independent screenshot review then identified F-V03: Linux lacks the circled-i font glyph and shows an empty About box. This is not waived by green automation. Replace only that glyph with an inline SVG in the unchanged labeled 23px button; focused support/shell checks 18/18 pass, strict TypeScript passes. Final Linux vector-icon package capture and review required before Done. Native rerun remains separately pending; no native source changed.

Final production 91eb13e2451af77150b82b62a448bf97e9d5ac58: native CI 41208986510 passes Linux/macOS/Windows. Prior unchanged-source native 41207699395 attempt 2 also passes all three; original attempt 1 failures and independent diagnosis remain preserved, neither race claimed fixed. Final Linux Desktop 41208986515 passes 15/15 actual GUI phases; vector artifact audit verifies 435 report files, 13 normal manifest files, six whole focus bounds and six cross-game layouts. Reviewer personally viewed eight new actual Linux normal/minimum/chooser/About captures and closes F-V03 from Linux pixels; 60 total screenshots reviewed so far. Windows final actual GUI journey remains running. Task stays In Review with criteria unchecked pending final evidence reconciliation.

Final acceptance: independent source/visual/accessibility PASS at 91eb13e; 74 actual screenshots personally reviewed with exact package/hash/focus/layout provenance. Final native CI 41208986510 and Desktop CI 41208986515 all three platforms PASS; Linux/Windows each 15 actual GUI phases. Full final artifact audit verifies all 831 report files, 20 normal package manifest entries and 12 complete focus outlines. Six AC and four DoD now supported; documentation links, doctor and diff checks pass. Final completion commit carries task/design/review/evidence only; production already integrated and pushed at 91eb13e. MSH-125 remains next dependency-ready and unstarted, MSH-121 retains integrated acceptance, M4 open.

Final staged whitespace check found Windows CRLF/trailing spaces in retained report/log evidence. Normalized repository text only, verified JSON semantic equality, recorded original/normalized byte hashes in text-normalization.json, retained original artifact ZIP identity and all image hashes. Both extraction scripts reproduce normalization; full artifact audits rerun PASS and staged diff check PASS. This mechanical evidence correction changes no product behavior or accepted observation.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered the bounded shared presentation refinement on codex/rewrite, production source 91eb13e2451af77150b82b62a448bf97e9d5ac58 (initial de5209d, About wrapping ead30e6, measured shared minimum 4a624d6). One accepted dark shell owns Analysis → Game Actions → Moves, stable document/player/board/footer slots, shared 歩 branding and keyboard-accessible header About. The native game dropdown lists only Chess/Shogi/Xiangqi; existing side radio appearance, arrows, native labels, Random and lifecycle admission remain. Concise ordinary language retains essential warnings, claims/declarations, uncertainty, failures, capacity/saveability and readable Xiangqi fast/exact historical meanings. About retains raw context, provenance, resource facts, both bundled font notices, OFL/download and unchanged native Quit.

MSH-123's outward focus ring remains fully visible through eight-pixel nested gutters and bounded sidebar focus reveal. Actual packaged comparisons at 1180x820 and 760x650 establish full boards, native art/hands/intersections/river/palaces, common geometry, exact Moves/history/cues and pointer/keyboard accessibility. Independent source, visual and accessibility review resolves the DOM-order trap finding, F-V01 URL wrapping, F-V02 measured shared 232px minimum and F-V03 Linux missing About glyph using actual final Linux vector captures. Original failed checks and screenshots are preserved.

Author: 272 frontend tests/25 files and strict TypeScript PASS. Independent reviewer: 141 focused checks plus TypeScript, five final Support checks plus TypeScript, 73 driver checks, and personally viewed identified screenshots (exact count/method in review). Final Desktop CI 41208986515 and native CI 41208986510 pass all three platforms; Linux/Windows each pass 15 actual GUI phases. Normal-package smoke excludes instrumentation and exercises the real bundled engine. Exact source/package/environment/hash audits, screenshots, failed checks and review methods: docs/review/shell-124.md and docs/review/evidence/msh124/. Local Mac native CUA interaction is distinct from Mac CI packaging and remote Linux/Windows native-input GUI checks. Prior native attempt-1 timing/classification failures and the successful unchanged-source retry remain documented; neither race is claimed fixed. No Rust, archive, engine, capacity or R5 controller contract changes.

All six acceptance criteria and four completion items have supporting evidence. MSH-125 lifecycle changes remain unimplemented: Save/Save as/Open, replacement/quit guards, Resign confirmation and explicit Return/Resume retain current behavior. MSH-125 is next dependency-ready and unstarted; MSH-121 retains final integrated acceptance; M4 remains open. No task worktree created; unrelated worktrees preserved. Normal integration and push use codex/rewrite. The final completion commit contains documentation/task/evidence only; its actual revision is reported by the integrator.
<!-- SECTION:FINAL_SUMMARY:END -->
