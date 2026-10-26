---
id: MSH-62
title: Refine M4 shogi-first implementation and independent verification plan
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:23:00Z'
updated_date: '2026-10-26T08:04:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
documentation:
  - docs/m4/README.md
type: spike
ordinal: 62000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
M3 is closed. The next batch must turn the complete M4 scopes into executable shogi-first delivery and shared-boundary tasks without implementing a game or closing a delivery record. Preserve MSH41-47, their original acceptance obligations, M2/M3 and the MSH61 admission fix and MSH40 roadmap correction. Standard shogi first; provisional later order xiangqi, tori, chu, tenjiku, with independent rules research permitted. Owner decisions are limited to concrete unresolved material profile choices.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 An M4 implementation map cites current code boundaries and proposes the smallest concrete chess-plus-standard-shogi extensions for requests/results, sessions/search, complete actions, side/orientation, analysis, records and presentation while preserving authoritative rules/history, stale protection and lossless capacity admission.
- [x] #2 A named/versioned standard-shogi proposal cites primary authorities and editions and covers complete rules and game/match conclusions, distinguishes legal rejection/tournament fouls/replays/scoring, and presents only actual material owner choices with sourced options and a recommendation.
- [x] #3 Independent verification strategy and initial fixture inventory distinguish rules authorities, expected cases, a pinned reference with exact comparison-operation/semantic audit, differential tools and opponent engines; complete legal choices and known semantic gaps are explicit.
- [x] #4 Native Backlog tasks fully trace MSH41 obligations through rules/state/actions/reversal/history/adjudication, notation/console/records, functional search/interoperability, desktop/persistence and game-specific integration review without transferring M4 obligations to M5.
- [x] #5 MSH46 has acyclic consumer-driven early shared tickets and later extension/final acceptance tickets with single ownership; game deliveries depend on required shared work without depending on their own final integration. MSH41-47 original identities and acceptance obligations stay open and intact.
- [x] #6 Later-game sourced/unresolved inventory, chess-document compatibility, independent planning review, documentation and Backlog/DAG validation, normal integration and push are recorded. M2/M3/completed history/persistence/roadmap remain preserved; no implementation or M4 delivery closure occurs.
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
Inspect actual engine/session/search/action/side/analysis/archive/UI chess coupling at closed M3 source6f29713. Research primary rules authorities and editions, pin/audit an independent shogi reference candidate, and establish a fixture inventory and later-game source gaps. Write concrete minimal chess+shogi boundary proposals and coherent versioned shogi profile options. Use Backlog CLI to create traceable shogi child tasks and acyclic early shared consumers under the preserved41/46 scopes; retain final complete-delivery/closure gates. Validate original AC preservation, ownership/coverage and DAG/readiness, commission fresh independent planning review, resolve findings, integrate and push documentation/task changes normally; no game code or M4 game delivery completion.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owned on codex/m4-planning in reused .claude/worktrees/next-task; baseline closed-M3 integration6f297137, builder2b9849c product sources identical with unrelated legacy workspace exclusion qualified. Wrote current-code implementation map, named unapproved CSA36 profile proposal with one SHOGI-P1 owner bundle, exact pinned cshogi/YaneuraOu source-operation audits and55-ID fixture inventory, bounded later-game primary-source/gap inventory, complete MSH41/46 original-criteria traceability and native future tasks63-89. Original41-47 criteria/DoD/statuses intact; only41-46 deps/docs/append-only notes refined. Fresh m4_plan_review PASS architecture/ownership/DAG and m4_rules_review PASS primary clauses/reference semantics/fixtures, no material findings; minor counter-fixture wording aligned. Structured CLI validation89tasks/238edges/all61 original protected fields/acyclicresolvablegraph/earlyshared nofinalcycles/futurenoactiveownerorplans passed; doctor and links/diff checks passed. No product code, game implementation, runtime reference/opponent run or M3 audit repeated. Planning docs-only normal integration will use documented CI skip marker consistent with requested M3 evidence reuse; no new nativeCI pass claimed. SHOGI-P1 remains unapproved, MSH63ToDo;64/65 are first future implementation candidates afterplanning;67 waits63. M4 delivery/closure41-47 stayopen.

Normal planning integration/push complete on codex/rewrite at0f2e7d6f761bcb8e0052951820b0ea1f05af8334 (builder16fce28). Re-ran structured validation and doctor in the actual integrated checkout:89tasks/238edges/all61 original protected records unchanged, complete DAG resolves/acyclic, 27futuretasks unassigned with no detailed plans,55fixtureIDs/local links/diff checks pass. Scoped native/frontend/host/scripts/workflow and build-manifest diff against closed-M3 integration6f297137 is empty; unrelatedprototypecleanup preserved. Both fresh independent reports PASS with no material findings; smallSH-F041 import-counter wording alignment applied. Planning-only CI skip explicitly recorded; gh run list for integration returns no native/application run, and no newCI pass is claimed. Closed M3 evidence reused per owner. All six planning acceptance criteria satisfied. SHOGI-P1/MSH63 still unapprovedToDo; no game implementation, reference runtime build, opponent match or M4 delivery/closure started. First future implementation64/65 then66; affected67 waits63. Final completion is task bookkeeping only.

Integrated structured validation:
```json
{
  "status": "passed",
  "baseline": "6f29713795804eed3605114d90fed37aa5f554a0",
  "backlog_version": "1.55.1",
  "tasks": 89,
  "edges": 238,
  "original_tasks_checked": 61,
  "original_criteria_sha256": "88609add8479db0d212dc2819c5bb16552dadffbb61e1425602fdb849eb26d3f",
  "fixture_ids": 55,
  "local_links_checked": 16,
  "original_41_47_criteria_unchanged": true,
  "completed_history_statuses_preserved": true,
  "full_graph_acyclic": true,
  "all_dependencies_resolve": true,
  "early_shared_no_final_delivery_prerequisite": true,
  "later_research_independent": true,
  "no_future_implementation_plans_or_active_owners": true,
  "native_product_diff_from_closed_m3": [],
  "first_implementation_candidates_after_62": [
    "MSH-64",
    "MSH-65"
  ],
  "owner_choice_gate": "MSH-63 \u2192 MSH-67"
}
```
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the bounded M4 planning batch, not game implementation. Normal integration/push is codex/rewrite0f2e7d6f761bcb8e0052951820b0ea1f05af8334 with documentation/task-only builder16fce28. M2/M3, all completed history, MSH61 lossless persistence admission, MSH26 and MSH40 roadmap correction remain intact; MSH41-47 stayToDo with every original AC/DoD unchanged.

Delivered docs/m4/README.md and current-code map, complete named/versioned proposed shogi profile, primary source editions/clauses and one current owner decision SHOGI-P1, exact immutable cshogi/YaneuraOu operation/source audits,55-ID independent fixture inventory, and bounded later-game source/unresolved-rule inventory. Recommendation is mshogi.shogi.csa36.v1: JSA board rules plus explicitly CSA36 game conclusions and untimed software adaptations; it is NOT approved by planning completion. JSA professional/other event profiles remain distinct alternatives. Reference builds/matches/conformance tests are future evidence, not claimed executed.

Created native future tasks63-89. Shogi64-75 cover complete game components; shared69/72/73 become prerequisites before aggregate41/46 completion; later76-79 consume game-owned84-87 after independent profile research80-83; final88/89 preserve shared packaged acceptance and review. Original41/46 criteria map completely to bounded children and original47 remains fullM4closure. Single ownership and acyclic dependencies preserve real consumers and no speculative universal framework.

Backlog1.55.1 doctor, 89-task/238-edge structured preservation/DAG/readiness checks, original61 protected fields and criteriahash,55fixture IDs, local links and whitespace/source-diff checks passed. Fresh independent architecture/task and primary-rules/reference/fixture reviews passed with no material findings. Native/application M3 evidence reused; planning-only documentedCI skip produced no new nativeCI run or pass claim.

Stopped after planning. Only current owner choice is SHOGI-P1 via63; first bounded future implementation is64/65 then66, while affected history/outcomes67 waits63. Provisional later order remainsxiangqi,tori,chu,tenjiku, with independent research possible. All M4 game deliveries/shared final acceptance/M4closure remain open; M5/M6 obligations unchanged.
<!-- SECTION:FINAL_SUMMARY:END -->
