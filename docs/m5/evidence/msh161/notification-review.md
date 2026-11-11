# Independent exit-notification review — MSH-161

Reviewer: execution_review; 2026-11-11. Scope: `notify_exit.py` and its fake-CLI
tests. No engines or real queue invocation ran, and no security settings changed.
The installed CLI's read-only `codex queue --help` confirms the exact supported
arguments: `queue --thread <THREAD> --message <TEXT>`. The helper invokes that argv
without a shell, using the caller-supplied installed executable and target chat.
It adds no sandbox bypass, approval override or scheduled watcher.

`python3 -m unittest discover -s docs/m5/evidence/msh161 -p test_notify_exit.py -v`
passed all3 tests: successful computation queues once, failed computation queues
once, and queue failure remains visible. Each also checks duplicate launch refusal
and that a second invocation does not rewrite the fake queue capture.

The exclusive receipt prevents automatic duplicate launches and notifications.
Recording `queue_attempted` before the bounded attempt is appropriately conservative:
a timeout or interrupted attempt may already have delivered its message, so no
automatic retry occurs. Actual computation and queue return codes remain distinct;
notification failure makes the wrapper exit nonzero even after successful work.
Logs and command/state references support inspection. These mechanisms are
appropriate for an exit-triggered one-shot validation of an already completed
batch. They do not claim that queue acceptance proves the later review completed.

## Signal-cleanup finding

Before using this wrapper for future live computation, record and verify cleanup
scope. The initial implementation signals only its own created child process
group and waits for the group leader. It neither checks surviving same-group
children nor controls descendants that existing helpers launch in separate
sessions. Therefore wrapper exit must not be presented as proof that every engine
has stopped. Add bounded same-group settlement/termination and a signal-path test
using a fake child; explicitly retain an unresolved detached-descendant caveat or
perform reconciliation through the underlying helpers' own inflight receipts.
Do not use broad unrelated-process termination to address this limitation.

This does not block the proposed one-shot validation command for a batch already
known complete. For future runs, normal successful helper completion retains its
existing bounded-engine cleanup evidence; interrupted runs require explicit
reconciliation. No fresh Windows/Linux or product GUI compatibility is claimed
for this local POSIX operator helper. The initial review finding remains open
until a corrective diff/test is assessed.

## Bounded cleanup revision and platform limitation

Reviewed revised helper SHA256
`3782fb9b36650ce8179b2e64544e13ef047c51667e2212b1a39c952976186ea4`
and the signal-test snapshot inspected before the final assertion adjustment.
The helper now attempts bounded same-group TERM/KILL settlement and records its
cleanup scope explicitly. Cleanup exceptions are retained with
`process_group_settled=false` and `cleanup_review_required`; they do not suppress
the one failure notification. Detached helper-owned engine sessions still require
their own receipts and reconciliation. Whole-descendant cleanup is not guaranteed.

The coordinator reports macOS `PermissionError` during group cleanup/probing even
under normal escalated local test execution. This is a real unverified-cleanup
outcome, not permission to alter security settings or report successful cleanup.
The three nonsignal fake-CLI tests were independently rerun and passed after this
revision. The current signal test still insists on `process_group_settled=true`
and is therefore **not recorded as passed** by this review.

Recommended signal-test contract: require recorded SIGTERM, failure status and
accepted failure notification; additionally require either verified settled group,
or explicit cleanup error with settled=false and review-required=true. This tests
honest handling of both permitted cleanup and OS refusal, without weakening the
reported guarantee. A deterministic denied-probe fixture can supplement the local
signal test. Do not infer that a denied group probe detected surviving processes:
record that detection as unknown (with its error), while retaining failure/review
status whenever settlement is unverified. Preserve the original failure evidence.


## Final resolution

Final helper SHA256:
`501b67b9fb2b7cfed459dd7cad99070ee43450c60f92f00ffa8684d4ce7574b3`.
Final test SHA256:
`2aa3715af8bd82f432e16dd2c36f0bf0f7149e10cf24190a804b62a287d395e6`.
The final diff records denied group detection as unknown/null and makes unverified
settlement a failure requiring review. The signal test now permits only settled
cleanup or explicit cleanup-error/review-required handling while still requiring
recorded signal and accepted failure notification. These changes resolve the
review's receipt-accuracy and signal-test findings.

Inspected the coordinator's preserved `notification-tests.log`: all4 tests passed,
including signal handling. Independently inspected
`notifications/qualification-extension-completed.json`: the one-shot validation
verified eight complete qualification pairs and complete all-move query coverage,
then exited0; the wrapper recorded success, its own process group settled, queue
return code0 and queue acceptance. This review made no real queue call. That
receipt establishes successful validation-exit-to-queue acceptance, not that the
subsequent model review has finished or that arbitrary detached descendants can
always be terminated.

**No notification-helper blocker remains.** The platform refusal limitation is
explicit and remains an operational recovery condition, never a successful
cleanup claim. Future interrupted computation must inspect underlying helper
inflight/cleanup evidence before resuming. No timed watcher or security-policy
change is introduced by this resolution.
