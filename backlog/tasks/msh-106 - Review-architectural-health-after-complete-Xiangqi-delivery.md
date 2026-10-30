---
id: MSH-106
title: Review architectural health after complete Xiangqi delivery
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T20:26:00Z'
updated_date: '2026-10-30T03:20:00Z'
labels:
  - architecture-review
  - planning
milestone: m-4
dependencies:
  - MSH-45
documentation:
  - docs/architecture-health-roadmap.md
  - docs/design.md
  - docs/review/architecture-health-106.md
type: task
ordinal: 106000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Complete Xiangqi delivery adds a third real game across rules, records, engine, console and desktop. Diagnose accumulated architectural drift against actual Chess, Standard Shogi and Xiangqi consumers after MSH-45 is complete; board rules, routing95 or review103 alone do not establish this trigger. Do not modify the active Xiangqi implementation queue. This review does not gate unrelated earlier Tori/Chu/Tenjiku work.

Future diagnostic work only. Intent, scope, acceptance and verification expectations are recorded here; the reviewer researches the then-current implementation and records a plan when activating this task. See docs/architecture-health-roadmap.md and docs/design.md.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Compare all delivered Chess, Standard Shogi and Xiangqi paths, including square/intersection presentation and differing history/adjudication semantics; preserve the completed MSH-45 evidence. Feed the later all-six M4 architectural checkpoint without reopening complete game delivery.
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
Diagnostic review at exact integration2f17510e4c5b8c3b92a051adfa070b4d5d834b21. Root Codex owns durable records/integration in the primary codex/rewrite checkout. 1) Map actual Cargo/native rules, identity/history/replay, search/execution/protocol and product UI/session/archive responsibilities for all three delivered games. 2) Use bounded read-only research delegates for native and desktop/lifecycle comparison while root assesses engine/dependency structure; inspect source rather than trust summaries. 3) Evaluate genuine shared semantics and intentional differences; classify source-grounded findings A-D with benefits, risks, timing and concrete proposed bounded follow-ups, retaining owner decisions and no implementation authorization. 4) Produce exact-source architecture report, source-linked evidence and accepted-area coverage; obtain fresh independent review of map/findings/missed duplication/premature abstractions. 5) Run proportionate static/link/task/graph checks, reuse unchanged execution/GUI/CI evidence, finalize106 alone and normally commit/push; no refactors, later variant or M5 work.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup2026-10-30: clean primary codex/rewrite at accepted MSH45 integration2f17510, origin fetched/aligned0/0. MSH45Done, sole direct dependency satisfied; no InProgress/InReview owner. Other worktrees preserved. Global Backlog1.55.0 not used for mutations; pinned1.55.1 and overview/execution guides read. Review-only work needs no new checkout. User attached request is authoritative and explicitly permits independent review delegation; production changes/refactors not in scope.

Diagnostic report ready2026-10-30 at docs/review/architecture-health-106.md, exactsource2f17510e4c5b8c3b92a051adfa070b4d5d834b21. Actual native/engine/console/desktop/session/archive maps compare three delivered games. Read-only research arch106_native/arch106_desktop, then fresh independent arch106_independent inspected complete report/source/request/roadmap and missed-duplication/premature-abstraction risks; APPROVED, no material disputes. Editorial proposal-label ambiguity, bridge worker/run symbol and routine-metadata/owner-decision wording resolved/rechecked. Recommended A: narrow native-choice interaction lifetime and single productfrontend session owner; B: runtime installation seam, typed capacity facts, private protocol mailbox, real future complete-action handoff; C: measure transactions/replay/snapshots/search before optimizing; D: stale comments/intentional native dispatch/codecs/rendering differences. ProposalsP1-8 are report-only, not new tasks, approvals or prerequisites. Native semantic types, archiveChessv1/v3 vs taggedv2, exactUnknown/history/claims and saveability are retained. No production changes/tests/builds/GUI/benchmarks made. Offline Cargo metadata/normalized73line mailbox comparison and source checks executed; prior delivery/test/GUI evidence reused with qualifications. Existing2f native40994147428 all3PASS; Desktop40994147366 Windows/macOSPASS, UbuntuFAIL at actual Chess/Shogi GUI verification-standard missing .square e2 selector (gui-smoke.py1221). Cause untriaged, distinct from confirmed111driver diagnosis and112runner race. Exact metadata/excerpt/logSHA in report evidence; earlier c80 all3native+desktop success preserved, not substituted for laterfailure. Separate V01/P9 investigation proposal does not start or reopen any task. Independent reviewer rechecked CIaddendum/rawlog hash and approved it with acceptance retained. Final graph/docs/preservation/CLI checks and integration remain primary integrator responsibility.

Final diagnostic acceptance2026-10-30: full architecture map/report plus CIevidence meet all8AC and DoD. Source2f17510 remains fixed; no production/fixture/profile/game-delivery evidence changed. Final independent arch106_independent source/report review and later V01addendum both APPROVE, no material disagreements.24localfile/line links and evidenceJSON validated; full114taskgraph unique/resolved/no missing,self,duplicateedge,cycles; doctor/diff pass; historicalroadmap verbatim and107-109 gates unchanged. No new tasks created: P1-P8 architecture and P9Linuxverification proposals require ownerprioritization; no automatically approved refactor. Native CI all3PASS; existingDesktopWindows/macOSPASS, Linux verification-standard selector FAIL with unknowncause retained. New commands were staticanalysis/metadata/docs/graph only; no producttests/builds/GUI/benchmarks. Current production remains accepted; openverification V01 and existing112 are not hidden or falsely called fixed. Normal codex/rewrite integration/push carries this report/task completion. STOP106; M4/46/47 open,107/108/109 not started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
DONE — diagnostic architectural review at exact2f17510e4c5b8c3b92a051adfa070b4d5d834b21, main codex/rewrite. Source-grounded report docs/review/architecture-health-106.md maps all three delivered games across state/actions/candidates/attacks/legal/applyundo/history/identity/records/perft, evaluation/search, tagged dispatch, supervised transport, console/matches, desktop interaction, session/archive/capabilities and actual dependencies. Overall native authority/strict replay/complete context/sharedshell/saveability boundaries are coherent; no new rules/semantic correctness blocker or universal rules/search abstraction justified. A priorities: narrow native-choice interaction lifecycle and one productfrontend session owner. B: runtime/install seam without merging archivecodecs, typedcapacityfacts, private protocolmailbox, real nextvariant complete-action handoff. C: measured history/replay/snapshot/search costs before tuning. D: intentional native types/rendering/schema differences and small stalecomment cleanup. P1-8 are concrete unimplemented reportproposals with scope/deps/timing/risks/verification; no newBacklogtasks or gates. Ownerpriority needed, significantpublicsemantic/ownership changes require concrete establisheddecision; routineinternal choices remain autonomous after assignment. Independent readonly research native+desktop and fresh arch106_independent fullmap/source/missedduplication/prematureabstraction review approved; editorialfindings resolved and CIaddendum reapproved. Static offlineCargometadata, actual73line workercomparison,24links/JSON,114taskgraph/doctor/diff/preservation PASS; no new producttests/builds/GUI/performance campaigns. Existing exactsource Rust40994147428 all3PASS; Desktop40994147366 macOS/WindowsPASS but Ubuntu GUIverification-standard missinge2selector FAIL, causeunknown. Exactmetadata/logSHA/excerpt retained; separateP9triage proposed before relying on affectedfinalplatformevidence, not assumed111reset/112race.111Done/historypreserved;112ToDo. Prior green source-equivalentCI/game evidence retained without replacing laterfailure or claiming XiangqiGUI onWindows/Linux. Completed45/41/75/M2/M3/61, acceptedprofiles and sixgames preserved. Normal commit/integration/push oncodex/rewrite; stopafter106, no refactor/variant/M5work begun. M4/46/47 remainopen.
<!-- SECTION:FINAL_SUMMARY:END -->
