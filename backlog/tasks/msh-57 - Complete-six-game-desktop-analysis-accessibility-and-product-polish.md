---
id: MSH-57
title: Complete six-game desktop analysis accessibility and product polish
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
labels: []
milestone: m-6
dependencies:
  - MSH-47
documentation:
  - docs/design.md
ordinal: 57000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-03/P-04/P-05 require a polished inspectable product across all implemented games. M3 supplies a bounded chess desktop slice and M4 supplies complete functional six-game integration; this task owns the remaining analysis, usability, accessibility and failure-recovery product-quality gaps, not a repeat of their acceptance or speculative new product features.

Open scope questions: inventory the actual residual product gaps against P-03/P-04/P-05 and A-05/A-07 after M3/M4 evidence; define concrete analysis, keyboard/text accessibility, diagnostics and recovery acceptance for the six-game product. Additional modes such as timed desktop play require their own decided clock/suspend/restore contract if selected; this ticket does not silently approve unresolved features or an online/mobile product.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 An evidence-based gap register identifies and scopes the remaining required analysis, interaction/accessibility and polish deliverables, assigning each to this task or an explicit child without duplicating completed M3/M4 obligations. Required gaps stay visible until closed.
- [ ] #2 All six games offer the agreed usable analysis/inspection and accessible board, complete-action selection and readable textual records, including large/intersection boards and compound choices where applicable. Native progress, cancellation, stale results, persistence and recoverable failures remain coherent and responsive.
- [ ] #3 Actual installed-application usability/accessibility and analysis workflows are verified on Windows, macOS and Linux with recorded revisions/environments and independent review. Required product gaps are closed; browser-only checks or successful research experiments do not substitute for product acceptance.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
