---
id: MSH-216
title: Design Trim
status: Done
assignee:
  - '@claude'
created_date: '2026-11-20T23:23:00Z'
updated_date: '2026-11-20T23:23:00Z'
labels:
  - docs
milestone: m-6
dependencies: []
ordinal: 223000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Goal: trim docs/design.md to the current product requirements (P), architectural boundaries (A) and decisions (D), removing dated handoffs, milestone narratives and process rules that AGENTS.md now holds. No requirement or decision may change meaning.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Every P, A and D ID of the previous design.md is present with its current meaning
- [x] #2 Dated handoffs, milestone history and process rules are removed; links resolve
<!-- AC:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
design.md 819 -> 381 lines. Kept all P-01..P-06, A-01..A-11 and D-01..D-22 IDs. Removed: dated handoffs (MSH-123/124/125/121 status paragraphs), M0/M1 definitions and exit criteria, 2026-11-08/09 strength-direction paragraphs (superseded by the roadmap, D-09 now points to it), planning-correction narrative, verification Level history and the development workflow section (now AGENTS.md). Tenjiku visual decisions moved into A-07 with the existing anchor kept. Milestone table states checked against MSH-25/39/47 (Done) and MSH-56/60 (To Do). D-04 marked superseded by AGENTS.md, D-05/D-06/D-12 marked done. P-06 and D-03 (design in ChatGPT) kept verbatim; they need John's decision if the workflow changes. Mechanical check: all IDs and 31 key requirement terms present; link check clean.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
design.md reduced from 819 to 381 lines with all requirement, boundary and decision IDs kept and their meaning unchanged; history and process moved out (Git history, AGENTS.md, strength roadmap). Open question for John: P-06 and D-03 still name ChatGPT for design decisions.
<!-- SECTION:FINAL_SUMMARY:END -->
