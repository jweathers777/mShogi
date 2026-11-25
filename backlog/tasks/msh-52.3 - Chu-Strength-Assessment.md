---
id: MSH-52.3
title: Chu Strength Assessment
status: In Progress
assignee:
  - '@claude'
created_date: '2026-11-24T21:07:00Z'
updated_date: '2026-11-25T06:32:00Z'
labels: []
milestone: m-5
dependencies:
  - MSH-52.2
documentation:
  - docs/m5/strength-roadmap.md
  - docs/m5/chu-strength-harness.md
parent_task_id: MSH-52
ordinal: 247000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Measure the accepted Chu engine with internal time odds and a much deeper same-family blunder screen. John requested these next steps after the evaluation round; no external opponents or absolute rating claim.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A frozen internal scaling curve reports paired outcomes, uncertainty, search depths and all unfinished games.
- [ ] #2 A deeper-search screen reports coverage, clear blunders, classes and reusable positions with a held-out split; rules are fixed before results.
- [ ] #3 Any general fix follows the declared fix rule and predecessor test; checks, costs, run identities and cleanup are recorded.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Freeze accepted product e2658c7 (binary83d00cff) from MSH-52.2. Reuse native runner and opening catalog; no engine change for measurement. Cloud c3-standard-4 only, at most US$10 and two hours of aggregate campaign wall time before a result/stop checkpoint.
2. Scaling: 2x versus1x clock at bases2.5,5,10 seconds, increment base/100, nominal allocation. Same seeds1..32 at each clock, both colors; two bins of16 pairs per clock, four concurrent pairs per VM. Separate same-clock control at10s+0.1s, seeds1..16, two bins of8 pairs. Fixed maximum, no SPRT early stop (min-pairs above maximum); existing stop-on-unscored per bin,1000 steps/game,600s pair watchdog,1500s/bin. Preserve and report every launched pair and failure; estimates conditional on fully scored pairs, no absolute Elo. One campaign, not selective reruns.
3. Deeper screen: use all control games, including unfinished prefixes. Select up to16 evenly spaced played moves per game by index before seeing scores. Search each selected position and its immediate successor with full native history, fresh table,5 million nodes, depth64; use native terminal results. Record player and judge depths/nodes to test whether the judge is materially deeper. Pilot runtime only may reduce the number of samples uniformly before looking at blunder results; never select on score. At most three30-minute judge campaigns within total cap.
4. Freeze rules: p=1/(1+exp(-cp/600)), native wins/losses1/0, native draws0.5, mate scores1/0. Clear blunder p_before>=0.30 and loss>=0.30; mistake p_before>=0.20 and loss>=0.15. Classes first match: time_trouble(clock<1000ms), repetition(exact native repetition key seen earlier), allowed_short_mate(opponent mate<=9 plies), missed_capture(judge captures but played does not), lion_refutation(opponent best move captures a Lion), compound_refutation(opponent best move captures multiple pieces), promotion_refutation(opponent best move promotes), other_capture(opponent best move captures), other. Classes describe symptoms; a class alone does not prove a mechanism.
5. Fix rule: at most one general mechanism accounting for>=30% of clear blunders warrants a fix round; diagnose native lines first. A fix needs its own predecessor SPRT and tactics; otherwise keep product unchanged and report limits. Retain reusable error positions and full-history references under GCS, split by opening SHA256 modulo4 to avoid cross-color leakage. No run output committed.
6. Finish checks and durable tool documentation, record result/cost/cleanup in task, integrate. Leave the final Chu baseline stop decision to John.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Checkpoint: branch codex/msh-52.3-chu-assessment; frozen scaling/control campaign msh213-msh523-scaling is running at gs://mshogi-experiments-216675633801/runs/msh-52.3/scaling. Added full-history sampled chu_judge tool and shared-rule Python classifier; two Rust and three Python tests pass, focused Clippy passes. Shortest-clock bins stopped after first four concurrent pairs: predecessor-side (1x clock) returned no completed iteration, sometimes zero nodes; keep these unscored. This is a time-control measurement limitation, not a native-rule failure. Next finish campaign, build judge in cloud, measure sampled full-history positions and diagnose classes.

Frozen judge implementation source e36cf8393b87854334049cdafdfc292ca5aa1521; source archive uploaded to build-judge, awaiting scaling controller completion before starting the single controller build. Rust judge tests2/2 and Python assessment/Tori regression tests11/11 pass. Added opening-cluster bootstrap intervals (2000 draws, seed523) for sampled blunder rate and mean loss; no result-based sampling changes. Local workspace checks running. Judge orchestration uses32 shards/four processes per c3-standard-4 VM,16 evenly spaced moves/game,5M nodes before and after.

Before full judge results: rename the planned allowed_short_mate symptom to allowed_short_win. Chu WIN_SCORE denotes a native win, not specifically checkmate (royal capture and other native conclusions also use it); threshold remains nine plies, class order and all probability rules unchanged. This corrects the label, not a game rule or sample. Control bins finished:16 pairs launched,15 fully scored,31 native conclusions; all32 played prefixes remain eligible for the fixed512-move sample.

Scaling1 complete, all resources verified absent; conservative compute boundUS$0.43843. At bases2.5/5/10s (increment base/100),2x-clock scored WDL2/0/0,17/0/1,34/0/4 from1/9/19 pairs; launched8/12/20. Conditional estimates:2.5s unusable,5s+492 Elo (95% lower+215, upper unbounded),10s+372 (lower+226, upper unbounded). First-iteration interruptions caused7/3/1 unscored pairs. Late median depths2x/1x:4/3,5/4,6/5. Control16 pairs:15 scored11W/19L, one1000-step stop;31 native conclusions. Its equal-engine interval includes zero. Source and conditions unchanged.
Engineering extension fixed before new results: these clocks do not reach the harness depth8 target. Add20s+0.2s and40s+0.4s,2x time,16 pairs each, new seeds33..48, two8-pair bins/level, four workers,1000steps,900s pair watchdog,1400s/bin,30-minute campaign. This adds new longer controls, not reruns or score-based replacement of the first campaign. Keep all earlier results. Fixed control sample remains all32 games from the first campaign. At most two judge campaigns now; total actual campaign wall time remains capped at2h andUS$10. Judge cloud build is active frome36cf83.

Validation after origin/main merge (already current): fmt and all-target workspace Clippy pass;1188 workspace tests pass,12 ignored; three Chu judge tests and twelve Python assessment/Tori regression tests pass. Native report smoke test: four sampled move pairs parsed/replayed and judged at1000 nodes; smoke scores excluded from the full screen. Runtime judge source remains e36cf83; later Rust change adds only a terminal-perspective test. Longer-clock campaign will share its eight-VM allocation with the first16 judge shards (four game VMs plus four judge VMs); remaining16 judge shards run after it. Same samples/nodes, no score-based scheduling.

Judge build succeeded and its VM/disk/job are verified absent. Binary SHA256 f4ecf3177391f1b51e6987fb3a4c7d9657190528247172a1ad76d93c2abbb995, sourcee36cf8393b87854334049cdafdfc292ca5aa1521; core/search source differs from accepted e2658c7 only in one documentation comment. Accumulated conservative compute boundUS$0.56912. Campaign msh213-msh523-long-scaling now active: four longer-clock bins plus judge shards0..15, GCS runs/msh-52.3/long-scaling. All16 control reports hash-pinned in judge-inputs. Next collect full longer-clock results and first256 sampled moves; run remaining judge shards16..31, then assess all512 together. Before full judge results, allowed_short_win also includes an already concluded opponent win (zero plies); tests cover it. No engine changes.

Longer-clock orchestration error diagnosed before games: all four match commands rejected --pair-wall-ms900000 because native_match accepts at most600000. No game configs or reports were created; judge shards0..15 are unaffected and continue. Keep failed command logs/manifests. Correct only the watchdog to600000 and run the same planned20/40s seeds33..48 in fresh campaign long-scaling-2, together with remaining judge shards16..31. No score-based reruns and no engine/rule changes. This replaces the remaining standalone judge campaign, so the total2h/$10 caps are unchanged.

Clarification from the completed MSH-213 harness: it explicitly accepted an affordable late-depth target5, with control median6. Eight is the roadmap ideal, not an unmet Chu acceptance criterion. The extra20/40s controls broaden the time-scaling range because the first levels are heavily censored; they do not impose a new depth requirement.

Runtime checkpoint: long-scaling judge bin7 (shards12..15, final quarter of each game) completed all64 rows, each shard exit0 in795..938s. Earlier/middle-game shards0..11 remain active. No full-screen blunder scores inspected yet. Controller session owns job msh213-msh523-long-scaling and handles cleanup; after it finishes run prepare-long-scaling-2.py and submit fresh job msh213-msh523-long-scaling-2 with the same pinned worker. Scripts and manifests are in ignored runs/msh-52.3. Corrected campaign runs the unstarted20/40s pairs and judge shards16..31. All product code unchanged; final checks already pass.

First judge campaign reached the controller wait deadline. Only bin7/shards12..15 uploaded64 complete rows; the other192 planned rows were not preserved by the command-level uploader. Do not claim them judged. Cleanup command failed with gcloud SIGPIPE(-13); independent gcloud job/VM/disk lists then all returned empty. Reconciliation receipt confirms absence and bounds this stage atUS$0.75492; cumulative boundUS$1.32404. No native search error was reported, but incomplete shard outputs are unavailable.
Recovery changes scheduling only: same512 fixed positions, same5M-node searches, same frozen binary. Run each sampled move as one helper invocation against its original report, upload its completed row immediately to judge-samples/sample-NNN.json, shuffle pending jobs with seed523, and stop worker submissions within1200s to leave cleanup time. Keep the64 completed rows; remaining448 are queued without inspecting blunder scores. Corrected long-scaling-2 runs the previously unstarted20/40s games plus four judge VMs; a final eight-VM judge-finish batch will process only missing rows if needed. This replaces the two long, uncheckpointed shard runs; total actual campaign time remains below2h and spend belowUS$10. Orchestration checkpoint archive: GCS runs/msh-52.3/checkpoint-scripts-cc0014a.tar.gz; current recovery scripts will be archived at the next checkpoint.

Recovery checkpoint: corrected job msh213-msh523-long-scaling-2 is running at runs/msh-52.3/long-scaling-2. All eight workers started; no command failures so far. judge-samples contains118 of512 exact planned rows and grows per completed move. Current orchestration and reconciliation are archived at gs://mshogi-experiments-216675633801/runs/msh-52.3/checkpoint-recovery-scripts.tar.gz. After cleanup, summarize-scaling.py long-scaling-2; prepare-recovery.py judge-finish creates a fresh eight-VM queue of only missing rows. Then download judge-samples and run verify-screen.py: it checks all512 source identities, exact history prefixes, actions, player observations and clocks before scoring. Do not score partial color/phase subsets. Branch codex/msh-52.3-chu-assessment; accepted engine remains unchanged.

HANDOFF FOR ANOTHER AGENT — requested by John; continue this task, do not start a new assessment.

State and scope
- Task MSH-52.3 (Chu Strength Assessment), In Progress, @codex. Branch codex/msh-52.3-chu-assessment; code/checkpoint HEAD20c571e before this handoff commit. Main remains db9875b. Only this task work is in the worktree.
- Accepted product is frozen e2658c7, engine SHA25683d00cff4b9ab07c396a806f37aff5b2a6d68f2fd1b8481963ad6ff8670ef597. No product/search/rules changes. Judge source e36cf8393b87854334049cdafdfc292ca5aa1521, SHA256f4ecf3177391f1b51e6987fb3a4c7d9657190528247172a1ad76d93c2abbb995. Core/search diff is only one comment.
- Implemented chu_judge Rust example (full native report replay, exact history prefixes, fresh searches before/after each sampled move), chu_assessment Python classifier, tests and durable harness docs. Frozen rules and fix rule are in the plan. Keep the512 selected moves and5M nodes; do not select by scores or invent labels. Do not inspect partial blunder totals as final evidence. Same-family judge, score-derived probability proxy, no absolute rating or external opponents.

Durable recovery material
- GCS root: gs://mshogi-experiments-216675633801/runs/msh-52.3/.
- handoff-20c571e.tar.gz contains current orchestration scripts, manifests/configs, all16 control reports, first64 completed rows, final check logs and controller ownership/cleanup receipts. Extract under runs/msh-52.3 if local files are missing. Source/tool code is committed. GCS judge-samples/sample-NNN.json is the authoritative growing set of completed rows;149 were saved at this snapshot.
- Local controller Python: runs/msh-52.3/venv/bin/python. Recreate only if missing: python3 -m venv runs/msh-52.3/venv; install tools/experiments/requirements-controller.txt there. Heavy compute stays in Cloud Batch, c3-standard-4. No new images or subscriptions were created.
- Worker image: us-central1-docker.pkg.dev/mshogi/mshogi-experiments/worker@sha256:c15692c83e248b70e8c758662674559691ae4f362141dc7b045cd7f3c122f191. Existing subscription projects/mshogi/subscriptions/mshogi-pilot-completion. Use one controller at a time.

LIVE JOB — do not duplicate or cancel it while its controller is working
- Job msh213-msh523-long-scaling-2, us-central1, project mshogi. Local out runs/msh-52.3/long-scaling-2-controller. Current execution session3566 if available in the same chat. Manifest/config have the same stage prefix; intent.json records the ownership label.
- Four20/40s time-odds game bins and four checkpointed judge workers. At the snapshot it had run about12 minutes. 40s bins2/3 finished:8 launched pairs, only1 fully scored;7 pairs hit the600s pair wall budget during their second game. This is experiment truncation, not a native loss or a search failure. Keep all records and do not rerun these selected pairs. 20s bins0/1 were still running. Judge workers save each completed move immediately to judge-samples and stop within1200s.
- Controller normally deletes resources, verifies absence, then downloads artifacts and exits. Nonzero exit is expected if a game bin stops on an unscored pair; inspect actual reasons. If it is no longer running, inspect job status and ownership before cleanup; do not blindly resubmit. If cleanup verification fails, use fresh gcloud job/instance/disk list commands for this exact job and retain a reconciliation receipt. Delete only the owned job if it remains, then verify VMs/disks absent. Previous stage had a gcloud SIGPIPE cleanup check error, resolved by independent empty lists; no prior resources remain.

NEXT COMMANDS after the live controller has finished and cleanup is verified
1. PYTHONPATH=. runs/msh-52.3/venv/bin/python runs/msh-52.3/summarize-scaling.py long-scaling-2
   Inspect every unscored reason. The script checks summary/report hashes and records pooled and per-game late depths. Results at2.5/5/10s are already in scaling-result.json and task notes.
2. Count judge-samples. If fewer than512, run:
   PYTHONPATH=. runs/msh-52.3/venv/bin/python runs/msh-52.3/prepare-recovery.py judge-finish
   Then submit with tools.strength.chu_cloud using --manifest runs/msh-52.3/judge-finish-manifest.json --config runs/msh-52.3/judge-finish-config.json --image [pinned worker above] --job msh213-msh523-judge-finish --out runs/msh-52.3/judge-finish-controller.
   This creates eight VMs with a shuffled queue of ONLY missing rows; each invokes the unchanged judge on one original report and one selected move. New rows are saved immediately. Do not reuse an already-submitted stage or overwrite immutable GCS inputs. The prepare script asserts spend stays belowUS$10 and uses reconciliation for the earlier failed cleanup bound.
3. After all512 samples are stored, download them into runs/msh-52.3/judge-samples/ (GCSStore list/get; parallel reads are safe). Run:
   PYTHONPATH=. runs/msh-52.3/venv/bin/python runs/msh-52.3/verify-screen.py
   It proves exact coverage, source-report identity, record prefix, played action, player observation, clock and side change, then writes screen-result.json and blunder-suite.json. Do not treat absent rows as clean moves. If the final batch is incomplete, inspect runtime and finish missing rows within the remaining task caps; record any limitation honestly.
4. Assess full-screen depth/node ratios, blunder rate and opening-cluster intervals. Diagnose any >=30% symptom class for a concrete general mechanism; class frequency alone is not a mechanism. Inspect development-split lines first if considering a fix. A retained product fix needs predecessor SPRT and tactical checks; otherwise keep the accepted engine unchanged. Leave the final Chu baseline stop/continue decision to John.
5. Update task/parent notes and roadmap status, upload final evidence/suite/costs, integrate main, mark this subtask Done only with evidence, and clean own branch, target, runs/msh-52.3 and generated Python caches. Do not mark parent MSH-52 Done on behalf of John. No GitHub CI was requested; no platform-sensitive product code changed.

Checks and budget
- Already passed after merging current main: fmt; workspace all-target Clippy -D warnings;1188 workspace tests,12 ignored;3 Rust judge tests;12 Python assessment/Tori regression tests. Main was unchanged at last fetch. Later edits are Python reporting/tests and task/docs, tested. No fresh subagent review was required for these additive tools.
- Before the LIVE job, conservative compute boundUS$1.32404: scaling0.43843, build0.13069, first judge/failed longer-game stage0.75492 (reconciled bound). Two-hour aggregate campaign cap andUS$10 spending cap; roughly52 campaign minutes had elapsed before the live job. Configured rate0.201USD/VM-hour; bounds use eight VMs even for a smaller build, not actual billing.
- Earlier long-scaling failed game commands before any games because900000ms exceeded the driver maximum600000. Corrected current stage uses600000. Its first judge run lost unuploaded partial shards when the controller deadline expired; only64 rows were saved. The per-move checkpoint recovery fixes that orchestration issue without changing any sample, score, engine or rule.

HANDOFF UPDATE — live job is now msh213-msh523-judge-finish (execution session8456 if available), not long-scaling-2. Its out directory is runs/msh-52.3/judge-finish-controller. The previous job finished; cleanup verified no job/VM/disk remains. Saved227/512 samples; the new eight-VM queue contains only the285 missing rows. No full-screen scores inspected. After this controller finishes, download judge-samples, verify exact coverage with verify-screen.py, then assess and finish as in the full handoff above. Current conservative compute bound before this jobUS$1.92722; all earlier stages absent, including the independently reconciled stage.
Validated longer-clock result:20s+0.2s versus40s+0.4s,16/16 scored pairs,31W/0D/1L; conditional relative estimate+596.5 Elo (95% lower+323.5, upper unbounded). Median late depths2x/1x7/6, both pooled and per-game. At40s+0.4s versus80s+0.8s, only1/8 pairs scored (1W/1L); seven second games stopped at the600s pair experiment budget. This point has no useful strength estimate. The fixed ten-minute runner/driver ceiling is separately tracked as MSH-238 (Remove the fixed ten-minute experiment cap), Ready and unassigned; do not expand this task to implement it.

Latest full handoff archive: gs://mshogi-experiments-216675633801/runs/msh-52.3/handoff-judge-finish.tar.gz. It includes the new live-job ownership intent/manifest, completed long-scaling-2 result and cleanup receipts, scripts and control inputs. All eight judge-finish workers have started. The source-history verification also passed on all four local smoke rows; those low-node smoke results remain excluded.

Handoff taken by @claude after Codex stopped. judge-finish finished (SUCCEEDED, 22.8 min, bound US$0.59716); cleanup verified absent, independent job/VM/disk lists empty. Saved 511/512 rows; only sample-352 was not started (worker 2 reached its 1200 s stop limit). Per-move runtime max 262 s. Cumulative bound before final stage US$2.52438; campaign wall about 89 min of 120. Final stage msh213-msh523-judge-finish-2: one c3-standard-4 VM, same pinned worker and binary, queue contains only sample-352. prepare-recovery.py now also accepts stage judge-finish-2 (one worker); no sample, node or rule change. Next: download-screen.py, verify-screen.py, assess.
<!-- SECTION:NOTES:END -->
