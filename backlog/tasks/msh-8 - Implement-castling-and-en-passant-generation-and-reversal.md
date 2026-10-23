---
id: MSH-8
title: Implement castling and en passant generation and reversal
status: In Progress
assignee:
  - '@claude'
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-23T17:36:00Z'
labels:
  - rules
  - special-moves
  - undo
milestone: m-1
dependencies:
  - MSH-7
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 8000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Implement the chess special moves whose legality and effects cannot be reduced to an ordinary single-piece capture.

## Design references

A-02; A-03; M1 special move categories

## Scope

Implement castling movement/rights/attack constraints and en passant timing, victim-square effects, relevant occupancy changes, and complete undo. Integrate their candidate generation with the preceding work.

## Verification expectations

Use independent cases for both castling sides, lost rights, attacked transit positions, blocked paths, en passant expiry, and en passant discovered attacks. Compare exact post-state and restored pre-state.

## Out of scope

Chess960, universal multi-leg variant support, and weakening the adopted chess validation policy.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Castling handles the relevant pieces, rights, occupancy, and king-safety conditions under the adopted orthodox profile.
- [ ] #2 En passant removes the correct off-destination victim and handles its availability and occupancy effects.
- [ ] #3 Illegal special actions are rejected at the checked boundary without changing caller state.
- [ ] #4 Every special-action mutation and derived field is restored exactly by undo.
- [ ] #5 Independent legality and restoration cases are recorded as deterministic regressions.
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
1. Claim MSH-8 on claude/next-task at 54060d3 (equal to origin/codex/rewrite), worktree .claude/worktrees/next-task. Review the MSH-5 resolution code, the MSH-6 attacks, the MSH-7 candidate contract and the MSH-9 boundary.
2. Add a checked general boundary Position::apply(action) that executes ordinary actions and the two special actions; keep apply_ordinary as the ordinary-only subset. Rename the deferred Unsupported* errors to CastlingNotOrdinary/EnPassantNotOrdinary, and add CastlingUnavailable (right missing or path occupied) and CastlingKingAttacked (king origin or transit attacked).
3. Castling: king e1/e8 to g/c on the home rank with the right, home king and rook, and empty squares between. The king must not be attacked on its origin or transit square; its destination safety is checked on the resulting board. The rook moves to f/d, both rights of the color are lost, the halfmove clock increments and the fullmove number advances for Black.
4. En passant: a pawn moves diagonally forward onto the raw target, removes the off-destination victim, resets the halfmove clock and clears the target. Own-king safety, including discovered rank/diagonal attacks, is validated on the post-effect board.
5. All failures are atomic. Undo reuses the full snapshot and restores every field.
6. Add Position::special_candidates() (pseudo-legal: castling with right and empty path; EP with an adjacent own pawn) and Position::candidates() (ordinary plus special). Contract: apply accepts only candidates, and rejects a candidate only for IllegalPosition, CastlingKingAttacked or CounterOverflow. Document that it is still not the public legal list (MSH-9).
7. Add a separate fixture family mshogi.chess.special-actions.v1. Positions get hand special-candidate and legal-special sets. Sequences get exact after-FENs. Rejections get expected error. Hand-curated first, then confirmed by pinned chess==1.11.2, with a verify-only checker.
8. Rust tests: exact sets and after-states; reverse undo; atomic rejection; the exhaustive bounded-action contract for apply; and the corpus full legal sets equal candidates accepted by apply on all 35 positions. Update the MSH-5/MSH-7 tests and docs for the renamed errors.
9. Workspace checks, independent review, normal integration, native CI evidence, close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-23): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at 54060d3, equal to origin/codex/rewrite; no other task In Progress. User explicitly selected MSH-8 (MSH-7 Done). Pinned backlog.md 1.55.1, Rust 1.89.0.

Implementation candidate on parent 54060d3.
- action.rs: resolution is generalized to a Resolved effect record carrying the capture square/piece (an off-destination en passant victim) and an optional castling rook move.
- New public Position::apply is the checked boundary for every chess action. apply_ordinary stays the ordinary-only subset.
- Error changes: UnsupportedCastling/UnsupportedEnPassant are renamed to CastlingNotOrdinary/EnPassantNotOrdinary (old names claimed execution was deferred). New CastlingUnavailable (right, home rook or empty path missing) and CastlingKingAttacked (check or attacked transit).
- Castling destination safety and every en passant discovery or pin are judged on the post-effect board by the shared structural king-safety check (IllegalPosition).
- Castling loses both rights, clears the target and increments the halfmove clock. En passant removes the victim, resets the halfmove clock and clears the target. Undo reuses the full snapshot.
- generate.rs: new special_candidates() (castling with right + home rook + empty path; en passant per own pawn behind the raw target, with no king-safety filtering) and candidates() (ordinary then special).
- Contract: apply accepts only candidates and rejects a candidate only as CastlingKingAttacked, IllegalPosition or CounterOverflow. Still not the public legal list (MSH-9).

Fixtures: separate family tests/fixtures/chess/special-actions.json (mshogi.chess.special-actions.v1): 21 positions (hand special candidates, hand legal specials, full oracle legal set), 6 sequences with 25 exact after-FENs, 18 rejections with hand error categories. All hand expectations matched pinned chess==1.11.2 on the first run. Verify-only checker tools/check_chess_special_actions.py.

Local verification PASS, macOS arm64, rustc/cargo 1.89.0:
- cargo test -p mshogi-core --test special_actions --locked (6/6): position sets plus the exhaustive 64x64x5 apply contract; sequences with reverse undo and repeated-undo rejection; rejections atomic with exact error; corpus full legal_moves equal candidates accepted by apply on all 35 positions; ordinary-only refusal; exact castling after-FENs for all four castlings (independently confirmed with the oracle).
- oracle venv python -I tools/check_chess_special_actions.py: PASS 21 / 6 sequences / 25 steps / 18 rejections.
- Mutation check (temporary, reverted): drop origin check, drop transit check, queenside b-file path, ignore right, keep EP victim, skip rook move, reset halfmove on castling, queenside candidate path, one-sided EP candidate. All 9 fail the suite.
- cargo fmt check; clippy -D warnings; cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 6 ordinary + 5 candidates + 6 special + 4 doctests); rustdoc -D warnings; build; profile smoke; cargo tree core-only; whitespace diff check.
Native CI pending integration.

Independent review: a fresh delegated reviewer (Claude subagent, no repository edits) read AGENTS, MSH-8/MSH-9, the design, the profile and the full bounded diff. No blocking correctness defects; all five AC are supported.
- Confirmed castling check order, the transit square (= rook destination), post-board destination safety, rights/target/counter effects, en passant victim location, availability, discoveries and counters.
- Ordinary-path behavior is unchanged apart from the renames.
- The candidate contract is exactly true; error precedence matches the docs.

Reviewer bounded differential probe in scratch outside the repo, against pinned chess 1.11.2:
- 5,496 positions: seeds 8 and 9 random playouts from 12 castling/EP-rich starts including Kiwipete and perft 3/4/5, plus seed 10 with 1,500 synthetic EP/castling setups with random attackers;
- 141,435 legal moves, about 1,830 legal castlings, about 1,030 legal en passant captures, 961 candidates rejected by apply (670 CastlingKingAttacked, 291 IllegalPosition);
- checks: all 64x64x5 bounded actions per position; candidates filtered by apply equal the oracle legal set; every after-FEN equals oracle push; exact undo and failed repeat undo; atomic rejections; apply_ordinary and apply agree except the NotOrdinary refusals.
Result: 0 failures.

Should-fix findings (documentation only), all resolved:
(1) the root README still said special-move execution remained;
(2) the coverage-ledger ordinary row said 'No special execution';
(3) the profile named the removed private resolve_ordinary.
Nits resolved: candidates() rustdoc now says profile-legal because of the u32 counter domain; the mod.rs wording now says checked transitions.
Final reruns PASS: fmt, clippy, cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 6 ordinary + 5 candidates + 6 special + 4 doctests), rustdoc -D warnings, whitespace diff check. No re-review round for these documentation-only corrections (proportionate review per AGENTS).
<!-- SECTION:NOTES:END -->
