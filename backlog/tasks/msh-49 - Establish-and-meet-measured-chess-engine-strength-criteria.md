---
id: MSH-49
title: Establish and meet measured chess engine strength criteria
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-08T10:03:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-48
  - MSH-25
  - MSH-109
documentation:
  - docs/design.md
ordinal: 49000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Functional chess search/play does not satisfy P-02. This ticket owns the chess-specific opponent pool, transparent baseline, release-strength criteria, search/evaluation improvement and measured verdict, using MSH-48 shared infrastructure. An initial learned evaluator improvement is separately owned by MSH-55; no requirement for an identical neural architecture in every game is inferred.

Open D-09 questions: establish meaningful chess opponents and starting positions, resource/fairness conditions, release-strength targets and uncertainty criteria from baseline/pilot evidence; record owner decisions on material targets before claiming success. Search tuning, evaluator choices and compute budgets remain empirical, not resolved by this planning ticket.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A reproducible chess baseline and suitable independent opponent pool are established, with profile compatibility, limits, openings/colors, failures and executable identities recorded. Unavailable external opponents remain explicit measurement limitations; self-play alone is not independent strength evidence.
- [ ] #2 Game-specific chess strength targets and comparison/uncertainty criteria are explicitly decided and documented from baseline evidence before a release-strength verdict. Numeric targets are not fabricated from other games or nodes-per-second results.
- [ ] #3 The mShogi chess engine has measured search/evaluation improvements and meets the agreed criteria in reproducible matches against the defined pool. Search correctness, legal complete actions, history-aware terminal behavior and regression evidence remain intact; speedups alone do not establish stronger play.
- [ ] #4 The final chess report records the tested revision, all experimental conditions and uncertainty, actual successes/failures and independent review. Open required targets block completion and M5 closure rather than being absorbed into optional M6 research.
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
Primary builder/integrator Codex, clean origin-aligned primary codex/rewrite at05eeba326341fb65efa0e60f2641795c5276e863;48/25/109Done, no active worker, historical-access checkout preserved. One bounded Chess strength assignment. (1) Research actual current full-width/material baseline, acceptedM2/48runner/protocol/pilot evidence, available independent opponents and reproducible fair paired starting distributions; use small bounded native measurements to inform conditions. Preserve exact baseline executable/source and allconformance/history/claim/terminal/cancel/no-fallback contracts. (2) Bring concrete release-strength target, uncertainty/stopping rule and material compute/opponent conditions to owner for explicit D09 selection from evidence before claiming success; continue independent bounded baseline/protocol/correctness preparation while pending. Do not invent an Elo target or treat dependency readiness as owner approval. (3) Within selected scope implement measured conventional Chess search/evaluation improvements using authoritative native Game, no board-only history cache or second legality/adjudication; retain a reproducible frozen baseline path, actual execution/console/product integration and truthful identities. Exact implementation follows inspected sources/pilots and approved criteria; no neural work orothergame changes. (4) Meaningful independent search/evaluation oracles and restoration/cancel/claim/history/terminal/strict-result regressions, actual paired multiple-baseline/independent-opponent matches with attrition and uncertainty, one evidence ledger per source, independent review. Required existing native3OSCI for changednative/execution source; fullGUI campaign only if affected appboundary requires it. (5) Complete original4AC/4DoD only when agreed strength criteria and actual verification support them, normal integrate/push; otherwise preserve unfinished status and exact unresolved target/measurement blockers. No49success inferred from tactical/perft/speed/selfplay alone; no50–55/104implementation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Pre-target common preparation: frozen clean production baseline05eeba3 built/captured with exactexeSHA; officialSF19 archive/exec hashes independently matched historicalpins andnotices retained in temporary storage only. Actual6fixedauthoredprefixpairs atcommondepth2/10knodes128plycap:0W2D10L,all12nativeconclusions,353completeddepth2requests,nofallback/process/truncation/cleanupfailures. Independent read-only review recheckedhashes/context and replayed6reports. Found/reproduced necessaryharness correction: valid depth1PV Move+terminalClaim wasrefused by total-command depthguard; countboardMoves<=depth plus atmostoneClaimtail with fullnativevalidation. Actualchild red/green andextraboardnegative+9admissionregressions+fullrunner+clippypassed; independentfixreview approved. docs/m5/chess-strength.md andevidence/msh49/verification.json retain baseline/correction/proposal; no search/eval candidate orstrengthacceptance yet. OwnerD09/AC2 decision pending concrete first-releaseproposal:64relativepairs70%+exactpairedp<=.05,32externalmatchedroots25%+positive/significantgain vsdepth2SFreference,equalmShogi10k/depth8ceilings,holdoutprotocolfrozen,2hourlocalmatchbudget,noratingclaim. No answer inferred. Alloriginalcriteria remainunchecked; taskunfinished. Commoncorrection frozen-source required3OSCI pending; no GUIcampaignneeded.

Common preparation/correction normally integrated+pushed at926ae8f60eee1a05f02fb7cff36b7bd1d924a992; task49 remainsInProgress withall4AC/4DoDunchecked. RequirednativeCI42464154960 running atthat exactsource withoneboundedcompletion-awareobserver (no retries); livecache/private/tmp/msh49/ci/{run,jobs}.json andobserverexecsession48956 are localresume conveniences, GitHubrun/source are durableauthority. UnrequiredDesktop42464154953 cancellationrequested because noUI boundarychanged. Explicitownerproposal nowin docs/m5/chess-strength.md, latestasyncquestion supersedesbroad directionquestion. Noresponse/approvalreceived: stopdependentcandidate/strengthwork untilownerselectsbar/budget; originalAC2/AC4+D09requirethismaterialdecision. Onresume reconcileCI+ownerreply, preservebaseline05e andallactualcalibration/admissionreceipts. Do notmarkDone fromcommonfixorpilot. Noother taskbegun.

OWNER DECISION2026-11-08: John says “Okay. Let’s proceed with your first option for question2.” This explicitly selects Approve this initial bar and match budget from the concrete question, superseding priorpending/differentcriteria state. Ratified:64heldoutcandidate-vs-frozenbaselinepairs score>=70% andone-sidedexactpairedsignp<=.05;32heldoutmatchedexternalroots eachparticipantplaysbothcolors against pinnedSF19depth2, candidate score>=25%, positive mean gain andone-sidedmatchedsignp<=.05. mShogi10kvisit/depth8ceilings; Stockfishdepth2/10khandicapexplicit; nohumanElo. Freeze sampler/test rulesbeforecandidateevaluation, retainallfailure/incomplete/fallbackevidence andrequirecompleteeligiblegames. Up to2hlocalmatchwork, stopnewbatchesatbudget,no paidcompute. Ordinarytests/review/requiredCI separate. Resumeone49assignment; no50–55/104work. Newisolatednativecandidate module delegated, rootownsprotocol/harness/integration. Initial implementationexactalpha-beta+transparentpositionalterms, noTT/boardonlyreuse/selectivepruning/neural/qsearchbeforemeasuredneed. Freezeindependentlydrawnroots protocol thenuseonlyseparateddevelopmentdata untilfinalcandidatefreeze; formalholdoutone fixedstudy, nooptional favorablep stopping.

Approved protocol/sampler/dev roots frozen and pushed at7004d47 before candidate matches. Development only: initial exact alpha-beta/transparent positional candidate v1 won8/8 vs frozen baseline; candidate vs pinned depth2SF19 scored37.5%(1W1D2L-equivalent halfpoints3/8; actual per-game results retained in source reports), frozen baseline0/4; all eligible. Charged local match work881.281s including54s calibration. No formal seed/roots generated and no acceptance inference. Runtime attribution shows immutable baseline search cost dominates; native available_claims exact necessary-condition shortcut independently proved by2reviewers and controlled complete-catalog validator/history/undo/replay regressions; no rule/effect/cache/schema change. Per-participant UCI limits and replay actor checks pass complete runner suite incl21historical games. Actual internal/UCI candidate routing tests pass. Final deterministic capture/promotion ordering is being evaluated on development only to improve bounded play; no new target/budget. Original claim-admission correction926ae8f required nativeCI42464154960 actually passed all3 jobs (macOS128679669239/Linux128679669333/Windows128679669348), attempt1, exactsource reconciled live. Final candidate requires its own exactsource3OS receipts. Task remains unfinished; original AC/DoD unchanged.

Clarification of previous score-only shorthand: actual v1 external candidate results are0W3D1L (37.5%), baseline0W0D4L. The earlier 1W1D2L-equivalent wording described an equal point total, not observed wins; use the actual0W3D1L source receipts.

Finalpreholdout source review approved(no material findings): capture/promotion-first exactalpha-beta with unchanged transparent evaluator; native claim-query prerequisite shortcut; actual shared UCI/internal routing; participant-specificlimits/replay; strict offlineconfig/artifact/work/budget/inference audit. Focused/native relevant regression checkpoint: search+engine154pass/1ignored; actualinternal/shared/UCI/claimconsole20pass; runner80pass/2ignored incl21historicalgames; nativeadjudication/history14pass; lint; regeneratedactualChessanalysisfixture and36frontendtests+typecheck. Finalcandidate devfirstroot2/2wins90.206s(vs168.570sv1); totalcharged971.486s. Exactdevelopmentreports/sourceoverrides/exehashes andonecanonicalexecutionledger retained underdocs/m5/evidence/msh49. Existing2hourbudgetpointforecastinsufficient despiteimprovement; askedownerwhetheramendonlytotalbudgetto4hours BEFOREholdout. No answer/amendment inferred, noformal seed/rootsgenerated. Continueclean-sourceintegration/build/required3OSCI whilepending; noheldoutworkdependentonamendment untilreply.

Normal source checkpoint integrated+pushed7818c3371758cee8e9624b58b0edb1073c32c89d. Clean releasebuild andall3artifactSHA frozen in study/candidate-freeze.json BEFOREanyheldoutseed; newcandidateSHA9ba4dbd6..., finalrunner9c8598ad... distinctfromdevrunner. Finalsampler reproduces12frozendevroots byte-identically;2samplertests pass. Independentfreshread-onlyprovenance reconciliation approved all9devpairs/source/artifacthashes andexact971.4862487493083s ledger, noformal/inflight. Required nativeCI42469534590 exactsource attempt1: macOS15job128696846118, Windows2022job128696846383, Ubuntu24job128696846507, allrunning/acquiredrunners. Oneboundedcompletion-awaremonitor85021 (/private/tmp/msh49/monitor-final.py;ci-final/{run,jobs}.json), checkcompletionbeforeanotherwait; no retrycampaign. UnrequiredDesktop42469534578cancellationrequested. Ownerbudgetamendmentquestionpending:4hTOTALincluding971.486sspent, onlybeforeholdout; original7200sgate remainsbinding, noapprovalinferred. ResumeCIactualreconciliation plusownerreply; nevergenerateholdoutonunapprovedamendment, weakencriteria orclaimDone. Source/evidence in docs/m5/chess-strength.md +singleverificationledger; noothertaskbegun.
<!-- SECTION:NOTES:END -->
