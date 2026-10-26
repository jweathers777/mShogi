---
id: MSH-72
title: >-
  Extend shared native sessions sides and complete-choice presentation for chess
  and shogi
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T19:37:00Z'
labels:
  - m4
  - owner-msh-46
milestone: m-4
dependencies:
  - MSH-69
  - MSH-70
references:
  - >-
    backlog/tasks/msh-46 -
    Deliver-shared-six-game-integration-across-console-engine-records-and-desktop.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 72000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-46. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Native command/context tests,native-generated frontend fixtures,keyboard/orientation/drop/promotion and cross-game cancellation/focus tests; actual consumer integration follows shogi desktop task.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Native session dispatch serializes chess/shogi commands,result acceptance and profile replacement with exact document/session identity; the webview receives native snapshots/legal choices and never computes legality.
- [x] #2 Profile-qualified side IDs,first-mover labels,piece ownership/orientation and native board descriptors separate chess White/Black from shogi Sente/Gote. Game switching retires old results and selection even at matching coordinates.
- [x] #3 Shared board/selection supports9x9 square geometry,board or hand origin,ordered routes and promotion/decline completion with click/keyboard/focus/cancel behavior; partial selection cannot commit.
- [x] #4 Native analysis capability/score metadata preserves chess Full and exposes only supported bounded shogi operations; stale/unsupported results cannot repaint or become engine-turn actions.
- [x] #5 Concrete native shogi-generated hand/choice fixtures and unchanged chess journeys verify the common boundary,while game-specific labels/glyphs/outcomes/application journeys remain owned by MSH41.
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
1. Preserve the chess DesktopController, v2 execution and archive compatibility; add an explicit typed chess/shogi session and presentation adapter on the same serialized controller and EngineBridge, with native profile-qualified sides, capabilities and complete choices. 2. Implement shogi native command/history inspection, semantic undo and bounded read-only analysis/engine-turn routing with fresh runtime identities across replacement and exact acceptance. Expose the boundary through Tauri without advertising a finished shogi GUI or archive. 3. Adapt the existing shared Board for native orientation, semantic pieces, hand origins and promotion completion; generate actual native fixtures and test click/keyboard/cancel/focus and cross-game retirement. 4. Verify native/controller, real subprocess, existing chess/frontend regressions and three-platform native/desktop CI; obtain separate independent boundary review, resolve findings, document API/limitations, integrate and push codex/rewrite.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary builder: Codex in /Users/johnweathers/Development/Personal/mShogi on codex/rewrite, baseline eb9cc7ef43500c9d32a5bc99ebbc9c6c2978ff60. Clean main checkout, no active Backlog owner or other worktree. Scope limited to MSH-72; MSH-73/74/75 and complete-delivery tasks remain open. Existing M2/M3, MSH-61 archive admission, native shogi profile and fixture expectations remain authoritative.

Implemented typed shared snapshots/commands on the existing serialized DesktopController and single EngineBridge; native shogi record replacement, legal complete moves/drops/promotion, declaration advice/formal terminal admission, exact event-prefix inspection, semantic undo, read-only analysis, cancellation/retry and cross-game identity retirement. Stable chess APIs/v2 observations/archive remain compatibility-owned. Added NativeBoard/shared Board hand/native-orientation adapter and native-generated fixtures, plus separately compiled shared-contract QA surface; normal UI stays chess, shogi saving remains unsupported. Independent read-only agent review traced Tauri raw JSON through native admission/supervision/presentation and independently ran native5, subprocess2, Board16 tests. Findings: monotonic close document preconditions and bounded UTF-8 parser diagnostics; both fixed with targeted regressions. Current tests expanded to11 native shared tests and83 frontend tests; final integrated regressions and current three-platform CI pending. Detailed contract: docs/m4/desktop-session.md.

Final independent review by msh72_contract_review: both material findings resolved; independent final11 native tests, real subprocess2, and Board/NativeBoard/routes16 passed. Additional owner/tone and Idle/9x9 threshold follow-ups reviewed and retested with no outstanding findings. Primary checks: workspace366, session all-feature57, frontend83, host5; fmt and clippy passed. Actual separate macOS development app verified native human7g7f/engine4c4d, read-only analysis, root/history/ReturnLive and native Cmd-Q unsaved-discard. Final QA host SHA2566b6404be4fdcea3ea4d2a5a11f0e281376cfb2fe24b21b264fa7a81fbd29ee0b; engine d36e48dc8d6175ab3c9fc073e4de1e119f6850fd7feef5e6bc4bacc3c8830826. Owner clarification recorded in M3/M4 design/evidence and MSH-74 added product acceptance criteria: this is a development-only semantic verification harness, not approved/completed shogi visual design or its product design basis. Semantic DTOs impose no Latin rendering, scrolling, button-row hands, raw diagnostics or identical game layouts. No further visual work in MSH-72; MSH-74 implementation unstarted. Current final-source native/desktop CI pending push.

Integrated source883651fc2ff2226b1429be29da1da111421b9b61 pushed to codex/rewrite. Normal macOS app build and package-smoke passed, including verification endpoint exclusion and real bundled UCI/play/analysis/Full queries. Current-source native run https://github.com/jweathers777/mShogi/actions/runs/40386309205 attempt1: Linux/Windows passed; macOS Test failed only unchanged MSH-71 missing_retirement_eof_cannot_hide_a_delayed_duplicate_bestmove (observed LateReply vs expected ResourceLimit). Independent reviewer diagnosed390ms duplicate vs375ms drain scheduling window; either native path rejects unscored candidate, retains incomplete traffic and reaps child. Exact regression independently passed locally1/1. No runner/test/product behavior changed; reran failed macOS job once on identical source, preserving attempt1 failure. Desktop https://github.com/jweathers777/mShogi/actions/runs/40386309478 in progress. Ticket remains In Review pending required CI.

Acceptance mapping at integrated source 883651fc2ff2226b1429be29da1da111421b9b61:
AC1: shared DesktopController commands/replacement and native admission on one existing EngineBridge; exact context tests and actual chess/shogi/chess subprocess tests.
AC2: explicit immutable native profile/edition and game-qualified sides, first-mover labels and native coordinate orientations; NativeBoard ownership/tone remains stable on flip; replacement retires work/selection.
AC3: actual native-generated 9x9 board/hand complete actions, optional/forced promotion; native11 and frontend83 checks include click/keyboard/Escape/focus, drop origins and existing same-origin/end synthetic routes. Partial selections never submit.
AC4: bounded shogi read-only analysis has exact provenance and separate acceptance; engine move/declaration admission stays native. Unsupported Full/claims reject; chess Full/v2/observations/archive semantics preserved by existing regression tests and package checks.
AC5: committed native generator and shogi fixtures, known rules expectations retained; existing chess fixtures unchanged. All-feature archive/capacity tests preserve MSH-61. MSH-74 owns actual visual design/product acceptance, as explicitly clarified by owner; no GUI-design approval inferred from the development-only harness.
Final native CI https://github.com/jweathers777/mShogi/actions/runs/40386309205 attempt2 passed Windows/macOS/Linux on the same source; original macOS attempt1 scheduling-sensitive MSH-71 failure remains recorded above. Final desktop CI https://github.com/jweathers777/mShogi/actions/runs/40386309478 passed all three platforms on this source, with actual Windows/Linux existing packaged chess GUI journeys, normal package/engine checks, QA build and instrumentation exclusion. macOS game-specific development harness evidence is separately scoped in docs/m4/desktop-session-validation.md; unchanged M2/M3 evidence reused, not relabeled as shogi product delivery.
Independent separate reviewer: read-only tracing plus final11 native /2 real subprocess /16 Board tests, both material findings resolved, no remaining findings. Primary verification: workspace366, all-feature session57, frontend83, host5; fmt/clippy/strict TS/builds/package-smoke passed. No product/rules/fixture expectation changes outside MSH-72. MSH-73 next eligible; MSH-74/75 and MSH-41–47 remain open; M2/M3 closed and M4 open.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed MSH-72 on codex/rewrite, tested/pushed source 883651fc2ff2226b1429be29da1da111421b9b61. Added explicit typed chess/shogi native session and semantic presentation dispatch on the existing controller/supervisor; native shogi moves/drops/promotion, declaration versus advice, event-prefix inspection/undo, read-only bounded analysis and exact execution retirement. Stable chess APIs, v2 execution, archive semantics and MSH-61 admission are preserved.
Verified with focused native11, actual subprocess2, existing workspace366, all-feature session57, frontend83 and host5, independent review, local macOS development application interaction and normal package/engine checks. Native CI attempt2 and desktop CI passed all three platforms; initial unchanged timing-sensitive MSH-71 macOS failure is retained in notes.
The MSH-72 DEVELOPMENT CONTRACT is solely a semantic verification harness, not completed/approved shogi UI design. Owner clarification and added MSH-74 product criteria are recorded in M3/M4 docs. Semantic API requires no Latin rendering, scrolling, button-row hands, raw diagnostics or identical game layouts. Normal app stays chess; shogi saving/full product UI remain MSH-73/74. Launch/API/evidence: docs/m4/desktop-session.md and desktop-session-validation.md. Stop at MSH-72; next eligible MSH-73. MSH-74/75 and complete-delivery MSH-41–47 remain open, M4 remains open.
<!-- SECTION:FINAL_SUMMARY:END -->
