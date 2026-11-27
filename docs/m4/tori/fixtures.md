# Independent modern fixture specification

The [corpus](fixtures.json) contains independently derived inputs and
expected outputs. The [ownership manifest](rules-fixture-ownership.json)
identifies the executable assertion or proof role for each fixture.

Board entries `s:Fa:4c` mean Sente Falcon at 4c; `g` means Gote. Kind aliases
are declared at the corpus top. Hands omit zero counts. A diagram root explicitly
allows missing material, records unknown earlier history and makes no claim of
historical reachability. Standard root has full 32-piece inventory. Move commands
are complete typed specs, including required result face; drops identify exact
base kind and destination. These are harness inputs, not a prematurely frozen
wire schema. Future representation may map without altering meaning.

- `exhaustive_local_geometry`: complete movement tables on an otherwise empty
  board, not whole legal positions. Limited rays and jumps are distinct.
- `game_admission`/`game_admission_rotated`: concrete diagram and submitted
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

Coverage: all 9 faces, both owners, rays/friendly and enemy blockers,
leaps, capture/demotion/handed transfer, compulsory entry/within/exit, zone drops,
no unrelated promotion,2/3Swallow restriction and exceptions, last-rank/drop-mate
with legal defense, royal safety/pins/evasion/mate/stalemate, root-only terminal,
resignation/unsupported endings, third occurrence/departure responsibility,
forced checked/quiet units and actual forced-check full cycle, unequal overlapping
cycles, same board/different history, exact undo/replay/prefix return, identity
retirement and malformed/illegal atomic rejection.

Canonical raw duplicate snippets exercise malformed boundaries. Complete
legal-set and perft expectations use independent derivations.
