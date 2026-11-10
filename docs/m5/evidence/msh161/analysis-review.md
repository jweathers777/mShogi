# Independent reference execution review — MSH-161

Reviewer: execution_review agent; 2026-11-10, macOS arm64. Scope: unattended
qualification follow-on reference analysis, summary and source balance screening.
No engines ran during this review. This is execution/method review, not an
endorsement of resulting chess judgments or permission to bypass pilot review.

## Decision

**No blocker remains to launching the unattended qualification follow-on chain.**
Its automatic two-query smoke must pass before bulk analysis. After completion,
independent gameplay analysis and protocol freeze are still required before the
main sample. An initial node-limited result is a screen, never a confirmed mistake.

Reviewed hashes:

| File | SHA256 |
| --- | --- |
| analysis.py | `5be616aeb03c11eb6def2fd44773ec9a2c6ae25d4fb8210b9df15afe4e98e088` |
| test_analysis.py | `ec7f5d3ee24cdb1b8254c9c2b82645061ae1fbe89090f19aa3852363a1b429ba` |
| summarize_analysis.py | `5ed528b567354a37bd4c3d72ca6e565e9ca1682f882714c73dc808213de07465` |
| after_qualification.py | `28647c05fb69d3994f1d3487c3b01d0c816aded402a49134fe1f5167f1cd533d` |

## Checks and resolved findings

Ran `python3 -m unittest discover -s docs/m5/evidence/msh161 -p test_analysis.py -v`:
**14 passed**, no engines/network. Independently extracted all12 completed
MSH-151 histories using the new parser and compared each with its prepared prefix
plus actual transcript bestmoves: all match. Canonical histories, rather than
transcript parsing, remain the analysis input; the cross-check is separate evidence.

Code review verifies full FEN/history and side perspective, exclusion of rejected
moves and unplayed intended claims, separation of prepared moves from competitors,
same-root unrestricted/forced searches, fresh process/cleared hash, fixed options,
nonbound score/PV/WDL alignment and independent legal-PV checking. Negative loss
is preserved as uncertainty. Mate scores remain distinct. Comparison now rejects
missing or unequal history/root/mover, nodes, executable and options.

Review findings were corrected before launch:

- Surviving process groups receive bounded termination; unresolved cleanup keeps
  the inflight marker and marks failure.
- Executable stat identity is rechecked before each query against the hash-verified
  initial binary, preventing ordinary mid-batch replacement from using a stale hash.
- Mismatched comparisons are refused rather than assigned numerical damage.
- Raw UCI truncation marks analysis uncertainty.
- The dependency chain now runs a separate two-query first-position smoke and
  checks complete, aligned nonbound WDL, legal PV, empty full starting history,
  requested work, positive bounded elapsed time and settled process state before
  all-move analysis. Its scripts/source/protocol hashes are frozen before waiting
  and rechecked after qualification.

The chain blocks on the shared OS lock rather than polling moves. It refuses an
incomplete native qualification batch, runs reference work only after competitive
work releases the lock, preserves failed states and stops on nonzero child exit.
Per-query analysis has a60-second response loop deadline plus bounded cleanup;
raw/score retention and command/line sizes are bounded. Fresh output directories,
manifest equality, flock and inflight refusal prevent silent resumption over
changed inputs or unfinished work. Actual child CPU/wall and failed-attempt
receipts are retained. Native replay is explicitly a coordinator prerequisite;
the qualification runner performs it before declaring pair completion.

The summary separates prepared/played moves and both competitors, preserves
query hashes/history references and labels threshold events as provisional.
Opponent-error/reply links are candidates only; it does not infer recovery,
conversion, themes, causal defeat or search/evaluation defects. Biggest raw loss
is explicitly not necessarily the largest consequential mistake. This restraint
is appropriate for independent batch analysis.

## Limits

Actual Stockfish transport and runtime timing are not established by these offline
tests; the automatic real smoke and preserved qualification analysis provide that
next evidence. The supervisor assumes the pinned local Stockfish UCI executable,
not an adversarial child. Abrupt host/process interruption requires explicit
inflight/PID reconciliation before resume. Reference WDL is a fitted Stockfish
self-play expected-score proxy, not actual opponent win probability. Controlled
higher-work alternatives, native continuation admission, uncertainty review and
where applicable history-aware tablebase interpretation remain necessary for
consequential classifications. Source balance screening does not itself freeze
the90-root selection or prove all planned structural coverage survived rejection.
No fresh Linux/Windows or packaged-GUI verification is claimed.
