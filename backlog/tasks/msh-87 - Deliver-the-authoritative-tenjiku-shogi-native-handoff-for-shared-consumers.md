---
id: MSH-87
title: Deliver the authoritative tenjiku shogi native handoff for shared consumers
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-11-04T10:44:00Z'
labels:
  - m4
  - owner-msh-44
milestone: m-4
dependencies:
  - MSH-83
references:
  - >-
    backlog/tasks/msh-44 -
    Deliver-complete-tenjiku-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/later-game-sources.md
  - docs/m4/delivery-plan.md
  - docs/testing/conformance-policy.md
  - docs/m4/tenjiku/handoff.md
  - docs/m4/tenjiku/profile.md
  - docs/m4/tenjiku/fixtures.md
type: task
ordinal: 87000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-44. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Refine after source/profile decisions into independently executable native slices; require rule corpus,exact reversal,record replay and real native consumer fixtures. The complete game remains open.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 After profile refinement,a native game-owned kernel exposes authoritative state,complete legal actions/derived effects,exact reversal,history/adjudication and canonical record contracts needed by its named shared consumer.
- [ ] #2 Independent rule/transition/history fixtures establish the handoff,including relevant board topology,hands/promotions,compound routes,royal identity and non-chess terminal behavior; no unverified rule convention becomes an interface default.
- [ ] #3 The shared extension consumes stable documented native payloads/capabilities without frontend rules or duplicate ownership; further game-specific engine/console/desktop work remains visible in its complete delivery record.
- [x] #4 Before Ready,the game owner splits this native scope further where useful and records all child dependencies/verification. No implementation plan is prescribed by this planning task and no dependency on its own aggregate delivery is introduced.
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
A. Implement explicit two-profile16x16 state,44faces/30promotions/156setup, complete local actions/effects/inspection and instance-bound reversal; focused independent local-rules review.
B. Implement authoritative Game history/promotion rights/repetition/royal and bare-response endings, compact reversal tokens, strict cancellable schema1replay and measured native admission with actual Stop/Resign reserve.
C. Add thin Tenjiku conformance corpus boundary and real production adapters for all applicable350rows, exact geometry/action/effect sets, concrete histories, native properties/adversarial cases and external-core-consumer/perft checks. Keep native versus annotations versus future-consumer accounting explicit.
D. Review source/adapters/resource measurements independently, check root and standalone-host consumer boundaries, run coherent relevant regressions/format/lint and one exact-source three-platform nativeCI campaign. Integrate/push, finalize only supportedAC1–3/DoD, preserveAC4 and update79/44handoffs. No shared/product activation or new task hierarchy; stop after87.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner2026-11-01 testing-policy supersession: rules-profile-derived independently expected fixtures/history, deterministic regression, exact reversal/replay, meaningful property/metamorphic/adversarial tests and independent review establish conformance without any external engine. External comparisons are optional investigative evidence, not build/test/CI/milestone/release gates. Historical observations/source audits retained; no completed-game campaign reopened. For Tori,126 is now the self-contained executable harness; no Fairy build/admission prerequisite. Existing task dependencies and implementation plans unchanged; no activation by this note. Apply game-specific properties where meaningful, never expected outputs copied from the implementation. See docs/testing/conformance-policy.md.

MSH83 current handoff4November2026: implement the coherent native kernel directly after ratified83, as one bounded assignment with internal state/transitions, LegalAll/effects, history/records and conformance/acceptance checkpoints. No additional child hierarchy is useful under current owner direction; originalAC4 where-useful refinement is satisfied by this direct bounded scope, without changing its text. Two whole immutable profiles: mshogi.tenjiku.adams1999-modern.v1 and mshogi.tenjiku.muller-completed.v1. Preserve actual FD/FE/HT/general differences, explicit shared A/S completions, original/promoted identities and per-piece refusal rights.350corpus rows:204native-boundary/129annotations/17consumer; all native observations pending87, not loader passes. Add thin native corpus adapters reusing shared comparators, independently exercise full action/effect/royal/history/undo/replay/negative/property cases. Native resource accounting must be measured;107 replay-retention/cancel/inspection risks are measurement obligations, not automatic refactor gates or inherited Chu constants.79 consumes actual handoff;44retains engine/console/notation/kanji/playable delivery. No implementation plan or activation is added by83.

Ready refinement completed by83: independent handoff/contract review confirms one bounded direct native task is appropriate, no children useful now under owner instruction; originalAC4planning obligation supported by docs/m4/tenjiku/handoff.md. RemainingAC1–3/native assertions/DoD staypending; unassigned, no implementation plan or implementation started. Dependency83Done, no dependency changes.

Startup4November2026: clean primary codex/rewritea7f9205 after unrelated historical-access documentation integration; origin configuredgithub:jweathers777/mShogi.git, existingfourotherworktrees preserved. No InProgress/InReview owner; @codex sole integrator. PinnedBacklog1.55.1 verified, global1.55.0 not used for writes. Both ratified profile/corpus data remain fixed. Isolated native local-state/rules and record/diagnostic file ownership may be delegated; root owns Game/integration/conformance; fresh review remains read-only and independent. No87child hierarchy,79/44implementation or historical research campaign.

Native checkpointsA–Dsource complete: public two-profile Tenjiku state/local actions/effects/Game/history/strict schema1/reversal/perft; frozen204rows/621native expected fields perprofile allpass,129annotations/17consumer rows explicitlyexcluded. Current core+conformance regressions373pass/0fail/2existingignored, native41 and externalconsumer2 included; fmt/workspacealltargetsClippy/standalonehostcheck pass. Fresh independent source/adapters/capacity/consumer review passes with material findingsresolved. Native1MiB/1024events/512Munits plus actual eight-actor-combination Stop→Resign→Stop and runtime ending reserve. Release128plies bothprofiles and1021event controls pass; Adams1021boardplies557612B/299287458units/3.264s replay, fullprocess40.3MBRSS. See docs/m4/tenjiku/native-v1.md and native-verification.json; exact-source threeplatform RustCI and final reconciliation stillpending, AC1–3/DoD unchecked. No79/44 implementation or normal activation.

Reviewed native source integrated/pushed on codex/rewrite5786e5950a0185359f9ed5c3c4ae758448832622. One required RustworkspaceCI campaign41815468988 (https://github.com/jweathers777/mShogi/actions/runs/41815468988) is running exactsource; required Ubuntu24.04/macOS15/Windows2022 jobs. One completion-aware ghwatch monitor; automatically triggered Desktoppackages41815468887cancelled as outside native-only scope. Fresh reviewer reconciled17sourcehashes,5rawmetrics/RSS,actual373pass0fail2ignored andnativeassertion accounting, preCIacceptancepasses no materialopenfinding. Native sourcefrozen; only CI/evidence/backlog finalization remains. SSHfetchpublickeyfailure handled with per-command authenticatedHTTPS withoutremotechange; initialGitHubpush500 normalretry succeeded. AC1–3/DoD remainunchecked untilrequiredCI/finalacceptance.

CI41815468988 at5786e59 found Windows fixture hash mismatch before native tests: Tenjiku JSON lacked existing Tori/Chu .gitattributes LF policy, so checkout converted reviewed bytes toCRLF. Preserve Windowsfailure/log/provenance; cancelled superseded Ubuntu/macOS jobs, no platformsuccess claimed. Bounded correction pins fixtures/profile text eol=lf and explicitly adds CRLF-rejection loader mutation; no expectationbytes/hash/rule/nativecode change. Focused1test, fmt and conformanceClippy pass; independent reviewer verifiesattrs/hashes/negative test and accepts correction. Corrected exact-source3platform campaign required; task remainsInReview uncheckedAC1–3/DoD. Structured native-verification.json retains initialsource/campaign and reusedunchangednative measurements.
<!-- SECTION:NOTES:END -->
