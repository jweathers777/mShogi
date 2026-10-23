---
id: MSH-5
title: Implement reversible ordinary actions and promotions
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-23T15:00:00Z'
labels:
  - rules
  - state
  - undo
milestone: m-1
dependencies:
  - MSH-4
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
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
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
<!-- SECTION:NOTES:END -->
