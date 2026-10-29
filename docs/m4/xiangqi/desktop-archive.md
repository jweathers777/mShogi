> Current desktop adaptation — MSH-102 (2026-10-29):
> [analysis, outcomes, recovery and document journeys](desktop-journeys.md) now
> use this native contract in the normal product, including strict Xiangqi Open,
> exact-event history, stopped reopen and explicit Resume. Final source is
> `a1f8234e6d82add1c485e7e2537e414cffd4d49d`; see
> [execution/review/CI evidence](../evidence/msh102/completion-2026-10-29/README.md).
> Earlier unstarted/product-gate statements below are preserved historical
> handoffs, superseded by these bounded deliveries. MSH-45/M4 remain open;
> MSH-103 and MSH-106 are not started.

# Xiangqi native desktop archives (MSH-99)

This is lossless native document persistence and admission, not the Xiangqi
product UI. MSH-76/101/102 retain shared reconciliation, visual play and complete
product journeys. MSH-111 owns the recorded Linux WebDriver proxy failures;
native archive tests do not relabel those campaigns successful.

## Version and authority

The existing `mshogi.desktop.archive.v2` envelope adds the explicit
`game_kind:xiangqi` variant with `mshogi.xiangqi.wxf2018.v1` and its immutable
edition. Chess still writes the unchanged strict v1 archive; shared reads of
v1 Chess and v2 Chess/Shogi remain supported. Shogi payloads are unchanged.
Older readers reject the new game identity rather than reinterpret it.

The Xiangqi payload requires engine transport version3 and the actual native
play limits (depth1/128 visits). Its initial/current canonical envelopes retain
their independent envelope version1 and Xiangqi native schema1. Wrong versions,
profiles, editions, native modes or unsupported play limits reject explicitly.
No restoration silently substitutes another mode or limit.

The payload retains the complete initial/current native ledgers, human side,
initial/current native orientation and ordinary analysis defaults, accepted
desktop controls and their exact order, ordinary observations, completed exact
evidence and a retired execution high-water mark. Native ledgers own root/counter
provenance, actions and their effects, claimant identity, claim audits, WXF
warnings/obligations, offers/agreement, resignation and conclusions. There is no
FEN-only or board-ply-only restoration path.

`RawValue` carries original envelopes and nested native contexts into strict
typed parsing. Resource probes never replace their bytes with a map. Native
replay validates derived events/current positions/conclusions; ordered desktop
replay additionally checks each control's actual event/board-action prehistory,
analysis intent, full target history, work and execution binding. An observation
cannot change purpose or use a same-board context with different history.
Malformed, duplicate, unknown, conflicting or unsupported fields reject.

Within one Open, original byte-identical canonical envelopes reuse an immutable
record only after its strict authoritative parser has succeeded. A changed byte
is parsed afresh; no JSON map normalization is used. Pure native variation
validation also reuses a result only for the same complete authoritative record
and complete typed action/claim sequence. All observation identities, purpose,
target, journal bindings and actual engine-turn application are checked every
time. These temporary caches have at most the bounded archive's records and256
variations; they retain no execution work across Opens.

## Admission and measured capacity

The existing limits remain:16MiB actual encoded archive,8192 controls and256
combined ordinary/exact observations. Xiangqi's independent native limits are
512 commands,2048 events and1MiB record; canonical-envelope and request/response
limits also apply to their actual encodings. These are distinct dimensions.

Every command/result is checked on a private candidate before accepted state
changes. The controller captures immutable admitted values; native restoration,
transitions, historical reconstruction and encoded-capacity checks run on the
existing bounded worker. Invalid commands leave the accepted game and running
request intact. Capacity refusals may pause execution, retain all accepted
evidence and explain that Save/reopen cannot enlarge the structural limit.
No retry-save advice is offered for an unchanged structural limit.
The guard also checks each actual pretty-encoded canonical wrapper, including
archive indentation, against its independent1MiB parser limit. A compact native
payload fitting its own bound is insufficient by itself.

The replay ceiling stays16,000 work units. Xiangqi charges4 per native event
and an additional12 per board action, for both initial/current records and each
observation source history. Each observation adds8. Each distinct complete
canonical context/nonempty PV adds512 for native reconstruction plus16 per action;
identical validated PVs pay once within the Open. This fixed charge conservatively
covers the measured34ms uncached root validation even when history is empty.
Each exact positive witness pays512 plus64 per witness action, since its local
terminal/legality replay is not cached. Cancellation is checked per witness action;
an individually excessive witness is refused before replay. Controls cost1, except Undo/Inspect charge their
actual pre-command history reconstruction at the same weights. Saved prehistory
counts are checked before reconstruction. Raw probes charge the maximum of
command-derived and event-derived counts, so omitted forged derived events
cannot make expensive native command replay appear cheap.
Raw Open preflight also charges distinct original-source/PV encodings and positive
witnesses before native reconstruction. The writer produces consistent source
encodings; externally reformatted duplicates may conservatively cost more. Every
original encoding still undergoes strict parsing before use.

An active or durably active journal reserves one control, one replay unit and
1024 encoded bytes for stop/close. Close consumes that reservation once and is
idempotent. Identity fields are budgeted at maximum supported width; historical
observation flags use the longer encoding. Reopening pauses runtime work without
releasing a journal's outstanding reservation. Saving/reopening does not reset
observation count or discard observations.

Actual debug measurements on macOS arm64 establish why event count alone was
insufficient. The initial4/event-only policy admitted a100-ply natural-count
claim document with35 observations; Save passed but Open exceeded the existing
15-second worker deadline. That failed experiment motivated the board-action
charge and removal of redundant native reconstruction for empty PVs. Neither
the ceiling nor deadline was raised.

With the corrected policy, actual pretty archives containing a historical-root
observation and repeated real native search results reached these admission
boundaries and reopened losslessly:

| Native ledger | Events | Observations | Controls | Encoded bytes | Debug reopen |
|---|---:|---:|---:|---:|---:|
| WXF warning plus40 rejected-claim audits |89|32|67|834,597|317ms|
|100 board actions plus accepted Red natural-count claim |103|7|17|227,553|739ms|
| Standard root, completed ordinary observations |0|256|512|1,110,084|335ms|
| Standard root,1ordinary plus255exact Unknown observations |0|256|257|754,793|215ms|
| Standard root,distinct legal-proposal PV fixtures |0|29|59|128,841|1,067ms|
| Two-ply native exact positive-witness evidence |0|24|24|74,293|15ms|

The ordinary256 test repeats one actual completed native result under fresh
admitted request identities at the identical history; it does not claim256
engine executions. A separate independent probe executed256 actual supervised
child analyses: after variation reuse, its same1,110,084byte archive reopened in
337ms. Before variation reuse, those actual observations reopened in8,732ms
locally (the template fixture measured8,550ms). The earlier Windows CI
attempt failed the unchanged15-second Open budget for both256-observation
fixtures. Those failures motivated validation reuse; neither budget nor criteria
was relaxed. Exact-source corrected platform results are in the task record.
The distinct-PV fixture substitutes native legal alternatives into a captured
result solely to stress admission; its scores are not evaluation facts. The30th
distinct one-action PV and25th two-action exact witness exceed the documented
work budget and are refused before commitment. Tests also reject raw archives
that bypassed admission, preserve all29/24 accepted observations and reopen them.

Times are observations, not cross-platform or deadline guarantees. The worker
remains independently cancellable and bounded. The work budget can bind before
the byte limit; tests distinguish meaningful valid-record boundaries from
isolated structural byte guards rather than manufacturing a valid16MiB history.

## Historical exact reports

`retain_xiangqi_exact_evidence` explicitly attaches **completed historical
evidence** from the immutable native `ExactObservation`. It does not launch an
exact request, accept pending execution or advertise a shared exact engine
operation. The snapshot precondition and journal bind the attachment to its
source record and exact selected event, including an intra-command prefix.

`mshogi.xiangqi.exact-observation.v1` preserves the native in-process executor
(there is no engine process generation), native session/generation, natural-board
goal/purpose, full source/root provenance, exact target diagram/counters,
limits and complete report/work observations. Fast evidence remains in the
native canonical records and ordinary telemetry. Exact positive witness,
reported proven no-win, Unknown/resource exhaustion, cancellation and failure
remain distinct. Positive witnesses receive bounded native board replay;
strict report/work metadata and target provenance are checked.

Open performs **no exact graph search**. Negative reports remain historical
reports with provenance, not newly certified graph proofs or current outcomes.
No exact attachment can conclude the live game, supply a forced-win score or
enter the engine-turn action handler. Full source history and the selected
prefix remain separate; future moves do not enter the target board.

A native reply with an invalid variation is retained as rejected, unscored
evidence without applying even its first move. If a consumed response cannot
otherwise pass document admission, execution is retired with an explicit failure;
the controller never restores Thinking for a reply that has already been consumed.

## Save, Open and execution ownership

Save reuses same-directory temporary write/flush/sync and atomic replacement.
Failure/cancellation preserves the prior valid file and does not clear dirty
state. An immutable accepted revision is saved; newer accepted changes remain
dirty. Stale Save completion cannot claim a replacement document's path.

Open validates entirely off the controller lock before token-checked installation.
Corruption, unsupported identity/mode, resource refusal or cancellation preserves
the current accepted document. Success creates fresh document/selection/session/
request identities above current and saved high-water marks, retires old work,
keeps all observations historical and preserves the exact selected event.
Return to live is explicit; play stays stopped until Resume. Semantic Undo,
selection and replacement never rewind execution ownership.

Native callers use `queue_shared_save`/`queue_shared_open` and stable admission
receipts. Tauri's existing product uses `queue_product_open`, which rejects
Xiangqi before installation while its UI is unavailable. Existing Chess/Shogi
file controls and unsaved replacement/quit guards remain in place. This prevents
an archive feature from accidentally admitting an unrenderable product game.

## Reproduction and evidence

```sh
cargo test --locked -p mshogi-desktop-session shared::xiangqi::archive::tests -- --nocapture
cargo test --locked -p mshogi-desktop-session --all-features
cargo test --locked -p mshogi-core --test xiangqi_exact
cargo clippy --locked -p mshogi-desktop-session --all-targets --all-features -- -D warnings
cargo build --release --locked --bin mshogi
cargo run --locked -p mshogi-desktop-session --features desktop-verification \
  --example xiangqi_session_probe -- target/release/mshogi
```

Tests reuse frozen WXF cycle/count fixtures and independently expected natural
board witnesses/closed palace cases. They cover actual engine payloads, all
ordinary/exact evidence categories, same-board/different-history, saved selection,
fresh stopped reopen/Resume, stale work, strict malformed/derived disagreements,
interrupted/failed Save/Open, exact work/control/byte boundaries and observation
256→257. Measurements are emitted by the native test; no benchmark, fixture or
test instrumentation is added to normal distributables. Exact tested revisions,
independent review and actual CI outcomes belong to the MSH-99 task record.
The separately gated probe exercises actual child play/analysis/cancellation,
Save at a historical event, fresh-controller stopped Open, Return to live,
explicit Resume and continued child play/cleanup. It is native/process/archive
evidence and is excluded from normal distributables.
