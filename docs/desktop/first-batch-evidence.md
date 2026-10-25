# M3 first desktop batch evidence

Scope: MSH-27–32 only. M2 stays closed. M3 remains open for MSH-33–39.
See the [M3 contract and area mapping](m3-design.md),
[internal engine interface](internal-engine-v1.md), and
[desktop launch instructions](../../desktop/README.md).

## Native session and engine boundary

Integrated source: `074fe515b9c9b9ab65dfc9a256dcbf0d07eb3132`, `codex/rewrite`.
[Native CI run 40196021766](https://github.com/jweathers777/mShogi/actions/runs/40196021766)
passed on Ubuntu 24.04 (job 121176901775), macOS 15 (121176901942), and Windows
2022 (121176901969): locked workspace format, Clippy with warnings denied,
tests, build and console smoke, using the unchanged Rust 1.89 toolchain.

New focused evidence: 11 desktop-session tests and 12 engine boundary/process
tests. Native session tests cover atomic rejection, move/claim application,
duplicate responses, cancellation, ABA/new-game protection, generation mismatch,
identical FEN with different known histories, invalid/incomplete engine results,
native promotion/checkmate and explicit Full restoration refusal. Protocol tests
cover strict/duplicate JSON, mode rejection before replay, actual subprocess
execution, bounded streams, failure/missing/malformed/timeout responses,
cancellation/replacement/shutdown and retirement of already published results.
Existing CLI/UCI and workspace regression tests also passed.

Independent reviewers: `/root/m3_contract_review` reviewed the contract,
foundation and controller, independently running all 11 controller tests;
`/root/desktop_foundation` reviewed the engine and independently ran all 12 new
engine tests. No actionable findings remained in these reviewed boundaries.
Process cleanup is scoped to the direct trusted bundled child, as documented
in the interface; arbitrary descendant containment is not claimed.

## Foundation and packaged application

The local arm64 macOS foundation built and launched a normal `.app`. Strict
TypeScript/Vite production build, desktop Rust 1.90 format/Clippy/test target
compilation passed. The external package verifier observed a real bundled-engine
UCI depth-one result: 21 nodes, `a2a3`, exit zero. Engine SHA-256 was
`d987f826ef87f7d004a1a9604f4b63e0bd8ac171fdff3fe862bb62a4e93352c0`.
This was an initial working-tree foundation build; the final source package
checks below supersede its binary identity. Package inventory contained only
the plist, host, engine and icon. The native screenshot is
[foundation-macos.png](evidence/foundation-macos.png).

[Foundation package CI 40196021909](https://github.com/jweathers777/mShogi/actions/runs/40196021909)
passed on the same integrated source, with exact retained observations:

| Native platform | Job | Package verification | Retained report |
| --- | --- | --- | --- |
| macOS 15 | 121176902044 | `.app` contents and actual bundled UCI search | [macOS](evidence/foundation-macos.json) |
| Ubuntu 24.04 | 121176902086 | Built `.deb`, extracted contents and actual bundled UCI search | [Linux](evidence/foundation-linux.json) |
| Windows 2022 | 121176902007 | Built and installed NSIS package, contents and actual bundled UCI search | [Windows](evidence/foundation-windows.json) |

Each depth-one search visited 21 nodes and chose `a2a3`. Package construction
and engine smoke are distinct from the full installed-app interaction
verification retained in MSH-38.
The documentation-only bookkeeping push `6f41f69` had redundant workflows
cancelled; production source was unchanged from the successful runs above.

## Playable board and local native integration

The normal macOS application was exercised through native UI automation, using
its actual bundled engine and no injected game state:

| Journey | Observed moves and native results | Screenshot |
| --- | --- | --- |
| Human White, click then keyboard | `e2e4 a7a5 g1f3 a5a4`; engine depth 2, 642 then 631 nodes, score 0 from Black's searched perspective | [White game](evidence/playable-white-macos.png) |
| Human Black, independent board flip | `a2a3 e7e5 a3a4`; engine depth 2, 442 then 610 nodes, score 0 from White's searched perspective | [Black game](evidence/playable-black-macos.png) |
| Default and compact window | All eight ranks and keyboard guidance visible at 1180×820 and 1180×650 | [Start](evidence/playable-start-macos.png), [compact](evidence/playable-compact-macos.png) |

The knight move `g1f3` used arrows and Enter entirely on the native application
board. Flipping preserved human color, move history and session version. The
history remains a read-only live action list; no earlier-position change is
offered. Thirteen frontend tests pass against native-generated JSON, including
all promotion choices, focus recovery, orientation, selection cancellation,
claims/conclusions, actual telemetry and out-of-order/stale-render protection.
Eleven native session tests and the new process-cleanup regression also pass.

Independent review found and resolved promotion focus loss, command version
binding and native shutdown issues. Actual controlled-child verification then
exposed the macOS predefined Quit bypass; the failed check and corrected
observations are preserved in [shutdown evidence](evidence/shutdown-macos.json)
and [reproduction instructions](shutdown-verification.md). The separate copy
used the byte-identical normal host with only its copied sidecar replaced.
Corrected actual Quit and window-close child removal preceded both watchdog and
natural completion. Normal packages contain no instrumentation.

The documented development command compiled/launched the native debug host,
served the actual Vite/React assets on port 1420 and stopped with exit zero.
CUA cannot bind this unbundled executable, so this is build/process/HTTP evidence,
not a claim of development GUI play. Normal packaged GUI play is verified above.
The final board source still requires native CI/integration evidence before
MSH-31 closure; MSH-32 will separately verify the complete minimal-play slice.

## Remaining scope

MSH-33 adds analysis and supported bounded/terminable Full execution; MSH-34
completes outcomes/recovery; MSH-35 adds persistence and historical viewing;
MSH-36 adds the labeled synthetic large-board/route fixture; MSH-37 verifies
integrated interaction/accessibility; MSH-38 verifies installed applications
across all three platforms; MSH-39 independently reviews M3 integration/closure.
No timed play or timed suspension policy is implemented in this batch.

The [unchanged M2 closure evidence](../review/m2-closure.md), rule fixtures,
search correctness checks and recorded independent UCI matches remain applicable.
This desktop batch does not claim stronger play, complete default dead-position
recognition, another implemented real game, or M3 milestone closure.
