---
id: MSH-229
title: Cut Overbuilt Auditing and Exactness
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-22T16:43:00Z'
updated_date: '2026-11-23T10:59:00Z'
labels:
  - refactor
  - simplification
milestone: m-6
dependencies:
  - MSH-228
priority: high
ordinal: 236000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Early development (ChatGPT agents, approved without enough challenge) built audit trails, exactness proofs and defensive checks far beyond what the product uses: for example the saved document's full journal and context copies (MSH-228), engine failures written into game history as Stop commands, per-command full-record JSON size checks and full game copies in the rules cores (MSH-227), work-unit models and reservations to protect data nobody reads, and redundant identity fields in every embedded record. John (2026-11-22): find all such fat across the codebase and cut it. Game rules and real safety (no crash, no hang, no data loss of the game itself) stay.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A written inventory (task notes) of every audit, provenance, exactness or defensive mechanism in crates/ and desktop/, each with what it protects, whether anything uses it, and a keep/cut/simplify verdict with a reason
- [ ] #2 Every cut or simplification is either done here in small reviewed steps or split into its own task with a clear goal; no game rule changes and no rules test is loosened
- [ ] #3 Line counts before and after, and the checks (fmt, clippy, workspace tests, frontend tests) pass after each step
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started from main 8a9bf74 after MSH-225. Branch codex/msh-229-cut-auditing. First map mechanisms and their consumers across crates/ and desktop/, accounting for the completed minimal document/session cuts and existing MSH-227 replay work. No game rules or safety guarantees will be weakened.
<!-- SECTION:NOTES:END -->
