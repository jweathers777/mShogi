---
id: MSH-15
title: Define the chess match, history and adjudication semantics profile
status: Done
assignee:
  - '@codex'
created_date: '2026-10-24T19:56:00Z'
updated_date: '2026-10-24T20:57:00Z'
labels:
  - rules
  - match
  - design
milestone: m-2
dependencies:
  - MSH-14
references:
  - tests/fixtures/chess/match-profile.json
documentation:
  - docs/rules/chess-coverage.md
  - docs/design.md
  - docs/review/m1-action-boundary.md
  - docs/rules/chess-match-v1.md
priority: high
ordinal: 15000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M1 deliberately left history and match adjudication deferred. Define the versioned M2 software profile from John’s approved 2026-10-24 directions and primary FIDE references, with supporting fixtures and honest M4 completeness ownership. Only additional material behavior or scope conflicts return to the owner; internal APIs, representations, universal save schemas and speculative cache design do not block this work.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A versioned chess match/history profile specifies history, repetition identity, current/intended claims, automatic precedence, a sound bounded dead-position recognizer, match commands, timeout/resignation uncertainty, and untimed/increment clock ordering with primary references.
- [x] #2 Material M2 behavior and scope decisions are recorded from John; ordinary internal APIs and representations belong to the implementer. Neither an all-six-game save schema nor future transposition-table design blocks M2. Deferred completeness has named M4 ownership.
- [x] #3 Independent expected fixtures with provenance or hand-derived rationale cover each specified claim, automatic result, limited recognition, match policy and clock event ordering for later implementation.
- [x] #4 Coverage ledger and design link the decided profile without describing specifications as implemented.
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
1. Research FIDE 2023 articles 5, 6 and 9 and document the approved software profile with explicit unknown outcomes.
2. Define independent declarative fixtures for history, claims, precedence, recognition, commands and clocks; validate fixture positions/actions with existing rules.
3. Update design/ledger and link named M4 follow-up. Obtain bounded independent review, resolve findings, integrate and record CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner direction 2026-10-24: preserve optional core claims; unattended runner may exercise valid claims under explicit recorded policy respected by search. Imported FEN starts known repetition history and preserves counters with missing earlier history recorded. Sound incomplete dead-position recognition is approved with named M4 completion; unknown timeout/resignation mate feasibility is unresolved and unscored. Support untimed and basic increment clocks; record match commands/policy/clock events. Evaluation adjudication is disabled; engine failures are separate from chess outcomes; physical tournament procedures are outside scope.

Owner: Codex; clean existing checkout .claude/worktrees/next-task, branch codex/m2-baseline from origin/codex/rewrite c4ed581. No active task owner was present. Backlog.md 1.55.1 cached standalone CLI used because global 1.55.0 mismatches baseline and system Node has a missing dylib.

Independent review by review_profile resolved projected fourth-occurrence wording, added explicit-only versus auto-claim policy fixtures and intended-fifty-before-mate coverage, corrected history-activation/threshold details, and validated clock paths and mating witnesses. Final reviewed profile has no unresolved material finding. Preparation validates inputs/paths only, not future adjudication.

Profile docs/rules/chess-match-v1.md and 35 hand-derived position/history/command cases plus 6 injected-clock cases in match-profile.json. cargo test -p mshogi-core --test match_profile_preparation --locked passed; verifies fixture positions, action/clock paths, mating witnesses and identical-FEN history pair, not future adjudication. Primary FIDE 2023 articles 5/6/9 consulted. Review_profile independent review resolved wording and fixture policy/threshold findings; no material profile/planning issue remains. Backlog.md 1.55.1 doctor reports no duplicate IDs, self-dependencies or cycles. New work: MSH-25 M2 integration closure, MSH-26 M4 full dead/possible-mate completeness.

Local verification 2026-10-24: macOS 27.0.1 (26A434), arm64; rustc 1.89.0 (29483883e), cargo 1.89.0. cargo fmt --all -- --check, cargo clippy --workspace --all-targets --locked -- -D warnings, cargo test --workspace --locked, cargo build --workspace --locked passed on the working batch based on c4ed581. Existing opt-in deep perft/sequence tests remain ignored; no rerun of the M1 deep oracle suite is claimed. Native three-platform CI will be recorded against the integrated source revision before completion.

Integrated normally and pushed to origin/codex/rewrite at 1264e4bf90f2277a11840fefdbafafe8e0d30e64. Native Rust workspace CI https://github.com/jweathers777/mShogi/actions/runs/40131950077 completed successfully on ubuntu-24.04 (job 120985329413), windows-2022 (120985329549) and macos-15 (120985329551): toolchain identity, formatting, Clippy, workspace tests, build and console smoke all passed. This final source revision covers the three batch tasks; unchanged M1 deep/oracle evidence is reused. Closure bookkeeping changes only Backlog records and reuses this exact source evidence. The previously stale rewrite-bootstrap checkout was fast-forwarded without altering master or legacy history.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the versioned mshogi.chess.match.v1 specification and 35 position/history/command plus 6 deterministic-clock fixtures, with primary FIDE 2023 references and independently reviewed interpretations. Claims remain optional and distinct from outcomes; intended claims, precedence, known-history imports, bounded dead recognition, unscored feasibility uncertainty, clocks and replay/failure policy are explicit. Routine internal choices remain autonomous. Refined existing M2 identities/dependencies, selected UCI with independent-engine match evidence required in MSH-23, corrected determinism and added MSH-25 closure / MSH-26 M4 completeness. Preparation and native CI passed; no material review finding remains. Specification does not claim adjudication is implemented. Source a8f016d; integrated evidence 1264e4b. Next: MSH-16.
<!-- SECTION:FINAL_SUMMARY:END -->
