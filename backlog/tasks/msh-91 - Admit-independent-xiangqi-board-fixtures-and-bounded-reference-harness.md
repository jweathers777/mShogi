---
id: MSH-91
title: Admit independent xiangqi board fixtures and bounded reference harness
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T18:28:00Z'
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
- [x] #1 Freeze and independently review rule-derived complete board/action fixture expectations before differential runs, including both sides, full initial44 action set, horse legs, elephant eyes/river, palace, soldiers, cannon screens, facing generals, all check evasions and no-action loss. Keep history/outcome and consumer families with their named owners; no all-family inventory prerequisite for state work.
- [x] #2 Build the audited pinned comparator in isolation and retain source/patch/toolchain/flags/binary/license provenance, exact supported board domain and separately pinned weights if required. Execute positive/negative admission sentinels and exact apply/undo checks; source inspection or a mutable network fetch does not admit a runtime reference.
- [x] #3 Compare complete legal action sets and independently established shallow perft/divide expectations; normalize dialects explicitly. Demonstrate useful deliberate mismatch, malformed input, timeout and engine-failure reporting. Keep expected facts and reference observations separate; resolve disagreements through sources and reviewed positions.
- [x] #4 Record independent fixture/harness review and reproducible commands/results. Neither reference outcome helpers nor software agreement establish WXF history conformance or playing strength.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [x] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [x] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Root@codex is sole integrator on primarycodex/rewrite at53e90a93aa8954965cd69e336a9806ea5f59acaa; independent fixtures and restricted reference/harness work use disjoint files. 1.Freeze source-derived board cases preserving44/F002; independent expectation review before reference observations; cover bothsides/topology/captures/evasions/pins. 2.Build pinned audited Pikafish source in isolatedtmp via minimal network-free board-only harness (or auditedcompatibleFairyfallback); retain exact patch/source/compiler/commands/binary/operation/notation/provenance and exclude allhistory/outcome helpers. 3.Execute admission sentinels/fullsets/applyundo, malformed/domainnegative/timeout/failure/deliberate-mismatch diagnostics without changing expectations. 4.Independent fixture/harness review, reproducible evidence, normalintegration/push beforeDone. No94/105/console/engine/protocol/archive/desktop implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
XIANGQI-P1 selected2026-10-27. This independently settled board/state scope is unaffected by G-CYCLE/G-NOWIN later history/proof questions or missing complete AXF/CXA corpora. Ready status will be set only after80 reviewed planning integration; no execution in80 batch. Preserve44-action opening set/F002 seeds; actual reference build/admission remains91, not80.

MSH80 planning completed under XIANGQI-P1 with reviewed integrated/pushed evidence412bbe743f78cef47a931854eef3cdd39783a83b. This task is now Ready and dependency80 satisfied. No implementation/reference build/tests performed for this future ticket, no ACchecked, no assignee/implementationplan. Root stops at implementation boundary;91+92 then93 is first bounded queue. G-CYCLE/G-NOWIN remain affected94/105gates, not this scope.

Ownerauthorized batch91+92 then93 only. Startup clean primarycodex/rewrite53e90a9, one worktree, nootheractiveBacklogownership. Backlog1.55.1 pinned mutationbinary resolvesPATH1.55.0difference.80Done selectedXIANGQI-P1 governs. Root sole durabletaskowner/integrator; isolatedindependentfixture/reference contributors will be recorded; no taskDone until own evidence/review/integration. G-CYCLE/G-NOWIN stay94/105 and must not affectlocal boardlegality.

Completed independent105-fixture corpus (101 admitted,4 deliberate structural exclusions) and actually built pinned restricted Pikafish board reference. 15 admission/failure sentinels and520 sequence-prefix observations passed; independent perft2/3/7 and divide5+2 confirmed. Source-based B38 correction, foreign counter exclusion, process-group cleanup and prefix-provenance findings retained/resolved. Separate independent reviewer manually inspected rule fixtures and reran harness/corpus: final PASS. Evidence/reproduction: docs/m4/xiangqi/board-foundation-evidence.md, reference/, reviews/. No history/strength admission.

Post-integration native CI confirmation: Rust workspace run40557737936 succeeded on actual ubuntu-24.04, macos-15 and windows-2022 at16042f291bf3190ff2712c77344b2c6885d65f10. Final combined batch source a71d27b1bca2d75d789ead1acc2f8a3132dcdc81 additionally passed run40559507815 on all three systems (fmt, workspace all-target clippy, tests, isolated desktop QA constructors, unchanged frontend snapshot generation, build and existing console smoke). https://github.com/jweathers777/mShogi/actions/runs/40559507815 . No Xiangqi GUI/engine or history conformance claim; unchanged earlier product evidence remains applicable.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed and independently reviewed; integrated and pushed to codex/rewrite at 16042f291bf3190ff2712c77344b2c6885d65f10. Task-scoped acceptance evidence and exact commands: docs/m4/xiangqi/board-foundation-evidence.md; independent reports/hashes: docs/m4/xiangqi/reviews/. MSH-91 provides105 independent fixtures,101 admitted reference cases,15 sentinels/520 prefixes; MSH-92 provides native state/codecs/private transitions with12 focused tests. No full local legality claim until93, no history/adjudication/consumer implementation. Normal final native workspace CI will additionally cover integrated93; unchanged prior product/platform evidence retained.
<!-- SECTION:FINAL_SUMMARY:END -->
