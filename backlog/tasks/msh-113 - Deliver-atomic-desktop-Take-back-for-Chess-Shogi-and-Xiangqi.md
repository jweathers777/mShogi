---
id: MSH-113
title: Deliver atomic desktop Take back for Chess Shogi and Xiangqi
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-29T15:39:00Z'
updated_date: '2026-10-29T16:21:00Z'
labels:
  - desktop
  - m4
milestone: m-4
dependencies:
  - MSH-102
  - MSH-75
  - MSH-61
documentation:
  - docs/design.md
  - docs/desktop/take-back.md
type: feature
ordinal: 113000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner-approved new product scope from attachment17dba816: human-versus-engine users need to return to their preceding playable choice rather than undo a raw event. Preserve completed101/102, accepted common shell and native rules/records. One native transaction must identify the most recent human board decision across complete native command/event boundaries. Current raw undo differs across games and Chess observation validation assumes a single final lineage; retention and saveability must be resolved within this feature rather than dropping accepted evidence. Do not start MSH-103, MSH-106 or subsequent tasks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Native command boundaries and actor identities own targets and reversal. Preserve game-specific repetition, claims, warnings, conclusions, counters, provenance, complete actions and canonical replay; no frontend reconstruction or two-request undo sequence.
- [ ] #2 Retire outstanding turn/analysis execution and selection before committing restoration; advance runtime identities and prevent all abandoned or late results and stale markers from becoming current. Restored human position resumes through the native lifecycle without automatically replaying the engine response.
- [ ] #3 Save/Open after takeback preserves complete accepted document evidence including historical observations with original targets and abandoned native continuation needed for replay. Preserve old Chess/Shogi/Xiangqi compatibility, strict parsing, atomic file failures and fresh stopped reopen. Admit actual byte/event/control/observation/replay work and close headroom before commit; resource refusal leaves a complete save path and never discards evidence.
- [ ] #4 All three games expose Take back consistently in Game Actions using existing styles, focus/status/accessibility patterns; native eligibility disables it at beginning or unavailable/history states, selections and read-only inspection clear on restoration, no unnecessary confirmation or diagnostics.
- [ ] #5 Focused native/frontend tests cover owner scenarios, terminal/history-dependent state, cancellation/late results, repetition, archive roundtrip/capacity/failure and cross-game compatibility. Exercise each game in actual packaged macOS application with genuine native engine/session, record source/package identities and distinguish QA instrumentation, obtain independent review, resolve findings and run applicable CI.
- [ ] #6 Document delivered semantics, compatibility decisions, limitations and concrete future MSH-106 findings without broad refactoring; integrate and normally push, preserve closed milestones/history and stop at this task.
- [ ] #7 One native atomic Take back restores the most recent playable human decision point: completed human/engine pairs reverse both moves; a pending response is retired before reversing the human move; engine-first opening is preserved; repeated operations and concluded native histories restore eligible preceding human choices. Historical selection and unavailable/beginning states refuse atomically.
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
1. Root Codex is primary builder/integrator on clean primary codex/rewrite at8f21eb212d1847d75a7d8580882ae4afeacf193b, origin refreshed equal; no active task owner. Use pinned Backlog1.55.1 instead of installed1.55.0. Dependencies102/75/61 are Done; preserve completed tasks and existing unrelated app worktrees. Read owner Take back specification and native contracts before coding.
2. Add native TakeBack transaction and native take_back eligibility across explicit game dispatch, plus versioned legacy Chess command/capability for existing consumers. Targets come from accepted native actors/command boundaries, retaining engine opening and all game-specific history. Concluded terminal decisions need a preceding playable human point, including terminal-only cases when authoritative. Restore paused at that human point with existing explicit Resume; no abandoned engine auto-replay. Native candidate admission must retire pending/analysis and advance identities atomically; invalid/unavailable/historical requests leave accepted state intact.
3. Reuse native Shogi/Xiangqi journal replay and reversal/prefix facilities with bounded replay charges. Chess needs a narrow compatible archive extension retaining authoritative abandoned match lineage and exact observations; validate by native replay and bind observations to admitted lineage, not arbitrary independent records. Preserve existing records/readers or explicitly reject unsupported new versions; do not discard observations/history or raise limits. Account actual encoded bytes, cumulative events/controls/replay and close reserve before commit. One isolated native worker may own these files; root owns UI/host/docs/integration.
4. Expose one small shared Take back control in consistent Game Actions positions, native eligibility only, no confirmations/history reconstruction. Use complete shared source token for normal product dispatch (including Chess), native legacy version where needed. Reuse work-command focus/status and token-based selection/inspection invalidation; no broad Chess/Shogi architectural refactor. Document concrete106 findings without starting it.
5. Focused native/frontend tests cover both human colors, pair/pending/engine-opening, repeated and terminal/history/claim/warning cases, stale/late/cross-purpose results, exact archival retained lineage, Save/Open fresh stopped identities, strict malformed records/capacity rejection and compatibility. Independent completed-boundary review and relevant regressions/CI; actual packaged macOS GUI for all3 games using genuine engine/session, labeled external delay instrumentation only as needed. Preserve current user app state; separate QA instance if needed.
6. Record exact source/package/hash/environment/GUI/test/review/CI evidence and limitations, finalize only after all original owner requirements and task checklist supported. Normal commits/integration/push, primary staysrewrite, no own temporary worktree left. Stop113; do not start103/106 or other tasks.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implementation underway: root owns frontend/host/docs and integration; independent isolated native builder /root/takeback_native_audit owns session/core changes without commits. Added one shared Game Actions control; product Chess dispatch uses full shared source token rather than legacy version-only path. Generated authoritative existing snapshot fixtures with native capabilities. Initial frontend regression: 204 tests/24 files and TypeScript passed. Current user application session was preserved through actual native Save As to /private/tmp/msh113-preserved-preverification.mshogi before closing for rebuild. No task103/106 work started.

Independent read-only frontend review (/root/takeback_ui_review) approved corrected boundary after59 focused tests/5files +TypeScript. Resolved P2: product Chess shared dispatch refusal errors were disconnected from legacy notice; shared error/busy now reach Chess with stale and capacity refusal regressions. Fullfrontend207/24files PASS. Native independent review (/root/takeback_boundary_review) found retained abandoned Chess records needed current-record untimed/witness restrictions and control-timestamp binding (fixed), unavailable Shogi request must synchronously preserve pending initial engine work (fixed/test), and v3 replay budget must reserve mandatory stop/close control (fix/test in progress). Root source readback is integrator verification, not independent review. First normal package construction began before final review corrections; it is not GUI acceptance evidence and will be rebuilt.

Implementation complete for independent assessment. Native review approved final production after resolving3 boundary findings: retained Chess checkpoint restoration restrictions/control timestamp, unavailable Shogi atomic refusal, and reserved stop/close replay headroom. Independent native reviewer executed12 focused Take back tests PASS (117.04s on equivalent slower boundary fixture). Native builder final focused all-feature12/12 PASS6.56s after test-only efficiency change; all-target/all-feature clippy PASS. Concrete final boundary fixture1320 genuine Observe commands,123 accepted Preferences,128 controls,333246 encoded bytes, actual pause_for_close plus strict native Open PASS. Final API TakeBack shared/legacy and native capability, paused explicit Resume. Chess writes v1 until firsttakeback then v3 retained authoritative lineage; Shogi/Xiangqi stayv2. Actual packaged macOS GUI, final checks and CI remain required before Done.

Actual ff0b1ce packaged Chess human e2e4/bundled a7a5→TakeBack restored0plies/paused/1retainedobservation; nativeSaveAsv3succeeded. GUI exposed clipped Resume notice when sidebar remained scrolled to GameActions. Sharedcontrol now reveals existing .play-paused notice with nearest scrolling after accepted-context status focus; no layout redesign. Added4thcontroltest, independently reviewed approved. Rebuild/recheck final GUI source required; earlier clipped capture is finding evidence only.

Final bfe09cf packagedMac3gamejourneys PASS withgenuineengines: Chess/Shogipairs→0, Shogihistorydisabled/nativeinspectioncleared, XqBlack3→1preservesRedopening, terminalresignation→playable1, keyboardTabReturn, eachSaveOpenstopped/historicalobsretained. QAexternal8slaunchdelay eachpending1→TakeBack0 stayspausedbeyonddeadline; nativeinjectedlate-response testsseparate. Finalartifactindependentreviewapproved. Nativeallfeatures144PASS91.62s, frontend208PASS, host5PASS, fmt/clippy/doctorPASS. LinuxDesktopCI40907948723 failedtwoobsoleteShogiconcluded-no-actionsassertions992/996, exactlyconfirmedbyartifact; notdriverreset. UpdatedexternalrunnerrequiressoleenabledTakebackandactuallyrestoresdeclaration/resignation0/Sente/paused, keepsoldhistory/conclusioncriteria. Independentverificationdiffreview+24Pythonselftests+compilePASS. Originalfailurepreserved; correctedplatformGUIrunstillrequired.
<!-- SECTION:NOTES:END -->
