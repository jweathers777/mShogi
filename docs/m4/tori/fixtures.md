# Independent modern fixture specification

`fixtures.json`, schema `mshogi.tori.fixture-spec.v1`, is **declarative input and
independently expected output for the future harness**. It does not execute
or certify an unimplemented Tori kernel. No perft/legal-set count came from an
engine being tested. All39 original seed identities are retained; the historical
bytes remain in `archive/2026-10-31/fixtures.json` with the original hash in its
review/validation. Modern expectations supersede the12 old unresolved/null seeds.

Board entries `s:Fa:4c` mean Sente Falcon at4c; `g` means Gote. Kind aliases
are declared at the corpus top. Hands omit zero counts. A diagram root explicitly
allows missing material, records unknown earlier history and makes no claim of
historical reachability. Standard root has full32-piece inventory. Move commands
are complete typed specs, including required result face; drops identify exact
base kind and destination. These are harness inputs, not a prematurely frozen
wire schema. Future representation may map without altering meaning.

- `exhaustive_local_geometry`: complete movement tables on an otherwise empty
  board, not whole legal positions. Limited rays and jumps are distinct.
- `game_admission`/`rotated_game_admission`: concrete diagram and submitted
  command; accepted/rejected plus named effect or unchanged-state expectations.
  Every accepted transition also requires apply→undo, restoring all
  state/history/caches exactly. This includes
  capture/demotion/hands, drops and forced promotion for both owners.
  Unlisted legal choices are not claimed excluded. Effects are independently
  derived from published geometry/selected clauses. A rotated row is the exact
  180° owner exchange of its named base row; textual proof annotations need not
  be treated as mirrored coordinates.
- `terminal_position`: concrete diagram, exhaustive empty action set plus the
  hand-written escape proof. Initial installation must derive the conclusion.
- `complete_history_sequence`: concrete root and full admitted action sequence,
  exact occurrence/obligation/conclusion/prefix expectations. Forced-check and
  unequal-cycle sequences are complete, not fabricated occurrence counters.
- `history_adjudication_unit*`: explicitly supplied position-key/choice stream
  tests the bounded history algorithm alone. It is not a proof those inputs form
  a legal whole game; complete sequences separately establish that integration.
- `canonical_context_pair`, key/execution/notation/match/strict-dispatch contracts:
  independently specified semantic boundary cases for later rules and shared consumer tests;
  no current parser/schema/capability support implied.

Coverage: all9 faces, both owners, rays/friendly and enemy blockers,
leaps, capture/demotion/handed transfer, compulsory entry/within/exit, zone drops,
no unrelated promotion,2/3Swallow restriction and exceptions, last-rank/drop-mate
with legal defense, royal safety/pins/evasion/mate/stalemate, root-only terminal,
resignation/unsupported endings, third occurrence/departure responsibility,
forced checked/quiet units and actual forced-check full cycle, unequal overlapping
cycles, same board/different history, exact undo/replay/prefix return, identity
retirement and malformed/illegal atomic rejection.

Next rules fixture work is to instantiate these specs in the real kernel and
add legal-set/perft expectations only where independently established. Comparator
source audit is not a substitute. Canonical raw duplicate snippets are targeted
malformed boundary probes; no otherwise valid current Tori envelope is claimed.

## Implementation ownership after MSH-85 refinement

The103 immutable specs now have an explicit [ownership manifest](rules-fixture-ownership.json)
and bounded queue.126 supplies executable infrastructure
for all IDs,127/128/129 implement structural/local/game assertions. Real process
acceptance, desktop preservation and human notation assertions remain explicit
77/42 obligations; a rules-only pass cannot claim those future journeys.

## Executable machinery — MSH-126

The engine-independent Rust harness now loads this corpus
and the full owner manifest. Fixture-data admission passes separately from
assertions: without actual engine observations all605 assertions are pending,
with0executed/passed; four prose proofs are annotations and83 consumer requirements
remain pending. Selected property/mutation/rotation machinery is executable now;
that does not claim legality or replay. No external comparator is required.


## Current local execution — MSH-128

Actual execution is recorded separately in local evidence:
403fields pass across83rows (all14primary127 and69primary128),202history/record/
consumer assertions pending,83later-consumer requirements pending. The four prose
proof annotations are not automatic passes. Local status and legal choices are
implemented; actual full-game conclusions/history/records and real consumers
remain129/85/77/42. No expectation was generated from or external output.


## MSH-129 Game handoff

The game evidence records the actual authoritative
T05/T06 Game, exact request token/retirement/reversal, immutable history and complete
prefix records, strict record schema1 replay and typed core dispatch. All 591 rules
assertions execute and pass (162127+241128+188129),14genuinely later assertions and
83real-consumer requirements remain pending. Bounds are measured separately from later desktop quotas;
ordinary admission preserves complete encoding and reserved resignation headroom.
Tori execution remains explicitly unsupported/unscored and unadvertised; no Tori
session, engine, console, desktop codec/UI or chooser is supplied.85 retains separate
acceptance;77/42 remain real shared/product consumers. Earlier handoff sections
above are historical checkpoints, superseded only as to delivered rules129 behavior.
No later task is started or closed by this evidence update.
