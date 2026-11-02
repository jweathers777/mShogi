# Tori Shogi development: forensic time and workflow analysis

Analysis date: 2 November 2026. Historical endpoint: **`c89802e3e6961eaf8448e59794998e61a277da91`**, committed **2 November 10:28:43 Europe/Paris / 09:28:43 UTC**. This report covers development through MSH-131, not subsequent MSH-132 work. All displayed calendar times use **Europe/Paris**; durations use `hours:minutes:seconds` unless days are explicit. Paris changed from UTC+02 to UTC+01 on 25 October; conversion uses the timezone database.

## Executive summary

The elapsed timeline is much longer than the observed execution timeline. First focused later-game source research began **26 October 08:24:56**: **7 days 2:03:47** before the endpoint. Earlier explicit Tori scope planning began **25 October 16:34:00**, **7 days 17:54:43** before it. The practical modern-profile owner direction arrived **1 November 12:47:59**: **21:40:44** before the endpoint. Formal profile ratification followed at 13:08:04, leaving **21:20:39** to MSH-131 integration.

Accessible local Codex records allow more than a commit-gap estimate. Eleven dedicated delivery assignments from 31 October through MSH-131 contain **6:53:46 of observed primary execution windows**, clipped at the requested endpoint. Adding the earlier Tori-only research worker gives **6:57:34 of directly attributable observed windows**. These windows include reasoning, authoring, tools, tests, review coordination and waiting; they are **not measured productive labor or CPU time**. Earlier mixed-game planning/research also contributed, but its Tori share cannot be isolated. Nested agents are not added to the primary windows.

From modern direction onward, observed primary execution is **6:27:19**, while **15:13:24** lies between assignments. Those gaps are real calendar intervals, not evidence of continuous coding, idle workers, or needless delay. The largest is **9:06:04 overnight** after MSH-130. Earlier days also contain substantial non-Tori development. The record supports reducing repeated assignment gates; it does not support claiming 15 hours of recoverable effort.

CI consumed **12:03:39 of summed job runner time** across **20 workflows / 60 jobs**, but only **2:32:42 of overlapping job occupancy**, or **2:32:57 including dispatch-to-first-job gaps**. The applicable native gates for MSH-126–129 plus native/desktop campaigns for MSH-130–131 occupy **2:03:20**. That is an upper envelope for CI-related blocking, not measured exclusive waiting. Documentation and review continued during CI. MSH-126–129 integrated their completion records before their desktop campaigns finished.

The most consequential avoidable mechanisms were serial owner reassignment, repeating broad workflows after narrow native changes, maintaining overlapping evidence summaries, and initially treating historical authenticity/external comparators as potential delivery gates. Verification itself was valuable: independent review found real fixture, admission, preservation and lifecycle gaps. The recommendation is **one approved dependency-ordered assignment per coherent native or application batch**, with internal task checkpoints, independent review at meaningful boundaries, proportional CI, durable evidence once, and one final owner report.

## 1. Method, sources and limits

### Available evidence

| Source | Actually available | What it measures / does not measure |
|---|---|---|
| Codex execution history | Read-only `~/.codex/thread_history_1.sqlite`: `thread_turns`, `thread_items`; `state_5.sqlite`: thread identity, source, rollout paths | Turn start/completion and recorded duration; command start/completion, outputs/statuses; messages, review handoffs. A turn includes waits. Thread `created_at`/`updated_at` alone are not activity durations. |
| Codex JSONL | Main rollout `~/.codex/sessions/2026/10/24/rollout-2026-10-24T22-42-48-01a16fa6-da21-7b40-8b4b-ac86c437071f.jsonl`, plus relevant researcher/reviewer rollouts | UTC `task_started`/`task_complete` events corroborate projected turns. Exact event timestamps used for primary window totals. No inference from a session's entire lifetime. |
| Agent records | Named research/review threads and their individual turns | Observed reviewer/researcher elapsed windows, often overlapping the builder and each other. Not separate serial labor. Guardian approval threads are excluded from independent rules-review counts. |
| Tool/command records | 905 primary command items through the endpoint (909 across the full original turns) | Tracked process wall intervals; mixed scripts often write, test and document in one invocation. Background work, short/zero-duration projections and polling complicate complete attribution. No comprehensive CPU or model-inference timer. |
| Git and reflog | Local history and primary reflog; author/committer times; original and corrected revisions | Milestones, checkout/integration corroboration. First commits are not implementation starts. MSH-130's completion commit was locally amended: final author time 23:34:13, committer/reflog time **23:34:26**; the latter is used for integration. |
| Backlog | Immutable task files at the endpoint; CLI read-only queries | Scope, dependencies, ownership, plans, historical notes, criteria and completion evidence. `created_date` is ticket creation, not activation; minute-resolution `updated_date` is not a precise work clock. |
| GitHub Actions | Live REST run/job/step timestamps, conclusions and selected failure logs | Real runner/step elapsed durations and parallel occupancy. Compilation inside tests/composite steps cannot be independently separated. |
| Existing Tori evidence | `docs/m4/tori/`, committed CI JSON, local `/private/tmp/msh126…msh131…` logs | Outcomes, fixtures, measured replay cases, intermediate failures. Local log mtimes are not used as historical clocks. |

The API was accessed through the connected GitHub plugin, including its generic fetch capability where the job wrapper omitted timestamps. Run listing covered the relevant dates; job queries used `per_page=100&filter=all`. Every selected run has attempt 1. Existing committed CI evidence was cross-checked against current API data. No new native, frontend, GUI or external-engine campaign was run for this investigation.

The source bundle [tori-time-analysis.json](evidence/tori-time-analysis.json) retains sanitized primary turn/command intervals, reviewer IDs/windows, exact integrated commit hashes, and selected original CI run/job/step fields with UTC timestamps. It omits private conversation bodies, credentials and command outputs. CI data is independently retrievable via `/repos/jweathers777/mShogi/actions/runs/{id}` and `/actions/runs/{id}/jobs?per_page=100&filter=all`. Private session records are local provenance, not a future project prerequisite.

Inspection found MSH-132 dirty work and an active owner in the primary checkout. It was excluded and preserved. The report reads task history at the endpoint rather than treating today's task state as the historical state. Installed global Backlog is 1.55.0 versus recorded 1.55.1; this investigation performs no Backlog mutations, so no incompatible writes were attempted. Prior execution records show use of the pinned 1.55.1 executable.

### Definitions and arithmetic

- **A — measured active execution:** no reliable complete productive-work total exists. The narrower measured quantities are recorded command process intervals and agent turn windows. Neither establishes human effort or CPU use.
- **B — elapsed wall clock:** timestamp subtraction, with endpoints named. Primary turn windows include model work, tools and awaits. Exact JSONL events are used; millisecond `duration_ms` differs slightly because event projection endpoints are not identical.
- **C — concurrent activity:** merge overlapping intervals before summing. Research, review, local commands and CI frequently overlap the builder. Their columns cannot be added together.
- **D — unobserved/unclassified:** between-assignment intervals and activity that cannot be allocated to a semantic category. Unclassified is not idle.
- **E — bounded estimates:** stated attribution/occupancy envelopes only. No percentage allocation of coding/review/documentation and no invented hours-saved figure.

For intervals `[start,end)`, sort by start, merge intersecting intervals, then sum `end-start`. CI runner time instead sums each job's duration deliberately. Headline values round the final total to the nearest second; rounded table rows may differ by a second from their sum. No file modification time is used. No commit gap is labelled active work. No package result is promoted to a GUI result on another platform.

## 2. Chronology and task timing

### Earliest planning and research

| Event | Paris timestamp | Evidence and interpretation |
|---|---|---|
| Six-game baseline imported | 23 Oct 00:45:38.722 | Bootstrap user event, thread `01a165ca-91e6-78e1-95f8-357af61ca032`; `d876a6190f5296f8a20b3e4ea529ec84d4211eef`. The design already promises Tori; this is not focused research. Its source package's earlier preparation is undated here. |
| Explicit Tori delivery scope | 25 Oct 16:34:00.033 | Mixed M 4–M 6 roadmap assignment, thread `01a173b2-7756-7bf3-bbc8-cbf66436088b`; task-creation script 16:39:23.237; `9b48e9ae5c47554fd5b3fa1ff2762040bf33bf86`. |
| First focused source inventory | 26 Oct 08:24:56 | `/root/later_game_sources`, thread `01a17719-1aef-7fb2-8584-546281ed4e0b`; Tori/Chu/Tenjiku/Xiangqi together; `16fce28d89565728b0432848fc1e4fae6f8972d6`. Two turns total 7:58.425, including a 5:02.122 turn ending in model-capacity/server-overload failure. Not a failed test campaign. |
| First Tori-only research | 27 Oct 15:35:46–15:39:34 | `/root/msh81_research`, thread `01a17dc9-e660-7d60-a437-e13c71dc37c2`; **3:47.857** inside the concurrent primary MSH-80 assignment; `f4e564b66ccecca6d76cadea5a354bb1e366a77d`, [research ledger](../m4/research/tori.md). |
| Historical acquisition checkpoint | 31 Oct 22:28:44–22:55:11 | First dedicated MSH-81 owner assignment below. Partial evidence integrated; task remained To Do. |
| Modern owner direction | 1 Nov 12:47:59.349 | User event in turn `01a196f0-162e-7352-a119-80f39e887e84`; practical playable profile replaces historical-acquisition completion gate. |
| Formal selected profile | 1 Nov 13:08:04 | `ec1ce01680800327eb155d7b790650eb294d4bd6`; MSH-81 completion record `6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc` at 13:08:53. |

The broad baseline-to-endpoint span is **10 days 10:43:04**; explicit Tori planning **7 days 17:54:43**; focused source inventory **7 days 2:03:47**; first Tori-only researcher **5 days 18:52:57**. These answer different questions and must not be collapsed into “development effort.” Earlier mixed work spans cannot be allocated wholly to Tori.

### Task-level delivery table

“First observed” below means the dedicated delivery assignment, **not the earliest broader planning above**. Completion means the integrated completion/checkpoint commit, not the later final chat message. “Observed turn” includes push/report work after earlier commits, but the last row stops at the requested endpoint. Review is union of named reviewer-turn intervals, includes awaits/tests and overlaps the builder. The parent MSH-77 is still open at the endpoint; its row closes only its planning assignment.

| Task / phase | First observed work, Paris | Integrated completion/checkpoint, Paris | Wall span to checkpoint | Observed turn | Measured active work | Applicable CI occupancy ceiling | Review window union | Unaccounted / rework |
|---|---|---|---:|---:|---|---:|---:|---|
| MSH-81 historical | 31 Oct 22:28:44 | 31 Oct 22:54:05 | 0:25:21 | 0:26:27 | Unknown | None | 0:08:27 | Four fixture/provenance findings corrected; no final profile. |
| MSH-81 modern | 1 Nov 12:47:59 | 1 Nov 13:08:53 | 0:20:54 | 0:21:18 | Unknown | None | 0:10:07 | Root-terminal/counter clarification; Eagle fixture correction. Prior overnight gap 13:52:48 unobserved. |
| MSH-85 native planning | 1 Nov 13:14:53 | 1 Nov 13:28:34 | 0:13:41 | 0:13:58 | Unknown | None | 0:03:20 | Four child slices and fixture ownership; two task-creation script failures. |
| MSH-126 policy/harness | 1 Nov 14:39:40 | 1 Nov 15:30:14 | 0:50:34 | 0:50:51 | Unknown | 0:19:16 native | 0:12:03 | Harness/property gaps; Windows LF correction; unrelated macOS runner failure. |
| MSH-127 transitions | 1 Nov 16:54:03 | 1 Nov 17:26:21 | 0:32:18 | 0:32:55 | Unknown | 0:14:05 native | 0:02:38 | Adapter diagnostic/test-access/case-partition corrections; stronger rejection assertions. |
| MSH-128 local legality | 1 Nov 17:56:45 | 1 Nov 18:41:36 | 0:44:51 | 0:45:24 | Unknown | 0:16:01 native | 0:02:08 | Public reversal-token/inspection coverage added; no recorded failing primary verification command. |
| MSH-129 history/records | 1 Nov 20:25:27 | 1 Nov 21:08:49 | 0:43:22 | 0:43:58 | Unknown | 0:15:56 native | 0:10:21 | Native exhaustive consumers, doctest rerun, standalone Tauri match correction. |
| MSH-85 final native acceptance | 1 Nov 21:44:17 | 1 Nov 21:57:07 | 0:12:50 | 0:13:02 | Unknown | Reused CI | 0:03:38 | Two simultaneous reviewers; wording/link-audit correction; no new production campaign. |
| MSH-77 shared planning | 1 Nov 22:25:13 | 1 Nov 22:34:30 | 0:09:17 | 0:09:34 | Unknown | None | 0:07:38 | Three codec/runtime/presentation slices, 83 consumer obligations mapped; parent remains open. |
| MSH-130 document/admission | 1 Nov 22:46:19 | 1 Nov 23:34:26 | 0:48:07 | 0:48:25 | Unknown | 0:21:32 native+desktop | 0:24:14 | Ending reservations corrected; compile/resource-test/Node environment failures; long-record measurements. |
| MSH-131 runtime/lifecycle | 2 Nov 08:40:48 | 2 Nov 10:28:43 | 1:47:55 | 1:47:54 | Unknown | 0:36:30 native+desktop | 1:06:58 | Save-under-lock, replacement preservation, identity/floor capacity, child-test race, host formatting and deadline-test corrections. |

Rounded first-observed timestamps explain the last row's one-second discrepancy. MSH-81's earlier 3:47.857 Tori-only research belongs in its total but is outside these delivery rows. MSH-85's two dedicated turns total **26:59.999**, excluding the child tasks rather than double-counting them. MSH-77 planning is **9:34.004**, not completion of shared integration.

The eleven primary turns occupy **6:53:45.900**. The known Tori-specific union with the Oct 27 researcher is **6:57:33.757**. All selected late researcher/reviewer turns fit within their builder windows. Including the *entire* three earlier mixed owner assignments produces an **8:32:17 attribution envelope**, not a Tori effort estimate; it includes unrelated roadmap, Shogi and Xiangqi activity. Their Tori fraction remains unknown.

### Implementation, tests, activation and integration

Code-edit observations establish implementation before the first source commits. They do not establish uninterrupted coding between these markers. Test entries are observed checkpoints/attempt periods, not full duration allocations.

| Task | Recorded activation/plan command, Paris | First observed product/harness write | Tests and corrections before source commit | First source commit / subsequent corrections |
|---|---|---|---|---|
| 81 historical | 31 Oct 22:29:29.997 | Research fetch 22:31:25; documentation/seed writes by 22:39 | Parallel historical and modern acquisition; profile review 22:33–22:43; integrity/link/graph checks before integration | `15e6bd66c58869031ba3248df9dfe24e7bf8f355` 22:51:52; completion `26a2fa9ce8165b6e7ae713d12f16548b65c110da` |
| 81 modern | 1 Nov 12:51:36.887 | Fixture-generation script 12:57:40.606; no product code | Audits 12:50–12:53; independent profile/fixture review 12:55–13:06; 103 fixture specs, 39 original IDs retained | `ec1ce01680800327eb155d7b790650eb294d4bd6` 13:08:04; completion `6893f1249dff0414cdbe81ce99fac3a3c0c0e7dc` |
| 85 planning | 13:17:32.642 | No product code | Two failed creation-script invocations 13:19–13:21, about 61 s summed process time; reviewed ownership/DAG | `13210d13638666e41be202c5f949a32f37c4d196` 13:28:02; completion `04225043545a7219fbbad6024eb2398c5f68aaf2` |
| 126 | Plan-containing command 14:43:08.948 | Harness crate 14:46:38.193 | First compile/test failures 14:51; 18 harness tests passed by 15:04; review strengthened rotation/effect/reversal checks; CI LF correction 15:12 | `134f81ba67ef81918734ed1ed1acb5de7532fb99` 15:06:06; `d5f98a2af9e38ae690e2fd5c4940a4d26d12407b` 15:12:28 |
| 127 | 16:55:30.174 | `tori/mod.rs` 16:58:14.667 | Adapter failures 17:00–17:03; 202 assertions by 17:06; review required actual rejection, not conditional nonmutation; core/conformance regressions before 17:10 | `ce1801d7801f12e447b625739acd6a0c53aa2640` 17:09:36 |
| 128 | 18:00:46.243 | `legal.rs` 18:03:38.847 | 483 cumulative assertions by 18:08; public consumer/reversal checks added after review; 262 core/conformance checks reported at 18:21 | `5c60ac65dc03a7f9afcaa796a2babb252329d361` 18:21:20 |
| 129 | 20:28:57.276 | Native type generator 20:30:55.359 | Compile/adapter corrections 20:33–20:40; 713 assertions by 20:38; session doctest serial rerun; core/engine/child regressions by 20:49 | `e1cc89d454db5e0ce98da34bf4cda7960d395413` 20:49:13; host correction `994521bec0637d6f8f17103f1106c0025d950753` 20:50:38 |
| 85 acceptance | 21:45:51.477 | No product change | Native assertions reproduced; independent kernel/pending-obligation reviews; existing CI reused; evidence-link correction | `2d4737603bd851534ee24c4b21cd70f4dc3229d2` 21:55:05; completion `a3677e1e862fdf3877c784f12f56a99c4bfbf167` |
| 77 planning | 22:26:37.451 | No product code | Codec-first order/consumer mapping reviewed concurrently; graph/ownership checks | `8977a4b9c749e46146255861d1ead10f744e5b97` 22:34:05; checkpoint `1e51ea51b3ba6997b75a7c11a25df0c7f4eeaad5` |
| 130 | 22:48:23.361 | `tori_document.rs` 22:50:01.556 | First 10 checks by 22:55; ending reservations/replay/resource corrections; native 713 plus frontend focused 36; 360-move/save measurements and 17 codec checks | `8fd12f953beeacac46fdd417b2054bedeafe2d6e` 23:10:39; completion `086579f7361c5e1e395ea8f766779688446441b2` |
| 131 | 2 Nov 08:43:09.035 | Runtime plumbing script 08:45:00.484 | Compile/controller/child corrections 08:47–09:06; broad session 239/241 then 242/242; review/capacity/failed-replacement corrections through 09:45; CI/test corrections through 10:03 | `253e4c949614d133b5295f6a362e3d5279adcef3` 09:46:06; `ce48e84e64b9631343416acec865505dbbab570d` 09:50:17; final tested `6a2edd71b3136a9c2043f9137eb6d3fb8abf120d` 10:03:18; completion `c89802e3e6961eaf8448e59794998e61a277da91` |

Final completion commits for 126–129 are respectively `62825dd77576ac2d3b8e18d1d3080a9c08112e82`, `4710a4d9c1c188293fc89870cef68b2d0232e6c7`, `e1f7ed407f360ef7b4267225c6921e73ddd1417d`, `558402b4426a1d8faff3931f65643bcda95dd2fe`. See the source bundle for all 26 hashes and author/committer times.

### Assignment gaps, concurrency and waiting

| Completed primary turn → next assignment | Elapsed gap | Classification |
|---|---:|---|
| 81 historical → modern | 13:52:48 | Overnight; unobserved, not active historical research. |
| 81 modern → 85 planning | 0:05:36 | Between separate owner assignments. |
| 85 planning → 126 | 1:10:49 | Between assignments; includes owner's policy redirection before 126. No measured continuous author work. |
| 126 → 127 | 1:23:31 | Between assignments; desktop CI finishes early in this interval. |
| 127 → 128 | 0:29:47 | Between assignments; desktop CI continues for about 4½ minutes after prior turn. |
| 128 → 129 | 1:43:18 | Between assignments; desktop CI finishes within the first minute. |
| 129 → 85 acceptance | 0:34:52 | Between assignments; corrected desktop CI finishes about 3¼ minutes after prior turn. |
| 85 acceptance → 77 planning | 0:27:54 | Between assignments. |
| 77 planning → 130 | 0:11:32 | Between assignments. |
| 130 → 131 | 9:06:04 | Overnight; no observed active Tori implementation. |

These use rounded SQLite endpoints and sum to **29:06:12** from the first dedicated historical assignment through the endpoint; exact event arithmetic differs below one second. The modern subset is **15:13:24**. Owner-instruction gaps are observed; the owner's activities and reasons for timing are not. Earlier Oct 27–31 periods include recorded Xiangqi/shared architecture/desktop work, not a four-day Tori research campaign.

Parallel activity is explicit: Oct 31 historical acquisition 8:23.516, modern acquisition 4:04.043 and source review overlap inside a 26:26.587 builder window. Nov 1 comparator audit 2:31.949 overlaps modern-profile audit and builder work. MSH-85 acceptance uses two simultaneous reviewers. MSH-131's reviewer starts roughly 42 seconds after activation and remains in a long turn while implementation evolves; its 58-minute initial turn is not 58 minutes of sequential post-implementation review. CI runs native/platform/package jobs concurrently, and evidence preparation proceeds during those jobs.

## 3. Where the observed time went

The following is deliberately not a fabricated pie chart. Several categories share the same command/turn. “Envelope” means elapsed execution window containing that activity, not a measured amount exclusively spent doing it.

| Requested category | Measured evidence / bounded quantity | Attribution limit |
|---|---|---|
| 1. Historical research/source acquisition | Oct 27 Tori worker 3:47.857; Oct 31 historical worker 8:23.516; entire dedicated historical assignment 26:26.587 | Worker is nested; entire assignment also produces modern source/fixture/review/bookkeeping work. Earlier mixed research share unknown. |
| 2. Modern specification/fixtures | Modern root 21:18.094; modern audit 1:46.814 and comparator audit 2:31.949 nested | Includes profile writing, 103 fixtures, review, corrections and integration; no exclusive specification timer. |
| 3. Backlog planning/decomposition | Dedicated 85 plan +77 plan **23:32.294** | Includes actual architectural design/ownership, source inspection and independent review. Not pure CLI overhead. Earlier mixed roadmap work unallocatable. |
| 4. Native implementation | 127–129 combined observed turns **2:02:16.892** | Includes tests, review, CI waiting and finalization; pure coding time unknown. |
| 5. Runtime/document/frontend integration | 130–131 observed turns **2:36:18.931** | Includes tests, review, CI and finalization. Full presentation and actual Tori engine/product delivery are outside endpoint. |
| 6. Tests/infrastructure authoring | 126 observed turn **50:51.388**, including conformance-policy correction | No exclusive authoring duration; shared reusable harness plus verification and CI. Later fixture adapters/tests are interleaved with implementation. |
| 7. Local execution | Primary tracked command interval union **48:55.731**, across all categories, including CI monitoring | Not all testing: one CI watcher alone spans 17:16.766; scripts mix edits/checks, and background/subagent test processes prevent a complete exclusive test total. Logs preserve actual local outcomes. |
| 8. CI execution/wait | Runner 12:03:39; job occupancy 2:32:42; applicable campaign ceiling 2:03:20 | Exclusive idle wait unavailable; do not add to primary turns. |
| 9. Independent review | 24 observed reviewer turns; **2:31:32 union**, **2:34:16.657 summed recorded reviewer duration** | Includes waits/tests; overlaps builder. Review handoff overhead alone unknown. |
| 10. Corrections/rework | Exact failures/findings below; 131 first source→final tested source 17:12; 126 source→LF correction 6:22; 129 source→host correction 1:25 | These are correction envelopes, often with CI in parallel; not measured correction effort. |
| 11. Documentation/acceptance/finalization | Separate 85 acceptance **13:01.714**; 26 dedicated commits include **12 completion/evidence-record commits** | Acceptance includes useful audits/tests; remaining evidence authoring is interleaved. No complete documentation-only timer. |
| 12. Waiting for instructions/between assignments | Modern 15:13:24; historical-onward 29:06:12 | Calendar gaps, not measured idle labor. Pure human-input latency cannot be isolated. |
| 13. Unclassified | Inside the eleven primary windows, **3:55:11.505** is outside the union of tracked primary command intervals and named reviewer windows | Arithmetic residual only: includes reasoning, authoring and tool activity absent from those two interval sets. Not idle, not all orchestration. |

The primary command-or-review union is **2:58:34.395**, already overlapping; subtracting it from 6:53:45.900 gives the category 13 residual. Review activity does not account for the builder's simultaneous work. No valid implementation-versus-orchestration percentage can be derived from these instruments. The largest measured calendar category is between-assignment gaps; largest CI step bucket is native testing; largest individual dedicated task window is 131. These are different measurement domains.

## 4. CI: timestamps, platforms, failures and critical path

The full run and platform tables appear in Appendix A. [Original API fields and UTC step timestamps](evidence/tori-time-analysis.json) permit recomputation; the run IDs link to GitHub. “Run elapsed” is `created_at` to last completed job, rather than a workflow metadata update. Job durations include setup and teardown, not billing-rounded minutes or CPU time.

| Task | Workflows | Runner time | All-CI dispatch/finish union | Native-only union | Acceptance relationship |
|---|---:|---:|---:|---:|---|
|126|4|2:21:38|0:28:12|0:19:16|Native CI gated completion; desktop final 15:34:22 after completion commit 15:30:14.|
|127|2|1:20:35|0:21:47|0:14:05|Native gate; desktop 17:31:29 after completion 17:26:21.|
|128|2|1:27:33|0:21:33|0:16:01|Native gate; desktop 18:42:59 after completion 18:41:36.|
|129|4|2:12:20|0:23:23|0:15:56|Corrected native gate; desktop 21:12:41 after completion 21:08:49.|
|130|2|1:28:51|0:21:32|0:18:09|Native passed 23:28:52; Windows desktop still running while finalization began, finished 23:32:15 before final evidence commit.|
|131|6|3:12:42|0:36:30|0:33:40|Final exact-source native+desktop required; native 10:20:01, desktop 10:22:51, integration 10:28:43.|
|Total|20|**12:03:39**|**2:32:57**|**1:57:07**|Occupancy, not exclusive developer wait.|

All six implementation assignments explicitly remained unfinished awaiting required native CI at some point. For 131, final commentary at 10:08:36 says CI is the remaining gate, with last job 10:22:51: a **14:15 completion-blocked elapsed interval**, during which monitoring/evidence/review still occurred. Similar explicit remaining-CI markers are 15:17:39→15:25:26 for 126 (7:47),17:17:28→17:23:47 for 127 (6:19),18:30:21→18:37:27 for 128 (7:06),20:53:32→21:05:14 for 129 (11:42). These five marker intervals total **47:09**; they measure a documented closure dependency, not exclusive idle time or recoverable savings. For 130 the last desktop job overlaps ongoing finalization, so its whole 21:32 is especially unsuitable as idle time.

Native 126–129 occupancy plus both-workflow 130–131 occupancy sums **2:03:20**, without adding parallel matrices or source reruns twice. This bounds time while applicable CI runs were actually in flight, under immediate observation of their completion. Polling delays after completion and correction work outside run intervals are not included; it is not a bound on every CI-related activity. Actual marginal delay against a counterfactual with immediate CI is **unknown**: review/docs/readiness are not independently timed prerequisites. Sum of all workflow elapsed durations, **5:10:38**, would be an incorrect developer-wait number. Sum of job durations,12:03:39, answers runner consumption only.

Queue overhead is generally small: first job starts 2–3 seconds after workflow creation. Per-job delay is generally 2–10 seconds, except 130 macOS native at**304 seconds**. API timestamps cannot split that delay into GitHub scheduling, environment provisioning and account concurrency reasons.

Across runners, explicit step buckets are native test/verification**5:25:25**, composite frontend/native checks**1:55:19**, existing-game GUI**1:53:58**, explicit builds**1:39:58**, format/lint**9:35**, setup/packaging/other**55:25**, macOS identity**0:08**. Step spans exclude some inter-step gaps, so buckets do not sum to job runtime. Cargo test/check compiles internally; “build duration” means explicitly named build steps, not total compiler time. Composite checks are not separable into frontend tests, native tests and compilation from step metadata alone.

### Failed attempts and successful corrections

| Task / source | Failure / intervention | Outcome and cost interpretation |
|---|---|---|
|126 `134f81b` native 41314482706|Windows 124871524762: fixture byte hash mismatch,13 harness failures after CRLF checkout. macOS 124871525333: unchanged runner test expected `ResourceLimit`, got `LateReply`.|`.gitattributes` LF fix at`d5f98a2` preserved audited bytes. Native 41315037374 passed. No claim the LF fix repaired the unrelated macOS race. Both old/new desktop runs continued successfully, duplicating package/GUI work.|
|129 `e1cc89d` desktop 41343664148|All 3 standalone Tauri checks failed an unhandled Tori save/document match arm.|`994521b` host refusal fix; native 41343787560 and desktop 41343787588 passed. Corrected campaigns overlapped the first native campaign.|
|131 `253e4c9` desktop 41415490841|All 3 fail formatting in the separate Tauri workspace.|Formatting-only`ce48e84` launched fresh runs. Main Cargo workspace fmt had missed the separate host manifest.|
|131 `253e4c9` native 41415490915|All 3 fail the 360-action test's unconditional success assumption under the unchanged 15-second deadline.|Intermediate`ce48e84` still has a Linux failure before cancellation. Final`6a2edd7` requires 180-action full success;360 may only return exact typed deadline refusal with complete-state preservation. Limits/profile unchanged; final runs passed.|
|131 superseded 41415982022/41415982048|Cancelled after final source became available.|Avoided some remaining duplicate work; already-consumed runner time remains counted.|

There are **4 failed workflow runs**, **2 cancelled**, **14 successful**; **12 failed jobs**, **3 cancelled**, **45 successful**. Failures are not 12 different product defects. Every run has `run_attempt=1`: **zero same-run reruns**, but **three source-correction cycles** (126,129,131), with 131 using two successive corrective source revisions. Those four corrective revisions generated eight additional workflow runs.

Local primary execution additionally records **at least 29 nonzero compile/check/lint/test invocations**: MSH-126: 2; MSH-127: 3; MSH-129: 7; MSH-130: 3; MSH-131: 14, including 130's unusable system Node invocation. This is an invocation lower bound, not 29 failed test campaigns or 29 bugs: several are compilation iterations, redirected/background outcomes can be missed, and intentional negative tests are not counted as failed campaigns. A separate acceptance-link verifier failed before its own output existed; two planning task-create attempts and one unsupported multi-task notes command also failed. No complete count of every subagent/background failure or every successful local rerun is established.

Useful verification caught mistakes that narrower formatting or compilation would miss: strict fixture bytes, explicit structural rejection, Swallow-defense legality, repetition/history separation, complete saveability reservations, stale identity retirement, failed replacement preserving running work, and the distinction between resource refusal and rule illegality. Broad repeated existing-game GUI runs after harness/native-only changes have a weaker marginal justification than these targeted checks.

## 5. Agent orchestration and avoidable work

### Assignments and review counts

There were **11 dedicated owner assignments** in the delivery sequence: historical 81, modern 81,85 planning,126,127,128,129,85 acceptance,77 planning,130,131. **Two were strictly planning-only**,85 and 77; source/profile work is substantive specification/research, not counted as mere planning. Three earlier mixed Tori-containing owner assignments (Oct 25 roadmap, Oct 26 MSH-62, Oct 27 MSH-80 plus research) make **14**, or 15 including the broad bootstrap baseline. Two of those three earlier mixed assignments were planning-only, giving **four planning-only assignments across the 14**, explicitly not four Tori-only planning assignments. Task IDs, owner assignments, subagent turns and chat sessions are different units.

All eleven dedicated assignments ran in **the same primary Codex thread**,`01a16fa6-da21-7b40-8b4b-ac86c437071f`, rather than eleven fresh chats. The record supports repeated task startup/source inspection and compactions, not eleven cold repository checkouts or eleven complete cache rebuilds. Each assignment rechecks ownership, Git/dependencies/requirements and records a plan. Activation-to-plan is a preparation envelope containing useful scope/design work, not an independently measured startup tax. Short direct Backlog process times do not measure time composing notes or reconstructing context.

Independent review in the dedicated sequence comprises **12 reviewer engagements**, organized into **11 acceptance/review gates** (historical 81,modern 81,85 plan,126,127,128,129,85 acceptance,77 plan,130,131).85 acceptance uses two simultaneous reviewers at one gate. There are **24 observed review turns**, including corrections/follow-ups, not 24 fresh independent acceptance rounds. Early roadmap review and two MSH-62 planning reviewers are additional mixed-scope reviews, excluded from the 12/24 Tori-focused count. Research/audit workers and guardian approval decisions are excluded.

| Review engagement | Observed reviewer turns | Consequential result |
|---|---:|---|
|81 historical/profile|2|Four source/seed issues corrected; partial evidence only.|
|81 modern/profile|2|Imported terminal handling/declared counters clarified; final Eagle fixture rechecked.|
|85 native planning|1|Four-slice dependency/ownership review.|
|126 conformance|2|Rotation side exchange, structural effects and exact reversal coverage added; LF correction reviewed.|
|127 structural|1|Negative tests must actually reject, not conditionally check nonmutation.|
|128 local rules|2|Drop-mate defense argument/public reversal-token coverage verified.|
|129 game/records|2|Capacity/event-limit and real-consumer boundaries checked; host correction reviewed.|
|85 kernel + pending-obligations reviewers|4 total, parallel pair|713 native assertions and 14/83 later-consumer obligations audited; no fresh production campaign.|
|77 shared boundary|1|Codec-first saveability sequencing and concrete ownership.|
|130 admission|2|Reservation consumption corrected; actual ending/save preservation checked.|
|131 runtime|5|Save-under-lock, replacement admission, identities/floor capacity, deadline-test/evidence reconciliation.|

The repeated native acceptance adds 13:02 of primary window beyond component acceptance. It verified a meaningful whole-kernel/consumer boundary and reused CI. It is a candidate to fold into the end of 129, not proof of wasted verification.131 follow-ups respond to material changes and exact-source reconciliation, not five redundant full native acceptance campaigns.

### Candidate avoidable work, with counterevidence

| Candidate | What it accomplished | Assessment / cheaper mechanism |
|---|---|---|
|Historical-first 81|Acquired/read Tohoku 15-image witness; separated internally dated 1828 text from undated physical copy; corrected movement/quail/promotion evidence; acquired modern texts; produced 39 fixture seeds.|Useful evidence survived the redirection. Requiring further editions/article acquisition to finish a practical playable profile exceeded the clarified objective. Select the delivery profile first; time-box historical work to consequential contradictions. Historical worker 8:23.516 was parallel, not an extra serial 8 minutes; entire 26:27 root is not waste.|
|Fairy-Stockfish dependency|Source-only audit at`53c1f2bd14a2d3b685904ccf8ec9dec8401800ff` found fourth-occurrence repetition and raw-legal drop-mate mismatches; ordinary geometry corroboration.|No Fairy build/run occurred. The 2:31.949 audit was requested, parallel and diagnostic. Reintroducing mandatory comparator build/admission in 85 planning despite known mismatch caused policy/planning rework; removing it before 126 prevented a larger cost. Optional narrow investigation is adequate.|
|Repeated planning assignments|Native planning 13:58 + shared planning 9:34 = 23:32 total; identified exact ownership and codec-before-runtime ordering.|Architecture planning was necessary. Separate owner dispatch/completion reports and parent/component bookkeeping need not be separate sessions. Fold current-scope refinement into the authorized batch before coding.|
|Harness relative to implementation|Reusable strict admission, duplicate-key rejection, observations separate from independent expectations, rotation/negative/metamorphic/reversal helpers; later 713 native assertions.|12650:51 contains policy work/review/CI, not 50 minutes of harness coding. Native 127–129 turns 2:02:17 likewise contain CI. No valid authoring-cost ratio follows. For future games reuse infrastructure; avoid bespoke audit scripts duplicating existing integrity manifests.|
|Repeated CI|Real three-platform build/test/package evidence, caught LF/host/deadline issues.|20 runs on 10 revisions; some package/GUI campaigns are automatic consequences of native-only changes. Use path/dependency-based checks for isolated native slices and one full shared/application campaign per coherent batch; retain platform coverage at integration boundaries.|
|Evidence/provenance bookkeeping|Allows exact-source acceptance, pending ownership, preservation of historical failures, reproducible profile fixtures.|Necessary core evidence. Multiple status summaries, source hashes, task notes and dedicated audit scripts repeat facts. Store one machine-readable result/ownership manifest plus a concise review/task summary. No measured “documentation hours wasted” is available.|
|Stopping after every task|Respects then-current bounded authorization; owner can steer at each boundary.|Material calendar mechanism: modern 15:13:24 between assignments. Grouping already-approved tasks removes a requirement for serial redispatch; owner's availability/overnight preferences mean achievable savings cannot be equated with the whole gap.|

The harness, real complete-history rules, native record admission, application saveability, worker cancellation, stale-result rejection, failed-replacement preservation and multi-platform compatibility remain necessary regardless of ticket size. Grouping cannot remove these product obligations. Most 131 complexity is application integration/capacity/lifecycle, not historical Tori movement research.

## 6. Bottlenecks and revised workflow

**Directly supported conclusions:**

1. Calendar time is dominated by gaps between separate assignments, not evidenced continuous implementation. Native rule delivery took three dedicated turns totaling 2:02:17 including CI/review, while document/runtime turns totaled 2:36:19 including CI/review. Pure authoring effort is unknown.
2. CI runner use is dominated by test/verification and repeated matrices. Queuing is usually seconds; test/build/campaign duration, not queue latency, dominates recorded CI elapsed.
3. Historical and comparator gates were initially stronger than the clarified product objective required. Redirection reused acquired evidence and reached a ratified modern profile in a 21:18 turn; it did not erase all value of earlier research.
4. Independent review produced concrete correctness improvements. Removing it would sacrifice demonstrated safeguards. Starting focused review in parallel helped; its elapsed windows must not be treated as sequential overhead.
5. Separate manifests/workspaces caused repeat integration misses:129 host exhaustiveness and 131 host formatting. A cheap relevant-workspace check before push would likely have avoided those particular CI failures. The deadline test failure required a contract-aware correction, not simply faster CI or relaxed runtime limits.

**Hypotheses and limits:** batching would shorten redispatch gaps and repeated context/evidence work, but saved hours are unmeasured. Scheduling a full application workflow only after a coherent boundary should reduce runner use, but would change when cross-boundary failures surface; retain fast host compile/format checks earlier. There is no demonstrated need to abandon all small tickets. Small tickets remained useful for scope, exact test ownership and resumability; forcing each to become a separate owner assignment is the avoidable coupling.

Recommended operating model for remaining Tori and later Chu/Tenjiku:

1. **One approved batch, one builder/integrator.** At assignment start identify the dependency-ordered tickets already authorized. For Tori, a future authorized assignment can cover remaining presentation, shared acceptance and the next appropriately scoped product boundary with internal checkpoints. This report authorizes or starts none of them. Keep ticket-specific scope, evidence and acceptance; return once with the batch outcome unless a material rule/product ambiguity blocks progress.
2. **Select a coherent playable profile before an acquisition campaign.** Record ordinary-play conflicts for John; explicitly label rare-edge conventions with distinguishing fixtures. Reuse the current engine-independent conformance policy. External engines remain optional investigations, never implied acceptance gates. Chu/Tenjiku compound movement/capture rules still need their own independent expectations.
3. **Plan just in time inside the batch.** One short current implementation plan per active ticket; map shared ownership once. Avoid a separate dispatch to prepare a ready queue unless an owner decision or genuinely large architectural problem requires it.
4. **Review where it changes confidence.** Independently review the selected profile/expected fixtures, then meaningful completed native or shared-boundary diffs. Reuse unchanged component evidence in aggregate acceptance. Keep re-review of changed admission/history behavior; distinguish a brief evidence reconciliation from a fresh full campaign.
5. **Match checks to risk.** Native slices: focused Tori/game conformance and relevant regressions, platform-native coverage at the agreed integration boundary. Shared tagged enums: immediately compile/check all known real consumers, including the standalone Tauri manifest. Application boundary: full native/frontend/package checks and applicable existing-game GUI regressions. Do not rerun expensive GUI matrices solely because documentation/final summaries changed.
6. **Design tests around contracts.** Separate mandatory comfortably bounded successful journeys from near-limit success-or-exact-refusal probes. Preserve independent rule expectations, production deadlines and complete-state nonmutation. Freeze test-only child process instrumentation before a CI campaign.
7. **Keep one evidence record per tested revision.** Command, environment, source hash, outcome, CI IDs and limitations; fixtures/ownership remain authoritative. Link tasks/review summaries to this record. Completion updates should be concise and normally one coherent evidence commit, rather than rewriting the same result in several reports.
8. **Overlap useful work without premature acceptance.** While CI runs, complete independent review, prepare acceptance mapping and inspect/plan the next already-authorized boundary. Keep dependent implementation gated where source can still change, and keep tasks unfinished until their required evidence passes. Cancel superseded campaigns promptly.
9. **Capture timing with low overhead.** Future evidence can record assignment start/end, first substantive edit, verification subprocess start/end, review request/verdict, CI dispatch/job completion and actual blockers in a small structured log. Preserve parallel IDs. Do not introduce a separate timesheet requiring manual prose bookkeeping.

A batching trial should compare elapsed assignment gaps, run counts/runner-minutes, correction loops and final quality across the next coherent authorized batch. Do not promise an hours-saved target from this historical dataset.

## 7. Quantitative inventory and remaining uncertainties

| Metric | Result / qualification |
|---|---|
| Earliest explicit Tori planning→endpoint |7 days 17:54:43; mixed roadmap scope.|
| First focused source inventory→endpoint |7 days 2:03:47; mixed later-game researcher.|
| Modern owner direction→endpoint |21:40:44; formal profile ratification→endpoint 21:20:39.|
| Known Tori-specific observed execution union |6:57:34; late primary 6:53:46 +early Tori researcher 3:47.857; waits included.|
| Modern primary execution / assignment gaps |6:27:19 /15:13:24.|
| Measured productive active labor / pure coding time |Unavailable. No percentage implementation vs orchestration justified.|
| CI runner / distinct job occupancy |12:03:39 /2:32:42.|
| Applicable CI critical-path ceiling / exclusive blocked time |2:03:20 /unknown. Five explicit remaining-CI marker intervals 47:09, with other work overlapping.|
| Dedicated / broader Tori-containing owner assignments |11 /14; bootstrap baseline separately 15 th.|
| Strict planning-only assignments |2 dedicated;4 across 14 including mixed roadmap/MSH-62.|
| Independent review gates / reviewer engagements / review turns |11 /12 /24 in dedicated sequence; early mixed reviews separate.|
| Failed / cancelled CI workflow attempts |4 /2;14 successful; zero same-run reruns; corrective revisions generated 8 additional runs.|
| Local failed verification invocations / full rerun count |At least 29 primary nonzero compile/check/lint/test invocations; complete background/subagent/rerun total unavailable.|
| Integrated dedicated Tori commits |**26**, contiguous`15e6bd66…` through`c89802e…`; includes 12 completion/evidence-record commits (11 completion/checkpoint records plus the native acceptance document commit).|
| Earlier mixed commits attributable in part to Tori |At least`9b48e9ae…`,`16fce28d…`,`f4e564b6…`; broad design/roadmap commits also contain Tori. No honest single “all Tori commits ever” count without a subjective attribution rule.|

The 26 dedicated count excludes the amended-away local 130 completion object`268600a`, preserving the distinction between integrated commits and reflog-only objects. Commits are not units of effort. Review counts depend on the declared unit: one reviewer engagement may have several follow-ups; two concurrent reviewers can serve one acceptance gate.

Historical acquisition duration outside instrumented turns, the Tori fractions of mixed assignments, model inference versus writing time, complete test-authoring time, all background local execution, exclusive review handoff latency, owner's between-assignment activity and marginal hours saved by batching **cannot be established reliably**. No unknown gap has been assigned to coding, research, review or idle time.

## 8. Provenance map and independent report review

Requirements and durable evidence used:

- [Design and owner clarification](../design.md), [conformance policy](../testing/conformance-policy.md), [later-game source inventory](../m4/later-game-sources.md), [early Tori ledger](../m4/research/tori.md).
- [Selected profile](../m4/tori/profile.md), [sources](../m4/tori/sources.md), [historical archive](../m4/tori/archive/2026-10-31/README.md), [profile review](../m4/tori/review.md), [comparator audit](../m4/tori/comparator-audit.md).
- [Native plan/review](../m4/tori/native-plan.md), [planning review](../m4/tori/native-plan-review.md), [harness evidence](../m4/tori/harness-evidence.md), [structural evidence](../m4/tori/native-structural-evidence.md), [local legality evidence](../m4/tori/native-local-evidence.md), [history/records evidence](../m4/tori/native-game-evidence.md), [native acceptance](../m4/tori/native-acceptance.md).
- [Shared plan](../m4/tori/shared-plan.md), [shared review](../m4/tori/shared-plan-review.md), [document evidence](../m4/tori/desktop-document-evidence.md), [runtime evidence](../m4/tori/desktop-runtime-evidence.md), and their committed`evidence/*ci.json`records.
- Backlog records MSH-81,85,126–129,77,130,131, read at`c89802e3e6961eaf8448e59794998e61a277da91`; integrated commit timestamps and primary reflog corroboration.

Primary turn IDs, in table order: `01a193dd-6e6c-7e23-a2a7-757f82c9c7ab`, `01a196f0-162e-7352-a119-80f39e887e84`, `01a19708-b7cc-7e93-a73b-9091a7544b88`, `01a19756-57e1-7481-80f9-06a37b3f0cb3`, `01a197d1-5eee-7b80-ab01-7907b3d9621e`, `01a1980a-c54d-74a0-b928-fe5e9d67211f`, `01a19892-e965-7711-bf93-8c054d97b780`, `01a198db-16fc-7642-b5c5-61a14cbc6ad9`, `01a19900-91a7-7d93-81d3-60e7f3f495a8`, `01a19913-e354-7353-a85c-6f4ccdf3f51e`, `01a19b34-26a1-7ff0-93de-0b2e1e731770`. Their sanitized timestamps and all reviewer identities are retained in the source bundle, so private chat history is not necessary to understand the arithmetic.

Independent read-only report review: **ACCEPT, no remaining findings**, 2 November 2026, separate reviewer `/root/report_review`, thread `01a19bbd-c367-7a91-838e-2f390ba4e377`. The reviewer independently recomputed primary/command/review unions, calendar gaps, CI job/run/step totals, commit/record counts, review units and the 29-invocation lower bound against the original extracts. Corrections: distinguish 905 pre-endpoint commands from 909 full-turn commands; qualify the CI envelope as runs in flight, excluding post-completion polling and separate corrections; make per-task failure counts readable. Final read-only recheck accepted these resolutions. The 0.118-second difference between the modern owner user event and task-start event does not affect rounded totals. No historical test campaign was rerun, and this review does not independently re-ratify game rules. No production or task-status change belongs to this report.


## Appendix A. All CI runs and platform durations

The following tables are generated from the retrieved API timestamps. `Queue first/max` means creation→first job start /creation→latest matrix job start, not a separately instrumented scheduler metric. Source hashes are exact. Platform test/build/GUI columns refer to named step buckets and may include compilation within tests. Zero means no such timed explicit step, not that no compilation happened.

| Run | Source | Workflow | Conclusion | Created Paris | Last job Paris | Queue first/max | Run elapsed | Runner |
|---|---|---|---|---|---|---|---|---|
| [41314482706](https://github.com/jweathers777/mShogi/actions/runs/41314482706) | 134f81ba67ef81918734ed1ed1acb5de7532fb99 | Rust workspace | failure | 2026-11-01T15:06:10+01:00 | 2026-11-01T15:13:55+01:00 | 3/4s | 0:07:45 | 0:15:06 |
| [41314482738](https://github.com/jweathers777/mShogi/actions/runs/41314482738) | 134f81ba67ef81918734ed1ed1acb5de7532fb99 | Desktop packages | success | 2026-11-01T15:06:10+01:00 | 2026-11-01T15:28:03+01:00 | 3/5s | 0:21:53 | 0:46:12 |
| [41315037365](https://github.com/jweathers777/mShogi/actions/runs/41315037365) | d5f98a2af9e38ae690e2fd5c4940a4d26d12407b | Desktop packages | success | 2026-11-01T15:12:32+01:00 | 2026-11-01T15:34:22+01:00 | 2/3s | 0:21:50 | 0:47:12 |
| [41315037374](https://github.com/jweathers777/mShogi/actions/runs/41315037374) | d5f98a2af9e38ae690e2fd5c4940a4d26d12407b | Rust workspace | success | 2026-11-01T15:12:32+01:00 | 2026-11-01T15:25:26+01:00 | 2/3s | 0:12:54 | 0:33:08 |
| [41324873161](https://github.com/jweathers777/mShogi/actions/runs/41324873161) | ce1801d7801f12e447b625739acd6a0c53aa2640 | Desktop packages | success | 2026-11-01T17:09:42+01:00 | 2026-11-01T17:31:29+01:00 | 2/3s | 0:21:47 | 0:44:26 |
| [41324873187](https://github.com/jweathers777/mShogi/actions/runs/41324873187) | ce1801d7801f12e447b625739acd6a0c53aa2640 | Rust workspace | success | 2026-11-01T17:09:42+01:00 | 2026-11-01T17:23:47+01:00 | 2/3s | 0:14:05 | 0:36:09 |
| [41331055051](https://github.com/jweathers777/mShogi/actions/runs/41331055051) | 5c60ac65dc03a7f9afcaa796a2babb252329d361 | Rust workspace | success | 2026-11-01T18:21:26+01:00 | 2026-11-01T18:37:27+01:00 | 3/3s | 0:16:01 | 0:41:51 |
| [41331055108](https://github.com/jweathers777/mShogi/actions/runs/41331055108) | 5c60ac65dc03a7f9afcaa796a2babb252329d361 | Desktop packages | success | 2026-11-01T18:21:26+01:00 | 2026-11-01T18:42:59+01:00 | 3/4s | 0:21:33 | 0:45:42 |
| [41343664138](https://github.com/jweathers777/mShogi/actions/runs/41343664138) | e1cc89d454db5e0ce98da34bf4cda7960d395413 | Rust workspace | success | 2026-11-01T20:49:18+01:00 | 2026-11-01T21:01:51+01:00 | 2/3s | 0:12:33 | 0:35:12 |
| [41343664148](https://github.com/jweathers777/mShogi/actions/runs/41343664148) | e1cc89d454db5e0ce98da34bf4cda7960d395413 | Desktop packages | failure | 2026-11-01T20:49:18+01:00 | 2026-11-01T20:54:06+01:00 | 2/4s | 0:04:48 | 0:11:30 |
| [41343787560](https://github.com/jweathers777/mShogi/actions/runs/41343787560) | 994521bec0637d6f8f17103f1106c0025d950753 | Rust workspace | success | 2026-11-01T20:50:41+01:00 | 2026-11-01T21:05:14+01:00 | 3/3s | 0:14:33 | 0:38:57 |
| [41343787588](https://github.com/jweathers777/mShogi/actions/runs/41343787588) | 994521bec0637d6f8f17103f1106c0025d950753 | Desktop packages | success | 2026-11-01T20:50:41+01:00 | 2026-11-01T21:12:41+01:00 | 3/5s | 0:22:00 | 0:46:41 |
| [41355796434](https://github.com/jweathers777/mShogi/actions/runs/41355796434) | 8fd12f953beeacac46fdd417b2054bedeafe2d6e | Rust workspace | success | 2026-11-01T23:10:43+01:00 | 2026-11-01T23:28:52+01:00 | 3/304s | 0:18:09 | 0:45:48 |
| [41355796473](https://github.com/jweathers777/mShogi/actions/runs/41355796473) | 8fd12f953beeacac46fdd417b2054bedeafe2d6e | Desktop packages | success | 2026-11-01T23:10:43+01:00 | 2026-11-01T23:32:15+01:00 | 3/3s | 0:21:32 | 0:43:03 |
| [41415490841](https://github.com/jweathers777/mShogi/actions/runs/41415490841) | 253e4c949614d133b5295f6a362e3d5279adcef3 | Desktop packages | failure | 2026-11-02T09:46:21+01:00 | 2026-11-02T09:49:32+01:00 | 2/7s | 0:03:11 | 0:07:44 |
| [41415490915](https://github.com/jweathers777/mShogi/actions/runs/41415490915) | 253e4c949614d133b5295f6a362e3d5279adcef3 | Rust workspace | failure | 2026-11-02T09:46:21+01:00 | 2026-11-02T09:57:49+01:00 | 2/10s | 0:11:28 | 0:28:09 |
| [41415982022](https://github.com/jweathers777/mShogi/actions/runs/41415982022) | ce48e84e64b9631343416acec865505dbbab570d | Desktop packages | cancelled | 2026-11-02T09:50:30+01:00 | 2026-11-02T10:04:51+01:00 | 2/4s | 0:14:21 | 0:35:53 |
| [41415982048](https://github.com/jweathers777/mShogi/actions/runs/41415982048) | ce48e84e64b9631343416acec865505dbbab570d | Rust workspace | cancelled | 2026-11-02T09:50:30+01:00 | 2026-11-02T10:04:51+01:00 | 2/3s | 0:14:21 | 0:36:29 |
| [41417589059](https://github.com/jweathers777/mShogi/actions/runs/41417589059) | 6a2edd71b3136a9c2043f9137eb6d3fb8abf120d | Rust workspace | success | 2026-11-02T10:03:29+01:00 | 2026-11-02T10:20:01+01:00 | 2/8s | 0:16:32 | 0:40:31 |
| [41417589127](https://github.com/jweathers777/mShogi/actions/runs/41417589127) | 6a2edd71b3136a9c2043f9137eb6d3fb8abf120d | Desktop packages | success | 2026-11-02T10:03:29+01:00 | 2026-11-02T10:22:51+01:00 | 3/5s | 0:19:22 | 0:43:56 |


| Run | Platform | Start Paris | End Paris | Job | Build steps | Native tests/checks | Composite checks | GUI |
|---|---|---|---|---|---|---|---|---|
| [41314482706](https://github.com/jweathers777/mShogi/actions/runs/41314482706) | Rust (windows-2022) | 2026-11-01T15:06:13+01:00 | 2026-11-01T15:08:12+01:00 | 0:01:59 | 0:00:00 | 0:00:58 | 0:00:00 | 0:00:00 |
| [41314482706](https://github.com/jweathers777/mShogi/actions/runs/41314482706) | Rust (ubuntu-24.04) | 2026-11-01T15:06:13+01:00 | 2026-11-01T15:13:55+01:00 | 0:07:42 | 0:00:00 | 0:07:11 | 0:00:00 | 0:00:00 |
| [41314482706](https://github.com/jweathers777/mShogi/actions/runs/41314482706) | Rust (macos-15) | 2026-11-01T15:06:14+01:00 | 2026-11-01T15:11:39+01:00 | 0:05:25 | 0:00:00 | 0:04:39 | 0:00:00 | 0:00:00 |
| [41314482738](https://github.com/jweathers777/mShogi/actions/runs/41314482738) | Desktop (windows-2022) | 2026-11-01T15:06:13+01:00 | 2026-11-01T15:28:03+01:00 | 0:21:50 | 0:06:20 | 0:00:00 | 0:05:52 | 0:07:44 |
| [41314482738](https://github.com/jweathers777/mShogi/actions/runs/41314482738) | Desktop (ubuntu-24.04) | 2026-11-01T15:06:13+01:00 | 2026-11-01T15:22:10+01:00 | 0:15:57 | 0:03:38 | 0:00:00 | 0:03:26 | 0:07:03 |
| [41314482738](https://github.com/jweathers777/mShogi/actions/runs/41314482738) | Desktop (macos-15) | 2026-11-01T15:06:15+01:00 | 2026-11-01T15:14:40+01:00 | 0:08:25 | 0:03:18 | 0:00:00 | 0:04:16 | 0:00:00 |
| [41315037365](https://github.com/jweathers777/mShogi/actions/runs/41315037365) | Desktop (ubuntu-24.04) | 2026-11-01T15:12:34+01:00 | 2026-11-01T15:29:58+01:00 | 0:17:24 | 0:03:37 | 0:00:00 | 0:03:41 | 0:07:16 |
| [41315037365](https://github.com/jweathers777/mShogi/actions/runs/41315037365) | Desktop (macos-15) | 2026-11-01T15:12:35+01:00 | 2026-11-01T15:20:35+01:00 | 0:08:00 | 0:03:08 | 0:00:00 | 0:04:04 | 0:00:00 |
| [41315037365](https://github.com/jweathers777/mShogi/actions/runs/41315037365) | Desktop (windows-2022) | 2026-11-01T15:12:34+01:00 | 2026-11-01T15:34:22+01:00 | 0:21:48 | 0:06:19 | 0:00:00 | 0:05:41 | 0:07:45 |
| [41315037374](https://github.com/jweathers777/mShogi/actions/runs/41315037374) | Rust (ubuntu-24.04) | 2026-11-01T15:12:34+01:00 | 2026-11-01T15:21:11+01:00 | 0:08:37 | 0:00:00 | 0:08:04 | 0:00:00 | 0:00:00 |
| [41315037374](https://github.com/jweathers777/mShogi/actions/runs/41315037374) | Rust (macos-15) | 2026-11-01T15:12:35+01:00 | 2026-11-01T15:24:14+01:00 | 0:11:39 | 0:00:01 | 0:10:33 | 0:00:00 | 0:00:00 |
| [41315037374](https://github.com/jweathers777/mShogi/actions/runs/41315037374) | Rust (windows-2022) | 2026-11-01T15:12:34+01:00 | 2026-11-01T15:25:26+01:00 | 0:12:52 | 0:00:00 | 0:11:47 | 0:00:00 | 0:00:00 |
| [41324873161](https://github.com/jweathers777/mShogi/actions/runs/41324873161) | Desktop (windows-2022) | 2026-11-01T17:09:44+01:00 | 2026-11-01T17:31:29+01:00 | 0:21:45 | 0:06:08 | 0:00:00 | 0:05:42 | 0:07:59 |
| [41324873161](https://github.com/jweathers777/mShogi/actions/runs/41324873161) | Desktop (macos-15) | 2026-11-01T17:09:45+01:00 | 2026-11-01T17:18:24+01:00 | 0:08:39 | 0:03:22 | 0:00:00 | 0:04:29 | 0:00:00 |
| [41324873161](https://github.com/jweathers777/mShogi/actions/runs/41324873161) | Desktop (ubuntu-24.04) | 2026-11-01T17:09:45+01:00 | 2026-11-01T17:23:47+01:00 | 0:14:02 | 0:02:50 | 0:00:00 | 0:02:50 | 0:06:38 |
| [41324873187](https://github.com/jweathers777/mShogi/actions/runs/41324873187) | Rust (macos-15) | 2026-11-01T17:09:45+01:00 | 2026-11-01T17:19:00+01:00 | 0:09:15 | 0:00:00 | 0:08:30 | 0:00:00 | 0:00:00 |
| [41324873187](https://github.com/jweathers777/mShogi/actions/runs/41324873187) | Rust (windows-2022) | 2026-11-01T17:09:44+01:00 | 2026-11-01T17:22:36+01:00 | 0:12:52 | 0:00:00 | 0:11:52 | 0:00:00 | 0:00:00 |
| [41324873187](https://github.com/jweathers777/mShogi/actions/runs/41324873187) | Rust (ubuntu-24.04) | 2026-11-01T17:09:45+01:00 | 2026-11-01T17:23:47+01:00 | 0:14:02 | 0:00:00 | 0:13:24 | 0:00:00 | 0:00:00 |
| [41331055051](https://github.com/jweathers777/mShogi/actions/runs/41331055051) | Rust (windows-2022) | 2026-11-01T18:21:29+01:00 | 2026-11-01T18:37:27+01:00 | 0:15:58 | 0:00:00 | 0:14:58 | 0:00:00 | 0:00:00 |
| [41331055051](https://github.com/jweathers777/mShogi/actions/runs/41331055051) | Rust (ubuntu-24.04) | 2026-11-01T18:21:29+01:00 | 2026-11-01T18:35:28+01:00 | 0:13:59 | 0:00:00 | 0:13:25 | 0:00:00 | 0:00:00 |
| [41331055051](https://github.com/jweathers777/mShogi/actions/runs/41331055051) | Rust (macos-15) | 2026-11-01T18:21:29+01:00 | 2026-11-01T18:33:23+01:00 | 0:11:54 | 0:00:00 | 0:11:01 | 0:00:00 | 0:00:00 |
| [41331055108](https://github.com/jweathers777/mShogi/actions/runs/41331055108) | Desktop (ubuntu-24.04) | 2026-11-01T18:21:29+01:00 | 2026-11-01T18:37:55+01:00 | 0:16:26 | 0:03:42 | 0:00:00 | 0:03:44 | 0:07:18 |
| [41331055108](https://github.com/jweathers777/mShogi/actions/runs/41331055108) | Desktop (windows-2022) | 2026-11-01T18:21:29+01:00 | 2026-11-01T18:42:59+01:00 | 0:21:30 | 0:06:22 | 0:00:00 | 0:05:38 | 0:07:33 |
| [41331055108](https://github.com/jweathers777/mShogi/actions/runs/41331055108) | Desktop (macos-15) | 2026-11-01T18:21:30+01:00 | 2026-11-01T18:29:16+01:00 | 0:07:46 | 0:03:08 | 0:00:00 | 0:03:51 | 0:00:00 |
| [41343664138](https://github.com/jweathers777/mShogi/actions/runs/41343664138) | Rust (macos-15) | 2026-11-01T20:49:21+01:00 | 2026-11-01T21:01:45+01:00 | 0:12:24 | 0:00:00 | 0:11:22 | 0:00:00 | 0:00:00 |
| [41343664138](https://github.com/jweathers777/mShogi/actions/runs/41343664138) | Rust (ubuntu-24.04) | 2026-11-01T20:49:20+01:00 | 2026-11-01T21:01:51+01:00 | 0:12:31 | 0:00:00 | 0:11:53 | 0:00:00 | 0:00:00 |
| [41343664138](https://github.com/jweathers777/mShogi/actions/runs/41343664138) | Rust (windows-2022) | 2026-11-01T20:49:20+01:00 | 2026-11-01T20:59:37+01:00 | 0:10:17 | 0:00:00 | 0:09:27 | 0:00:00 | 0:00:00 |
| [41343664148](https://github.com/jweathers777/mShogi/actions/runs/41343664148) | Desktop (macos-15) | 2026-11-01T20:49:22+01:00 | 2026-11-01T20:52:43+01:00 | 0:03:21 | 0:00:00 | 0:00:00 | 0:02:36 | 0:00:00 |
| [41343664148](https://github.com/jweathers777/mShogi/actions/runs/41343664148) | Desktop (ubuntu-24.04) | 2026-11-01T20:49:20+01:00 | 2026-11-01T20:52:43+01:00 | 0:03:23 | 0:00:00 | 0:00:00 | 0:02:23 | 0:00:00 |
| [41343664148](https://github.com/jweathers777/mShogi/actions/runs/41343664148) | Desktop (windows-2022) | 2026-11-01T20:49:20+01:00 | 2026-11-01T20:54:06+01:00 | 0:04:46 | 0:00:00 | 0:00:00 | 0:03:56 | 0:00:00 |
| [41343787560](https://github.com/jweathers777/mShogi/actions/runs/41343787560) | Rust (macos-15) | 2026-11-01T20:50:44+01:00 | 2026-11-01T21:01:49+01:00 | 0:11:05 | 0:00:00 | 0:10:25 | 0:00:00 | 0:00:00 |
| [41343787560](https://github.com/jweathers777/mShogi/actions/runs/41343787560) | Rust (ubuntu-24.04) | 2026-11-01T20:50:44+01:00 | 2026-11-01T21:05:14+01:00 | 0:14:30 | 0:00:00 | 0:13:49 | 0:00:00 | 0:00:00 |
| [41343787560](https://github.com/jweathers777/mShogi/actions/runs/41343787560) | Rust (windows-2022) | 2026-11-01T20:50:44+01:00 | 2026-11-01T21:04:06+01:00 | 0:13:22 | 0:00:00 | 0:12:14 | 0:00:00 | 0:00:00 |
| [41343787588](https://github.com/jweathers777/mShogi/actions/runs/41343787588) | Desktop (ubuntu-24.04) | 2026-11-01T20:50:44+01:00 | 2026-11-01T21:07:24+01:00 | 0:16:40 | 0:03:40 | 0:00:00 | 0:03:44 | 0:07:16 |
| [41343787588](https://github.com/jweathers777/mShogi/actions/runs/41343787588) | Desktop (windows-2022) | 2026-11-01T20:50:44+01:00 | 2026-11-01T21:12:41+01:00 | 0:21:57 | 0:06:35 | 0:00:00 | 0:05:41 | 0:07:46 |
| [41343787588](https://github.com/jweathers777/mShogi/actions/runs/41343787588) | Desktop (macos-15) | 2026-11-01T20:50:46+01:00 | 2026-11-01T20:58:50+01:00 | 0:08:04 | 0:03:28 | 0:00:00 | 0:03:55 | 0:00:00 |
| [41355796434](https://github.com/jweathers777/mShogi/actions/runs/41355796434) | Rust (ubuntu-24.04) | 2026-11-01T23:10:46+01:00 | 2026-11-01T23:26:14+01:00 | 0:15:28 | 0:00:00 | 0:14:46 | 0:00:00 | 0:00:00 |
| [41355796434](https://github.com/jweathers777/mShogi/actions/runs/41355796434) | Rust (macos-15) | 2026-11-01T23:15:47+01:00 | 2026-11-01T23:28:52+01:00 | 0:13:05 | 0:00:00 | 0:12:11 | 0:00:00 | 0:00:00 |
| [41355796434](https://github.com/jweathers777/mShogi/actions/runs/41355796434) | Rust (windows-2022) | 2026-11-01T23:10:46+01:00 | 2026-11-01T23:28:01+01:00 | 0:17:15 | 0:00:00 | 0:16:06 | 0:00:00 | 0:00:00 |
| [41355796473](https://github.com/jweathers777/mShogi/actions/runs/41355796473) | Desktop (macos-15) | 2026-11-01T23:10:46+01:00 | 2026-11-01T23:15:41+01:00 | 0:04:55 | 0:01:58 | 0:00:00 | 0:02:24 | 0:00:00 |
| [41355796473](https://github.com/jweathers777/mShogi/actions/runs/41355796473) | Desktop (ubuntu-24.04) | 2026-11-01T23:10:46+01:00 | 2026-11-01T23:27:25+01:00 | 0:16:39 | 0:03:43 | 0:00:00 | 0:03:46 | 0:07:17 |
| [41355796473](https://github.com/jweathers777/mShogi/actions/runs/41355796473) | Desktop (windows-2022) | 2026-11-01T23:10:46+01:00 | 2026-11-01T23:32:15+01:00 | 0:21:29 | 0:05:59 | 0:00:00 | 0:05:07 | 0:08:26 |
| [41415490841](https://github.com/jweathers777/mShogi/actions/runs/41415490841) | Desktop (ubuntu-24.04) | 2026-11-02T09:46:26+01:00 | 2026-11-02T09:49:11+01:00 | 0:02:45 | 0:00:00 | 0:00:00 | 0:00:56 | 0:00:00 |
| [41415490841](https://github.com/jweathers777/mShogi/actions/runs/41415490841) | Desktop (macos-15) | 2026-11-02T09:46:23+01:00 | 2026-11-02T09:48:18+01:00 | 0:01:55 | 0:00:00 | 0:00:00 | 0:01:20 | 0:00:00 |
| [41415490841](https://github.com/jweathers777/mShogi/actions/runs/41415490841) | Desktop (windows-2022) | 2026-11-02T09:46:28+01:00 | 2026-11-02T09:49:32+01:00 | 0:03:04 | 0:00:00 | 0:00:00 | 0:01:52 | 0:00:00 |
| [41415490915](https://github.com/jweathers777/mShogi/actions/runs/41415490915) | Rust (macos-15) | 2026-11-02T09:46:24+01:00 | 2026-11-02T09:55:44+01:00 | 0:09:20 | 0:00:00 | 0:08:28 | 0:00:00 | 0:00:00 |
| [41415490915](https://github.com/jweathers777/mShogi/actions/runs/41415490915) | Rust (windows-2022) | 2026-11-02T09:46:31+01:00 | 2026-11-02T09:57:49+01:00 | 0:11:18 | 0:00:00 | 0:10:13 | 0:00:00 | 0:00:00 |
| [41415490915](https://github.com/jweathers777/mShogi/actions/runs/41415490915) | Rust (ubuntu-24.04) | 2026-11-02T09:46:23+01:00 | 2026-11-02T09:53:54+01:00 | 0:07:31 | 0:00:00 | 0:06:55 | 0:00:00 | 0:00:00 |
| [41415982022](https://github.com/jweathers777/mShogi/actions/runs/41415982022) | Desktop (windows-2022) | 2026-11-02T09:50:34+01:00 | 2026-11-02T10:04:17+01:00 | 0:13:43 | 0:04:20 | 0:00:00 | 0:04:05 | 0:03:37 |
| [41415982022](https://github.com/jweathers777/mShogi/actions/runs/41415982022) | Desktop (macos-15) | 2026-11-02T09:50:33+01:00 | 2026-11-02T09:58:24+01:00 | 0:07:51 | 0:03:07 | 0:00:00 | 0:03:47 | 0:00:00 |
| [41415982022](https://github.com/jweathers777/mShogi/actions/runs/41415982022) | Desktop (ubuntu-24.04) | 2026-11-02T09:50:32+01:00 | 2026-11-02T10:04:51+01:00 | 0:14:19 | 0:03:49 | 0:00:00 | 0:03:40 | 0:05:02 |
| [41415982048](https://github.com/jweathers777/mShogi/actions/runs/41415982048) | Rust (ubuntu-24.04) | 2026-11-02T09:50:32+01:00 | 2026-11-02T10:00:23+01:00 | 0:09:51 | 0:00:00 | 0:08:50 | 0:00:00 | 0:00:00 |
| [41415982048](https://github.com/jweathers777/mShogi/actions/runs/41415982048) | Rust (macos-15) | 2026-11-02T09:50:33+01:00 | 2026-11-02T10:04:51+01:00 | 0:14:18 | 0:00:00 | 0:12:58 | 0:00:00 | 0:00:00 |
| [41415982048](https://github.com/jweathers777/mShogi/actions/runs/41415982048) | Rust (windows-2022) | 2026-11-02T09:50:32+01:00 | 2026-11-02T10:02:52+01:00 | 0:12:20 | 0:00:00 | 0:11:16 | 0:00:00 | 0:00:00 |
| [41417589059](https://github.com/jweathers777/mShogi/actions/runs/41417589059) | Rust (macos-15) | 2026-11-02T10:03:37+01:00 | 2026-11-02T10:20:01+01:00 | 0:16:24 | 0:00:00 | 0:15:32 | 0:00:00 | 0:00:00 |
| [41417589059](https://github.com/jweathers777/mShogi/actions/runs/41417589059) | Rust (ubuntu-24.04) | 2026-11-02T10:03:31+01:00 | 2026-11-02T10:14:33+01:00 | 0:11:02 | 0:00:00 | 0:10:28 | 0:00:00 | 0:00:00 |
| [41417589059](https://github.com/jweathers777/mShogi/actions/runs/41417589059) | Rust (windows-2022) | 2026-11-02T10:03:32+01:00 | 2026-11-02T10:16:37+01:00 | 0:13:05 | 0:00:00 | 0:12:10 | 0:00:00 | 0:00:00 |
| [41417589127](https://github.com/jweathers777/mShogi/actions/runs/41417589127) | Desktop (ubuntu-24.04) | 2026-11-02T10:03:32+01:00 | 2026-11-02T10:20:20+01:00 | 0:16:48 | 0:03:55 | 0:00:00 | 0:03:54 | 0:07:18 |
| [41417589127](https://github.com/jweathers777/mShogi/actions/runs/41417589127) | Desktop (macos-15) | 2026-11-02T10:03:34+01:00 | 2026-11-02T10:11:23+01:00 | 0:07:49 | 0:02:33 | 0:00:00 | 0:04:25 | 0:00:00 |
| [41417589127](https://github.com/jweathers777/mShogi/actions/runs/41417589127) | Desktop (windows-2022) | 2026-11-02T10:03:32+01:00 | 2026-11-02T10:22:51+01:00 | 0:19:19 | 0:04:58 | 0:00:00 | 0:04:34 | 0:08:00 |
