---
id: MSH-10
title: Implement reproducible perft and divide diagnostics
status: Done
assignee:
  - '@claude'
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-23T19:14:00Z'
labels:
  - diagnostics
  - perft
  - verification
milestone: m-1
dependencies:
  - MSH-9
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Make move-generation disagreements reproducible and localizable to a branch of the move tree.

## Design references

P-05; M1 diagnostics; verification contract

## Scope

Add perft and stable per-root-action divide output with explicit depth semantics, validated inputs, and useful failure reporting. Consume independently established fixture counts.

## Verification expectations

Run the selected reference corpus at bounded documented depths and compare expected totals/divisions where available. Confirm that diagnostics restore the starting state and handle zero/invalid depth and malformed positions.

## Out of scope

Search strength measurement, comparing arbitrary variants through a chess-only representation, and using newly generated mShogi counts as independent expected answers.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Perft totals match the independently established corpus at documented depths.
- [x] #2 Divide output identifies complete root actions in deterministic order and supports locating the first disagreement.
- [x] #3 Running diagnostics preserves the complete starting state.
- [x] #4 Depth/input semantics and failure output are documented and tested.
- [x] #5 A repeatable performance baseline is recorded with build and hardware context without turning it into a strength claim.
- [x] #6 Perft and divide never silently skip a legal branch or treat counter overflow as a terminal leaf: traversal either proceeds independently of counters irrelevant to move generation, or reports an explicit error when it cannot continue (owner requirement, 2026-10-23, from MSH-9).
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
1. Claim MSH-10 on claude/next-task at 6a0b1eb (equal to origin/codex/rewrite), worktree .claude/worktrees/next-task. Review the profile perft/divide semantics, MSH-9 legal_actions, and the owner's AC #6.
2. Add public Position::perft(depth) -> Result<u64, PerftError> and Position::divide(depth) -> Result<Vec<DivideEntry>, PerftError>.
   - Semantics: depth 0 is 1 even at mate/stalemate; positive depth with no legal action is 0; no draw/insufficient-material shortcut.
   - Divide requires depth >= 1 (an explicit ZeroDivideDepth error) and maps every legal root action to perft(depth - 1), summing to perft(depth).
   - Deterministic documented order: source a1..h8, then destination, then promotion none/Q/R/B/N.
3. AC #6: traverse a private copy whose counters are normalized at the root. With depth <= u32::MAX, halfmove <= depth and fullmove <= 1 + depth/2, so no CounterOverflow can occur. Any apply/undo failure is reported as an explicit Traversal error, never skipped or counted as a leaf. u64 accumulation is checked and reported as CountOverflow. Tests show counter-limit positions give the same counts and divides as normal ones.
4. Traversal uses apply/undo on one private copy and leaves the caller's position untouched (&self). Malformed positions cannot exist past the FEN boundary; document and test.
5. Separate fixture family mshogi.chess.divide.v1: per-root divide expectations from pinned chess==1.11.2 for the six reference positions at depth 3, special-move and terminal positions, and depth-4 totals and divides for selected references, run as #[ignore] deep checks. Verify-only checker. The corpus depth 0..3 perft totals are consumed unchanged.
6. Rust tests: corpus perft totals; exact divides with a first-disagreement locator (itself tested on a seeded mismatch); state preservation; zero-depth semantics and errors; counter-limit equivalence.
7. Record a release-build performance baseline (perft timings with host, toolchain and build) without any strength claim. Update docs and the coverage ledger. Workspace checks, independent review, integration, native CI, close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Input from MSH-9 review (2026-10-23): legal_actions() treats legality as independent of the u32 counter domain (pending owner approval in MSH-9). Perft from a position at the counter limit therefore cannot be computed by applying listed actions at depth 2 or more. MSH-10 must define this explicitly; normalizing counters on a private copy, as legal_actions does, is the consistent choice.

Startup (2026-10-23): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at 6a0b1eb, equal to origin/codex/rewrite; no other task In Progress. User explicitly selected MSH-10 (MSH-9 Done). Pinned backlog.md 1.55.1, Rust 1.89.0.

Implementation candidate on parent 6a0b1eb. New crates/mshogi-core/src/chess/perft.rs adds:
- public Position::perft(depth) -> Result<u64, PerftError>;
- Position::divide(depth) -> Result<Vec<DivideEntry>, PerftError>;
- DivideEntry {action, nodes};
- PerftError {ZeroDivideDepth, CountOverflow, Traversal(ActionError), Restore(UndoError)}, implementing Display/Error.

Semantics:
- depth 0 = 1 even at mate/stalemate; no-legal-action positions give 0 at positive depth; no draw/material shortcuts;
- divide needs depth >= 1 and sums to perft;
- deterministic documented order (source a1..h8 file fastest, destination, promotion none/q/r/b/n);
- traversal uses apply/undo on one private copy (&self is unchanged);
- depth-1 leaves are bulk-counted from legal_actions.

AC #6 (owner requirement): counters are normalized on the private copy at the root. Along at most u32::MAX plies halfmove <= depth and fullmove <= 1 + depth/2, so CounterOverflow is impossible. Any apply/undo failure stops with an explicit Traversal/Restore error and is never skipped or counted as a leaf. Accumulation is checked (CountOverflow).

Fixtures: separate family tests/fixtures/chess/divide.json (mshogi.chess.divide.v1), 14 complete ordered divides enumerated with pinned chess==1.11.2: the six references at depth 3, Black castling d3, EP-discovered d3, promotions d2, checkmate d1 (empty), a counter-limit castling position at d3, and three depth-4 divides (deep, #[ignore]). Nine nonzero totals also equal hand-recorded CPW published values (recorded as an unretrieved secondary cross-check). Verify-only checker tools/check_chess_divide.py. The Stockfish sf_17.1 tests/perft.sh was re-downloaded as data to scratch; its SHA-256 1f2a7b2d...c82a equals the corpus-registered source.

Local verification PASS, macOS 27.0.1 arm64 (Apple M1 Pro), rustc 1.89.0:
- cargo test -p mshogi-core --test perft --locked (5 pass, 1 ignored): the corpus's 38 perft expectations (11 positions, depths 0..3); 11 ordered divides with a first-disagreement locator; the locator itself on seeded count and missing-entry mismatches; depth/error semantics (perft(0)=1, divide(0) error, mate/stalemate 0 at depths 1..4, empty divide, order of depth-1 initial divide, malformed FEN rejected first); counter-limit positions equal to normal counters for every non-deep divide; caller state unchanged.
- cargo test -q --release --locked -p mshogi-core --test perft -- --ignored: 3 depth-4 divides pass (3.0 s).
- oracle venv python -I tools/check_chess_divide.py: PASS 14 divides, 5,004,333 leaves.
- Mutation check (temporary, reverted): depth-0 value, depth-1 bulk count, no counter normalization, no sorting, promotion order, wrong child depth all fail. Silent skip of a failed apply alone survives (unreachable while normalized); combined with removing normalization it fails counters_never_limit_traversal and the divides.
- cargo fmt check; clippy -D warnings (including the new example); cargo test --workspace --locked (all suites, 6 doctests); rustdoc -D warnings; build; profile smoke; cargo tree core-only; whitespace diff check.

AC #5 performance baseline (correctness-diagnostic speed, not a strength claim): cargo run --release --locked -p mshogi-core --example perft_baseline [upstream]. Apple M1 Pro 10 cores 32 GiB, macOS 27.0.1, rustc 1.89.0 / LLVM 20.1.7, default release profile, single thread:
- initial d4 197,281 in 0.064 s (3.07M nodes/s)
- kiwipete d4 4,085,603 in 1.284 s (3.18M)
- position 4 d4 422,333 in 0.132 s (3.19M)
- upstream Stockfish sf_17.1 totals reproduced exactly: kiwipete d5 193,690,690 in 59.5 s; position 5 d5 89,941,194 in 27.7 s; position 6 d5 164,075,551 in 50.0 s (about 3.25M/s).
Recorded in docs/development.md. Docs updated: profile perft section, coverage ledger (perft row; separate inspection-console row for MSH-11), README, development guide, fixture README, console line. Native CI pending integration.

Independent review: a fresh delegated reviewer (Claude subagent, no repository edits) found no blocking or should-fix issues. It confirmed:
- perft.rs correctness (depth handling, bulk counting, undo before error propagation, checked sums, order_key matching docs and checker);
- the soundness of the no-overflow-after-normalization argument;
- no path that silently drops a branch;
- no documentation overclaim (perft is library-only and console commands remain MSH-11);
- fixture independence.

Reviewer reruns PASS: oracle checker; debug perft suite (4.03 s); release --ignored depth-4 divides (2.99 s); baseline example (0.075 / 1.304 / 0.135 s); clippy; fmt.

Reviewer bounded differential probe in scratch outside the repo against pinned chess 1.11.2:
- 397 reachable positions (seed 20261023, playouts from 14 castling/EP/promotion-rich starts; 120 with an EP target, 252 with castling rights, 50 with root promotions, 4 terminal);
- each run at 4-5 counter variants including both counters at u32::MAX, for 1,865 variants;
- compared perft at depths 0-3, the full ordered depth-2 divide, the depth-3 divide sum, and unchanged caller state.
Result: 0 failures. The reviewer estimates 10-15 s for the default debug suite on hosted runners, which is acceptable.

Nits:
(1) the checker now validates the depth type and the deep flag type; rerun PASS 14 divides, 5,004,333 leaves.
(2) the first-disagreement locator is test-only while the library provides the deterministic order and the documented descent procedure; satisfies AC #2 as worded. A console locator naturally belongs to MSH-11, so no new task.

Integration and native verification (2026-10-23): implementation 8a080f8f74fee14700c780764cb32f3b41275a0a committed on claude/next-task and fast-forward pushed to origin codex/rewrite (6a0b1eb..8a080f8). No force push, PR, default-branch or settings change.

Actual native CI PASS for exact head 8a080f8: https://github.com/jweathers777/mShogi/actions/runs/40009700442. All three jobs succeeded on rustc 1.89.0:
- Rust (ubuntu-24.04), job 120606597347, x86_64-unknown-linux-gnu
- Rust (macos-15), job 120606597659, aarch64-apple-darwin
- Rust (windows-2022), job 120606597617, x86_64-pc-windows-msvc
Each job passed every suite (perft: 5 passed, 1 deep test ignored as designed) plus 6 doctests. CLI tests: 5 on Linux/macOS, 4 applicable on Windows. Console smoke reports perft-in-library coverage. The default debug perft suite took 6.27 s on Linux, 9.14 s on Windows and 5.36 s on macOS.

The deep depth-4 divides and the Stockfish depth-5 baseline are local release evidence only (macOS arm64, recorded above); they are not CI-verified on Linux/Windows.

Next eligible: MSH-11 (headless console, now unblocked), MSH-12 (sequence/differential verification, now unblocked) and MSH-13. No subsequent task started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added Position::perft(depth) and Position::divide(depth), with DivideEntry and PerftError. They are correctness diagnostics, not search or strength measurements.
- depth 0 = 1 even at mate/stalemate; positions with no legal action give 0 at positive depth; no draw/material shortcuts;
- divide needs depth >= 1, lists every complete root action in a documented deterministic order (source, destination, promotion) and sums to perft, so descending into the first differing entry localizes a disagreement;
- traversal uses apply/undo on a private copy and leaves the caller's position unchanged.
Owner AC #6: counters are normalized at the root, so traversal is independent of them and overflow is impossible within any u32 depth. Any traversal failure is an explicit error, never a skipped branch or leaf, and count overflow is checked.

Verification:
- the corpus's 38 perft expectations match;
- new fixture family mshogi.chess.divide.v1: 14 ordered divides from pinned python-chess 1.11.2, including a counter-limit case; three depth-4 divides are run with --ignored in release;
- a first-disagreement locator, tested on seeded mismatches;
- depth/error semantics and state preservation;
- mutation check, including a combined silent-skip mutation;
- an opt-in release baseline reproduces three Stockfish sf_17.1 depth-5 totals (up to 193,690,690 nodes) at about 3.2M nodes/s on an Apple M1 Pro, recorded as diagnostic speed only.
Independent review found no blocking or should-fix issues; its 1,865-variant differential probe had 0 failures and its checker nit was resolved. Native Linux/macOS/Windows CI run 40009700442 passed for 8a080f8 on codex/rewrite.

Limitations: console perft/divide commands and an interactive disagreement locator belong to MSH-11. Deep and upstream checks are local evidence only.
<!-- SECTION:FINAL_SUMMARY:END -->
