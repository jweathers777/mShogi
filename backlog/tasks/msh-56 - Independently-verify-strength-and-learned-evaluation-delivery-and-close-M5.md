---
id: MSH-56
title: Independently verify strength and learned evaluation delivery and close M5
status: To Do
assignee: []
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-10-27T16:18:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-47
  - MSH-48
  - MSH-49
  - MSH-50
  - MSH-51
  - MSH-52
  - MSH-53
  - MSH-54
  - MSH-55
  - MSH-104
documentation:
  - docs/design.md
  - docs/m5/opening-knowledge.md
ordinal: 56000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M5 must account for every promised game-specific strength obligation and the initial learned evaluation deliverable. Completing shared tooling or a single-game model does not establish six-game engine strength. Numerical targets and empirical choices remain D-09 decisions in the owning tasks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 M4 is closed at MSH-47, shared infrastructure MSH-48 is complete, and all six game-specific strength tasks MSH-49 through MSH-54 meet their recorded targets with reproducible evidence. No game disappears from the strength scope for lack of an easy opponent.
- [ ] #2 MSH-55 demonstrates the required initial learned-evaluator lifecycle, native integration and reproducible improvement. It does not imply that all six games require the same model or that optional policy/value research succeeded.
- [ ] #3 Independent review checks integrated product behavior, reproducibility, uncertainty and correctness/strength separation, resolves material findings, and records the M5 verdict with exact revisions and remaining M6 work. Incomplete required delivery cannot be waived by task-count percentage or moved into optional research.
- [ ] #4 The required MSH-104 per-game opening-knowledge experiment is complete with independently reviewed reproducible evidence and recommendations. Adoption/shipping of a book or learned opening policy is not mandatory; an evidence-supported no-special-opening conclusion is valid. Original six-game strength targets and MSH-55 learned-evaluator improvement remain required and separate.
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
Owner-approved roadmap addition2026-10-27: MSH104 next availableID is future scope-level opening-knowledge experiment including first-class compactteacher-distilledpolicy arm distinctfrom55evaluator and59optionalpolicy/value. All original AC1–3,DoD and47–55dependencies preserved; adds104 and experiment-completion criterion, not mechanism-adoption requirement. No M5implementation/runtimePython dependency introduced.
<!-- SECTION:NOTES:END -->
