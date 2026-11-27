# Independent Chu fixture accounting

The [corpus](fixtures.json) has 249 rows. There are 220 authored rows,
29 owner-exchanged rotations and 10 rows for actual consumer behavior
within the total. Explanatory fields give the reason for an expectation.

Expectations precede implementation. C1/C2/C3 diagrams and prose, the selected
[profile](profile.md), hand enumeration and independently reviewed examples
supply facts. No mShogi, legacy or external engine output supplies expected values.
`expectation_method` and source/clause references identify each derivation.
Source diagrams completed with remote royals/pawns or a promoted Kirin origin
are labeled constructions, not claims that the source printed those pieces.

## Coverage

Rows can cover several clauses; these counts overlap.

| Clause | Rows | Concrete boundary |
| --- | ---: | --- |
| C01 |1|92-piece asymmetric original setup,46/side, base identity |
| C02 |75|All 29 movement faces, source-derived landing sets/counts,29 rotations; complete empty-center Lion88/SE41/HF42 action sets |
| C03 |21|Actual one/two-leg routes, jump versus intermediate capture, quiet/capture return, friendly/offboard/turn rejection, two victims |
| C04 |68|All 18 promotions: accept/decline/no-chain; camp entry, capture/exit, reentry, Pawn rescue, no ordinary Lance/GB rescue |
| C05 |18|Five supplement cases, hidden defense pair, distance/adjacency, P/GB exclusions, old/new Kirin-Lion protection, override/expiry |
| C06 |11|Royal exposure, ignore/counterwin, surviving Prince, capture of both royals, finite mate, Prince rescue, unresolved quiet immobility |
| C07 |11|Same-board history, quiet/check/attack responsibility, both-jitto/first actual passer, equivalent routes, last own opportunity before fourth |
| C08 |13|Narrow three-piece threshold, exclusions/dead blockers, immediate equalizer/counterwin, current-turn imports, actual draw consent/refusal |
| C09 |7|Exact private reversal, complete sequence/replay, imported provenance/counter,7/42 perft/divide |
| C10 |14|Strict/malformed/conflicting records, ordered events and paths, history identity, atomic refusal and retired execution |
| C11 |10|Real consumer requirements: codec/admission, choices/inspection/history, bounded engine, notation and finished12×12 UI |

The complete empty-center actor sets preserve **actions**, not only destinations:
Lion8 single steps+16 outer jumps+8×8 two-step routes=88; SE33 ordinary ray
actions+2×4 restricted actions=41; HF38 rays+4 restricted actions=42. Empty
routes sharing endpoints remain distinct. Source-derived geometry probes are
not automatically the full-game LegalAll set. The game also checks
turn, Lion law, game history obligations and conclusions.

The remote kings/golds/pawn case has 7 Sente actions, each followed by 6 Gote
actions: perft 1=7, perft 2=42,7 divide entries of 6. Hand enumeration was separately
reviewed; no general perft expectations are guessed from future engine output.

## Fixture execution

`given` separates imported diagrams, actual ordered commands, action probes,
geometry queries and future consumer scenarios. The development `[file,rank]`
grammar is a fixture notation, not a frozen Rust API or public protocol. Adapt
it to the actual game types; derive victims/effects in the rules library. Deliberately
malformed requests remain raw negatives rather than being sanitized by an adapter.
Explanatory expected fields need explicit traceability to assertions or
reviewed derivations, never a generic expected-value echo.

Run exact sets for applicable LegalAll tests, not a convenient selected move;
compare actual ordered intermediate/final effects and complete routes. Exercise
full Game history separately from local geometry/transition helpers. Royal mate
and repetition admission must not recursively adjudicate mate or silently use
a Chess check filter. Replay from the game root and verify duplicated facts.

Add deterministic admitted sequences, apply/undo restoration of all logical
state/caches, canonical roundtrips, rejected-request atomicity, capture/discard
accounting and meaningful adversarial mutations. Rotation exchanges owners and
rotates all squares 180°; an imported Gote-to-move rotation is not called the
standard starting game. Do not assume horizontal mirror symmetry of the actual
asymmetric setup. Test all 144 coordinates and exact route reversal where valid;
do not claim every transformed outcome is valid without the corresponding
semantics/history transformation.

## Development validation, not execution

`python3 docs/m4/chu/validate.py` strictly admits data, checks identities, source/
fixture links, setup inventories, coverage, rotations and reviewed data hashes.
`python3 -m unittest discover -s docs/m4/chu -p 'test_validate.py'` exercises
negative data admission. These implement **no Chu move generator or adjudicator**.

The validator uses a 1 MiB development-data limit. See the
[conformance policy](../../testing/conformance-policy.md).
