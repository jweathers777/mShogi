---
id: MSH-67
title: Implement shogi history adjudication and canonical game-match records
status: In Review
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T12:48:00Z'
labels:
  - m4
  - owner-msh-41
milestone: m-4
dependencies:
  - MSH-66
  - MSH-63
references:
  - >-
    backlog/tasks/msh-41 -
    Deliver-complete-shogi-rules-and-functional-product-integration.md
documentation:
  - docs/m4/shogi-profile.md
  - docs/m4/delivery-plan.md
type: task
ordinal: 67000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implementation ownership: MSH-41. This bounded ticket supplies part of that preserved complete-delivery record; it does not replace its original acceptance obligations. See docs/m4/delivery-plan.md for scope traceability and dependency order.

Verification expectations: Profile clause-to-fixture ledger including declaration thresholds/evidence, continuous-check intervals, imported history and exact terminal/length precedence; strict canonical replay and rejection tests.

Planning-only batch MSH-62 creates this future work; no implementation or game-delivery completion is authorized by creation. Detailed implementation plans belong in the active task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Rules-owned history separates saved-state,repetition and search identity; repetition includes promoted ownership,both hands and turn, with complete checking intervals and explicit known-history origin.
- [ ] #2 All selected-profile conclusions and precedence are implemented: fourfold/perpetual check,no-legal-action loss,resignation,formal entering-king declaration and declared length policy; advisory probes,illegal API rejection and tournament penalties remain distinct.
- [ ] #3 Canonical native game/match records preserve initial state,complete actions,commands/events,profile/edition,modes,limits,policies,counters,conclusions and score/replay semantics; strict replay validates independently expected history and rejects unsupported/lossy input.
- [ ] #4 Imported-SFEN counter/history semantics and overlapping boundary fixtures are resolved under the ratified profile; completing a history-dependent losing cycle is not erased by generic invalid-request rejection.
- [ ] #5 History/record roundtrips,undo,prefix reconstruction and same-position/different-history cases are independently verified. Timing or external-event policies are either explicitly supported or clearly unsupported,never inferred from chess.
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
Resume f259bb2 and reconcile newer9325c48 handoff without replacing working history/record features. Preserve original failed proof review, source research and CI failures. Inspect and checksum supplied supplement, run both necessary-geometry enumerators and check all exclusion constraints/case accounting. Obtain fresh independent complete-chain proof review for the locally admitted live-root domain, including drops/captures/promotion/source changes and semantic multisets. Investigate any precise failed implication. If accepted, resolve current HIST-INTERVAL documentation and replace provisional mutual rejection with detailed internal invariant diagnostics, preserving broad Position analysis and explicit imported roots. Run affected focused tests and relevant regressions, obtain fresh final implementation review, integrate/push codex/rewrite and normal native CI, finish AC/DoD only with complete evidence, then archive temporary worktree. Stop before68/69/70 and keep41-47 open.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-26 SHOGI-P1 selected option A mshogi.shogi.csa36.v1 against integration6db8500. Profile/coverage docs now name HIST-INTERVAL (SH-F028–030), DECL-BOUNDARY (034–037), IMPORT-LENGTH (041) as concrete independently reviewed fixture/research gates before affected MSH-67 implementation. No implementation started by this note; existing dependency/acceptance scope preserved.

MSH-67 attempted 2026-10-26 by @codex in temporary codex/shogi-history worktree, based986a527. BLOCKED and returned To Do, all AC/DoD unchecked. Unmerged partial implementation preserved and pushed at f259bb23834d6156f21aeec53db9ee00ef7242e4 (codex/shogi-history); do not merge as complete. It contains GameRoot/live admission, exact native history/context/undo, selected untimed outcomes for established cases, strict GameRecord replay,15 new focused tests and independent review reports. Local macOS arm64 Rust1.89 cargo fmt/clippy PASS; workspace275PASS,0failed,2existingignored. Separate AI sessions established independent21history/import+9declaration+7command fixtures,158 board-prefix corroborations; fresh code review33 targeted Rust tests and30case independent geometry/arithmetic audit PASS. No additional material code findings; minor declaration-eligibility documentation finding resolved. Exact blocker: no legal mutual-continuous-check history with sourced outcome and no independently verified impossibility argument. Nanka v3 section5/Table1 needs exhaustive exclusion of mixed-radius/shared-endpoint discovery-bishop configurations including intervening direct checks/captures/drops. See docs/m4/mutual-check-proof-review.md (source URL/version/SHA256 and concrete candidate). Provisional UnresolvedMutualCheck rejection would violate the complete live-action contract; no arbitrary outcome/policy adopted. Fixture expectations and research are integrated as handoff only; native implementation is NOT integrated. Required next action: complete that exact proof/authoritative-rules obligation, then re-review/final verification/integration. MSH-68/69/70 and later work not started. M2/M3,63-66,61 persistence and40 roadmap correction unchanged. CI for WIP source: https://github.com/jweathers777/mShogi/actions/runs/40317821544; initially queued, not represented as passed.

Final bounded handoff: research/fixtures/blocker integrated and pushed on codex/rewrite at f291862c1fb49e6343a9a48b68994a715fb0561b; provisional native source remains unmerged at f259bb23834d6156f21aeec53db9ee00ef7242e4 on pushed codex/shogi-history. Native CI40317821544 PASS all three OS (Ubuntu24.04/macOS15/Windows2022) for that exact WIP source: https://github.com/jweathers777/mShogi/actions/runs/40317821544. Research-only integration native CI40317964799 PASS all three after one failed Ubuntu job retry: https://github.com/jweathers777/mShogi/actions/runs/40317964799. Original attempt failed unchanged crates/mshogi-runner/tests/runner.rs172: early nonzero child exit expected UnexpectedExit, observed handshake MissingResponse with Broken pipe; rerun of only that job passed without source change. Retain this observation; root cause not resolved, no runner/chess policy changed in this task. Supporting desktop-package workflows40317821476/40317964927 were still running at handoff; no result inferred and no new GUI campaign claimed. Temporary app-managed worktree archived after pushed commit and clean-tree verification; task source can be restored from attachment or recreated from origin/codex/shogi-history. Main checkout remains codex/rewrite. Next shogi action is to resolve the documented MSH-67 mutual-proof/authoritative-adjudication gap, then fresh final review/integration; MSH-68/69 remain dependency-blocked. Globally Ready80-83 are other profile research, outside this authorization and untouched. MSH-41-47 remain open; M2/M3 and63-66 completion evidence unchanged.

Resumed2026-10-26 by @codex on codex/shogi-history in restored app-managed /Users/johnweathers/.codex/worktrees/shogi-history/mShogi. Owner supplies proposed proof supplement (archive SHA256 a96bc888fec6c3a0da9c375cd398881d64454c67fd965f82c372acdc6eee33f8) for review, not as mathematical authority; selectedprofileunchanged. Reconciled newer rewrite handoff by retaining canonical Backlog/fixture-derivation versions; original branch history remains intact. Supplement SHA256SUMS all match. Inspected standard-library scripts and ran verify_geometry.py:23normalized pairings/20offsets,46withreflections, two-endpoint cycles; this is conditional necessary geometry, not all-state search. Fresh full-chain and enumeration-constraint reviews pending; no gate markedclosed by supplied claims.

Resumption proof review ACCEPTED. Fresh independent proof_chain_review reconstructed complete Nanka+supplement chain for all locallyadmitted roots with standard stock, including reducedmaterial/unknownprehistory, fixedkings, singlechecker, nongazingcounts, anglemultisets, drops/captures/promotion/sourcechanges, stockreduction and all23normalized cases. Separate geometry_supplement_review ACCEPTED necessaryfilters,bothenumerators,20offsets/23pairings/46reflections,84singlecyclegraphs,all5Hcases,2mixedcases,2simultaneous3Rexclusions,14lockedpairs; addedexplicitnonincreasing-source-inventory justification atarrival. Originalfailedreviewandattempts preservedunchanged, supplementchecksumsallmatch, no profilechange/newoutcome. Currentresolution/docsmetadataupdated; allconcreteexpectedfixturearraysbyte-semanticallyunchangedagainstf259. Implementation replacesprovisionalmutualerror with detailed internalMutualCheckInvariant; prefixResultandRecordErrorretain diagnostics;2privatecorruptiontests do notclaimlegalcounterexample. Rootfocused35PASS; workspace277PASS/0failed/2existingignored; fmt/clippyPASS macOS27.0.1arm64 Rust1.89.0 Python3.14.7. Final independentimplementationreviewandnativeCI/integrationpending; AC/DoDnotcheckedyet. AllBacklogmutationsusepinned1.55.1binary,notglobal1.55.0. Priorrunnerbrokenpipeclassificationfailure/retryretainedwithoutrootcausefixclaim.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
MSH-67 remains unfinished (To Do, explicit rules blocker). Research handoff integrated at f291862; tested provisional Game/history/records retained unmerged at f259bb2 on codex/shogi-history. Local275 testsPASS/two existingignored; WIP native CI passes all three OS, research-only nativeCI passes after one recorded existing runner classification mismatch/retry. Independent AI reviews with exact source fingerprints found no additional material defect, but mutual-continuous-check impossibility/interpretation remains unestablished; no fallback invented or criteria weakened. Worktree archived. Resume the exact proof/source obligation in docs/m4/mutual-check-proof-review.md before integration; no68/69/70work begun.
<!-- SECTION:FINAL_SUMMARY:END -->
