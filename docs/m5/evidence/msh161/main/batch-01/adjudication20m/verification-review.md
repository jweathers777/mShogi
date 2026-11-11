# Batch01 consequential20M integrity review

Independent reviewer: execution_review. **Passed; no integrity blocker to batch02.**
This clears the evidence's execution and matching, not its chess interpretations.
Independent event/episode adjudication and any resulting follow-up remain the
progression gate. No engines, new native replay or queue command ran in this review.

Reproduction:

```sh
/private/tmp/msh139/venv/bin/python docs/m5/evidence/msh161/main/batch-01/adjudication20m/verification-audit.py
```

The final offline run passed. [audit.json](audit.json) records the audit script,
plan, freeze, chain and notification hashes, environment, exact totals and native
draw context. Frozen main helpers remain unchanged.

## Verified evidence

All73 planned decisions across ten native reports match the final result list.
All142 orchestration steps exited0 and produced144 query receipts; the two
unrestricted-discovery steps each also contain a forced-played query, explaining
the two extra receipts. Every expected receipt is referenced exactly once. No
unreferenced attempt, failed query, truncated raw output, unsettled process group,
inflight marker or partial temporary file was found.

Verified plan/freeze/report/source hashes; per-query manifest, executable, options
and full canonical history; legal planned and returned root moves; exact20M-node
commands; fresh-game and clear-hash commands; independently legal selected PVs;
receipt hashes and preserved finite-evidence descriptions. All matched choices
received the same requested node budget from the same root/history. Actual work
and finite reported depth remain in raw records, not equated with proven values.

For both discovery decisions, the played move was searched as a singleton alongside
unrestricted discovery, and a distinct discovered alternative received its own
matched singleton. Reusing the already searched played choice is correct; an
unrestricted report alone was not substituted for the alternative's singleton.
All supplied additional alternatives remain represented. Recorded query intervals
are serial and begin after the original batch's computation completed.

## Uncertainty remains explicit

135/144 queries meet the last-two-report stability heuristic.72 retain the
stale-final-report flag, six have directional final-bound conflicts, and none
lacks an aligned numerical estimate. These categories overlap. A clean exit or
stable earlier reports cannot erase final-bound conflict. The results still
require the frozen finite-evidence and consequence tests before entering confirmed
mistake counts; the audit makes no automatic event or technical-mechanism claim.

## Native claim-policy interpretation

The first batch contains exactly two native draws:

| Game | Candidate color | Native result |
| --- | --- | --- |
| main-01-00, game0 | White | Dead position |
| main-01-06, game1 | Black | Threefold claim by White under `auto_claim` |

The second game's canonical history contains100 actually played plies. After
zero-based ply99, `a8d5` (50...Bd5), the native controller accepts White's intended
`f3g4` threefold claim. That intended move is **not played**. The final accepted
claim request and native outcome are preserved in audit.json.

At zero-based ply98, the20M records give White's `g4f3` and `g4f5` expected-score
proxy1.0. That finite opinion does not replace the actual draw. Stockfish's
self-play assessment and a controller that automatically exercises a valid claim
have different decision/outcome semantics. The observed controller claim must
not be attributed to an independently chosen opponent move, converted into a
search/rules defect, or used alone to certify candidate recovery/conversion skill.
This is an interpretation limitation under the frozen conditions, not an execution
failure or authorization to change claim policy.

## Resource reconciliation

| Measure | Additional20M work |
| --- | ---: |
| Query attempts | 144 |
| Summed query process wall | 2,546.393s |
| Summed query child CPU | 2,540.266s |
| Summed orchestration-step wall | 2,574.360s |
| Whole chain elapsed | 2,578.532s |
| Maximum individual query wall | 22.884s |

These wall figures overlap and must not be added together. All individual queries
were within the60-second response bound plus cleanup allowance. Artifacts totaled
8,876,522 bytes before this audit's final documents; the audit JSON records its
current measured snapshot.

Adding only these new query attempts to the existing batch audit yields7,722
unique reference attempts. Native games remain20. Combined native/reference
process wall is26,720.850s and child CPU26,494.276s; neither includes a second charge
for parent stage/chain elapsed time. Existing initial analysis and5M evidence were
not rerun or recounted as new work.

The actual notification receipt records computation exit0, its own wrapper group
settled, queue return0 and acceptance. This establishes the completed exit-trigger
notification, not completion of later model review. No timed watcher is involved.

**Recommendation:** no integrity repair or rerun is needed. Finish the independent
finite-event/episode interpretation, retain the automatic-claim limitation and any
unresolved cases, then proceed to batch02 under the unchanged frozen study. Do not
use this review as a final ranking or a reason to reduce the planned200-game coverage.
