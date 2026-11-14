---
id: MSH-170
title: Remove unused code
status: Done
assignee:
  - '@claude-code-msh170'
created_date: '2026-11-14T11:26:00Z'
updated_date: '2026-11-14T15:14:00Z'
labels:
  - handle:unused-code-removal
milestone: m-5
dependencies: []
priority: medium
ordinal: 174000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Unused Code Removal.

**Problem:** The rewrite left code that no product path, test or tool uses. One known example is `crates/mshogi-search/src/chess_candidate.rs`, an older Chess search with no consumer after MSH-164 (Shared Search Core). The compiler does not report such code when it is public or is reached only by its own tests. Unused code costs reading time and can mislead later work.

**Objective:** The repository contains no unused Rust, frontend, tool or script code on the rewrite branch.

**Approach:** Owner request, 2026-11-14. Identify candidates in all workspaces (Rust crates and their examples and benches, the standalone Tauri host, the frontend, `tools/`, scripts): modules, public items, features, dependencies and files with no consumer other than their own tests. Record the list with the evidence for each item. Remove the items that are clearly unused. Bring to John each item that a design requirement, a planned variant or a recorded task still needs, or whose status is not clear; do not remove those. Historical evidence under `docs/` and the preserved original history are out of scope. Behavior must not change. Level 1.

**Completion:** A recorded candidate list with a disposition for each item, the clear items removed, and all workspace checks passing.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A list of unused code candidates across the Rust workspace, the Tauri host, the frontend and the tools is recorded with the evidence for each
- [x] #2 Each candidate is removed, or kept with a recorded reason or owner decision
- [x] #3 Workspace builds, tests and linters pass after the removal, and product behavior does not change
- [x] #4 Unused dependencies and features found in the same search are removed or recorded
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
1. Find candidates: scratch copy per library crate with pub items made crate-private where no other workspace file names them (compiler dead_code), two passes (all consumers / production-only consumers); grep review; examples, features, dependencies, Tauri host, frontend, tools.
2. Remove each clear item; confirm by cargo check --workspace --all-targets --all-features after each removal; repeat until no cascade.
3. Keep test seams and documented reference paths; list unclear items for John.
4. Update current docs that name removed items (not historical evidence).
5. Verify Level 1: cargo test/clippy workspace, Tauri host build, frontend checks; independent review; commit and push.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Interim (2026-11-14): removed chess_candidate.rs; chess search_access (SearchCursor etc., remains of chess_conventional) with its test and profile example and the search-only Game helpers it alone used; unused accessors AdjudicatedGame::dead_analysis, chu LocalLegal::is_defended, tenjiku StructuralState::{side_to_move,root,accepted_plies}, tenjiku Position::from_state, RequestContext::allow_draw_claims, SearchError::TimedFullUnsupported (never constructed). Per peer note: keep engine serve/serve_once and Tori search (MSH-169, MSH-165).

## MSH-170 candidate list (2026-11-14)

Method: scratch copy per library crate; each `pub` item not named by any other workspace file (other crates, own tests/examples/bins, Tauri host, frontend, tools) made crate-private; `cargo check --lib` dead_code warnings = no consumer except own unit tests. Second pass with only production consumers as roots = test/example-only items. Each removal confirmed by `cargo check --workspace --all-targets --all-features` with no warning. Frontend: import graph and export-name scan of desktop/src. Dependencies: per-crate name scan of every Cargo dependency. Examples, tools and scripts: consumer search in workflows, docs, package.json and scripts.

### Removed (no consumer)
- mshogi-search `chess_candidate` (older Chess search + `tapered-position.v1` evaluator): no consumer after MSH-164.
- mshogi-core `chess::search_access` (`SearchCursor`, `SearchKey`, `SearchNode`, `SearchPreview`, `SearchTurn`): remains of the removed `chess_conventional` search; only its own test `tests/chess_search_access.rs` and example `examples/chess_search_access_profile.rs` (both removed). Cascade: `AdjudicatedGame::{search_state, search_needs_claim_actions, search_outcome, apply_for_search, undo_for_search}`, `Game::reversible_prior_identities`.
- mshogi-core Chess queen contact-mate queries (`queens_in_zone`, `queen_in_attack_zone`, `queen_contact_mate`, `has_queen_contact_mate_threat`, `contact_mate_for`) and `Position::{legal_actions_to, captures_and_promotions}`: added by perf(chess) commits f1484b15/d9147713 for the removed search; only their own tests (2 tests in `tests/attacks.rs`, `tests/chess_target_queries.rs`, removed).
- Unused accessors: `AdjudicatedGame::dead_analysis`, chu `LocalLegal::is_defended`, tenjiku `StructuralState::{side_to_move, root, accepted_plies}`, tenjiku `Position::from_state`, `RequestContext::allow_draw_claims`.
- `SearchError::TimedFullUnsupported`: never constructed.
- Frontend `ArchivedObservation` interface (desktop/src/native.ts): no reference.
- Current docs updated: docs/evaluation.md, docs/development.md (test filter `chess_conventional` matched nothing).

### Kept: used (analysis false positive or consumer found by compile)
- engine `ChuTelemetry::score_label` (desktop + test), `RequestId::get` (console), `ChessEngine::table_megabytes`, `ChessPosition::{legal_moves, perft, side_to_move}` (bitboard perft tests), tori `Obligation::at_ply`.
- `chess_kpk::{probe, index}` with `#[allow(dead_code)]`: the module is also compiled by `examples/generate_chess_kpk.rs` (needs `index`, verifies with `probe`); module tests validate the committed table. The allow is justified.

### Kept: test seams (only tests use them; they carry rules/contract coverage)
- core Chess rules-step APIs with M1 conformance tests and Python tools: `apply_ordinary`, `set_fen`, `ordinary_candidates`, `special_candidates`, `AdjudicatedGame::from_history{,_with_mode}`.
- core native envelope `dispatch_command`/`NativeCommand*`/`NativeReply`/`DispatchMismatch` (5 envelope conformance tests + xiangqi handoff).
- core per-game query/inspection seams: chu `protected_squares`, `geometric_defenders`, `choices_from`, `select_prefix`, `capacity_usage`, `from_code`, `profile_id`, `as_json`; tenjiku `choices_from{,_metered}`, `geometry_actions_from`, `select_prefix`, `as_json`; tori `select_prefix`, `capacity_usage`, `profile_id`; shogi `is_legal`, shogi/xiangqi `pseudo_legal_actions`.
- conformance `properties::{preserves_material, rotate_*, round_trip}` (harness tests).
- engine `ExecutionOperation`/`execution_capabilities` (routing/protocol tests in engine and console).
- search `xiangqi::search_requiring_root_exact` (MSH-97 documented bounded root-exact entry; tests + `xiangqi_baseline` example).
- desktop-session `verify_*` (feature `desktop-verification`), `verification_deadline_failure`, `queue_shared_open`, chu/tenjiku/tori document `new`/`usage`/`initial_record`/`retired_branches`, `TenjikuDocumentError::stopped`.
- `serve`/`serve_once` (MSH-169) and the Tori search (MSH-165 will replace it): not touched.

### Kept: tools, examples, features, dependencies
- All 23 examples: documented reproduction or fixture-generation tools (docs, CI workflow or Cargo `[[example]]`).
- All Cargo dependencies and the `desktop-verification` feature have consumers. Tauri host `src/main.rs` has no own warnings; `vendor/tao` warnings are third-party and out of scope.
- tools/ and desktop/scripts: each has a workflow, doc or test consumer; `tools/strength/make_openings.py` documents the provenance of the committed opening set.
- Frontend `__fixtures__` with no frontend import (e.g. `shared-chess-capacity`, `shared-tori-stopped`): generated by the fixture examples; CI regenerates and diffs the whole directory, so they are native snapshot drift guards.
- Exported frontend names used only inside their own file (e.g. `pieceGlyph`, `publicSessionError`): used code; the extra `export` is harmless.

### Questions for John (kept until decided)
1. Full-width Chess search `mshogi_search::search::{search, search_with_stop}` (MSH-20) and the material evaluator `mshogi_search::chess` (MSH-19): no product consumer; only tests, `search_baseline` and `chess_study_roots` examples. docs/search.md calls them reference paths. Remove, or keep as reference?
2. `ToriDocumentWorker`/`ToriFileResult` with `NativeWork::Tori`, `CandidateWork::Validate`, `NativeCompleted::ToriValidated`: product uses the `ToriSave`/`ToriRuntime` path; only tests use the worker. docs/m4/tori/desktop-document.md describes the worker as part of the design. Remove (and convert its save tests to `execute`), or keep?
3. Frontend `ToriBirdPiece` (desktop/src/product/ToriBirdPiece.tsx, tori-birds.css): marked "dormant original artwork alternative"; only its test uses it. Remove, or keep for a later option?

### Observation (not changed)
- The MSH-164 Chess engine has no timed Full-mode handling; the removed `TimedFullUnsupported` variant was no longer constructed. docs/evaluation.md still describes the `EvalState` push/pop of the removed evaluator; that text belongs to MSH-164 follow-up, not to this removal.

Verification (Level 1: platform-independent removal of unused code; no portability, process, persistence or protocol risk; no broader platform checkpoint required). Environment: macOS Darwin 27.0.0 arm64, Rust 1.99.1, Node per desktop/package.json. Source: commit cbe685f0 (parent 97c7e280, includes merged MSH-169).
- cargo fmt --all -- --check: pass.
- cargo clippy --locked --workspace --all-targets --all-features -- -D warnings: pass (after MSH-169 merge).
- cargo test --locked --workspace: pass, 1144 passed, 0 failed, 11 ignored (run started 15:58 on merged source plus this change). An earlier run had 1141 passed and one desktop-session doctest build failure (E0463 cannot find mshogi_engine although --extern was given) while other cargo processes shared the target directory; the clean rerun passed.
- cargo test -p mshogi-core --test chess_capture_queries: 2 passed.
- desktop/src-tauri cargo build --locked, and with --features desktop-verification: pass.
- desktop npm run check, npm run build: pass; npm test: 42 files, 570 tests passed (frontend unchanged after that run).
Independent review (fresh general-purpose reviewer, read-only, diff of this change): no blocking findings; cargo check all targets clean. Low 1: tests/chess_target_queries.rs also held the only direct tests of the retained Position::legal_captures_to -> resolved, the independent-expectation test and an every-square check against legal_actions are kept in tests/chess_capture_queries.rs. Low 2: stale doc comment naming legal_actions_to -> resolved. Note: legal_actions_matching now has one caller -> left unchanged (optional).
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Removed code with no consumer on codex/rewrite (commit cbe685f0, about 3,200 lines): the chess_candidate search; the Chess SearchCursor (search_access) of the removed conventional search with its own test and profile example and the Game helpers only it used; queen contact-mate queries and Position::legal_actions_to / captures_and_promotions, added for that search and used only by their own tests; unused accessors in Chess, Chu and Tenjiku; RequestContext::allow_draw_claims; the never-constructed SearchError::TimedFullUnsupported; the unreferenced frontend ArchivedObservation type. docs/evaluation.md and docs/development.md no longer name removed code. Behavior is unchanged: removed items had no production caller, the compiler confirmed each removal over all targets and features, and the full workspace, Tauri host and frontend checks pass (Level 1, macOS arm64). Independent review had two low findings, both resolved. The candidate list in the notes gives the disposition and evidence for every item, including kept test seams, examples, tools, dependencies and generated fixtures.

Open questions for John (items kept until decided): (1) remove the full-width Chess search and material evaluator (MSH-20/MSH-19; only tests and two examples use them; docs/search.md calls them reference paths)? (2) remove the test-only ToriDocumentWorker path that docs/m4/tori/desktop-document.md describes? (3) remove the dormant ToriBirdPiece artwork? Observation: the MSH-164 Chess engine has no timed Full-mode handling, and docs/evaluation.md still names the removed EvalState.
<!-- SECTION:FINAL_SUMMARY:END -->
