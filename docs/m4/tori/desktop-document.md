# MSH-130 — immutable Tori desktop document and admission

Selected profile `mshogi.tori.modern.v1`, edition `MSH81-MODERN-2026-11-01`.
Builds on [accepted native production](native-acceptance.md)
`994521bec0637d6f8f17103f1106c0025d950753` and shared refinement
`1e51ea51b3ba6997b75a7c11a25df0c7f4eeaad5`. This supplies a codec and immutable
candidate, **not an installable live Tori session or playing engine**.

## Current format and authoritative semantics

The existing `mshogi.desktop.document.v1`, `envelope_version: 1` remains the
only current desktop envelope. Its explicit game/profile/edition/native schema1
routes to a concrete Tori payload; no second archive format, migration adapter
or obsolete development reader. Shared execution remains transport4 and durable
observation schema1. Existing current Chess/Shogi/Xiangqi formats are unchanged.

`crates/mshogi-desktop-session/src/tori_document.rs` provides immutable
`ValidatedToriDocument`, native `ToriSettings`, ordered `ToriControl`, measured
`ToriDocumentUsage` and typed `ToriDocumentError`. Private payload fields prevent
mutation bypassing admission. Its public accessors expose initial/current native
records, human, settings, controls, retained branches, observation evidence and
retired identity floor for MSH-131. It holds no runtime or game process.

The payload preserves:

- `mode: untimed`, initial/current **native canonical envelopes**, Sente/Gote
  human identity, initial/current native orientation settings. Engine defaults
  are deliberately absent while no actual Tori engine exists.
- Full native roots/provenance/immutable declared counters and all ordered native
  events, effects, repetition/departure obligations, conclusions and incomplete
  reasons inside authoritative native schema1. No FEN/diagram/move-only restore.
- Ordered Native commands, Settings, Rewind (Undo/TakeBack intent), Observation
  index and Close controls. Native non-board commands do not advance board ply.
  Controls' relation to the native ledger is proved by actual replay, not trusted
  serialized derived values.
- Every retired branch's complete native record and the exact rewind control
  index. Reconstruction derives the requested event prefix; saved before-state
  must equal the authoritative journal state. No abandoned observations or
  native events are silently discarded.
- Existing `ExecutionObservation` values with exact native history, purpose,
  target/event/board-ply/selection, request/session/process identities and limits.
  Only the current family's actual unscored Failure results are supported for
  Tori. They are historical evidence, never pending/resumable work. No invented
  Tori score/result/analysis family or successful execution capability.
- Retired execution high-water evidence, including historical selection IDs.
  This is not authority to restore an old runtime. MSH-131 must issue fresh IDs,
  stop work on reopen, and check exact current receipt/snapshot tokens.

`new`, `with_control` and `with_observation` create independently validated values;
refusal leaves the accepted value intact. These are codec candidate operations,
not live actor authorization, view selection or a second session. MSH-131 owns
human turn/Take back eligibility and all runtime command admission/retirement.

## Strict parsing and replay

Input16MiB bound and a recursive duplicate-preserving visitor precede tagged enum
buffering. The current envelope retains RawValue bytes. Shape-only preflight
counts raw command kinds and contexts before expensive native parsing; it does
not trust saved `replay_work`. Inner canonical game/profile/edition/schema/version
must match Tori; no fallback into another game's parser.

Each native canonical record then reaches its authoritative parser. Cancellation
is checked between native records and observations; each individual native call
is bounded by its own512KiB/512events/65,536work policy and is not internally
interruptible. MSH-130 changes no native rule, record parser or conformance input.

The document journal uses `tori::Game::submit`, `record_prefix` and fresh native
reconstruction. Current/settings/retired ledgers must match replay exactly.
Observations must match the current family's context validity and encoded limits,
be an exact prefix known **at their journal point**, and remain unscored. Wrong
history, future prefix, unsupported Full analysis or wrong game/result rejects.
All request/session/process/selection IDs must be positive and covered by the
safe saved identity floor. No clock/pending intent is silently restored.

Tests exercise all seven original TORI-REJECT cases through document admission,
raw duplicate/unknown/missing/conflicting outer and nested fields, unsupported
mode and commands, corrupt/truncated/oversized data, forged intermediate effects/
conclusions/work and contradictory root counters/provenance. These are **pure
codec** rejection checks; the thirteen pending actual runtime assertions remain
MSH-131's obligation.

## Measured capacity policy

Existing desktop limits remain16MiB encoded file,4,096retained native events,
8,192controls and256observations. Native limits remain512KiB per record,512events
and65,536native units. Encoded execution requests/responses retain1MiB/2MiB bounds.
No limit was raised. Native event counts include all encoded initial/current,
retired and observation records, even repeated prefixes; controls separately count
all durable journal entries.

The **new Tori whole-document budget is262,144 deterministic validation units**.
It is distinct from the native per-record quota and existing Shogi/Xiangqi16,000
policies. This permits the measured maximum ordinary360board native history,
retained real branches and a complete full-history failure observation, while
refusing another repeated full history before expensive replay. It is a supported
bounded policy, not a claim that any native record combined with arbitrary
observations must fit. Measured maxima/refusals appear in the evidence below.

Preflight charges:

1. Each native parser invocation:128root-admission units, plus one per event and
   the actual number of prior board keys for each Play. The latter is derived
   from raw command categories, independent of forged cached work. Stop/Resign
   still cost events.128is conservative fixed root/serialization overhead, so
   zero-history observations cannot escape the budget.
2. Initial parsing **and** initial reconstruction; current parsing; every retained
   and observation record's native parsing. Native limits are checked for every
   record before replay, not only the live current record.
3. One per journal control, plus native board admission charges for each Play.
   The initial board count is included. Rewinds conservatively charge a full
   native65,536allowance plus256root/prefix overhead, and do not reset the
   running board-count upper bound. This overcharges rather than undercounts
   arbitrary replay/branch work.
4. An extra128per observation for exact native prefix projection. Each context
   is parsed even when its diagram equals another; native identity retains history.

This budget is complemented by the **existing15-second FileWorker budget**.
Timing observations on one host are not performance guarantees. The worker can
cancel between native calls and suppresses retired completions; an individual
bounded native replay may finish before cancellation is observed. Candidate
construction itself can perform another bounded native reconstruction; consumers
must run expensive preparation/validation on the existing worker, not the UI path.

### Reserved endings and exact boundaries

Before ordinary admission, reserve the **remaining** operations:

| Remaining operation | Encoded desktop reserve | Whole-document work reserve | Native reserve |
| --- | --- | --- | --- |
| Native Stop |8,192B |512units |1event,1native unit,4,096native bytes |
| Native Resign |8,192B |512units |1event,1native unit,4,096native bytes |
| Desktop close |1,024B |16units |No native event |

Reserve one control per remaining operation. Stop consumes its allowance; an
already concluded game needs no Stop/Resign; close consumes its control allowance.
Closing a still-live native game does not erase native concluding headroom.
Native4,096B/event uses the already reviewed bound; outer8,192B covers native
record growth plus the pretty command/control encoding. Actual growth and work
are checked against these allowances. The byte/work reserve decreases as each
operation commits; a fixed unchanged boolean reserve was rejected during review.

At every supported boundary, refusal returns authoritative `ResourceFailure`
facts (`scope: tori`, category/count/limit/reserved/unit, candidate provenance).
An unmeasured native counter refusal has no invented count. Resource exhaustion
is not an illegal move, chess-family conclusion or successful/failed engine turn.
The shared frontend displays existing truthful capacity language, including
“Your current game can still be saved”; diagnostic strings are never parsed.

Tests cover actual8192control and4096retained-event boundaries,256→257observations,
native512KiB input and plus-one, native replay/event precharge, identity headroom,
complete encoded input ceiling, and independently isolated exact scalar byte/work
limits using actual ending growth. The16MiB padding probe verifies the encoded
**input** bound; it is not a claim that a semantically valid stored document can
reach16MiB before another quota. Canonical encoding removes irrelevant whitespace.
The measured reachable long-document replay refusal is separately documented.

## Atomic file and worker ownership

`ToriDocumentWorker` is a thin pure-codec adapter over the **existing FileWorker**,
`Work::Native` and `NativeCompleted` family. It introduces no active document,
application runtime, process supervisor or second worker implementation. It
validates/saves immutable candidates and returns either a validated value or the
existing Saved receipt with exact path/revision/document identity. It never marks
a live document clean. A runtime must compare the receipt with its current exact
identity and content revision; old/failed/cancelled work grants no authority.

Save uses existing exclusive same-directory temporary creation, checked chunks,
flush/sync and atomic rename. Rename is the commit point; a successful committed
save cannot become a failed receipt merely because scheduling crossed a budget.
Interrupted/precommit failures preserve the old valid file and remove temporary
output. Failed validation produces no Saved receipt. Cancelling/replacing a worker
job retires its result; stale operation IDs reject. Reopen is pure decode only.

`OpenedNative`, `ActiveDocument`, sole `install_document` and normal Tori New Game/
Open remain unsupported and unchanged in ownership. The explicit Open diagnostic
now clarifies that **live installation** is missing, rather than denying this
codec. Normal games, product chooser, renderer and frontend session owner are
unchanged. MSH-131 must wire this payload into the sole runtime and perform actual
live-document/unsaved lifecycle/stale-result verification; MSH-132 then completes
presentation. MSH-42 retains engine/console/notation/final board/playability.

## Verification, review and handoff

Actual commands, source hashes, measurements, acceptance mapping, independent
review and CI are recorded in [codec evidence](desktop-document-evidence.md).
No Tori packaged GUI campaign is required or claimed. Test-only fixture loaders,
measurement output and scenarios are excluded from normal distributables.
The immutable103fixtures/713native assertions and all14pending/83consumer
obligations retain their existing ownership and history. This task supplies codec
support, not pretend passes for the actual live installation still owned131.
