---
id: MSH-49
title: Establish and meet measured chess engine strength criteria
status: In Progress
assignee:
  - '@codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-08T08:45:00Z'
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
<!-- SECTION:NOTES:END -->
