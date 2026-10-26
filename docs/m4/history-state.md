# MSH-67 native history and records

Implementation work in progress for `mshogi.shogi.csa36.v1`. The mutual-check
research gate must be resolved before this work can be accepted as complete.
The [independent fixture derivations](history-fixture-derivations.md) distinguish
expected facts from board-only corroboration. Earlier MSH-63–66 research and
reference evidence remains applicable and unchanged.

## Root and identity boundaries

`Position` remains the broad structural analysis API. Its `HistoryOrigin` is
provenance, not an event log: applying local moves to an initial `Position`
retains `Initial`. A game cannot recover those actions from that value or from
an SFEN counter.

`GameRoot::standard()` supplies the exact initial state. `GameRoot::import`
starts a new known segment, with explicit full-material or reduced-material
category and supplied/defaulted counter provenance. It rebuilds the position's
origin at the imported root rather than inheriting an earlier bare position's
origin. Full material means forty pieces across board and hands; reduced
material means fewer. Neither classification proves historical reachability.
Defaulted means next-ply one. Supplied positive SFEN counters remain unchanged.

Live admission requires the nonmoving king to be unattacked. The moving king
may be checked. This is a necessary local condition, not a reachability proof.
Structural diagrams outside it remain available through `Position` analysis;
they cannot silently become live games. Imported root repetition count is one,
and earlier occurrences/checks remain unknown regardless of the counter.

The existing `RepetitionIdentity` compares board ownership/promotion, both hands
and turn, excluding counters and provenance. A `GameContext` instead retains the
exact root and ordered actors/commands. Identical SFENs can have different
contexts and conclusions. Canonical saved state also retains policies, edition,
provenance and derived effects. These are distinct from future search/session/
request/process execution identities. This task introduces no engine adapter.

## Command admission and conclusions

`Game` owns its position, accepted event ledger, every known repetition identity
and every board-action check observation. Its public position and ledger are
immutable views. `GameRequest` supplies the expected complete `GameContext`, one
actor and a typed complete `GameCommand`. Stale contexts, terminal games, wrong
board-action actors and locally illegal actions reject atomically. No rejected
request becomes a tournament penalty or chess outcome. Malformed wire values
are rejected by the strict record decoder; a command cannot carry a second
conflicting actor. Caller authentication belongs to a later transport/host.

The implementation checks no-legal-action loss first, then fourfold repetition
and continuous-check loss, then the declared 320-action cap. Repetition examines
all actions in the first-to-fourth interval of the repeated identity, including
checked and check-relieved endpoints and overlapping intervals. A legal move
that finishes a losing checking cycle remains an accepted event. No-action loss
retains whether the immobilized king is checked. A root at next-ply321 or later
is already terminal by length unless no-action loss has precedence.

`declaration_eligibility(actor)` is advisory and side-effect free. A formal
`DeclareEnteringKing` commits a win or false-declaration loss and retains its
evidence. Own-turn is one of the prerequisites: a correctly addressed formal
declaration out of turn loses, rather than becoming a wrong-board-actor error.
`Resign` is permitted for either named side while live. Both commands preserve
the exact board and counter, add an event, and conclude the game. Further play
or result-changing commands reject. CSA scoring is stored separately as sente/
gote half-point units: win `[2,0]` or `[0,2]`, draw `[1,1]`; a live game is unscored.

Only untimed local play is supported. Its time-remaining declaration prerequisite
is satisfied. Timing, external illegal-move/disconnection policies, chess claims,
dead-position tests and resignation/mating-feasibility adjudication are absent.
Unsupported record mode/policies reject explicitly; they are never downgraded.

The hypothetical mutual-continuous-check case remains a research blocker at
this draft revision. The provisional `UnresolvedMutualCheck` admission error
avoids fabricating a winner but is **not** an accepted selected-profile outcome
or a completed legal-action contract. This branch must not be integrated as a
complete MSH-67 implementation until the proof/interpretation gate is settled.

## Reversal, prefixes and runtime work

`GameUndo` checks the complete current semantic state and restores board,
counters, root, events, checks, occurrences and conclusion exactly. A token for
another state rejects. `Game::prefix` reconstructs accepted event prefixes,
including a pre-declaration/resignation prefix and the terminal prefix at the
same board ply. Semantic undo can restore an identical earlier `GameContext`;
this is intentionally not a monotonic runtime execution generation. A future
host must invalidate pending work through its own request/session/process
identities when changing a session.

## Strict native game/match record

`GameRecord` is the complete rules-owned untimed game and match-command ledger,
not an SFEN restoration shortcut. Schema v1 records root/category/provenance,
known-history origin, immutable profile identity/edition, explicit policies and
limits, ordered commands and actors, derived state/check/declaration evidence at
each event, conclusions and half-point scores. Replay creates the admitted root
and submits every command to `Game`; every derived prefix is compared, not only
the final board. The root's derived conclusion is checked too.

Typed Serde decoding rejects duplicate/unknown fields and requires nullable
fields to be present. Invalid roots/counters/categories/origins, unsupported
editions/modes/policies, noncanonical SFEN/actions, forged effects/conclusions/
scores, and post-terminal events reject. Imported prehistory is never invented.

Input/output is bounded to 1 MiB and a conservative 321 events. Supported live
games admit at most 320 events: at most 320 board actions, or319 followed by a
terminal command; already-terminal roots admit none. A conservative canonical
event upper bound of2048 bytes and envelope/root8192 bytes gives665600 bytes,
below1 MiB. SFEN is bounded by forty pieces, bounded hands and a u32 counter.
Thus ordinary accepted work cannot exhaust this native codec's complete save
path. This is game-owned encoding only; shared archives, engine transports,
console and desktop integration remain later tickets. MSH-61 is unchanged.
