---
id: MSH-225
title: Chess Document and Session Fit
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-22T09:57:00Z'
updated_date: '2026-11-22T16:02:00Z'
labels:
  - desktop
  - refactor
milestone: m-6
dependencies:
  - MSH-226
priority: medium
ordinal: 232000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Chess has its own DesktopSession, ChessDocument and mutation journal (lib.rs, persistence.rs), from the first desktop slice. John (2026-11-22) asked for a serious check whether Chess belongs in the single generic document and session, or at least reuses its parts without copies that can drift. Chess match rules (D-11: claims, auto-claim policy, clocks, ordered match events) are real differences; the rest may be history.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measured comparison of the Chess session and document against the generic ones, with each difference classed as rules, product or history; result recorded in the task
- [ ] #2 Chess uses the generic document and session, or the parts that are the same are shared code with no copies; the reason for any remaining separation is recorded
- [ ] #3 Desktop-session and workspace tests pass; one fresh-agent review
<!-- AC:END -->
