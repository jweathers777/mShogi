---
id: MSH-97
title: Deliver own functional xiangqi evaluator search and structured child
status: Done
assignee:
  - '@codex'
created_date: '2026-10-27T15:03:00Z'
updated_date: '2026-10-28T09:33:00Z'
labels:
  - m4
  - owner-msh-45
milestone: m-4
dependencies:
  - MSH-84
  - MSH-95
documentation:
  - docs/m4/xiangqi/profile-proposal.md
  - docs/m4/xiangqi/fixtures.md
  - docs/m4/xiangqi/handoff.md
ordinal: 97000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Future bounded Xiangqi delivery work refined by MSH-80. Intent and acceptance only; no implementation authorization or speculative implementation plan. MSH-80 ratification and named source/fixture obligations gate affected behavior. Native rules ownership and existing complete-delivery criteria remain authoritative. Verification expectations are the observable checks in the criteria; record actual commands/revisions, independent review and normal integration before Done.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Provide transparent native Red/Black evaluation and functional bounded search/play using authoritative full known game context and selected native adjudication, complete legal actions and claim policy; retain native terminal, unresolved, incomplete and failure distinctions.
- [x] #2 Implement the real structured child consumer, truthful execution capabilities and purpose-specific read-only analysis acceptance. Bind full target/prefix/identities/limits; enforce cancellation, replacement, bounded queues/diagnostics/process cleanup and stale results across games.
- [x] #3 Run independent tactical/terminal/history correctness fixtures, deterministic work-limit searches, actual child entry-point and cancellation/replacement/resource tests; actual observations only, perspective and position provenance explicit. Expensive native assessment uses bounded or independently terminable work and returns unknown on exhaustion.
- [x] #4 Independently review evaluator/search/consumer evidence and run relevant chess/shogi regressions. M4 establishes functional legality and execution, not competitive strength; MSH-54 retains measured strength/tuning.
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
1. Implement transparent native Red/Black material plus documented soldier advancement evaluation and bounded full-width iterative minimax over private authoritative Game history, complete actions/optional valid count claims, native conclusions/pending and exact balanced semantic restoration; omit score caches.
2. Preserve recorded sound_fast_only ordinary policy and expose a separate explicit bounded exact-required native entry, retaining accepted native provenance/report. Exact Unknown/cancel/failure/no-win proof remain read-only/unscored and never manufacture a canonical conclusion; positive existential witness is not a minimax score.
3. Measure Xiangqi representative positions before setting conservative default/maximum limits. Report only completed iterations, all actual visits and native root perspective; no fallback choice without completed work.
4. Add actual Xiangqi result family and bounded limits to shared transport3/real internal-search consumer; retain strict raw parsing, complete context/purpose/target/IDs, supervisor cancellation/replacement/cleanup and explicit unsupported Chess Full. Make only compile/compatibility adaptations in desktop, not a Xiangqi desktop consumer.
5. Independently specify and execute arithmetic/tactical/history/warning/exact/stop/restoration fixtures and actual child/supervision tests; review completed search/boundary diff independently, run focused plus normal native/desktop regressions, integrate/push and required CI, then complete97 and stop.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Root@codex primary builder/integrator on sole primary codex/rewrite after96Done/integrated/pushed6c37398. Re-read84/95 dependencies: bothDone; recorded graph unchanged. Existing native consumer contract and immutable WXF profile remain authoritative. No external protocol100 or desktop98+ implementation, strength54/complete45/M4/review106 remain future. Backlog1.55.1 pinned CLI owns mutations.

Native evaluator/search and real structured child implemented. Independent fixture authors: console96_tests native11 then integratorcapacity12th; engine97_tests routing7/handoff6/internal6 then integratornormalchildcancel7th. All32focusedtests PASS, independent reviewer reran32 and release workload; C11 directly proves warning/history changes best action with native capture/penalty witnesses. macOS27.0.1 AppleM1Pro Rust1.89 release: default1/128 initial45visits~24ms; max3/512 initial512visits retaineddepth1~306ms; representativecases+exact255closure/Unknown in docs/m4/xiangqi/engine-evidence.md. Ordinaryfastonly and explicit bounded root-exact read-only distinction documented. No scorecache. Workspacefmt/Clippy-Dwarnings/test--workspace--locked PASS; Node24.19 TScheck+161frontendtests PASS. Final docsreview/nativehostchecks/CI/integration pending; no Done claim.

Final separate reviewer console96_review: no unresolved findings,33 distinct focused tests independently PASS plus release workload. docs/m4/xiangqi/engine-reviews/msh-97.md records method, actualcommands and resolvedverificationgap (C10 alone insufficient; native C11 proves actionchange). Minor docs reconstruction wording fixed. Additional deterministic fixture PASS (45visits/+400/nativeb2b9tie/repeatedequalresults). Native session allfeatures93tests/Clippy and frontend snapshotregenerationunchanged PASS. Tauri fmt/Clippy/test/checkdesktop-verification PASS with existingvendoredTao warnings unchanged. Backlogdoctor/diffcheck PASS. Onlydevserde_json added for frozenfixture input; recordmetadata is derived/strictvalidated, no wire/schema change or scorecache. Ready for source push/native and desktop CI;97 notDone yet.

Integrated/pushed tested source14b981fd05fa56e6a4e7adf2834ea3e7aa222191 on codex/rewrite. Required Rust workspace CI40661642370 PASS macos-15/windows-2022/ubuntu-24.04, including new33focusedcases and existing regressions/QA/snapshots/build/smoke: https://github.com/jweathers777/mShogi/actions/runs/40661642370 . AC1 native13fixtures+transparentcontract; AC2 actualchild7/context/strict/cancel fixtures; AC3 deterministic/arithmetic/tactical/history/exact/stop/capacity+measurements; AC4 separate reviewer33PASS+allworkspace/desktopregressions. Supporting desktop CI40661642327 passed frontend/nativechecks onall3 and is building packages; await final result before Done. No substantive source changes after reviewed integration.

Supporting desktop CI40661642327 attempt1: macOSPASS; Linuxnormalpackage/smoke/native/frontendPASS,13/14GUIphasesPASS, verification-shogi-concludedfailed onread-onlyWebDriver execute/sync afterReturntolive (RemoteDisconnected/connectionreset104). Retained artifact12803719557 and job122725848693: https://github.com/jweathers777/mShogi/actions/runs/40661642327/artifacts/12803719557 . Failurecapture/DOMaftertransporterror showappalive/correctLiveconcludedstate/event1; followingcapacityphasePASS. No productionissueestablished; separate reviewerinspecting beforejustifiedunchanged-source jobrerun. Preservefirstfailure; no testcriteria/harnesschange. Windowsstillrunning;97InReview until requiredremainingverificationresolved.

Independent reviewer inspected firstLinuxfailure report/trace/driverlog/DOM/screenshot/narrowdesktopdiff: a single unchanged-source Linuxjobrerun justified by transientreadonlydrivertransport evidence; precisecauseunproven. MacandWindowspackagejobsPASS attempt1. RequestedonlyLinuxjob122725848693 rerun atsame14b981f, no source/harness/criteriamutation. Assessnewresultseparately; retainattempt1failure.

FINAL CI: source14b981fd05fa56e6a4e7adf2834ea3e7aa222191 unchanged. Rust40661642370 all3PASS. Desktop40661642327 attempt2 all3PASS: Linuxjob122732132161 reran unchanged checks and passed normal/verificationpackages plus all14actualinstalled/packagedChess/ShogiGUIphases; Macpackage/smoke and Windowspackage/actualGUI retained successfulattempt1evidence. https://github.com/jweathers777/mShogi/actions/runs/40661642327/attempts/2 . FirstLinuxread-onlydrivertransportfailure remains accurately recorded above; successfulrerun does not establishprecisecause. No harnessweakening or productionfixneeded. All4AC/4DoD supported, finalseparate review33PASS, no unresolvedfindings. Production scope stops97;98 and100 dependencies now satisfied but remainToDo/unstarted;45/M4open,106unstarted. Primarycodex/rewrite retained; no temporaryworktreecreated. Completion metadata only follows tested source.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed transparent Red/Black material+Soldier-advancement evaluator and deterministic full-width native-history minimax, complete board/count-claim choices, native outcome/pending handling, private balanced undo and canonical root restoration, no scorecache. Defaultdepth1/128visits, maximumdepth3/512 from representative M1Pro release measurements. Ordinary sound_fast_only preserves fastUnknown without declaring winnability; explicit bounded required root-exact entry retains original native observation, negativeproof read-only/unscored, positivewitness notforcedwin, Unknown/cancel/failure unscored. Real transport3 internal-search now advertises only Xiangqi EngineTurn/MoveAnalysis, preserves fullcontext/purpose/IDs/limits/stale protections; ChessFull unsupported. Native13 + handoff6 + routing7 + actualchild7 =33 independently reviewed/executed focusedtests; allworkspace/Clippy, frontend161, native93allfeatures/snapshots and Tauri checks PASS. Tested/integrated/pushed source14b981fd05fa56e6a4e7adf2834ea3e7aa222191; native40661642370 and desktop40661642327 final all3platformsPASS. InitialLinuxWebDrivertransportfailure preserved with independent investigation and unchanged-source successfulrerun. Functional M4 baseline, not strength54; desktop98/99/101/102 and externalprotocol100 remain future. No material decision/blocker remains for97;45/M4stayopen.
<!-- SECTION:FINAL_SUMMARY:END -->
