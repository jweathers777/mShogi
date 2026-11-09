# Risk-based verification

Owner direction, 2026-11-09; implemented by MSH-146 (Risk-Based Verification).
This is the prospective repository standard for all six games. It supersedes
blanket per-task platform-matrix requirements, not rules, test assertions,
independent review or supported-platform product requirements. Completed task
records and their historical acceptance decisions remain unchanged.

## Choose scope from the actual risk

| Level | When it applies | Required evidence |
| --- | --- | --- |
| **1 — routine local** | Platform-independent search, classical evaluation, pruning, ordering, algorithmic optimization and isolated native correctness work | Focused local correctness and independent regressions, relevant profiling and independent review. Broaden local tests for shared native consumers. Record the actual host and source. |
| **2 — targeted platforms** | Material portability, process supervision, concurrency, filesystem/persistence, protocol, dependency/toolchain or platform-sensitive interface changes | Select affected operating systems and the smallest suites that exercise the risk. A subprocess change may require the focused runner suite on all three systems; an OS-specific fix may need one. Include actual native consumers where affected. |
| **3 — full integration/release** | Significant integration milestones, major architectural migrations, release candidates and releases | Comprehensive Linux, macOS and Windows native and desktop evidence, including actual packaged/native journeys appropriate to the checkpoint. Reuse still-valid evidence only where source and scope justify it. |

Production Rust changing is **not** sufficient reason to require Level 2 or 3.
An ordinary search/evaluation task can be Done and integrated after its applicable
Level 1 checks and independent review pass. Missing unrelated hosted-runner
capacity does not block that completion. Automatic lightweight CI is a backstop,
not a replacement for behavior-specific local tests or review.

Changing platform behavior requires appropriate actual platform evidence even if
most of the code is portable Rust. Choose the level based on the changed behavior
and dependencies, not the filename alone. Mixed changes take the union of their
risks; shared native serialization, process or desktop boundaries must not be
hidden by narrow path filters. Do not run unrelated packaging or GUI suites for a
focused process or persistence change.

No new owner-approval checkpoint follows from choosing a verification level.
Routine test selection is the implementer's responsibility. Escalate a material
product/semantic tradeoff or substantial resource requirement, not ordinary
engineering verification.

## Task and evidence conventions

Put a short verification statement in the active task's plan: selected level and
reason, local checks, any required platform jobs, and the later checkpoint for
broader compatibility evidence if it is deferred. Record actual commands,
environment, source revision, pass/fail/not-run results and independent review in
the existing task/evidence record. Do not create a second status system.

For example, a classical-evaluation change may record: “Level 1: portable scalar
search/evaluation only; local search/native fixtures, cancellation/restoration,
profile and independent review. Windows/Linux execution not run for this change;
full compatibility is deferred to the next significant integration or release
checkpoint.” This is sufficient to complete that engineering task, not evidence
that untested platforms passed.

For new acceptance criteria, require risk-appropriate verification rather than a
matrix by default. Preserve explicit Level 3 product/milestone requirements.
For an existing unfinished task, reconcile an obsolete blanket obligation in a
CLI note/plan before closing it; retain failed or unavailable attempts and explain
which requirement the new owner policy supersedes. Do not silently abandon an
active platform campaign. Genuine platform-risk obligations remain unfinished
until supported or explicitly resolved.

Conformance still follows [the independent conformance policy](conformance-policy.md).
Native rules/history remain authoritative, exact state restoration and cancellation
remain required, and independently established expectations must not be changed
to make implementations pass. Separate correctness, useful search depth, execution
speed, decision quality and measured playing strength. A faster search or a green
CI job does not establish stronger play.

## CI and manual checkpoints

The [development guide](../development.md#ci-and-evidence) documents the concrete
workflows and supported invocations. Ordinary push/PR automation is lightweight.
Targeted platform checks and full native/desktop campaigns are explicitly selected
when their verification level calls for them. Existing full-job names remain
available for historical receipts and status-check compatibility.

Path selection must handle changes spanning several areas and shared dependencies.
Prefer a consistently emitted lightweight check with conditional expensive steps
rather than a required workflow that never starts for an excluded path. A skipped
full campaign is **not run**, not a successful full-platform result. Do not label a
lightweight job with a full-platform job's name.

If branch protection later requires a particular check, select a reliably emitted
lightweight context for ordinary integration and use explicit checkpoint evidence
for full verification. Never bypass or silently alter protections. Inspect current
protection/ruleset settings before changing workflow names or triggers.

The original C++ `master` remains the default branch. Registered workflow IDs can
be dispatched against `codex/rewrite` through the supported GitHub API/CLI; the UI
button may depend on a default-branch workflow file. Do not change default branch
or original history merely to expose a button. Preserve the requested revision,
actual job scope and outcome in receipts.
