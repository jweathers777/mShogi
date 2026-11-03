# Native Chu human action notation

MSH-43 supplies a compact mShogi display/input adapter. This is **not a claim of
standard JCSA notation** and does not change the selected playing profile or
canonical record schema. The canonical native action and exact game history
remain authoritative. The console selects an actual native local choice, then
submits the complete action through `Game` with its current context.

Grammar: `piece source (separator landing){1,2} [^]`, without spaces.

- `piece` is the original native code: P, GB, L, RC, Cu, Si, G, FL, BT, DE, K,
  Ki, Ph, SM, VM, R, B, DH, DK, Ln or FK. A trailing `+` means that original
  identity has already promoted. For example `DE+` is the Prince and `Ki+` is a
  promoted Kirin moving as a Lion; they are not relabelled as original pieces.
- A coordinate is file 1–12 followed by rank a–l. Both owners use the same board
  coordinates. Sente/Gote ownership is supplied by the position and history.
- `-` means a quiet actual landing; `x` means a capture at that stage. Every
  landing is recorded in order. Jumped-over squares are not route stages.
- The final `^` means this action promotes at its final landing. Without `^`,
  the action does not flip. Absence does not assert that promotion was available.

Examples, against the appropriate native positions:

| Complete action | Display |
| --- | --- |
| Lion jumps over 6e and captures at 6d | `Ln6fx6d` |
| Lion captures at 6e, then captures at 6d | `Ln6fx6ex6d` |
| Lion captures at 6e and returns to 6f | `Ln6fx6e-6f` |
| Quiet Lion return through 5e | `Ln6f-5e-6f` |
| Pawn enters the camp and promotes | `P6e-6d^` |
| The corresponding unflipped choice | `P6e-6d` |
| Prince (promoted original Elephant) | `DE+2f-2e` |

The formatter preserves all landings, capture order, original identity and final
promotion choice. It does not grant legality. Parsing is exact and case-sensitive;
partial routes, incidental whitespace, wrong capture markers, collapsed endpoints,
wrong original identities and unavailable flips cannot match a native choice.
The live game separately applies Lion rights, repetition obligations, terminal
state, actor and context admission. Historical formatting uses the exact preceding
native event prefix, including the original capture occupancy.

## Console

Run `cargo run --release --locked --bin mshogi -- --game chu shell`, then `help`.
The console accepts standard roots and strict canonical record-file/record-json
custom roots. It does not invent SFEN, USI or a second machine protocol. Compound
moves are entered as complete labels; `legal` prints each label with its native
JSON action. A `move` batch commits only if every complete action is admitted.

`history` includes all native events and obligation/outcome metadata; `undo` is
one-event native reversal. Actual `offer-draw`, `accept-draw` and `decline-draw`
commands retain the chosen actor and do not pretend an offer is consent. `stop`
is an unscored work annotation. `analyze` is read-only history-aware native search.
Perft/divide are explicitly position-local, depth0–2 (divide1–2), at most100,000
visited positions; an exhausted diagnostic is incomplete/unscored and leaves the
live session untouched. There are no captured-piece hands in Chu.

Canonical Save uses the existing bounded atomic no-replacement publisher. A save
to an existing path fails while preserving that file; strict replay must succeed
before Load replaces the current session. Human labels are never saved in place
of complete native canonical semantics.
