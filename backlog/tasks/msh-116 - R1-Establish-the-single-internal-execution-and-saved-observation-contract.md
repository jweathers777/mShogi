---
id: MSH-116
title: 'R1: Establish the single internal execution and saved-observation contract'
status: Done
assignee:
  - '@codex'
created_date: '2026-10-30T12:06:00Z'
updated_date: '2026-10-30T15:02:00Z'
labels:
  - architecture-migration
  - planning
milestone: m-4
dependencies:
  - MSH-115
documentation:
  - docs/design.md
  - docs/architecture-migration-plan.md
  - docs/review/architecture-target-115.md
  - docs/desktop/internal-execution-v4.md
  - docs/review/execution-migration-116.md
priority: medium
type: task
ordinal: 116000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved MSH-115 migration stage; shared integration ownership supports MSH-46 without changing its existing prerequisites or closure gates. Development-era internal compatibility is not required. Preserve native selected profiles, complete history/actions/adjudication, atomicity, exact work identity, safe cancellation, current-format losslessness, admission/headroom and external interoperability. Planning creates future intent only; inspect current source and record a bounded plan on activation. One integrator owns overlapping cutovers; stop at this ticket after normal verification, independent review, integration and push.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Exactly one current tagged internal request/response family and submit/poll path serves actual Chess/Shogi/Xiangqi consumers. Remove Legacy/Shared dual transport, V2 wrapper conversion, chess_adapter and unknown-version legacy-shape fallback after all callers migrate; standard UCI/USI/Xiangqi interoperability remains unchanged.
- [x] #2 Complete submitted game/profile/schema/history, purpose, selected prefix, request/session/process identities and limits bind acceptance. Moves, claims/declarations, concluded games, incomplete/Unknown, failure and Chess Full remain native-distinct; no fabricated fallback/score/outcome or revived retired work.
- [x] #3 Durable observation structures, validators, fixtures, desktop/archive consumers and actual child entry points migrate with the transport. Observations retain exact native target/result/provenance as historical data; current-stage complete Save/Open works without preserving an old execution parser.
- [x] #4 Actual encoded request/response/diagnostic limits, bounded queues/watchdogs/cleanup, cancellation/replacement and stale-result protection remain verified through real subprocess tests, including same-position/different-history and wrong purpose/limits/identity/selection.
- [x] #5 R1 and R2 may coordinate saved-observation/codec cutover as independently reviewed subunits under one bounded integration owner if separate landing would create incomplete persistence or throwaway formats. Keep an acyclic task dependency, record exact subunit evidence, and do not mark either Done with unsupported criteria or leave a permanent adapter.
- [x] #6 Focused native/consumer regressions and proportionate independent review support all acceptance criteria at exact revisions; applicable CI and normal integration/push are recorded with passed/failed/not-run/platform distinctions. Historical evidence remains unchanged; obsolete interface-only tests are replaced by semantic tests rather than preserving removed APIs. No next stage starts automatically.
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
1. Establish one strict current execution family around a common native-tagged semantic context, with a durable historical observation type independent of transport version. Preserve complete native records, purpose/selection/perspective, identities and actual limits. 2. Replace dual wire/server/bridge paths and Chess conversion with direct native dispatch; migrate every actual engine/controller/child caller and remove obsolete V2/fallback APIs. 3. Migrate desktop observation journals, validators, Take back lineages, fixtures and current-stage archive roundtrips without starting the R2 envelope redesign; retain native Xiangqi in-process exact evidence. 4. Port independent semantic and real subprocess tests for all three games, strict parsing/context/limits, cancellation/stale work, bounded resources and current saveability; update package verifiers to the surviving contract. 5. Obtain separate contract/persistence/deletion review, resolve findings, run focused native/frontend/consumer checks and applicable CI, record exact evidence, then finalize only116 with normal integration/push.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Startup 2026-10-30: primary builder/integrator Codex owns116 in primary codex/rewrite checkout at899ee343bdba981381649457bca483e14f5f9109. Clean checkout, origin fetched aligned0/0; unrelated old worktrees preserved and no In Progress owner. Installed Backlog1.55.0 differs from project1.55.1; pinned1.55.1 CLI and overview/execution guides used before mutation. Read owner request/design/approved migration target and task;115and122Done. Actual source inspection confirms wire Legacy/Shared wrappers, bridge submit/poll pairs, SharedRequest::chess_adapter and archived EngineResponse/SharedResponse coupling. Separate read-only persistence audit delegated; no competing source builder, no117activation.

Implementation now uses one protocol4 ExecutionRequest{protocol,context}/ExecutionResponse{protocol,observation} and durable schema1 ExecutionObservation{schema_version,context,result}. Semantic ExecutionContext retains native CanonicalRecord and complete purpose/target/identities/limits; ChessResult remains domain-only. All actual callers/journals/validators/examples/fixtures migrated; wire.rs, dual bridge methods, old request/response types and chess_adapter deleted. R1 needs no R2 activation or permanent observation adapter; existing outer desktop codecs remain R2-owned. Native Xiangqi exact evidence and Take back lineages retained. Focused actual console8suites30PASS, engine39PASS, shared94+newwire1PASS, Chess/analysis testsPASS (one obsolete oversized-diagnostic capacity expectation corrected to strict earlier atomic rejection), frontend236full+25focusedPASS, TS/gui41PythonPASS. Added actual255-observation QAconstructor test to catch implicit response deserialization; PASS. Regenerated native fixtures including verification constructors; examplesPASS. Clippy initial test-only unnecessary .context expression corrected; final relevant clippyPASS. Separate reviewer r1_independent_review found no material findings and independently ran36engine,20Xiangqiarchive,2Shogi strict/schema and25frontend testsPASS. Its defaultNode launch failed missinglibsimdjson; pinnedNode24.19rerunPASS. Final controller/protocol checks and CI pending; noACchecked yet.

Final current local controller suite148/148PASS (80.03s), externalUCI/USI/Xiangqi29/29PASS; relevant alltarget/allfeatureClippyPASS, format/diff/doctorPASS. Source/deletion review approved with no material findings. Resource measurements include currentwire: Chess4096events request573986/response574161;Shogi320 request80074/response80399;Xiangqi1024events request253992/response254436; no limit changes. Currentarchive warning89events32obs834832bytes/reopen323ms; acceptedcount103events7obs227613bytes/709ms. Fullsource/verification detail docs/review/execution-migration-116.md. Current source commit and CI next; do not markDone before evidence.

Final verification-feature snapshot audit removed remaining old framing in255 capacity.json observations via actual native generator. Existing7Xiangqi snapshots refresh native Take back capability only. Separate reviewer compared all255 prior/current native semantics and metadata, noloss; independently9HistorytestsPASS. RootfocusedAnalysis/History26PASS andTypeScriptPASS. No old execution identifiers in productioncallers and no old observation framing in currentfrontend snapshots. Historicalevidencefilesunchanged; no R2 taskactivated.

Reviewed source integrated and pushed on codex/rewrite at900544cf813ebb6b11428ac0fc9d5d45399dee1f. Independent commit audit confirms reviewed source/deletions and no later-stage implementation. Exact native CI41073291155 and desktop/packageCI41073291104 running. No AC/DoD marked complete while CI/evidence pending. Current contract/evidence doc links resolve; checkout source remains unchanged. One integrator only; no task117activation.

At exactsource900544c, LinuxDesktop41073291104/job124122703356 PASSED actual14GUIphases. Downloaded artifact12986746658 (sha256:df89bd78b0cf928023d21d8df9759e67c7595b9deee566db3bd30ac6beb357e1). Root and separate reviewer audited229GUIhashes,13normalmanifestfiles,QAdebhash,14pre-teardownnativeexitproofs, normal16QAmarkerexclusions. Actual savedChess/Shogi ordinary/history and256-capacity observations allsemanticschema1,nooldresponse/protocolfields; stoppedreopen/exactprefix/ReturnLive/Resume verified. Packagedengineactualprotocol4 turn/analysis/FullUnknown verified. NativeLinux/macOSCIgreen; WindowsnativeandpackageCIpending. No newXiangqiGUIorMacGUIclaim. Selected artifacts underdocs/review/evidence/msh116/linux; finalcriterionchecksawaitremainingCI.

Final acceptance at production source900544cf813ebb6b11428ac0fc9d5d45399dee1f: Rust41073291155 and Desktop41073291104 each3/3platformjobsSUCCESS. Linux actual14packagedGUIphases,229artifacthashes and14nativepre-teardownexitproofs; Windows actual14phases,202hashes and14exitproofs audited independently. Durable schema1 observations verified in savedChess/Shogi histories and both256capacityarchives. Final fresh read-only reviewer supportsAC1-6 with no material findings. CI fullfrontend241PASS; native148,engine39,console30,externalprotocol29,verifier41 and other focused checks retained in docs/review/execution-migration-116.md. Mac package/engine/nativeCI only; no newMac orXiangqiGUI claim. No production changes after900544; final task/docs/artifact reconciliation is evidence-only. Source integrated/pushed normally; no permanent execution adapter, no R2 activation, no new worktree. MSH112 ownership unchanged. All sixAC/fourDoD supported by linked criterion matrix and actual review;117 next-ready remains unstarted.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed R1 on codex/rewrite at reviewed production source900544cf813ebb6b11428ac0fc9d5d45399dee1f. One strict protocol4 ExecutionRequest/Response and submit/poll path now serves actual Chess/Shogi/Xiangqi consumers; obsolete wire families, Chess conversion adapter, fallback and paired bridge APIs deleted. Durable schema1 observations contain native semantic context/result rather than transport framing; controllers, archives, validators, fixtures, frontend and package verifiers migrated without loss of native history, exact evidence or Take back lineages. Current Save/Open works without starting117. Native semantics, identity/limits, cancellation/cleanup and external protocols preserved. Independent read-only review found no material remaining issue; all6native/desktopCIjobs green with actual Linux/Windows14-phase packageGUI evidence. Full evidence, exact revisions, resource measurements, development failures and platform limits: docs/review/execution-migration-116.md and evidence/msh116. No newMac/XiangqiGUI claim; no rules/strength change. Outer archive consolidation117, ownership118 and frontend119 remain later stages;117 dependency-ready and unstarted. Final evidence-only reconciliation follows the already integrated/pushed source; no other task closed.
<!-- SECTION:FINAL_SUMMARY:END -->
