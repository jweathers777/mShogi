# M3 outcomes, analysis and persistence batch

Authorized order: MSH-34, MSH-33, MSH-35. M2 and MSH-27–32 stay closed/Done.
M3 remains open; MSH-36–39 are subsequent work. The eleven-area mapping and
owner clarifications are in [M3 design](m3-design.md). Unchanged M1/M2 and
[first-batch evidence](first-batch-evidence.md) remain applicable.

## MSH-34 — outcomes and recovery

A native serialized resignation command uses the existing M2 mating-feasibility
contract without an evaluation-based result. Missing proof can conclude play as
unresolved/unscored; a sound impossibility proof can establish a draw. Human
resignation is supported during either player's turn and retires pending engine
work. Version-bound confirmation cannot resign a newer position. Cancel only
applies to active work; Retry only applies to a paused engine turn. Neither
rewrites the authoritative match or game history.

Focused local arm64 macOS checks: 16 native all-feature tests; four public
controller/actual-process integration tests; warnings-denied native/host Clippy,
formatting and host tests; verification-feature host compilation; strict
TypeScript and 22 frontend interaction tests; normal and verification `.app`
builds. Native-generated frontend fixtures include current/intended claims,
claimed outcomes and unresolved resignation.

Actual macOS packaged journeys (not inferred from controller tests):

| Package and journey | Observed result | Screenshot |
| --- | --- | --- |
| Normal app: `e2e4 a7a5`, confirm resignation | Unresolved/unscored; same board and two-action history | [Resignation](evidence/recovery-resignation-macos.png) |
| Labeled verification: intended `a1b1` fifty-move claim | Claimed draw; no intention played, zero-action history | [Claim](evidence/recovery-intended-claim-macos.png) |
| Labeled verification: actual zero-node child, then Retry | Incomplete depth 0/no score; real retry `a2a3`, depth 2/442 nodes | [Incomplete](evidence/recovery-incomplete-macos.png), [retry](evidence/recovery-retry-macos.png) |
| Labeled verification: real missing-response helper after `e2e4`, restore actual engine, Retry | Ongoing paused game with one move; one real reply `a7a5`, depth 2/642 nodes | [Failure](evidence/recovery-failure-macos.png), [recovery](evidence/recovery-failure-retry-macos.png) |
| Labeled verification: external slow child after `g1f3`, Cancel | Three actions preserved, no conclusion; owned child removed | [Cancellation](evidence/recovery-cancelled-macos.png) |

The [retained report](evidence/recovery-local-macos.json) identifies the normal
host (`f5b149…`), unchanged actual engine (`1461e6…`), separate verification host
and observations. The normal package contains no fixture initializer; package
verification also runs actual UCI and structured searches. The feature build has
a separate name/identifier/title; only its sidecar was temporarily replaced for
failure/cancellation, then restored. Normal distributables have neither the
initializer nor the helper. [Reproduction/launch](../../desktop/README.md).

The [MSH-34 record](<../../backlog/tasks/msh-34 - Complete-desktop-outcomes-and-recovery-interactions.md>)
retains independent review, integrated source and exact native/platform CI.
This task adds no clocks or evaluation-based resignation/draw policy.
