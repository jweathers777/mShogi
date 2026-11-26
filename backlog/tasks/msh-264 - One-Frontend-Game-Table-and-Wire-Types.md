---
id: MSH-264
title: Frontend Consistency and Desktop Cleanup
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
updated_date: '2026-11-26T11:55:00Z'
labels:
  - desktop
  - frontend
  - refactor
  - cleanup
  - model:medium
  - effort:high
milestone: m-6
dependencies:
  - MSH-239
priority: medium
ordinal: 7700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
One Frontend Game Table and Wire Types:
In desktop/src, the list of game kinds is typed again in product/native.ts, NativeBoard.tsx, ApplicationShell.tsx, NativeSessionPanels.tsx and resources.ts; game names differ (ProductApp 'Chu Shogi', ApplicationShell 'CHU', SupportDialog has its own list); the thinking and analyzing checks are written four times, with two definitions of analyzing (ApplicationView.tsx, NativeSessionPanels.tsx, GameFrame.tsx); ExecutionResult in src/native.ts has no tenjiku case although the Rust type has one; TenjikuEffectPiece (boardPresentation.ts) repeats TenjikuPieceIdentity (product/native.ts); the snapshot wire type is in NativeBoard.tsx; side and piece labels are rebuilt in several components (colorName turns any non-white side into Black). Small helpers are copied (Tab focus loop three times, scroll-into-view twice). Frontend tests: eleven files build their own fake session, six define deferred(), about ten define board() or square(). MSH-239 (Uniform Desktop Operations and Board Components) covers the operation shape and the piece and board components and runs first; do not repeat it.

Remove Remaining Retired Desktop References:
MSH-248 (Remove Unused Console and Frontend Code) found leftovers outside its list (checked 2026-11-25; check them again before you change them). The verification surfaces that showed a 'DEVELOPMENT CONTRACT' label and a shared-contract-application class are gone from the source, but desktop/scripts/gui-smoke.py and desktop/src/product/ChuContract.test.tsx still assert that they are absent, and docs/m4/desktop-session.md and docs/m4/desktop-archive.md still describe the label. Two rules use custom properties that no style sheet defines: .document-dirty in desktop/src/style.css (var(--muted)) and .moves-events in desktop/src/product/shell.css (var(--shell-line)).

One Kept Result Count:
The desktop session keeps the newest 256 engine results with KEPT_RESULTS (native_session.rs). A second constant, MAX_ARCHIVE_OBSERVATIONS (persistence.rs), has the same value but no production use; only the verification capacity fixtures use it. Use one constant for both, and remove the unused one. Found during MSH-268 (Remove Game Size and Engine Request Limits).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One game table (kind, name, sides, labels) and one session-status helper serve the frontend; wire types are in one module and match the Rust types, including the Tenjiku result.
- [ ] #2 Copied helpers and test helpers each live in one shared module; frontend tests and the TypeScript check pass; line counts before and after are recorded.
- [ ] #3 No test or script asserts only that the retired contract class or label is absent; checks of kept behavior stay.
- [ ] #4 The desktop session and archive documents do not say that a current surface shows a 'DEVELOPMENT CONTRACT' label.
- [ ] #5 No style sheet uses a custom property that is not defined; the document state label stays readable.
- [ ] #6 One constant defines the number of kept engine results; MAX_ARCHIVE_OBSERVATIONS is removed and the verification fixtures use the remaining constant.
- [ ] #7 Workspace, desktop-session all-features and GUI script tests pass.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John approved consolidation on 2026-11-26: this task retains the full scope of MSH-264 (One Frontend Game Table and Wire Types), MSH-271 (Remove Remaining Retired Desktop References), MSH-272 (One Kept Result Count). The small retired-reference and result-count corrections share desktop verification with the frontend consolidation. Agent labels are model:medium, effort:high for the combined scope. Absorbed task records are archived, not completed; all implementation criteria remain unchecked.

Backlog consolidation, 2026-11-26: the dependency fields and current acceptance criteria supersede earlier scheduling prose and obsolete requirements in historical notes. Ready means approved for implementation, even when an unfinished dependency blocks work. The --ready filter excludes blocked tasks until their dependencies are satisfied; no manual status promotion is needed. No implementation has started.
<!-- SECTION:NOTES:END -->
