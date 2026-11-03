---
id: MSH-107
title: Review architectural health after complete Chu Shogi delivery
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T20:26:00Z'
updated_date: '2026-11-03T16:19:00Z'
labels:
  - architecture-review
  - planning
milestone: m-4
dependencies:
  - MSH-43
documentation:
  - docs/architecture-health-roadmap.md
  - docs/design.md
  - docs/review/architecture-health-107.md
type: task
ordinal: 107000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Complete Chu Shogi delivery supplies real compound actions, royal semantics and a large-board product consumer. Diagnose accumulated architectural drift after the entire MSH-43 delivery is complete, comparing every then-delivered game. A synthetic compound fixture or native handoff86 alone does not establish the trigger. This checkpoint does not gate unrelated earlier Tenjiku or other game implementation.

Future diagnostic work only. Intent, scope, acceptance and verification expectations are recorded here; the reviewer researches the then-current implementation and records a plan when activating this task. See docs/architecture-health-roadmap.md and docs/design.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Compare actual Chu compound/route and royal semantics across native rules, search, records, transport and desktop with every then-delivered game. Account for the post-Xiangqi review when available, without requiring Chu to share Chess semantics or filenames. Feed the all-six M4 architectural checkpoint.
- [x] #2 Publish a current architecture map at an exact source revision identifying native position/state ownership, complete-action representation, candidate generation, attack/movement queries, legality, apply/undo, history and adjudication, canonical records/codecs, perft/diagnostics, evaluator/search boundaries, shared game/profile dispatch, transport/session/archive abstractions, capability discovery, desktop presentation boundaries, frontend/native rules separation and dependency structure. Map responsibilities to actual source symbols and real consumers; explicitly distinguish delivered, unavailable and unchanged coverage.
- [x] #3 Assess duplicated logic, game-specific branching in shared code and verification consistency across games using concrete source/examples. Examine both duplicated implementations whose real consumers demonstrate stable shared semantics and premature abstractions that force different game semantics through misleading common models. Apply: "Generalize when multiple real consumers demonstrate the same semantics, not merely similar-looking code."
- [x] #4 Judge whether responsibilities are explicit, coherent and independently testable, without requiring identical file layouts. No game needs generate.rs, attack.rs or another filename merely because Chess has one. Very large modules, unclear responsibility boundaries, repeated game-identity conditionals, awkward dependency direction, duplicate serialization and tests difficult to isolate are investigation indicators, not automatic defects.
- [x] #5 Produce concrete findings tied to source/examples with risk, severity, evidence, affected consumers and rationale; explicitly record areas reviewed and found acceptable. Distinguish rule correctness, search correctness, execution performance, playing strength and application integration; preserve all six games, accepted profiles/contracts and completed evidence.
- [x] #6 For each recommended change, if any, record expected benefit, affected consumers, regression risks and suggested bounded follow-up tickets with focused differential/regression verification expectations. Record decision/disposition and any genuine blocker separately from advisory recommendations. "No material architectural changes recommended" is a valid evidence-supported outcome.
- [x] #7 Keep this task diagnostic: completing the review neither requires nor authorizes a refactor. Perform no substantial restructuring here; only a very small correction explicitly required by an existing acceptance criterion may be in scope. Any material refactor becomes a separate bounded task preserving behavior with focused differential/regression evidence, and any product/rules/contract change requires the established owner decision.
- [x] #8 Record the report, actual commands/outcomes/environment/source revision and proportionate independent assessment of the findings and coverage. Reuse valid existing evidence with precise references; identify checks not run and unavailable evidence honestly. Complete the full review coverage and record unresolved findings without turning every recommendation into a mandatory refactor.
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
1. Map current five-game production ownership and real consumers against design, MSH-106/115 and accepted R1–R6 records at a8581f09107648d03e3016786b41c267ace88bca.
2. Trace Chu integration stress, shared execution/document/frontend boundaries and documented prospective Tenjiku constraints; distinguish demonstrated defects from measured costs and unmeasured risks.
3. Publish source-anchored architecture map and prioritized diagnostic report with bounded recommendations, acceptable areas, verification and MSH-108 handoff.
4. Obtain independent read-only assessment, resolve report errors, evaluate all eight AC and DoD, then integrate and push documentation/evidence only to codex/rewrite. Stop after MSH-107.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup: clean primary codex/rewrite at a8581f09107648d03e3016786b41c267ace88bca, origin git@github.com:jweathers777/mShogi.git; no active In Progress/In Review owners found. Existing three unrelated worktrees preserved. Owner @codex in primary checkout; diagnostic documentation needs no temporary worktree. Global CLI1.55.0 read only; pinned /private/tmp/mshogi-bootstrap-npm-cache/_npx/749414256ca1c44f/node_modules/backlog.md-darwin-arm64/backlog1.55.1 verified and selected for writes.

Draft map/report published for independent assessment. Source-backed findings F01–F04 are advisory performance/latency/maintenance risks; no proven rule/saveability defect or architectural Tenjiku blocker. Focused checks at unchanged a8581f0: core3, desktop runtime8 + compound observation1 + deadline2, frontend45 all passed. Offline metadata includes separate Tauri workspace. Required new CI jobs none for documentation-only diagnostic; accepted exact-source product/CI evidence reused with platform limits. Fresh independent reviewer requests installation-owner pointer, qualified retention wording and explicit conformance map; these were corrected. Final evidence/review reconciliation pending.

Final independent read-only assessment by /root/independent_review APPROVED: all source/report corrections resolved. Reviewer independently inspected actual source/consumers, read three logs, verified four evidence size/SHA256 entries and59-test total; no reviewer product tests or GUI claimed. Report distinguishes measured existing elapsed costs from unmeasured allocation/cancellation/inspection risks; no correctness/saveability defect, serious architecture blocker or material redesign recommendation. Structured verification record docs/review/evidence/msh107/verification.json; original8AC coverage table in report. MSH108 handoff added without status/owner/dependency/criteria/plan changes; no follow-up tasks created. Pinned doctor and23 local links pass; production/manifests/workflows/accepted evidence remain unchanged. Origin fetched and0/0 at a8581f0 before integration. Ready dependency-satisfied queue empty; next recommended assignment83 needs separate refinement/authorization. Normal diagnostic integration/push and terminal bookkeeping remain.

Integrated and pushed reviewed diagnostic report/evidence/handoff as8632b7ecf7bba10df545e17fb9366b4a52d295e2 on codex/rewrite; fetched origin and push confirmed. Exact tested production remains a8581f0; commit contains only nine diagnostic/task/evidence files. Independent final readback retained APPROVAL, verified all normalized hashes and reconstructed original stdout hashes by restoring one trailing newline; no test-output content changed. All8AC/fourDoD supported by report coverage matrix and structured evidence. Ready dependency-satisfied queue remains empty; no next work activated. Completion bookkeeping commit follows with no production change.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the five-game post-Chu architecture diagnostic at a8581f09107648d03e3016786b41c267ace88bca. Published source-symbol/consumer map, dependency/conformance boundaries, Chu integration stress, Tenjiku readiness and prioritized F01–F04 in docs/review/architecture-health-107.md, with one structured verification record. Verdict: coherent R1–R6 ownership survives Tori/Chu; no material redesign, new correctness/saveability defect or demonstrated architectural Tenjiku blocker. Moderate risks: retained full undo/repeated reconstruction, cooperative replay/response-parse cancellation granularity, synchronous native inspection and complete-choice costs. Low advisory: identical syntax helpers, native projection duplication and stale capability comments. P1–P4 are bounded proposals with benefits/regressions, not implemented refactors or new gates. Native rules/actions/history/outcomes, current envelope/authority, shared shell/frontend/choice owner, profiles/resource policies and all six promised games preserved. All59 focused checks pass; metadata/static links/Backlog doctor pass. Fresh independent read-only reviewer approved source/report/coverage and evidence after corrections; existing exact-source platform evidence reused without a broad new CI/GUI or strength claim. MSH108 handoff updated while Ready/unassigned and original dependencies/criteria/plan preserved. Report integrated/pushed8632b7ecf7bba10df545e17fb9366b4a52d295e2 on codex/rewrite. No implementation/follow-up task started. No currently dependency-ready Ready task; recommend separate MSH83 complete modern-profile reconciliation assignment, then existing Tenjiku chain. STOP after107.
<!-- SECTION:FINAL_SUMMARY:END -->
