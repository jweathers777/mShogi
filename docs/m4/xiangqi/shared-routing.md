# Xiangqi native context and routing contract — MSH-95

This is the real-record shared boundary for `mshogi.xiangqi.wxf2018.v1`, not
Xiangqi evaluator/search delivery. [Native history](history-contract.md) remains
authoritative. [Evidence](routing-evidence.md) records verification and review.
The owner's correction preserves the original task split:95 routes,105 owns
required exact/bounded no-win verification,97 owns evaluator/search/real child
after native handoff84 including105. The superseded request and correction are
preserved in MSH-95's notes; no ownership/dependency or acceptance criterion changed.

Current execution update: MSH-97 implements the [functional native engine](engine.md)
and ordinary turn/analysis capabilities. The MSH-95 routing-only scope and
then-unsupported execution statements below are preserved historical evidence;
they do not describe current capabilities. Native schema/envelope/transport
identities, strict parsing and context ownership remain unchanged.

## Explicit identities and strict canonical bytes

`GameKind::Xiangqi`, `RulesProfile::XiangqiWxf2018V1`,
`NativeSchema::XiangqiV1`, `CanonicalRecord::Xiangqi` and
`NativeSide::Xiangqi` extend the existing small dispatch layer. There is no common
board/action representation or universal side/score interpretation. Xiangqi
Black is distinct from Chess Black; Red is not White or Sente.

| Identity | Value |
| --- | --- |
| Game | `xiangqi` |
| Rules profile | `mshogi.xiangqi.wxf2018.v1` |
| Immutable edition | `WXF-2018+XIANGQI-P1-2026-10-27` |
| Native schema |1 (`mshogi.xiangqi.game`) |
| Canonical envelope version |1, unchanged |
| Shared transport version |3, unchanged; existing Chess legacy2 remains supported |

The six-field canonical envelope derives identity metadata from its native
record. `RawValue` preserves complete original envelope and native payload
bytes until strict typed parsing and the selected native parser reject duplicate,
unknown, missing or conflicting fields. No intermediate map precedes that
boundary and no failed parse falls through to another game. Outer identities
must agree with the native profile/schema/edition. Unsupported proof modes or
limits remain explicit rejections by the native MSH-94 parser.

The extension keeps versions because all existing game encodings and semantics
are unchanged. Older binaries explicitly reject the new game/profile tag rather
than guessing another game; existing saved Chess/Shogi records need no migration.
Native canonical encoding is unchanged: root provenance/declared counters, full
known commands and events, warning/retirement state, claim submissions and denial
audits, conclusion, policy and native G-NOWIN assessment/mode/goal/limit all
remain in the payload. No FEN-only reconstruction or inferred earlier history.
Validated native records now cache only event count, known board-action count
and current native turn, derived at authoritative construction/replay. These
getters do not repeat replay, execute proof work or trust new wire metadata.
Runtime instance/revision identities remain outside canonical data.

## Native command ownership

`NativeCommandTarget::Xiangqi` borrows the real native `Game` and dispatches an
unmodified native `GameRequest`. `NativeReply::Xiangqi` preserves the complete
`Submission` (its accepted/rejected-claim audit result AND undo) or native
`GameError`. Wrong-game tags reject before mutation. The dispatcher does not
apply shared side-to-move rules: board-action actors, count claims, resignation,
agreement and stale native contexts retain their MSH-94 admission distinctions.
An invalid count claim can preserve the board while adding its separately
admitted two-event audit. That is neither an illegal board move nor a conclusion.

## Execution stays explicitly unavailable

`execution_capabilities(Xiangqi)` is empty. A well-formed Xiangqi v3 engine-turn,
read-only move-analysis or Chess-Full-shaped request receives only
`SharedResult::Failure` / `UnsupportedMode`, with its exact submitted context.
It receives no invented action, claim, score, telemetry, proof or conclusion,
even if the native record already contains a genuine conclusion or fast proof.
Chess Full is never reinterpreted as a Xiangqi analysis operation. Malformed
identities/targets are `InvalidRequest` or strict transport parse errors instead.
There is no evaluator/search/default/max-work policy or execution implementation
in95. The `u16` depth / `u64` node fields are retained as opaque requested settings
for an unsupported response, including0/0 and65535/u64-max; this is transport
representability, not advertised search capability. Full-shaped settings retain
their existing bounded wire structure and sentinel before unsupported rejection.

The server dispatches Xiangqi explicitly before Chess/Shogi execution adapters.
No Xiangqi native search or exact verifier is called. G-NOWIN remains the owner's
existential natural board-win question (mate or stalemate loss only), excluding
repetition penalties and other non-board awards. Proven, unknown and failure
values in the native record are preserved as data; unknown is not asserted to
be ongoing, winnable or drawn.105 remains responsible for its operational exact
verification contract. No feasibility or strength measurement is claimed here.

## Exact context and execution lifetime

Acceptance binds the entire native canonical record, purpose, limits, request
identity, host session version and process generation. Analysis also binds fresh
selection identity, exact complete event-prefix count, known board-action count,
target session version, label and native current turn. A supplied Xiangqi target
must match the cached authoritative turn, not merely name the correct game.
The target describes the complete submitted native record; it cannot relabel a
live/full-history record as an earlier selection. Event prefixes inside an atomic
native command remain read-only snapshots, not newly resumable canonical games.

An unsupported response still must echo every field exactly. Foreign Chess/
Shogi result families cannot become Xiangqi results. The existing single
supervisor, one current/pending/result slot, bounded pipes/diagnostics, child
watchdog and kill/reap cleanup remain unchanged. Request and process identities
increase across games and wire versions; cancellation/replacement retires both
pending and published work. Native semantic undo or reopen may reproduce the
same canonical record but cannot restore retired runtime work. Real-subprocess
and separately labeled timing/forgery tests establish those distinctions.

## Bounds and compatibility

Caps are unchanged: native payload1MiB, canonical envelope1MiB, complete request
1MiB, complete echoed response2MiB (newline excluded), retained diagnostics4096
bytes. Xiangqi retains native512-command/2048-event admission and reserved native
closure/saveability. Chess keeps its existing shared4096-event bound; Shogi keeps
its native record limits. An enclosing frame must fit independently: a native
payload fitting its own cap does not promise room for envelope/purpose overhead.
Actual complete encoding, not payload-only arithmetic, determines admission.

The measured512-command/1024-event native audit ledger plus resignation is
253656 native bytes,253832 envelope bytes,254001 request bytes and254283 echoed
unsupported-response bytes. Exact1MiB request and standalone-envelope input is
accepted; one additional byte is refused. These are concrete measured cases,
not a proof that all native maximums fit an enclosing frame. No constants rose.

Chess native schemas1/2, legacy2 engine contexts and historical observations,
and Shogi native schema1/shared3 execution remain semantically unchanged.
Frontend native fixtures remain unchanged. Existing desktop creation/replacement
explicitly refuses Xiangqi before retiring the current document or engine work;
minimal native-qualified label/exhaustive-match compatibility adds no desktop
session, archive, renderer, selectable game or analysis capability.95 does not
implement later console, external protocol, engine, desktop or archive features.
MSH-45 and M4 remain open. After95 completes,105 is dependency-ready; native
handoff84 then gates97 and the later consumers under the existing graph.
