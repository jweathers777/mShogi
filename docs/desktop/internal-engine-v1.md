# Bundled engine interface v1

`mshogi internal-search` accepts one UTF-8 JSON object followed by LF on stdin,
writes one JSON response followed by LF on stdout, and exits. Diagnostics use
stderr. `mshogi uci` remains the independent compatibility adapter. The internal
interface is for the trusted bundled executable, not arbitrary external engines.

`mshogi_engine::EngineRequest` defines the strict wire contract: `protocol: 1`,
nonzero `request_id`, `session_version`, nonzero `process_generation`, `game`
(the canonical GameRecord JSON object), and `limits: {depth, nodes}`. Nodes is a
required integer cap; depth is 1 through 128. Full adjudication is rejected
before live replay with `unsupported_mode`; the echoed record retains its exact
mode and limits. Duplicate/unknown fields and unsupported record schemas are
rejected by the strict native decoders. Invalid framing cannot safely echo an
identity, so the process reports a diagnostic and exits unsuccessfully.

Responses contain `context` (the entire original request) and tagged `result`:

| `kind` | Meaning |
| --- | --- |
| `move` | Complete native action plus telemetry |
| `claim` | Actor, named rule and optional unplayed intended action, plus telemetry |
| `finished` | Rules-owned already concluded chess outcome, plus telemetry |
| `incomplete` | No complete iteration, telemetry only; no fallback move |
| `failure` | Typed failure kind and diagnostic; no chess outcome or score |

Telemetry is final native output: optional score from the searched side's
perspective, completed depth, all attempted node visits, complete move/claim PV,
and `complete`, `node_limit` or `stopped` status. No periodic synthetic progress
or estimated depth/nodes is published. A work-limited result with a complete
iteration retains that real decision; zero completed iterations remain incomplete.

`EngineBridge` uses one supervisor thread, one replaceable pending request and
one completion slot. `submit`, `cancel`, `poll` and shutdown manipulate short
metadata sections; process launch, all pipe I/O, wait and cleanup occur off the
host interaction path. Request and response byte caps are 1 MiB and 2 MiB;
retained stderr is 4 KiB. Default execution/response watchdog is 15 seconds and
cleanup observation is 250 ms. Cancellation/new game discards even a result
already published in the completion slot. Replacement stops the old child
before launching the new one. Request IDs and generations must increase.

Failures distinguish invalid/unsupported requests, search failure, start/I/O,
unexpected exit, malformed/oversized/missing response, timeout, context mismatch
and cleanup uncertainty. The session validates the complete current envelope a
second time immediately before applying a choice through ChessMatch; invalid
choices pause play without changing the game or inventing a result.

These watchdogs depend on OS scheduling and are not hard realtime guarantees.
Cleanup supervises the direct bundled child. Pipe helpers still blocked after
the cleanup deadline can detach; arbitrary descendant process containment is
not promised. The bundled engine creates no child processes. Closing the host
signals supervisor shutdown without joining substantial work on the UI thread.

Fixtures and independent expected results are in
`crates/mshogi-engine/tests/protocol.rs` and
`crates/mshogi-console/tests/internal.rs`. They run actual native subprocesses,
including a separate test-only helper executable. Packaging builds and copies
only `--bin mshogi`; the helper is not a sidecar or application resource.
