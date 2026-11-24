---
id: MSH-52.2
title: Chu Evaluation Round
status: In Progress
assignee:
  - codex
created_date: '2026-11-24T19:15:00Z'
updated_date: '2026-11-24T20:06:00Z'
labels:
  - chu
  - evaluation
milestone: m-5
dependencies:
  - MSH-52.1
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 245000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace the untuned Chu material and placement weights with a measured classical evaluation, using the shared Texel fitter and native self-play outcomes. This is the evaluation step John requested after the selectivity round; no neural evaluation or external opponents.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 About 500,000 or more quiet self-play positions have native outcome labels, a split by game or opening, duplicate control and recorded held-out loss before and after fitting.
- [ ] #2 Evaluation features agree with the search score through moves, captures, promotion and undo; native correctness and tactical checks pass.
- [ ] #3 Kept weights have supporting predecessor SPRT evidence at H1 about 10; unsupported candidates are reported and reverted.
- [ ] #4 Required checks and review are complete, durable documentation is current, and run identities, costs and resource cleanup are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add a Chu model for the shared Texel fitter, preserving the current score as INITIAL and sharing fitted material with static exchange. Test feature/search agreement and undo.
2. Add native self-play corpus generation; measure a small cloud pilot before scaling to about 500,000 quiet labelled positions. Freeze game/opening splits and remove duplicates.
3. Fit static material and placement first; measure held-out loss, tactical results and search cost. Add per-node terms only if evidence supports the extra work.
4. Run the candidate against frozen MSH-201 source 79c2409 at the established fast time control, SPRT 0/10. Keep only supported changes.
5. Complete required checks and fresh review, record costs and run identities, integrate and remove task resources.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint: branch codex/msh-52.2-chu-evaluation. Added an 88-weight static material/advance/central-file/tempo model; DEFAULT still INITIAL. Shared material feeds SEE and capture ordering. Added shared-fitter adapter and native self-play tool with no labels for unfinished games. Initial feature/move/undo tests passed. Next: cloud corpus pilot (500/2000 nodes per turn, native outcomes), then freeze production protocol after measuring yield. No cloud spend yet; no candidate weights fitted.

Fresh review of 55191fe..3913370 found one issue: even-ply sampling gave only Sente-to-move inputs. Fixed to sample every ply; pilot source 3913370 is for yield measurement only, not fitting. Reviewer found no other concrete defects. All three chu_texel tests pass, including old-placement agreement for every piece and square. Workspace Clippy passed. Cloud build build-msh522-tools started; source 391337067e07610bf52a4f576a7cd0c0c1e3e5c8; GCS runs/msh-52.2/build-tools.

Pilot source 3913370: 500 nodes produced 0/16 concluded games; 2000 nodes produced 2/16, 539 even-ply quiet samples. Other games stopped with no completed first search iteration. Pilot data will not be fitted. Added doubling node retries for these turns (up to existing search maximum, within game deadline); test proves a 1-node starting allowance obtains a native move and unfinished games remain unlabelled. Shared splitter now streams two passes and retains only board hashes/partition masks; 9 Python tests pass and fresh review found no defect. Workspace tests passed 1188 with 12 ignored; fmt and Clippy passed before the retry edit. Both initial cloud jobs/VMs/disks deleted. Conservative compute upper bounds: build $0.13169, pilot $0.06766 (actual lower). Next build corrected generator, measure full-game yield, then production corpus and fit. Branch codex/msh-52.2-chu-evaluation.

Corrected pilot source 80f32b643c193a0614be81e52ce3130a5b2c12c8: same 16 openings, 60-second/game and 2000-ply observation stops. At 2000 initial nodes: 14 native outcomes, 2 unlabelled ply stops, 4379 quiet rows, 138.28 aggregate CPU-task seconds. At 8000: 16 outcomes, 3633 rows, 447.38 seconds. Choose 2000 with doubling retries for corpus yield; no strength claim from node count. Pilot2 data excluded from fitting.
Production protocol frozen before fitting: source 80f32b6, 2000 starting nodes, retry up to current engine maximum only if no first completed iteration; sample every quiet ply; only native conclusion labels; 60 s/game, 2000 plies. Request 3072 games across 8 c3-standard-4 VMs, four processes each; stop starting games near the campaign deadline. Use all completed native outcomes, retain unlabelled stops. Existing 128-opening catalog; every randomized continuation stays with its catalog opening; SHA256 opening split (0 heldout, 1 validation, rest training) and global duplicate removal. Require >=500000 unique quiet positions before fitting. Static model bounds and K range in 80f32b6 are frozen; fit training/validation for at most 900 seconds, then check heldout. Shared splitter uses current 80f32b6 streaming implementation. Resources/run files remain under runs/msh-52.2 and corresponding GCS prefix.

Corpus1 complete: 3072 games; 3004 native outcomes, 68 unlabelled 2000-ply stops; 733621 quiet samples before global deduplication. No wall stops or search failures. Source80f32b6, 2000 starting nodes with retries, eight c3-standard-4 VMs/four processes each. Sum of per-game elapsed times22956.48s; this is elapsed process time, not measured CPU time. GCS runs/msh-52.2/corpus1 contains32 compressed game files, summaries and progress. All production job/VM/disk resources verified absent. Conservative accumulated compute upper boundUS$0.89281.
Pilot pipeline check (excluded from fitting): streaming split retained4303 unique samples, both turns present; frozen model stats accepted all4303 with zero duplicate rows. The feature test checks material/placement agreement and undo, and now explicitly checks capture and promotion coverage.
Fitting job msh213-msh522-fit-static is active at runs/msh-52.2/fit-static. It first compares the18 fixed predecessor positions at depth5/3 repeats for exact unfitted search-result agreement, then globally splits the production data, requires500000 positions, fits static weights for<=900s and checks heldout separately. Binary source80f32b6; orchestration and input hashes are in fit-manifest.json under runs/msh-52.2 (and GCS manifest). Next: inspect fit/heldout loss, install supported candidate weights, build/probe, freeze match batch sizing before game results, then predecessor SPRT0/10. Branch codex/msh-52.2-chu-evaluation; no fitted weights installed yet.

Match protocol fixed before candidate game results: compare the fitted candidate with frozen79c2409, c3-standard-4,10s+0.1s nominal,1000 experiment steps,600s pair watchdog. Up to four30-minute campaigns, eight bins of32 paired seeds each, four concurrent pairs per4-core VM. Seeds1..256 then257..512,513..768,769..1024; frozen128-opening catalog cyclically, both colors. Use the existing native_match stop-on-unscored policy in each bin; preserve every launched pair and diagnose all failures/cutoffs. Pool complete campaigns only: SPRT H0=0/H1=10,alpha=beta=.05,min30. Stop on H0/H1 or declared maximum; revert unsupported weights. No multi-hour extension. Existing tactical probes use20ms; fixed cost set is18 positions at depth5/3 repeats. No external opponents or rule changes.
<!-- SECTION:NOTES:END -->
