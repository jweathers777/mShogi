# MSH-86 native Chu consumer contract

Selected rules remain **`mshogi.chu.jcsa2019.v1`**, edition
`JCSA-2019-09-14+2004-06-06+MSH82-2026-11-02`. The immutable MSH-82
[profile](profile.md), [expected corpus](fixtures.json) and audit hashes are
unchanged. Actual native verification is recorded in
[native-verification.json](native-verification.json), separately from MSH-82's
source/data validation. This implements no engine, console, desktop or shared
transport dispatch; MSH-78 and MSH-43 retain those obligations.

## Game-owned API

`mshogi_core::chu` owns `Side`, bounded `Square`,21 original `Base` kinds,
18 once-promoted origin states/29 `Face` movement identities, `Piece`,
`RootMetadata`, structural `Position`, complete `Action`, `Effects` and
`LocalPosition`. Captured victims retain owner/base/promotion and are discarded;
there are no hands, drops or demotion.12×12 uses square geometry and native
Sente/Gote orientations. A structural diagram is not proven historical play.

`Action { from, route, promote }` contains one or two actual landing squares.
A direct jump has one landing; two-step captures have two; a quiet/capture return
has a real pivot and returns to its origin. Distinct paths with the same endpoints
remain distinct native choices. `Effects` derives ordered `Capture { square,
piece }` entries, original/resulting mover and old/new Lion protection. Clients
cannot supply victims or commit partial routes. Public checked admission refuses
malformed, obstructed, friendly, wrong-owner, illegal-promotion or Lion-law choices
atomically. Local undo/redo is instance-bound, LIFO and single-use.

`LocalPosition` supplies complete local choices and immutable opponent inspection,
including native ordered routes/effects. `capture_influence`, `preview_for` and
geometric defense queries are read-only observations, not live command permission.
The private counterless simulation exists only for finite rules adjudication;
it cannot commit a participant action or become a persisted branch. Real apply
always checks and advances counters. This prevents numeric exhaustion from being
misread as absence of a mate escape or immediate material response.

`Game` owns full relevant history and `GameRequest { expected, actor, command }`.
`GameContext` binds exact root/rights, ordered accepted events and fresh native
instance/revision. `Play` validates board turn; resignation, actual draw offers/
acceptance/refusal and typed `Stop` controls have their own actor semantics.
Only opposing consent concludes an offered draw. A stop records cancelled,
resource-limited, failed-engine or incomplete work without a game result.

Game choices apply native history restrictions in addition to local rules.
There is no Chess self-check filter. Outcomes are actual last-royal capture,
finite immediate-threat mate, narrow JCSA material award, resignation or agreed
draw. Quiet no-action is unresolved/unscored. The finite mate algorithm considers
complete replies and a single opposing action capturing all surviving royals;
Prince creation can escape separate individual attacks. It never calls mate
recursively or performs unrestricted forced-mate search.

Repetition keys include original/promoted placement, owner, turn and Lion rights.
They exclude counters/provenance/runtime/derived obligations. Full search/saved
context retains those distinct semantics and all known history. Witnesses retain
three board-boundary occurrences, responsible sides/category, cycle progress and
prospective closing mover. Admission rejects the responsible side's last matching
continuation before fourth; no fourth event/forfeit is invented. Equivalent empty
return routes do not evade the obligation. Check, attack/discovered attack,
first actual jitto streak and fallback responsibility retain the selected priority.
Non-board events do not add board occurrences or reset cycle history.

`event_prefix` and `record_prefix` preserve exact event prefixes and rights with
no future knowledge. A historical snapshot cannot submit a live command.
`select_prefix`, retirement, semantic undo/redo and record reconstruction cannot
revive retired runtime authority. Logical state/caches restore exactly while
runtime revisions advance. Outer request purpose/process generation/execution
limits remain the existing shared owner's responsibility in MSH-78/43.

## Native record1 and bounded admission

`GameRecord` is strict canonical `format:"mshogi.chu.game"`, `game:"chu"`, the
exact profile/edition, `schema_version:1`, `mode:"untimed"`. Native schema is
independent of any outer transport/document version. It stores validated root
structure/provenance/counters/explicit Lion rights, full command/event order,
actors, complete routes, ordered effects, occurrences/obligations, offers,
conclusions, stopped-work annotations and current derived state.

Recursive duplicate rejection operates on raw JSON before serde enum buffering
or `Value` maps. Unknown, missing, conflicting, forged or unsupported data rejects.
Replay executes every command through the native Game and compares each derived
event and final summary; serialized conclusions/rights are never trusted.
Relabeling a custom root as proven standard play rejects even if all duplicated
metadata is forged consistently. Standard root identity requires the exact native
starting state and empty prior Lion rights. Runtime/reversal handles are absent;
reconstruction assigns fresh native authority. RawValue serde preserves nested
duplicates for this parser. No new application document format exists.

Current native policy, distinct from later desktop admission:

- 512KiB actual encoded native record; strict action/command syntax is4KiB.
- 512 native events, with two events reserved from ordinary admission.
- 4,000,000 deterministic replay units: root/threat candidate work, per-command
  work, complete reply work and squared board-history scan budget. These are
  bounded accounting units, not CPU milliseconds or a game result.
- Checked64-bit declared/accepted counters and monotonic native identities.

Before commitment the entire candidate record is encoded. Ordinary admission
also encodes the actual worst Stop(Sente)+Resign(Gote) continuation, including
variable-size obligations, decimal indices and derived summaries. It reserves
those bytes, two event slots and work units. Capacity is a typed native resource
refusal, separate from illegal actions and conclusions; prior accepted state is
unchanged and remains completely encodable. No history is discarded and no limit
is reset by replay. Save/Open/observations/Close are not implemented here:78 must
measure and reserve the complete outer document, including its own controls.

Measured debug/macOS probe:227 board events,132,199 encoded bytes,3,979,825 replay
units before the next move's work refusal; parse plus a second reconstruction
about2.5 seconds locally. These are observations, not portable timing promises.
Another test reaches510 ordinary control events and still records Stop+Resign
at512. Exact512KiB padded input accepts; one extra byte refuses. Three overlapping
witnesses require1,277 closing bytes in the concrete probe; admission computes
actual bytes rather than assuming that sample bounds every record.

## Executed conformance and remaining consumers

The actual production adapter executes all240 native rows: **1,076 checks pass**,
zero failed/pending native assertions.35 source/derivation fields remain documentary;
two additional Lion explanation strings remain documentary while their native
`LionLaw` classifications are checked instead. Thus37 annotations are not claimed
as literal native output. Ten shared/product rows remain explicitly pending78/43.
The frozen corpus's `native_conformance_executed:false` describes MSH-82's data
artifact, not a claim that this later execution did not occur.

Complete Lion88/Soaring Eagle41/Horned Falcon42 action sets preserve all171 routes.
Independently established perft/divide7/42 passes.583 real local choices exercise
valid180° owner rotation, ordered effects and exact apply/undo/redo. Game tests
add deterministic sequences, caches/replay/branch retirement, strict mutations,
wrong actors/contexts, history prefixes and capacity boundaries. Actual external
core tests consume real typed choices, two captures, tagged records, imported
provenance and fresh/stale contexts through raw payloads. No reference engine or
frontend rules implementation supplies expected answers.

**MSH-78 next:** explicit Chu GameKind/profile/schema/record/side dispatch;
consume these stable native records through duplicate-preserving native routing;
one existing document/controller/runtime/frontend owner; full royal/obligation
metadata; real native compound interaction/inspection/history; measured outer
saveability and stop/close headroom. Preserve four existing games. Keep execution
explicitly unsupported/unscored until43 supplies actual engine behavior.

**MSH-43 remains open:** bounded full-history functional search/evaluator,
console/native notation, finished traditional-kanji12×12 board in the accepted
shell, actual playable product and appropriate package/platform journeys.
Strength52/M5, shared46/47/88/89 and diagnostic107 remain separately owned.
