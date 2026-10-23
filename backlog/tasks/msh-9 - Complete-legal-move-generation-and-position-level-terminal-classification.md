---
id: MSH-9
title: Complete legal move generation and position-level terminal classification
status: In Review
assignee:
  - '@claude'
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-23T18:25:00Z'
labels:
  - rules
  - legality
  - terminal
milestone: m-1
dependencies:
  - MSH-8
documentation:
  - docs/design.md
  - AGENTS.md
priority: high
ordinal: 9000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Expose a trustworthy legal action set and the explicitly scoped chess position classifications.

## Design references

A-01; A-04; M1 legal scope

## Scope

Filter or directly generate legal moves with correct king safety, including pins, single/double check, king moves, and special moves. Classify check, checkmate, and stalemate separately from full match adjudication.

## Verification expectations

Compare complete legal sets and terminal classifications with the independent corpus. Exercise invalid requested actions and distinguish an empty legal set in check from one outside check.

## Out of scope

Calling limited material checks complete dead-position adjudication, full draw claims/clocks, or engine strength.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The public legal-action API returns exactly the expected legal set on the selected independent corpus.
- [ ] #2 Pins, discovered attacks, single/double check, king safety, and special-action edge cases are covered.
- [ ] #3 Check, checkmate, and stalemate are unambiguous and match independent fixtures.
- [ ] #4 The coverage ledger clearly identifies remaining draw/history/match obligations instead of reporting complete chess.
- [ ] #5 Applying a public legal action and rejecting an illegal one obey the transition and restoration contracts.
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
1. Claim MSH-9 on claude/next-task at b64093d (equal to origin/codex/rewrite), worktree .claude/worktrees/next-task. Review the MSH-8 apply/candidate contract, the profile classification rule and the corpus.
2. Add public Position::legal_actions(): candidates() filtered by the checked apply on a temporary clone. Decision: legal = rule-legal under the profile, independent of the u32 counter domain (counters are normalized on the private clone), so classification at the counter limit stays correct. Apply may still reject a legal action atomically with CounterOverflow at the limit; this is documented.
3. Add Position::in_check() and Position::classify() -> Classification {Quiet, Check, Checkmate, Stalemate} per the profile (in_check, has_legal_action). Explicitly position-level, not match adjudication; there is no dead-position, repetition, 50/75-move, claim or clock logic.
4. Add a separate fixture family mshogi.chess.classification.v1 for gaps in the corpus: Black single and double check, smothered and protected-queen mates, a White back-rank mate, stalemate with a pinned piece, Black en passant check evasion, and bare kings at the counter limit. Hand classification and legal count first, then pinned chess==1.11.2 legal_moves/is_check/is_checkmate/is_stalemate; verify-only checker.
5. Rust tests: exact legal sets and classification for all 35 corpus positions and the new fixtures; legal sets for the special-actions and candidate fixtures; every legal action applies and undoes exactly; illegal requests are rejected atomically; empty legal set in check vs not in check; counter-limit behavior.
6. Coverage ledger: mark the legal list and classification as implemented and keep draw/history/match obligations visible. Workspace checks, independent review, integration, native CI, close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-23): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at b64093d, equal to origin/codex/rewrite; no other task In Progress. The user explicitly selected MSH-9 (MSH-8 is Done). Pinned backlog.md 1.55.1, Rust 1.89.0.

Implementation candidate on parent b64093d. New crates/mshogi-core/src/chess/legal.rs adds:
- public Position::legal_actions(): candidates() filtered by the checked apply on a private clone with normalized counters;
- Position::in_check();
- Position::classify() -> Classification {Quiet, Check, Checkmate, Stalemate}, implementing Display.
Docs state this is position-level only, with no dead-position, repetition, 50/75-move, claim, resignation or clock logic.

Recorded decision (flagged for John's review, not a rule change): legality is independent of the profile's u32 counter domain. Without this, a position at halfmove 4294967295 whose only legal actions are quiet would be misclassified as stalemate. Effect: at the limit, apply can reject a listed action atomically with CounterOverflow, which is the profile's existing atomic-failure contract; apply accepts every other listed action and nothing unlisted.

Fixtures: separate family tests/fixtures/chess/classification.json (mshogi.chess.classification.v1), 8 diagrams filling corpus gaps: Black single/double check, smothered mate, protected-queen mate, White back-rank mate, pinned-knight stalemate, Black en passant check evasion, counter-limit bare kings. Stored hand classification, legal count and required actions all matched pinned chess==1.11.2 on the first run. Verify-only checker tools/check_chess_classification.py.

Local verification PASS, macOS arm64, rustc/cargo 1.89.0:
- cargo test -p mshogi-core --test legal_actions --locked (6/6):
  - exact legal sets and classifications for all 35 corpus positions, with all four classes present;
  - the 8 classification fixtures;
  - the 21 special-action legal sets, and the candidate family's legal-ordinary sets plus exactly the described specials;
  - per position, every listed action applies and undoes exactly, and every other bounded action is rejected atomically;
  - empty legal set in check vs out of check while candidates remain;
  - counter-limit behavior;
  - determinism and no mutation (Kiwipete 48).
- oracle venv python -I tools/check_chess_classification.py: PASS 8.
- Mutation check (temporary, reverted): remove counter normalization, in_check wrong color, swap checkmate/stalemate, check as quiet, ordinary-only candidates, specials unchecked. All 6 fail the suite.
- cargo fmt check; clippy -D warnings; cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 6 legal + 6 ordinary + 5 candidates + 6 special + 5 doctests); rustdoc -D warnings; build; profile smoke; cargo tree core-only; whitespace diff check.
Docs updated: profile, coverage ledger (draw/history/match rows stay deferred), development guide, README, fixture README, console line.
Native CI pending integration.

Independent review: a fresh delegated reviewer (Claude subagent, no repository edits) found legal.rs correct and the counter normalization sound. Counters are read only by next_counters (overflow only) and by the post-state EP halfmove check, which a double push always satisfies. Fixtures are independent and the assertions strong; ledger, console and README do not overclaim.

Reviewer bounded differential probe in scratch outside the repo against pinned chess 1.11.2, seeds 9/2026/77: 33,569 positions (playouts from castling/EP-rich starts, random sparse placements, a terminal-biased set; variants at halfmove and/or fullmove 4294967295). Compared legal sets, classify/in_check against is_check/is_checkmate/is_stalemate, every after-FEN against push, exact undo, and CounterOverflow exactly where expected. About 579k action after-states; 0 mismatches.

Findings and resolution:
- should-fix: profile line 99 (MSH-7 text) said final filtering must establish counter bounds before a set is called legal, contradicting the new counter-domain decision. Amended to match the decision.
- should-fix: perft at the counter limit is unspecified. Recorded in the profile and as an MSH-10 note.
- nit: the checker now also calls is_checkmate/is_stalemate, as the provenance claims.
- nit: the evidence wording in the profile and development guide now excepts CounterOverflow at the limit.
- nit: the stale assert_contract comment in special_actions.rs is corrected.
- nit: legal.rs now states the normalization soundness assumption.
Reruns PASS: oracle checker 8; fmt; clippy; cargo test --workspace --locked (all suites, 5 doctests); whitespace diff check.

BLOCKER for Done: the reviewer and author agree that the counter-domain decision reinterprets an accepted contract statement (the meaning of legal versus counter bounds). Per AGENTS.md it needs John's explicit approval. Both recommend approval: the alternative misclassifies bare kings at halfmove 4294967295 as stalemate, and the decision matches the profile's atomic-overflow text, the corpus fen-maximum-counters rationale and the MSH-8 reject-castle-overflow fixture (oracle_legal true). Implementation and evidence are committed locally on claude/next-task and not yet integrated into codex/rewrite. Next action: on approval, push, record native CI and close. On rejection, the alternative (legal list excludes overflow; classification defined separately) needs a profile change proposal.

Owner decision (John, 2026-10-23, in session): approved the contract change separating chess move legality from counter representability, including the profile wording change at the former line 99. The approval was conditioned on verifying five points before closing; the task stayed In Review while they were completed. Mapping to evidence:

(1) Legality and classification are independent of counter overflow. New test legal_sets_and_classification_are_identical_across_counter_values covers every position of the corpus (35), classification (8), special-actions (21) and ordinary-candidates (17) families. Each is rerun with counter variants (0, max), plus (17, 1), (max, 1) and (max, max) when no raw EP target exists, since EP requires halfmove 0. That is 276 variants; legal sets, classify and in_check are identical to the base in every one.

(2) API docs: the ActionError::CounterOverflow rustdoc and the Position::apply rustdoc now state that an otherwise legal action can fail because its resulting state cannot be represented, and that this is a representation failure, not illegality. legal_actions rustdoc and the profile say the same.

(3) Atomic overflow. The same variant test independently predicts overflow per listed action: a non-reset (not a pawn move, not a capture) at halfmove max, or a Black move at fullmove max. It checks 1,659 CounterOverflow rejections, each leaving Position and FEN unchanged; every non-predicted action applies and undoes exactly. The new focused matrix counter_overflow_is_atomic_for_each_counter_side_and_action_kind covers:
- halfmove max, White: quiet and castling fail; a pawn double push and a rook capture reset and apply;
- halfmove max, Black: quiet and castling fail; a pawn move applies;
- fullmove max: White quiet and castling apply; Black pawn move, capture and castling fail;
- Black EP at fullmove max fails; White EP at fullmove max applies.
Existing MSH-5 counter_overflow_cannot_publish_any_piece_right_or_ep_effect still covers apply_ordinary. There is no derived state (no caches), so full Position equality plus FEN identity cover board, side, rights, raw target and counters.

(4) Simulation still handles all effects and king safety. The variant test exercises castling and EP positions at the limits, and the new overflow_never_turns_a_live_position_into_mate_or_stalemate covers Black in check at fullmove max (4 king replies listed, all overflow, classify Check, not Checkmate) and bare kings at halfmove max (Quiet, not Stalemate). Removing the normalization now fails 5 of 9 tests (previously 2).

(5) Docs: the Classification and classify rustdoc, the profile and the ledger now call quiet a result of the limited M1 position-level classifier, not complete game adjudication. Bare kings are explicitly quiet although FIDE 5.2.2 would end the game. Dead-position and other draw rules stay in the deferred ledger rows.

MSH-10: added AC #6 that perft/divide must never silently skip legal branches or treat counter overflow as a terminal leaf, and must traverse independently of counters irrelevant to move generation or report an explicit error. Implementation is left to MSH-10.

Reruns PASS (macOS arm64, rustc 1.89.0): cargo test -p mshogi-core --test legal_actions --locked (9/9); fmt; clippy -D warnings; cargo test --workspace --locked (5 CLI + 4 attacks + 8 FEN + 9 legal + 6 ordinary + 5 candidates + 6 special + 5 doctests); rustdoc -D warnings; whitespace diff check. Earlier reviewer evidence (33,569-position probe including counter-limit variants, 0 mismatches) still applies; the changes since are tests and documentation only.
<!-- SECTION:NOTES:END -->
