---
id: MSH-136
title: >-
  Profile and correct major conventional Chess search costs before cross-game
  transfer
status: Done
assignee:
  - '@codex'
created_date: '2026-11-08T19:52:00Z'
updated_date: '2026-11-08T20:53:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-49.2
documentation:
  - docs/m5/chess-search-costs.md
  - docs/m5/evidence/msh136/verification.json
priority: high
ordinal: 138000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Owner requests a focused bounded engineering-performance checkpoint after completed49.2 and before transferring conventional search to Shogi/Xiangqi. Profile native queries/transitions, quiescence/exchange evaluation and effective TT reuse; correct significant practical architectural inefficiencies while preserving authoritative rules/history/claims/outcomes and exact undo/cancellation. No additional Chess matches, strength/statistical program, extensive benchmark, speculative optimization or universal redesign. Escalate only major architectural tradeoffs; carry lessons into50/54.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Bounded reproducible release profiling on a small fixed representative position set identifies native query/transition, quiescence/exchange and actual TT eligibility/reuse costs, with exact source/commands/workload/environment and no opponent matches or strength inference.
- [x] #2 Measured significant practical inefficiencies are corrected or justified within existing native/search ownership. Rules, complete actions, profile/history/claim/terminal semantics, collision-safe TT context, exact state/evaluation restoration and bounded cancellation remain intact; no duplicate legality/adjudication.
- [x] #3 Focused independently expected/comparative regressions and proportionate fixed-work before/after checks support correctness/performance. Relevant native regressions/lint, independent review and actual required Linux/macOS/Windows CI approve exact corrected source; reuse unaffected GUI evidence.
- [x] #4 Concise report/single structured source record preserve outcomes/limits and transferable lessons; this checkpoint precedes50/54 without reopening49.2 or repeated Chess studies. Normal integration/push completes checkpoint and releases roadmap continuation.
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
Codex primarybuilder, clean origin-aligned codex/rewrite b005f80, Backlog1.55.1, no competing worker; preserve historical-accessworktree. Sourceinspection: SEE recursively generates full legal lists thenfiltersdestination; fullhistory TT deliberatelyconservative, qnodesdominate. (1) Short fixed-work release measurements and nativeCPU sample, atmostfew four-position passes; inspect counters fornative/SEE/TT, no matches. (2) Correct measured redundantwork through authoritative native queryprojection/narrow searchchanges; compare same actions/score/PV/restoration when behaviorunchanged. Separate TT board/context/depth eligibility; preservefullhistory unlessmajortradeoff requiresowner. (3) Focused regression/undo/history/cancel/exchange tests, independentreviewearly/final, coherentnative regression/lint andactual3OSCI withonecompletionawaremonitor; no broadGUIcampaign. (4) Report/sourceledger andhandoff50/54, normalcommit/pushDone onlyifsupported; thencontinue roadmap.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Measured checkpoint: short CPU sample identified exchange work~40%, native queries~39%, transition/undo~16%. Native destination projection now filters before unchanged checked resolution; SEE selection/order unchanged. Borrowed TT/PV and deferred evaluation/action copies preserve native terminal/claim precedence. Twelve fixed-work paired rows match in every field except elapsed time; developed~1.995x, standard~1.171x, recapture/endgame essentially unchanged. Total profiling producers72queries/27.43s, zero matches; no further profiling planned. Native hand/special-move/counter tests3pass; conventional/evaluator focused19pass/1ignored. Independent narrow review approved, exact overflow assertion and original example formatting/SHA provenance resolved. Canonical report/record linked; workspace tests/lint running47022, required3OS exact correctedsource CI notstarted. No major owner tradeoff; keep full-history TT bounds. Handoff50/54 carries native subset queries/caching and staged TT measurements.

Coherent localnative checkpoint passed: cargo test --workspace --locked1138pass/0fail/11intentionalignored across102groups; workspacealltargetsClippy -Dwarnings passed; formatting/diffcheck clean. Independent source/fixedwork/report/provenance review approved; original baseline example bytes+formattingproof, all3sampledproducerreceipts andup-to40propertycoverage wording resolved. Exact12row output/work equivalence preserved; zero matches andno furtherprofiling. Report/structuredledger ready, normal correctedsource commit/push andrequiredactualRustUbuntu24/macOS15/Windows2022CI next. All4AC/DoD remainunchecked untilactualplatformreceipts/finalreview. No IPC/schema/UI consumers changed; packagedevidence reused, no newGUIcampaign.

Correctedsource normallyintegrated/pushed d9147713aa5ad5044a4e25a41d5142f6834a2685. RequirednativeCI42519269011 attempt1 running atthatexactsource; oneboundedcompletion-awaremonitor81811 (/private/tmp/msh136/monitor.py, cacheci/{run,jobs}.json), no retries/overlappingcampaign. AutomaticDesktop42519269077 cancellationrequested asunchangedUI/IPC evidence reused. Sourcechangesstop; remaining136work isactualCI+finalindependentreceipt/ACreconciliation andevidence-onlycompletionpush. Localworkspace1138pass/0fail/11ignored,lintpassed;72queries27.43s totalprofiling,zero matches. Canonicalverification.json recordsallpins/provenance/outcomes. Dependency136 continuesblocking50/54untilDone; no nextimplementationstarted.

Final acceptance approved by independent msh136_review: actualRustCI42519269011 attempt1 exactd9147713aa5ad5044a4e25a41d5142f6834a2685 passed macOS128848966309/Linux128848966487/Windows128848966668, all39steps completed/success with actualrunners. Local1138pass/0fail/11ignored andlint/fmt; all17artifacthashes/source/formattingproof/all72queries27.4324s and12identical-recorded-work rows independentlyreconciled. Counterprecision/up-to40propertywording/3producerdurability findings resolved. All4AC/4DoD supported/checked, Done. Code normallypushed d914771; finalmetadata/report/taskcompletioncommit follows withactualhash discoverableinGit history/report. No sourcechange aftertestedcheckpoint, no furtherprofiling orChessmatchstudy. No majorarchitecturaltradeoff: retainfullhistoryTTbound key andcheckednativeeffects. Nativequery/transition dominance andconservativeTT reuse remainexplicit limits. Transferlessons linkedto50/54; dependencies nowreleaseconventionalShogi/Xiangqi roadmap, no othergameimplementation in136.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the bounded conventional Chess performance checkpoint. A short fixed-position release profile found redundant exchange work: every recapture step previously validated all legal moves before selecting one destination. Native legal_actions_to now filters existing candidates before the same checked resolver, preserving counters, king safety, EP, castling, promotions, ordering and failure semantics. Search borrows TT entries, clones PVs only on usable cutoffs and defers pure evaluation/action copies past safe cutoffs; full-history TT identity, terminal/claim precedence, rules, budgets and execution schemas remain unchanged.

Three before/after rounds on four fixed20k-visit/depth8 workloads produced identical recorded results/PVs/depths/node/SEE/TT counters. Median developed-position time844.6→423.3ms (1.995x); standard444.8→379.8ms (1.171x); recapture/endgame essentially unchanged. All profiling producers total72queries/27.4324s, including a preserved denied sampler attempt; no additional Chess match experiment or strength study ran. Remaining native queries/transitions and intentional full-history TT limits are documented, not claimed optimal.

Local workspace1138pass/0fail/11intentionalignored, all-target Clippy andformatting passed. Exact corrected source d9147713aa5ad5044a4e25a41d5142f6834a2685 passed required actualLinux/macOS15/Windows nativeCI42519269011 attempt1. Independent source, data/provenance andfinal platform review approved all4AC/4DoD. docs/m5/chess-search-costs.md andsingle docs/m5/evidence/msh136/verification.json preserve source overlays, original/formatted example hashes, CPU samples, all producer receipts, successes/failures and actual CI. Final evidence-only integration oncodex/rewrite is identified by Git history/completion report.

MSH49.2 remains accepted unchanged. Existing50Shogi/54Xiangqi dependencies nowrelease, carrying native subset queries/cached turns, early transaction-cost measurement and stage-specific history-safe TT profiling. Shogi currently retains full before/after Games for undo; measure that before scaling, without bypassing native declaration/repetition/drop rules. Xiangqi pending/no-win/claim guards remain authoritative. No major owner decision was required, and no Shogi/Xiangqi implementation or broader M5 closure is claimed in136.
<!-- SECTION:FINAL_SUMMARY:END -->
