---
id: MSH-74
title: Deliver shogi desktop play analysis saving and historical inspection
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-27T08:39:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-73
  - MSH-70
  - MSH-68
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/desktop-session.md
  - docs/m4/shogi-desktop-product.md
type: task
ordinal: 74000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Native/controller and frontend checks plus actual packaged shogi GUI journeys/screenshots; distinguish local game-specific evidence from final three-platform cross-game gate.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The actual desktop can select standard shogi,play either human side against the bundled native engine,and present readable9x9 owner-oriented pieces,hands,turn,history and complete native choices.
- [x] #2 Drops and optional/mandatory promotions work by click and keyboard with clear cancellation/focus. Declaration eligibility versus committed formal action,outcomes,resignation,recovery and failures have profile-correct meanings.
- [x] #3 Bounded read-only analysis and live engine work remain separate; cancellation/replacement/profile switching/historical selection preserve exact context and never apply stale or wrong-purpose actions.
- [x] #4 Complete Save/Open and history inspection preserve hands,profile,commands/events,conclusions and observation provenance. Reopen is stopped; Return to live/Resume and unsaved replacement/quit guards remain explicit.
- [x] #5 Running native application evidence covers full shogi journeys at supported sizes and shogi capacity/defaults. Unsupported chess Full or timed restoration is rejected clearly,not silently relabeled or downgraded.
- [x] #6 Product design shows the complete standard 9x9 board without scrolling at supported normal desktop window sizes; standard shogi does not inherit synthetic large-board scrolling or assume zoom/pan is needed.
- [x] #7 Deliberate shogi-specific piece presentation preserves Sente/Gote orientation, ownership and promoted semantics; pieces-in-hand/drop and promotion choices form integrated player interaction rather than diagnostic button rows.
- [x] #8 The player interface has coherent player, turn and game-status hierarchy with move/history and analysis presentation that does not crowd the board; polished keyboard and pointer operation is verified in the running native application.
- [x] #9 Developer/raw diagnostic information is separated from normal play. The MSH-72 DEVELOPMENT CONTRACT harness is not evidence of approved/completed visual design and is not used as the MSH-74 design basis or acceptance target.
- [x] #10 Explicit New Game selects Chess/Shogi and native human side; normal default orientation follows the human with Flip. Japanese pentagonal pieces use a cross-platform licensed font with provenance; native semantics remain independent of glyphs.
- [ ] #11 Actual normal packaged screenshots cover Sente/Gote, both hands, drop selection, optional promotion, history, active analysis, conclusion and minimum size; independent visual review resolves material findings and owner explicitly approves the polished surface before Done.
- [x] #12 Chess and Shogi use the same actual shell/component structure and stable locations for header/runtime/document actions, title/dirty status, live heading/Flip, board/player region, Analysis→History→Game actions and collapsed technical details, retaining distinct game-appropriate visual identities.
- [x] #13 Native token-bound read-only movement inspection supports opponent board pieces and Shogi available hands, both orientations and exact historical positions, with complete routes/continuations, no authoritative or execution mutation, no executable enemy actions, distinct hollow markers/accessibility names and pointer/keyboard/switch/Escape tests plus actual normal packaged screenshots.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Implement the recorded normal product design in docs/m4/shogi-desktop-product.md; retain MSH72/73 harness as verification-only and preserve M3 chess behavior. 2. Extend normal native host game/session/file APIs over established controller/archive with typed New Game selection, both sides and authoritative status/choice data; focused native/host tests, no TS rules. 3. Build warm grid/pentagonal owner-oriented kanji pieces with bundled licensed Japanese font, compact native hands, contextual complete-action promotion, adaptive full-board fit and keyboard/focus; obtain actual packaged macOS visual evidence during implementation. 4. Integrate normal New Game/analysis/history/recovery/outcomes/SaveOpenResume/unsavedQuit with exact native identity protections, clean player-facing language and optional diagnostics; focused frontend/native/process regressions. 5. Capture complete normal packaged screenshot/journey set at1180x820 and760x650, independently review implementation and visuals, run native/desktop CI and Windows/Linux packaged GUI evidence, integrate/push codex/rewrite. 6. Present launchable macOS build/screenshots for owner visual review and leave task In Review until explicit approval; do not start75 or close M4/41–47.

Owner refinement2026-10-27, primary integrator Codex, clean primary codex/rewrite at aafcbfb: (1) document unified shell hierarchy and native read-only movement-inspection semantics; retain In Review and all prior evidence. (2) Add a small token-bound read-only native query using exact selected position, side-qualified complete choices and no authoritative mutation or work polling; native fixtures/context-preservation tests. (3) Share header/document/status/board/sidebar/actions/details component structure across distinct Chess dark and Shogi light themes; ordered Analysis→History→Actions and fixed document location. (4) Extend both board renderers with native inspection, compound route staging, burgundy hollow markers/occupied rings, keyboard/Escape and explicit accessible labels; preserve actionable friendly choices and guard stale query responses. (5) Focused tests, independent implementation and visual review, normal packaged Mac journeys and updated Chess/Shogi/selection/inspection/hand screenshots, applicable CI, normal integration/push. Reuse unchanged archive/rules/platform evidence; stop In Review for new owner visual approval, do not begin75.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner clarification during MSH-72, 2026-10-26: MSH-74 must perform actual shogi UI design and implementation. Preserve existing semantic/play/analysis/archive/application criteria; added product criteria define full-board fit, deliberate owner-oriented/promoted shogi pieces, integrated hand/drop/promotion interaction, visual hierarchy and diagnostic separation. MSH-72 harness sizing/layout/Latin fallback/buttons/control strip are verification-only and not product approval. Future large boards must assess full-board fit, informed by owner-reported legacy master full16x16 display on smaller screens; zoom/pan/scroll are not assumed necessary. No MSH-74 implementation started. See docs/m4/desktop-session.md and docs/desktop/m3-design.md.

Owner request2026-10-26 recorded before substantial implementation: product UI plus implementation, not harness polish. Primary integrator Codex in clean primary checkout codex/rewrite at570ef0e2c931e6ecb07749d638294722c3e1198d, one worktree; dependencies73/70/68 Done. Backlog1.55.1. Explicit Chess/Shogi choice with native sides, human-bottom+Flip, warm uniform grid/pentagonal Japanese pieces with licensed bundled font, compact hands, complete choices/promotion, board-first side history/analysis, normal user language/optional diagnostics, full-board760x650+1180x820fit, no required zoom/pan for future12/16. M3 chess/73 saveability and stale-work contracts retained. Screenshot/review staging and owner visual checkpoint are required; this approval authorizes implementation but not final UI closure. Ordinary choices autonomous.

Implementation checkpoint: normal product source integrated/pushed in 41a3009, independently reviewed focus/retirement correction in 3803c1d, and visual contrast/numeric-editing correction in 60796ec18539f600354594f1fab6ee643b93bede. Native game_new/game_snapshot/game_command/game_file reuse strict shared admission, receipts, supervisor and v2 archive; chess normal behavior retained. New shogi renderer uses native complete choices and bundled licensed kanji font, with no TS legality or replay. Local full native workspace all-target/all-feature suite 395 passed; workspace clippy/fmt passed; frontend 118 passed; host independent review 7 passed; actual subprocess product journey passed. Independent implementation and visual reviews are recorded in docs/m4/evidence/msh74. First platform GUI runs honestly retained 11 passes/3 failures caused by numeric editing/driver string checking and missing dependent archive; fixes are in current source. Current native CI passes all three systems; current desktop package/GUI CI still running. Owner visual approval remains required, not supplied; do not mark Done or start75.

Final implementation audit found a concrete cached-game close-event race during native game replacement before polling/view remount. Fixed in b26fd736e0b2ae5ca3e3b80b7cc1f4f0730cf085: persistent ProductApp owns native close for both games, fresh shared snapshot supplies authority, embedded Chess suppresses only duplicate close confirmation while standalone default/Open/New guards remain. Independent fresh review traced native close/save locking and both transition directions; focused24tests, strictTS/diffcheck passed; root complete122frontend tests passed. Final normal Mac bundle built/package-smoke passed, actual CUA Chess Quit/Cancel→NewShogi→Quit/Cancel→Resume passed. Prior5de Desktop40408728525 and Rust40408728368 passed all3; Windows/Linux14/14 actualGUI phases each, package/hash/native shutdown independently validated. Final b26 source CI40411273334/40411273387 running. No owner visual approval yet.

Polished owner checkpoint reached at21bfe3e2a3be2d35bd8096b6207f6a8a6af75576 (integrated/pushed). One last independently reviewed modal fix retires the uncommitted New Game chooser on nativeQuit; independent11 ProductApp tests passed, full123 frontend tests/strictTS passed. Final normal Mac build/package-smoke passed; actual CUA Quit while chooser open→single Close guard→Cancel→originalChess retained→NewShogi succeeded. Normal app left at freshSente event0 for owner play. Gallery docs/m4/evidence/msh74/gallery.md, hashes/provenance screenshots.json, 8Mac +4Win/Linux normal captures, independentvisual review no outstanding material finding. Explicit owner visual approval requested and pending; AC11/DoD1 intentionally unchecked. AC5 awaits final source CI40412052098/40412052082; b26 Rust40411273387 all3 passed, b26 desktop40411273334 cancelled as superseded by21bfe3e. No75 started.

Actual final21bfe3e Windows GUI exposed REAL native projection race (13/14 passed): fresh-process canonical Shogi Open succeeded but Chess view remained25s. Root and independent platform reviewer identified same cause: shared_snapshot enteredChess, legacy snapshot polled files and committedShogi, then published Chess with NEW epoch; correct nextShogi equal token was properly rejected. Fixed both read paths in a9111ae83de1d91149367a136da4ed66efdefee5; legacy adapter also stops before hidden Chess engine work. Deterministic REAL archive/worker test forces completion before shared-first/legacy-first poll, failed oldcodeChess!=Shogi, nowpasses correctfamily/nativegeometry/side/stopped/token/nohiddenChesspending. Independent reviewer ran regression+adjacentarchive test PASS, no findings; strictfrontendfreshnessunchanged. Fullnative396/56groups +clippy/fmt PASS; normalhost4/all-featurehost7 PASS, frontend123 unchangedPASS. FinalnormalMacrebuilt/package-smokePASS and actualfreshChessOpen of EXACT Windowsfailedarchive→Shogi stopped2events/2observations→event1→Returnlive→Resume PASS; archive preserved, newemptySente leftopen. FailedWindowsreport+archive/validation retaineddurably. Prior21Rustall3PASS, desktop21cancelledafterWindowsfailure/asLinuxsuperseded (noLinuxGUIclaim). FinalsourceRust40414800449/Desktop40414800679 running; AC5 remainsunchecked, ownerAC11pending.

Final evidence complete at a9111ae83de1d91149367a136da4ed66efdefee5: Rust40414800449 and Desktop40414800679 passed Windows, macOS and Linux. Independent artifact audit confirmed Windows14/14 and Linux14/14 ACTUAL packaged GUI phases, every native host/engine absent before driver teardown, normal GUI executable hashes matching inspected packages, normal/QA manifests and all164Windows/187Linux retained file hashes. Former fresh-process Chess-to-Shogi reopen failure now passes on BOTH platforms. Final compact CI/GUI/package evidence retained under docs/m4/evidence/msh74 with earlier failures/source-specific screenshots preserved. All implementation and independent-review criteria supported; AC1–10 and DoD2–4 checked. AC11 (explicit owner visual approval) and DoD1 remain unchecked; task intentionally In Review. Owner visual question pending; elapsed time is not approval. No75 work started,41–47/M4 open, M2/M3 closed.

Owner approves overall visual quality and distinct game-appropriate character, but NOT final visual approval. Required refinements: one actual shared shell (global identity/runtime/document actions, fixed document title/dirty location, live heading+Flip, board/player left, sidebar Analysis→Game history→Game actions, collapsed technical details) while preserving game-specific board/theme. Educational opponent board/hand movement inspection must be native, read-only, token/history-bound, no state/engine/analysis mutations, complete-action routes/continuations retained. Hollow burgundy inspection rings+non-color distinction, pointer/keyboard/switch/Escape and explicit inspection names; no engine-threat claim. Document native counterfactual-side movement semantics, including transient en-passant scope. Final review checkpoint remains required; no75.

Owner refinement implementation ready: native immutable exact-selection query plus complete choices, actual shared ApplicationShell, Chess/Shogi inspection (including hands, history, compound route staging, keyboard/Escape and stale-response retirement). Independent implementation reviewer traced full boundary diff and independently passed core7/session5/host1/UI35 checks, no material finding. Root full workspace all-target/all-feature408 tests/56groups, frontend147/21files and strictTS, root fmt/clippy, host8 allfeatures passed. Normal Mac package built and package-smoke passed; actual friendly/enemy selection verified, discovered extra status-row overflow and corrected by reusing help row, shared-main padding specificity and Shogi contrast also corrected. Fresh independent static visual review passed structure/markers, requests final contrast/minimum captures. Relevant three-platform CI and final actual-package captures pending; task remains In Review, AC11/DoD1 unchecked, no75.

Owner refinement complete at integrated/pushed product d6d76816932763182eee574c9c241daa6ef9a91a. Shared ApplicationShell now fixes common header/document/actions, live heading/Flip and Analysis→History→Game actions/details locations across distinct Chess/Shogi themes. Native immutable exact-selection movement query (strict1KiB raw JSON) returns semantic complete choices from private hypothetical-owner selected positions; Chess EP entitlement is not transferred. Both boards/available Shogi hands support read-only hollow burgundy/dashed inspection, keyboard/Escape/switching/history and stale-query retirement, with no enemy commit or authoritative/execution mutation. Root408native/56groups,147frontend/21files,host8 allfeatures,strictTS/fmt/clippy and normalMacbuild/package-smoke passed. Independent source/test reviewer executed core7/session5/host1/UI35 and reviewed GUI assertions; separate screenshot/contrast reviewer inspected final nine actual normalMac images and Windows/Linux minimum/inspection images, no material findings. RustCI40472402479 and DesktopCI40472402431 PASS ALL3 exactd6; actualWindows14/14 andLinux14/14GUI phases include sharedshell/nativeinspection/canonicalSaveequality and existing full journeys. Fresh independent artifact auditor verified all190Windows/217Linux files, actualpackage/install/PID/hash bindings, native shutdown beforeteardown and byte-identical Shogi before/after/source semantics (Windows checkoutCRLF explained). Windows large native captures crop at1024x768desktop; paired1180x820client captures full,760x650native minimum complete. NormalMac1180x820GUI covers friendly/enemy board, opponenthand/keyboard, exacthistory/occupiedrings,Returnlive/Flip/SaveAs/quit; temporary teststate preserved after automatic review rejected discard. CurrentMacminimum unavailable; actualWin/Linuxminimum states and normal-size inspection captured separately, not combined. All original source-specific evidence preserved. Final report/screens/hashes/CI/audits/launch at docs/m4/evidence/msh74/refinement-2026-10-27. AC1–10/12/13 andDoD2–4 supported; AC11/DoD1 intentionally unchecked for explicit owner finalvisualapproval. KeepInReview; no75,41–47/M4 open,M2/M3 closed.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented and pushed the owner-requested MSH-74 refinement on codex/rewrite. Tested product revision d6d76816932763182eee574c9c241daa6ef9a91a. Chess and Shogi share the actual application shell and control locations while retaining distinct board/player treatments. Native token-bound immutable movement inspection supplies complete non-executable choices for opponent board pieces and available Shogi hands, including exact historical prefixes, both orientations and read-only route continuations. Friendly actions, native engine/context retirement, archive semantics and previous shogi play/analysis/history/recovery behavior are preserved.

Verification:408 native tests/56groups,147 frontend tests/21files,8 all-feature host tests,strict TypeScript,fmt/clippy and normal Mac package checks passed. Separate independent implementation/test, actual-screenshot/contrast and platform-artifact reviews found no material unresolved finding. Exact-source Rust40472402479 and Desktop40472402431 passed Windows/macOS/Linux; Windows/Linux each passed14/14 actual GUI phases, with canonical inspection invariance and native shutdown independently audited. Nine normal packaged Mac captures plus current Windows/Linux minimum/inspection captures, reports/hashes and launch instructions are in docs/m4/evidence/msh74/refinement-2026-10-27. Earlier evidence retains its original revision/provenance.

MSH-74 remains In Review for final owner visual approval:AC11/DoD1 unchecked; all other criteria/DoD supported. No MSH-75 work started. M2/M3 stay closed; MSH41–47/M4 remain open. Untimed bounded-search development packages; no formal screen-reader certification or strength/release-signing claim. Windows full client captures and complete minimum native window are distinguished from cropped large native desktop images; no new Mac minimum or combined minimum/active-inspection capture is claimed.
<!-- SECTION:FINAL_SUMMARY:END -->
