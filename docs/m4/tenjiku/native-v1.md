# Tenjiku native handoff v1 — MSH-87

The public Rust boundary is `mshogi_core::tenjiku`. It implements both immutable
whole profiles ratified by MSH-83: `mshogi.tenjiku.adams1999-modern.v1` and
`mshogi.tenjiku.muller-completed.v1`. Constructors require an explicit `Profile`;
there is no default profile or live rules switch. Native delivery does not activate
shared application routing, the normal game selector, a console or an engine.

## State and actions

`Position::standard(profile)` supplies the source-derived 156-piece, 16×16 setup.
`Square` is a checked one-based file/rank pair. `Piece` retains physical identity,
side, original `Base`, once-only promoted status and original-piece refusal rights;
`Face` is movement, not original identity. There are 36 originals, 44 faces and
30 promotion relationships. Captured/burned pieces are discarded, without hands.

`Position::from_diagram`/`install` validate structure, inventory, unique identities,
promotion rights, provenance and counters. `RootMetadata` distinguishes the exact
standard root from unproved diagrams/imports. Source attribution is bounded to
1,024 UTF-8 bytes. Imported counters do not establish earlier repetition or
reachability. `Game::from_root` additionally requires zero known accepted plies;
a current board cannot masquerade as a new verified root.

`Action` retains kind, origin, **ordered route**, promotion and the explicit target
of stationary igui. Returns and alternative paths remain distinct choices. Travel
cannot include an igui target, even null; igui requires an empty route and no flip.
No action accepts caller-supplied victims. `LocalPosition` owns geometry, complete
local choices, metered inspection, transition and local reversal. Inspection can
query either owner without granting current-turn permission. It does not enforce
Game history; consumers must submit through `Game`.

`Effects` retains mover before/after and ordered `EffectPhase`s: direct captures,
canonical simultaneous burn batches, mover destruction, then surviving promotion.
A batch is sorted by absolute rank then file. Fire Demon hazards resolve at the
completed final position. Promotion into Fire Demon cannot burn that turn or
resurrect a mover killed before promotion. Adams burns enemy bystanders before a
moving Demon dies beside an opposing Demon; Muller suppresses that collateral.
The selected Free Eagle, Tetrarch and ranked-general differences remain explicit.
Full Lion Hawk movement is Lion plus Bishop in both profiles.

## Game authority, history and endings

`GameRequest` supplies the full current `GameContext`, actor and `GameCommand`.
`submit`/`submit_with_stop` admit atomically: a refusal leaves exact accepted state,
history, rights, conclusions and runtime authority unchanged. Contexts bind profile,
root, complete events and instance/revision. Restore creates fresh authority.
`retire_context` and historical `select_prefix` retire outstanding authority;
selection returns a read-only prefix and does not change the live board.

`RepetitionKey` includes profile, side, original base, promotion and placement;
it excludes physical IDs, refusal rights and control counters. The first known
board-plus-turn recurrence is forbidden. `search_identity` retains full root/history
and rights but excludes runtime authority. `GameContext::same_semantics` separates
logical equality from authority. Cross-profile identities never match.

After complete effects, native royal survival has priority. King and promoted
Drunk Elephant/Crown Prince are royals. Exposure is legal; there is no imported
Chess self-check, Chu Lion protection or mate rule. Losing the final royal loses;
both final royal sets disappearing draws. Bare-royal response uses the ratified
finite actual one-action escape, including repetition and complete effects, without
recursive forced-series proof. Both bare roots draw; a bare side not to move loses.
No legal board action alone produces no invented result: `NoBoardActions` is an
unscored status. Resignation and consensual draw commands are native; `Stop` is
administrative and distinct from a game conclusion or engine result.

`events`, `snapshot`, `root_snapshot`, `event_prefix`, `record_prefix` and royal
metadata expose actual history. `GameUndo`/`GameRedo` are instance-, branch-, LIFO-
and single-use-bound capabilities. Undo restores the exact logical prefix and
retires authority. Ordinary replay drops each private compact undo handle; explicit
`to_game_with_undo` returns actual handles. Game retains each required prefix once
through shared immutable snapshots, rather than a full Game copy per undo handle.

## Canonical schema 1

`GameRecord` uses native format `mshogi.tenjiku.game`, game `tenjiku`, explicit
profile and edition, schema 1 and mode `untimed`. It includes admitted root,
ordered complete commands and derived effects, current state and derived root/current
summaries. Runtime authority and work counters are never trusted from wire input.

Strict raw parsing rejects recursive duplicate keys before buffered deserialization.
Unknown, missing, conflicting, unsupported and malformed fields reject. Replay
independently applies every command and compares all supplied derived facts,
including burns and outcomes. Reordered route/effect phases are semantic changes.
Records cannot be reinterpreted under another profile. This is the native record,
not another application document envelope.

## Measured native capacity and cancellation

The native admission policy is **1,048,576 encoded bytes**, **1,024 commands/events**
and **512,000,000 deterministic reconstruction units**. All limits apply together.
Units count native rule work, inspected repetition cells and canonical encoded
bytes per accepted prefix; they are not CPU instructions or elapsed milliseconds.
The work cap does not count every transient allocation/CPU operation in capacity
validation. In particular, exact ending-reserve validation explores eight native
actor combinations and copies event/key vectors; shared consumers must measure
that cost rather than convert units to a latency promise.

Each admission also computes the widest **actual Stop → Resign → Stop** sequence
for both actors, reserving its encoded bytes, events, deterministic work and runtime
counter authority. Existing Stop/conclusion state reduces the necessary reserve.
Near-limit refusal is typed capacity, never an outcome; the prior accepted record
remains saveable. Runtime retirement and undo cannot spend ending headroom.

These are Tenjiku policies selected against actual workloads, not inherited Chu
constants. Both profiles complete 128 native board plies with all 156 pieces,
about 90 kB records and 7.62 million units. A 1,021-event control history is about
194 kB/111.4 million units; the next ordinary command refuses atomically and all
three reserved endings reach 1,024 events and replay exactly. Adams also completes 1,021 board plies (557,612 encoded bytes, 299.3 million
units), replays in 3.264 seconds and completes all reserved endings at 1,024 events.
The full measurement process peaks at 40,288,256 bytes RSS. This supports the
representative journeys and real event-boundary reserve; it does not imply every
possible history reaches 1,024 board plies before another budget refuses it.

Release measurements and process peak memory are recorded in
`native-verification.json`. The reproducible `tenjiku_work` example accepts
`adams|muller LENGTH play|controls|replay|undo`. It reports construction, encoding,
ordinary replay, retained-undo allocation, per-origin inspection, large empty-center
choice generation, complete-action serialization, active burn processing and actual
ending replay. Process RSS includes the entire measurement process and multiple
simultaneously live reconstructions; handle-vector bytes exclude Game history and
allocator overhead. Neither is an isolated allocation count for one Game.

Cancellable APIs check during strict raw traversal, local generation/effects,
repetition-history loops, adjudication, admission and replay. Tests distinguish
`Stopped` from capacity and verify atomicity. A requested checkpoint stop is not
an external watchdog/preemption measurement. Native measurements do not prove
shared worker/lock duration, frontend rendering or the 15-second outer deadline.

Full-Game perft/divide deliberately bounds depth to 3 and visited nodes to 100,000,
further limited by caller budget. It uses real history/adjudication. Cancellation,
capacity and incomplete traversal return typed failure, never a partial exact count.

## Independent expectations and consumer boundary

The MSH-83 corpus/hash is unchanged. Production adapters execute **204 native rows,
621 top-level expected-field assertions per profile, zero failures**. Outer complete
sets preserve nested route/effect ordering. The 129 documentary/property annotation
rows and 17 later consumer rows are reported separately, not counted as native
executions. The 25 Python data-admission tests remain source-package validation.
Additional native adversarial, sequence, reversal, strict replay, counter, rotation
and bounded perft tests are separate evidence. Source-derived perft is 1/4/16 with
four divide entries of four. No external engine is a prerequisite.

`tests/tenjiku_consumer.rs` compiles outside the module and consumes public types
for both profiles, actual dying-Demon differences, complete effects, native records,
strict forged-fact rejection, inspection, prefix selection, undo/redo and fresh/stale
context handling. Its small test payload is not a production document schema.

MSH-79 owns explicit selected-profile New Game plumbing, typed native dispatch,
shared execution/session/controller, native-derived observation and complete-choice
projection, whole-document record/branch/observation admission, ending/Close reserve,
installation/cancel races and frontend interaction. Preserve profile in every path;
board dimensions and side names cannot infer it. Project off-route burns and multiple
royals directly from native effects/metadata without frontend legality. Measure
aggregate retained bytes/work and per-origin lock/serialization costs with real
histories; native budgets alone do not admit a desktop document. No normal selector
activation belongs to this handoff.

MSH-44 retains functional history-aware search/evaluation, console and complete
notation, actual engine journeys, traditional-kanji/promoted-face assets and final
16×16 playable presentation. M5 playing strength and final six-game/platform
acceptance remain separate. MSH-87 stops at native delivery.

## Acceptance revision

Native implementation `5786e59`, with LF-checkout correction `9749eec`, is
independently accepted under MSH-87. The exact latter source passed every required
Ubuntu24.04, macOS15 and Windows2022 step in the
[Rust campaign](https://github.com/jweathers777/mShogi/actions/runs/41816309261).
The verification record preserves the initial Windows checkout failure and the
known MSH-112 Ubuntu runner race before its unchanged-source retry. MSH-112 is
unfixed; a passing retry does not establish a correction. Documentation-only
closure reuses the tested source and its measurements. MSH-79 is Ready and
unstarted; MSH-44's complete playable delivery remains open.
