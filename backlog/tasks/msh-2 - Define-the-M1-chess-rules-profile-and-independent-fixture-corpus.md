---
id: MSH-2
title: Define the M1 chess rules profile and independent fixture corpus
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-23T11:23:00Z'
labels:
  - rules
  - reference
  - design
milestone: m-1
dependencies:
  - MSH-1
documentation:
  - docs/design.md
  - AGENTS.md
  - docs/rules/chess-position-v1.md
  - docs/rules/chess-coverage.md
  - tests/fixtures/chess/README.md
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Establish what the first chess milestone promises and where independently expected answers come from before the implementation defines its own behavior.

## Design references

P-01; P-05; A-03; A-04; M1; verification contract

## Scope

Document a named orthodox-chess position profile, FEN validation policy, M1 rule coverage, outcome limitations, and a reusable fixture format with source/provenance. Curate expected move and perft examples covering ordinary moves, pins/checks, castling, en passant, promotions, mate, and stalemate. Document full-adjudication obligations deferred beyond M1.

## Verification expectations

Review primary rule sources and independently maintained reference cases. Record source version or retrieval details, precise expected values, and any discrepancies. At this stage fixtures may be plain data; executable consumption follows with the implementation.

## Out of scope

Writing the optimized move generator, treating legacy mShogi output as a rule oracle, or declaring full chess conformance.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The rules profile states M1 inclusion and exclusion explicitly, including the distinction between position-level terminal classification and full match adjudication.
- [x] #2 FEN syntax, structural validation, analysis-position policy, and missing history are distinguished; serialization equivalence is defined semantically.
- [x] #3 Fixture format includes rules profile, starting state, operation, independently established expectation, and provenance.
- [x] #4 The corpus covers every M1 move category and meaningful king-safety cases, with expected legal moves or perft counts established independently of mShogi.
- [x] #5 Unresolved rule questions and deferred draw/dead-position/claim obligations are visible in a coverage ledger and do not become silent implementation defaults.
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
1. Inspect repository, completed MSH-1, existing chess prototype and dependent task contracts; claim MSH-2 only on codex/rewrite in the existing isolated worktree.
2. Read primary FIDE rules and FEN specification, plus independently maintained reference cases; record exact versions/retrieval and independent oracle provenance.
3. Document named M1 position profile, deterministic FEN validation/serialization policy and coverage/deferred obligations; curate reusable plain-data fixtures for all M1 move categories, king safety, terminal classification and parser boundaries.
4. Verify fixture structure and all independently checkable expectations using a pinned external oracle, without using mShogi as the oracle; obtain independent review and resolve findings.
5. Update task evidence, commit and normally push to origin/codex/rewrite, preserving existing code/history; report actual integration and next eligible task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup: clean codex/rewrite at 5a0d63b4c9fef18db2e63e9917429ceb65ba40d3 tracking origin/codex/rewrite; existing isolated worktree /Users/johnweathers/.codex/worktrees/rewrite-bootstrap/mShogi. Original master and main_v2 WIP refs preserved. Ready+ready lists MSH-2 then MSH-3; no In Progress task. MSH-1 dependency Done. Owner @codex, MSH-2 only. Reviewed AGENTS.md, design, MSH-2/MSH-3/MSH-4/MSH-12 and prototype manifest, variant loader, move generator/tests. Prototype is incomplete and is not a rules oracle. Global Backlog 1.55.0 remains untouched; all task operations use cached pinned 1.55.1; overview read earlier this conversation, execution guide refreshed. No product code or MSH-3 work in scope.

Candidate evidence (2026-10-23 Europe/Paris): profile mshogi.chess.position.v1 in docs/rules/chess-position-v1.md; coverage/deferred questions in docs/rules/chess-coverage.md; plain JSON corpus and format/provenance under tests/fixtures/chess; optional independent checking tool tools/check_chess_corpus.py. Existing product source/Cargo/variants remain unchanged. README adds links to the profile, ledger and corpus.

Primary sources read: English FIDE Laws effective 2023-01-01 (Articles 3, 5, 6.9, 9); Edwards PGN specification section 16.1 (1994; Saremba transcription 2000); Stockfish sf_17.1 tests/perft.sh standard cases; python-chess v1.11.2 source, documentation and GPL-3.0-or-later license. URLs, retrieval date and source SHA-256 values are recorded in corpus.json. Initial web FIDE fetch timed out; direct HTTPS retrieval succeeded and its actual text was read. Optional oracle installed separately in /private/tmp/mshogi-msh2-oracle (chess==1.11.2, Python 3.11.0), no global/product dependency or vendored upstream code. Native host macOS 27.0.1/Darwin arm64, Git 2.55.0. Standard-only checker also ran under host Python 3.14.7. Baseline verification revision 5a0d63b4c9fef18db2e63e9917429ceb65ba40d3 plus this MSH-2 candidate diff.

Passed python3 tools/check_chess_corpus.py: all 68 fixture records have valid structure, unique IDs, profile, source-backed rationale, input/operation/expectation. Passed /private/tmp/mshogi-msh2-oracle/bin/python tools/check_chess_corpus.py --oracle: all 35 complete legal-move/classification cases, 3 pinned-attacker queries, 38 perft depth/count expectations across 11 cases, and 6 accepted-FEN canonicalizations agree with pinned independent chess 1.11.2 (module SHA-256 1fde6a8e932508d14b31f6a584497b49c2969fc3c7cdc675347f819f0d667a5b). Counts at depth three for six maintained reference positions total 271312. Twenty-seven rejected-FEN examples are explicit future MSH-4 parser contracts, not passed product parser tests. All expectations were established without executing mShogi. Published Stockfish deep counts were not run; shallow counts are honestly attributed to oracle enumeration.

Verification tool sensitivity: four temporary mutations (remove one initial legal move; change initial depth-two count 400 to 401; unknown source ID; unknown operation) each exited 1 with the case ID and an actionable diagnostic. Temporary mutants were outside the repo. Relative documentation links resolve, git diff --check passed, tracked product files are unchanged and original master checkout is clean. No cargo/product build, rules conformance, performance, strength or Windows/Linux execution is claimed. MSH-3 and later tasks retain those obligations. Independent review is in progress; no AC/DoD checked yet.

Independent review complete: separate read-only reviewer /root/msh2_review found no actionable findings and confirmed all five MSH-2 acceptance criteria have supporting documentation/data. Reproduced 68 structure checks and 41 pinned-oracle comparisons, confirmed all source snapshot hashes and the six Stockfish positions/counter adaptations, and manually checked critical FIDE-based move memberships for both castling colors, origin/transit/destination safety, b1 blocker, attacked-rook allowance, EP discovered check/evasion, pinned attacks, all promotions, mate and stalemate. Reviewer ran six temporary negative probes (unknown operation, duplicate move, wrong perft, missing legal castle, wrong pinned attack, suppressed raw EP); all failed as expected with diagnostics. No repository/task mutations by reviewer. No findings remain. Review does not establish mShogi parser rejection, apply/undo, deep perft, full adjudication or platform certification.

AC mapping: #1 named profile scope/outcome boundary + coverage ledger; #2 explicit syntax/range/local structure/analysis policy and canonical semantic tuple + 33 FEN contracts; #3 68 records with profile, starting FEN, operation, exact expectation, source/version/locator/method/rationale; #4 35 full legal move/classification cases, both special-move colors, pins/checks/x-ray and six independently maintained perft positions, independently checked; #5 ledger explicitly retains dead position, all claim/automatic draw obligations, resignation/time exceptions, history/event/API questions and later implementation owners. All data/contract criteria satisfied; no product implementation completion is inferred.

Next Ready+dependency-ready task will remain MSH-3 only. MSH-4 still requires MSH-3 in addition to MSH-2. No new tasks or unrelated work started. Normal integration and push verification follow; final task completion will record the actual implementation commit and remote outcome.
<!-- SECTION:NOTES:END -->
