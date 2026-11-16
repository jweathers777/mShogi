---
id: MSH-184
title: Native Report Replay Cost
status: Ready
assignee: []
created_date: '2026-11-16T15:38:00Z'
labels:
  - handle:native-report-replay-cost
milestone: m-5
dependencies:
  - MSH-183
documentation:
  - docs/m5/evidence/msh183/round2/README.md
priority: medium
ordinal: 191000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Native Report Replay Cost.

**Problem:** MSH-183 (Tenjiku Search Core Adoption), round 2 Adams, wrote a complete 9.8 MB native report but exceeded the outer 645-second process bound during post-write processing. Execution, JSON export and summary generation each validate the report. Actual request time was 570.69 seconds; the validation passes were not timed separately. This cost is separate from the candidate node-limit failure.

**Objective:** Export and summarize a completed native experiment with bounded cost while keeping native replay and strict report validation authoritative.

**Approach:** Measure the full path on the retained long report and ordinary smaller reports. Remove repeated work where evidence supports it. Preserve rejection of forged, stale, malformed and inconsistent external reports, reproducible failures and process cleanup. No game-rule or score-admission change is authorized.

**Completion:** Measured reduction in repeated work, preserved native validation and independent review. Select verification from actual risk; no new playing-strength campaign is required.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Measure separate execution, validation, export and summary costs on the retained long Tenjiku report and ordinary reports; record baseline, bounds and a decision checkpoint.
- [ ] #2 Reduce repeated native replay in report export and summary generation without weakening external report validation or changing scores, native rules, refusals or cleanup.
- [ ] #3 Relevant runner regressions, malformed-report rejection and independent review pass; record measured benefit and actual verification scope.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->
