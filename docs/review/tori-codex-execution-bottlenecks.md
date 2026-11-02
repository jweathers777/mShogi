# Where time went inside the Tori Codex assignments

Follow-up forensic report, 2 November 2026. Scope: **MSH-126–131**, with **MSH-85 final acceptance** and **MSH-77 planning** as comparisons. Every interval is inside one of those original assignments. All displayed clock times are **Europe/Paris (UTC+01 on these dates)**. No time between assignments, owner response delays or broader development calendar is included.

## Executive summary

A typical 33–51 minute run in this sample contains roughly **4–7 minutes before its first source write, 11–21 minutes from that write to its first source commit, 13–22 minutes from final source to its selected CI checkpoint, and 2½–5 minutes afterward**. These are measured, nonoverlapping **mixed activity envelopes**, not an invented allocation of every minute to coding or waiting. Tests, review, corrections and evidence work overlap within them.

**MSH-131 was longer primarily before its first source commit.** It spent **61:05.516 from first runtime write to that commit**, versus 11–21 minutes for the other tasks. This period contains actual controller integration, repeated compile/test iterations, substantive reviewer findings, Save preparation changes, stale-work/failed-replacement fixes, capacity tests, process instrumentation and evidence preparation. It then spent **17:12 in two corrective-source intervals**, **19:33 from final tested source commit to final CI job**, and **7:02.841 after that job**. Pure coding, reasoning and evidence-authoring time cannot be separated reliably inside those envelopes.

The earlier report's **107:54.314** for MSH-131 stopped at integrated commit `c89802e3e6961eaf8448e59794998e61a277da91`. For this question, its actual task-start→task-complete runtime is **109:05.155**. The final **70.841 seconds** include push/check/report work. The other five runtimes are 50:51.388, 32:54.645, 45:24.258, 43:57.989 and 48:24.617. Together the six assignments total **330:38.052**.

Local command execution alone does not explain those runtimes. Captured test-containing commands from the primary and original reviewers occupy **25:47.174 across the six tasks**, or **26:09.248 including the comparisons**, after merging overlap. Including formatting/lint/check/verification commands raises the latter to **27:37.435**. These are process lifetimes, sometimes mixed with edits and including compilation/locks, not pure assertion time or time writing tests. MSH-131's two broad session runs took **182.485 seconds failed** and **188.106 seconds passed**; its total captured verification union is **11:09.315**. They are material iterations, but cannot account for its full pre-commit hour.

The clearest completion dependencies were **CI, not a long sequential reviewer handoff**. Reviewers generally ran during implementation. No primary `wait_agent` calls appear. CI-dependent phases also contain useful evidence/finalization work, so exclusive necessary CI waiting is **unknown**, not the full workflow duration. Actual primary pauses are measurable: **43 explicit sleeps totaling 35:10.106**, including **3:09.134 after the selected CI checkpoint had already finished**. In MSH-131 a background watcher had completed before the primary's final 55-second sleep ended.

The highest-value within-run improvements are: catch separate-workspace and prior-stage contract errors before broad campaigns; avoid rediscovering allowed resource refusal as a CI “failure”; overlap focused review with evolving implementation; use one completion-aware CI monitor; and prepare one concise acceptance record from structured results. The data supports these mechanisms, not a precise promised number of minutes saved or eliminating independent review.

## 1. Evidence and measurement rules

This analysis reuses the original read-only Codex SQLite/JSONL records and actual GitHub REST run/job/step timestamps, then examines command contents, process completion, test summaries, reviewer messages and tool-call returns in more detail. The primary thread is `01a16fa6-da21-7b40-8b4b-ac86c437071f`. Its rollout is `~/.codex/sessions/2026/10/24/rollout-2026-10-24T22-42-48-01a16fa6-da21-7b40-8b4b-ac86c437071f.jsonl`. `thread_history_1.sqlite` and `state_5.sqlite` were accessed read-only. No internal reasoning content is used or reproduced; reasoning/composition duration is not independently instrumented.

The [follow-up source bundle](evidence/tori-codex-execution.json) preserves exact UTC task events, reconciled phase boundaries, original reviewer windows, primary sleeps/polls, sanitized executable verification lines and command endpoints/results. The [original API bundle](evidence/tori-time-analysis.json) retains exact workflow/job/step timestamps and source hashes; those historical runs are reused, not rerun. The [earlier report](tori-development-time-analysis.md) supplies broader provenance but its between-assignment analysis is outside this report.

Three timing domains must remain distinct:

1. **Primary wall-clock phases** partition task-start→task-complete exactly. Phase labels describe observed mixed work. A source commit is a milestone, supported by earlier writes/test/review events, not evidence that all preceding time was coding.
2. **Activity/process lifetimes** can overlap: local subprocesses, reviewers and CI continue while the primary does other work. Merge overlapping intervals before summing; never add those columns to the phase table.
3. **Primary direct pauses/awaits** show when a tool call had not returned. An async Cargo invocation can return a process ID in a second while its process runs for three minutes. The same applies to the 17-minute CI watcher. Its entire process lifetime is not a 17-minute primary tool await.

Gaps between observable actions may contain source composition, reasoning, output interpretation, background execution or other activity. They are not labelled idle. Mixed heredoc-edit→format→test commands remain mixed. A successful final shell exit can mask an earlier failed test in that shell; test summaries are examined too. Readbacks of prior logs are not fresh test executions. No log modification time is used.

Exact JSONL task-event subtraction is authoritative for runtime here; projected `duration_ms` differs by milliseconds, e.g MSH-131 records 6,545.136 seconds versus JSONL 6,545.155. API endpoints have second precision. Arithmetic uses raw precision; prose rounds when indicated.

No new test, build, frontend or GUI campaign was run. No production/workflow code or task status was changed. Current MSH-132 work and unrelated worktrees were preserved. This is an investigation, so no new Backlog task was needed or created.

## 2. Reconciled wall-clock breakdown

### Nonoverlapping phases: the additive view

Durations below are **minutes:seconds.milliseconds**. All five phase columns add exactly to runtime using source precision. The “before first write” column includes inspection/planning **and composition of that first change**; it is not a pure startup tax. The development column includes authoring, tests, corrections, review coordination, documentation and reasoning. “CI phase” includes dispatch/monitoring/evidence and may include review; it is not exclusive CI waiting. “Finalization” includes result-observation lag, evidence, review reconciliation, Backlog/Git and response composition.

| Task | Total runtime | Before first source write | First write→initial source commit | Post-commit corrective source | Final source→chosen CI checkpoint | Checkpoint→task complete |
|---|---:|---:|---:|---:|---:|---:|
| MSH-126 | **50:51.388** | 6:57.440 | 19:27.807 | 6:22.000 | 12:58.000 | 5:06.141 |
| MSH-127 | **32:54.645** | 4:11.265 | 11:21.333 | 0:00.000 | 14:11.000 | 3:11.047 |
| MSH-128 | **45:24.258** | 6:53.689 | 17:41.153 | 0:00.000 | 16:07.000 | 4:42.416 |
| MSH-129 | **43:57.989** | 5:28.078 | 18:17.641 | 1:25.000 | 14:36.000 | 4:11.270 |
| MSH-130 | **48:24.617** | 3:41.678 | 20:37.444 | 0:00.000 | 21:36.000 | 2:29.495 |
| MSH-131 | **109:05.155** | 4:11.798 | 61:05.516 | 17:12.000 | 19:33.000 | 7:02.841 |

MSH-130's chosen checkpoint is full desktop completion at 23:32:15. Required native CI had passed at 23:28:52 and the independent reviewer accepted its criteria with Windows GUI still pending; primary finalization already overlapped this final 3:23. The table does not assert that desktop GUI completion was necessary codec acceptance. For 126–129, the primary explicitly used native CI; automatic desktop runs were not the task's final gate.

The post-checkpoint columns sum to **26:43.210**. This is observed completion work/latency, not 26 minutes of removable administration. MSH-131 still performed final independent evidence reconciliation during its tail.

### Requested activity categories: what can actually be measured

| Category | Best supported measurement | What remains unknown |
|---|---|---|
| **A. Inspection/planning** | Initial reads/ownership checks/plan commands and the pre-first-write envelopes above; six envelopes total **31:23.948** | Pure inspection duration: source composition and reasoning also occur before the write. Repeated reads exist, but not every read is unnecessary. |
| **B. Implementation** | Timestamped source writes and development milestones in six timelines below | Exclusive authoring duration; the 61-minute 131 development envelope is not all coding. |
| **C. Local compilation/tests** | Per-task process unions in section 5; all captured six-task verification **27:15.361** including reviewer work, versus test-containing **25:47.174** | Full pure compiler/assertion time, and time designing tests/interpreting output. Mixed commands include other work. |
| **D. Debugging/corrections** | Post-first-commit correction envelopes **24:59** total; numerous earlier failure/correction events inside development | Exclusive debugging labor or avoidable fraction. Corrections and CI overlap. |
| **E. Review coordination/blocking** | Reviewer launch/verdict/process intervals; no primary `wait_agent` calls | Exclusive necessary reviewer wait is **unknown**. Reviewer lifespan is not a primary wait timer. |
| **F. CI dispatch/monitoring/wait** | Actual runs/jobs; explicit sleeps **35:10.106**; post-checkpoint sleep portions **3:09.134** | Total exclusive necessary CI blocking. Sleeps are actual pauses, but not all were avoidable; they can overlap useful external work. |
| **G. Evidence/documentation** | Evidence writes, manifest/test-count extraction and acceptance mapping before/during/after CI | Exclusive writing duration. Documentation frequently shares a command and phase with checks and Git. |
| **H. Backlog/Git/finalization** | Actual CLI/commit/push/check actions; measured post-checkpoint envelopes above | End-to-end administration time, including composing notes, is not the sum of short CLI process durations. |
| **I. Reasoning/other unclassified** | Actions outside instrumented subprocess/await intervals and mixed commands explicitly remain unallocated | No inference-speed, reasoning, keyboard or productive-labor timer exists. No residual is labelled idle. |

Thus there is a complete **wall-clock** decomposition, but not a complete exclusive A–I decomposition. The companion measurements below are deliberately non-additive.

## 3. Six assignment timelines

Dates: MSH-126–130 are **1 November**; MSH-131 is **2 November**. Final-response times below are completion of the recorded assistant message, immediately before the task-complete event, not the start of composing it. Exact UTC events/IDs are in the source bundle.

### MSH-126 — 14:39:40.753 to 15:30:32.141

| Paris time / interval | Observed primary and concurrent activity |
|---|---|
|14:39:40.753–14:46:38.193|Read instruction/policy/dependencies, inspect crate/workflow boundaries, record plan, compose first harness change. **6:57.440 mixed A/B/I**.|
|14:46:38.193–14:51|Create conformance crate and policy; first harness build/test work.|
|14:51–15:04|Compile/test corrections, growing harness cases, rotation/effect/reversal assertions. Reviewer starts **14:51:35**, finishes **15:03:12.398**, overlapping development. Test summaries include compile failure,13 pass/1 fail, and a later 15 pass/1 fail hidden by an overall successful shell exit. Final 18 harness tests pass by the 15:04 checkpoint.|
|15:06:06 /15:06:10|Initial source `134f81ba67ef81918734ed1ed1acb5de7532fb99` committed; native/desktop workflows launch. Evidence work and status reads continue.|
|15:08–15:12:28|Windows fixture hash failure diagnosed as checkout line endings; unrelated macOS runner failure retained. LF-only correction independently reviewed **15:11:17–15:11:43.470**; `d5f98a2af9e38ae690e2fd5c4940a4d26d12407b` committed 15:12:28; corrected workflows launch 15:12:32. **6:22 correction envelope**.|
|15:12:28–15:25:26|Corrected native CI/evidence/monitoring. Four 45-second sleeps total 3:00.010. Windows native is last required job. Desktop continues beyond this assignment and is not counted as blocking it.|
|15:25:26–15:30:32.141|First all-success output 15:26:20.695; evidence, criteria/notes, graph/ownership checks, completion commit/push and final response at 15:30:32.132. **5:06.141 tail**.|

**Bottleneck:** the late phase is CI-bound, with a concrete LF correction/re-campaign. Local harness command execution is under a minute captured; harness design, source composition, review corrections and policy/evidence work occupy the much larger mixed development envelope. Calling all 50:51“testing” would be wrong.

### MSH-127 — 16:54:03.402 to 17:26:58.047

| Paris time / interval | Observed activity |
|---|---|
|16:54:03.402–16:58:14.667|Task/dependency/source inspection and plan; first structural source write. **4:11.265**.|
|16:58:14.667–17:06:41.313|State/transitions/adapter/tests. Failures include diagnostic label, private test access and accepted-case partition. Review **17:04:03–17:06:41.313** requires negative tests to actually reject, rather than check nonmutation only conditionally. 202 assertions pass by 17:06 checkpoint.|
|17:06:41–17:09:36|Focused/core regressions, API-boundary verification, evidence and integration; `ce1801d7801f12e447b625739acd6a0c53aa2640`.|
|17:09:42–17:23:47|Native CI. Primary reconciles 42-vs 40 board-transition partition,202 assertions and resource-boundary handoff. At **17:21:55.511** it says completion record is ready; native CI ends **1:51.489** later. Eight sleeps total 6:30.018; last sleep straddles completion.|
|17:23:47–17:26:58.047|Success visible 17:24:32.022, then completion record/Backlog/Git/push/response. Final response completes 17:26:57.997. **3:11.047**.|

**Bottleneck:** CI is the final technical dependency; review already finished before dispatch. No post-commit code correction. Local test-containing process union is 1:47.066 including reviewer work, far smaller than the 14-minute CI phase. The 1:51“record ready” tail establishes a late CI dependency, not pure idle time for the full campaign.

### MSH-128 — 17:56:45.158 to 18:42:09.416

| Paris time / interval | Observed activity |
|---|---|
|17:56:45.158–18:03:38.847|Read structural/local boundaries and fixtures, plan nonrecursive Swallow-drop defense admission, compose local API. **6:53.689** before first write.|
|18:03:38.847–18:15:48.557|Legality/inspection/conformance plus adversarial tests. 483 cumulative assertions by 18:08 checkpoint. Review **18:12:08–18:13:47.589**; follow-up on added public token/inspection checks **18:15:19–18:15:48.557**.|
|18:15:48–18:21:20|Focused/full relevant checks, perft diagnostics, evidence and source commit `5c60ac65dc03a7f9afcaa796a2babb252329d361`. CI launches 18:21:26.|
|18:21:26–18:37:27|Native CI while primary checks fixture/perft accounting and writes handoff. Five sleeps total 4:10.012. Commentary 18:30:21.162 explicitly awaits final jobs; completion 7:05.838 later.|
|18:37:27–18:42:09.416|A stale status description at 18:38:19 still says Windows running; 18:38:21–18:39:11 sleep begins after job completion. Successful poll 18:39:16.826, then evidence/task/Git/report. Final response 18:42:09.363. **4:42.416**.|

**Bottleneck:** CI final checks and delayed observation, not review or extensive local command time. Captured test-containing union 1:10.213; there is no recorded primary failing verification command for this task. The wholly post-completion 50-second sleep is a concrete monitoring inefficiency.

### MSH-129 — 20:25:27.281 to 21:09:25.270

| Paris time / interval | Observed activity |
|---|---|
|20:25:27.281–20:30:55.359|Inspect complete histories, fixture contracts, native dispatch consumers, plan and compose type changes. Separate fixture investigation 20:28:15–20:29:26.723. **5:28.078**.|
|20:30:55.359–20:44:50|Game/record/history/native routing plus compile/adapter corrections. 713 assertions by 20:38 checkpoint. 201 session tests pass; subsequent concurrent-build doctest cannot find engine crate. Focused serial doctest rerun passes. Engine/process regressions run alongside review.|
|20:37:16–20:47:22.630|Independent review overlaps implementation/tests, adds meaningful event-limit/capacity checks. Historical-prefix API and final evidence incorporated.|
|20:49:13 /20:49:18|Initial source `e1cc89d454db5e0ce98da34bf4cda7960d395413`; CI launches. Standalone Tauri exhaustive match found outside workspace.|
|20:49:13–20:50:38|Host refusal correction, local host check, brief review 20:50:00–20:50:15.374; corrected `994521bec0637d6f8f17103f1106c0025d950753`; fresh workflows 20:50:41. **1:25 correction envelope**.|
|20:50:38–21:05:14|Corrected native CI and handoff/failure-evidence capture. At 20:53:32.201 local verification/review reported complete; native CI ends 11:41.799 later. Nine sleeps total 7:25.026.|
|21:05:14–21:09:25.270|First success output 21:06:12.109; acceptance/evidence/task/Git/report. Response 21:09:25.237. **4:11.270**. Corrected desktop GUI still runs and is not required for this task's completion.|

**Bottleneck:** final native CI; avoidable consumer discovery triggered a second source campaign. Local verification is larger than 126–128 but still 5:59.066 all captured commands. The doctest issue required a 1.877-second targeted rerun, not repeating the entire 84-second session campaign.

### MSH-130 — 22:46:19.878 to 23:34:44.495

| Paris time / interval | Observed activity |
|---|---|
|22:46:19.878–22:50:01.556|Inspect native record/document/worker admission; plan and compose codec adapter. **3:41.678**. Reviewer starts 22:46:35,16 seconds into the assignment.|
|22:50:01.556–23:02:30.098|Implement codec/admission/worker checks; first 10 tests by 22:55 checkpoint. Reservation and byte-boundary test corrections; long measurements; default Node fails before frontend tests, bundled runtime succeeds. Full 230-test session command **22:59:26.038–23:02:30.098**,184.059 seconds, overlaps native/frontend/lint/review.|
|23:02:30–23:10:39|Actual long-save/admission checks, revised reservations,17 codec checks and evidence. Initial reviewer finishes **23:10:19.943**. Source `8fd12f953beeacac46fdd417b2054bedeafe2d6e` committed 23:10:39; workflows 23:10:43.|
|23:10:43–23:28:52|CI and evidence. macOS native starts 304 seconds after dispatch; native finishes 23:28:52. Nine 45-second sleeps total 6:45.017.|
|23:28:52–23:32:15|Primary begins finalization. Reviewer acceptance reconciliation 23:30:18–23:30:48.859 approves criteria **with Windows GUI pending**. The chosen full-desktop checkpoint ends 23:32:15; not established as necessary codec acceptance.|
|23:32:15–23:34:44.495|First post-completion poll has transport error; retry returns success 23:33:27.971. Final evidence commit/push/report; response 23:34:44.466. **2:29.495 after chosen checkpoint**.|

**Bottleneck:** both substantive saveability review and long local regressions appear before first commit; native CI later limits readiness. The final GUI tail overlaps acceptance work. No CI source correction was needed. Waiting for the entire desktop workflow should be a conscious checkpoint decision, rather than assumed codec correctness.

### MSH-131 — 08:40:48.686 to 10:29:53.841

| Paris time / interval | Observed activity and reconciliation |
|---|---|
|08:40:48.686–08:45:00.484|Read task/ownership/native-document/runtime boundaries, record plan, compose plumbing. Reviewer launches 08:41:30. **4:11.798**.|
|08:45:00.484–08:57:50|Controller/worker/native-command wiring, compile/helper corrections, admission/history/unsupported-process tests; first broad controller fixture milestone reported 08:57:50.|
|08:57:50–09:11:24|Review finds synchronous Save encoding under lock; switch preparation to existing worker/precomputed close candidate. Identity/headroom and ending reservations checked. First full 241-test session **09:06:30.268–09:09:32.753** yields 239 pass/2 obsolete-contract failures. Long 360-action runtime journey reported passed 09:11:24.|
|09:11:24–09:33:24|Update obsolete prior-stage assertions; corrected full session **09:13:00.261–09:16:08.368**,242 pass. Add exact forced-cycle/history/identity/interrupted-save/floor checks, reconcile 82 consumer requirements/13 fields and prepare evidence. This 22-minute envelope is mixed; it is not 22 minutes of test execution or documentation alone.|
|09:33:24–09:46:06|Final failed-replacement race and child-start instrumentation corrected/retested; reviewer initial turn ends 09:39:30.517. Relevant process targets pass, evidence prepared; source `253e4c949614d133b5295f6a362e3d5279adcef3` committed 09:46:06, workflows 09:46:21. **First-write→commit 61:05.516**.|
|09:46:06–09:50:17|Standalone Tauri formatting CI failure; local correction/check; `ce48e84e64b9631343416acec865505dbbab570d`, next workflows 09:50:30. Handoff review 09:47:41–09:48:19.114; formatting review 09:50:54–09:51:11.271.|
|09:50:17–10:03:18|Original near-limit 360-action CI test hits allowed 15-second refusal; correct test contract, retain mandatory 180-action successful journey and exact refusal preservation. Review **09:55:46–10:03:07.963** overlaps correction/tests. Final `6a2edd71b3136a9c2043f9137eb6d3fb8abf120d` committed 10:03:18, workflows 10:03:29. Superseded workflows cancelled. Together both source corrections **17:12**.|
|10:03:18–10:22:51|Final exact-source CI, failed-run log preservation, package/test-count extraction and acceptance script. Remaining-gate statement 10:08:36.245 precedes final job by 14:14.755, but evidence work continues during that period. Native ends 10:20:01; final Windows desktop job 10:22:51. **19:33**.|
|10:22:51–10:29:53.841|Watcher finishes 10:23:25.342; primary final 55-second sleep 10:23:58.637–10:24:53.644; reads success 10:24:57.120. Final evidence review **10:26:15–10:26:57.006**, criteria/Backlog/commit/push/check/report. Commit 10:28:43; response completes 10:29:53.800. **7:02.841 tail**, including 70.841 seconds beyond integration.|

## 4. Why MSH-131 took almost 109 minutes

The measured differentiator is the **61-minute first-write→first-commit envelope**, not a 58-minute sequential review or 17-minute blocking watcher. Three concrete classes of work made it larger:

- **Real runtime boundary corrections:** Save re-encoding under the controller lock, prepared close candidates, identity/floor-width reservations, and replacement paths retiring old Tori work before a replacement had successfully validated. These are actual responsiveness/saveability/stale-work defects or gaps. Their independent review and correction are productive work.
- **Broad integration feedback:** the 182-second session run found two tests still asserting the prior stage's unsupported-Tori behavior. The 188-second rerun and added controller/process tests verify changed contracts and existing games. Compiler/helper/child instrumentation iterations are visible. The full source changed through 09:45, so elapsed time after the first passing controller tests cannot be relabelled pure paperwork.
- **Evidence and additional edge coverage:**82 consumer requirements,13 pending fields, historical saved observations, forced cycles,256→257 observation refusal, identity headroom, ending reservations, floor widening and failed-outgoing-replacement preservation. Source writes, tests, review feedback and evidence scripts interleave. Their separate authoring costs are unmeasured.

After first source commit, two avoidable assumptions caused new campaigns: the separate Tauri manifest was missed by workspace formatting; then the near-limit test demanded success when the production contract allowed resource refusal. The **17:12 correction envelope** is measurable, but it is not 17:12 of exclusive debugging or fully recoverable savings—original/replacement workflows and useful review ran concurrently. Raising the runtime limit would not have been a sound fix.

Once final source was committed, the 19:33 phase was dominated by completing the existing three-platform test/package/GUI evidence, with useful evidence preparation alongside it. Final Windows package job itself lasted 19:19, with 4:58 explicit build,4:34 composite checks and 8:00 existing-game GUI steps. Native macOS lasted 16:24, including 15:32 test/verification steps. Compilation embedded in test/check steps is not separable from those step timers.

The last 7:03 contains actual final review and acceptance work, not just waiting for a reviewer. The final review is 41.637 seconds; from that verdict to task completion is 2:56.835 using the precise task-complete event. The roughly 2:06 CI-result visibility lag includes useful log extraction; its whole 55-second sleep after both job completion and watcher completion is a demonstrated monitoring defect.

## 5. Local compilation, tests and rework

These **overlapping activity measurements must not be added to phase runtimes**. Executable command detection excludes documented commands inside heredocs. 119 captured primary/reviewer command items contain test invocations; 169 items contain captured Cargo/npm verification commands of the selected kinds. This ledger covers cargo test/check/fmt/clippy/run/build and npm check/test chains; Python provenance audits, Backlog commands and evidence authoring are outside these totals, so these are coverage-limited measurements rather than all local validation activity. Multiple binaries/commands can share an item. Zero means no captured execution in that comparison, not no review work.

| Assignment | Primary test-containing union | Reviewer test-containing union | Combined test-containing union | Captured Cargo/npm verification union |
|---|---:|---:|---:|---:|
|126|0:37.362|0:00.902|0:38.264|0:40.687|
|127|1:44.696|0:02.370|1:47.066|1:49.847|
|128|0:48.482|0:21.731|1:10.213|1:13.170|
|129|3:59.889|1:21.312|5:21.025|5:59.066|
|130|6:03.272|0:52.705|6:11.331|6:23.276|
|131|10:39.017|0:40.698|10:39.275|11:09.315|
|85 final acceptance|0|0:22.074|0:22.074|0:22.074|
|77 planning|0|0|0|0|
|Selected 8 total, overlaps merged|23:52.718|—|**26:09.248**|**27:37.435**|

Primary recorded test-command durations sum to 27:19.381; their endpoint-based interval union is 23:52.718 because some processes overlap. The sum of endpoint lifetimes is 27:19.456; the 75 ms difference comes from projection rounding. Review adds only 2:16.530 of distinct test-containing occupancy to that union. A stricter primary subset of single-test-only shell invocations has a 10:34.736 interval union; even it includes compilation, Cargo locks and other process overhead, not pure assertion execution. Missing or mixed suite records prevent a complete compiler-versus-test CPU split.

### Measured long commands and repeated attempts

| Actual command/result | Process lifetime | Independent log evidence / assessment |
|---|---:|---|
|129 session regression|84.622 s|201 tests pass 75.29 s; compilation 7.60 s; later rustdoc fails finding engine crate during concurrent builds. Serial focused doctest 1.877 s passes.|
|130 full session|184.059 s|230 tests pass 175.07 s; compile 7.27 s. Mixed edit/fmt/test command 22:59:26–23:02:30, overlapping other useful work.|
|131 first full session|182.485 s|239 pass/2 fail in 179.50 s; two obsolete “Tori unsupported” assertions, not failed native rules fixtures.|
|131 full session rerun|188.106 s|242 pass in 185.11 s; compile 2.42 s. Shared lifecycle/capacity regressions include long existing Chess/Xiangqi tests.|
|130 changed long fixture/capacity runs|45.600 s,45.759 s,49.335 s|One boundary test initially padded after JSON's closing brace; corrected to pad within object. Targeted correction 5.873 s command/0.15 s suite avoids immediate repetition of all long scenarios.|
|131 long/final bounded journeys|34.948 s and 37.179 s captured commands|Logs report 31.50 s and 37.24 s suites; projection/timer endpoints differ slightly. Mandatory representative 180-action successful journey is separately established. Neither whole journey is promised to finish within a 15-second per-operation budget.|
|130 default Node→bundled Node|0.432 s failed;2.692 s corrected chain|System runtime missing`libsimdjson.33.dylib`; failure before meaningful frontend tests. Small process cost plus unmeasured diagnosis.|

Most repetition is justified incremental verification: new harness assertions, real structural rejection, local defense/reversal cases, native history/record/capacity behavior and controller/process races. They are not identical ceremonial reruns. Distinguish those from avoidable discovery order:

- Prior-stage unsupported-Tori tests could have been found and updated with a focused search/test before a three-minute broad 131 run. The broad shared-boundary evidence still had value, so 182 seconds is not a demonstrated fully saved duration.
- Tagged native consumer exhaustiveness should be compiled early across the standalone host as well as the root workspace. 129's missing host match and 131's separate-manifest fmt miss triggered extra source campaigns.
- Concurrent Cargo/rustdoc work produced 129's `E0463` failure; targeted serial retry fixed it. Shared build-cache lock waits also occur. More parallel processes do not guarantee faster feedback.
- 131 has 27 captured fmt executable lines and 10 clippy lines; 130 has 14 fmt/3 clippy. Many occur in fast cached edit→check chains. Counts alone do not establish a major elapsed bottleneck. A coherent final check can reduce repetition without withholding useful compiler feedback.

The 182 s,188 s and 184 s suites are genuine multi-minute test execution; most captured local checks are far shorter. Across all six assignments, captured verification-process occupancy is 27:15.361 of 330:38.052 wall time. This is an observed process-occupancy comparison, **not a claim that everything else was coding, bureaucracy or model reasoning**. Test authoring, output interpretation and background CI waiting live outside that measurement.

Readback traps matter: a 0 ms`tail`can display a previous 37-second suite; it is not a 0 ms test run or another 37-second execution. 126 also contains an overall exit 0 shell with a 15 pass/1 failtest followed by successful reporting commands. Intentionally failing negative cases inside successful test suites are not counted as regressions.

## 6. Reviewer scheduling and blocking

| Assignment | Initial reviewer execution, Paris | Follow-up relevant verdict(s) | Primary blocking conclusion |
|---|---|---|---|
|126|14:51:35–15:03:12.398|LF correction 15:11:17–15:11:43.470|Initial review overlaps implementation; correction review precedes corrected dispatch. No measured primary-only review wait.|
|127|17:04:03–17:06:41.313|No further completed turn|Review complete before source/CI.|
|128|18:12:08–18:13:47.589|18:15:19–18:15:48.557|Review of added public-token checks complete before CI.|
|129|20:37:16–20:47:22.630|Host correction 20:50:00–20:50:15.374|Review and capacity tests overlap; last correction verdict precedes corrected dispatch.|
|130|22:46:35–23:10:19.943|23:30:18–23:30:48.859|Reviewer starts 16 seconds after task start; substantive admission findings corrected during build. Acceptance reconciliation happens during GUI tail.|
|131|08:41:30–09:39:30.517|Handoff 09:48:19.114; format 09:51:11.271; bounded-test 10:03:07.963; final evidence 10:26:57.006|Reviewer starts 42 seconds after task start. Initial 58 minutes overlap implementation. Final 41.637-second review is after CI, but primary is also preparing finalization.|
|85 acceptance|21:45:00–21:48:01.811 and 21:45:08–21:47:23.459, parallel|Final documents 21:53:55.958/21:53:56.690|Whole assignment 13:01.714; no new primary tests/CI; reviewers reproduce checks. Primary writes acceptance/audit/handoff concurrently.|
|77 planning|22:25:31–22:33:09.931|One turn|Whole assignment 9:34.004. Reviewer overlaps task decomposition/ownership mapping;1:37.861 remains after verdict. No sleeps or`wait_agent`.|

No event demonstrates a long span in which the primary had finished every other requirement and was waiting solely for a reviewer. **Exclusive reviewer-blocking time is unknown, not zero and not reviewer runtime.** Early reviewer turns sometimes wait for evolving work; their entire duration is not active code inspection either.

Repeated 131 reviews track changed evidence/source: initial runtime findings, handoff, formatting-only change, substantive bounded-test correction, and exact-source final acceptance. Some handoff/evidence reconciliation could be consolidated; eliminating the substantive review would remove safeguards that found actual defects.85's 13-minute acceptance comparison shows whole-boundary audits can be concise when existing CI/source evidence is reused.77's 9½-minute comparison shows meaningful planning can proceed alongside review rather than afterward.

## 7. CI critical path and monitoring

### Actual final checkpoint jobs and overlap

All source hashes, job/step timestamps and failures are in the linked original API bundle. These are the **last jobs at the primary's selected checkpoint**, not a sum of platform durations. Dates are 1 November except 131 on 2 November.

| Task | Dispatch and run | Last checkpoint job/platform | Job completion, Paris | First printed success available to primary | Completion→observation |
|---|---|---|---|---|---:|
|126|15:12:32;[41315037374](https://github.com/jweathers777/mShogi/actions/runs/41315037374)|124873158131/Windows native|15:25:26|15:26:20.695|54.695 s|
|127|17:09:42;[41324873187](https://github.com/jweathers777/mShogi/actions/runs/41324873187)|124902606999/Ubuntu native|17:23:47|17:24:32.022|45.022 s|
|128|18:21:26;[41331055051](https://github.com/jweathers777/mShogi/actions/runs/41331055051)|124921046921/Windows native|18:37:27|18:39:16.826|109.826 s|
|129|20:50:41;[41343787560](https://github.com/jweathers777/mShogi/actions/runs/41343787560)|124959167485/Ubuntu native|21:05:14|21:06:12.109|58.109 s|
|130|23:10:43;[41355796473](https://github.com/jweathers777/mShogi/actions/runs/41355796473)|124995418934/Windows desktop, chosen full-workflow checkpoint|23:32:15|23:33:27.971|72.971 s|
|131|10:03:29;[41417589127](https://github.com/jweathers777/mShogi/actions/runs/41417589127)|125185114760/Windows desktop|10:22:51|10:24:57.120|126.120 s|

Native checkpoints for 126–129 last roughly 13–16 minutes after dispatch. Their last jobs spend 11:47,13:24,14:58 and 13:49 in native-test/verification step buckets respectively. These include compilation and separate QA-constructor checks; explicit build steps are tiny and do not measure all compiler activity. CI's test/verification workload, rather than ordinary queue delay, is the long pole. 130 macOS native's 304-second start delay is an exception; native still ends 23:28:52 before chosen desktop completion. 131 native ends 10:20:01 before Windows desktop's final package/GUI completion.

Useful work continues during CI: 126 preserves failed-run provenance and policy/handoff results; 127/128 reconcile fixture/perft/resource coverage; 129 records the initial host failure/corrected source and prefix handoff; 130 extracts test counts and evidence; 131 captures failed logs, source provenance, native test counts and package metadata. Specifically, after 131's 10:08:36 remaining-CI statement, log capture at 10:08:41, provenance edits 10:09:37, package evidence 10:14:20, native counts 10:15:28 and evidence-script authoring 10:21:17 are visible.

Therefore **necessary exclusive CI-blocking duration cannot be fully measured**. The event ordering establishes a final technical dependency, while active preparation remains interleaved. It would be wrong to call every post-dispatch minute idle. Even the remaining-gate statements mark a dependency, not the instant all evidence/finalization work was complete.

### Measured primary pauses and observation lag

| Task | Explicit sleeps | Measured sleep elapsed | Sleep portion after actual checkpoint |
|---|---:|---:|---:|
|126|4|3:00.010|0|
|127|8|6:30.018|0:36.727|
|128|5|4:10.012|0:50.002|
|129|9|7:25.026|0:47.403|
|130|9|6:45.017|0|
|131|8|7:20.023|0:55.002|
|Total|43|**35:10.106**|**3:09.134**|

Sleep elapsed here is the recorded sleep-item start→end interval in the Codex history projection. Raw tool call→return and the tool’s returned wall-time scalar differ by milliseconds; neither is substituted for these item intervals. Sleep proves a primary pause, not that external work stopped or that the pause was unnecessary. Post-checkpoint portions identify pauses whose selected external condition had finished. 127 and 129 straddle completion; 128 and 131 include whole 50/55-second calls beginning afterward. The source bundle retains every interval; the table uses those recorded item intervals rather than the slightly longer raw call round trips.

The completion→printed-success lags sum to **7:46.743**, but include useful work and 130's failed network poll/retry. They are not 7:47 of proven waste. Primary 131's watcher had observed all-success at 10:23:25.337738,34.338 seconds after the job; it was launched asynchronously at 10:06:08.576,finished 10:23:25.342 and was read 10:24:57.120. The primary also extracted native logs 10:23:03–10:23:08, so the entire 2:06 lag was not idle. Its final 55-second timer was unnecessary for discovering a result already available from the watcher.

Literal`gh run`command lifetimes sum to 135.041 seconds across selected assignments, including status/evidence/diagnostic requests; this excludes Python subprocess calls inside watchers and is not total monitoring effort. The 17:16.766 watcher lifetime overlaps primary work and sleeps. Neither number should be presented as pure developer waiting. A single completion-aware watcher would reduce duplicate status handling and stale updates; actual API call time alone is not the principal bottleneck.

### Failure campaigns


- MSH-126: initial native[41314482706](https://github.com/jweathers777/mShogi/actions/runs/41314482706) fails Windows hash admission and an unrelated macOS runner test; corrected-source native succeeds. Old and new desktop workflows continue, consuming resources concurrently. The 6:22 source-correction envelope does not equal the total elapsed cost of either workflow.
- MSH-129: desktop[41343664148](https://github.com/jweathers777/mShogi/actions/runs/41343664148) fails missing Tauri exhaustive handling; corrected-source workflows launch while first native workflow still runs. The 1:25 host correction avoids claiming the native kernel itself failed conformance.
- MSH-131: desktop[41415490841](https://github.com/jweathers777/mShogi/actions/runs/41415490841) fails standalone formatting; native[41415490915](https://github.com/jweathers777/mShogi/actions/runs/41415490915) fails an unjustified unconditional timing expectation. Intermediate[41415982022](https://github.com/jweathers777/mShogi/actions/runs/41415982022)/[41415982048](https://github.com/jweathers777/mShogi/actions/runs/41415982048) are superseded/cancelled after already consuming resources. Final native[41417589059](https://github.com/jweathers777/mShogi/actions/runs/41417589059) and desktop 41417589127 pass.

Parallel platforms and overlapping superseded campaigns are never summed as sequential primary waiting. No same-run reruns occurred; corrective revisions created new run IDs. Existing-game GUI checks are not Tori GUI verification.

## 8. Ranked ways to shorten one assignment

| Priority | Recommendation | Observed impact and confidence |
|---|---|---|
|1|**Preflight every changed real consumer, including separate workspaces, and old capability assertions before broad checks.** Maintain a small list of required consumer manifests/targets and run cheap relevant checks early.|Demonstrated misses:129 Tauri match,131 standalone fmt,131 obsolete unsupported-Tori tests only found after 182 s. Potentially avoids correction campaigns and expensive discovery. Exact minutes saved unknown because campaigns/other fixes overlap.|
|2|**Make bounded resource tests reflect the contract before CI.** Separate representative mandatory successful journeys from near-limit success-or-precise-refusal cases; preserve full-state checks and limits.|131 required a 13:01 format-correction→final-test-source interval and new CI after the invalid timing promise. Review-supported correction preserved rules/deadlines. Do not “optimize” by raising limits or weakening expected rule outcomes.|
|3|**Choose and record the required CI checkpoint explicitly; overlap evidence/review with it.** Use focused native/shared checks first and full application campaigns where relevant; retain necessary platform coverage.|13–22-minute CI phases recur. 130 was accepted while Windows GUI was still pending; last 3:23 overlapped finalization. Whether a whole workflow is required is an acceptance decision, not an automatic assumption. No measured claim all GUI coverage is excessive.|
|4|**Prepare one acceptance record from structured results; finalize it once.** Populate commands, outcomes, source and pending ownership while checks run; generate task summary/manifest from it.|26:43 post-checkpoint envelopes across six runs contain evidence/admin/review/reporting. Only a fraction is removable; 131 has real final review. Comparison 85 finishes whole-boundary acceptance in 13 minutes by reusing existing source/CI.|
|5|**Replace independent timer polls plus a watcher with one completion-aware monitor.** Resume on changed/final state; recheck before sleeping; cancel superseded campaigns promptly.|Measured 3:09 of primary pauses after selected CI checkpoints, whole 50/55-second stale sleeps, up to 2:06 visibility lag. Demonstrated opportunity is minutes, not the entire 35-minute sleep total.|
|6|**Use targeted incremental tests, then one broad relevant regression checkpoint after coherent edits.** Update expected prior-stage contracts before the broad suite; use targeted reruns for local failures.|129 doctest correction reruns in 1.877 s; 130 boundary correction targeted 5.873 s; 131 full session 182 s failed/188 s passed. Avoid repeating slow unchanged witnesses casually; shared-boundary changes still justify broad regression evidence.|
|7|**Schedule reviewers early with bounded questions and re-review changed semantics, not unchanged evidence.** Hand them evolving ownership/fixture questions, then a concrete final diff.|130/131 review launches 16/42 seconds after start; 58-minute 131 review already overlaps development. No demonstrated long reviewer-only idle phase. Preserve this overlap; consolidate optional handoff/evidence-only rounds when source is unchanged.|
|8|**Use the known runtime and avoid competing Cargo/rustdoc builds in one target directory.** Reuse bundled Node and serialize/partition builds where contention actually appears.|130 Node process failure 0.432 s plus unmeasured diagnosis; 129 rustdoc `E0463` and lock waits. Concrete but smaller measured process costs; no basis for making generic compilation optimization the top priority.|
|9|**Reduce initial inspection to current task/dependencies, affected owners and changed contracts.** Reuse stable architectural maps and source manifests; still verify Git/ownership and current requirements.|Pre-first-write envelopes 4–7 minutes include first-source composition, so a pure inspection saving cannot be claimed. Narrowing repeated full-file reads is plausible, lower-confidence opportunity.|

These priorities address **one run**. They do not depend on faster owner responses, eliminating between-assignment gaps, or batching multiple owner assignments. No workflow change is implemented by this report.

## 9. Instrument future runs without creating more bookkeeping

Record structured events automatically at existing boundaries: task-start/complete; source-write/checkpoint; subprocess launch/exit and output status; reviewer request/first actionable finding/verdict; CI dispatch/job completion and primary observation; evidence finalization; final response. Retain parent IDs and revision hashes so parallel intervals can be merged.

A useful additional primary event is **“blocked on X; other required work ready”**, followed by **“resumed from X”** with the matching command/run/review ID. Today a statement that CI remains a gate does not prove all evidence work is done. Instrumenting that distinction would allow actual exclusive-blocking accounting without guessing from silence.

Capture compile and suite timers separately when tooling provides them, plus Cargo lock waits and async process IDs. Make shell wrappers preserve failing subcommand status while retaining negative-test semantics. Record `mixed` scopes explicitly. Keep one machine-readable result manifest and generate concise prose from it; do not add a manually maintained narrative timesheet. Model reasoning/source-composition CPU time remains unknown unless the runtime exposes a reliable timer; action gaps must stay unclassified.

## 10. Provenance and independent review

Historical code requirements/evidence: [conformance policy](../testing/conformance-policy.md), [harness evidence](../m4/tori/harness-evidence.md), [structural evidence](../m4/tori/native-structural-evidence.md), [local legality evidence](../m4/tori/native-local-evidence.md), [history evidence](../m4/tori/native-game-evidence.md), [document evidence](../m4/tori/desktop-document-evidence.md), [runtime evidence](../m4/tori/desktop-runtime-evidence.md), [native acceptance](../m4/tori/native-acceptance.md), [shared planning review](../m4/tori/shared-plan-review.md), all interpreted at the historical task revisions, not later 132 edits.

Original primary turn IDs:126`01a19756-57e1-7481-80f9-06a37b3f0cb3`; 127`01a197d1-5eee-7b80-ab01-7907b3d9621e`; 128`01a1980a-c54d-74a0-b928-fe5e9d67211f`; 129`01a19892-e965-7711-bf93-8c054d97b780`; 130`01a19913-e354-7353-a85c-6f4ccdf3f51e`; 131`01a19b34-26a1-7ff0-93de-0b2e1e731770`; 85 acceptance`01a198db-16fc-7642-b5c5-61a14cbc6ad9`; 77 planning`01a19900-91a7-7d93-81d3-60e7f3f495a8`.

Representative process IDs:129 session`exec-92c750bd-8a75-44a2-bf49-8a94078a4409`; 130 full session`exec-38fd2b31-e00c-4564-a14a-6de829985c53`; 131 failedfull session`exec-3a0a9d2d-4f72-4a01-8d3b-3ee7b1cacae0`; 131 passedfull session`exec-b80fbb81-5fd6-4746-813e-7fd929fb9f57`; 131 watcher`exec-27bef30a-bced-42af-8bac-4a83f3f4fc88`. Their timestamps/executable snippets/results are retained in the follow-up bundle. Exact CI job/step records remain in the original API bundle.

Independent read-only review of this follow-up: **ACCEPT; no remaining findings**, 2 November 2026, `/root/report_review`, thread `01a19bbd-c367-7a91-838e-2f390ba4e377`. The reviewer independently recomputed all six task runtimes and additive phases, local process unions, reviewer overlap, 43sleep-item intervals, post-checkpoint pauses, observation lags, and the 109:05.155 task-complete versus 107:54.314 integration-cutoff distinction. Resolutions clarified sleep-item versus tool-return timing, the 75 ms recorded-duration/endpoint difference, and list rendering. The reviewer verified the corrections and accepted the final report. No historical test campaign or rule re-ratification was performed. No production changes, test campaigns or task-status changes are part of this deliverable.
