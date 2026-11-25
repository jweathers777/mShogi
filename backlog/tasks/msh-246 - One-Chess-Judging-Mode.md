---
id: MSH-246
title: One Chess Judging Mode
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-25T12:21:00Z'
updated_date: '2026-11-25T16:29:00Z'
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

Handoff (2026-11-25, claude, stopped at usage limit). Branch: claude/msh-246-one-chess-judging (WIP commit dd7f27a).
Done: removed AdjudicationMode, the Full solver (chess/completeness.rs and tests/completeness.rs), CHESS_FULL_RECORD_SCHEMA_VERSION and NativeSchema::ChessFull (ChessM2 renamed ChessV1), feasibility_evidence, MatchRecord schema 2, replay_with_stop and mating_feasibility_with_stop; ExecutionPurpose::ChessFullReachability, ChessResult::Reachability, FullLimits/ReachabilityTarget/FULL_SENTINEL/MAX_FULL_*, and the now-dead FailureKind::UnsupportedMode, ExecutionOperation and execution_capabilities (only tests used them); desktop AnalysisOperation is now a plain {depth,nodes} struct (no kind tag), with_analysis_operation removed, FULL_ANALYSIS renamed ANALYSIS_BUDGETS; capabilities.chess_full and ChessPresentation.rules_mode removed; frontend Full panel, ReachabilityEvidence, reachabilityDescription, unsupported_mode notice and CSS removed; frontend fixtures regenerated with all four generators (Full fixtures deleted); gui-smoke 'concluded' and 'full-unknown' phases, the verification 'concluded' chess fixture and the package-smoke Full query removed; docs/rules (chess-completeness rewritten as coverage of the one behavior, chess-adjudication, chess-coverage, chess-match-api, chess-match-v1, chess-history-record-v1), docs/search.md, docs/desktop/*, desktop/README.md updated. No conflict with open tasks found.
Checks so far: cargo fmt --check passed; cargo clippy --workspace --all-targets -D warnings passed; npm run check passed; vitest 589 tests passed (after one test fix); python unittest test_gui*.py 166 passed. NOT yet run to completion: cargo test --workspace --locked (stopped part way), Chess perft tests.
Next: run cargo test --workspace --locked and the perft tests, fix failures; get the fresh-agent review (AC #4) and record it; merge main, rerun checks, merge into main as 'Merge MSH-246 (One Chess Judging Mode)', push; check ACs, final summary, Done; clean up (delete branch, cargo clean, remove desktop/node_modules and desktop/scripts/__pycache__).
<!-- SECTION:NOTES:END -->
