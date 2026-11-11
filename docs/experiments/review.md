# Independent source and execution review

MSH-162 (Parallel Experiment Runner), 2026-11-11. Primary builder owns core/CLI,
packaging and integration. Separate agents authored only the Chess adapter and GCP
backend. They then independently reviewed code they did not author.

- Chess-adapter author reviewed core/CLI/GCP/packaging and independently ran the
  core/process/GCP tests. Found missing derived-confirmation coverage after an
  interruption, substituted dependency identities, worker signal initialization,
  notification exception accounting, and undispatched manifest jobs. All fixed;
  adversarial regressions cover the first four and schedule validation rejects
  missing roots. Real SIGTERM testing additionally found selectors swallowing
  InterruptedError; RuntimeError now reaches frozen cleanup and reaps the child.
- GCP-backend author reviewed the Chess adapter/core/CLI/operating instructions
  and independently ran the full suite. Confirmed frozen pair/query/selection
  behavior; found divergent submit/output stores, unbound image/machine identity,
  incomplete pipeline coverage, Linux cohort mislabeling, and notification IAM
  wording. All corrected. No further concrete source blocker found in
  prepare/package workflow.
- Final cross-review caught cancellation/deletion leaving a Pub/Sub listener
  waiting indefinitely. Cancelled/deleting states now fail with a retained
  authoritative receipt and explicit cleanup uncertainty. Added tests cover both
  initial lookup and event delivery. Controller dependency is separately pinned.

Review acceptance is limited to inspected source and actual local tests. No claim
of actual Google Cloud execution, Linux x86_64 engine build correctness, fair C3
competitive conditions, eight-VM throughput, billed cost, or successful live
notification delivery is made. Those are pilot gates, not inferred from mocks.
The reviewer did not authorize or integrate the coordinator branch.

## Setup and Linux packaging continuation

At source51351c48, a separate reviewer verified the novice guide and dedicated
network configuration. Corrected the required role to roles/batch.agentReporter
and included logging.googleapis.com. Bucket create-only permissions and conditional
claim deletion, worker Pub/Sub publishing, private registry reading and no-ingress
network commands were reviewed against official documentation. No cloud changes
were made. All50 tests passed on Mac, Linux ARM and the actual packaged Linux
x86_64 image (emulated locally); fixed native/opponent builds and native replay
also passed. These establish local packaging/portability, not cloud throughput.

## Controller project alias correction

At controller revision 348926274cb56cedcb6328bbfd809f552910ae63, a separate reviewer approved numeric project
alias matching after the first real Batch event exposed the mismatch. UID and
full region/job validation remain required, followed by an authoritative lookup.
The reviewer independently passed all28 GCP tests. The full controller suite
passed54 tests on macOS. Worker source51351c48 and its image stay frozen.
