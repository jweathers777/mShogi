---
id: MSH-49.2
title: >-
  Establish an initial conventional Chess diagnostic baseline against
  unrestricted Stockfish
status: Done
assignee:
  - '@codex'
created_date: '2026-11-08T13:07:00Z'
updated_date: '2026-11-09T11:16:00Z'
labels:
  - handle:stockfish-baseline
milestone: m-5
dependencies:
  - MSH-49.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/design.md
  - docs/m5/chess-conventional.md
  - docs/m5/evidence/msh49-1/verification.json
  - docs/m5/chess-unrestricted-comparison.md
  - docs/m5/evidence/msh49-2/verification.json
parent_task_id: MSH-49
priority: high
ordinal: 136000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner decision 2026-11-08 supersedes the unapproved 32-game formal proposal. Run an initial diagnostic comparison of 8–12 games using paired openings, equal one-second thinking time and full-strength Stockfish 19, within 30 minutes total match work. Choose eight games/four familiar authored opening prefixes using engineering judgment; use the existing runner and pinned accepted conventional engine. Report results, observed nominal/selective depths, performance limitations and representative loss mechanisms with exact positions/moves and honest attribution. No significance test, confidence interval or Elo estimate. Preserve reproducibility, failures and unscored caps without adding a new experimental framework. After this baseline, continue conventional Shogi/Xiangqi development; deeper Chess strength analysis returns during learned-evaluation research. No repeated small Chess benchmarks or invented numeric release bar. Original 64/32 depth-2 study and later unapproved 32-game/one-hour proposal remain historical, not executed.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The accepted MSH-49.1 conventional engine and exact source/build/executable, Stockfish 19 default network/options and host resources are pinned. Both receive one second per move with no external depth/node/skill/Elo handicap, one search thread each and no pondering; memory policies remain explicit.
- [x] #2 The owner-authorized initial diagnostic uses 8–12 games with paired openings and at most 30 minutes total match invocation work, including prior setup smoke and failed attempts. Routine details are selected autonomously and recorded before execution; no statistical significance, confidence interval or Elo study is performed.
- [x] #3 Actual games and all failed/capped/incomplete attempts retain native histories, process/cleanup diagnostics, exact effective controls and reproducible receipts. Native replay supports the observed results; failed or capped work is never silently scored, replaced or discarded.
- [x] #4 An independently reviewed diagnostic report records W/D/L, actual search depths/work, practical performance limitations and representative reasons for losses using concrete positions/moves. It distinguishes observation from inference and establishes the initial baseline without a numerical release-strength claim; handoff prioritizes Shogi/Xiangqi, with deeper Chess analysis deferred to learned evaluation.
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
Resume Codex ownership in clean origin-aligned codex/rewrite at47f2383; compatible Backlog1.55.1, unrelated historical worktree preserved. Owner explicitly replaces 32-game formal proposal with8–12game/1s/30min diagnostic. (1) Record decision and proportionately revise task/parent/roadmap before results; keep production engine and existing runner unchanged. Use eight games across four familiar authored prefixes (Open Game, Queens Gambit, English, French), reversed colors. (2) Narrow existing offline launcher to diagnostic mode, 1000ms-only requests, no statistics/sampler campaign; retain exact pins, one budget ledger incl prior5.8404s smoke, bounded cleanup and native replay. Focused offline controls plus bounded read-only delta review; commit reviewed inputs before games. (3) Run four pairs once sequentially, stop on failure/budget, retain actual receipts. Extract nominal/selective depth, nodes/time and native move histories; inspect representative loss positions through authoritative native console, without extra engine matches or tuning. (4) Independently reconcile source/controls/charge/results and bounded diagnostic observations, reuse unchanged native platform evidence, complete criteria only when supported. Update parent49 acceptance/handoff to owner-selected diagnostic checkpoint, make conventional Shogi/Xiangqi next; no implementation of those tasks here. Normal commit/push, no broad GUI/CI campaign or further Chess optimization.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
MSH-49.1 handoff: conventionalengine implementation eb4601d0fd33e790ac9905309fb011400aeafb5c and fixture-onlycorrection/testedsource b8cc85167f85eb031830d5f5268c1f94520f8a25 integrated codex/rewrite. RequirednativeCI42492362033 attempt1 passedactualUbuntu24/macOS15/Windows2022; independent source/fixture/provenance reviews approved. Canonicalengineeringevidence docs/m5/evidence/msh49-1/verification.json distinguishes1134localtests/affectedconsumers/profiles from49.6924s selfplay (one native mate, oneunscoredplycap), no strengthestimate. This formalstudyhasnotbegun, noopponenthandicap/threshold/budgetapprovalinferred. PreserveToDo/unassigned untilconcretecomparisonconditions/materialbudgetareselected withJohn perAC2; routinepreparationmaymakeproposalreviewable butmustnotlaunchformal/substantialmatches. Old64/32/depth2SFstudy remainsstopped; do not revivethatprotocol or4hourrequest.

Ownership: @codex, primary codex/rewrite at 0b3b4585c48619f1c9781eb3e623fb6d873342bc, clean and exact origin alignment. Backlog 1.55.1 matches baseline. No Ready dependency-satisfied task exists; next planned MSH-49.2 has completed dependency 49.1 and is activated for concrete authorized preparation. Owner approval of formal study budget/comparison remains mandatory AC2, not inferred from this task assignment. No formal or routine matches have run in this task; routine charge starts 0/900 seconds.

Preparation delivered by Codex at clean origin-aligned0b3b458, primary codex/rewrite; no production changes. Concrete reviewed proposal docs/m5/chess-unrestricted-comparison.md +canonical protocol/build/verification at docs/m5/evidence/msh49-2. Proposed16iid opening pairs/32games, equal go movetime500 only (no externaldepth/node handicap), full SF19 defaultnetwork,1searchthread each/no pondering, same M1Pro32GiB. Memory explicitly unequal/modest: native4096entryTT vsSF16MBHash+98.5MBnetwork. Proposed3600s total active local matches inclusive smoke/failures/cleanup; no paidcompute/vendor fees; stopnewpair<600s remaining, shared5s outercleanup reserve, incomplete/failure remains unscored/no replacement. Proposal accepts complete independentlyreviewed measurement regardlessscore, not numerical releasebar/parity/Elo. OwnerAC2 decision remains mandatory; no formal seed/roots/freeze/results or approval exists. One routine smoke consumed5.840419957879931/900s with8 actual go movetime500 requests,2intentional4plycapsunscored, cleanprocess/no stderr/failures andnative replayexit0. Fresh independent msh492_protocol_review approved setup/protocol/provenance; all5artifacthashes/sourceequivalence and7offlinecontroltests independentlyverified. Foundnestedcleanupwindow issue corrected toone shared5s deadline, controlled timeout test verifies no renewedwindow andhonestunsettledcharge. Actual negative formal invocation refusesbeforeanyengine/rootdraw. Existing acceptednative3OS evidence reused atb8cc851; no Rust/UI/productchanges ornewCIcampaign. ReturnToDo/unassigned solely pending material ownerbudget/interpretation decision, all4AC/DoD unchecked. Upon approval recordactualreply+exactprotocolSHA, reactivateownership, commit decision then freeze/draw/run once; do not revive old64/32/depth2study. Prep integration/push follows, hashes inGit history/completion report.

OWNER DECISION: latest reply explicitly authorizes an initial diagnostic of8–12games, pairedopenings, equal1second, unrestrictedStockfish19 and30minutesTOTALmatchwork. Rejects unnecessary32-game formalstatisticalstudy, significance/Elo and further small Chess benchmark loops. Initialconventionalbaseline should handoff toShogi/Xiangqi; deeperChessanalysis duringlearnedevaluation. Select8games/four familiarprefixes, no furtherapprovalneeded. Prior5.840419957879931s smoke stayschargedwithin1800s total; no diagnostic games yet. This supersedes pending exact32game/500ms/3600s proposal, not an approval of it. All4AC replaced prospectively to match explicitowner direction before observing results; allremainunchecked.

Diagnostic preexecution delta approved independently (msh492_protocol_review):6focusedoffline tests anddiffcheck pass;8plannedgames/1000ms/fullSFsettings/1800saggregateinclprior5.8404s verified, no statistical/samplerbranch. Four familiar authoredprefixes, max256plies healthycapsstayunscored, failurestops/no replacements,60snewpairfloor/shared5scleanupreserve. Productionengine/runner/network unchanged; exacthashes reused. Commit reviewedstaticinputs thenexecuteonce; actualresults andlossdiagnostics stillpending. No furtherownerdecision required.

Reviewed owner-authorized diagnostic inputs integrated/pushed cc2ec7c (exactfreeze in evidence/msh49-2/freeze.json). One sequential launcher running execsession23859; local log/private/tmp/msh492/diagnostic.log, canonicalverification.json tracksinflight/perpaircharges. Fourpairs only,1000msonly controls,1800stotalinclprior smoke. No concurrentcampaign/retry/tuning; do not start another while thisinvocation is active. Productionbuildsource0b3b458 andsameengineSHA4250eb9e remainunchanged fromaccepted49.1. Finishactualgames, then inspectdepths/losspositions andindependentfinalreview; allACremainunchecked.

All8 diagnosticgames completed at frozenexecution sourcecc2ec7c:0W0D8L, allnativeStockfishcheckmates, no failed/unscored/cappedgames, stderr/truncation/cleanupissues. Fourpairs onepass, total494.88940828992054s includingprior5.840419957879931s smoke (diagnostic489.0489883320406s), no inflight/no furtherbatches. All522requests are go movetime1000 only. Native replaypassed4pairs and independentlyrepeated4pairs; exactartifacts/config/hash/freeze/sourceequivalence reconciled. Descriptive cp-turn nominaldepthmedians5(mShogi) vs22(SF), selective18vs36; countersengine-specific, SFmate-depth245excluded. Report docs/m5/chess-unrestricted-comparison.md documents2nativeverified kingattack/materialloss sequences andlimitations. Updatedroadmap/ownercriteria beforegames, no significance/interval/Elo. Finalreport/4AC+DoD independentacceptance pending; then normalintegration andparent49/handoff50/54. No productionchange/newCI orfurtherChesswork.

Final independent acceptance APPROVED (msh492_protocol_review) all4owner-rescopedAC/4DoD andparent49: raw+inner hashes/9compressedartifacts/freeze/source/resourcepins/all522go1000controls, fourindependentnative replaypasses, exact494.88940828992054s/noinflight,8actualnativeSFmates/zero failures andfaithful diagnosticreport reconciled. Finaltable/aggregatecp-depths/engine-specificcaveats andtwo nativeverified losssequences supported; no significance/interval/Elo, firstblunderproof orsingle-technique causalclaim. Allcriteria checked; Done supported. Executioninputs normallypushedcc2ec7c, build0b3b458/unchangedengine4250, priornativeacceptanceb8cc851 reused. Finaldocs/evidence/task normalintegration/push follows; hash inGit history/report. No furthermatches orothergameimplementation. Parent49baselinecomplete;50Shogi/54Xiangqireadyengineeringnext, deeperChessanalysisduring55.

Stable task handle: Stockfish Baseline. Use MSH-49.2 (Stockfish Baseline) in new conversational reports and development summaries. Historical titles/evidence/status unchanged.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the owner-selected initial conventional Chess diagnostic: eight games from four familiar paired openings, equal one-second thinking time against full-strength Stockfish19, no external depth/node/skill handicap. mShogi scored0W/0D/8L; all games were native checkmates, with no failed/unscored diagnostic games or process/cleanup issues. Exact total match work494.88940828992054s (8min14.9s), including the earlier5.8404s unscored transport smoke, stayed within the authorized1800s. No further matches or tuning ran.

Observed ordinary cp-turn median nominal/selective depths were5/18 for mShogi versus22/36 for Stockfish; node/depth accounting is engine-specific. Native inspection supports representative late recognition of kingside attacks, a queen-for-bishop concession and a favorable-looking rook capture followed by mate. These diagnose a substantial practical gap without isolating a technique or claiming significance/Elo.

Exact execution sourcecc2ec7c, pinned build0b3b458 and unchanged accepted engine SHA are recorded in docs/m5/evidence/msh49-2/verification.json. Four native pair replays and four independently repeated replays passed. Six focused offline controls and independent setup/final results/report review approved all four AC and four DoD. Production code is unchanged, so prior native3OS acceptance atb8cc851 is reused. docs/m5/chess-unrestricted-comparison.md is the report. Final evidence-only integration oncodex/rewrite is identified by Git history and the completion report. Parent49 establishes this initial baseline; Shogi50/Xiangqi54 follow, with deeper Chess analysis deferred to learned evaluation55. M5 and other-game work remain unfinished.
<!-- SECTION:FINAL_SUMMARY:END -->
