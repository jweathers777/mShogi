# MSH-77: current shared Tori integration refinement

Planning baseline `a3677e1e862fdf3877c784f12f56a99c4bfbf167`, primary
`codex/rewrite`, 2026-11-01. This is **planning only**. Accepted production remains
`994521bec0637d6f8f17103f1106c0025d950753`; [MSH-85 acceptance](native-acceptance.md)
and completed126–129 supply `mshogi.tori.modern.v1`, edition
`MSH81-MODERN-2026-11-01`. No rules research, implementation or GUI campaign is
reopened. Earlier [native plan](native-plan.md) and historical handoff remain
intact. [Testing policy](../../testing/conformance-policy.md) requires independent
expectations and review; no external engine or historical acquisition gate.

## Bounded sequence and ownership

Three flat components under77 follow existing repository practice. The document
codec/admission comes **before** installable session work: the first ordinary
accepted Tori change must already have a complete current-format Save path.
This is a real dependency, not parallel work on the same shared contract.

| Component | Complete direct dependencies | Independently completable result |
| --- | --- | --- |
| **MSH-130** — current document codec and lossless admission (suggested B) |69,72,73,85 | Immutable native payload/control/settings vocabulary, strict replay, measured whole-document candidate admission, bounded worker/atomic-file tests. Native-controller Tori installation stays unsupported until131; normal product admission stays gated for42. No active document or runtime owner. |
| **MSH-131** — native runtime/session/lifecycle (suggested A) |130 | One ActiveDocument Tori arm, native command admission and inspection, minimal correct snapshot, exact selection/Undo/Take back, sole installation and Save/Load/lifecycle integration using130 from the first accepted change. Real unsupported subprocess evidence, no engine. |
| **MSH-132** — shared semantic presentation/consumer verification (suggested C) |131 | Complete semantic snapshots and existing frontend/session/choice-controller contract tests using actual native-generated fixtures; final cross-boundary evidence and handoff. No final Tori artwork, notation or playable chooser. |
| **MSH-77** — coordinating final acceptance |69,72,73,85,130,131,132 | All four **original unchanged** criteria plus DoD independently supported after integrated components; planning alone never closes77. |

All three children have scoped Ready status with unchecked acceptance/DoD and no
assignee or implementation plan; **130 alone is dependency-ready**. The next
worker records its current plan on activation.77 ends this planning phase Ready,
unassigned and blocked on its components. No child depends on77,42or46.42 retains
its existing77 prerequisite and all eight delivery criteria;46 retains its full
shared/all-game gates. No original dependency was removed.

## Exact surviving source consumers

Paths are repository-relative, symbols inspected at the baseline. Routine type
spellings can be chosen by the activating implementer within these boundaries.

| Current source / owner | Required concrete extension and verification owner |
| --- | --- |
| `crates/mshogi-core/src/native.rs`, `tori/{game,legal,record}.rs` | **Already delivered.** GameKind::Tori, RulesProfile::ToriModernV1, NativeSchema::ToriV1, NativeSide::Tori, CanonicalRecord::Tori, native command dispatch. No alternative movement, T06 history, canonical record or rules model.130/131 consume actual native values. |
| `crates/mshogi-engine/src/shared.rs`, `server.rs`, `bridge.rs` | One transport4 ExecutionContext/Purpose/Target/Result and schema1 ExecutionObservation already carry real Tori records; `capabilities(Tori)` is empty and real server returns UnsupportedMode.131 exercises these. No Tori score/result/search family is required. Full equality includes record/history, purpose, request, session, process generation, limits and exact target. |
| `crates/mshogi-desktop-session/src/document.rs` | Sole `mshogi.desktop.document.v1`, envelope_version1, game/profile/edition/native_schema_version1 and RawValue payload.130 adds the concrete native Tori document payload; existing strict envelope can name its profile, while worker dispatch currently cannot install it. No obsolete-format migration/reader or second envelope. |
| `shared_archive.rs`, `persistence.rs` |130 defines immutable payload/settings/control journal and off-path strict replay/admission/atomic-file operations.131 extends validated worker result/install routing and **sole** `persistence::install_document`. A validated document candidate is data, not another live document owner. |
| `capacity.rs`, `resource.rs`; `desktop/src/resources.ts`, `product/resourcePresentation.ts` |130 accounts actual native and whole-document budgets, adds native Tori ResourceScope and frontend union as needed, preserves measured categories/counts/reservation.131 uses receipts and immutable admission before every accepted change. No diagnostic-number inference. |
| `lib.rs` DesktopController/ActiveDocument/RuntimeMetadata; `lifecycle.rs` |131 adds exactly one Tori document variant with game-owned state. Reuse monotonic runtime/document/selection/save identities, one bridge and workers, lifecycle/close ownership. No second controller or Shogi shadow document. |
| `shared.rs` SharedDesktopCommand/SessionOperation/SessionToken/SharedSnapshot |131 adds explicitly Tori-qualified command/settings dispatch and a correct minimal snapshot;132 completes semantic presentation. Existing `Play` contains **shogi::Action**, not a generic move. Use a Tori native GameCommand arm (not Shogi notation/string conversion). |
| `shared_inspection.rs`; `takeback.rs`; `shared_xiangqi.rs` as ownership evidence |131 calls Tori native `LocalPosition::inspect` on the exact selected prefix for hypothetical owner movement. Actual live choices use `Game::legal_actions` and `submit`.131 owns native lineage, exact selected event, Undo/Take back and permanent work retirement. Never copy Xiangqi warnings/claims or Shogi declaration semantics. |
| `moves.rs` + game adapters |132 supplies exact board-action/event mapping and native last-action geometry. Stop/resignation are event context, not extra move rows. Final human Tori notation is42; technical action labels are explicitly technical, never falsely standard notation. |
| `desktop/src/product/native.ts`, `useGameSession.ts`; `desktop/src-tauri/src/main.rs` |131 extends only actual host dispatch required for document/controller operations;132 extends strict frontend game/side/settings/operation types. One `game_snapshot`/`game_command`/file/lifecycle path and one frontend owner. Product `game_new`/supported selector cannot advertise Tori play. |
| `ProductApp.tsx`, `ApplicationView.tsx`, `GameFrame.tsx`, `GameBoard.tsx`, `NativeBoard.tsx`, `boardPresentation.ts`, `interaction/useNativeChoiceController.ts` |132 verifies actual metadata with existing shared composition and one interaction lifetime, not a new page/renderer framework. Existing square geometry, Board/Hand origins, routes/completions suffice for7×7. A development fixture can exercise these without claiming a final Tori product renderer. |

### Required native payloads and semantics

- `tori::GameRequest { expected: GameContext, actor: Side, command: GameCommand }`
  admits Play/Resign/Stop and explicitly refuses unsupported AgreeDraw/DeclareImpasse.
  Board turn validation must not accidentally reject permitted out-of-turn resignation.
  `GameContext` is full root/history plus non-rewound native authority; it is not
  a saved process identity. Outer commands retain the complete `SessionToken`.
- `Game::record_prefix`/`event_prefix`, immutable `HistorySnapshot`, `GameUndo`
  and native context retirement supply exact known history, occurrence/departure
  obligation, conclusion and incomplete reason. Inspection calls the prefix's
  `LocalPosition`, not a current diagram with future repetition knowledge.
- `GameRecord` (`mshogi.tori.game`, schema1) stays authoritative, including every
  redundant prefix effect/occurrence/obligation/conclusion verified by native replay.
  Structural admissibility, declared counter, known history and proven reachability
  remain distinct. No SFEN/USI alias or profile conversion is introduced.
- `PresentedPiece`, `PresentedHand`, `BoardDescriptor`, `Orientation`,
  `ChoiceOrigin::{Board,Hand}`, `PresentedChoice`, `Completion`, `PresentedEvent`,
  `PresentedStatus` and `MovementInspection` carry native semantics. All9faces,
  distinct handed quails, forced promotion, Sente/Gote ownership, hand counts and
 7×7 orientation come from Rust. No optional promotion menu or declaration by analogy.
- `ExecutionObservation` stores only actual observations with complete native
  context. UnsupportedMode, cancellation, work exhaustion and process failure
  remain unscored and cannot mutate a native conclusion. Ordinary Analyze/Resume/
  Retry must explicitly decline nonexistent execution; no hidden child launch or
  fabricated fallback. Native human board admission can be tested through explicit
  record/session commands before an engine exists. Saved work never resumes itself.

## Executable acceptance and fixture ownership

[Supplemental consumer mapping](shared-consumer-ownership.json) enumerates every
remaining field and requirement, retaining the original owner and pending status.
It supplements, and does not edit, the immutable103-ID native ownership manifest.
No test result is claimed by this planning map.

| Existing obligation | Exact primary implementation / supporting checks |
| --- | --- |
|13 pending fields owned77 |131. C-IDENTITY2fields: reject A-for-B and same-diagram undo revival. O-ADMIN3fields: no native conclusion, recorded reason, no score. H-REPLAY-UNDO1: no work revival. Seven root/strict rejects: actual live document unchanged.130 supplies strict invalid candidate cases for the seven rejects;132 checks frontend late-state handling. |
|82 consumer requirements owned77 |131 final actual runtime verdict for all rows:35base+35ROTATED+7rejects plus3history/prefix+1identity+1admin.130 supports77admission/rejection rows and3history/persistence rows;132 verifies applicable semantic/frontend projections and reconciles every row's integration evidence. Never count supporting checks as extra completed requirements. |
|C-NOTATION,1pending field+1requirement |Remain42, actual human notation/console/display adapter.132 must preserve opaque native action authority without claiming that future adapter passes. |
|713 completed native assertions,0native pending |Fixed accepted foundation. Future slices rerun the real native adapter as affected regression, not rewrite expectations or reclassify these consumer facets as kernel failures. |

Concrete tests in each future task must emit fixture ID, exact input/expected fact,
actual native/controller result and real consumer boundary. Use the existing
harness for native projections, with a consumer adapter or supplemental report
for actual runtime assertions rather than making a fake native pass. All pending
counts remain honest until the owning real test executes.

**130 document cases:** actual I01; CAPTURE-OWNER/DEMOTION/QUAIL; P-ENTRY/
P-DROP-EXIT; native full8/16 repetition sequences; C-HISTORY/H-IDENTITY/
H-REPLAY-UNDO; resignation and all REJECT-* fixtures. Strict duplicates at both
current envelope and native nested record; unknown/missing/conflicting identities;
forged intermediate/final effects/obligations/conclusions; bytes/events/replay/
controls/observations exact boundaries and interrupted atomic writes. Native
profile expectations, not an external engine, establish facts.

**131 runtime cases:** every mapped82requirement/13field; all native admission
fixtures and their owner rotations enter the real controller with a preserved
before-state and exact expected accepted/rejected result. Test wrong game/side,
context/purpose/limits/request/session/process/selection, A→B replacement,
Cancel→replacement→old result, semantic Undo/Take back→same diagram→old result,
failed and successful Open. Use the actual existing internal child for Tori
UnsupportedMode; forced process termination and cancellation establish lifecycle
facts, **not a Tori playing engine**. O-ADMIN's four reasons remain independently
exercised; don't relabel every operational failure EngineFailure or conclude a loss.

**132 presentation cases:** native-generated7×7 fixtures include both orientations,
all face identities, quail capture/drop, hand counts and forced zone/drop-exit
promotion, terminal and incomplete events, exact historical prefixes and selected
last move. Existing one-controller tests exercise pointer/keyboard origins,
hand selection, Esc, complete submission once, read-only enemy/hand inspection,
async inspection/selection replacement, snapshot staleness, focus and announcements.
Use the durable common wording `Inspecting {side} {piece} · Esc clears`.
A diagnostic fixture is not final Tori UI design or human-engine acceptance.

Cross-game regressions are explicit, scaled to the actual changes: core native
Tori/Chess/Shogi/Xiangqi envelope and real internal subprocess tests; desktop
`document_tests`, `runtime_tests`, `workflow_tests`, `shared_archive_tests`,
`shared_xiangqi_archive_tests`, `shared_capacity_tests`, `shared_takeback_tests`,
inspection and Moves suites; frontend `useGameSession`, `useNativeChoiceController`,
`NativeBoard`, `Application*`, `Moves`, `TakeBackControl` and resource/lifecycle
checks as affected. Compile/check the **standalone Tauri crate** as well as the
workspace (129's historical exhaustive-match failure is preserved). Run relevant
format/lint and actual native/desktop CI for changed production; exact commands,
counts and revisions belong to each future implementation record. Do not rerun
unrelated packaged GUI journeys by rote or infer Tori GUI support from CI.

## Measured resource/saveability verification contract

These are existing caps, not a measurement of an as-yet unimplemented Tori outer
payload.130 must measure before selecting its actual whole-document charge policy.

| Layer at baseline | Boundary / required accounting |
| --- | --- |
|Native `GameRecord` / `Game` |512KiB encoded native record;512native events/commands;65,536deterministic replay units. Ordinary native admission reserves Stop+Resign. `capacity_usage` exposes actual bytes/events/work. |
|Canonical context / execution |1MiB canonical envelope;1MiB execution request and2MiB response;4KiB diagnostics. Account actual encoded context/purpose/target and envelope overhead, not only native JSON. No protocol-version or limit bump presumed. |
|Current desktop |16MiB complete encoded file;4,096retained native events (including lineage);8,192controls;256ordinary+any separately supported observations; safe runtime IDs with close/stop reservation. Current Shogi/Xiangqi whole-document replay limits are16,000 with **different charges**, not Tori's per-record65,536 units. |

Native acceptance's360board record is156,573B/65,341units with104captures,
103drops and16promotions; Stop+Resign reaches362events/157,044B/65,343units.
The510event case reaches512with Stop+Resign at165,930B/33,153units. These are
reusable **native** measurements, not evidence that such a history fits the future
outer aggregate replay budget. A record may fit native caps yet exceed supported
whole-document budget; reject the candidate before installation/acceptance,
without trimming it. Multiple embedded roots/prefixes, controls, observations and
retired Take back lineage all count for actual required replay. No cheap per-ply
proxy may undercount repeated work or terminal/control history.

Measure realistic actual complete documents plus worst supported retained contexts,
preferably sharing deterministic native long-record builders without changing
independent expectations. At each cap: at-boundary candidate succeeds when other
bounds permit; next candidate refuses atomically; reserve still admits required
Stop/close and supported concluding action; previous document roundtrips completely.
Observe real cancellation/replay latency and encoded bytes, document the units and
bounded off-path cancellation/deadline policy. Native admitted root/record cannot
bypass outer admission. An operation that cannot fit supported observations must
pause/refuse before commitment;256→257 never discards an old observation.

Atomic Save/Open: use existing writer and one bounded worker. Failed/interrupted
Save keeps valid old bytes and dirty revision; stale save acknowledgement cannot
clean newer content. Bad/unsupported/resource-limited/cancelled Open preserves
live game, runtime and accepted execution; successful validated installation gets
fresh never-restored identities and historical observations, stopped until explicit
user action. Until a real engine exists that action cannot start Tori engine work.
Preserve native close/stop and unsaved replacement/quit guards. Reopen never resets
structural capacity and cannot authorize stale observations.

## Complete-delivery and architecture boundaries

77 supplies shared consumer plumbing and evidence only.42 retains functional
engine/evaluation/history-aware search, console/human notation, final Tori-specific
board/artwork/hands, actual human-engine play/analysis, game-specific Save/Load and
outcome/recovery journeys and independent complete-game assessment.51/M5 retains
strength;46/88/89/47 retain six-game/shared/platform/M4 closure. No final delivery
criterion or pending evidence disappears because this native consumer works.

Native controller Save/Open and replacement are tested through the real native
contract and verification paths. Until the real Tori product renderer/playable
consumer exists under42, normal host New Game/Open must explicitly refuse Tori
rather than deliver its snapshot to a Chess/Shogi fallback renderer. This is a
product capability gate, not a second controller or record implementation.
The supported New Game dropdown remains only Chess/Shogi/Xiangqi.132 may extend
Tori snapshot types and a separately labeled verification path, never expose an
unsupported variant to normal players. No new visual shell: keep the same theme,
document controls, status, board/sidebar grammar, Analysis→Game Actions→Moves,
common keyboard/accessibility and native last-action cues. Final Tori pieces and
hand layout belong42. No obsolete pre-R1 bridge, per-game application owner,
mandatory common board representation, universal framework or advanced analysis
facility is required.

Planning validation and independent disposition are recorded in
[the review](shared-plan-review.md); actual integration references are recorded in
77 after commit/push. Stop after this refinement; no child implementation begins.
