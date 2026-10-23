---
id: MSH-4
title: Implement explicit chess position state and the FEN boundary
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-22T13:26:00Z'
updated_date: '2026-10-23T14:22:00Z'
labels:
  - rules
  - state
  - parsing
milestone: m-1
dependencies:
  - MSH-2
  - MSH-3
documentation:
  - docs/design.md
  - AGENTS.md
  - docs/rules/chess-position-v1.md
  - docs/rules/chess-coverage.md
  - tests/fixtures/chess/README.md
priority: high
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Represent all M1-relevant chess state explicitly and make positions reproducible through a well-defined input/output boundary.

## Design references

A-02; A-03; A-04; M1 rules profile

## Scope

Implement board/side state, castling rights, en passant state, relevant counters, explicit validation, and FEN parsing/serialization under the adopted profile. Keep history requirements and match state distinct. Prefer understandable state over speculative compression.

## Verification expectations

Use valid and malformed independent fixtures, semantic round trips, position invariants, and explicit error cases. Check that failed external input leaves existing caller state unchanged.

## Out of scope

A universal six-game FEN format, claims that a FEN establishes repetition history, final public ABI, and move generation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All M1-relevant position fields have explicit meanings and invariants documented in the code or profile.
- [ ] #2 Valid profile fixtures parse and serialize with the defined semantic equivalence.
- [ ] #3 Malformed or unsupported input produces clear deterministic errors without partially mutating an existing position.
- [ ] #4 The code does not imply that board/FEN equality establishes match-history or repetition equivalence.
- [ ] #5 Tests cover boundary fields and validation policy, with independent fixture expectations rather than only self-round-trips.
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
1. Claim MSH-4 only on clean codex/rewrite; inspect completed MSH-2/MSH-3 contracts, fixtures, core and CI.
2. Add straightforward chess value types and private validated position state (64 cells, side, historical castling permissions, raw EP target, u32 counters), with read-only access and explicit saved-state equality semantics.
3. Implement strict profile FEN parsing/canonical serialization, ordered syntax/range/structure errors and atomic replacement. Add only the private geometric attack helper required by nonmoving-king validation; leave public attacks, actions and move generation to later tickets.
4. Consume the unchanged independent corpus in Rust tests via a pinned test-only JSON dependency; add meaningful boundary/error precedence/ASCII whitespace, king-safety, state identity and failed-replacement tests. Keep production core free of I/O/external dependencies.
5. Update capability/coverage/docs honestly, run focused checks, obtain independent review, resolve findings, commit/push normally, inspect native CI and record exact evidence and next ready tasks.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup (2026-10-23 Europe/Paris): clean codex/rewrite at be615fa14b9c5eca8aefc77a4b93af4775c73227, remote tip independently confirmed equal. Reusing /Users/johnweathers/.codex/worktrees/rewrite-bootstrap/mShogi. No In Progress task; MSH-4 sole Ready+ready. MSH-2 and MSH-3 dependencies are Done. @codex owns MSH-4 only. Reviewed AGENTS, design, named chess profile, corpus, development/CI and core/console sources; pinned Backlog.md 1.55.1 and Rust 1.89.0 retained. Original master/main_v2 refs, legacy snapshot and unrelated records remain preserved.

Implementation choices stay inside the adopted profile: private direct board state rather than generic variant framework; no history/result/repetition-key claim; staged syntax/range/structure validation and temporary parse before replacement. A private geometric attack scan is necessary for profile requirement that the nonmoving king not be attacked; it will not expose or certify MSH-6's public attack API or implement move generation.

Test dependency rationale: consume the existing committed JSON corpus in Rust rather than duplicate its expected answers or invent a JSON parser. Reviewed official serde_json v1.0.145 manifest/license (https://github.com/serde-rs/json/tree/v1.0.145; MIT OR Apache-2.0, rust-version1.61) before selecting it as an exact-version dev-only dependency. Lock transitive versions, inspect their declared licenses and verify production cargo tree stays dependency-free. No new runtime dependency or release-license decision.

Candidate verification (2026-10-23 Europe/Paris), base be615fa14b9c5eca8aefc77a4b93af4775c73227 plus MSH-4 diff: explicit private 64-cell chess state with bounded squares, color/kind, historical rights, raw EP target, u32 counters and read-only accessors; no history/cache/result fields or move/action API. Position equality covers the entire saved tuple and is documented not to establish repetition/history identity. from_fen/FromStr, canonical to_fen/Display and atomic set_fen implement the existing profile unchanged. Minimal private geometric scan enforces nonmoving-king safety, including pinned attackers and blockers; public attacks remain MSH-6 work.

Passed on macOS 27.0.1 arm64 with pinned Rust/cargo 1.89.0: cargo fmt --all -- --check; cargo clippy --workspace --all-targets --locked -- -D warnings; cargo test --workspace --locked (8 core integration tests plus 5 console subprocess tests); cargo build --workspace --locked; cargo run --locked --bin mshogi -- profile (honest partial coverage). Core test independent_corpus_fen_contracts_and_canonical_positions consumes all 68 unchanged MSH-2 inputs: 41 accept/canonical answers and 27 expected rejection categories, repeated deterministic errors, failed replacement leaves full caller state unchanged, successful replacement and semantic round trips. It does not execute stored legal-move/perft/classification expectations. Initial compiler check found one inferred i32 castling-board index; explicit usize fixed it before the passing suite; no unresolved failure.

Other durable Rust cases cover all six specified ASCII whitespace characters (including VT, which standard split_ascii_whitespace omits), leading zeroes, u32 maxima/overflow/fullmove zero, cross-field syntax-before-range-before-structure precedence, unsupported symbols, both-color EP invariants, non-inferred rights and attacked/blocked paths, saved-state identity differences, geometric pawn/knight/slider/pinned attacks and moving-side single/double check allowance. Fixed-seed robustness loop exercises 3072 malformed inputs without panic or partial replacement; it is supplementary robustness evidence, not generated rule truth or a fuzzing platform.

Dependency evidence: cargo tree --workspace --locked --edges normal shows console -> core only; core third-party dependency is exclusively dev serde_json =1.0.145. Cargo metadata declarations inspected for locked dev graph: serde_json/serde/serde_core/serde_derive, itoa, proc-macro2, quote and syn are MIT OR Apache-2.0; memchr Unlicense OR MIT; ryu Apache-2.0 OR BSL-1.0; unicode-ident (MIT OR Apache-2.0) AND Unicode-3.0. No copyleft runtime dependency or vendored source; existing product license decision unchanged. Relevant versions/checksums are in Cargo.lock. Optional Python structural checker passes all68; messaging now points rejection-policy execution to Rust tests rather than stale future MSH-4 text.

Preservation/docs checks passed: corpus.json, legacy snapshot, BSD-LICENSE and AGENTS.md byte-unchanged from base; relative documentation links resolve; git diff --check clean. README/development, API/profile, coverage ledger and corpus reproduction now distinguish implemented FEN from future game operations. No new console FEN commands, six-game notation, public attack/move behavior, full adjudication, GUI or training framework added.

Independent review: separate read-only /root/msh4_review found no actionable findings against original profile/AGENTS/design/task. Independently ran all8 core integration tests, corpus structural checker, dependency/license/tree and preservation checks. Additional temporary external review probe (outside checkout, not a claimed committed/CI suite) passed111990 generated validation cases with a separately written target-outward ray oracle covering both colors, attacking piece kinds, edges and blockers; counter probes with up to100000 leading zeroes, max/overflow/zero also passed. Reviewer confirmed state privacy, saved equality, validation ordering, atomic replacement, canonicalization, analysis allowances, rights/raw EP and accurate scope text. No material finding remains. Review was host-only; native CI and commit/push evidence follow.

AC mapping: #1 typed state, private invariants and getters documented in code/profile; #2 exact independent accepted/canonical fixture answers; #3 deterministic staged errors and unchanged caller on every rejected fixture plus targeted precedence cases; #4 explicit equality/history boundary in API/docs; #5 original independent corpus consumed unchanged plus meaningful boundary/robustness tests. Next tasks after completion will be MSH-5 and MSH-6, neither started.
<!-- SECTION:NOTES:END -->
