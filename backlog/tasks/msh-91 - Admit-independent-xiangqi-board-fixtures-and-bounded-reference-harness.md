---
id: MSH-91
title: Admit independent xiangqi board fixtures and bounded reference harness
status: Ready
assignee: []
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T16:30:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-80
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/owner-decisions.md
  - docs/m4/xiangqi/coverage.md
ordinal: 91000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Freeze and independently review rule-derived complete board/action fixture expectations before differential runs, including both sides, full initial44 action set, horse legs, elephant eyes/river, palace, soldiers, cannon screens, facing generals, all check evasions and no-action loss. Keep history/outcome and consumer families with their named owners; no all-family inventory prerequisite for state work.
- [ ] #2 Build the audited pinned comparator in isolation and retain source/patch/toolchain/flags/binary/license provenance, exact supported board domain and separately pinned weights if required. Execute positive/negative admission sentinels and exact apply/undo checks; source inspection or a mutable network fetch does not admit a runtime reference.
- [ ] #3 Compare complete legal action sets and independently established shallow perft/divide expectations; normalize dialects explicitly. Demonstrate useful deliberate mismatch, malformed input, timeout and engine-failure reporting. Keep expected facts and reference observations separate; resolve disagreements through sources and reviewed positions.
- [ ] #4 Record independent fixture/harness review and reproducible commands/results. Neither reference outcome helpers nor software agreement establish WXF history conformance or playing strength.
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
XIANGQI-P1 selected2026-10-27. This independently settled board/state scope is unaffected by G-CYCLE/G-NOWIN later history/proof questions or missing complete AXF/CXA corpora. Ready status will be set only after80 reviewed planning integration; no execution in80 batch. Preserve44-action opening set/F002 seeds; actual reference build/admission remains91, not80.

MSH80 planning completed under XIANGQI-P1 with reviewed integrated/pushed evidence412bbe743f78cef47a931854eef3cdd39783a83b. This task is now Ready and dependency80 satisfied. No implementation/reference build/tests performed for this future ticket, no ACchecked, no assignee/implementationplan. Root stops at implementation boundary;91+92 then93 is first bounded queue. G-CYCLE/G-NOWIN remain affected94/105gates, not this scope.
<!-- SECTION:NOTES:END -->
