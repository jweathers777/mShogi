---
id: MSH-18
title: Implement chess match commands and a minimal clock model
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T23:23:00Z'
labels:
  - match
  - clocks
milestone: m-2
dependencies:
  - MSH-16
  - MSH-17
references:
  - tests/fixtures/chess/match-profile.json
documentation:
  - docs/rules/chess-coverage.md
  - docs/design.md
  - docs/rules/chess-match-v1.md
  - docs/rules/chess-match-api.md
priority: medium
ordinal: 18000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Resignation, agreed draws and time loss are match commands or match state, not board actions (A-02), and FIDE 5.1.2 and 6.9 make their outcomes depend on whether mate is still possible. Reproducible self-play and engine matches in M2 need at least a deterministic time-control model with recorded limits.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Resignation and agreed-draw commands are distinct from board actions and follow the decided profile, including the possible-mate exceptions
- [x] #2 A clock model supports the time controls the match runner needs, deterministically under test, and time loss applies the possible-mate exception per the profile
- [x] #3 Match commands and clocks never mutate the rules position directly and are covered by tests with independent expectations
- [x] #4 Untimed and basic increment clocks obey documented deterministic event ordering and boundary tests. Timeout/resignation without established mating feasibility ends explicitly unresolved and unscored; agreed draws require both players to have moved.
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
1. Add rules-owned match commands with explicit versions/actors, offer/agreement handling and conservative side-specific mating certificates for resignation/timeout. 2. Implement untimed and injected-time increment clocks with exact event ordering, atomic rejected board commands, explicit uncertainty and typed replayable requests/observations. 3. Execute independent profile command/clock fixtures and replay/boundary tests; document semantic replay and limitations, obtain independent review and native integration evidence.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex, same clean integration source after MSH-17; scoped implementation is core matchplay plus tests/docs. Search task MSH-20 is delegated to isolated search-crate files; Codex remains primary integrator. Positive mating possibility requires a verified legal witness; failure to prove either possibility or impossibility is unresolved and unscored.

ChessMatch implements versioned commands, persistent offer state, injected untimed/increment clocks, explicit unresolved/unscored mating uncertainty, sound bare-king/dead proofs and verified cooperative mate certificates. MatchRecord replays full requests and verifies computed responses without private undo. Eight focused tests pass, executing independent MSH-15 clock and resignation/flag fixtures plus rejection/overflow/offers/claim/replay corruption tests. Hypothetical witness paths follow Article 3 legal move reachability, with counters normalized privately; current live outcomes retain precedence. This interpretation was checked against primary FIDE 3.10.1 and 5.1.2/6.9, not invented as a scope change.

Independent review_hardening found two P2 record-boundary gaps: nested ChessOutcome unknown fields and direct Deserialize schema bypass. Both are fixed with strict nested decoding and replay identity validation; reviewer independently confirmed 10 matchplay tests pass with no material findings. A self-review regression also ensures bad mating evidence cannot cancel an immediate resignation. Full workspace tests/build, formatting, Clippy -D warnings and diff checks pass on macOS arm64/Rust1.89. Tests inject integer time and directly consume profile fixtures; no sleep/timing inference. Console metadata now accurately labels diagnostic status as position-only rather than denying the new library adjudication.
<!-- SECTION:NOTES:END -->
