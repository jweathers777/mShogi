# MSH-115: pre-release target architecture reassessment

Reviewed source: **`82bec546991acd552824a1c2e4a554541dc03b15`**, main
`codex/rewrite`, 2026-10-30. Diagnostic proposal only. No production refactor,
implementation task, new variant, P9 investigation or M5 experiment is started.
MSH-106 remains Done; its [historical report](architecture-health-106.md) and
verification artifacts are unchanged.

## 1. Corrected premise and recommendation

The owner's accepted [pre-release policy, A-09/D-17](../design.md) changes the
optimization target. There are no external consumers of internal interfaces,
and no requirement to reopen development-era saves. Old API names, wrappers,
archive versions and tests that assert them have no independent entitlement to
survive. Preserve independently expected **behavior**, not obsolete wire bytes.
External UCI/USI/Xiangqi interoperability remains required.

**Recommend coherent boundary replacement, delivered in bounded stages**, rather
than preserving every development layer through permanent adapters:

- One native application controller, explicit common runtime/document metadata
  and one active **native-tagged document**, with no dormant Chess session serving
  as metadata for other games.
- One current internal execution request/response family, one frontend session
  owner and one host command/snapshot/file/application-close surface.
- One supported **current desktop document envelope**, with game-native payloads,
  journals, settings, outcomes and proof observations. Remove old development
  format readers; do not flatten native semantics into one generic record.
- One native-choice interaction controller with specialized Chess/Shogi/Xiangqi
  renderers. Preserve ordered routes, completion choices and native authority.

This is a recommended target for owner review, **not an approved implementation
sequence**. Routine naming, Rust enum layout, IPC spelling and new format number
are implementation choices after authorization. Neither this recommendation nor
recording the already approved compatibility policy creates refactor tasks or
a new variant/M4/M5 dependency.

### Alternatives compared

| Alternative | Benefits | Costs and risks | Verdict |
| --- | --- | --- | --- |
| Incremental extraction with all existing formats/hooks/wire families retained | Small initial diffs and easy old/new comparison | Leaves dual session ownership, nested Chess wrappers, dormant state and compatibility test matrix; repeated adapters become permanent without a deletion endpoint | Useful temporary landing technique; withdraw as preferred final target |
| Coherent common lifecycle/current envelope plus native variants, staged replacement | Clear owners, smaller supported surface, one place for runtime/install/admission facts; removes development-only constraints | More deliberate contract/test migration; native histories and expensive replay must remain distinct; staged cutovers need explicit retirement/deletion criteria | **Recommended** |
| One current format per game with common file/runtime operations | Avoids forcing dissimilar native payloads into a common shape; still deletes historical readers | Repeats outer identity/dispatch/version bookkeeping already shared by all three; may be worthwhile if common envelope gains game-specific flags | Viable alternative; same semantic contracts, higher outer-format count, not forbidden merely for asymmetry |
| Universal rules/state/action/search/event framework | Superficial uniformity | Forces Chess claims/clocks, Shogi declarations/hands and Xiangqi warnings/exact goals through misleading common semantics; speculative future variant design | Reject |
| One large replacement of frontend, engine, runtime and every codec at once | No temporary adapters | Excessive simultaneous verification/debugging scope and difficult independent review | Reject; coherent target does not require a big-bang implementation |

## 2. What changes in MSH-106

| MSH-106 conclusion | Revised disposition | Reason |
| --- | --- | --- |
| F01/P1 narrow shared interaction lifetime | **Retain and strengthen** | Replace duplicated state implementations, rather than add a helper beside them. Preserve specialized geometry/artwork and complete native route semantics. |
| F02/P2 single product session facade while preserving legacy clients | **Modify: delete legacy production path** | No external consumers justify retaining old hook/DTO/IPC. Migrate required Chess functionality first, then remove its compatibility envelope and duplicate owner. |
| F03/P3 runtime/install seam; do not merge formats | **Modify; withdraw blanket format-retention advice** | Separate native payload semantics from development schema evolution. Replace common runtime ownership and choose a new current outer format without old readers. |
| F04/P4 structured capacity reasons subject to public compatibility | **Modify** | Internal DTO/error changes are permitted. Preserve exact admission and truthful presentation, not current text parsing or a development snapshot shape. |
| F05/P5 private protocol mailbox sharing | **Retain as orthogonal** | UCI/USI grammar and native results still differ. It is not a prerequisite for deleting internal execution V2. |
| F06/P6 actual later compound-action handoff | **Retain** | Removal of compatibility does not establish a real Chu/Tenjiku rules/action contract. Native complete actions/effects still need sourced fixtures. |
| F07/P7 measurement before optimization | **Retain** | Cleaner architecture is not proof of lower node cost or stronger play. No reason to discard validation or raise limits without evidence. |
| F08/P8 native types/dispatch/renderers appropriate; comments stale | **Retain** | Semantic differences survive the policy correction. Old temporal comments can be updated in a later scoped change. |
| Chess v1/v3 versus tagged v2 are necessary compatibility | **Withdraw compatibility obligation** | They contain necessary semantic facts; their version/readers do not have to survive. In `persistence.rs`, v1/v3 switches solely on whether Take back evidence is empty. |
| P9 Linux selector failure | **Unchanged and separate** | Cause was not established. No cleanup proposal may assume a cause, weaken checks or fold driver triage into a refactor. |

The old report was accurate about **what the existing formats contain**. The
incorrect premise was treating development compatibility as a reason to keep
their representation and readers indefinitely. Its historical source, tests,
package evidence and independent reviews remain evidence of prior behavior.

## 3. Current source and target ownership

The MSH-106 responsibility map remains applicable: production source did not
change between its reviewed2f17510 and current82bec54. This reassessment reread
actual source at the boundaries below; it does not rely solely on that map.

### 3.1 One frontend and host application owner

Current [`ProductApp`](../../desktop/src/product/ProductApp.tsx) mounts
`useGameSession`, then renders Chess `App`, which unconditionally mounts
[`useNativeSession`](../../desktop/src/useNativeSession.ts). Two polling/listener
and busy/error lifetimes remain, with document guards in both ProductApp and
`Document.tsx`. Current shared Chess presentation embeds the old `Snapshot`:
[`shared.rs::chess_snapshot`](../../crates/mshogi-desktop-session/src/shared.rs)
sets `chess: Some(snapshot)` while common analysis/observation fields are empty.
[`product/native.ts`](../../desktop/src/product/native.ts) exposes no complete
Chess claim/preferences/Full operation family, although Rust can nest an old
`DesktopCommand` in `SessionOperation::Chess`.

**Target:** one product `useGameSession`-style owner of subscription, command
serialization, latest-snapshot acceptance, file receipts, dirty replacement and
native close state. A controlled `ChessView` consumes native Chess-specific
presentation through the same owner as Shogi/Xiangqi. Common chrome and
lifecycle fields are shared; command, analysis and rule-advice payloads remain
native-tagged. Do not turn the current Shogi-oriented `NativeAnalysis` into a
universal Chess Full/claim presentation by adding guessed booleans.

The surviving internal API need not keep current names or nested shapes.
Tauri currently registers duplicated `desktop_snapshot/command/save/save_as/open`
alongside `game_*` in [`main.rs`](../../desktop/src-tauri/src/main.rs).
Remove the obsolete Chess endpoints after migration. **Do not delete common
native Quit/window-close behavior** merely because its functions are currently
named `desktop_request_close/cancel_close/finish_close`. Move that behavior to
the single application lifecycle surface, preserving host reservations, native
menu/window events, Tao termination handling and exact close approval.

#### Chess deletion prerequisites: real functionality, not old API coverage

A replacement must explicitly carry:

- Current and intended-move claims, actor/reason/intended action, opportunity
  versus concluded result; intended action is not played by a claim.
- Bounded read-only move analysis and distinct controlled Full cooperative
  mating analysis, target, perspective, limits, proof/Unknown/cancel/failure.
- Complete match history, read-only selected prefix, saved observation targets,
  explicit Return to live, stopped reopen and Resume.
- Orientation, analysis defaults, actual engine limits/telemetry, retry,
  resignation including unresolved/unscored outcomes, native Take back.
- Dirty replacement and close guards, failed/cancelled/stale Save, native
  quit/window-close, keyboard/focus recovery and snapshot identity protection.

These behaviors currently live in `App.tsx`, `Analysis.tsx`, `History.tsx`,
`Recovery.tsx`, `Document.tsx`, native `DesktopSession` and analysis/persistence
code. Delete old **ownership/wrappers**, not these functions. A clean-looking
shared snapshot with no Chess observations is not a completed migration.

### 3.2 Shared interaction, specialized rendering

[`Board.tsx`](../../desktop/src/Board.tsx),
[`ShogiBoard.tsx`](../../desktop/src/product/ShogiBoard.tsx) and
[`XiangqiBoard.tsx`](../../desktop/src/product/XiangqiBoard.tsx) separately
implement selection, candidate filtering, inspection retirement, pending-query
counters, capture precedence, keyboard activation/Escape and completion.
`useMovementInspection` already protects exact query context and non-actionability.

**Target:** a small reusable interaction owner consumes opaque native origin/
action IDs, ordered steps, completion alternatives and a context/capability key.
It owns selected origin, route prefix, terminal-versus-continuation decisions,
completion choice, cancellation and inspection lifetime. Renderers supply visual
focus ordering/neighbors, origin kinds, focus targets and presentation callbacks.
Keep square artwork, intersection/palace/river geometry, Shogi hands/kanji/
rotated ownership and promotion UI appropriate to their native content.

Replace the three copies; do not preserve a second compatibility controller
for the synthetic fixture. Keep the fixture as a real consumer of complete
selection paths. Never base the controller on Xiangqi's current `steps[0]`
shortcut or require endpoint equality to identify an action. Remove the generic
`>9` board scrolling heuristic during its renderer migration; development
fixture behavior is not a product permission to require scrolling/zooming.
Preserve the common inspection wording, accessible read-only announcements,
non-actionable markers and full-board product sizing requirements.

### 3.3 Native runtime/document, admission and installation

Current [`DesktopController/DesktopSession`](../../crates/mshogi-desktop-session/src/lib.rs)
always retains a Chess session. `SharedState` overlays optional Shogi/Xiangqi;
shared/native archive integrations mutate the Chess host's version, selection,
document, archive, capacity and execution fields. Installation and retirement
are repeated in `persistence.rs`, `shared_archive.rs` and
`shared_xiangqi_archive.rs`. The underlying native game/record types remain sound.

**Target responsibility split:**

| Owner | Responsibilities | Explicit exclusions |
| --- | --- | --- |
| Application controller | Serialized command admission/commit, active document, replacement/close decisions, exact native result application | No frontend rules; no mutation from read-only analysis |
| Runtime metadata | Document/selection/session revisions, monotonic execution/request/process identities, operation reservations/receipts, file status | Not a dormant Chess game or replayable pending work |
| Active native document enum | Exactly one Chess/Shogi/Xiangqi variant containing its native game/match, journal, native settings and observations/lineage | No common royal/check/claim/declaration semantics |
| Bounded execution/admission/file services | Native typed candidate/validated completion, actual encoding/work budgets, cancellation/watchdogs and atomic file replacement | No universal replay algorithm or one guessed per-event cost |
| Install transaction | Check receipt/current authority, retire execution, allocate fresh runtime identity, install validated document stopped, mark saved observations historical | No replaying exact graphs or interpreting duplicated conclusions as authority |

A typed `ActiveDocument` enum plus small common metadata/services is sufficient.
A public plugin/trait framework is unnecessary. Native adapters own replay,
complete commands, legal choices, claims/declarations, conclusions, decision
prefixes and game-specific costs. Keep expensive replay/validation outside the
serialized UI/controller lock. Cancellation, failed validation and stale I/O
must leave the accepted saveable document intact. Install only validated typed
candidates, never a parsed arbitrary map of a supposedly common game.

### 3.4 Current desktop document format

[`persistence.rs::Archive`](../../crates/mshogi-desktop-session/src/persistence.rs)
currently writes v1 or v3 depending on `takebacks.is_empty()`.
Shogi/Xiangqi use [`ArchiveV2`](../../crates/mshogi-desktop-session/src/shared_archive.rs)
with the same outer identity fields but different native payloads. These outer
version distinctions are development evolution. Necessary native differences
are a separate concern.

**Recommended target:** one new, explicitly identified supported current desktop
envelope with game/profile/edition/native-schema identity and a tagged native
document payload, common document settings/control metadata where semantics
match, historical observations and retained lineage. Preserve native record
representations where they remain appropriate; replace codecs/readers whose
only role is understanding old desktop shapes. Unknown/old formats reject
clearly and atomically. No old-file importer, dual writer, rolling-upgrade
negotiation or permanent archive migration chain is required.

Common outer structure does **not** mean a universal native journal. Preserve:

| Native facts | Target validation/retention obligation |
| --- | --- |
| Chess full match commands, timestamps/policies/clock context and conclusions | Use authoritative `ChessMatch`/`MatchRecord` replay; `GameRecord` history alone cannot replace a complete saved match |
| Chess abandoned Take back match/observation lineage | Retain the semantic content of `TakeBackEvidence` in the current lineage representation; optional empty lineage does not require a second format version |
| Shogi move/drop/promotion, eligibility versus formal declaration and terminal commands | Replay native commands and compare native derived events; no Chess claim substitution |
| Xiangqi warnings/obligations, claimant state, offers/conclusions and exact event order | Preserve same-board/different-history and intra-command inspection versus committed prefix distinctions |
| Ordinary and exact historical observations | Preserve native target/history, purpose, original IDs, modes, limits, actual result/work/proof/witness/failure; reopen does not rerun work or convert proof evidence into a live outcome |
| Desktop preferences, accepted controls and branch context | Preserve documented semantics and ordering; avoid duplicate authoritative state where derivation suffices, but verify any redundant saved conclusions by replay |

One current-format writer/parser must implement lossless roundtrip and strict
raw-preserving dispatch. Game-native bytes are not parsed through a map that
forgets duplicate keys. A change in development encoding does not change the
selected rules profile or turn Chess M2/Full modes, current exact proof families
or known-history provenance into disposable legacy data.

Retain admission **before acceptance** for actual encoded bytes, native events,
controls, combined ordinary/exact observations, replay work and close/stop/
identity headroom. Charge the actual new envelope, retained lineage and worst-
width future runtime IDs, not old-schema byte estimates. Re-measure realistic
long records and exact boundaries; do not merely raise constants or truncate.
Current256-to257 semantics and complete Save/Save As availability remain required
unless a separately approved product policy changes them. Failed/interrupted
Save preserves disk and dirty state; corrupt/unsupported/resource-limited Open
preserves the current live document. Current-format reopen is fresh/stopped;
old development files can be explicitly rejected without a converter.

### 3.5 One internal execution and IPC contract

Current [`wire.rs`](../../crates/mshogi-engine/src/wire.rs) has Legacy/Shared
request/response families and unknown-version legacy-shape fallback.
[`bridge.rs`](../../crates/mshogi-engine/src/bridge.rs) exposes paired
submit/poll methods. `SharedRequest::chess_adapter` converts shared Chess into
V2, and `server.rs::execute_shared` calls the old execution entry point.

**Target:** one current typed request/response family for all three games,
explicit purpose/context, native-tagged input/results and one submit/poll path.
Call the Chess execution helper directly with the appropriate native context
and purpose instead of constructing an obsolete V2 transport wrapper. Native
Chess result values may remain useful domain types; delete their legacy outer
transport, not move/claim/finished/incomplete/failure/Full distinctions.
Unknown transport versions reject; no old-shape fallback is necessary.

The complete acceptance key still includes game/profile/schema, full known
history, purpose, exact selected event/position, request/session/process IDs and
work limits. Semantic undo/history selection/reopen cannot revive work. Preserve
bounded queues, real encoded request/response/diagnostic caps, watchdog/cleanup
and explicit unscored uncertainty. Internal simplification does not change UCI,
USI or audited external Xiangqi command/coordinate/result conventions.

A single host IPC family likewise carries tagged native commands and snapshot
payloads, rather than embedding a Chess snapshot plus nested legacy operation.
Current TypeScript types are incomplete for this target and must be replaced,
not treated as the already finalized universal contract. Internal DTO/category
names may change freely; required meanings may not.

## 4. Retain / migrate / replace / delete inventory

“Delete” means after its functional consumers and tests have moved in the
approved implementation sequence. No item is deleted by this reassessment.

| Existing component | Disposition and required endpoint |
| --- | --- |
| Native Chess/Shogi/Xiangqi rules/actions/history/strict replay/perft/exact algorithms | **Retain** semantic authorities and independently expected corpora; no uniform board/rules/search model |
| `ApplicationShell`, game artwork/CSS, inspection wording and native read-only query | **Retain** accepted shell and interaction conventions; update contract imports only as necessary |
| `ProductApp` and `useGameSession` | **Replace/complete ownership**, retaining useful guard/freshness logic; one product subscriber and operation owner |
| `App.tsx` legacy orchestration, `useNativeSession`, duplicated `Document` guard and fallback props | **Migrate behavior, delete obsolete owners**; a new controlled Chess view/panels need not retain those component names |
| `Analysis`, `History`, `Recovery`, Chess claim/preferences/Take back controls | **Migrate/retain behavior** in native Chess presentation; never delete solely because reached through legacy types |
| `desktop/src/native.ts` versus `product/native.ts`, embedded `chess: Snapshot`, nested `SessionOperation::Chess{DesktopCommand}` | **Replace** with current tagged contract; delete redundant compatibility DTOs/wrappers and empty placeholder projections |
| Three board controller copies | **Replace** with one interaction owner; keep specialized renderers and synthetic route consumer |
| Chess-specific `desktop_snapshot/command/save/save_as/open` and redundant wait/file helpers | **Delete** after all product/harness/example/test callers use current host contract |
| `desktop_*close` helpers and native menu/window/Tao handling | **Migrate common behavior**, not indiscriminate deletion by prefix |
| Ever-present Chess `DesktopSession` plus optional native sessions as global owner | **Replace** with runtime metadata and one active native document; move actual Chess state/behavior into its variant |
| Old `Archive`, `ArchiveV2`, `decode_chess_v1`, `decode_chess_compatible`, version-specific read/write branches | **Delete/replace codecs** with one current envelope/native adapters; retain replay, complete history/observations/lineage and resource costing |
| File worker, atomic writer, admission receipts/headroom and native validators | **Retain useful mechanics; replace duplicated orchestration** without weakening invariants |
| `WireRequest::Legacy`, `WireResponse::Legacy`, legacy fallback, `chess_adapter`, paired bridge methods | **Delete** after current consumers move to single internal contract |
| Native game result/analysis types, explicit profile dispatch | **Retain semantics**, rename/restructure only as useful; type names are not compatibility obligations |
| Fixtures, verification-only harnesses and internal examples | **Migrate to current contract**, delete legacy invocation paths; instrumentation remains outside normal packages |
| Historical Git/tasks/reports/screenshots/logs | **Preserve unchanged**; not executable compatibility constraints |

## 5. Tests: preserve the oracle, replace the obsolete boundary

| Current test category/examples | Target treatment |
| --- | --- |
| Native independent rules/conformance fixtures, complete legal choices, source-derived histories and exact proof/witness cases | Retain independently expected facts. Do not regenerate expectations from the replacement implementation. |
| `App` intended claims, `Analysis` Full/Unknown, `History` prefix/provenance, `Recovery` unresolved outcomes | Port these behavior cases to the controlled Chess variant/current API before deleting old orchestration tests. |
| `Document.test` failed/cancelled/stale Save and native quit; `useNativeSession.test` stale snapshots/close/listener cleanup | Port to the surviving owner; assertions about old hook names or `desktop_*` invokes can disappear. |
| `ProductApp.test` mocking both hooks; dual legacy/shared poll-family isolation tests | Replace with one-owner/no-legacy-route assertions and cross-game replacement/result-retirement behavior. Do not preserve the unwanted architecture because a test mocks it. |
| `BoardRoutes`, board promotion/drop/keyboard/inspection tests, `useMovementInspection` | Retain semantic cases through shared controller and specialized renderers, including equal-endpoint distinct paths, terminal prefixes and non-actionable historical inspection. |
| `v1_chess_and_explicit_tagged_chess_preserve_existing_semantics`, schema1/3 literals and dual readers | Delete obsolete-format acceptance obligations. Re-express retained claims/history/lineage/Full semantics in new current-format fixtures; test explicit old/unknown format rejection. |
| Strict duplicate/unknown/conflicting/missing identity fields and native replay authority | Retain against current envelope/IPC; changed field names do not remove malformed-input obligations. |
| 256→257, mixed exact/ordinary, control-only, byte/event/replay/stop reserve, interrupted/stale Save/Open and retained branches | Retain/port with newly measured encoded-size/work expectations. No truncation, hidden semantic downgrade or waived headroom. |
| Actual internal subprocess and UCI/USI/match tests | Migrate internal request shape; retain native command/result, context, cancellation and cleanup cases. External standard behavior stays unchanged. |
| Packaged native quit/window-close/Tao, focus/layout and normal instrumentation exclusion | Targeted actual packaged verification after corresponding migration; old recorded evidence remains historical, not proof of newly changed code. |

Differential verification should compare **native semantic transcripts**, not
old serialized wrappers: root/provenance, complete actions/commands, exact event
prefixes, warnings/claims/conclusions, restored decisions/lineages, settings,
observation target/results, and retired-work rejection. Capture independently
expected fixtures and source-pinned current behavior as separate oracles.
Temporary old/new execution in a verification-only harness can help compare;
it need not remain in product code or support old files after cutover.

Do not require every task to repeat the complete rules/reference/three-platform
GUI campaign. Run focused current-boundary and changed-consumer checks; actual
packaged Chess/Shogi/Xiangqi lifecycle/interaction verification belongs at the
integrated cutover, with platform-specific results accurately distinguished.
An unchanged old CI success is not validation of a new implementation.

## 6. Proposed bounded sequence (not Backlog implementation tasks)

Owner review/selection of the target precedes **all** task creation below.
Letters describe outcomes/dependencies, not a prescribed detailed coding plan.
One integrator reconciles overlapping runtime/contract/frontend changes.

| Proposed unit | Bounded outcome and deletion criterion | Prerequisites / timing |
| --- | --- | --- |
| R1 Current execution contract | One typed internal family; migrate actual Chess/Shogi/Xiangqi consumers and observations/fixtures; remove V2/fallback/chess_adapter/dual bridge path; preserve Full/native result/context/encoded caps and actual child tests | Target approved; before runtime/frontend final cutover; independent of external protocol grammar cleanup |
| R2 Current desktop codec and semantic corpus | New current outer envelope, native payload validators and complete lineage/observation semantics; writer/Open use current format; retire old desktop readers/version switches; remeasure admission bytes/work/headroom and atomic failure cases | Target approved; may use existing runtime privately while R3 lands; after R1 if saved observation transport changes |
| R3 Native application/runtime owner and current host contract | Exactly one active native document plus common metadata/services; one installation/admission/file lifecycle, structured resource facts, current tagged command/snapshot payloads covering complete Chess behavior; remove dormant global Chess metadata owner | R1/R2 contracts stable; then one migration branch/integrator; before another desktop adapter |
| R4 Controlled Chess and single frontend session | All three games use one product owner/current IPC; migrate full Chess feature matrix and close/document guards; delete App/hook/legacy DTO/endpoint consumers and temporary adapters; preserve shell and game-specific panels | R3; before another desktop adapter; cannot claim complete while placeholders/legacy hooks still carry Chess behavior |
| R5 Shared board interaction | Replace all duplicated selection/inspection controllers; native complete paths/completions, specialized renderers, synthetic route fixture and keyboard/focus behavior; remove old controller/scroll heuristic copies | Target approved; extraction can develop separately from R1/R2, integrate against R3/R4 contract with one frontend owner |
| R6 Integrated deletion and current-product acceptance | No obsolete production paths/imports/feature flags, old readers or mixed wire family; all semantic regression cases ported; current-format complete-save/cancel/reopen/Take back/native quit verified in actual packages with explicit platform evidence | R1–R5; focused cross-game/native/frontend/host/process checks plus applicable CI and packaged cutover journeys |

### Staging contracts and deletion checkpoints

R1 includes **persisted observation structures and their validators/callers**,
not merely the child pipe. Prefer a durable native observation payload containing
full semantic target/request/result/provenance as data, independently of an old
transport wrapper. R2 can consume that current payload without retaining a V2
execution parser. Verify current-stage Save/Open after R1. If splitting the
observation/codec transition creates two throwaway schemas or incomplete saves,
land the adjacent R1/R2 cutover on one integration branch with separately
reviewed subunits; do not pay a permanent compatibility cost to force an
arbitrary staging order. Refining that boundary is routine implementation
planning, not an instruction to rewrite all subsystems in one PR.

R2's permissible temporary bridge has a concrete direction: existing native
runtime state → new current writer; new parser's validated native document →
existing runtime installation adapter. It reads **only the new format** and
owns no separate live state. R3 deletes that adapter when installation and
metadata move to the new active-document owner.

R3 may need a thin current-controller → old frontend-shape/command forwarder
while R4 migrates the remaining in-repository Chess caller. It cannot own a
second session or serve arbitrary old clients. R4 deletes the forwarder, old
host endpoints, hook and embedded snapshot together. This is finite cutover
scaffolding, not backward-compatibility support.

Each of R1–R5 retires the obsolete implementation it replaces in that stage;
only the two named cross-stage bridges can remain until their stated deletion
endpoint. R6 audits that all deletion obligations were met; it is not permission
to leave old readers/hooks/controllers active until a final cleanup. Tests and
normal/verification startup must exercise the surviving current path at each
integration. A migration is incomplete while behavior still secretly depends
on an unlisted old path.

R2/R3 may need further splitting by verified installation boundary if too large;
do not combine all six into one PR. Each landing must retain working game
functionality and a complete current save path; a staged private conversion is
permitted only to bridge a named incomplete migration, with an explicit later
deletion item. It is not a supported compatibility layer or reason to keep old
interfaces forever. Do not advertise the target as delivered while it remains.
A final negative-reference audit covers normal startup, host handlers, crate
exports, examples, fixtures and normal/verification package builders—not only
React imports. Preserve historical evidence files containing old names.

**Costs/risks:** R1 is a bounded transport/consumer migration; R2/R3 have the
highest semantic verification burden because they touch replay/admission and
identity installation; R4 must port several real Chess presentation features;
R5 must preserve action completion rather than merely pointer selection.
Deletion removes future maintenance surfaces but is not a promised speedup.
Benchmark separately before caching, trusted replay shortcuts, persistent
children, transposition tables or a generic search kernel. M5 ownership/gates
remain unchanged.

Recommended timing is R3/R4/R5 completion before another desktop game copies
the current paths. This creates no artificial dependency on unrelated Tori/
Chu/Tenjiku source research or native rules. The next real variant still owns
its sourced profile, native complete actions/effects and independently expected
fixtures; the synthetic interaction contract is not its rules implementation.

## 7. Decisions, evidence and review

### Owner decisions versus routine choices

Already approved: abandoning development-era internal compatibility. Do not ask
again merely because an API/format changes. Proposed for owner review: select
the coherent target and staging/priorities, including single current desktop
envelope versus separate current per-game envelopes. This report recommends
the former while retaining native payload variants. No implementation tasks
are created until that review, as expressly requested.

Routine after authorization: enum/trait-private helper shape, hook/file names,
new format/version number, neutral IPC names and bounded typed resource fields.
Those choices do not independently require another approval. Material changes
to native rules/profile, current functionality, losslessness/lineage retention,
restoration capability or external interoperability require the established
owner decision; none is proposed here. There is no unresolved rules decision
preventing this reassessment from completing.

### Actual method and limits

Primary root source inspection plus bounded read-only research sessions
`/root/reassess115_ui` and `/root/reassess115_archive`, at the exact revision
above. They inspected actual imports, callers, host handlers, native snapshot/
operation projections, archive structures/replay/install/capacity and internal
wire/server/bridge families. Both support coherent staged replacement with
native semantic variants and complete feature migration before deletion.

No tests/builds/GUI/benchmark campaigns were executed. Unchanged MSH-106 source
map, independent delivery assessments and qualified test/package evidence are
reused for context, not as proof of unimplemented target behavior. Its P9 Linux
GUI missing-e2 selector failure remains a separate untriaged observation;
MSH-111 is Done with its own retained history and MSH-112 remains separate.
No new CI rerun or cause attribution is made for this documentation-only review.

### Independent critique and disposition

Fresh read-only reviewer `/root/reassess115_critique` read the owner request,
AGENTS/design/roadmap, new A-09/D-17 and task/report, then inspected actual
ProductApp/App/hooks/boards, host IPC/close handling, engine wire/server/bridge,
native session/archive/admission, Take back/exact-observation structures and
relevant test boundaries. It explicitly challenged unnecessary compatibility,
overgeneralization, missing migration/deletion, hidden semantic changes,
shared ownership gaps and implementation/verification cost.

**Approve; no material findings.** It supports coherent staged replacement,
retained native semantic variants and the viable separate-current-envelope
alternative. Two nonblocking staging findings were resolved above: name the
codec/runtime bridge direction and deletion endpoint, and make R1–R5 delete
their obsolete paths rather than postpone deletion to R6. R1 also explicitly
includes saved observation structures and validators. No tests, builds, GUI,
benchmarks or production edits were made by the reviewer. There is no material
review disagreement; owner selection/prioritization of the target remains open.

Final static/task/preservation checks are recorded before completion. MSH-106
and its evidence stay historical, with this report superseding only the
compatibility-based recommendations for future work.

### Final static and preservation checks

Primary integrator used pinned Backlog1.55.1 after detecting installed1.55.0;
read overview, creation, execution and finalization guides. Clean source baseline
and fetched origin matched82bec54. The new review depends only on completed106.
An initial sandboxed create could not acquire Backlog's required `.git` creation
lock; the authorized CLI retry created exactly one review record, MSH-115.

Newly performed: scoped source/reference reads, declaration/caller inventory,
38 local documentation-link checks, unique design boundary/decision ID checks,
and full115-record dependency validation. All114 older task files, the complete
MSH-106 report/evidence, production code/manifests/workflows and accepted game
evidence remain byte-unchanged. Historical roadmap content remains a verbatim
suffix beneath the new current-policy pointer. There are no missing/self/
duplicate-edge/cyclic dependencies; Backlog doctor and diff checks pass.
No implementation records were created and no future gate was changed.

The independent reviewer re-read and approved the staging clarifications:
observation-validator migration, directional finite bridges, per-stage deletion
and R6 as audit. No unresolved review finding remains. Target selection is still
an owner decision; implementation is neither claimed complete nor authorized
by closing this diagnostic review. Existing other worktrees were preserved;
this review created no temporary worktree.
