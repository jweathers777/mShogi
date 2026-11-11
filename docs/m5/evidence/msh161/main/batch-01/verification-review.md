# Batch01 independent verification review

Reviewer: execution_review. Scope: completed first main batch against the frozen
method, raw records, query coverage, confirmation/audit selection and resource
accounting. **No engine or new native replay was launched during this review.**
The reproducible offline command is:

```sh
/private/tmp/msh139/venv/bin/python docs/m5/evidence/msh161/main/batch-01/verification-audit.py
```

Its final run passed; [audit.json](audit.json) records exact evidence hashes,
environment, counts, timing, quality flags, all82 audit strata and native results.
Two reviewer-only audit assumptions were corrected before that final run: missing
selected nonbound reports legitimately have no PV-validation field, and screening
runs inline rather than as a third subprocess per pair. No experiment was retried,
rewritten or counted successful because of those corrections.

## Verified completion and conditions

All20 games reached native conclusions, with complete paired histories, clean
recorded process exits and successful native replay receipts for the exact hashed
reports. The existing replay evidence is reused, not described as a new invocation.
Independent legal replay and prefix/actual-bestmove reconciliation also passed.
No technical cap, missing result, truncation, failed stage/query, unexpected retry,
or unreconciled inflight/temporary artifact was found.

| Stratum | Games | Candidate wins | Draws | Candidate losses |
| --- | ---: | ---: | ---: | ---: |
| Prepared, nine opening pairs | 18 | 8 | 2 | 8 |
| Unaided, one start pair | 2 | 2 | 0 | 0 |
| Total | 20 | 10 | 2 | 8 |

These are actual native results, not finite-analysis labels or a general rating.
Prepared and unaided results remain separate; only20 of200 main games are complete.

Every frozen source/schedule/binary hash matched. Actual native commands were
`go movetime 5000`; reference commands used the requested fixed nodes and full
history, with fresh-game/clear-hash commands and frozen options. Native colors,
claim policy, operational caps and identities match the schedule. Candidate and
Oxidation response medians were5.001657s and5.000421s; maxima5.028277s and5.022702s.
Early responses are retained rather than forced to a fictitious five seconds.
The host was shared/unpinned, with recorded starting/ending load. This establishes
recorded comparable requested conditions, not isolated CPU throughput.

## Complete query and selection coverage

- 2,494 played decisions plus288 prepared-prefix decisions produced5,564 initial
 queries: unrestricted and forced played move for each decision, both players.
 Prepared moves are excluded from competitor follow-up attribution.
- All1,167 v1 follow-up played decisions entered the matched5M queue. Additionally,
 all174 fixed-hash audit decisions match the frozen game/player/phase selection
 exactly, including every nonempty cell's minimum and rounding rule.
- The1,341 selected decisions produced2,014 singleton queries after668 identical
 choices were deduplicated. No alternative was missing in this batch. Every
 receipt's complete root/history/mover/played action, forced choice, node budget,
 options, executable identity and manifest matched its plan.
- All2,035 orchestration steps exited successfully. All7,578 reference attempts
 succeeded; no failed attempts were silently dropped. Confirmation outputs remain
 explicitly unadjudicated, with `confirmed_event=false`.

The audit independently regenerated v1 screens and phase tags, reconstructed the
whole selection plan, checked all receipt hashes and command histories, reconciled
resource totals and verified that recorded native/reference execution intervals
had no overlap. The stored states are consistent with the serial execution policy.

## Data quality and the missed-error audit gate

Numerical uncertainty remains substantial and is not an execution failure. Initial
reference receipts have2,040 stale-final-report flags,373 directional-bound
conflicts and9 missing aligned numerical/PV reports. Those nine retain explicit
uncertainty and legal returned root moves; they were not called exact or discarded.
The initial played-decision screen has1,117 flagged rows,239 raw losses>=.05 and
269 negative raw differences; these categories overlap and are not mistake counts.

At5M,1,900/2,014 queries meet the two-report stability heuristic, but1,021 retain
a stale-final-report flag and162 have directional-bound conflicts. None lacks a
numerical estimate. Stability alone therefore cannot certify all these judgments.
Keep conflicts and provisional status for the required consequential20M review.

The174 nonflagged audit decisions comprise111 same-choice controls,59 stable
two-move comparisons and4 uncertain two-move comparisons. **Zero stable audit
comparisons cross expected-score loss.05**, using the frozen eligibility that
allows a lone stale-final-report flag but rejects other defects/conflicts. None
of the four uncertain two-move comparisons has a raw.05 crossing either. Thus
this audit reveals **no automatic missed-important-error stratum-expansion
trigger**. The111 controls establish behavior under the tested same choice, not
global best-play correctness or exhaustive discovery of alternatives. Uncertain
rows and consequential claims still need disposition; a later established missed
important error must activate the recorded expansion policy.

## Actual resource accounting

| Component | Attempts/games | Process wall seconds | Child CPU seconds |
| --- | ---: | ---: | ---: |
| Native games/replay/cleanup | 20 games | 11,929.047 | 11,922.292 |
| Initial500k reference | 5,564 queries | 3,264.835 | 3,123.491 |
| Matched5M reference | 2,014 queries | 8,980.574 | 8,908.227 |

Native response intervals sum11,926.829s. Total chain elapsed is24,822.438s
(about6h54m), including orchestration/artifact overhead. Stage, query and whole-chain
wall figures overlap: **do not add the chain duration again**. Maximum initial
query wall time was0.944s; maximum5M query time6.021s, within individual bounds.
Artifacts totaled324,820,554 bytes at audit time, before later review material and
archiving. All actual resource use remains reported; no aggregate budget stop
truncated the planned coverage.

## Twenty-million-node follow-on helper

Reviewed the new, unfrozen `review_checks.py` helper at SHA256
`67cdaaa2f01076bbde4d1753462a7d646ee2ec6843d632d47f136c844828ed02`.
No launch blocker found. It validates full legal histories, duplicate decisions
and inclusion of the played move before creating output; binds the plan, report
and main freeze; uses exclusive output; and delegates engine locks, bounds, raw
receipts and cleanup to the existing reviewed analysis helper. Discovery includes
unrestricted and played20M queries, then evaluates a distinct discovered choice
with a matched singleton. It preserves failures and leaves results for independent
adjudication. No method or production behavior is changed by this orchestration.
The coordinator separately reports real-plan preflight and negative-input checks;
this review does not claim to have executed those checks or any20M search.

## Gate recommendation

**No integrity or coverage blocker was found.** Run the queued consequential20M
plan and complete the independent gameplay/adjudication checkpoint before batch02,
as the coordinator proposes. There is no automatic audit-expansion trigger from
this batch, but future confirmed misses must still be followed through. Preserve
remaining episode, favorable-boundary, strong-play and every-decision disposition
obligations. Continue the full planned scientific coverage; this first batch
neither settles weakness rankings nor authorizes engineering changes.
