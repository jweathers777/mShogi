---
id: MSH-5
title: Implement reversible ordinary actions and promotions
status: Done
assignee:
  - '@codex'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-23T15:02:00Z'
labels:
  - rules
  - state
  - undo
milestone: m-1
dependencies:
  - MSH-4
references:
  - https://github.com/jweathers777/mShogi/actions/runs/39974586573
documentation:
  - docs/design.md
  - AGENTS.md
  - docs/rules/chess-position-v1.md
  - docs/rules/chess-coverage.md
  - tests/fixtures/chess/README.md
priority: medium
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Establish an explicit transition and undo contract before legal move generation depends on it.

## Design references

A-02; A-03; M1 state restoration

## Scope

Implement quiet moves, ordinary captures, promotion choices, side/counter updates, relevant rights updates, and complete undo records. Distinguish a checked external-action boundary from any trusted internal transition primitive.

## Verification expectations

Use hand-reviewed expected before/after states and exact restoration checks. Cover captured rook/king-related rights where applicable, promotions, and all mutated counters/derived fields.

## Out of scope

Castling and en passant execution, a universal variant event engine, and accepting arbitrary client-supplied capture lists.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Ordinary moves, captures, and all chess promotion choices produce independently expected resulting state.
- [x] #2 Undo restores every affected position field and any derived cache exactly.
- [x] #3 Transition preconditions and checked-versus-trusted entry points are explicit; the public boundary cannot partially apply a rejected action.
- [x] #4 The rules determine capture effects; an untrusted victim list is not authoritative.
- [x] #5 Focused regression tests accompany the transitions and expose state differences clearly.
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
1. Claim MSH-5 on clean codex/rewrite; review completed MSH-4, design/profile and real transition/state boundaries.
2. Add a chess-specific complete Action (bounded from/to plus typed promotion choice, no client victim list), checked apply_ordinary and a private resolved transition primitive. Validate side/occupancy/geometry/promotion/counters and resulting king safety on a temporary state before publishing.
3. Update all ordinary effects: board-derived capture, side, raw EP creation/expiry, clocks and historical king/rook/captured-home-rook rights. Reject castling/en passant execution explicitly until MSH-8. Reuse private validation geometry without exposing or completing MSH-6's public attack API.
4. Return an opaque full-state Undo snapshot; checked undo verifies matching post-state, restores all fields exactly, and rejects mismatched/out-of-order tokens atomically. Document local-state/LIFO semantics and absence of match-history/provenance claims.
5. Establish independent before/after FEN sequence/promotion fixtures with pinned python-chess and hand-reviewed rule rationale; test complete restoration, invalid actions, overflow and rights/EP boundaries. Preserve old corpus and legacy work.
6. Update API/coverage docs, run focused tests/checks, obtain independent review, resolve findings, commit/push normally, inspect actual native CI and record evidence. Complete MSH-5 only and report next Ready tasks.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-23 Europe/Paris): clean codex/rewrite at 5b45e77ec49ec05bcfe68feb244225de0d280d99, remote independently confirmed equal; no active task owner. Ready+ready lists MSH-5 then MSH-6; selecting MSH-5 by ordinal. MSH-4 dependency is Done. Owner @codex in existing isolated worktree /Users/johnweathers/.codex/worktrees/rewrite-bootstrap/mShogi. Original master/main_v2 WIP and legacy snapshot preserved; original checkout clean. Reviewed AGENTS, accepted design/profile, MSH-4 implementation, MSH-5 and MSH-13 boundary expectations. Backlog.md 1.55.1 and Rust 1.89.0 remain pinned; execution guide refreshed, overview already read in this conversation.

Scope decisions: public apply_ordinary checks the currently supported subset, including own-king safety through the existing structural validator. It never exposes an unchecked mutator or accepts caller-supplied victims. Castling and en passant action patterns are rejected as unsupported, not executed or declared legal. A private resolved primitive assumes checked geometry/occupancy/counters and runs only on a temporary clone. Existing private ray/validation helpers may be shared internally; MSH-6 still owns public attack queries and their independent examples.

Undo uses understandable full before/after Position snapshots, not compression or an unmeasured optimization. It is a private in-memory restoration record, not a game-save/history format. A borrowed record can restore only a matching complete post-state; mismatched/out-of-order/repeated immediate use fails without mutation. Callers retain LIFO/context responsibility; equality is saved-state equality, not proof of token origin or match history. No performance, full adjudication or six-game action-representation claim is added.

Implementation/evidence candidate on parent 5b45e77ec49ec05bcfe68feb244225de0d280d99 (the implementation commit will contain this record): added chess-specific Action/Promotion, checked apply_ordinary with board-derived captures and private resolved effects, candidate-only king-safety validation, exact borrowed snapshot Undo with complete post-state matching. Ordinary effects include both colors' promotion choices, raw EP creation/expiry, historical king/home-rook/captured-rook rights and checked u32 counters. Castling/EP execution stays explicitly unsupported. No derived caches exist. Runtime core remains dependency-free; original corpus, legacy snapshot, Cargo.lock, AGENTS and accepted design are unchanged.

AC evidence: ordinary_actions integration tests consume the new separate-format ordinary-transitions fixture family: 49 hand-reviewed independent sequences, 74 exact after-FENs, every reverse step compares full Position plus FEN. Coverage includes all quiet/capture promotions of both colors, promotions capturing home rooks, rights loss without restoration on return, check evasions and representable counter boundaries. Five additional focused test functions verify explicit preconditions, atomic invalid/unsafe/unsupported/overflow rejection, complete bad-undo mismatch and wrong-order protection. Client choices have no victim list; all captures are read from board occupancy. Public API doctest demonstrates apply/undo. Action/undo contract, coverage ledger, fixture provenance/recheck instructions, developer docs and honest console capability output updated.

Local verification PASS on macOS 27.0.1 arm64, rustc/cargo 1.89.0, candidate based on the parent above:
- cargo check --workspace --locked
- cargo test -p mshogi-core --test ordinary_actions --locked (6/6)
- /private/tmp/mshogi-msh2-oracle/bin/python tools/check_chess_transitions.py (pinned chess 1.11.2/module SHA checked; 49 sequences/74 exact expected after-states; no mShogi-generated expectations)
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --locked -- -D warnings
- cargo test --workspace --locked (8 FEN + 6 ordinary + 5 Unix CLI + 1 public API doctest, all pass)
- cargo build --workspace --locked; cargo run --locked --bin mshogi -- profile (reports partial ordinary-action/undo coverage)
- cargo tree -p mshogi-core --edges normal (only core itself); git diff --check.
Native Linux/Windows/CI verification is pending push; no other-platform success inferred.

Independent review: fresh delegated reviewer msh5_review inspected AGENTS/task, complete bounded source/tests/docs diff and fixture rationale; final result no actionable findings, no repository edits. Independently reran six focused Rust tests in a separate target and oracle checker. A temporary external differential probe at /private/tmp/mshogi-msh5-review passed 354,685 action cases across 349 positions against pinned chess 1.11.2: 300 deterministic reachable positions (seed 5315, resets after 70 plies/game over) plus 49 fixture starts, every occupied current-side source/destination, all legal promotions and extra randomized promotion choices. Compared acceptance/rejection, full raw-EP after-FEN, exact undo and atomic failure, excluding special execution and counter overflow (committed tests cover overflow). This is bounded review evidence, not a committed MSH-12 harness, full rules certification, performance result or other-OS execution. Reviewer confirmed explicit value-equality/context undo semantics are consistent.

Remaining integration: normal push on codex/rewrite, inspect native workflow for the exact implementation head, then append actual revision/run evidence and close. Follow-ups remain MSH-6 public attacks, MSH-7 generation, MSH-8 special execution, MSH-12 broad generated verification and MSH-13 compound-action boundary review.

Integration and native verification (2026-10-23): implementation fdf3c45c2722a244536d868e61354762b1b177e9 committed on codex/rewrite and normal push to origin succeeded (5b45e77..fdf3c45). Remote head independently confirmed equal; no force push, PR/default-branch change, settings change or unrelated integration. Original master checkout clean; master 941cd4dc37e6e6210d4f993c96a553753e228b19, local main_v2 WIP fd280dff02fa7ef73809e20b28bdec83fa9c955f and remote main_v2 90458437d8872d6264b5536ae6ab15142c11e357 remain unchanged.

Actual native CI PASS for exact head fdf3c45c2722a244536d868e61354762b1b177e9:
https://github.com/jweathers777/mShogi/actions/runs/39974586573
All jobs and steps succeeded: Rust (ubuntu-24.04) 120485745745, Rust (macos-15) 120485745576, Rust (windows-2022) 120485745227. Logs inspected using gh api run/jobs and gh run view --log. All identify rustc/cargo 1.89.0; hosts x86_64-unknown-linux-gnu, aarch64-apple-darwin and x86_64-pc-windows-msvc respectively. Each ran cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test --workspace --locked; cargo build --workspace --locked; help/version/profile console smoke. Each passed 8 FEN tests + 6 ordinary-action tests + 1 public API doctest; CLI passed 5 on Linux/macOS and 4 applicable on Windows (Unix byte-argument case correctly not compiled there). Console output reports ordinary actions/undo and unavailable special moves. This is actual native task verification, not complete chess or packaged-GUI certification.

All 5 acceptance criteria and 4 applicable completion items are now supported by committed tests, independent oracle/reviewer evidence, updated docs and actual native integration evidence. The completion commit containing this record changes only this ticket; implementation fdf3c45 is the tested code revision, and backlog-only pushes are intentionally ignored by the unchanged workflow. Next eligible work after this closure: MSH-6 public chess attack queries (ordinal 6000) and MSH-13 concrete action-boundary/compound fixture review (ordinal 13000). MSH-7 still waits for MSH-6. No subsequent task started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented checked chess-specific ordinary actions, captures and all four promotion choices, including king safety, board-derived effects, rights/raw-EP/counter updates and atomic failures. Opaque snapshot undo restores every saved field after matching the full post-state; callers retain LIFO/game-context responsibility. No caches, history proof or universal variant action type added.

Verification: 49 independent sequences/74 exact after-states plus six focused regression tests; independent reviewer found no actionable issues and passed a bounded 354,685-action differential probe. Workspace formatting/lint/test/build/smoke and native Linux/macOS/Windows CI all passed for implementation fdf3c45c2722a244536d868e61354762b1b177e9 (run 39974586573). Integrated and normally pushed to codex/rewrite; this ticket's completion commit preserves the same tested product code.

Castling/EP execution, public attacks, generated moves and full adjudication remain scoped follow-ups. Next dependency-ready tasks are MSH-6 and MSH-13; MSH-7 awaits MSH-6. Legacy refs and the original checkout remain intact.
<!-- SECTION:FINAL_SUMMARY:END -->
