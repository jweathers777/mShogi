---
id: MSH-19
title: Add a transparent chess evaluation baseline
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T20:54:00Z'
labels:
  - search
  - evaluation
milestone: m-2
dependencies:
  - MSH-14
documentation:
  - docs/design.md
  - docs/evaluation.md
priority: medium
ordinal: 19000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-02 requires mShogi's own engines, and M2 starts with a transparent evaluator before any learned evaluation (A-08). Search tests need an evaluation whose score perspective and components are inspectable and deterministic. Strength targets are explicitly not set yet.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A deterministic transparent static evaluator is available in a separate search layer with documented components, units and score perspective, without UI or Python dependencies; it remains executable without game history or adjudication.
- [x] #2 Tests cover signs for both sides and symmetry under color/rank reflection. Static evaluation does not conclude outcomes or claim availability; documented use requires search to consult rules-owned adjudication before evaluating a live leaf.
- [x] #3 No strength claim is made; any timing is recorded with hardware and build context.
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
1. Add a small native search crate depending on the rules library, with a deterministic material-only chess static evaluator, explicit centipawn constants and score perspective. 2. Hand-derived tests cover both perspectives, piece contributions, color/rank symmetry, counter/history independence and non-adjudicating behavior even on terminal diagrams. 3. Document the search obligation to call rules-owned game adjudication first, without coupling this evaluator to MSH-16/17; run focused tests, independent review and integrated native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex; branch codex/m2-baseline. MSH-19 remains dependency-ready through MSH-14 alone. No strength benchmark or timing claim is intended; material-only is a transparent untrained baseline.

cargo test -p mshogi-search --locked passed (5 integration tests, 1 doctest). Material-only evaluator identity mshogi.chess.material.v1; Pawn100/N320/B330/R500/Q900/K0 centipawns, explicit requested-side perspective. No history/adjudication dependency or strength/timing claim. Only local crate dependency added to lockfile, no third-party production dependency. Independent review_evaluator found no material issue; tests independently rerun by reviewer. See docs/evaluation.md.

Local verification 2026-10-24: macOS 27.0.1 (26A434), arm64; rustc 1.89.0 (29483883e), cargo 1.89.0. cargo fmt --all -- --check, cargo clippy --workspace --all-targets --locked -- -D warnings, cargo test --workspace --locked, cargo build --workspace --locked passed on the working batch based on c4ed581. Existing opt-in deep perft/sequence tests remain ignored; no rerun of the M1 deep oracle suite is claimed. Native three-platform CI will be recorded against the integrated source revision before completion.
<!-- SECTION:NOTES:END -->
