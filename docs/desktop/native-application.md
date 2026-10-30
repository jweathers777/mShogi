# Current native desktop application contract

MSH-118/R3 replaces the always-present Chess session and optional Shogi/Xiangqi
overlays. Native rules, profiles, histories, actions and results remain owned by
their existing game implementations. This document does not authorize R4 or R5.

## State and authority

The application controller holds exactly one `ActiveDocument` variant: Chess,
Shogi or Xiangqi. Only that variant's native match/game, settings, journal,
observations, retained Take back branches and historical selection exist in the
accepted live state. Shogi and Xiangqi do not retain a dormant Chess game.

`RuntimeMetadata` owns common document/session/selection revisions, document
epoch and identity, monotonic request floor, saved/content revisions, path and
capacity. Native execution contexts retain complete game history, purpose,
request/session/process identities and limits. A saved identity is provenance,
never permission to resume old work. The single supervisor's process generation
and cancellation remain authoritative across game replacement.

The controller captures a native candidate and exact operation authority under
serialization. Bounded worker execution performs native command application,
reconstruction, replay, encoding and admission. Failed, cancelled or stale
candidates do not replace the accepted complete document. Receipts identify the
specific operation rather than whatever a later snapshot happens to report.

Current-file validation returns native typed data. One `install_document`
transaction checks authority and identity headroom, retires old execution,
assigns fresh runtime identity and installs the validated native variant stopped.
Saved observations become historical evidence. Return to live and Resume remain
separate operations; neither historical selection nor Take back revives work.

## Current host surface

Raw command strings reach strict native parsing before controller admission;
duplicate fields must not disappear through an intermediate object map.

| Host operation | Native contract |
| --- | --- |
| `game_snapshot` | Native-tagged snapshot, exact token, game-specific geometry/capabilities/history/results, document state, file state and resource facts |
| `game_command` | Exact rendered `SessionToken`, explicit game and typed native/common operation; stable admission receipt |
| `game_new` | Exact `SharedOpenToken`, native game and human side; native replacement admission |
| `game_inspection` | Native read-only complete movement inspection with exact selection authority |
| `game_file` | Save/Save As/Open through native pickers, bounded worker validation and atomic replacement |
| `game_request_close` | Request native unsaved confirmation and stop the active game without inventing a result |
| `game_cancel_close` | Clear confirmation while leaving stopped execution stopped |
| `game_finish_close` | Approve only the complete current `SharedOpenToken`, after command/file reservations release |

Chess keeps current/intended claims, match command history, ordinary read-only
analysis, cooperative Full proof/Unknown/cancellation/failure, preferences,
telemetry, unresolved outcomes and Take back. Its actual observations and native
MatchRecord are available through the current snapshot, not empty placeholders.
Shogi keeps native drops/promotions/declarations and advisory eligibility.
Xiangqi keeps exact event prefixes, WXF warnings/obligations/claimants/offers,
fast/exact proof provenance and explicit Unknown. Routine presentation never
starts exact graph analysis. No frontend rules or common outcome reinterpretation
is introduced.

## Native lifecycle and file guarantees

`ApplicationLifecycle` is owned by the controller; the host holds a shared
handle. Exclusive RAII reservations cover the complete command waiter or native
picker/file operation. Failure and cancellation release their own reservation.
The host retains technical event-loop shutdown observation, not a second set of
document-operation or close flags.

The native menu, window close, preventable Quit, final Exit and recorded Tao
macOS termination patch retain their cleanup paths and provenance. Close approval
requires the exact current document/version/content revision and no outstanding
operation. Save is permitted during confirmation. Open rechecks close authority
after the picker: an unchanged token alone cannot authorize replacing a document
whose close guard opened while the picker was visible.

The [current document envelope](document-v1.md) and its atomic Save/Open contract
remain unchanged. Failed Save preserves the prior valid file and dirty state;
invalid, oversized, inconsistent, stale or resource-limited Open preserves the
current accepted document. All native histories and observations remain lossless.

## Authoritative resource facts

Capacity and file snapshots carry optional `ResourceFailure` metadata, separate
from bounded technical diagnostics. A native check supplies the category,
measurement basis, actual measured count, separate operation/mandatory reserve, limit,
scope and units it can establish. Combined observation facts retain ordinary and
exact components without pretending each is a separately exhausted allowance.

`measurement: candidate` describes proposed usage, including a rejected257th
observation; it is not a claim that257 observations were accepted.
`measurement: accepted` describes retained current state, such as all256 slots
already occupied before starting work. Unknown facts contain no fabricated
count. Multiple established causes remain explicit; bounded omission is reported.
Native categories are never inferred from a number or phrase in a diagnostic.

Categories distinguish complete encoded bytes, retained observation bytes and
their response reserve, native-record bytes, native
events, controls, ordinary/exact/combined observations, native replay units,
identity headroom, validation budget and unknown causes. Illegal commands,
engine failures and game outcomes remain separate. The16MiB/256-observation/
8192-control limits, native costs and stop/close reserves are unchanged.

R4 consumes these facts in one shared notice/recovery implementation. Until
then the accepted truthful generic fallback remains appropriate whenever the
exact cause cannot be established. Technical Details retain native facts and
diagnostics. Reopening is not presented as a structural-capacity reset.

## Finite frontend forwarder

The existing Chess `App`/`useNativeSession` still consumes the Chess snapshot and
`desktop_snapshot`, `desktop_command`, `desktop_save`, `desktop_save_as`,
`desktop_open`, `desktop_request_close`, `desktop_cancel_close` and
`desktop_finish_close` routes plus `desktop-close-requested`. These are a narrow
current-controller-to-existing-frontend forwarder; they own no second native
session and read no obsolete file format. After successful cross-game Open the
old Chess file response is `null`; the product's native game owner remounts the
new game rather than receiving an invented Chess projection.

Current product close uses `game-close-requested` and the exact-token current
close operations. Development-only verification views use current close authority
too. **MSH-119/R4 deletes the old frontend owner, forwarding endpoints/event and
obsolete Chess frontend DTO together.** Game-native Chess types and genuine
semantic capabilities remain. R3 does not consolidate application layouts or
board interaction controllers.

## Evidence

Source, tests, independent review, exact package identities and actual platform
results are recorded in the MSH-118 task and [completion evidence](../review/native-application-118.md). R6 owns
final integrated Chess/Shogi/Xiangqi packaged GUI acceptance on all three systems;
native tests or package construction do not establish that later criterion.
