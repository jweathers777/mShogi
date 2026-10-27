---
id: MSH-94
title: Implement selected xiangqi history adjudication and canonical native records
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-27T19:43:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-80
  - MSH-93
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
  - docs/m4/xiangqi/coverage.md
  - docs/m4/xiangqi/owner-decisions.md
  - docs/m4/xiangqi/history-contract.md
  - docs/m4/xiangqi/history-evidence.md
ordinal: 94000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Implement the ratified complete history/event policy with exact known prefixes and persistent relevant piece identity. Cover all applicable WXF19/20 and chapter5 classification/exception/precedence families, repetition warnings/refusal, natural-count claimant-specific checks/capture boundaries and native terminal commands; resolve R-NOWIN and any required semantic decisions before affected implementation.
- [x] #2 Keep advisory opportunities, committed claims, warnings, refusal, board conclusions, resignation/agreement, unresolved unscored assessments, illegal requests and resource/process failures distinct. Native adapters own actor/turn admission; terminal events may preserve board ply. Missing pre-root history cannot supply inferred occurrences/checks.
- [x] #3 Canonical game-owned records preserve immutable profile/edition/schema, root provenance/declared counters, all complete actions/commands/events/policies and exact replay/undo semantics. Strict bounded parsing rejects duplicate/unknown/missing/conflicting identities atomically; saved/repetition/search identities are distinct.
- [x] #4 Run independently reviewed legal history fixtures including same board/different history, published diagram boundaries, alternating versus simultaneous threats, roots/pins/victims, natural-count10/11/100 and neither-can-win proof/unknown cases. Retain exact replay/reversal and independent review evidence; no reference-engine outcome shortcut or required rule deferred to M5.
- [x] #5 Before affected history/recognizer implementation, resolve G-CYCLE witness/anchor/overlap/responsibility fixtures and G-NOWIN verification-goal/domain source cases or obtain an explicit owner decision on any material residual convention. Third-occurrence warning and continued fourth-occurrence loss/otherwise applicable WXF draw are fixed XIANGQI-P1. Fast sound recognition and honest ternary assessment are owned here; full exact verification is separately required by105, and84/103/45 cannot close without it.
- [x] #6 Manual natural-count claim validates claimant-specific authoritative history, rejects invalid requests with semantic state unchanged and retained bounded request/rejection audit, and applies documented serialized conclusion ordering. Optional auto_valid_claims is named/recorded; no untimed tournament penalty or shared-global check subtraction. Exact replay/save admission includes audit/proof modes and limits.
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
Root@codex is the sole builder/integrator in primary codex/rewrite at067d1767d06fa0e781aff2a325e4b21a9ecf1d75; startup clean, no other active task ownership or disposable worktrees. 1.Reconcile governing bilingual WXF19/20/Art8/3.2A source and independently instantiate/review conduct, cycle-overlap and proof-goal fixtures before affected adjudication is accepted; separate source workers own disjoint research only. 2.Build unaffected native complete history/event/actor/persistent-piece identity foundations, claimant-specific known-history count commands, atomic bounded rejection audit, terminal commands/precedence, exact undo and strict replay-validated native canonical records. Preserve the existing board API and never turn unresolved classification into a draw/loss or board illegality. 3.Implement only source/fixture-resolved conduct and sound assessment behavior; if a material source ambiguity remains, retain concrete histories/options and continue unaffected work while requesting that narrow owner decision. 4.Focused native/replay/strict parsing/limit fixtures, separate completed-boundary review, relevant regressions and normal native CI, integrate/push completed scope; do not mark Done unless all criteria are supported. MSH105 is not ready: direct dependencies MSH94 and MSH95 are both To Do at startup; do not start it or change its graph. No later console/engine/protocol/archive/desktop Xiangqi implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
XIANGQI-P1 owner-refinement2026-10-27: original4AC/DoD preserved, complete native contract remains required; R-NOWIN now G-NOWIN before affected native recognition/proof and R-CYCLE now G-CYCLE before classification.94 owns history/event/canonical contracts, sound fast recognition and explicit unknown/proven statuses;105 owns full bounded exact backend after94/95. This is explicit staged allocation of the computational obligation, not silent scope removal:84,103,45 wait105. Strict native replay handles modes/limits honestly and refuses unsupported restoration before actual backend exists. Rejected claim audit is distinct from semantic game mutation and subject to61/99 admission.91/92 do not depend on these later cases.

Owner-authorized next batch inspected. MSH94 dependencies80/93 are Done. MSH105 dependencies94/95 are not satisfied, so the requested fallback applies: work only on94 and report those exact dependencies. Backlog PATH1.55.0 is read-only for this session; pinned1.55.1 mutation binary matches AGENTS baseline. Source research workers: /root/xq_cycle_sources and /root/xq_conduct_sources; root retains product files and durable planning integration. G-CYCLE/G-NOWIN gate affected semantics, not ordinary representation choices or unaffected history/record work.

Owner explicitly resolved G-NOWIN during this task (option2): WXF3.2A is existential natural-course board-win capability under3.1A: checkmate or no-legal-move/stalemate loss only. Exclude repetition/prohibited-conduct awards, resignation/agreement, time/lateness, technical/admin/misconduct/invalid-claim penalties and recursive no-win draw. Full known history remains authoritative for actual match outcomes but those mechanisms cannot create a positive natural-win witness. F002 C^3 must not prove Black natural capability. This supersedes research workers initial recommendation to include repetition-fault wins; preserve that as rejected historical proposal. MSH105 remains unstarted behind94/95. Root owns game.rs/record.rs/assessment.rs; isolated conduct.rs delegate /root/xq_conduct_sources; fixture integration test delegate /root/xq_cycle_sources; separate /root/xq_history_reviewer source and boundary review.

Completed native history/adjudication/canonical boundary. Durable contract and evidence: docs/m4/xiangqi/history-contract.md and history-evidence.md; source/conduct/cycle and separate completed-boundary reviews retained under history-reviews/. Independent reviewer /root/xq_history_reviewer actually ran8 conduct +15 history +9 boundary tests and resolved all material findings (warning retirement, pending exemption, undo/runtime identity, source-correct offer admission). Final production hashes match reviewed files. Local final fmt/workspace clippy pass; cargo test --workspace --locked462passed0failed2existing ignored deep-perft; isolated desktop-session all-features92passed, native fixture regeneration unchanged, workspace build pass. Final-source board-only admitted-reference refresh and normal native CI/integration still pending; no check is claimed before execution. All claim audits and actual encoded canonical limits are admitted before commit with reserved closure. MSH105 remains unstarted, dependencies94/95 unchanged; no subsequent consumer work begun.

Final-source native board probe rebuilt with cargo; reproducible collector PASS17histories/492native prefixes/492admitted-reference comparisons/0native-only. Compact provenance+fixture/source/binary hashes retained in docs/m4/xiangqi/history-reviews/board-observation-summary.json; WXF outcomes remain independently expected, never comparator-produced. Working-tree source hashes match the separate reviewed production hashes. Source/fixture gates resolved before adjudication acceptance. Review status retained until integration and normal native CI complete.

Integration a822450d1618810e0ef37efb9ef05035939c5746 committed and normally pushed to codex/rewrite. Required Rust workspace CI https://github.com/jweathers777/mShogi/actions/runs/40569025619 PASS on Windows2022/macOS15/Ubuntu24.04. Actual workspace results: Windows459passed0failed2ignored, macOS/Linux462passed0failed2ignored; isolated desktop QA92passed on each. Format/clippy/build/native frontend fixture compatibility/console smoke all PASS. Automatic Desktop packages CI https://github.com/jweathers777/mShogi/actions/runs/40569025438 also PASS all3: locked frontend/native checks, normal package inspection/bundled engines; actual existing Chess/Shogi packaged GUI regressions on Linux/Windows. macOS result is package/engine evidence, not a new GUI campaign; unchanged earlier macOS native interaction/termination/Tao evidence reused. Final independent documentary PASS (minor terminology corrections applied) retained in history-reviews/completed-boundary-review.md. No production changes after reviewed hashes. Every AC mapped in history-evidence.md.105 remains unstarted with94/95 dependencies unchanged; after94 completion only95 is unmet. No feasibility measurement or full exact backend is claimed. MSH45/M4 remain open; no later product consumer started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented the selected mshogi.xiangqi.wxf2018.v1 native Game/GameRequest/GameRecord boundary: persistent piece identity and exact known/event prefixes; source-resolved WXF check/chase responsibility, third-occurrence anchor warnings, proved retirement and continuing fourth-occurrence loss/otherwise draw; claimant-specific100-ply natural count with checks above10 deducted, bounded invalid-claim audit, optional/recorded automatic policy; mate/stalemate, resignation/agreement and serialized no-overwrite conclusions. Strict RawValue-preserving native canonical replay validates every command/event/derived field, preserves provenance/modes/limits, supports exact LIFO semantic undo and fresh non-reviving runtime identities. Candidate encoded byte/event/command admission reserves complete native closure/saveability.

G-CYCLE/R-CLASS independent source/fixture gates resolved. Owner G-NOWIN selects existential natural board win (WXF3.1A mate/stalemate only), excluding repetition penalties and other non-board awards. Sound bare-General proof and genuine0/1ply positive witnesses are implemented; misses remain UnknownExactUnavailable. Required exact backend/feasibility remains MSH105, unstarted behind MSH95 after this completion.

Evidence: separate bilingual pixel/source/fixture and completed-code/documentary review PASS with material findings resolved;32 focused tests; local workspace462pass/0fail/2existing ignored; final-source492 paired board prefixes over17histories against admitted pinned Pikafish (never an outcome oracle); native and desktop package CI both PASS all3 target systems at a822450d1618810e0ef37efb9ef05035939c5746. Contract/evidence/reviews retained under docs/m4/xiangqi/history*. No Xiangqi console, search, engine, protocol, archive or desktop implementation added. Next dependency-ready boundary is MSH95; then105 after95. MSH45 and M4 stay open.
<!-- SECTION:FINAL_SUMMARY:END -->
