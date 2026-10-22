---
id: MSH-14
title: Independently review and close the chess-kernel milestone
status: Ready
assignee: []
created_date: '2026-10-22T13:27:00Z'
labels:
  - review
  - milestone
  - ci
milestone: m-1
dependencies:
  - MSH-11
  - MSH-12
  - MSH-13
documentation:
  - docs/design.md
  - AGENTS.md
priority: high
ordinal: 14000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Goal

Establish that the integrated first milestone satisfies its stated contract and is reproducible by a fresh implementation session.

## Design references

M1 exit criteria; P-05; P-06

## Scope

Review the integrated rules/state/console boundaries, fixture independence, coverage ledger, and reproducibility. Obtain actual headless build/test/smoke evidence for all three platforms. Resolve material defects through bounded follow-up work and prepare the next milestone from real evidence.

## Verification expectations

Record reviewed revision, independent reviewer, findings, relevant commands, operating systems, and results. Reuse evidence that still applies to the integrated revision and rerun only what integration or a finding invalidated.

## Out of scope

Adding new product scope during review, rerunning expensive checks without a concrete reason, and declaring full chess adjudication or playing strength.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Every M1 exit criterion has an evidence reference and the integrated revision is identified.
- [ ] #2 Actual Windows, macOS, and Linux build/test/console smoke results are recorded; unavailable evidence is not marked passed.
- [ ] #3 Independent review has no unresolved material correctness finding within M1 scope.
- [ ] #4 The coverage ledger and public documentation clearly distinguish the completed kernel from deferred chess adjudication, search, GUI, and other variants.
- [ ] #5 A fresh session can reproduce the documented workflow, and concrete next-milestone tasks are prepared without silently reducing the full product scope.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->
