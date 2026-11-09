---
id: MSH-150
title: Lazy Native Move Queries
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-09T20:49:00Z'
updated_date: '2026-11-09T20:50:00Z'
labels:
  - handle:lazy-native-move-queries
milestone: m-5
dependencies:
  - MSH-148
documentation:
  - docs/m5/chess-effectiveness-diagnosis.md
priority: high
ordinal: 155000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Lazy Native Move Queries. Current native any-legal-action queries stop validation at a witness but first allocate and generate every candidate. MSH147 CPU samples identify significant native query/allocation cost; MSH148 shows that avoiding quiet projections alone gives no robust aggregate speed gain. Investigate true short-circuit candidate traversal shared by existing public generation and native legal queries. Preserve native rules authority, exact candidate order, promotions/castling/en-passant, counter-independent legality, histories and all public contracts. No second legality system, unsafe representation or new supervision/runtime.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Native existence queries stop candidate generation at the first checked legal witness and full filtered queries avoid redundant intermediate vectors, using one shared existing candidate/resolver authority with preserved public ordering and semantics.
- [ ] #2 Independent candidate/special-move/perft fixtures, exhaustive or seeded action/undo properties and search-access/history/claim comparisons pass, including deterministic early-stop coverage within ordinary/promotion/ray/special phases and negative cases.
- [ ] #3 Level1 broadened local native/search verification, bounded representative exact-result cost comparisons and independent review support normal integration; report actual performance and limitations separately from strength, then continue evaluation work before adaptive reassessment.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Root owns codex/rewrite after accepted MSH148 atab81609; no other Chess worker. Level1 isolated native query optimization, broaden local rules/search evidence but no platform-sensitive change. Replace eager candidate-vector traversal with one private ControlFlow visitor shared by existing public collection APIs and native legal query filtering/existence, preserving exact traversal order and unchanged permits_candidate/counter normalization. Delegate only generate.rs/legal.rs and focused visitor tests. Verify independent ordinary/special sets, exhaustive action admission, perft/divides, seeded sequences and search history/claims. Freeze before/after binaries and compare identical fixed-work results/counters with bounded sequential timing; independent source and evidence review before normal integration. No rule changes, extra engine protocol, match campaign or fullplatformCI.
<!-- SECTION:PLAN:END -->
