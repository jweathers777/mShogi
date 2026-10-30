# Current desktop document contract

MSH-117/R2 replaces development desktop archive versions1/2/3. The sole supported
current format is **`mshogi.desktop.document.v1`**, `envelope_version: 1`.
This is independent of protocol4 and durable observation schema1 from
[MSH-116](internal-execution-v4.md). Development-era saved documents are rejected;
there is no compatibility reader, fallback, converter or dual writer.

## Envelope and native payloads

Every field is required. Unknown/duplicate fields reject. `game_kind`,
`rules_profile`, `profile_edition` and `native_schema_version` must agree:

| Game | Selected profile | Live native schema |
| --- | --- | --- |
| Chess | `mshogi.chess.match.v1` |1, M2 untimed live restoration |
| Shogi | `mshogi.shogi.csa36.v1` |1, native untimed ledger |
| Xiangqi | `mshogi.xiangqi.wxf2018.v1` |1, native WXF ledger |

`profile_edition` is the exact immutable native `RulesProfile::edition()` value.
`payload` is captured as `RawValue`, never an intermediate map. Its original
nested bytes reach concrete strict parsers and authoritative native replay.
The common writer encodes a compact native payload within the identified outer
JSON. Byte admission counts the actual resulting complete document.

**Chess payload** retains full `match_record_json`, corroborating final
`game_record_json`, human side, settings, play limits, ordered controls with
original timestamps, observations, all abandoned Take back MatchRecord lineages
and their control binding, identity/revision watermarks and `selected_event`.
Native records remain strict parser input strings; a GameRecord cannot substitute
for the MatchRecord. Claims, policies, commands, actors and conclusions replay
natively. `selected_event` selects the exact desktop timeline boundary (initial
native events plus match events); it does not change the live match. It is
reconstructed with native replay before installation. A new runtime selection
identity replaces the old selection token. Timed/live-Full/witnessed match
restoration remains explicitly unsupported. Supported Full analysis evidence
remains historical, including bounded legal witness verification.

**Shogi payload** retains initial/current canonical native records, human side,
initial/current settings, ordered command/observation/close journal, all durable
observations and retired execution floor. The journal retains drops, promotion,
declarations, outcomes, historical viewing and abandoned Take back continuations.
Replay reconciles final records, settings and admitted observations rather than
trusting duplicated conclusions.

**Xiangqi payload** additionally retains native play limits, observation schema,
ordinary and exact observations. Native replay preserves warnings, obligations,
claimant/offer state, conclusions and exact event ordering. An ordinary event
selection can include a warning event that is not a committed execution prefix;
these remain distinct. Exact proof/witness/Unknown/cancellation/failure evidence
retains original source, target, identities, limits, work and provenance. Open
does not run graph search or turn reported evidence into a live outcome; positive
witness paths receive bounded native verification.

## Observation, resource and file guarantees

Durable `ExecutionObservation` schema1 retains full context, exact original
history/purpose/target, perspective, identities, limits and native result. Saved
observations are historical evidence, never resumable pending execution. Native
Take back lineages and observations are retained without truncation.

Limits remain16MiB encoded document,256 combined applicable observations,
8,192 controls and the existing per-game event/replay limits. Admission encodes
the new complete envelope, including retained lineages, worst-width future
identities, historical flags and stop/close reserve. Native canonical-wrapper
limits count actual encoded bytes too. Replay costs remain native-specific;
there is no generic history interpreter or raised constant. Chess reserves an
additional worst-selectable current-prefix reconstruction independent of view
selection in its existing16,000-unit retained-branch budget. Ordinary branchless
Chess retains its native event/response/Full work limits and15-second cancellable
worker budget; the branch-specific16,000 limit is not imposed on supported Full
observations with longer bounded cooperative witnesses. Structural refusal
precedes commitment and preserves the complete save path.

Save still uses a bounded cancellable worker, same-directory temporary file,
flush/sync and atomic replacement. Failure/interruption retains the previous
valid file and dirty state. Open validates off-path before guarded installation;
corrupt/unknown/oversized/resource-limited/stale input cannot replace the live
accepted document. Unsaved replacement and native close guards remain active.
Successful reopen assigns fresh runtime request/session/selection/process
identities with work stopped. Return to live and explicit Resume remain separate.

## Finite runtime installation bridge

R2 owns the single current codec, not the R3 application-owner rewrite. The
existing Chess/Shogi/Xiangqi runtime captures native payloads for the new writer;
validated new-format payloads install through existing native installation
methods. This bridge reads **only the current format**, owns no independent live
state, preserves file/admission/token guards and has no legacy parser.
**MSH-118/R3 deletes this capture/install bridge** when it owns the one active
native document and common metadata. R2 does not start R3 or preserve an old
reader until R6.

## Verification fixtures

`cargo run --locked -p mshogi-desktop-session --example write_document_fixtures --
desktop/scripts/fixtures` reconstructs the ordinary GUI hand-inspection input
from an explicitly reduced-material native root and the independently recorded
`5e5c+`, `4e4g+` transcript. Native assertions establish demotion into hands and
promoted rooks before the actual current writer saves it. The example is not
bundled application instrumentation; no old document is parsed or converted.
Historical MSH-74 input remains unchanged as prior evidence.
