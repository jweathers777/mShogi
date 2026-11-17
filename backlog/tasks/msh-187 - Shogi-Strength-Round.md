---
id: MSH-187
title: Shogi Strength Round
status: In Progress
assignee:
  - '@codex'
created_date: '2026-11-16T20:51:00Z'
updated_date: '2026-11-17T07:25:00Z'
labels:
  - handle:shogi-strength-round
milestone: m-5
dependencies:
  - MSH-186
documentation:
  - docs/m5/shared-search-core.md
  - docs/m5/strength-roadmap.md
  - docs/m5/strength-harness.md
  - docs/m5/evidence/msh180/README.md
priority: high
ordinal: 194000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Handle: Shogi Strength Round

## Problem
Shogi uses the shared search core since MSH-180 (Shogi Search Core Adoption), but its engine is still basic. On ordinary roots the median depth is 7 at 20,000 nodes. Quiescence has no check stage, although quiet checking drops often decide Shogi games. The exchange test is a one-step estimate. The evaluation is material with Pawn advance, adjacent king cover, hand flexibility and tempo; it is not tuned. The core margins (futility, razoring, aspiration, quiescence) assume Chess piece values. Desktop play and default USI use a node limit (300,000) instead of a time budget. See docs/m5/shared-search-core.md, "Shogi on the core" and "What Tori showed".

## Objective
A substantially stronger conventional Shogi engine, built by the same classical method that worked for Chess: speed and depth first, then selectivity and time, then a tuned evaluation. This prepares MSH-50 (Conventional Shogi) acceptance criterion 5 (a substantially complete conventional engine before the major strength study).

## Approach
- Follow AGENTS.md "Change direction when evidence shows little progress" and docs/m5/strength-roadmap.md "Game tests in this direction". Before each coherent set, record baseline, intended benefit, comparison method, resource limit and decision checkpoint. Freeze main at task start as the predecessor.
- Measure first with the MSH-186 (Shogi Strength Harness) tools: profile the search path (position updates, move and drop generation, cache identity and size) and depth at time.
- Candidate coherent sets (established techniques; the worker chooses order and detail after measurement):
  - Search: checking moves and drops at the first quiescence ply; a full exchange evaluation with attacker lists; margins scaled for Shogi piece values; move ordering for drops and promotions; review of reductions and null move for drop games (zugzwang is rare, but hand pieces change tactics).
  - Time: time-based default for desktop play and USI, using the MSH-179 (Clock and Session for All Games) session policy, instead of the fixed node limit.
  - Evaluation: Texel-tuned handwritten terms with Shogi meaning, for example piece and hand values (including value of pieces in hand by count), promoted pieces, king safety against attackers and possible drops near the king, castle shapes, piece-square terms and mobility. Neural learned evaluation is out of scope.
- Transfer techniques, not Chess constants. Native rules stay the authority for real moves, history, declarations and results; private analysis may approximate (design A-03).
- Shared-core changes must keep Chess, Tori, Xiangqi, Chu and Tenjiku tests passing. If a core change alters Chess search, show identical fixed-depth Chess output or run a Chess predecessor game test.
- If the work becomes too large for one task, finish a coherent set, record the result, and create a follow-up task for the next set.

## Completion
Each retained set passes Shogi perft and differential tests against the native rules, the tactical suite, and a sequential clock-mode game test against the frozen predecessor. The task records depth, tactical-suite and game-test results before and after, independent review, and an updated docs/m5/shared-search-core.md. Verification level 1, with level 2 checks only where the time-default change affects desktop or USI behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Before each coherent set, the task records baseline, intended benefit, comparison method, resource limit and decision checkpoint; each set is retained or rejected on that evidence.
- [ ] #2 Retained changes pass Shogi perft and randomized differential tests against the native rules, and all other games' search tests still pass.
- [ ] #3 Retained changes improve the Shogi tactical suite or depth at time, and win a sequential clock-mode game test against the frozen task-start predecessor.
- [ ] #4 Desktop play and default USI use a time budget through the shared session policy, with the affected desktop and USI checks recorded.
- [ ] #5 Results before and after, decisions, limitations and independent review are recorded, and docs/m5/shared-search-core.md is current.
<!-- AC:END -->

## Definition of Done
<!-- DOD:BEGIN -->
- [ ] #1 Acceptance criteria have supporting evidence; failed or unavailable checks are not presented as passed.
- [ ] #2 Applicable independent review is recorded and material findings are resolved.
- [ ] #3 Relevant documentation and follow-up tasks are current.
- [ ] #4 The result is integrated into the recorded integration branch with a durable completion or handoff summary.
<!-- DOD:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze task-start main1d19a544 and profile the Shogi full path on ordinary and hand-heavy roots; reuse MSH-186 baseline with a fresh identity-bound probe. 2. Implement one coherent classical set: faster attack/ray and evaluation updates, full exchange estimates, a bounded first-quiescence checking stage through a default-preserving shared-core hook; validate exact move/state restoration and tactical behavior. 3. Independently implement desktop/default-USI time budgets through the existing shared session policy; retain explicit fixed-work requests and validate affected native/desktop/USI consumers. 4. Before games, compare fixed-depth/time/development tactics and preserve Chess output through default-preserving hooks. Retain the search set only after native correctness and a frozen-predecessor sequential clock test; use cloud for a maximum that would exceed one local hour, reusing existing Batch infrastructure. 5. Record decisions and measured limits; do not add evaluation tuning without a useful independent corpus and a bounded subsequent-set plan. Review meaningful changes independently, update shared-core documentation and verification, integrate main and push. Scope already permits a coherent set with later evaluation work separately recorded.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Owner: Codex chat01a1e634-7fc9-72c2-b4b4-07022a7df3ec. Primary checkout was clean main1d19a544, owned by this chat after MSH-186 (Shogi Strength Harness), with no active Backlog owners. Feature branch codex/shogi-strength-round; other worktrees untouched. Installed Backlog1.55.3 matches baseline. Frozen task-start predecessor1d19a544cc3e1ca3e92a0e8f53268758967329bf. Initial inspection: existing fast u128 board/incremental material/hash,16MiB retained table, per-node scan/ray generation, no quiescence checkstage, one-step exchange estimate. Baseline MSH-186 ordinary1000ms depths14/16/11/11/10 and12/12shortmate cases (sixgeometries, ceiling-limited). Profile full path before first coherent searchset; native rules remain authoritative. Level1 native search; targetedLevel2 desktop/USI default time integration.

First-set hypothesis: reverse target attack lookup and incremental placement remove repeated per-node scans; exchange-aware ordering and bounded quiet checking moves improve drop tactics without multiplying every quiet move. Benefit target: materially faster/deeper ordinary timed search, no shortmate regression, and H1=15Elo against frozen1d19a544. Comparison: same compiler/build/CPU/5+.2 clock and sixpaired opening families, native replay, SPRT H0=0/H1=15 alpha=beta=.05 min30 max600pairs, stop on failures and sequential decision; no external engines. Focused profiling/baselines <=10min local. Cloud execution plan/cost ceiling must be fixed before launch; reuse established project resources, one machine type, no new infrastructure. Checkpoint after correctness and timed probes, then after bounded games; weak benefit requires direction change before another fix round. Tuned evaluation remains a later coherent set only if justified; do not treat tiny generation demo as training-ready.

Cloud resource checkpoint before launch: reuse existing mshogi project/us-central1 Batch, configured bucket/registry/worker identity,8 c3-standard-4 machines maximum, no retries. First-submit through cleanup <=60min, reserve300s cleanup, <=8VMh and US$2 compute at verified rate0.201608/VMh (8VMh=1.612864). Max600pairs is statistical ceiling, not guaranteed completion under resources. Start8pair pilot then24pair batches; immutable wholepairs, globalfailurestop, evaluateSPRT beforefurtherbatch, retainalllaunchedpairs. Same5+.2 control/candidatebaselineCPUtype. No externalratinggames/newinfrastructure. Infrastructure adaptation reviewed beforebillablelaunch. Nativequiet-check coreunitchecks12passed including first-q-entry cache discrimination and no material-delta pruning ofquietmate.

First coherent source checkpoint: precomputed rays/reverseattack lookup, incremental existing placement, full swapSEE with board+hand values/promotion/pins, firstquiescence quietchecking stage via default-offsharedhook, desktop/defaultUSI1s using existingretainedsessionpolicy. Positionfixedwork profile identical1448000childvisits/checksum across9roots: frozen0.366378/0.352444s, candidate0.215712/0.215788s (~1.67x throughput). This isolatespositionwork, notstrength. 135searchlibrary tests pass,5ignored; focusedShogi perft/randomnative/state/eval/quietcheck/SEE tests pass. Core independently accepted after removing extra optoutclassification cost. Time focusedUSI12/parser2/desktop2/sharedtiming3pass2ignored. Next frozen candidatebuild and same-root time/depth/tactics plus exactChessfixeddepthcomparison; no cloudlaunch until checkpoint evidence/review.

Measured first-set checkpoint at frozen candidate e902c31d: full fixed-depth total1.8376s ->2.1575s (nodes1594093->1884071; changedsearchwork,17.4%slower fixeddepth). Three1000ms repeats: ordinaryopening median depths14/16/11/11/10 ->15/16/12/12/10; hand-heavyactive/minor roots7/10 ->6/9, rookending16unchanged, syntheticmate64unchanged. Full timednode rate~1.129M->1.315M (+16.5%), but fasterboard work alone isnotstrength. Both12/12sixgeometryshortmate cases. Decision: continue to bounded coherent-set predecessor test, notcase-by-case tuning; addedchecking/SEE changes horizonwork and couldtrade depthforbetterdecisions. Practicalbenefit remainsunproved and hand-heavydepthregressions explicit. Noadditional localfixround before gameevidence; reject/reconsider if no gamebenefit.

Pre-cloud admission: candidateRustsource e902c31d unchanged in frozen cloudtooling ed92e6d28df59bc45836f108f24a3c8d9ff47e4e. 843consumer tests pass/14ignored; standaloneTauri checkpasses. Exact18Chess fixed-depth bestmove/score/PV/nodes/depth unchanged. ActualreleaseUSI barego/continuedhistory/newgamefixedprobe pass withcleanshutdown. Linuxbothengines samepinnedRust1.99.1/baseimage, portablex86-64 flags; baseline d9ea49d4..., candidate e98bb5d8..., cleanrunner observeded92. One-movepercolorLinuxsmoke unscoredstep_limit asintended, native replay/cleanup pass. Initialcompilerimage lackedGit; preservedfailedattempt, derivedbuildimage addsGitonly. Workerimage pushedexistingregistry atsha256:86127cbbf614223047e48f58ee33f366362657277e91c2310dd5afdb869ef90c. Cloudcode independentlyaccepted38offlinetests; readyfirst8pairpilot withinrecordedcaps.

Cloud round 1 stopped at its failure checkpoint. Pilot 8 pairs completed; batch 2 launched 24 pairs, of which 22 completed and 2 remain unscored. Pair 26 and pair 29 reversed-color games returned candidate Incomplete/Stopped with completed depth 0 at request budgets 115 ms and 177 ms. Native replay passed, with no process cleanup errors. This is not a native time loss or a scored draw. No further batch is authorized by this failed campaign protocol. All owned jobs, VMs and disks are verified absent. Actual lifecycle allocation: batch 1 0.3442295653 VMh / estimated $0.0693994342; batch 2 0.7188395954 VMh / estimated $0.1449238131. Decision: do not retain or integrate this set yet; inspect full request setup and first-iteration search before selecting a coherent correction or rejection. Preserve round 1 evidence and do not pool a changed candidate or control with it. AC3 remains unmet.

Second coherent engineering checkpoint, before edits: reject acceptance of round-1 candidate. Focused release measurements show redundant native history replay in timed cache-prefix admission: at 311/159 events, prefix alone costs 50.096/19.834 ms on macOS, in addition to decode 25.406/10.309 ms and server reconstruction 25.046/10.056 ms. Intended benefit: preserve exact native admission and cache identity while removing duplicate replay from validated-record prefix comparison. Implement a structural exact-prefix relation or equivalent native helper with independent native-prefix differential tests, incompatible-history/root/terminal/empty-prefix tests and fixed-search identity checks. In parallel, bounded local ablation (maximum 10 minutes) checks first-quiescence checks against original ordinary/hand-heavy roots and both failed roots; choose one coherent search policy only on these measurements. No fallback move or incomplete-result contract change. Verification Level 1 native semantics plus targeted Level 2 retained-session behavior. Existing failed campaign is final and cannot resume; any revised game comparison needs a separately recorded frozen candidate/protocol, remains within the existing aggregate resource ceiling, and cannot pool old games. No new cloud launch before review and this checkpoint decision.

Second-set selection checkpoint: on the failed bishop-exchange root, preloaded native and private search reproduce the failure without transport/history overhead. Original candidate completes depth 0; no quiet-check stage completes depth 1; an 8-ply quiescence bound still completes depth 0; removing the new checking stage and restoring predecessor exchange ordering completes depth 4, as does the predecessor. Ordinary and hand-heavy roots are included in the probe. Decision: reject the first-set quiet-check/full-SEE search changes, preserve their frozen source and failed evidence, and select a coherent speed/history/time set: fast attacks and incremental unchanged evaluation, exact validated native-prefix cache identity, and timed desktop/USI defaults. Remove unused experimental shared-core hook/generator rather than retain speculative interfaces. Require fixed-work identity against the frozen predecessor, native correctness/tactics and revised independent review before a separate game comparison. The revised comparison, if admitted in time, retains the original overall deadline 07:54:35 UTC, maximum 8 VMh and $2 across both campaigns; prior conservative use is 1.4993481530 VMh/$0.3022805824. No old games are pooled and no failed pair is replaced.
<!-- SECTION:NOTES:END -->
