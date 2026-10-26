---
id: MSH-72
title: >-
  Extend shared native sessions sides and complete-choice presentation for chess
  and shogi
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T19:22:00Z'
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
- [ ] #1 Native session dispatch serializes chess/shogi commands,result acceptance and profile replacement with exact document/session identity; the webview receives native snapshots/legal choices and never computes legality.
- [ ] #2 Profile-qualified side IDs,first-mover labels,piece ownership/orientation and native board descriptors separate chess White/Black from shogi Sente/Gote. Game switching retires old results and selection even at matching coordinates.
- [ ] #3 Shared board/selection supports9x9 square geometry,board or hand origin,ordered routes and promotion/decline completion with click/keyboard/focus/cancel behavior; partial selection cannot commit.
- [ ] #4 Native analysis capability/score metadata preserves chess Full and exposes only supported bounded shogi operations; stale/unsupported results cannot repaint or become engine-turn actions.
- [ ] #5 Concrete native shogi-generated hand/choice fixtures and unchanged chess journeys verify the common boundary,while game-specific labels/glyphs/outcomes/application journeys remain owned by MSH41.
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
1. Preserve the chess DesktopController, v2 execution and archive compatibility; add an explicit typed chess/shogi session and presentation adapter on the same serialized controller and EngineBridge, with native profile-qualified sides, capabilities and complete choices. 2. Implement shogi native command/history inspection, semantic undo and bounded read-only analysis/engine-turn routing with fresh runtime identities across replacement and exact acceptance. Expose the boundary through Tauri without advertising a finished shogi GUI or archive. 3. Adapt the existing shared Board for native orientation, semantic pieces, hand origins and promotion completion; generate actual native fixtures and test click/keyboard/cancel/focus and cross-game retirement. 4. Verify native/controller, real subprocess, existing chess/frontend regressions and three-platform native/desktop CI; obtain separate independent boundary review, resolve findings, document API/limitations, integrate and push codex/rewrite.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Primary builder: Codex in /Users/johnweathers/Development/Personal/mShogi on codex/rewrite, baseline eb9cc7ef43500c9d32a5bc99ebbc9c6c2978ff60. Clean main checkout, no active Backlog owner or other worktree. Scope limited to MSH-72; MSH-73/74/75 and complete-delivery tasks remain open. Existing M2/M3, MSH-61 archive admission, native shogi profile and fixture expectations remain authoritative.

Implemented typed shared snapshots/commands on the existing serialized DesktopController and single EngineBridge; native shogi record replacement, legal complete moves/drops/promotion, declaration advice/formal terminal admission, exact event-prefix inspection, semantic undo, read-only analysis, cancellation/retry and cross-game identity retirement. Stable chess APIs/v2 observations/archive remain compatibility-owned. Added NativeBoard/shared Board hand/native-orientation adapter and native-generated fixtures, plus separately compiled shared-contract QA surface; normal UI stays chess, shogi saving remains unsupported. Independent read-only agent review traced Tauri raw JSON through native admission/supervision/presentation and independently ran native5, subprocess2, Board16 tests. Findings: monotonic close document preconditions and bounded UTF-8 parser diagnostics; both fixed with targeted regressions. Current tests expanded to11 native shared tests and83 frontend tests; final integrated regressions and current three-platform CI pending. Detailed contract: docs/m4/desktop-session.md.

Final independent review by msh72_contract_review: both material findings resolved; independent final11 native tests, real subprocess2, and Board/NativeBoard/routes16 passed. Additional owner/tone and Idle/9x9 threshold follow-ups reviewed and retested with no outstanding findings. Primary checks: workspace366, session all-feature57, frontend83, host5; fmt and clippy passed. Actual separate macOS development app verified native human7g7f/engine4c4d, read-only analysis, root/history/ReturnLive and native Cmd-Q unsaved-discard. Final QA host SHA2566b6404be4fdcea3ea4d2a5a11f0e281376cfb2fe24b21b264fa7a81fbd29ee0b; engine d36e48dc8d6175ab3c9fc073e4de1e119f6850fd7feef5e6bc4bacc3c8830826. Owner clarification recorded in M3/M4 design/evidence and MSH-74 added product acceptance criteria: this is a development-only semantic verification harness, not approved/completed shogi visual design or its product design basis. Semantic DTOs impose no Latin rendering, scrolling, button-row hands, raw diagnostics or identical game layouts. No further visual work in MSH-72; MSH-74 implementation unstarted. Current final-source native/desktop CI pending push.
<!-- SECTION:NOTES:END -->
