---
id: MSH-246
title: One Chess Judging Mode
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T16:06:00Z'
labels:
  - rules
  - chess
  - cleanup
  - model:large
  - effort:high
milestone: m-6
dependencies: []
priority: medium
ordinal: 3300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
John decided on 2026-11-25 (MSH-209 decision 1; a rules change that John approved): Chess has one way of judging games and one record schema. Remove AdjudicationMode and the name M2, and keep only the behavior that M2 has today (material-subset dead positions). Remove the Full mode, record schema 2 (CHESS_FULL_RECORD_SCHEMA_VERSION, NativeSchema::ChessFull), the desktop panel 'Advanced: Full mating reachability' (desktop/src/product/ChessAnalysisContent.tsx), ExecutionPurpose::ChessFullReachability (mshogi-engine shared.rs, server.rs and the adapters; desktop-session chess_observation.rs, shared_chess.rs, native_session.rs, shared.rs) and the Full solver (mshogi-core chess/completeness.rs and its uses in chess/adjudication.rs and chess/matchplay.rs). Checked 2026-11-25: desktop chess_record.rs, engine server.rs, console session.rs and runner experiments/native.rs already reject records that are not M2; docs/rules/chess-completeness.md states no practical runtime bound for the solver (only optional position and transition budgets).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 mshogi-core has one Chess judging behavior (the M2 behavior of today) and one record schema; AdjudicationMode, the Full solver, schema 2 and the name M2 are gone from code, tests and fixtures.
- [ ] #2 ExecutionPurpose::ChessFullReachability, the engine and desktop Full analysis paths, the frontend panel, its fixtures and its GUI-script steps are removed; desktop, engine and frontend tests pass.
- [ ] #3 docs/rules (chess-adjudication, chess-completeness, chess-coverage, chess-match-api) describe the one behavior; rules tests whose expected results come from the rules pass and perft is unchanged.
- [ ] #4 One review by a fresh agent session.
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
John, 2026-11-25: deletion tasks run before MSH-222, so later consolidation has less code.
<!-- SECTION:NOTES:END -->
