---
id: MSH-62
title: Refine M4 shogi-first implementation and independent verification plan
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:23:00Z'
updated_date: '2026-10-26T08:00:00Z'
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
- [ ] #6 Later-game sourced/unresolved inventory, chess-document compatibility, independent planning review, documentation and Backlog/DAG validation, normal integration and push are recorded. M2/M3/completed history/persistence/roadmap remain preserved; no implementation or M4 delivery closure occurs.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [x] #2 Applicable independent review is recorded and material findings are resolved.
- [x] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded rewrite branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Inspect actual engine/session/search/action/side/analysis/archive/UI chess coupling at closed M3 source6f29713. Research primary rules authorities and editions, pin/audit an independent shogi reference candidate, and establish a fixture inventory and later-game source gaps. Write concrete minimal chess+shogi boundary proposals and coherent versioned shogi profile options. Use Backlog CLI to create traceable shogi child tasks and acyclic early shared consumers under the preserved41/46 scopes; retain final complete-delivery/closure gates. Validate original AC preservation, ownership/coverage and DAG/readiness, commission fresh independent planning review, resolve findings, integrate and push documentation/task changes normally; no game code or M4 game delivery completion.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owned on codex/m4-planning in reused .claude/worktrees/next-task; baseline closed-M3 integration6f297137, builder2b9849c product sources identical with unrelated legacy workspace exclusion qualified. Wrote current-code implementation map, named unapproved CSA36 profile proposal with one SHOGI-P1 owner bundle, exact pinned cshogi/YaneuraOu source-operation audits and55-ID fixture inventory, bounded later-game primary-source/gap inventory, complete MSH41/46 original-criteria traceability and native future tasks63-89. Original41-47 criteria/DoD/statuses intact; only41-46 deps/docs/append-only notes refined. Fresh m4_plan_review PASS architecture/ownership/DAG and m4_rules_review PASS primary clauses/reference semantics/fixtures, no material findings; minor counter-fixture wording aligned. Structured CLI validation89tasks/238edges/all61 original protected fields/acyclicresolvablegraph/earlyshared nofinalcycles/futurenoactiveownerorplans passed; doctor and links/diff checks passed. No product code, game implementation, runtime reference/opponent run or M3 audit repeated. Planning docs-only normal integration will use documented CI skip marker consistent with requested M3 evidence reuse; no new nativeCI pass claimed. SHOGI-P1 remains unapproved, MSH63ToDo;64/65 are first future implementation candidates afterplanning;67 waits63. M4 delivery/closure41-47 stayopen.
<!-- SECTION:NOTES:END -->
