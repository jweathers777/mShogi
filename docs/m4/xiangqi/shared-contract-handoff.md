# MSH-76: aggregate shared Xiangqi handoff

Current product adaptation (2026-10-29): [MSH-101](desktop-product.md) supplies
its real Xiangqi product union/intersection renderer, native New Game/play and
movement inspection in the accepted shell. The owner-approved checkpoint and
[completion evidence](../evidence/msh101/completion-2026-10-29/README.md) supersede
the then-closed frontend gate described in this historical MSH-76 handoff.
Product Open and full analysis/outcome/document/history journeys remain MSH-102.
No shared native semantic contract is reopened by presentation work.

The existing MSH-84/95/98/99 production contracts form one coherent native
consumer path. No production change or additional rules, engine, archive or
board abstraction is required by this reconciliation. MSH-76 adds two combined
native integration journeys and this consumer map. It does not establish a
finished Xiangqi desktop screen or packaged Xiangqi GUI interaction.

## Authority and identity across boundaries

| Boundary | Existing owner and concrete contract |
|---|---|
| Native rules/history | MSH-84: `xiangqi::Game`, complete commands, native replay and canonical root/event provenance; warnings, obligations, claimant-specific claims and conclusions stay native |
| Record dispatch | MSH-95: `GameKind::Xiangqi`, immutable `mshogi.xiangqi.wxf2018.v1`/edition, canonical envelope1/native schema1; original `RawValue` bytes reach the strict native parser |
| Execution | MSH-97/98: shared transport3, exact complete canonical history/purpose/target/request/session/process/limits equality; supervised bounded work and separate read-only analysis acceptance |
| Session/presentation | MSH-98: serialized authoritative command admission, fresh selection identity, native complete choices and immutable historical views; no consumer reconstructs legality or history |
| Archive/admission | MSH-99: tagged desktopv2 Xiangqi, authoritative native and ordered desktop replay, exact selection/settings/observations, fresh stopped reopening and pre-acceptance saveability |

Game identity, profile edition, native schema, canonical envelope and engine
transport are independent identities. Unknown/conflicting identities never fall
through to another game. Chess v1 archive writes and v1/v2 reads, Shogi v2 and
existing engine behavior remain unchanged. See [routing](shared-routing.md),
[session](session-contract.md) and [archive](desktop-archive.md) for their complete
contracts and measured limits.

Ordinary Xiangqi execution remains `sound_fast_only`, actual play depth1/128
visits and supported ordinary analysis depth1–3/1–512 visits. Unknown no-win
evidence establishes neither an ongoing game nor a positive win witness; it can
coexist with an ordinary heuristic evaluation. Incomplete or failed exact work
remains unscored. Exact graph
work remains the explicit bounded native facility; Chess Full is not a Xiangqi
operation. Saved exact proof/witness/Unknown/cancellation/failure evidence is
historical information, never resumed work or a newly invented outcome.

An event index is not a board ply. At a committed historical prefix, the request
contains the exact selected canonical history. Inside an atomic command (for
example before its derived WXF warning), `canonical_context_scope` is
`live_with_immutable_selection`: live canonical context and selected immutable
events/diagram are explicitly separate. `selected_canonical` is absent; analysis
and claim advice are unavailable rather than reconstructed from future events.
Read-only native inspection still uses the selected position and exact token.

Cancel, selection, replacement, undo and Open retire execution identities;
returning to an identical board or history cannot revive them. Analysis cannot
pass through engine-turn admission. Return to live and Resume are distinct.

## Semantic output into the accepted application shell

`desktop/src/product/ApplicationShell.tsx` supplies the accepted dark composition,
including `GameArea`'s stable player/board/footer slots. Its board content is a
React node, not a mandatory square topology. The following map identifies native
inputs; it is not a new UI design or an implementation of MSH-101.

| Stable shell slot | Native semantic input |
|---|---|
| Header/document controls/title/state | `document`, `open_token`, `io`, `capacity`; existing Save/Open and unsaved-replacement preconditions |
| Live-game heading/Flip | `turn`, `status`, `historical`, `paused`, `orientations`, native orientation setting |
| Upper/lower player areas | `sides`, `human`, native Red/Black ownership and side-to-move; no hand contents |
| Board region | `board.intersections`:90 points, river between ranks4/5, two owned palaces with diagonals; `pieces` native kinds/owners and opaque coordinate keys; complete `choices` |
| Common bottom status | `historical`, `status.concluded`, `paused`, `phase`, `turn`, `board_ply`; event index remains separately available |
| Analysis | `capabilities`, actual `play_limits`, `execution_purpose`, `analysis`, `search_observations`, target history and native score meaning |
| History/Return to live | `events`, `event_index`, `token.selection`, native selected events/canonical scope; no future warning/claim information |
| Game actions | Native complete choices and claimant-specific advice/capabilities; native resignation/agreement/count commands, explicit Resume/Retry/Cancel |
| Technical/details | Typed context/profile/edition, exact identities, root provenance, WXF warnings/obligations, diagnostics and historical exact evidence |

The sidebar order remains Analysis, History, Game actions, technical disclosure.
Document controls stay in the common header. Xiangqi does not advertise hands,
drops, promotion, declarations or Chess Full; empty/false optional capabilities
are intentional. Native `MovementInspectionQuery` binds the exact selected token
and returns complete non-actionable choices. No TypeScript movement calculation
is needed or authorized.

The current product frontend union and Chess/Shogi square renderer deliberately
remain closed to Xiangqi. MSH-101 owns their real Xiangqi adapter, label and
intersection renderer in this existing shell, rather than feeding9x10 into the
old square renderer's large-board scrolling path. The compatibility field name
`PresentedPiece.square` is an opaque native coordinate key, not square geometry.
Nothing in native intersection metadata requires pixels, scrolling, zoom or
panning. MSH-101/102 must prove full-board visibility and same-size product
composition on supported desktop sizes. No final rendering is claimed here.

## Saveability and compatibility

The MSH-99 policy is retained without raising limits:16MiB encoded document,
8192 controls,256 combined observations, independent native command/event/byte
limits and measured16,000 weighted replay units, with reserved stop/close space.
Admission checks a private candidate before committing. Historical flags, full
identity widths and actual encoded canonical wrappers are included. A capacity
refusal preserves all accepted data and a complete save path; Save/reopen does
not reset structural capacity. See the [measured policy](desktop-archive.md)
for weights, long-record experiments, exact256-to257 boundaries and failed
experiments retained as failures.

Strict native replay validates conclusions, warnings, claims and event ordering.
Observations retain their complete target history. Open is atomic and assigns
fresh runtime identities with work stopped; historical observations never become
pending execution. Existing failure/dirty-file protection and Chess/Shogi archive
behavior remain production-unchanged.

## Evidence and limits

`shared_xiangqi_handoff_tests.rs` is a test-only child of the existing native
Xiangqi test module. It reuses frozen, independently reviewed `F022-CC` in
`tests/fixtures/xiangqi/history-cycles.json`:8 board actions,9 events,1 warning.
It does not derive a new rules oracle from engine output.

1. Live canonical creation, actual native entry-point analysis, opponent inspection
   at warning-free event8, Save, Shogi replacement, Open with exact prefix and
   historical observation/settings, rejected old result/token, Return to live
   with warning9, explicit Resume and actual engine-turn result admission.
2. Same board/turn with different known histories, historical analysis, cancellation
   and fresh request, wrong-game and unsupported-profile rejection, Shogi
   replacement and restoration of the original Xiangqi history, with every old
   result still rejected and current analysis read-only.

These use deterministic injected responses from the real `serve_once` engine
entry point. The separately feature-gated existing `xiangqi_session_probe`
exercises the actual supervised subprocess and cleanup; neither is GUI evidence.
Independent review also runs native canonical strict-parser and Chess/Shogi
compatibility suites and the actual Xiangqi engine-routing suite. Exact commands,
revisions, outcomes and review method are recorded in MSH-76.

Unchanged MSH-99 production source at `dd510f3e9a89c934ef573b78d324478b336c3fa9`
has successful three-platform [Rust CI](https://github.com/jweathers777/mShogi/actions/runs/40722182393)
and [desktop CI](https://github.com/jweathers777/mShogi/actions/runs/40722182395).
This evidence covers unchanged production, not execution of the new tests.
The recorded Linux GUI failures remain failures; MSH-111's separate driver fix
and subsequent package interaction evidence do not rewrite those campaigns.

MSH-76 hands off to dependency-ready MSH-101; MSH-102 owns remaining product
journeys. Neither is implemented here. MSH-45/M4 remain open; MSH-106 begins
only after complete Xiangqi delivery. No architecture-health review is performed
as part of this aggregate acceptance.
