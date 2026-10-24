---
id: MSH-19
title: Add a transparent chess evaluation baseline
status: Ready
assignee: []
created_date: '2026-10-24T19:57:00Z'
labels:
  - search
  - evaluation
milestone: m-2
dependencies:
  - MSH-14
documentation:
  - docs/design.md
priority: medium
ordinal: 19000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-02 requires mShogi's own engines, and M2 starts with a transparent evaluator before any learned evaluation (A-08). Search tests need an evaluation whose score perspective and components are inspectable and deterministic. Strength targets are explicitly not set yet.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A deterministic evaluator with documented components and a stated score perspective is available through the rules-independent search layer, without UI or Python dependencies
- [ ] #2 Tests cover the score sign and perspective for both sides, symmetry on mirrored positions, and terminal handling consistent with the rules classifier
- [ ] #3 No strength claim is made; any timing is recorded with hardware and build context
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
