---
id: MSH-75
title: Independently verify complete standard-shogi delivery and reconcile MSH41
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-27T14:09:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-64
  - MSH-66
  - MSH-67
  - MSH-68
  - MSH-70
  - MSH-71
  - MSH-74
  - MSH-69
  - MSH-72
  - MSH-73
  - MSH-90
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/implementation-map.md
  - docs/m4/delivery-plan.md
  - docs/m4/shogi-delivery-review.md
  - docs/m4/evidence/msh75/closure-review.md
type: task
ordinal: 75000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Fresh independent game integration review and coverage ledger; appropriate native CI plus recorded running application evidence, reusing unchanged chess/M3 evidence.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Independent review traces every original MSH41AC1-8 to accepted profile,authoritative state/legal/reversal/history,independent corpus,console,own native engine,records and actual desktop evidence.
- [x] #2 Runtime reference comparisons,real interoperability,self-play,strict archive/history/capacity and game-specific application journeys are supported; unavailable/failed checks remain blockers and source audit alone is not conformance.
- [x] #3 Material findings and coverage gaps are resolved,with actual revisions/commands/environments and honest functional-versus-strength limits. Shared final acceptance stays in MSH46/MSH47.
- [x] #4 The complete MSH41 delivery record is closed only after all its original obligations and required shared tickets satisfy acceptance and integration. No child completion alone marks the game complete.
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
MSH-75 current integration review plan,2026-10-27. Sole integrator/builder Codex on primary codex/rewrite at80d6b5cc71c8ceb5bf782d93295797aed1e18d66; clean checkout, only primary worktree, no active task owner. All11 prerequisites Done. MSH-74 owner-approved/Done; do not redesign or reopen for taste. MSH-41 remains To Do pending this complete-delivery verdict; M4/other games remain open.
1. Trace all8 original MSH-41 acceptance criteria against selected mshogi.shogi.csa36.v1, current native consumers, clause ledger and MSH-63–74/90 evidence. Distinguish expected fixtures, admitted-reference observations, native/process/UI/platform execution, independent AI review and owner approval. Preserve failed/historical evidence and functional-versus-strength limits.
2. Delegate three fresh bounded independent review tracks under one integrator: complete native rules/history/records and independently sourced conformance; native search/structured transport/console/USI/self-play/external-opponent path; strict desktop archive/history/capacity/native session/presentation integration. Review actual integrated source and cross-boundary journeys, not task status alone. Reviewers may run focused read-only verification or isolated experiments, not competing implementation edits. Record actual methods and classify blockers against MSH-41.
3. Run fresh integrated native workspace tests/all-target/all-feature formatting+lint, relevant console/protocol subprocess and desktop-session/frontend/host checks, canonical replays and admitted reference where useful. Reuse unchanged actual reference/opponent/platform evidence with explicit source fingerprints; run concrete new end-to-end normal Mac Shogi journey inclengine play,Save/Open,history,Return/Resume/Flip and keyboard/inspection where applicable. Preserve current user session before replacement. Retain exact package/source/environment and screenshots, not harness-as-product claims.
4. Resolve concrete correctness/completeness/losslessness/interoperability/accessibility/product-contract findings with focused regressions and independent rereview. Never weaken accepted profile, expected fixtures, MSH-61 guarantee or original criteria. Record minor/outside-profile/strength limitations separately; bring back only material unresolved rule/product decisions or concrete blocker.
5. Produce integrated MSH-41 AC1–8 mapping, complete clause/fixture/execution ledger, findings/verdict and reproducible evidence. Check actual relevant Windows/macOS/Linux CI at reviewed source; repeat packaged-platform interaction only for affected production changes. Independent final acceptance review. If allobligations satisfied, complete75 then41 via CLI, integrate/push normally, preserveM2/M3 androadmap,leaveM4/othergames/sharedfinalclosureopen. Stop; report exacttested/integratedrevisions,scope/evidence,limitations andnextdependency-readyM4queue withoutstartingit.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-90 is the required bounded common-console follow-up to MSH-68/69/70, owned once under MSH-46. Added to this final acceptance gate without removing existing dependencies or changing criteria. MSH-71/72 remain unstarted.

Owner current request retained as task authority: attachment c9d5505e-91b8-409b-8893-33cd14a95d50. MSH-74 accepted final shared Chess/Shogi shell has owner visual approval. MSH-75 independently reviews complete integrated MSH-41 delivery; closure of41 is authorized only if alloriginalrequirements pass. No M4 closure or another-game implementation authorized.

Final review boundary 2026-10-27: 80d6b5cc71c8ceb5bf782d93295797aed1e18d66 (codex/rewrite), no production or expected-fixture edits. Three fresh independent AI reviewers inspected integrated native rules/history, engine/console/USI/runner, and archive/session/normal desktop boundaries; all report no material blocker. Reports and exact execution/provenance are in docs/m4/evidence/msh75/. Root fresh workspace: 409 passed, 0 failed, 2 optional deep chess checks ignored; fmt/clippy PASS; strict TypeScript and 161 frontend tests PASS. Reviewer fresh reference:169 independently expected fixtures,167 sequences,1304 plies/1471 prefixes,zero mismatches; independent21 history-ledger/149-prefix and9 declaration-root audits PASS. Six fresh selfplay + six actual pinned independent YaneuraOu games and replay PASS (functional, not strength). Fresh normal Mac CUA Sente/Gote, capture/demotion, promotion/drop cancellation/completion, engine, read-only opponent inspection, native Save/Open fresh tokens/stopped runtime, prefix analysis, Return/Resume/Flip and resignation journeys PASS; native records/screenshots/AX retained. Current80 Rust40505357243 and Desktop40505357155 all three platforms success. Independent source/hash applicability audit supports reused final MSH74 actual Windows/Linux GUI:14 phases each=3 normal+11 verification;175 retained artifacts match. Primary original41AC1–8 mapping: docs/m4/shogi-delivery-review.md; current ledger stale future-tense summary reconciled while preserving historical evidence. Fresh final reconciliation reviewer msh75_closure_review assesses complete mapping and fresh normal GUI; no ticket closure before verdict.

Post-integration verification at evidence commit4c4145eaacf7fcaf89a338773aa110a3700e4c75: Rust40512188338 all3 platforms PASS. Desktop40512188317 attempt1: macOS/Windows PASS; Linux13/14 phases, first Chess verification e2 pointer POST /actions lost its HTTP response (RemoteDisconnected, driver SendRequest peer reset). All3 normal+all6 Shogi verification phases pass; failure capture subsequently reads same0-ply native app, no panic/crash evidence. Independent bounded follow-up docs/m4/evidence/msh75/ci-followup-review.md audits all220original artifacts, failure DOM/image/traceback and unchanged product/driver/hash bytes; root cause of downstream reset remains unknown. Original failure evidence retained in ci-linux-attempt1; no assertion weakened and no ambiguous input retried. Only failed Linux job rerun unchanged as attempt2, currently pending; task stays In Review until actual result. Earlier actual-source CI and independent complete-delivery verdict remain distinct from this new incomplete check.

Final independent reconciliation PASS: docs/m4/evidence/msh75/closure-review.md traces original41AC1–8 through complete selected profile and actual integrated consumers. Three fresh independent AI source/test/artifact tracks plus fresh independent final reconciliation; no material product finding. Reviewed/freshly locally tested80d6b5cc71c8ceb5bf782d93295797aed1e18d66; evidence integration4c4145eaacf7fcaf89a338773aa110a3700e4c75 pushed to origin/codex/rewrite, production/expected-fixture sources identical. Final Rust40512188338 and Desktop40512188317 allthreeplatformjobscompleted/success at4c. Linux attempt2 actual14/14GUI passes; independent follow-up verifies exact artifact12739498661/ZIP75cd0650063f3b695e537ed9c63f779a9a931ceeb65e99188a8d64a0d17c5d8e,223original fingerprints,16unchanged fullinspectionobjects,14nativeabsences. Genuine missing-engine3ply failure→restore identical engine→Retry4plies→nativeQuit is now executed. Initial13/14failed pointer transport attempt remains failed original evidence; no ambiguous action retried, assertions weakened or source fixed. Slow11m47s OSdependency install and stale API summary fields are separately identified, final fullmetadata now success. All41obligations supported; close75 then41 perexplicitownerrequest. M4/42–47 remainopen, MSH50 strengthfuture, nexteligibleM4research80–83 only; no nextgame started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Independently verified complete standard-shogi delivery against every original MSH-41 AC1–8 and selected mshogi.shogi.csa36.v1. All required profile clauses and real native→console→history-aware engine→structured transport/USI/runner→v2 archive→normal desktop paths have evidence; MSH-61 lossless admission remains intact. Four fresh independent AI reviews found no material product defect; stale current-facing ledger prose was reconciled while historical records were preserved. Fresh local409native passes/2optionaldeepchess ignored,161frontend passes,fmt/clippy/strictTS and8host checks PASS. Fresh admitted-reference169fixtures zero mismatches, independent21history/nine declaration audits,6selfplay+6actualpinnedopponentgames/replays, normalMacSente/Gote Save/Open/history/analysis/hand/promotion/drop/engine/resignation journeys PASS. Source80d6b5c; integrated evidence4c4145eaacf7fcaf89a338773aa110a3700e4c75, codex/rewrite. Final native40512188338 and desktop40512188317 allthreeplatformCI PASS; unchanged Linux rerun14/14 resolves the incomplete verification while preserving its first13/14transportfailure and independent diagnosis. Full mapping/review/commands/provenance/limitations: docs/m4/shogi-delivery-review.md and evidence/msh75/. Supports MSH-41 closure; M4 and other-game/shared-final/strength obligations remain open. No production change or other-game implementation.
<!-- SECTION:FINAL_SUMMARY:END -->
