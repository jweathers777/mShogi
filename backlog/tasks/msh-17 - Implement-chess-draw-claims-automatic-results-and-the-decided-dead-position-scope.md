---
id: MSH-17
title: >-
  Implement chess draw claims, automatic results and the decided dead-position
  scope
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-24T19:57:00Z'
updated_date: '2026-10-24T23:12:00Z'
labels:
  - rules
  - adjudication
milestone: m-2
dependencies:
  - MSH-16
references:
  - tests/fixtures/chess/match-profile.json
documentation:
  - docs/rules/chess-coverage.md
  - docs/rules/chess-match-v1.md
  - docs/rules/chess-adjudication.md
priority: high
ordinal: 17000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The M1 classifier deliberately reports quiet for positions that FIDE ends or allows a claim on, for example bare kings, the 75-move rule or fivefold repetition. With the profile decided (MSH-15) and rules-owned history in place (MSH-16), the library must produce those outcomes. It must not present a heuristic material check as complete dead-position adjudication.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Threefold and fifty-move claims, including intended-move claims, are available exactly as the decided profile specifies and match independent fixtures
- [x] #2 Automatic fivefold and 75-move results apply with the specified precedence, including checkmate on the last move
- [x] #3 Dead-position recognition implements exactly the owner-decided scope, and any incompleteness is stated in the ledger and API docs
- [x] #4 Game results are distinct from the position classifier, and the coverage ledger rows move to Implemented only with evidence
- [x] #5 Available claim opportunities remain separate from concluded outcomes; fixtures cover current/intended claims, imported-history boundaries and the conservative recognition unknown state without treating unknown as mating possibility.
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
1. Add sound bounded dead recognition and rules-owned automatic outcomes, current/intended optional claims, and policy-aware claim selection over Game history. 2. Add an adjudicated game wrapper that rejects live play after outcomes, validates claims atomically, supports exact analysis undo and records/replays accepted claims while retaining the diagnostic history layer. 3. Execute independent profile fixtures plus negative/precedence/history/undo/replay tests, update ledger/API docs, obtain independent review, integrate and record native CI.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner Codex; branch codex/m2-remaining in the existing next-task checkout, based on clean origin/codex/rewrite a2e0c6e. MSH-16 is complete. Game remains a diagnostic/history foundation; AdjudicatedGame is the live rules boundary used by subsequent search/match tasks, preventing diagnosis-only histories from masquerading as validated live games.

Implemented Game snapshot outcome/claim queries and AdjudicatedGame live path enforcement, optional accepted-claim state, exact event undo, canonical claim records/replay and policy selection. Dead recognition is explicitly sound/incomplete; Unknown never asserts mating possibility. Tests directly consume independent MSH-15 expected fixtures, plus invalid/late/actor/overflow/live-replay cases. Diagnostic Game/perft behavior remains independent of match adjudication.

Independent review_profile found no material issues and independently passed seven fixture tests. Its optional exact-policy assertion improvement was added: current versus intended priority, threefold versus fifty and square order are now asserted directly (eight focused tests). Local workspace tests, build, Clippy -D warnings, formatting and diff check passed on macOS arm64 / Rust 1.89. Existing deep opt-in tests remain ignored. MSH-26 research is separate; no completeness claim beyond the approved subset.
<!-- SECTION:NOTES:END -->
