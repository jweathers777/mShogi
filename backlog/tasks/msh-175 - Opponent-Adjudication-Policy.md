---
id: MSH-175
title: Opponent Adjudication Policy
status: Ready
assignee: []
created_date: '2026-11-14T14:14:00Z'
labels:
  - handle:opponent-adjudication-policy
milestone: m-5
dependencies: []
references:
  - docs/design.md
  - docs/testing/conformance-policy.md
  - crates/mshogi-runner/src/experiments/mod.rs
  - crates/mshogi-runner/src/experiments/evidence.rs
  - docs/m4/chu/profile.md
priority: medium
type: task
ordinal: 179000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Opponent Adjudication Policy.

**Problem**
Legacy and other external engines can propose illegal actions, reject native-legal actions, disagree on repetition or endings, claim a draw, or fail to return a move. Treating these events as ordinary wins/draws would hide incompatibility and distort comparisons.

**Objective**
Give shared runner adapters a consistent policy for admitting opponent actions, adjudicating actual games and recording disagreements or execution failures.

**Approach**
The selected native profile remains the sole authority for real legality, history and game results. Adapters translate and report capabilities; they do not invent variant rules, substitute a legal move for an illegal proposal, or repair history silently. Scope first to legacy Chu and existing supported external protocols, sharing applicable runner mechanisms. Record compatibility before a match. By default, unsupported semantics or unresolved disagreement stop the game as unscored. A declared experimental forfeit policy may score protocol/participant violations only in a separate match ledger; it must not fabricate a native win or silently enter native-result strength statistics. Existing accepted protocol/clock contracts remain intact. Material rules ambiguity is reported with a concrete case rather than resolved by changing the selected profile.

**Completion**
A documented policy and executable shared handling distinguish native conclusions, actual concessions/consent, rule mismatches, illegal proposals, malformed output, hangs and crashes. Reports preserve replayable accepted state and raw failure evidence; independent review and tests establish behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Each game binds its selected profile and declared opponent capabilities; preflight identifies known incompatible or unsupported semantics and records whether execution is admitted or refused.
- [ ] #2 Every opponent action is translated and checked by native rules before commitment; illegal, ambiguous, malformed, stale or unsupported input leaves accepted state/history unchanged and receives a specific recorded classification.
- [ ] #3 Native endings, repetition obligations, valid resignation and draw agreement are distinguished from opponent result assertions, no-move responses and rejected native-legal moves; no claim or missing move silently becomes a native conclusion.
- [ ] #4 Timeouts, crashes, protocol violations and rules disagreements retain their actual causes and cleanup evidence. Unresolved compatibility failures are unscored by default; any explicitly configured experimental forfeit is labeled separately from the native result and excluded from native-result strength summaries.
- [ ] #5 Reports retain profile, policy, accepted complete history, attempted/rejected action or assertion, opponent identities and relevant transcript/state context; replay reproduces native accepted state and validates the recorded disposition without launching opponents.
- [ ] #6 Independent negative fixtures cover legacy repetition and bare-royal disagreement, illegal and ambiguous compound actions, rejection of a native-legal move, false result claims, no-move responses, malformed output, hangs and crashes; existing supported external paths receive relevant regression checks and independent review.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
