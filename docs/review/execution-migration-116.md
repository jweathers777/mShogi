# MSH-116 / R1 — current execution and observation cutover

## Scope and delivered boundary

Primary builder/integrator: Codex root, primary `codex/rewrite` checkout, clean
startup `899ee343bdba981381649457bca483e14f5f9109`; origin fetched aligned0/0.
Backlog.md1.55.1 used after detecting installed1.55.0. MSH-115/MSH-122 Done.
No competing active task. Existing unrelated worktrees preserved; no new worktree.

One current [internal contract](../desktop/internal-execution-v4.md) now serves
actual Chess, Shogi and Xiangqi callers: protocol4 request/response,
`ExecutionContext`, native-tagged `ExecutionResult`, and durable schema1
`ExecutionObservation` without transport framing. Direct Chess dispatch consumes
the native canonical record; it does not construct an obsolete Chess request.

Actual consumers migrated: engine child/server and supervised bridge; desktop
live turn/read-only analysis acceptance; Shogi/Xiangqi native adapters; archive
observations/journal replay/Take back lineages and validators; native frontend
fixture generation/probes; Chess history/analysis presentation and technical
context types; real console subprocess fixtures/tests; installed-package engine
verifier and GUI archive-observation inspections. No native rules/search/runner
source changes, engine-strength change, product layout redesign or external
protocol grammar change.

Deleted `wire.rs` and its Legacy/Shared request/response families, version-header
legacy-shape fallback, V2 request/response/purpose/target exports,
`SharedRequest::chess_adapter`, shared-to-Chess transport construction, and paired
`submit_shared`/`poll_shared` methods. `EngineBridge` has one submit/poll path.
Useful native Chess result/claim/choice/failure meanings remain; the result type
is now explicitly `ChessResult`.

Saved `ArchivedObservation`/`PresentedObservation` contain semantic
`observation`, not a response wrapper. Complete native context, actual result,
original identities, purpose/selection/perspective/limits, variations and proof/
uncertainty survive. The Xiangqi payload identifies observation schema1 instead
of an engine wire protocol. Native in-process exact evidence remains separate.

## Native and persistence preservation

| Risk/requirement | Concrete evidence |
| --- | --- |
| Exact native identity/history, strict outer/nested parsing | Current contract tests reject old versions/flat shapes, unknown/missing/duplicate fields and native identity disagreement; raw canonical bytes still reach authoritative parsers. Same-board/different-history subprocess tests retained. |
| Read-only purpose and selected target | Separate live/analysis admission; exact context equality includes native event prefix, board count, native side, selection/session/request/process IDs and limits. Frontend guards migrated from Chess-only ply/color to complete native target fields; wrong event/board/native-side regressions withhold scores and suggestions. |
| Chess claims/Full | Native intended/current claims, concluded roots, incomplete iterations, M2 restoration policy, bounded cooperative Full witness/proof/Unknown tests and actual child queries pass. Full is not forced-win search or an implicit match result. |
| Shogi actions/declarations | Actual native move/drop/promotion, formal declaration, terminal commands and historical prefixes exercised through current child and archive replay; eligibility remains advisory. |
| Xiangqi semantics | Native history/warnings/claims/terminal events and uncertainty preserved; ordinary search remains sound-fast-only. Native exact positive witness/no-win/Unknown/cancel/failure evidence retains provenance and remains read-only on reopen. |
| Cancellation/replacement and retired identities | Real shared bridge tests replace work across games and historical selections; cancellation retires published completion. Archive reopen leaves work stopped, assigns fresh runtime identities and rejects old results. |
| Lossless saveability/lineage | Current-stage Chess/Shogi/Xiangqi Save/Open, abandoned Take back histories/observations, interrupted Save/Open, duplicate-preserving replay,256→257/control/event/byte/replay boundaries and stop/close headroom tests pass. No constants raised or content discarded. |

R1 requires no coordinated R2 implementation: durable schema1 is the intended
observation boundary that R2 can retain. Current-stage complete Save/Open works
without any old execution parser. Old development observation payloads reject
strictly; there is no compatibility promise. Existing outer desktop archive
branches remain **R2/MSH-117 ownership**, not a temporary execution adapter.
Native runtime/dormant Chess owner and current frontend projections remain
R3/R4 ownership. No later task is activated or automatically authorized here.

## Actual resource measurements

Unchanged caps: request1,048,576bytes, response2,097,152bytes, retained diagnostic
4,096bytes; archive16MiB,256 combined observations and existing native/control/
replay/headroom bounds. Measurements include the actual current wire envelope.

| Independently meaningful fixture | Encoded bytes |
| --- | --- |
| Shogi320-event record | native79,733; canonical envelope79,914; request80,074; response80,399 |
| Chess4096-event max-counter observation record | canonical envelope573,826; request573,986; response574,161 |
| Xiangqi512commands/1024nativeevents | native253,656; canonical envelope253,832; request253,992; response254,436 |
| Xiangqi256 ordinary observations | complete current-stage archive1,111,887; reviewer reopen322ms |
| Xiangqi1 ordinary +255 exact observations | archive754,812; reviewer reopen202ms |
| Xiangqi warning/audit record89nativeevents/67controls/32observations | archive834,832; root reopen323ms, reviewer333ms |
| Xiangqi accepted natural-count record103nativeevents/17controls/7observations | archive227,613; root reopen709ms |

These are measured fixtures on macOS, not universal maxima, performance promises
or Linux/Windows timings. Boundary tests additionally exercise actual byte and
replay refusal before commitment and retain a complete save path. Expensive
work remains off the interaction path and bounded/terminable as before.

## Verification and actual review method

Working-source checks before CI, macOS arm64, Rust/Cargo1.99.1, pinned
Node24.19.0, Python3.11.0:

- Engine worker: `cargo test --locked -p mshogi-engine`:39 pass; Clippy all targets
  with warnings denied passed. Independent reviewer separately ran36 engine
  integration checks (the remaining3 are crate unit checks).
- `cargo test --locked -p mshogi-desktop-session --features desktop-verification --lib`:
  **148 pass**,80.03s. Includes shared archive/controller, Chess analysis, capacity,
  strict observation parsing, fresh stopped reopen and retained Take back tests.
- Eight focused console suites (`internal`, `shared_internal`, `shogi_engine`,
  `xiangqi_internal`, `desktop_analysis`, `desktop_play`, `desktop_persistence`,
  `desktop_shared`): **30 pass**, using actual built child/fixture entry points.
- `cargo test --locked -p mshogi-console --test uci --test usi --test xiangqi_uci`:
  **29 pass**; external compatibility remains separate from internal wire.
- Full frontend:236 pass before five additional target/history regressions;
  focused `Analysis.test.tsx` + `History.test.tsx`:26 pass after those additions.
  TypeScript strict checking passes; native fixtures regenerated, including
  verification constructors, using the actual native generator.
- GUI verifier boundary discovery:41 Python tests pass. Package verifier's
  actual built-engine turn/read-only/Full queries pass; this local query check
  alone is not installed-package or GUI evidence.
- `cargo clippy --locked -p mshogi-engine -p mshogi-desktop-session -p mshogi-console
  --all-targets --all-features -- -D warnings`, formatting, diff check and Backlog
  doctor pass. Production negative-reference audit finds no obsolete transport
  identifiers in crates, frontend or verification scripts.

Separate bounded implementation workers owned engine, console tests and native
shared archive files, with root owning integration and Chess/frontend changes.
File ownership was explicitly handed off before embedded Chess tests migrated;
no worker integrated independently. Review was performed by a **fresh read-only
agent** `/root/r1_independent_review`, not by a builder rereading its own work.
It inspected the complete diff against899ee34, owner/task/design contracts,
strict parsers, result/context boundaries, current framing/admission, durable
observations, lineages, frontend guards and actual deletion endpoints. It
independently executed36 engine,20 Xiangqi archive,2 Shogi schema/raw parsing and
25 frontend checks: all passed. **No material findings remain.** CI and package
evidence require a separate current committed-source disposition below.

Development failures retained accurately: compiler errors while consumers were
mid-cutover were resolved. An old oversized-diagnostic regression expected an
archive pause; the new strict boundary rejects that malformed response earlier.
The corrected test proves all state/pending work remain unchanged and a valid
bounded response is still accepted. A real QA capacity-constructor probe exposed
implicit response/observation deserialization; explicit typed extraction and a
real255-observation constructor regression resolved it. Initial Clippy found an
unused test-only `.context` expression, corrected mechanically. An external-test
invocation used nonexistent target names and ran no tests; the corrected29-test
invocation passed. Reviewer default Homebrew Node failed to launch (missing
libsimdjson); pinned Node24 rerun passed. None is claimed as a product/rules fix.

Final fixture-only review separately compared all255 regenerated capacity
observations against the prior fixture: native payload/result, labels, ordering,
perspective and historical metadata are preserved; only current context/schema
encoding changes. Seven feature-generated Xiangqi snapshots add existing native
Take back capability. The reviewer independently ran9 History tests including
the capacity regression: pass. No findings. No historical evidence was rewritten.

## Current-source CI and acceptance

Pending the reviewed source commit's normal CI. No new packaged GUI or
cross-platform success is claimed yet. MSH-122's existing baseline remains valid
historical evidence, not verification of this changed execution boundary.
MSH-111 stays Done; MSH-112's separate runner race is not changed or masked.

MSH-116 remains In Review until applicable CI, current-stage package checks,
independent evidence reconciliation and normal integration/push are recorded.
MSH-117 is the next task under the existing graph once116 is Done; it is unstarted.
