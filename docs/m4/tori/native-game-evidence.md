# MSH-129 — authoritative native Tori game and handoff

Selected `mshogi.tori.modern.v1`, edition `MSH81-MODERN-2026-11-01`.
Builds on reviewed [127 structural](native-structural-evidence.md) and
[128 local](native-local-evidence.md) behavior. The profile, all103 independent
fixture specifications and ownership bytes are unchanged. This is native kernel
and routing evidence, not Tori engine, console, archive, desktop or strength delivery.

## Implemented native contract

`mshogi_core::tori::Game` owns the admitted root, complete ordered accepted
commands/events, position, repetition ledger, obligation, conclusion and immutable
prefix snapshots. `LocalPosition` remains the single local legality/effects/inspection
owner; its choices remain distinct from a concluded or history-restricted Game's
permitted choices. No frontend, rules framework or alternative execution owner added.

Each accepted command produces one atomic native `GameEvent`, carrying its actor,
complete command, derived board effects (null for non-board work), exact event index,
known board ply, current-key occurrence list, obligation, conclusion, incomplete
reason and replay-work charge. Root terminal classification is recorded separately;
root installation is not an invented board move or participant command. Native
conclusions are Checkmate, StalemateLoss, RepetitionForfeit and Resignation. No draw,
engine failure or resource refusal is fabricated as a native game result.

- T06 identity contains every native owned face, both handed inventories and turn;
  declared counters/provenance/runtime do not enter it. Full saved context retains
  those additional semantics. The admitted root is occurrence1 (index0), including
  imported diagrams whose earlier history remains unknown.
- On the third or later occurrence, use the latest three indices. Responsibility
  belongs to the side moving **out** of that key. Deduplicate the actual complete
  first departures at the first two indices. Arrival is accepted; a forbidden
  departure rejects atomically. An allowed departure consumes the old obligation;
  its destination may install a new obligation. Local mate/quiet immobility is
  assessed first; only nonempty local choices entirely forbidden by repetition
  cause a repetition-forfeit loss. Checking/evading creates no exemption.
- Complete Play requires native turn and local legality plus history permission.
  Resign permits either actor while live. Agreement/impasse are explicit unsupported
  requests; concluded/stale/illegal requests reject without any semantic/runtime event.
- Stop records Cancelled/ResourceLimit/EngineFailure/WorkLimit as incomplete/unscored,
  preserving board ply, turn, occurrence ledger and obligations. Duplicate Stop
  without intervening board work rejects. A later context-checked board action clears
  the incomplete marker; Stop does not authorize starting/resuming an engine. Actual
  host pause/Resume and process scheduling remain MSH-77/42. Resignation can follow
  Stop using its reserved slot.

`GameContext` compares the complete root and ordered events plus a unique checked
native instance and monotonic revision. Runtime tokens never enter records. Failed
requests preserve authority; accepted commands, undo/redo and explicit selection/
retirement advance it. Instance-bound single-use LIFO undo restores every logical
field/cache; redo recomputes the same authoritative transition and issues fresh undo.
New commands retire old redo branches. Foreign/used/out-of-order tokens reject.

`event_prefix` is immutable historical data with native local inspection;
`select_prefix` also retires native command authority. `record_prefix` exports the
complete root/event prefix for historical analysis without future events or live
mutation. It is not an implicit live branch. Reconstructing any canonical record
creates fresh authority. Actual request/session/process/purpose/limits acceptance
remains later consumer ownership, not a second core execution framework.

## Strict record and typed consumer

Native format `mshogi.tori.game`, game `tori`, native schema1, mode `untimed`.
Rules identity/edition, native schema, shared envelope1 and engine transport4 remain
separate. Standard, diagram and imported roots encode exact board/hands/turn,
source, declared counter, missing-history/reachability facts and accepted counter.
A played LocalPosition cannot silently become a new root; explicit root creation
is required. Standard identity cannot relabel a reduced/altered diagram.

`GameRecord` is constructed only from an admitted Game or strict authoritative
replay. A bounded recursive visitor rejects duplicates **before** any map or
internally tagged enum normalization. Typed `deny_unknown_fields` decoding then
replays every command and compares every event's effects/indices/occurrences/
obligation/conclusion/incomplete/work facts. Final whole-record equality also
rejects omitted nullable fields, forged root/current state or missing provenance.
Object key ordering/whitespace may differ; canonical board/event ordering may not.
Rejected records return errors without mutating an existing Game. Diagnostics are
bounded enums; the existing outer diagnostic cap remains4096B.

`native.rs` adds Tori GameKind/RulesProfile/NativeSchema/NativeSide, real
CanonicalRecord RawValue routing, and typed NativeCommandTarget/Command/Reply.
The actual core consumer uses native legal actions/effects/inspection, imported
root/full-history/context/terminal roundtrips and exact prefixes. Wrong outer or
native game/profile/edition/schema never falls through to another game. Existing
Chess/Shogi/Xiangqi native schemas and records are unchanged.

Minimal exhaustive consumer adaptations explicitly reject Tori desktop installation
without replacing the live document. No chooser, Tori session or desktop codec is
added. `execution_capabilities(Tori)` is empty. Bounded ordinary requests are
parseable so the real internal server/subprocess returns unscored UnsupportedMode,
with exact context and no move, score or game result. No Tori Chess Full/Xiangqi
exact operation or playing capability is inferred. Existing transport/queue/
cleanup/cancellation limits are unchanged.

## Measured native persistence admission

Native caps: **512KiB actual encoded bytes,512 commands/events,65,536 replay units**.
One event equals one accepted command. A replay unit accounts for root admission1,
then command1 plus each prior key compared by a board action; non-board commands
charge1 and do not scan/add keys. Local board work remains finite7×7 complete native
validation, with at most32 pieces,6 hand kinds and49 endpoints. These units bound
history/replay work; they are not CPU milliseconds, engine nodes or a search score.

Admission constructs the full candidate, measures actual complete encoded JSON and
checks all caps before installation. Ordinary live Play retains **two events,
8192 bytes and two units** for Stop then Resign. Stop retains one event,4096B and
one unit; a native conclusion can use the remaining headroom. A constructed
conservative metadata bound (all513 possible small indices plus two longest
complete departures, event and changed current nullable fields) proves each
non-board reservation fits4096B. This is a storage-bound proof, not a fabricated
reachable conformance position. Capacity errors name EncodedBytes/Events/
ReplayWork/Counter and are unscored; no history or observations are discarded.

Actual macOS arm64 debug measurements (fixed seed `0x129ca11ab1e5`):

| Real record / admission | Board actions | Events | Bytes | Replay units |
| --- | ---: | ---: | ---: | ---: |
| Ordinary long record |360|360|156573|65341|
| After reserved unscored Stop |360|361|156804|65342|
| After reserved Resign |360|362|157044|65343|
| Alternating Stop/Play at ordinary event boundary |255|510|165461|33151|
| After Stop511 and Resign512 |255|512|165930|33153|

The360-action sequence contains104 captures,103 drops and16 promotions. The next
ordinary action would exceed replay admission; it rejects with identical bytes,
context and caches. The510-event case rejects further ordinary Play specifically
as Events, then actually accepts/replays Stop511 and Resign512. Every long-record
prefix is reversed/redone with exact full native cache restoration. Actual parser
input at512KiB (valid canonical record plus whitespace) passes; one byte beyond
rejects. Exact scalar policy boundaries separately challenge bytes/events/work
with zero/one/two reserved slots. Malformed over-count event arrays reject before
native replay. Declared `u64::MAX` counter refuses Play explicitly while still
allowing complete serialization, Stop and Resign; it does not manufacture mate.

The512KiB cap leaves more than the existing shared wrapper/request overhead under
its unchanged1MiB bound. Measured real histories are ~153–162KiB; work/event limits
bound replay before an arbitrary multi-megabyte ledger. Timings in the compact
[evidence](evidence/native-game-fixtures.json) are observations of this machine and
build, not promised rerun performance. MSH-77 must measure its actual outer document,
controls, historical observations and stop/close headroom and retain MSH-61's
complete save path,256→257 refusal and atomic Save/Load. These native bounds do not
silently become desktop quotas or a desktop file format.

## Conformance and actual checks

The unchanged103-ID corpus now has **713 executed /713 passed /14 pending /0 failed**:
202MSH-127 +281MSH-128 +230MSH-129. Every in-scope native assertion passes; remaining
13MSH-77 and1MSH-42 assertions and83 separately named real-consumer requirements
stay pending. The standalone data-only CLI still reports0executed/727pending;
it has no native adapter. Assertion counts are not distinct game positions.

Full forced8 checks the sole local defense4a→5a while Game concludes repetition
forfeit. Full unequal16 checks actual prefixes10 and16, latest[4,10,16], forbidden
4f/3f and formerly forbidden5g now allowed. Explicit-choice units exercise the same
production obligation algorithm without pretending those inputs are reachable
whole games. All40 accepted board fixtures plus2resignations execute full Game
apply→undo→redo and retirement in addition to preserved structural/local evidence.
Properties supplement independent expectations: native180° owner rotation of both
full cycles, full-prefix replay/reversal, same-board/different-history, wrong/used/
foreign/retired authority, malformed/duplicate/missing/unknown/tampered inputs,
terminal roots and honest incomplete/capacity behavior.

Reproducible commands:

```sh
MSH_TORI_129_REPORT=/tmp/tori129-assertions.json MSH_TORI_129_CAPACITY_REPORT=/tmp/tori129-capacity.json cargo test --locked -p mshogi-core tori:: --lib -- --nocapture
cargo test --locked -p mshogi-core --test native_tori_envelope --test native_envelope --test native_xiangqi_envelope
cargo test --locked -p mshogi-core -p mshogi-conformance
cargo test --locked -p mshogi-engine
cargo test --locked -p mshogi-desktop-session
cargo test --locked -p mshogi-console --test tori_internal --test internal --test shared_internal --test xiangqi_internal --test shogi_engine
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Local results and source hashes are recorded in the compact evidence. Two existing
release-only deep Chess opt-ins remain not run; no Tori ignored/stub assertion.
An initial adapter projected checkmate winner instead of the fixture's equivalent
loser; projection corrected without changing native result or expectations. Initial
compilation/lint issues were corrected before final checks. One local session
rustdoc failed E0463 finding its compiled engine crate during concurrent builds;
all201 session tests passed, and the focused serial doctest rerun passed0tests.
That transient build failure is retained as a failure, not silently erased.
No external engine or GUI campaign was performed or needed.

## Review, integration and remaining boundary

Independent read-only `tori129_review` inspected native rules/fixtures/diff and
challenged T06, exact replay, context/reversal, strict parsing, headroom and minimal
unsupported consumers; independently executed focused suites. The separate
`tori129_fixtures` read-only review audited the immutable profile/expected fields
before implementation. Final source, reviewer disposition and three-platform native
CI references are recorded below after actual integration. No pass is inferred
from a build on a different platform.

MSH-85 remains separate native-handoff acceptance. MSH-77 remains real execution/
session/document/presentation integration; MSH-42 remains functional engine,
console/notation/desktop and full playable delivery. MSH-51/M5 owns measured strength.
No later task starts and M4 remains open.

Independent final review PASS: `tori129_review`, separate read-only agent/session,
not the author's self-review. It independently ran13native game tests (including713
assertions),3current Tori envelope probes,10existing envelope compatibility tests,
2engine routing tests and1actual internal subprocess test; no unresolved material
finding. It checked immutable profile/corpus/manifest hashes and diff hygiene.
Review challenges added real event-limit admission, full long-sequence redo and
complete-prefix evidence. A minor public view invariant was corrected by keeping
HistorySnapshot's admitted state private behind an immutable getter, preventing
external construction of an invalid view that could panic in local inspection.
Reviewed `game.rs` SHA256:
`a3987b9276971f424f191ff28d9f4ee979bdedae604afb546c159e170d0c92c6`.

Local final focus:38Tori unit tests PASS;3Tori envelope tests PASS; all-target
workspace Clippy `-D warnings` and format PASS. Existing41engine checks,201desktop
session tests and19selected actual subprocess checks PASS, with the separately
recorded initial rustdoc artifact failure/focused rerun above. Metadata scope/link/
Backlog doctor checks PASS:129task records,344dependency edges, unchanged dependencies,
completed tasks, other plans, profile/corpus/manifest, existing game rules/search/
runner/frontend/workflows and dependency manifests. No new task/worktree or unrelated
production changes. Native CI is required at the actual integrated source before
MSH-129 can be marked Done.

Final core/conformance regression run:278checks PASS across34reported suites,0failed;
2existing deep Chess opt-ins not run. No production changes after the independent
reviewed source hashes.

First reviewed native source integrated/pushed at
`e1cc89d454db5e0ce98da34bf4cda7960d395413`; nativeCI41343664138 and automatic
Desktop packages41343664148 started. A post-workspace scan found the separately
built Tauri host's exhaustive save-filename match also needed an explicit Tori
unsupported arm. The additive correction returns before a picker/save queue and
adds no UI/document capability. Independent reviewer follow-up PASS; local
`cargo check --locked --manifest-path desktop/src-tauri/Cargo.toml --all-targets`
PASS, with20existing vendored Tao warnings. Core/engine review hashes are unchanged.
The corrected integrated source and actual finalCI disposition are recorded below;
no success is inferred for the earlier package run or a different platform.
