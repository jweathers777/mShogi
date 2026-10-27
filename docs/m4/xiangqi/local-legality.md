# Native Xiangqi local board contract (MSH-93)

`mshogi_core::xiangqi` implements the selected `mshogi.xiangqi.wxf2018.v1`
board clauses. It does not supply a Xiangqi match, search engine, shared record
adapter, console selection or desktop consumer. Native state/root codecs are
specified in `position-api.md`; that MSH-92 foundation remains independently
reviewed historical evidence. No stable Chess or Shogi API changes were needed.

## Explicit construction and query domains

`from_diagram` and strict `from_fen` construct structural positions with one
General each and original per-side material caps. Such a diagram may deliberately
contain off-home pieces or an already threatened non-moving General. These are
useful diagnostics, not automatically live roots or proof of historical play.

`validate_local` separately requires each General in its own palace, Advisors
on its five diagonal points, Elephants on their seven home-side points and
Soldiers on their forward-reachable geography (before the river, original even
files and no ranks behind their starting rank). It rejects an already attacked
non-moving General, including facing Generals. Sparse positions are allowed;
no impossible-to-win outcome is inferred. These necessary static conditions
still do not prove historical reachability. Neither counters nor omitted history
change this admission or the local choice set.

`pseudo_legal_actions`, `legal_actions` and `local_status` return explicit
`LocalPositionError` for invalid roots, never an empty set masquerading as
mate/stalemate. Four frozen General-threat/capture fixtures exercise precisely
this distinction rather than being silently omitted.

## Threats, complete actions and acceptance

- `is_attacked`/`is_in_check` are geometric diagnostics, including pinned
  attackers. Capture-mode geometry differs from a Cannon's quiet moves. A
  distant flying General attacks the opposing General on its file with no
  screen; it is not an ordinary long-range move or capturable royal. The
  distant royal interaction requires an opposing General at the target;
  arbitrary empty-target queries do not simulate a placed General.
- Pseudo choices apply all piece movement, blocking, occupancy, river/palace
  and screen rules. General capture is excluded. Complete legal choices also
  leave the moving General free of geometric threats, including discovered
  facing-General exposure. There is no recursive legal-move definition of
  attack. Horse-leg and Cannon-screen changes are recomputed from the whole
  resulting board; no chess sliding-check shortcut is used.
- `resolve_action` admits a complete native `Action`, derives effects and binds
  the exact before/after position. `apply` resolves then commits atomically;
  `apply_resolved` rejects a changed complete before-state. Private structural
  transition helpers cannot be used by consumers to bypass local rules.
- `undo` accepts only its exact matching post-state and restores all state,
  including native ownership, victims, turn, known/unknown counters. This is
  local value protection, not future session/request/process ownership. MSH-94
  and shared consumers must not let semantic undo revive retired work.

`ActionError` distinguishes invalid roots, wrong source/owner/occupancy,
movement, General exposure, stale resolution and counter exhaustion. Malformed
coordinates reject during strict parsing. No rejection assigns a game result.
An exhausted u32 counter cannot remove a legal board choice or produce a
no-action result: legal queries remain exact, while committing a transition
that cannot represent its counters explicitly returns `CounterOverflow`.
An actual capture may establish a known zero raw capture-free counter.

`LocalStatus::NoLegalAction` retains `Checkmate` versus `Stalemate`; both are
losing local no-action reasons in this profile. It does not conclude a game
or establish precedence against history outcomes. Same-board F002 check/quiet
prefixes have equal local choice sets but deliberately no repetition result.
All WXF warnings, chase, overlapping-cycle, claimant-specific natural counts,
software claim policy and impossible-to-win obligations remain MSH-94/105.

## Bounded enumeration and executable comparison

`perft`/`divide` enumerate only local board edges, depths0..3. Depth0 has one
leaf and no divide edges; positive-depth no-action has zero leaves. Invalid
roots and larger depths reject. Counts use checked u64 addition. A private
normalized counter copy prevents representation exhaustion from changing a
diagnostic tree; the original position is unchanged. Historical conclusions
and WXF3.2A do not truncate enumeration.

`cargo build --locked -p mshogi-core --example xiangqi_probe` builds a separate
developer-only executable. It accepts one strict bounded JSON request on stdin
using the reference tool contract in `tools/xiangqi/README.md`: root FEN,
optional complete replay actions (at most256), optional perft depth0..3.
Input/output bounds are4096/65536bytes; parsing rejects duplicate/unknown fields
without normalization. Illegal replay rejects, never repairs the prefix. Every
available choice and every replay-prefix undo is verified for exact restoration
before emitting an observation. The Python supervisor additionally bounds wall
time and diagnostics and retires the process group. This is a comparison tool,
not a game protocol or newly advertised execution capability; normal product
binaries do not link this example's input/output instrumentation.

## Verification interpretation

The frozen corpus contains105 independent source-reasoned cases:101 live roots
and4 explicit structural diagnostics. Exhaustive whole-position and selected-
origin sets remain distinguished from partial facts. Initial44 and sparse
perft2/3/7 (depth-three divide5+2) are independently established. Larger counts
and random walks are differential observations, not new oracle facts.

`tests/xiangqi_board.rs` consumes every frozen fact and separately covers
pseudo/legal pins, checked/unchecked no-action reasons, all-choice make/unmake,
three deterministic64-ply native sequences and exact prefix reversal, F002
check/quiet history seeds, and pinned enemy Rook geometric safety. Unit tests
add all8100 coordinate-pair public admissions, stale resolution, unknown/max
counters, strict domains and the raw24-ply checking-cycle counter (the foreign
reference emits20, which is explicitly not native counter authority).

Execution results, exact revisions, admitted reference identity, independent
review and normal CI are recorded in the MSH-93 task and its evidence document.
Passing this layer does not establish history conformance or playing strength.
