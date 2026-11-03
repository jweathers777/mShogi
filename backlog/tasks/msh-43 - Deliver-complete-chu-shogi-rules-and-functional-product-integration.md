---
id: MSH-43
title: Deliver complete chu shogi rules and functional product integration
status: In Progress
assignee:
  - '@Codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-03T10:46:00Z'
labels: []
milestone: m-4
dependencies:
  - MSH-39
  - MSH-72
  - MSH-73
  - MSH-78
  - MSH-82
  - MSH-86
documentation:
  - docs/design.md
  - docs/m4/delivery-plan.md
  - docs/m4/implementation-map.md
  - docs/testing/conformance-policy.md
  - docs/m4/chu/profile.md
  - docs/m4/chu/fixtures.md
  - docs/m4/chu/handoff.md
  - docs/m4/chu/shared-v1.md
  - docs/m4/chu/shared-verification.json
  - docs/m4/chu/engine-v1.md
  - docs/m4/chu/notation.md
  - docs/m4/chu/desktop-visual-checkpoint.md
ordinal: 43000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-01, P-03, P-04 and A-01 through A-07 require a complete chu shogi implementation, not a piece-movement demonstration. This scope owns the game-specific rules, adapters and conformance evidence across the product. Shared mechanisms and cross-game integration are owned once by MSH-46; coordinate their delivery without duplicating ownership here. Functional mShogi engine support is required in M4; measured strength targets and tuning belong to MSH-52 in M5.

Current MSH-82 profile/handoff: mshogi.chu.jcsa2019.v1 adopts modern JCSA2019 with compatible2004 supplement/movement sources, explicit owner direction and independently expected fixtures. docs/m4/chu/profile.md and handoff.md refine existing86(native)→78(shared)→43(product) without changing original8AC/DoD/dependencies. Preserve capture-based movement, full compound actions/derived effects, native before-fourth admission, finite immediate mate, narrow exhaustion/actual reply and agreed draw. Historical alternatives remain evidence, not a reconstruction gate. Select routine actual Rust/action/notation adapters against real consumers; no universal framework. Native/shared handoff does not itself deliver console, bounded functional engine, finished traditional-kanji12×12 UI or actual playable-product/platform journeys. No implementation begins under this profile-only assignment.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A sourced, identified and versioned chu shogi rules profile covers the complete game: initial state, every piece/action category, captures and derived effects, promotions and any drops/special choices, turn progression, legality, automatic/claimable outcomes and history-dependent adjudication. Consequential interpretations are recorded; unresolved required rules block conformance rather than disappearing from scope.
- [ ] #2 The reusable rules library owns authoritative position, complete actions and atomic rejection, exact apply/undo, rules-relevant history and match/adjudication state. Saved-state, repetition and search identities are specified separately; clients and training tools do not duplicate legality or outcomes.
- [ ] #3 Independent sourced chu shogi fixtures cover legal and illegal choices, transitions, special cases, history and outcomes. Exact reversal, sequence and record round trips, applicable perft/divide and differential/reference comparisons have reproducible evidence and provenance; self-consistency or legacy agreement alone is insufficient.
- [ ] #4 The independently usable console can select and inspect chu shogi, list and apply complete actions, undo, report history/outcomes and run appropriate diagnostics with honest coverage and separate human/machine output. Game-specific command/notation adapters preserve the authoritative semantics.
- [ ] #5 The native mShogi engine can search and play chu shogi using the same authoritative rules/history, with legal complete results, correct terminal/claim handling, request identity, limits, cancellation and stale-result safety. A functional baseline and reproducible self-play/match smoke evidence are recorded without claiming competitive strength.
- [ ] #6 Canonical saved chu shogi records preserve profile/version, initial state, necessary history, complete actions and applicable match policies/settings/events. Replay restores semantics exactly, and unsupported profiles or lossy notation/adapters fail explicitly. This ticket owns game-specific encoding/replay coverage; common schema migration belongs to MSH-46.
- [ ] #7 The desktop supports actual chu shogi human/engine play and bounded inspection/analysis, complete action entry, readable accessible presentation, outcomes/recovery, save/reopen and history inspection through the authoritative native session. Game-specific end-to-end evidence exists; common infrastructure, cross-game regression and packaged three-platform acceptance are owned by MSH-46 and verified at MSH-47.
- [ ] #8 The game coverage ledger and independent review show all required chu shogi scope satisfied, with no required behavior relabeled as optional or deferred to M5. Shared integration findings remain visible in MSH-46 and block M4 closure in MSH-47.
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
2026-11-03: Codex sole integrator on clean primary codex/rewrite a5d5bd4; original8AC/4DoD/dependencies preserved. A: consume native Chu Game in concrete transparent material/full-history bounded search and existing supervised protocol4 results; measure standard/tactical costs before interactive defaults, verify exact reversal/terminal/repetition/cancellation/incomplete behavior. B: native Rust route/capture/promotion notation and bounded console in existing application; actual subprocess and reproducible play/ending probes. C: extend current Chu immutable document/session/result admission and existing scheduling, preserving 12M aggregate/4M native/15s limits, complete observations before moves, exact PV/target identities, 256-to257 refusal, reserved endings, stopped reopening and stale retirement. D: real traditional-kanji12x12 renderer via existing GameFrame/choice controller, native routes/royals/effects/inspection, common panels/recovery/analysis, no hands or frontend rules. Normal chooser/Load stays4games; stable QA selector exercises actual functional Chu session for visual checkpoint. Delegate isolated engine, console and renderer files; integrator owns shared desktop/native execution contracts and durable task evidence; independent reviewers challenge completed native execution/admission and actual visual/interaction boundary. Check exhaustive production tags, both Cargo workspaces and strict frontend early, focused tests then coherent relevant4game regressions. Required visual-checkpoint CI:3platform Rust and affected Desktop frontend/host/package checks; distinguish existing4game GUI campaigns from new actual macOS Chu interaction. One structured verification record per tested revision. Capture actual stable-identity macOS app at1180x820,960x720,760x650 bothorientations and significant routes/captures/promotion/royals/history/analysis; integrate/push reviewed source, then STOP for owner visual approval with taskunfinished. No normal5thgameactivation/final3platformChu gameplay/final8ACclosure until approval;43/M4remainopen.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH62 planning refinement2026-10-26: original identity,status,AC and DoD remain intact. This complete-delivery record now depends explicitly on its required bounded component/shared work; docs/m4/delivery-plan.md traces each original criterion and single ownership. No game implementation or delivery completion occurred. Earlier M3-context wording remains historical; current authorization is planning-only.

Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

MSH82 current modern-profile handoff2026-11-02 supersedes original open-historical-question wording without deleting the old planning/research evidence. No original AC weakened:250 independently expected data rows precede native execution; actual conformance remains86, shared78 and full game43. Complete12×12 traditional-kanji board visible without scroll/zoom in accepted common shell; native inspection wording and full routes/royals retained.52 owns competitive strength;46/47/88/89 and107 retain shared/platform/diagnostic ownership. Original dependencies preserved; this record remains To Do/open.

MSH78 complete shared handoff2026-11-03: tested/integrated native/shared source 250b701efd23dac648ffdb984c83da868fa8840a, nativeCI41615048164 all3jobsPASS; full checkpoint fcccc37 Rust41610001673/Desktop41610001503 all6jobsPASS, unchanged Desktop evidence reused for final native-only diagnostic correction. Actual canonical Chu/immutable document/controller/history/Take back/choices/royals/inspection/capacity contracts and seven real DTOs documented in shared-v1.md. Exact counters remain decimal text; technical Moves number known history only. Execution unsupported/unscored and normal chooser/Load gated until this task delivers actual engine/product. Original dependencies now satisfied; To Do/unassigned remains unchanged, no43implementation begun. Traditional kanji/full12x12 no-scroll product, engine/console/notation, complete games and packaged GUI/platform evidence still owned here; preserve native/shared evidence,12M aggregate and actual ending reserves.

Owner2026-11-03 authorizes coherent functional-engine/console/notation/finished desktop visual checkpoint, then explicit owner aesthetic review. This supersedes old profile-only inactivity notes without deleting them. No additional planning phase or component-ticket split. Current78 source250b701/integrationa5d5bd4 and accepted82/86profile/evidence unchanged. Other3existing worktrees remain unrelated and untouched; no competing43owner.

Functional checkpoint2026-11-03 onprimarycodex/rewrite: actualnativeChu search/result/console/notation and traditional29facekanji renderer implemented with oneexistingcontroller/choiceowner; normalchooser/Load remains4games pendingownercheckpoint. Measuredreleaseplay1/512 standard37visits5ms andLion92visits11-15ms; analysismax2/2000,notstrengthclaim. Independentnativeexecution/console review27focusedtestsPASS; rendererreview44testsPASS plusportedfooterregression6PASS. Actualchild/controller6testsPASS includingbothassignments,cancelledPIDcleanup,history/SaveOpen/stalecontexts. Newactualresultcodec5testsPASS,32plyrealengineprogression32obs651611B/534616aggregateunits/528retainednativeevents;Takebackretainsbranch+allobs675048B/541771units;256actualzero-workobs2601304B,257typedrefusal preservesbytesandStop/Resign/Close. Historical1076nativechecks remain86evidence,notrecounted. Earlyworkspacealltarget/standalonehost/strictTS consumerschecked beforebroadCI. Coherent4gameregressions,finalreviews andactualstableQApackagedmacOSsizes/interactionstillpending;taskremainsInProgress,8AC/DoDunchecked. No owneraestheticapprovalorfinalproductactivationclaimed.
<!-- SECTION:NOTES:END -->
