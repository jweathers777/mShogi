---
id: MSH-24
title: 'Harden M1 boundaries: structured rejection reasons and console input bounds'
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
labels:
  - rules
  - console
  - hardening
milestone: m-2
dependencies:
  - MSH-14
documentation:
  - docs/review/m1-closure.md
priority: low
ordinal: 24000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M1 reviews left recorded, non-blocking limits. FenError exposes only a category and message, so a test matches rejection reasons by text and ActionError::IllegalPosition assumes king safety. The console echoes raw input in errors, accepts Unicode whitespace and a leading plus sign, has no line-length or perft-depth bound, and gives a parse error inside a move sequence without its index. These should be fixed before more adapters build on the same boundaries.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Structural FEN and position rejection reasons are available as a structured value, and tests no longer match message text
- [ ] #2 Console input handling documents and enforces its grammar (ASCII separators, numeric syntax) and has explicit bounds on line length and diagnostic depth, with clear errors
- [ ] #3 A parse failure inside a move sequence reports its index, consistent with other sequence errors, and existing process tests are updated accordingly
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
