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

Three-platform package CI and final playable-application evidence are pending.
Package construction/engine smoke is distinct from the full installed-app
interaction verification retained in MSH-38.

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
