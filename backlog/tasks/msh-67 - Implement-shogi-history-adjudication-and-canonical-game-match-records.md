---
id: MSH-67
title: Implement shogi history adjudication and canonical game-match records
status: Done
assignee:
  - '@codex'
created_date: '2026-10-26T07:44:00Z'
updated_date: '2026-10-26T13:06:00Z'
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
  - docs/m4/history-state.md
  - docs/m4/history-validation.md
  - docs/m4/mutual-check-resolution.md
  - docs/m4/history-final-review.md
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
- [x] #1 Rules-owned history separates saved-state,repetition and search identity; repetition includes promoted ownership,both hands and turn, with complete checking intervals and explicit known-history origin.
- [x] #2 All selected-profile conclusions and precedence are implemented: fourfold/perpetual check,no-legal-action loss,resignation,formal entering-king declaration and declared length policy; advisory probes,illegal API rejection and tournament penalties remain distinct.
- [x] #3 Canonical native game/match records preserve initial state,complete actions,commands/events,profile/edition,modes,limits,policies,counters,conclusions and score/replay semantics; strict replay validates independently expected history and rejects unsupported/lossy input.
- [x] #4 Imported-SFEN counter/history semantics and overlapping boundary fixtures are resolved under the ratified profile; completing a history-dependent losing cycle is not erased by generic invalid-request rejection.
- [x] #5 History/record roundtrips,undo,prefix reconstruction and same-position/different-history cases are independently verified. Timing or external-event policies are either explicitly supported or clearly unsupported,never inferred from chess.
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
Resume f259bb2 and reconcile newer9325c48 handoff without replacing working history/record features. Preserve original failed proof review, source research and CI failures. Inspect and checksum supplied supplement, run both necessary-geometry enumerators and check all exclusion constraints/case accounting. Obtain fresh independent complete-chain proof review for the locally admitted live-root domain, including drops/captures/promotion/source changes and semantic multisets. Investigate any precise failed implication. If accepted, resolve current HIST-INTERVAL documentation and replace provisional mutual rejection with detailed internal invariant diagnostics, preserving broad Position analysis and explicit imported roots. Run affected focused tests and relevant regressions, obtain fresh final implementation review, integrate/push codex/rewrite and normal native CI, finish AC/DoD only with complete evidence, then archive temporary worktree. Stop before68/69/70 and keep41-47 open.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
2026-10-26 SHOGI-P1 selected option A mshogi.shogi.csa36.v1 against integration6db8500. Profile/coverage docs now name HIST-INTERVAL (SH-F028–030), DECL-BOUNDARY (034–037), IMPORT-LENGTH (041) as concrete independently reviewed fixture/research gates before affected MSH-67 implementation. No implementation started by this note; existing dependency/acceptance scope preserved.

MSH-67 attempted 2026-10-26 by @codex in temporary codex/shogi-history worktree, based986a527. BLOCKED and returned To Do, all AC/DoD unchecked. Unmerged partial implementation preserved and pushed at f259bb23834d6156f21aeec53db9ee00ef7242e4 (codex/shogi-history); do not merge as complete. It contains GameRoot/live admission, exact native history/context/undo, selected untimed outcomes for established cases, strict GameRecord replay,15 new focused tests and independent review reports. Local macOS arm64 Rust1.89 cargo fmt/clippy PASS; workspace275PASS,0failed,2existingignored. Separate AI sessions established independent21history/import+9declaration+7command fixtures,158 board-prefix corroborations; fresh code review33 targeted Rust tests and30case independent geometry/arithmetic audit PASS. No additional material code findings; minor declaration-eligibility documentation finding resolved. Exact blocker: no legal mutual-continuous-check history with sourced outcome and no independently verified impossibility argument. Nanka v3 section5/Table1 needs exhaustive exclusion of mixed-radius/shared-endpoint discovery-bishop configurations including intervening direct checks/captures/drops. See docs/m4/mutual-check-proof-review.md (source URL/version/SHA256 and concrete candidate). Provisional UnresolvedMutualCheck rejection would violate the complete live-action contract; no arbitrary outcome/policy adopted. Fixture expectations and research are integrated as handoff only; native implementation is NOT integrated. Required next action: complete that exact proof/authoritative-rules obligation, then re-review/final verification/integration. MSH-68/69/70 and later work not started. M2/M3,63-66,61 persistence and40 roadmap correction unchanged. CI for WIP source: https://github.com/jweathers777/mShogi/actions/runs/40317821544; initially queued, not represented as passed.

Final bounded handoff: research/fixtures/blocker integrated and pushed on codex/rewrite at f291862c1fb49e6343a9a48b68994a715fb0561b; provisional native source remains unmerged at f259bb23834d6156f21aeec53db9ee00ef7242e4 on pushed codex/shogi-history. Native CI40317821544 PASS all three OS (Ubuntu24.04/macOS15/Windows2022) for that exact WIP source: https://github.com/jweathers777/mShogi/actions/runs/40317821544. Research-only integration native CI40317964799 PASS all three after one failed Ubuntu job retry: https://github.com/jweathers777/mShogi/actions/runs/40317964799. Original attempt failed unchanged crates/mshogi-runner/tests/runner.rs172: early nonzero child exit expected UnexpectedExit, observed handshake MissingResponse with Broken pipe; rerun of only that job passed without source change. Retain this observation; root cause not resolved, no runner/chess policy changed in this task. Supporting desktop-package workflows40317821476/40317964927 were still running at handoff; no result inferred and no new GUI campaign claimed. Temporary app-managed worktree archived after pushed commit and clean-tree verification; task source can be restored from attachment or recreated from origin/codex/shogi-history. Main checkout remains codex/rewrite. Next shogi action is to resolve the documented MSH-67 mutual-proof/authoritative-adjudication gap, then fresh final review/integration; MSH-68/69 remain dependency-blocked. Globally Ready80-83 are other profile research, outside this authorization and untouched. MSH-41-47 remain open; M2/M3 and63-66 completion evidence unchanged.

Resumed2026-10-26 by @codex on codex/shogi-history in restored app-managed /Users/johnweathers/.codex/worktrees/shogi-history/mShogi. Owner supplies proposed proof supplement (archive SHA256 a96bc888fec6c3a0da9c375cd398881d64454c67fd965f82c372acdc6eee33f8) for review, not as mathematical authority; selectedprofileunchanged. Reconciled newer rewrite handoff by retaining canonical Backlog/fixture-derivation versions; original branch history remains intact. Supplement SHA256SUMS all match. Inspected standard-library scripts and ran verify_geometry.py:23normalized pairings/20offsets,46withreflections, two-endpoint cycles; this is conditional necessary geometry, not all-state search. Fresh full-chain and enumeration-constraint reviews pending; no gate markedclosed by supplied claims.

Resumption proof review ACCEPTED. Fresh independent proof_chain_review reconstructed complete Nanka+supplement chain for all locallyadmitted roots with standard stock, including reducedmaterial/unknownprehistory, fixedkings, singlechecker, nongazingcounts, anglemultisets, drops/captures/promotion/sourcechanges, stockreduction and all23normalized cases. Separate geometry_supplement_review ACCEPTED necessaryfilters,bothenumerators,20offsets/23pairings/46reflections,84singlecyclegraphs,all5Hcases,2mixedcases,2simultaneous3Rexclusions,14lockedpairs; addedexplicitnonincreasing-source-inventory justification atarrival. Originalfailedreviewandattempts preservedunchanged, supplementchecksumsallmatch, no profilechange/newoutcome. Currentresolution/docsmetadataupdated; allconcreteexpectedfixturearraysbyte-semanticallyunchangedagainstf259. Implementation replacesprovisionalmutualerror with detailed internalMutualCheckInvariant; prefixResultandRecordErrorretain diagnostics;2privatecorruptiontests do notclaimlegalcounterexample. Rootfocused35PASS; workspace277PASS/0failed/2existingignored; fmt/clippyPASS macOS27.0.1arm64 Rust1.89.0 Python3.14.7. Final independentimplementationreviewandnativeCI/integrationpending; AC/DoDnotcheckedyet. AllBacklogmutationsusepinned1.55.1binary,notglobal1.55.0. Priorrunnerbrokenpipeclassificationfailure/retryretainedwithoutrootcausefixclaim.

Final reviewed source daf438e integrated/pushed in fd58644757bc5edd1842a855b07aad872659293c. Fresh independent final implementation review ACCEPTED (docs/m4/history-final-review.md),35focusedPASS andexactsourcehashes, no materialfindings. Reviewer confirms live-root/replay maintainallproofassumptions and broaderPositionanalysisunchanged. Rootworkspace277PASS/0failed/2existingignored; fmt/clippyPASS. IntegratednativeCI40325840465 PASS firstattempt all3 OS Ubuntu24.04/macOS15/Windows2022: https://github.com/jweathers777/mShogi/actions/runs/40325840465. Desktopregression40325840456 automaticallyrunning separately; no packagedGUI/strengthcampaign claimed. Historicalproofreviewcomparedagainst9325c48 andunchanged; concretefixturearrayscomparedagainstf259andunchanged. SupplementarchiveSHAa96bc888fec6c3a0da9c375cd398881d64454c67fd965f82c372acdc6eee33f8; sourcePDFSHAcd5f987133d89a32bd6de8391015354b16d066a97cea7f719b67a5089275e065; newacceptedproofindex docs/m4/mutual-check-resolution.md. Originalrunnerclassificationfailureandretryremainhistorical, nofixclaimed. Stopafter67;69nextdependencyreadywhenDone,68/70wait69. StillInReviewuntilfinalCI/evidencecompletion.

Completion evidence for MSH-67 (2026-10-26)

Reviewed source daf438e is integrated and pushed at fd58644757bc5edd1842a855b07aad872659293c on codex/rewrite. The integrated core tree matches the reviewed source. No subsequent engine, console, shared integration or desktop shogi task was started.

Acceptance mapping:
1. GameRoot and Game own exact root/category/counter provenance, accepted actors/commands, complete repetition identities and check intervals. GameContext retains full root plus ordered commands; repetition excludes counters/provenance. Runtime execution identities remain later shared-host work. See history-state.md and shogi_history.rs.
2. Selected untimed no-action loss, fourfold/continuous-check loss, resignation, formal entering-king win/false-declaration loss and length precedence are verified against independent expectations. Advisory eligibility is harmless; malformed/stale/illegal admission is atomic. The accepted complete-chain proof resolves HIST-INTERVAL without adding an outcome. Native mutual detection is an internal diagnostic with complete candidate evidence, no commit or score.
3. Strict versioned GameRecord preserves profile/edition, root category, supplied/defaulted counters, known-history origin, policies/limits/mode, event actors/order, derived effects, conclusions and half-point scoring. Authoritative replay checks every prefix and rejects duplicate/unknown/missing/unsupported/lossy/forged/post-terminal input. Maximum320-event complete ledgers and319-plus-terminal-command records fit the bounded1MiB codec. Later shared archives/observations remain separate.
4. Independently expected21 history/import cases and nine declaration diagrams cover both checker identities, checked/check-relieved endpoints, interruptions, overlapping[3,7,11,15] occurrences, both thresholds/prerequisites, imported n320/321/322/999, defaulted/supplied origins and conclusion precedence. A legal losing-cycle action is retained. Concrete expectation arrays remain identical to f259bb2; original incomplete proof research remains unchanged.
5. Exact semantic undo, checked event prefixes (including same-ply terminal commands), strict round trips and same-position/different-history cases pass. Prefix and record errors preserve internal diagnostics. Unsupported timing/external policies reject explicitly. Private corruption tests are cfg(test) only, not alleged legal mutual cycles or normal-package instrumentation.

Proof/review evidence: docs/m4/mutual-check-resolution.md indexes two fresh independent AI mathematical reviews (complete chain and geometry/supplement), exact Nanka v3 PDF and supplied archive hashes, both inspected enumerators and all case exclusions. Geometry verification agrees on23 normalized pairings/20 offsets/46 reflected pairings;84 cyclic fixed-king/side/radius graphs each have one simple cycle. This is conditional geometry plus reasoned exclusions, not an all-state search or formal proof assistant certificate. The positive-offset simultaneous-source argument was explicitly justified with a nonincreasing source count while a bishop blocks the ray. Fresh final independent AI implementation review accepts the source/domain boundary and diagnostic propagation, with exact source fingerprints and35 focused tests. Earlier independent fixture/game/record reviews remain applicable to unchanged work.

Verification: local macOS27.0.1 arm64 Rust/Cargo1.89.0 Python3.14.7; cargo fmt and workspace all-target Clippy -D warnings PASS; focused35 PASS; workspace277 PASS,0 failed, two existing deep chess diagnostics ignored. Integrated native CI40325840465 PASS first attempt on Ubuntu24.04, macOS15 and Windows2022: https://github.com/jweathers777/mShogi/actions/runs/40325840465 . The automatically triggered supporting desktop regression40325840456 passed macOS and Windows; Linux remains in native dependency installation before source checks, so its package/GUI result is pending and is not represented as passed: https://github.com/jweathers777/mShogi/actions/runs/40325840456 . Completion relies on this native task's passed three-platform native CI and reuses unchanged M2/M3 application evidence as authorized; no desktop source, packaging, transport or application behavior changed. The earlier runner classification failure/retry is preserved; no runner fix or root-cause resolution is claimed.

The temporary app-managed worktree was clean, had no referring process and its HEAD was an ancestor of pushed rewrite before archival; attachment now records archived_worktree. Main checkout remains codex/rewrite. MSH-41–47 and M4 remain open, M2/M3 and MSH-63–66 stay Done, MSH-61 and roadmap correction remain intact. Next eligible shogi work is MSH-69; MSH-68/70 depend on69. Stop after67.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented and integrated native shogi GameRoot/Game/GameContext and strict GameRecord under unchanged mshogi.shogi.csa36.v1. Includes explicit initial/imported provenance, complete known history, selected untimed conclusions/declarations, atomic commands, exact undo/prefixes and strict every-prefix replay. Complete-chain and geometry reviews resolve HIST-INTERVAL; original failed research remains historical. Internal invariant diagnostics retain full candidate evidence and never invent an outcome.

Source daf438e integrated/pushed at fd58644757bc5edd1842a855b07aad872659293c. Local277 tests pass (two existing ignored),35 focused tests pass, formatting/Clippy pass; required native CI40325840465 passes all three OS. Separate desktop regression40325840456 passes macOS/Windows; Linux dependency installation is still pending, with unchanged M3 evidence reused and no new GUI claim. Independent AI proof/implementation reviews accepted; source hashes, exact methods and fixture provenance are retained in docs/m4/history-validation.md and linked reviews.

Untimed game-owned API only; no new shogi engine/console/shared archive/desktop consumers. Worktree archived. MSH-41–47/M4 remain open. MSH-69 is next;68/70 await69. Prior runner classification failure/retry retained without claiming a fix.
<!-- SECTION:FINAL_SUMMARY:END -->
