---
id: MSH-35
title: Save reopen and inspect desktop game history
status: Ready
assignee: []
created_date: '2026-10-25T12:48:00Z'
updated_date: '2026-10-25T15:17:00Z'
labels: []
milestone: m-3
dependencies:
  - MSH-33
  - MSH-34
documentation:
  - docs/desktop/m3-design.md
ordinal: 35000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Games must survive application restarts and permit read-only inspection without accidentally altering the live game.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Saved sessions preserve canonical game/match records, policy, commands/events, modes/limits and engine observations; reopening either restores supported semantics or clearly rejects unsupported restoration.
- [ ] #2 Historical viewing is visibly read-only and distinct from live play; returning live is explicit, and any offered branch/change invalidates obsolete work with new session identity.
- [ ] #3 Round-trip, corrupt/unsupported record, history/claim and stale-response tests plus actual desktop save/reopen/inspection evidence pass without silently downgrading modes.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner clarification 2026-10-25: implement only after MSH-34 and MSH-33 complete. Versioned desktop persistence must include canonical MatchRecord and GameRecord semantics, claims/conclusions, commands/events, policies, modes/limits, relevant settings and exact-context observations; not merely FEN/actions or GameRecord helper restoration. Observations reopen as historical, never pending work; runtime identities are fresh, snapshots remain monotonic, untimed engine work stays stopped until explicit Resume. Atomic save failure/interruption preserves prior valid file and dirty state. Corrupt/unsupported/resource-limited open leaves current session intact; validate/replay off interaction path with limits, commit candidate atomically after current-session precondition. Protect unsaved changes on replace/open/ordinary quit. Historical view visibly read-only with explicit Return to live; analysis uses exact history prefix without future repetition/events and rejects late results after selection changes. No branching/variation editing required. Actual save/close/reopen/history/return-live journeys required; separate instrumentation only. Existing AC preserved. MSH-36–39 remain open. See M3 design second-batch clarifications.
<!-- SECTION:NOTES:END -->
