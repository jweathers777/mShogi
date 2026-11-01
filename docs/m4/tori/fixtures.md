# Independent modern fixture specification

`fixtures.json`, schema `mshogi.tori.fixture-spec.v1`, is **declarative input and
independently expected output for the future native harness**. It does not execute
or certify an unimplemented Tori kernel. No perft/legal-set count came from an
engine being tested. All39 original seed identities are retained; the historical
bytes remain in `archive/2026-10-31/fixtures.json` with the original hash in its
review/validation. Modern expectations supersede the12 old unresolved/null seeds.

Board entries `s:Fa:4c` mean Sente Falcon at4c; `g` means Gote. Native kind aliases
are declared at the corpus top. Hands omit zero counts. A diagram root explicitly
allows missing material, records unknown earlier history and makes no claim of
historical reachability. Standard root has full32-piece inventory. Move commands
are complete typed specs, including required result face; drops identify exact
base kind and destination. These are harness inputs, not a prematurely frozen
native wire schema. Future native representation may map without altering meaning.

- `exhaustive_local_geometry`: complete movement tables on an otherwise empty
  board, not whole legal positions. Limited rays and jumps are distinct.
- `native_admission`/`native_admission_rotated`: concrete diagram and submitted
  command; accepted/rejected plus named effect or unchanged-state expectations.
  Every accepted transition also requires apply→undo→redo, restoring all native
  state/history/caches exactly and reproducing the reviewed effects. This includes
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
  independently specified semantic boundary cases for later native/shared tests;
  no current parser/schema/capability support implied.

Coverage: all9 native faces, both owners, rays/friendly and enemy blockers,
leaps, capture/demotion/handed transfer, compulsory entry/within/exit, zone drops,
no unrelated promotion,2/3Swallow restriction and exceptions, last-rank/drop-mate
with legal defense, royal safety/pins/evasion/mate/stalemate, root-only terminal,
resignation/unsupported endings, third occurrence/departure responsibility,
forced checked/quiet units and actual forced-check full cycle, unequal overlapping
cycles, same board/different history, exact undo/replay/prefix return, identity
retirement and malformed/illegal atomic rejection.

Next native fixture work is to instantiate these specs in the real kernel and
add legal-set/perft expectations only where independently established. Comparator
source audit is not a substitute. Canonical raw duplicate snippets are targeted
malformed boundary probes; no otherwise valid current Tori envelope is claimed.

## Native implementation ownership after MSH-85 refinement

The103 immutable specs now have an explicit [ownership manifest](native-fixture-ownership.json)
and [bounded native queue](native-plan.md).126 supplies executable infrastructure
for all IDs,127/128/129 implement structural/local/game assertions. Real process
acceptance, desktop preservation and human notation assertions remain explicit
77/42 obligations; a native-only pass cannot claim those future journeys. The
selected fixture bytes and independently expected outcomes are unchanged.

## Executable machinery — MSH-126

[The engine-independent Rust harness](harness.md) now loads this unchanged corpus
and the full owner manifest. Fixture-data admission passes separately from native
assertions: without actual native observations all727 assertions are pending,
with0executed/passed; four prose proofs are annotations and83 consumer requirements
remain pending. Selected property/mutation/rotation machinery is executable now;
that does not claim native legality or replay. No external comparator is required.


## Current native local execution — MSH-128

The immutable specification file and its original data-only metadata remain
unchanged. Actual execution is recorded separately in [native local evidence](native-local-evidence.md):
483fields pass across83rows (all14primary127 and69primary128),244history/record/
consumer assertions pending,83later-consumer requirements pending. The four prose
proof annotations are not automatic passes. Local status and legal choices are
implemented; actual full-game conclusions/history/records and real consumers
remain129/85/77/42. No expectation was generated from native or external output.
