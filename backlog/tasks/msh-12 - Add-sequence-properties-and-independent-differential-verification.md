---
id: MSH-12
title: Add sequence properties and independent differential verification
status: In Progress
assignee:
  - '@claude'
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-24T19:37:00Z'
labels:
  - verification
  - properties
  - differential
milestone: m-1
dependencies:
  - MSH-2
  - MSH-9
  - MSH-10
documentation:
  - docs/design.md
  - AGENTS.md
priority: medium
ordinal: 12000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Extend focused tests with sequence-level evidence that detects state drift and move-generation disagreements.

## Design references

P-05; A-02; A-03; verification contract

## Scope

Generate bounded legal action sequences; test exact reversal and state serialization; compare recomputed derived state where present; add a bounded differential comparison with an independently implemented chess reference. Persist minimized or simplified failures as replayable regressions.

## Verification expectations

Record fixed seeds, limits, reference implementation/version, and reproducible commands. Separate a fast default suite from deeper optional or scheduled checks. Review licensing before adding an external test dependency.

## Out of scope

Deferring all tests to this ticket, treating agreement between two implementations as proof of full rules, and unbounded randomized CI runs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Fixed-seed legal sequences verify apply/undo restoration of every relevant state field.
- [ ] #2 Serialization and any incremental-versus-recomputed state checks run over meaningful generated positions.
- [ ] #3 A bounded independent differential comparison covers the documented position corpus and reports actionable disagreements.
- [ ] #4 Failures can be saved and replayed deterministically without requiring the original random run.
- [ ] #5 Fast and deeper verification commands have explicit purposes and limits, with no unsupported platform or completeness claim.
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
1. Claim MSH-12 on claude/next-task at 48a73f9 (equal to origin/codex/rewrite), worktree .claude/worktrees/next-task. Review the existing fixtures, the console shell (MSH-11) and the oracle tooling/licensing notes.
2. Fast Rust property suite (tests/sequences.rs) with no new dependencies: a small fixed-seed PRNG plays bounded random legal sequences from every corpus/fixture start position. Each ply checks:
   - FEN round-trip equality (incrementally reached state equals state recomputed from serialization; there are no derived caches to recompute);
   - perft(1) equals the legal count;
   - in_check equals the king-square attack query; classification is consistent;
   - apply and apply_ordinary agree on ordinary actions;
   - transition rules restated independently: side flip, halfmove reset/increment, fullmove after Black, raw EP target only after a double push, rights only ever lost.
   Then the whole sequence undoes in reverse with exact equality at every step. On failure it prints a minimal replay record (start FEN plus action prefix up to the failing ply).
3. Deep run: an #[ignore]d variant with more seeds and plies, run in release on request.
4. Replayable regressions: a fixture family mshogi.chess.sequence-regressions.v1 of (start FEN, actions, optional oracle expectations: after-FENs, final legal set, final classification). A Rust test replays every case with the same invariants plus the expectations, without the original random run.
5. Optional independent differential tool tools/differential_chess.py. It drives the real mshogi shell binary as a subprocess against pinned chess==1.11.2 from all corpus positions, with seeded oracle-chosen playouts, comparing perft 2 at the start and, at every ply, the legal set, status and after-FEN. It reports disagreements with FEN and diff and can --save them, or --fill expectations for Rust-recorded cases, as regression cases. Bounded defaults; documented seeds and limits. No GPL code is linked: it is a separate subprocess-level research tool, as before.
6. Seed the regression file with a few oracle-confirmed checkpoint sequences so replay is exercised. Document fast and deep commands, purposes, limits and platforms. Workspace checks, independent review, integration, native CI, close.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-24): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at 48a73f9, equal to origin/codex/rewrite; no other task In Progress. User explicitly selected MSH-12 (MSH-10 Done). Pinned backlog.md 1.55.1, Rust 1.89.0.

Implementation candidate on parent 48a73f9.

(1) New crates/mshogi-core/tests/sequences.rs, with no new dependencies. A SplitMix64 fixed-seed generator plays legal sequences, chosen from a text-sorted legal list so seeds are stable, from all 64 distinct corpus/special/classification/candidate start positions. Counter-limit starts are played with 0/1 counters, since CounterOverflow at the limit is MSH-9 behavior.
Every ply checks:
- FEN round-trip equality (the state reached by transitions equals the state recomputed from serialization; there are no derived caches yet);
- no duplicate legal actions; perft(1) equals the legal count;
- in_check equals the king-square attack query; classify equals the (in_check, has_legal) rule;
- every candidate is accepted or rejected only as CastlingKingAttacked, or IllegalPosition whose reason is king attack/adjacency, so no legal action is silently lost;
- independently restated transition effects: an exact board diff including the rook move and EP victim, side alternation, halfmove reset/increment, fullmove after Black, raw target only after a double push, and the exact castling-right survival rule;
- apply_ordinary agreement for ordinary actions and NotOrdinary refusals for specials.
Each sequence is then undone in reverse, with exact equality at every step and the start restored. A failure prints a minimal replay record (start FEN plus the prefix through the failing ply).
Fast default: 4,473 plies, pinned and deterministic. The #[ignore] deep run (seeds 1-8, 4 playouts x 120 plies) is 211,328 plies, pinned.

(2) Separate family tests/fixtures/chess/sequence-regressions.json (mshogi.chess.sequence-regressions.v1). Cases hold the start FEN, actions, optional oracle expectations (after_fens, final_legal, final_classification) and provenance. The Rust test replays them deterministically without the random run.

(3) Optional tools/differential_chess.py drives the real mshogi shell binary as a subprocess against pinned chess==1.11.2 (version and module SHA checked) from all 64 start positions. It compares perft N at each start, then the full legal set, classification and after-FEN at every ply of seeded oracle-chosen playouts, and reports disagreements with FEN and missing/extra actions. --save writes disagreements as regression cases with oracle expectations; --fill adds expectations to recorded cases. Licensing: GPL python-chess stays a separate subprocess-level research process, as previously reviewed; nothing is linked, vendored or shipped.

(4) The four committed regression cases come from running the differential tool against a deliberately injected fault (Black EP candidates dropped): it reported 13 disagreements, and the four shortest ending at a Black EP decision were saved. Proven: replay fails on the faulty build (final legal set mismatch) and passes on the correct one. --fill on a stripped case reproduced the committed expectations exactly.

Docs: development guide (fast/deep/differential commands, purposes, limits, platforms, save/replay workflow, licensing), fixture README section, coverage ledger state-properties row (bounded agreement is not proof; no derived caches yet).

Local verification PASS, macOS 27.0.1 arm64, rustc 1.89.0:
- cargo test -p mshogi-core --test sequences --locked (3 pass, 1 ignored; about 3 s debug): fast seeded sequences (4,473 plies); saved regression replay; a guard that the transition checker rejects a corrupted after-state.
- cargo test --release --locked -p mshogi-core --test sequences -- --include-ignored: deep run of 211,328 plies passes (about 7.5 s).
- Differential, oracle venv python -I tools/differential_chess.py --mshogi target/release/mshogi:
  - default seed 20261024: 64 starts with perft 2, 192 playouts up to 60 plies, 9,965 plies, 0 disagreements;
  - seed 7, 20 playouts x 150 plies: 163,344 plies, 0 disagreements;
  - seed 99, perft 3 at every start, 20 x 150: 164,441 plies, 0 disagreements.
- Fault injection: dropping Black EP candidates gives 13 disagreements, saved and replayed as above.
- Library mutation check against the fast suite (temporary, reverted): drop rook-capture right loss, fullmove on the wrong side, drop the EP victim, drop rook-move right loss. All 4 fail with precise reasons; the two rights mutations initially survived and were caught only after adding the exact rights rule and the candidate-rejection contract check.
- cargo fmt check; clippy -D warnings; cargo test --workspace --locked (all suites); whitespace diff check; cargo tree core-only.
Native CI pending integration. The differential and deep runs are local macOS evidence only and not CI.

Independent review: a fresh delegated reviewer (Claude subagent, no repository edits; library mutants only in a scratch copy) confirmed:
- the transition checks are independently restated and non-vacuous;
- the rights rule, board diff, EP/castling detection and candidate-rejection contract are correct;
- SplitMix64 plus sorted choice are deterministic and should be portable;
- --fill is exact; licensing and claims are honest.
Its mutants: dropped knight underpromotion and castling through an attacked transit were caught by the fast suite (pinned count) and by the differential with precise reports; no right loss on rook capture was caught by both with precise reasons. A queenside b-file blocker ignored in apply only escapes MSH-12 because both tools exercise listed actions; the existing legal_actions suite catches it.

Should-fix, resolved:
- S1: crash, empty-stderr or timeout failures were saved as 0-action cases with useless reasons, and short stdout or a timeout aborted the run. The tool now locates the failing ply from the first missing or differing console line (one line per successful command) and keeps the whole stderr, exit code or timeout as the reason. Verified with an injected panic at fullmove 15: 250 located cases, none empty, and replay fails on the faulty build.
- S2: perft-only disagreements were saved without a localizing prefix. They are now localized through divide down to the shallowest node whose legal set differs. Verified with an injected rook-capture rights fault: 13 perft-localized cases pass replay on the correct build and fail on the faulty build.

Nits, resolved:
- N1: --save deduplicates by (start, actions), and IDs skip existing ones.
- N2: the pinned ply-count assertions now explain that a trajectory change means differing legal sets and to run the differential.
- N3: removed the duplicate seeded run.
- N4: after_fens length is asserted.
- N5: comment on the reason-text match.
- N6: the attacks family was added to both start sets (92 distinct starts), and the ply-0 FEN echo is now compared.
- N7: timings corrected; the default binary path adds .exe on Windows.

Final local evidence (macOS arm64, rustc 1.89.0):
- fast suite 6,720 plies (pinned), about 2-4 s debug;
- deep --include-ignored 316,880 plies (pinned), about 10 s release;
- differential default seed 20261024: 92 starts with perft 2, 276 playouts, 15,008 plies, 0 disagreements;
- seed 7 with perft 3, 20 x 150: 245,564 plies, 0 disagreements;
- the committed regression cases still replay;
- fmt, clippy and cargo test --workspace --locked all pass; whitespace diff check clean.
Removed a stray tools/__pycache__ created by this session.
<!-- SECTION:NOTES:END -->
