---
id: MSH-101
title: Deliver xiangqi human-engine play in the accepted desktop shell
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-29T07:53:00Z'
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
  - docs/m4/evidence/msh101/visual-checkpoint-2026-10-28/README.md
  - docs/m4/evidence/msh101/completion-2026-10-29/README.md
ordinal: 101000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Implement actual untimed human-versus-own-engine Xiangqi through the shared native session in the accepted MSH-74 dark shell. Application header/theme/document controls/live heading/board-sidebar grammar and identical analysis-history-actions-technical order remain stable across games.
- [x] #2 Provide a deliberate intersection board with river/palaces and Xiangqi-specific native pieces/Red/Black labels, complete board visible without scroll or zoom at supported normal and minimum desktop sizes. No generic square-board visual assumption, shogi hands or promotion UI leaks into this game.
- [x] #3 Complete click/keyboard/orientation/action confirmation and native read-only opponent inspection with non-actionable markers; retain common live/historical footer, engine state and actual play limits. Keep raw diagnostics in details and instrumentation out of normal packages.
- [x] #4 Focused frontend/native regressions and actual running packaged play/inspection at same sizes beside chess/shogi screenshots support independent interaction/visual review. No claim of final game delivery before remaining journeys and review.
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
1. Preserve the accepted MSH-74 ApplicationShell/GameArea/theme and current native Xiangqi contracts. Root Codex is sole builder/integrator on the clean primary codex/rewrite checkout at f71e149; prerequisites76/97/96 are Done. Use pinned Backlog1.55.1 and current Rust1.99.1. Read-only independent font/native audits inform narrow changes.
2. Implement only the real product integration needed for the mandatory early visual checkpoint: native standard-Xiangqi New Game admission with human-bottom orientation and existing bounded supervised engine, strict complete move and native token-bound inspection; retain archive/capacity/identity protections. Preserve MSH-102 ownership of full analysis/outcome/recovery/document/history journeys.
3. Add a dedicated intersection renderer inside the existing shared player/board/footer slots, using native90-point/river/palace/orientation/piece/complete-choice metadata. Implement approved warm honey board, ivory round upright discs with differentiated Red/Black Chinese glyphs from a separately bundled licensed Traditional Chinese subset; retain semantic accessible names. Reuse native inspection freshness and actionable-versus-hollow inspection visual conventions, capture precedence, pointer/keyboard/Escape and responsive width/height fitting.
4. Run focused native product-admission and frontend interaction/shared-shell regressions, type/format/lint checks, independent bounded implementation and screenshot review. Build and inspect the normal macOS package; exercise actual native play/selection/inspection/orientation and capture normal1180x820, minimum760x650 and intermediate sizing plus matching Chess/Shogi comparisons. Record exact source, package hashes, environment, commands, screenshots and limitations without claiming GUI from unit tests.
5. Commit/push the reviewable checkpoint through the normal integration workflow, keep MSH-101 unfinished with all unproven criteria unchecked, and STOP for explicit owner approval of actual screenshots before remaining implementation/finalization. Do not start102/103/106 or close45/M4.

Completion continuation 2026-10-29: owner explicitly approves the actual3a1 visual checkpoint and authorizes completion. Root remains sole builder/integrator on clean primary codex/rewrite at02f4223, refreshed origin equal. (1) Replace native Xiangqi PresentedEvent Debug labels with exhaustive concise native-side/coordinate/capture/check and event-kind labels, keeping canonical payloads/event identities/replay untouched; test actual quiet/capture/check sequences and non-board/prefix semantics. (2) Add one small shared inspection-status formatter over the native label, used by Chess Board, ShogiBoard and XiangqiBoard: exact visible Inspecting {side} {piece} · Esc clears. Preserve accessible read-only announcements, hollow markers and all native inspection admission; add all-three regression assertions and durable shared product wording. (3) Run focused native/frontend/shared-shell checks and static checks, independent bounded source/interaction review, build normal macOS product and exercise genuine played history plus refreshed inspection across games. Reuse unchanged approved visual/size/native context evidence; label any sizing/GUI/platform evidence distinctly. (4) Integrate tested source and push, verify relevant CI; then CLI-finalize101 only when originalAC/DoD supported, record exactsource/evidence, stop without102/103/106 or45/M4closure.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner attachment2026-10-28 approves board/piece visual direction only and requires an early actual normal packaged visual checkpoint. Surrounding Chess/Shogi dark application shell remains authority; no redesign. Root owns implementation/integration in primary checkout; no new worktree needed. Startup clean f71e149 (newer user toolchain pin1.99.1 preserved), origin refreshed equal, no other active task owners. PATHBacklog1.55.0 is incompatible for mutation; pinned1.55.1 used. Existing98/100/111 worktree cleanup handoffs remain separate. Independent read-only font audit recommends separate TC2.003 OFL subset preserving accepted JP asset; native audit confirms existing strict admission/inspection/archives suffice with narrow New Game/product command gates. Full Xiangqi Open/analysis/history/outcome journeys remain102; no completed-delivery claim at checkpoint.

Early checkpoint source implemented (unfinished task): real product standard-Xiangqi creation goes through existing native archive admission and journaled Resume, human-bottom Red/Black, complete native Play wrapper and token-bound movement inspection. Dedicated SVG90-intersection renderer/river/nativepalaces, upright approved discs/glyphs and separate TC font subset; actual ApplicationShell/GameArea/theme retained. Native flat Xiangqi settings are represented explicitly in a TS discriminated union; no archive/schema normalization. Independent test author supplied18 UI tests using actual Rust-generated snapshots/query replies; caught and corrected flat-settings access and extra game_kind in strict Flip preferences. Root strictTS+182frontend tests,127all-feature native session tests, nativeproduct6tests, root/sessionclippy/fmt and normalmacOS build/package smoke passed (Rust1.99.1; Node24.19 runtime path, PATHnode26 broken dylib was bypassed without system mutation). Independent implementation reviewer read bounded source and ran18tests+TS, no unresolved finding.

Actual normal app first-pass CUA comparison at1180x820 exposed coordinate occlusion under edge discs and XQ empty-slot spacing8/14px drift against Chess/Shogi. Independent visual review confirmed. Corrected art margin90/viewBox980x1080, equalpitch100 andratio49/54; explicit common empty notice-slot spacing preserves accepted existing adapter positions; native history row uses current shared treatment. Separate source re-review no finding; normal app rebuild and fresh actual comparison/selection/minimum captures pending. Initial test failures/setup mistakes remain notes, not product semantic changes. CUA actual drag reached exact760x650 (1520x1300 raw), full board/player/footer fit; no final corrected minimum evidence yet. Owner checkpoint approval remains REQUIRED before remaining101implementation/finalization; all AC/DoD remain unchecked, no102/103/106started.

Early actual packaged visual checkpoint integrated/pushed at 3a1aefc73c4b956a193059bb99f114af5b69ebe9. Evidence: docs/m4/evidence/msh101/visual-checkpoint-2026-10-28/README.md, raw screenshots/AX, screenshot hashes/dimensions, exact three package identities, package smoke and native/desktop CI records. Normal1180x820 actual normal.app, initial Red/Black-bottom, native friendly selection and read-only inspection; measured760x650 and960x740 captures use separately labeled normal-feature product packages with initial window dimension/title/name/id overrides, no fixture/harness/desktop-verification feature. Runtime resize automation of the unchanged normal artifact was unreliable and is NOT established. All captured sizes show full board/player/footer, same Chess/Shogi shell. Native pointer occupied-capture precedence and keyboard a3a4 + real engine reply exercised; Black-human assignment and inspection non-action/Escape exercised. Separate implementation/screenshot reviewer independently ran31focusedtests+TS, confirmed original coordinate/spacing findings resolved and no remaining board/shell/sizing/marker finding. Found raw native Debug text in played-game history labels: preserve as unfinished101 presentation work after mandatory early visual approval, with native-readable labels/tests/refreshed played capture; do not transfer to102 or imply current raw labels approved. Root182frontend and127all-feature session tests, format/clippy, normal macOS build/smoke passed. Exact3a1 native CI40747229601 and desktop CI40747228852 all three systems passed; existing Windows/Linux GUI campaigns are Chess/Shogi compatibility, not Xiangqi GUI. Local Xiangqi GUI evidence is macOS only. MSH101 remains In Progress with original AC/DoD unchecked; STOP for explicit owner visual approval under attachment checkpoint. No102/103/106 started;45/M4 remain open.

Owner attachment bfc8a6e2 explicitly approves board/pieces/markers/sizing/shared-shell at3a1aefc; early visual gate is satisfied, not the remainingAC. Additional owner wording is exact common visible Inspecting {side} {piece} · Esc clears across Chess/Shogi/Xiangqi. No redesign or inspection-semantic change. PATHBacklog1.55.0 remains incompatible; pinned1.55.1 confirmed and used. Existing completed98/100/111 worktrees remain unrelated prior cleanup handoffs; no new worktree needed.

Completion source ready for fresh packaged GUI evidence: native PresentedEvent.label now exhaustively uses existing native coordinates/derived capture/check/event facts; no canonical or rules schema changes. One shared inspectionStatus(nativeLabel) owns exact visible Inspecting {side} {piece} · Esc clears in Chess/Shogi/Xiangqi, with unchanged sr-only explanation and native non-actionable/freshness/key paths. Native generator adds actual capture/recapture frontend fixture; existing fixtures unchanged. Root183frontend/23files + strictTS passed; native2label tests and14focused session tests passed; sessionclippy alltargets/allfeatures -Dwarnings andfmt/diffchecks passed. An initial misnamed native filter selected0tests and is not counted; corrected14test suite passed. Separate read-only reviewer independently passed82focusedfrontend+TS and2+14native tests, no source findings. Actual final normal.app played-history/inspection capture and current CI remain required beforeDone.

Exact reviewed/tested normal product sourcebe3055a1bc8d4b0d0fdf11f8509b97076d13fa43 integrated/pushed. Actual final macOS normal.app GUI: exact inspection language captured acrossChess/Shogi/Xiangqi; nativeXQinspectiondestinationstays0events andEscfocusreturnsb7. Redfriendlyb2b9capture+nativeBlacka9b9reply2plies shows readablehistory; a3UpReturn+engine4plies; Flipkeepsgame; BlackhumanassignmentRedengineopeningthenBlacka9LeftSpaceb9capture+Redreply3plies. Nine normal1180x820 actualJPEG/AX captures at docs/m4/evidence/msh101/completion-2026-10-29; no fixture/configoverride/instrumentation. Normalpackage-smoke passed. Separate reviewer independently inspectedall9, no sourceorvisualfindings; root183frontend+2label+14sessiontests passed, reviewer82frontend+2+14native+TS. Approvedunchangedboard/font/sizing evidence3a1 reused with separatelyconfiguredminimum/intermediate provenance and no runtime-resize claim. NativeCI40834914585/DesktopCI40834914598 pending; AC/DoD leftuncheckeduntilactualoutcomes. No102/103/106started.

Initial final-source desktopCI40834914598 LinuxGUIjob123317331840 exposed a stale externalverifier assertion, still requiring read only in ordinary visible inspection status. This contradicts approved newcopy; Chess/Shogi normaljourneys abort and normal-shogi-reopened then lacks their savedinput. This failedcheck is preserved, not a claimedpass or MSH111transportfinding. Updated only external gui-smoke.py to observe status span(orChessparagraph) and require EXACT Inspecting nativeLabel · Esc clears; all destinationaria/hollowmarker/nativecontext/nonactionable route/Escape checks remain. Addedexternalcopyregressions; root and independentreviewer24driver tests passed. Independent source review nofindings, no weakenedchecks/retrybehavior. Productsource and normal.app unchangedfrombe3055a; freshCI at nextverification-scriptrevision required beforeDone.

Final independent evidence/AC review PASS: exact1c8a168 native and desktop matrices all3jobs success; relevant steps and screenshot/package hashes verified. OriginalAC1–4 supported, no remaining finding. Current handoff/session/shared-handoff docs reconcile the historical frontend gate without rewriting earlier evidence. Root finalizes101 only and records the normal evidence/status commit; source references above are actual integrated commits, not a fabricated future hash. No102/103/106 work started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered native untimed Xiangqi human-versus-mShogi play in the owner-approved shared desktop shell, with dedicated intersection geometry/traditional discs, Red/Black assignment/Flip, pointer and keyboard complete actions and native read-only inspection. Completion fixes visible Debug history using native coordinate/capture/check/event labels and standardizes all three games to exact visible Inspecting {side} {piece} · Esc clears; native semantics, accessible explanations and stale-result protections remain unchanged. Durable all-game wording is design A-07.

Integrated/tested product source: be3055a1bc8d4b0d0fdf11f8509b97076d13fa43 on codex/rewrite; external-verifier-only follow-up 1c8a1682db5b98e765eb755b7978537c20a2ddd2. Actual normal macOS app verified Red/Black human play, engine replies, captures, keyboard/Flip/Escape and all-three inspection language. Nine refreshed raw GUI/AX captures and package hashes: docs/m4/evidence/msh101/completion-2026-10-29/README.md. Owner-approved unchanged3a1 normal/minimum/intermediate evidence retained with separate sizing-package and runtime-resize limitations explicit.

Root183frontend tests/strictTS,2native label tests,14native session tests,24externaldriver tests, clippy/fmt/diffchecks and normalpackage smoke passed. Separate read-only agent independently passed82focusedfrontend+TS,2+14native and24driver tests, reviewed source/all9GUIimages/evidence, and found no remaining blocker. Final nativeCI40836312043 and desktopCI40836312034 passed Windows/macOS/Linux at exact1c8a168. Initial40834914598 Linux/Windows GUI failure due obsolete visible-read-only assertion is preserved; corrected verifier keeps semantic/marker/context/Escape checks and passes fresh actual compatibility journeys.

All original AC/DoD supported; no remaining101finding. Xiangqi GUI here is macOS; Windows/Linux checks include actualChess/Shogi compatibility and do not establish Xiangqi GUI there. Depth1/128 ordinary limits and native profile/fast-exact semantics unchanged. MSH102 owns full analysis/outcome/recovery/document/history/live/resume journeys and remains unstarted, as do103/106. MSH45/M4 remain open. No new worktree; primary remains codex/rewrite. Stop after101.
<!-- SECTION:FINAL_SUMMARY:END -->
