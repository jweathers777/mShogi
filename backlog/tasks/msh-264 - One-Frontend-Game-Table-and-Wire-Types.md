---
id: MSH-264
title: One Frontend Game Table and Wire Types
status: Ready
assignee: []
created_date: '2026-11-25T12:23:00Z'
labels:
  - desktop
  - frontend
  - refactor
  - model:medium
  - effort:medium
milestone: m-6
dependencies: []
priority: medium
ordinal: 7700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In desktop/src, the list of game kinds is typed again in product/native.ts, NativeBoard.tsx, ApplicationShell.tsx, NativeSessionPanels.tsx and resources.ts; game names differ (ProductApp 'Chu Shogi', ApplicationShell 'CHU', SupportDialog has its own list); the thinking and analyzing checks are written four times, with two definitions of analyzing (ApplicationView.tsx, NativeSessionPanels.tsx, GameFrame.tsx); ExecutionResult in src/native.ts has no tenjiku case although the Rust type has one; TenjikuEffectPiece (boardPresentation.ts) repeats TenjikuPieceIdentity (product/native.ts); the snapshot wire type is in NativeBoard.tsx; side and piece labels are rebuilt in several components (colorName turns any non-white side into Black). Small helpers are copied (Tab focus loop three times, scroll-into-view twice). Frontend tests: eleven files build their own fake session, six define deferred(), about ten define board() or square(). MSH-239 (Uniform Desktop Operations and Board Components) covers the operation shape and the piece and board components and runs first; do not repeat it.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 One game table (kind, name, sides, labels) and one session-status helper serve the frontend; wire types are in one module and match the Rust types, including the Tenjiku result.
- [ ] #2 Copied helpers and test helpers each live in one shared module; frontend tests and the TypeScript check pass; line counts before and after are recorded.
<!-- AC:END -->
