# MSH-161 (Competitive Gameplay Diagnosis): batch03 followup execution review

Passed. All seven planned new alternatives completed as seven 20M singleton queries. The audit checked exact plan, selection, prior-plan, consumed-success and completion-verification bindings. It checked each new query against the accepted full native history, forced move, node budget, original build/image/options, clear-hash commands, legal PV and native replay receipt. Existing plan-aware artifact verification was reused; there was no duplicate full-artifact hash pass, engine run, retrieval or cloud submission.

The new alternatives were main-03-01 game1 ply33 `b6d8`, ply67 `g8h7`, ply69 `g8h7`; main-03-02 game0 ply18 `g1f3`, ply21 `b7b5`, ply33 `a6b6`; and main-03-06 game0 ply22 `g2g3`. Seven prior played queries have verified success/artifact hashes, identical historical roots, original Stockfish identity and equal 20M budgets. They were reused and were not counted again as new jobs, queries or compute.

No scientific attempt failed; no query was truncated and no claim remained. The new queries have three selected-report flags, one directional-bound conflict and four reports meeting the two-report stability heuristic. These overlap and are not confirmed error counts. The original AutoClaim conditions remain; a draw alone is not evidence of weakness. Independent gameplay reviewers own the scientific conclusion, including linked and earlier uncertainty. This execution pass closes no scientific gate by itself.

| Resource layer | Wall seconds | Child CPU seconds |
| --- | ---: | ---: |
| Seven new parent jobs | 184.737 | 171.963 |
| Seven nested queries | 171.978 | 171.744 |

The longest query took 26.955 seconds. Query totals overlap parent totals; do not add both. Reused queries remain accounted for in the original panel.

Submission returned zero. The accepted job was `msh161-primary-b03-followup-v1`, UID `msh161-primary-b03-9e54b249-d3ec-4b260`. Its separate terminal event reports SUCCEEDED. One read-only cloud snapshot found seven successful tasks on seven C3 workers, with peak concurrency seven. All seven successful VM insertions have deletion completions. There were no failed insertions or audit error statuses in the bounded job window, and no matching live VM or disk remained. The same review invocation found neither wrapper PID31181 nor child PID31184. The wrapper separately recorded child return zero, its own group settled without a residual, and queue acceptance with return zero. This does not claim a general descendant-process census.

Cloud creation to final task took 77.377 seconds; running tasks spanned 36.121 seconds. VM lifetimes totaled 0.186804 hours, priced at the recorded US$0.201608/hour: US$0.037661 estimated compute. The full wrapper took 171.757 seconds, including 35.476 seconds before the durable submission marker and 57.158 seconds after the last cloud task. Post-cloud time combines event delivery, retrieval, verification, publication and shutdown; download-only time is not measured. Actual billing remains unknown.

Primary cumulative totals are now 8,342 scientific jobs, 24,967 reference queries, 43.042446 VM-hours and US$8.677702 estimated compute. This includes three base batches, three 20M panels and nine new followup queries: two for batch02 and seven for batch03. Mac, pilots and the abandoned policy transition remain separate. Sixty games have finished; 40 are fully adjudicated until the coordinator closes batch03 scientific review.

Mean-workload scaling gives US$28.93 compute for all 200 games and US$20.25 for the remaining 140. Measured cloud-stage elapsed time totals 7.472829 hours; corresponding full/remaining mean estimates are 24.91/17.44 cloud hours. These exclude preflight, retrieval and review. Retain the conditional full-study US$35–90 inclusive cost and 29–48 processing-hour scenarios. These are planning estimates, not bills, deadlines, confidence intervals or caps. Exact resource arithmetic, raw snapshot observations, audit procedure and limits are preserved in the JSON records.

No execution or cleanup blocker was found. Scientific closure and admission of the next batch remain with the coordinator.
