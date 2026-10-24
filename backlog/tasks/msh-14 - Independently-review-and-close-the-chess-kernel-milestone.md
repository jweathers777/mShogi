---
id: MSH-14
title: Independently review and close the chess-kernel milestone
status: In Progress
assignee:
  - '@claude'
created_date: '2026-10-22T13:27:00Z'
updated_date: '2026-10-24T19:57:00Z'
labels:
  - review
  - milestone
  - ci
milestone: m-1
dependencies:
  - MSH-11
  - MSH-12
  - MSH-13
documentation:
  - docs/design.md
  - AGENTS.md
priority: high
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Establish that the integrated first milestone satisfies its stated contract and is reproducible by a fresh implementation session.

## Design references

M1 exit criteria; P-05; P-06

## Scope

Review the integrated rules/state/console boundaries, fixture independence, coverage ledger, and reproducibility. Obtain actual headless build/test/smoke evidence for all three platforms. Resolve material defects through bounded follow-up work and prepare the next milestone from real evidence.

## Verification expectations

Record reviewed revision, independent reviewer, findings, relevant commands, operating systems, and results. Reuse evidence that still applies to the integrated revision and rerun only what integration or a finding invalidated.

## Out of scope

Adding new product scope during review, rerunning expensive checks without a concrete reason, and declaring full chess adjudication or playing strength.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every M1 exit criterion has an evidence reference and the integrated revision is identified.
- [ ] #2 Actual Windows, macOS, and Linux build/test/console smoke results are recorded; unavailable evidence is not marked passed.
- [ ] #3 Independent review has no unresolved material correctness finding within M1 scope.
- [ ] #4 The coverage ledger and public documentation clearly distinguish the completed kernel from deferred chess adjudication, search, GUI, and other variants.
- [ ] #5 A fresh session can reproduce the documented workflow, and concrete next-milestone tasks are prepared without silently reducing the full product scope.
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
1. Claim MSH-14 on claude/next-task at 35a4d28 (equal to origin/codex/rewrite; last product-code CI 40126682337 on 4f4025c, followed by backlog-only commits). This is the owner-requested queue after MSH-13.
2. Fresh-session reproduction: clone origin codex/rewrite into an empty scratch directory and run exactly the documented README/development workflow there: build, help/version/profile, a shell smoke, fmt, clippy, the full test suite, and optionally the oracle checkers and differential. Record commands, outcomes, host and revision.
3. Write docs/review/m1-closure.md: the integrated revision; each of the 9 M1 exit criteria with its evidence references (task, tests and fixtures, CI runs, review); per-platform build/test/smoke evidence; deferred obligations (adjudication, history, search, GUI, other games); known limits.
4. Independent milestone review by a fresh reviewer of the integrated kernel against the exit criteria, the ledger and public documentation. Resolve material findings with bounded fixes, or record blocking follow-ups.
5. Prepare concrete M2 tasks (search and match baseline: match/history semantics, a rules-owned history layer, draw/dead-position policy, evaluator, an independently checked search, cancellation and request identity, a reproducible self-play runner, the first protocol adapter) plus small follow-ups found during M1, without reducing full product scope.
6. Integrate with a normal push; native CI on the closure revision; close MSH-14 and the M1 milestone records.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-24): owner @claude, branch claude/next-task in worktree .claude/worktrees/next-task at 35a4d28, equal to origin/codex/rewrite. MSH-11, MSH-12 and MSH-13 are Done. Owner-requested queue (carry on until no work remains). Pinned backlog.md 1.55.1, Rust 1.89.0.

Fresh-session reproduction (2026-10-24, macOS 27.0.1 arm64, Apple M1 Pro). A new clone of origin codex/rewrite at 35a4d2836ba907c803698e1824f78b1cbc81a51f went into an empty scratch directory; the clean tree had 0 changed files. Running the documented README/development workflow there, every step exited 0:
- rustup selected 1.89.0 through the repository pin; rustc 1.89.0;
- cargo build --workspace --locked;
- mshogi --help, --version (mshogi 0.1.0), profile, and a shell pipe giving perft 3 8902;
- cargo fmt --all -- --check;
- cargo clippy --workspace --all-targets --locked -- -D warnings;
- cargo test --workspace --locked: every suite, 13.6 s;
- python3 tools/check_chess_corpus.py (structure 68);
- oracle checkers, all PASS: corpus --oracle (41 checked, 27 rejection contracts deliberately skipped), transitions 49/74, attacks 29/155/3712, candidates 17/274/241, special 21/6/25/18, classification 8, divide 14 (5,004,333 leaves);
- cargo build --release; differential default seed: 92 starts, 15,008 plies, 0 disagreements;
- release --ignored deep perft (3 divides) and deep sequences (316,880 plies) both PASS.
The python-chess oracle environment is the separately installed research venv described in the fixture README.

Independent milestone review (2026-10-24): a fresh delegated reviewer (Claude subagent, no repository edits) reviewed claude/next-task at 35a4d28 (product code identical to 4f4025c) plus the draft closure document. Verdict: M1 can close once should-fix items are resolved; no blocking or material correctness finding within M1 scope.

The reviewer read all core and console code and found no correctness issues. It confirmed castling, EP, promotions, counters, IllegalPosition meaning only king safety, sound counter normalization, no skipped perft branches, atomic console move/undo, forbidden unsafe code and no production dependencies.

Reviewer reruns PASS (macOS arm64, rustc 1.89.0):
- cargo test --workspace --locked: 74 passed, 2 ignored;
- release --ignored deep perft and sequences (316,880 plies);
- fmt; clippy; cargo tree;
- differential seed 31337: 15,090 plies, 0 disagreements;
- differential seed 77 with perft 3, 20 x 150: 246,891 plies, 0 disagreements;
- all fixture counts recomputed and matching;
- all nine cited CI runs checked with gh run view: success on all three jobs at the cited heads; the Windows log of 40126682337 confirmed msvc, 11 CLI tests and the shell smoke.

Should-fix, resolved:
(1) The closure doc now identifies the reviewed revision 35a4d28, its identical product code at 4f4025c and primary run 40126682337; the closure commit's own run is recorded below.
(2) The platform claim is corrected: 40126682337 is the primary all-platform evidence, the per-task runs (adding MSH-4 39969119678) are history, the shell smoke exists only since 8670c73, and Windows runs 11 CLI tests.
(3) Fixture independence is described per family (oracle-enumerated answers with hand rationale versus hand expectations confirmed by the oracle).
(4) The ledger platform row is split into the implemented headless kernel and the deferred packaged desktop (M3).
(5) The ledger is rewritten with a State column (Implemented/Deferred/Unresolved/Outside scope), rows for protocols, search, learned evaluation, history layer and the Unresolved design questions; learned evaluation is added to the closure deferred list.
(6) Stale statements fixed in development.md (FEN suite note, MSH-3 evidence note, HTTPS clone URL) and README (platform evidence pointer); START-HERE.md and PACKAGE-VALIDATION.md are labelled historical M0 records.
(7) Milestone ownership is explicit: M2 owns every Deferred FIDE match-semantics row, and any postponement needs an owner decision and moves to M4 chess completeness.
Nits recorded and carried into MSH-24 rather than changed: sequence parse-error index, structured rejection reason, console input bounds, profile wording, non_exhaustive enums (not needed while publish = false).

Next-milestone preparation (AC #5): created M2 tasks with the pinned backlog.md 1.55.1 CLI, all Ready and unassigned, milestone m-2, with dependencies; backlog doctor reports no duplicates or cycles:
- MSH-15 match/history/adjudication semantics profile with owner decisions;
- MSH-16 rules-owned game history and record;
- MSH-17 draw claims, automatic results and decided dead-position scope;
- MSH-18 match commands and clock model;
- MSH-19 transparent evaluator;
- MSH-20 independently checked full-width search with a performance baseline;
- MSH-21 search request identity, cancellation and stale-result rejection;
- MSH-22 first protocol adapter with a machine output contract;
- MSH-23 reproducible self-play and match runner;
- MSH-24 M1 hardening carry-overs.
M3 (packaged desktop), M4 (five other games, chess completeness postponements) and M5 (strength, learned evaluation) stay as milestone-level obligations to refine when they approach, per design section 5. Full six-game scope is unchanged.
<!-- SECTION:NOTES:END -->
