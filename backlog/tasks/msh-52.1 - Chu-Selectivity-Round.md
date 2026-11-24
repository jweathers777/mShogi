---
id: MSH-52.1
title: Chu Selectivity Round
status: In Progress
assignee:
  - codex
created_date: '2026-11-24T17:44:00Z'
updated_date: '2026-11-24T18:43:00Z'
labels:
  - chu
  - search
milestone: m-5
dependencies:
  - MSH-201
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 244000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Improve Chu search decisions with a bounded round of classical shared-core selectivity techniques. John started this step after MSH-201; preserve native Chu rules and use the accepted MSH-201 engine as the frozen predecessor. Evaluation tuning and the later strength measurement are separate steps.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Quiescence checks, full SEE, game-scaled margins, move ordering and shared df-pn mate search each have a recorded measured outcome or a concrete reason they do not apply.
- [x] #2 Kept changes pass native correctness and tactical checks and have supporting clocked predecessor evidence; inconclusive or rejected experiments are reported honestly.
- [x] #3 Required workspace checks pass, useful design documentation is current, and run identities, settings, costs and cleanup evidence are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze accepted MSH-201 source 79c2409 (product-identical to main 6c2414b) and reuse its hashed Linux binaries and frozen opening/tactical inputs. 2. Test one coherent candidate: eight quiet royal-threat moves at the first quiescence ply, complete threat generation with native root restrictions, and quiet ordering by the existing placement delta. Existing full compound exchange model and MARGIN_UNIT=100 remain; shared df-pn is inapplicable because Chu admits moves into threat and its royal-capture rule_score is not the solver mate terminal. 3. Run focused native comparisons, elementary/forcing probes at 20ms, fixed-depth cost diagnostics, and clocked predecessor SPRT H0=0/H1=15 alpha=beta=.05. Use c3-standard-4,10s+0.1s nominal, at most600 paired openings, checkpoints in complete batches, within US$10 and under two hours of game campaigns; record cutoffs unscored and every launched pair. Choose batch sizing after the tactical/cost pilot before game results. Keep supported changes; report and revert null results. 4. Run workspace checks, document technique outcomes, integrate the result and delete task resources.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Started after John requested Chu selectivity under MSH-52. Only this step is assigned; later evaluation/measurement remain separate. Branch codex/msh-52.1-chu-selectivity. Frozen engine source79c2409 from runs/msh-201/build-final/out/mshogi, SHA25637f238fb151d90fea8af39067f4f7a26037d0fb9831eae0df3d602f84ed90472. No native rules, compatibility shims or history-work models will be added.

Candidate372b689 implemented and pushed. The new native comparison checks the complete set of royal-threat successors across sourced fixtures and restricted roots. Focused Chu checks30 passed/1 ignored, workspace Clippy and fmt pass; full workspace tests running. Cloud build build-msh521-candidate is active at runs/msh-52.1/build-candidate. Initial submission configuration failed locally before any allocation; corrected to the shared controller parallelism setting, actual build has one task/VM. Next: same-VM tactical probes and fixed-depth cost comparison, then freeze game batch sizing before results.

Pilot on one c3-standard-4, candidate372b689 versus accepted79c2409: elementary315/315 and80/80 igui unchanged; forcing69/71 ->71/71 (development53/55 ->55/55, held-out16/16 unchanged). Same18 frozen positions/depth5/3 fresh repeats:107238 ->158715 nodes, sum of per-position median times0.597921 ->0.876480s (+46.6% time for +48.0% work). Candidate explores more, with roughly unchanged per-node cost. The two old forcing failures are now solved; these constructed suites share templates and are not independent strength ratings. Build and pilot resources verified absent; compute upper boundsUS$0.1943. Pilot data runs/msh-52.1/pilot-1.

Game sizing fixed before any game result: at most four30-minute campaigns, each eight bins of16 paired seeds. Campaign seeds1..128,129..256,257..384,385..512, using the frozen128-opening catalog cyclically; no selective retries.10s+0.1s nominal,c3-standard-4,1000 experiment steps,600s pair watchdog,1500s bin wall,1800s campaign deadline. A bin stops on an unscored pair, preserving all evidence. Pool scored pairs at each completed campaign; H0=0/H1=15,alpha=beta=.05,min30. Stop on H1 or H0, otherwise continue to the fixed maximum; no decision is reported as such and candidate reverted if unsupported. No multi-hour extension. This fits the US$10 task cap.

Final candidate checks: workspace1189 passed/12 ignored/0 failed; fmt and workspace all-target Clippy pass. First workspace run stopped only at the deterministic Chu node snapshot (173 ->145, same depth-two move and score), updated in c2cba48; second full run passes. No native rules expectations changed. Product build stays372b689. Checks archive uploaded to runs/msh-52.1/candidate-checks.tar.gz. First game campaign msh213-msh521-match-1 is active; continuation is the prespecified remaining campaigns only while pooled SPRT is undecided. Controller and scripts are under ignored runs/msh-52.1; branch codex/msh-52.1-chu-selectivity.

First prespecified game sample completed:43 launched pairs,37 scored,20W/54L/0D, pentanomial[20,0,14,0,3], LLR-1.859/no SPRT decision. Six pairs have one1000-step cutoff, unscored;80 native conclusions retained. Native report hashes and summaries verified. All cloud resources deleted; sample compute upper boundUS$0.508, task totalUS$0.702 before sample2. Result is unfavorable; continuing the already declared second sample (seeds129..256), msh213-msh521-match-2. No product changes during this comparison.

Weak-set diagnosis: first-sample median completed depth candidate5 versus baseline6 (from all reported turns; different game trajectories, so not a paired-position cost measurement). The same-position pilot separately measured46.6% extra time. While the frozen first candidate continues its prespecified second game sample, prepare an ordering-only ablation: remove the new quiescence stage/generator and its test, retain only placement-delta quiet ordering. This is the specific weak-set ablation allowed by the roadmap, not a change to the running binary. Do not pool different candidates.

Ordering-only ablation source8b54048 is pushed; its product diff versus the accepted predecessor is only quiet_order using16 times the unchanged placement delta. Focused Chu checks and workspace Clippy pass. Build job build-msh521-ordering is queued while the second combined-candidate match occupies cloud capacity. Build uses temporary subscription projects/mshogi/subscriptions/msh521-build-completion; delete it at cleanup. Its controller and source receipt use ordering-build-* under runs/msh-52.1. Match2 remains source372b689; do not mix results. After build cleanup, run prepare_ordering_pilot.py and the resulting manifest on the same pinned worker image, with the temporary subscription if the game controller is still active.

Ordering-only pilot source8b54048 on one c3-standard-4: elementary315/315,80/80 igui and forcing69/71 unchanged from predecessor; depth5 fixed set107238 ->116882 nodes and0.593320 ->0.662469s (+9.0% nodes,+11.7% time). This diagnostic offers no tactical or cost benefit, so do not spend another game campaign on it. Revert the ordering-only candidate and snapshot update; this is a rejected pilot, not a demonstrated strength loss. Combined-candidate sample2 is still completing; available pooled50 scored pairs give25W/75L and LLR-3.001, but use the complete-campaign checkpoint as declared. All ordering-pilot resources are deleted.

Final combined-candidate game verdict:372b689 versus frozen79c2409, two completed prespecified campaigns,89 launched pairs/79 scored pairs,41 wins117 losses0 draws;10 pairs have one unscored1000-step experiment cutoff, all168 native conclusions retained. Pentanomial[46,0,25,0,8], SPRT H0=0/H1=15 alpha=beta=.05,min30: LLR-3.783, lower boundary-2.944, H0. Approximate paired estimate-182 Elo (95% interval-257 to-120), conditional on scored pairs; censored games and repeated catalog openings limit that estimate. This is rejection of the proposed gain, not an independent engine rating. Every downloaded report hash and native summary verified; replay logs passed, no engine failure. Stop here under the declared decision rule; no third/fourth campaign. Combined candidate and ordering-only pilot are reverted, including their snapshot/test changes. Product/search/test files are byte-identical to accepted main6c2414b.

Technique outcomes: (1) Quiescence quiet royal threats: tested with eight checks at the frontier as part of the combined candidate; rejected on game evidence despite two more constructed forcing solves. (2) Full SEE: existing threshold minimax already follows complete admitted compound recaptures, both victims, promotions and final landing; unresolved local exchanges retain the move. Keep that implementation unchanged; a Chess single-square swap list would lose Chu effects. (3) Margins: retain MARGIN_UNIT100, the native Pawn value with captures discarded and no hand gain. (4) Ordering: retain existing shared TT, tactical, killer, history and continuation ordering; tested placement-delta quiet bonus alone, rejected at pilot for more time/nodes and no tactical gain, not claimed as a measured strength loss. (5) Shared df-pn: not enabled because its no-legal-evasion mate and exclusion of rule_score terminals do not fit capture-based Chu; rationale now in chu-search-geometry.md.

Reproduction: chu_strength probe SUITE OUT 20; chu_bench bench bench.json OUT 5 3. Game commands native_match --game chu --base 10 --increment 0.1 --max-steps 1000 --max-pairs 16 --first-seed 1,17,...113 then129,145,...241 --wall-seconds 1500 --pair-wall-ms 600000, through chu_cloud. c3-standard-4 throughout, Rust1.99.1 x86-64 release. Exact argv, input/binary hashes and receipts are under gs://mshogi-experiments-216675633801/runs/msh-52.1/ (build-candidate,pilot-1,match-1,match-2,build-ordering,ordering-pilot). All six cloud controllers verified jobs/VMs/disks absent. Sum of conservative VM compute upper bounds US$1.5244, not actual billing; no new image build, small storage extra, withinUS$10. Temporary msh521-build-completion subscription is being deleted. Final restored-baseline workspace checks are running; branch codex/msh-52.1-chu-selectivity.

Final restored-baseline validation passed:1188 workspace tests,12 ignored,0 failed; fmt and workspace all-target Clippy pass. Main6c2414b remained unchanged and was merged into this branch; final crates/ and tools/ diff versus main is empty. No rules/format change or retained code refactor needs a separate review; no UI or platform change needs GUI/CI runs. Temporary cloud subscription deleted. Final evidence files final-results.json,experiment-records.tar.gz,candidate-checks.tar.gz,final-checks.tar.gz are under runs/msh-52.1 in GCS. Next step is Chu evaluation under MSH-52, not started.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the bounded Chu selectivity round. Quiescence royal threats plus quiet placement ordering solved two extra constructed tactics but lost the predecessor test: 41 wins and 117 losses in 79 scored pairs, with 10 cutoff pairs unscored; SPRT H0. Ordering alone added about 12% fixed-depth time with no tactical gain, so that pilot was also rejected. Both candidates were reverted; the accepted MSH-201 engine remains unchanged. Existing compound SEE and Pawn-scale margins stay; the shared mate solver does not fit Chu capture-based terminals. Final formatting, Clippy and 1,188 workspace tests pass; all cloud resources are deleted. Next: Chu evaluation under MSH-52.
<!-- SECTION:FINAL_SUMMARY:END -->
