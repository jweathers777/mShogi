---
id: MSH-7
title: Generate ordinary candidate moves and promotion choices
status: In Progress
assignee:
  - '@claude'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-23T17:21:00Z'
labels:
  - rules
  - move-generation
milestone: m-1
dependencies:
  - MSH-5
  - MSH-6
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Generate the complete intended ordinary chess candidate set with an explicit contract before final king-safety filtering.

## Design references

A-01; A-02; M1 move categories

## Scope

Implement ordinary piece moves, pawn single/double advances and captures, and all promotion choices. Define whether generated candidates are pseudo-legal and what final filtering still must establish.

## Verification expectations

Compare candidate sets with hand-reviewed fixtures, including blocked movement, edge positions, both colors, and distinct underpromotions. Test absence of duplicates and prohibited own-piece/king captures.

## Out of scope

Castling, en passant, final legal-move claims, search ordering, and micro-optimization without evidence.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Candidate generation covers the intended ordinary movement and promotion categories for both colors.
- [ ] #2 Blocked and edge cases match independent expected sets and do not produce duplicates.
- [ ] #3 All promotion choices remain distinct actions throughout generation and transition.
- [ ] #4 The candidate-versus-legal contract is explicit and callers cannot mistake this intermediate API for complete legality.
- [ ] #5 Relevant transition and attack tests remain passing.
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
1. Claim MSH-7 on claude/next-task at 999f52f (equal to origin/codex/rewrite) in worktree .claude/worktrees/next-task; review MSH-5 action resolution, MSH-6 attacks, MSH-8/MSH-9 boundaries.
2. Add a public Position::ordinary_candidates() returning Vec<Action>: every pseudo-legal ordinary move/capture of the side to move (single/double pawn advances, pawn captures, knight/king offsets, slider rays) with each of the four promotion choices as a distinct Action. Exclude castling and en passant (MSH-8), own-piece and king captures. Do not filter own-king safety or counter bounds; document that final filtering must still establish them, and that this is not a legal-move list (MSH-9 owns the public legal API).
3. Explicit contract doc in profile/API: candidate = an action that apply_ordinary accepts or rejects only for resulting own-king safety (IllegalPosition) or CounterOverflow; and apply_ordinary accepts nothing that is not a candidate.
4. Separate fixture family mshogi.chess.ordinary-candidates.v1: hand-curated positions (blocked/edge/corner pieces, both colors, pawn starts and blocked doubles, quiet/capture promotions, pinned pieces and check where candidates exceed legal moves, castling/EP-shaped positions excluded) with exact candidate sets, established by pinned chess==1.11.2 pseudo-legal generation minus castling/EP; verify-only checker.
5. Rust tests: exact sets, no duplicates, no own/king captures, distinct promotions survive apply/undo, exhaustive 64x64x5 action enumeration agreement with apply_ordinary across fixtures and the original corpus, and corpus legal ordinary moves equal candidates filtered by apply_ordinary. Keep transition/attack suites passing.
6. Workspace checks, independent review, normal integration, native CI evidence, close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-23): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at 999f52f, equal to origin/codex/rewrite; no other task In Progress. User explicitly selected MSH-7 (its only dependency MSH-6 is Done). Pinned backlog.md 1.55.1 and Rust 1.89.0.

Implementation candidate on parent 999f52f: new crates/mshogi-core/src/chess/generate.rs adds public Position::ordinary_candidates() -> Vec<Action>, derived from its own offset/ray tables (independent of apply_ordinary geometry, then cross-checked). Pseudo-legal contract documented in rustdoc and the profile: ordinary moves/captures, pawn single/double advances and captures, four distinct promotion actions per last-rank destination; no castling, en passant, own or king captures, or duplicates; not filtered for own-king safety or counters. Exact contract: every candidate is accepted by apply_ordinary or rejected only as IllegalPosition/CounterOverflow, and apply_ordinary accepts nothing outside the set. Naming, rustdoc, profile section and console coverage line say plainly that this is not a legal-move list (MSH-9 owns that; MSH-8 owns special generation). King-capture exclusion is a documented defensive guard; it is unreachable in valid positions.

Fixtures: separate family tests/fixtures/chess/ordinary-candidates.json (mshogi.chess.ordinary-candidates.v1), 15 curated positions, 245 exact candidates, 229 exact legal-ordinary actions. Hand counts and required/excluded actions were written first; pinned chess==1.11.2 pseudo-legal/legal generation minus castling/EP had to agree. Two hand legal counts were miscounted (pawn-edge-captures 9->10, corner-bishop-and-queen 26->28); both were rechecked by hand on the board and corrected without consulting mShogi. Verify-only checker tools/check_chess_ordinary_candidates.py.

Local verification PASS, macOS arm64, rustc/cargo 1.89.0:
- cargo test -p mshogi-core --test ordinary_candidates --locked (5/5: exact fixture sets; exhaustive 64x64x5 bounded-action contract plus exact accepted set on 15 fixtures and 35 corpus positions (corpus legal moves minus castling/EP shapes); promotion choices distinct through apply/undo; determinism/no mutation; counter-bound candidate)
- oracle venv python -I tools/check_chess_ordinary_candidates.py: PASS 15/245/229
- Mutation check (temporary, reverted): double-advance start-rank, ray blocker stop, pawn capture occupancy, dropped knight promotion, wrong knight offset and missing queen orthogonals all fail the suite. Removing the king-capture guard survives, as expected for the unreachable guard.
- cargo fmt check; clippy -D warnings; cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 6 ordinary + 5 candidates + 3 doctests); build; profile smoke; cargo tree core-only; whitespace diff check.
Native CI pending integration.

Independent review: fresh delegated reviewer (Claude subagent, no repository edits) read AGENTS, MSH-7/8/9, design and the full bounded diff. No blocking or should-fix findings.
- Confirmed generate.rs correctness by hand.
- Confirmed the documented apply_ordinary contract is exact: castling-shaped and raw-EP diagonals are never candidates; from a valid position IllegalPosition can only mean the moving king is unsafe; CounterOverflow comes only from the halfmove clock or the fullmove number.
- Confirmed naming and docs do not overclaim legality.
- Bounded differential probe in scratch outside the repo, against pinned chess 1.11.2 pseudo-legal and legal moves minus castling/EP, with all 20,480 bounded actions checked against apply_ordinary per position: seed 7, 800 positions, 26,825 candidates; seed 11, 2,000 positions (1,027 Black to move, 541 with promotions, 1,318 with candidates above legal), 66,711 candidates and 49,086 legal. 0 mismatches.

Nits resolved:
(1) hand expectations are now stored per case as hand_expectations and checked by both the Rust suite and the oracle checker, so 'hand first' is auditable;
(2) added curated Black diagrams black-pinned-rook and black-in-check-edge-knight-bishop, whose hand counts matched first time.
Totals are now 17 positions / 274 candidates / 241 legal ordinary actions, and docs are updated. The interpreter-path nit is consistent with earlier README sections and is left as is.

Reviewer handoff for later tasks: MSH-8 must replace the shape-based unsupported rejections with rights/path/transit checks and EP discovered-attack handling; MSH-9 should replace the corpus test's castling/EP shape filter with exact full-legal-set comparison.

Final local reruns PASS: oracle checker 17/274/241; cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 6 ordinary + 5 candidates + 3 doctests); fmt and clippy clean; whitespace diff check clean.
<!-- SECTION:NOTES:END -->
