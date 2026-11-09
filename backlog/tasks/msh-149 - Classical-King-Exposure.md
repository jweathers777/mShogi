---
id: MSH-149
title: Classical King Exposure
status: Done
assignee:
  - '@codex'
created_date: '2026-11-09T20:38:00Z'
updated_date: '2026-11-09T21:45:00Z'
labels:
  - handle:classical-king-exposure
milestone: m-5
dependencies:
  - MSH-145
documentation:
  - docs/m5/chess-effectiveness-diagnosis.md
priority: high
ordinal: 154000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Classical King Exposure. MSH147 shows queen/rook historical judgments changing materially with deeper search and identifies generic king danger fading to zero at the endgame phase despite remaining mating material. Investigate material-aware king exposure, supported checks and defensive resources against independent varied positions; select supported general classical corrections without overfitting a match loss or copying upstream weights. Preserve native tactical/adjudication authority, history, finite work and product contracts. This is part of the coherent conventional round before fresh adaptive reassessment; no per-technique tournament or learned evaluation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Independent tactical/positional controls and transparent term analysis establish the applicable king-exposure deficiency and negative cases, distinguishing heuristic judgment from native mate or outcome proof.
- [x] #2 Implement supported conventional evaluation/search interaction corrections with exposed-king, safe-king, material-exchange and defensive-resource counterexamples, preserving restoration/cancellation and native authority.
- [x] #3 Level1 local correctness, bounded realistic-time comparisons and independent review support integration; report decision-quality evidence separately from playing strength and carry the coherent round to proportionate adaptive reassessment.
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
Root owns integration on codex/rewrite after145accepted4f4fa99. Current king_safety end component is zero, globally fading pressure even when queen/rook checks remain. Freeze independent supported/unsupported queen and rook checks, native defensive resources, safe/blocked/no-major controls plus145oracle/tactical panel. Delegate isolated evaluator/tests: retain exactmiddle/shelter/storm/globalphase and add a modest capped end danger using existing ring/safe-check geometry, including supportedcontact before countingcap, only with participating enemymajor. No leaflegalgeneration, rules/history/searchpolicy/protocolchange. Verify exact component guards, mirror/turn/counter/incremental equality, native mate/claims/cancel, allsearchregressions and short frozen-source timedcomparisons. Level1 portableevaluation; independent source/evidence review, no matches or platform/GUIcampaign. Then151freshadaptiveassessment ofcombinedround.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Tested639b6465be384330c9fd7d49580c13bb0c314bf2: bounded128cp enddanger fromparticipatingmajor ring/safechecking pressure,supportedcontacts beforecap; middle/shelter/globalphase unchanged.103release searchtests pass2ignored,workspaceclippy/fmt;28conditions retain16oracleWDL,3publishedtactics,newmates/defence,allnativePV/root. No new solvedwin or strengthclaim; drawnrookending scores remainoptimistic. Independent source/testreviewapproved, finalreceipt reconciliationpending. Failedtestexpected84omittedpawn6; correctedtest90, unchangedproduction, historicalreceiptretained. Level1,no matches/platform/GUIcampaign.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Integratedcodex/rewrite tested639b6465be384330c9fd7d49580c13bb0c314bf2; finalreview/evidence accompaniescompletioncommit. Participatingmajor enddanger andsupportedcontact distinction preserve middle/shelter/phase/nativeauthority; independentblocked/defended/no-major/exchange/matecontrols.103searchtests pass2ignored,workspaceclippy/fmt;28conditions preserve16WDL,3tactics,newmates anddefence,allnativePV/root. Sourceandfinalreceipt independentreviewapprovednoissues. Historicaltestarithmeticfailure retained. Drawrookcalibration remainsincomplete; historicalmovechangesnotprovedbetter; no strengthclaim. Level1localmacOS27arm64; no matches/platform/GUIcampaign. docs/m5/chess-king-exposure.md,evidence/msh149/verification.json. Next151ChessCoherentRoundAssessment usesfreshindependentopponents afteracceptedround.
<!-- SECTION:FINAL_SUMMARY:END -->
