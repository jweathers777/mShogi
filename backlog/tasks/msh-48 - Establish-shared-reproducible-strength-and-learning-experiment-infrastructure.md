---
id: MSH-48
title: Establish shared reproducible strength and learning experiment infrastructure
status: Done
assignee:
  - '@codex'
created_date: '2026-10-25T15:39:00Z'
updated_date: '2026-11-08T01:38:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-25
documentation:
  - docs/design.md
  - docs/m5/experiments.md
  - docs/m5/evidence/msh48/verification.json
ordinal: 48000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
P-02/P-05 and A-08 require comparable measurements before optimization or substantial training. Extend the existing M2 search/match evidence into reusable experiment support. This ticket owns shared harnesses, evidence formats, native data generation and rules reuse; six game-specific opponent pools, targets and strength verdicts belong to MSH-49 through MSH-54, and the initial learned evaluator lifecycle belongs to MSH-55.

Open questions (D-09): evaluation conditions, statistically justified decision criteria, compute/data budgets and appropriate independent opponents must be researched against available baselines. Protocol extensions are selected only for real opponents. No throughput target, training scale, chosen model or invented strength target is approved here.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Reproducible experiment tooling records code/executable/model and profile identities, parameters, seeds, limits, hardware/build configuration, starting-position distributions, colors, results and failures; paired colors and multiple baselines are used where possible.
- [x] #2 Shared native self-play/data generation and any needed Python/PyTorch bindings reuse the authoritative Rust rules and preserve complete actions and history; provenance, model/data identities and replayable regressions are retained without adding Python to the shipped application.
- [x] #3 The harness supports the six game-specific strength protocols and independently assessed search/evaluation comparisons, including actual opponent adapters where required. Rule correctness, search correctness, execution performance and measured playing strength are reported separately.
- [x] #4 Budgeted pilot measurements establish usable evidence and expose limitations before substantial training. Unresolved budgets, licensing/provenance or opponent availability are recorded as blockers for affected experiments, with no fabricated successful comparison.
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
Primary builder/integrator Codex on clean/origin-aligned primary codex/rewrite ataa68dda38c5a5380bc0faeb095e36aa3b9567ee9. No Ready task is dependency-ready; documented next48 is dependency-satisfied (25Done,M4complete), prioritized before57/58. Preserve133review/historical-accessworktree. (1) Extend existing mshogi-runner, reusing protocol4 EngineBridge and concrete native Game/ChessMatch owners for all6games/7profiles; no new process runtime or legality/adjudication. Keep actual externalUCI/USI/Xiangqi0–9 runnerfamilies functional and expose them in experiment evidence integration. Internal live path explicitlyM2/untimedChess with fullMatchRecord, no silentFull/timed downgrade or imaginarymodel/options support. (2) Versioned strict bounded experiment config/report and headless commands: code/build/hardware provenance, exe/model identity, seeded starting-distribution schedule, candidate vs multiple baseline profiles/limits, reversed colors, full native records/observations/results/failures and deterministic native replay/validation. Native JSONL data export uses same authoritative histories/actions, grouped provenance/split identities; no Pythonbinding before a realconsumer needsone. Separate rule/search correctness, execution performance and descriptive measured outcomes; no invented Elo/strengthtarget or significance from tiny/failed samples. (3) Native admission and record/identity/corruption/resource/cancel/incomplete/no-fallback controls, exact deterministic schedule and paired summaries with unscored attrition, existingexternal adapters integration/replay tests, fresh independent review. Research primary statistical/opponent/provenance sources, document pergame protocol/availability constraints under49–55/104ownership. (4) Measure standard/tactical representative small searches acrossbothprofiles before choosing finite pilot limits; run budgeted reproducible all7profile real-subprocess pilot with multipleown-baseline settings, colorpairs, genuinecompleted tactical games and honest capped/incomplete standards. Pilot is infrastructure/performance evidence, not competitive strength; no substantialtraining/model/optimization before109. (5) One structured evidence record per frozen source, required existingRustUbuntu24/macOS15/Windows2022 nativeCI only, onecompletion monitor, unrelatedautomaticDesktop cancelled/provenance preserved; no broadGUIcampaign for untouched app. Complete original4AC/4DoD only when supported, normal integrate/push/Done; hand off109 and plannedgame tasks without startingthem. Escalate only material product/rules/budget decisions beyond this boundedlocalpilot.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented existing-runner native experiment path for all6games/7wholeprofiles, paired seeded finite catalogs/multiple baselines, exact protocol4 observations and native history replay, grouped JSONL data with missing terminal labels, and actual existing external adapter wrappers. Initial independent review found Chess/later Shogi-Xiangqi PV admission, typed observation cap and post-action representation crossing gaps; corrected before acceptance. Actual production-child tactical test completed14/14 games across7profiles; zero-work/cancel/corruption controls passed. Preliminary dirty-source debug standard-turn measurements range13.59ms–7373.18ms; these only inform release pilot limits and are not final source evidence. Real near1MiB MatchObserve capacity refusal regression passed; routine regression now uses controlled smaller injected bound through same refusal path to avoid96s fixture construction. Required existing native CI remainsUbuntu24/macOS15/Windows2022; desktop/game rules/source untouched. Final release pilots, frozen-source receipts and evidence reconciliation pending.

Frozen implementation9a164eb4379231dcb4fca484adb7ec5597e7c991 normally pushed. Clean release build+actual56-game main pilot+56-game deterministic repeat+8-game Tenjiku supplement passed native evidence replay. Main20concluded/28caps/8incomplete; supplement8concluded at619visits first iteration, depth2/2000 preserves completeddepth1 per existing contract. Generated84JSONLrows,76appliedtargets,20terminal labels; independent reviewer verified allrow bindings/root chains/splits/labels/exe SHA and durable compressed reports. No remaining material source/pilot blockers. docs/m5/experiments.md and single verification.json ledger retain allconditions/limitations, no training/strength inference. Required frozen-source RustCI42432584001 currentlyrunning actualUbuntu24/macOS15/Windows2022 jobs128584504212/128584504413/128584504572, all acquired runners; no retry campaign. Automatic unrelatedDesktop42432584003 cancellation requested. Final AC/DoD and final evidence reconciliation remain pending successful3OS receipts.

Final independent acceptance approved after all3 required native jobs succeeded at9a164eb, firstattempt: Ubuntu128584504212,macOS15 128584504413,Windows2022 128584504572 (run42432584001). Everyrequired step passed; actualrunners acquired. Initial30minute observation window ended whileWindowsactive; sequential continuation immediately observed success, no retries/duplicate campaigns. Production diff from9a empty; accepted requirements unchanged. Supplemental8-row Tenjiku export independently verified. Original4AC/4DoD checked with this evidence; code/pilot alreadyintegrated and normallypushed, final acceptance commit records onlycompletionmetadata. Stop48;109 next.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Delivered offline shared experiments in the existing mshogi-runner for six games/seven accepted whole profiles. Actual protocol4 subprocesses, complete native actions/history, paired seeded catalogs, multiple baselines, strict replay, bounded work/cancellation, provenance and grouped JSONL data reuse authoritative native owners. Existing actual UCI/USI/Xiangqi UCI0–9 adapters remain functional. No rules, search/evaluation, engine protocol, application runtime or product architecture changed.

Exact tested implementation 9a164eb4379231dcb4fca484adb7ec5597e7c991 is integrated/pushed on codex/rewrite; pilot evidence published at 9f0312a. Rust CI42432584001 passed Ubuntu24.04/macOS15/Windows2022, all13steps on attempt1. Independent source, pilot/data and final evidence review approved all original4AC and applicable DoD; material admission/strict-decoding/capacity findings were resolved before freeze. Main56-game pilot retained20native conclusions,28unscored caps and8unscored Tenjiku incompletes;56-game rerun matched semantically. Measured bounded8-game Tenjiku supplement concluded all8 without changing product limits. Actual exports total92rows with complete native histories,84accepted action targets and28terminal labels. Small compressed source reports allow regeneration without committing generated training sets.

See docs/m5/experiments.md and docs/m5/evidence/msh48/verification.json for commands, identities, precise platform receipts and limitations. No competitive strength, significance, learned model, substantial training or external-opponent success is claimed. Missing appropriate opponents, exact binary/model/data rights and larger budgets remain blockers for affected later experiments. MSH-109 is the next dependency-ready handoff before substantial49–55/104 work; it was not started. Final acceptance publication contains only task/evidence/documentation metadata after the tested source.
<!-- SECTION:FINAL_SUMMARY:END -->
