---
id: MSH-93
title: >-
  Establish complete local xiangqi legal actions and exact transition
  conformance
status: To Do
assignee: []
created_date: '2026-10-27T15:03:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-91
  - MSH-92
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 93000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Expose the complete local legal action contract using the authoritative native state, including both sides, every piece/geography restriction, captures, cannon screens, facing generals, king safety/pins and all evasions. Preserve mate versus stalemate reason although both no-action states lose under the selected profile.
- [ ] #2 Run independently expected fixtures, deterministic legal sequences, applicable established perft/divide diagnostics and admitted-reference complete-set comparisons. Board legality remains separate from history-dependent permission or concluded matches.
- [ ] #3 Verify exact apply/undo restoration for complete choices and atomic malformed/stale/illegal rejection. Include cannon-screen removal/insertion and horse-leg evasions rather than relying on chess sliding-check assumptions.
- [ ] #4 Independently review discrepancies and conformance coverage against the selected clauses. Record actual commands/source/reference revisions, failures and limitations; no expected-data edits solely to match either engine.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
