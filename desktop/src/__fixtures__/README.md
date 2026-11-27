# Session snapshot fixtures

Generated through the current `DesktopController::shared_command_with_receipt`
and `shared_snapshot` contract, using authoritative game records. Chess fixtures
use explicit-claim adjudication. The `shared-chess-*` family contains
Chess capability details within the same application snapshot as Shogi/Xiangqi;
there is no embedded former Chess Snapshot DTO. Analysis fixtures run the actual
engine through the generator executable's isolated `internal-search` child mode. They exercise frontend selection
and rendering against actual serialization; they are not an independent rules
oracle. M1/M2 rules fixtures remain authoritative correctness evidence.

Regenerate the frontend snapshots for all six games and the ordinary Load
documents from the repository root:

```sh
python3 desktop/scripts/regenerate-fixtures.py
python3 desktop/scripts/regenerate-fixtures.py --check
```

The check generates a fresh set in a temporary directory and compares the file
names and bytes with the checkout. The frontend generators enable
`desktop-verification` for capacity and terminal-state scenarios.

The ordinary Load documents cover Shogi captures, Tori rules positions, the
[Chu journeys](../../scripts/fixtures/CHU.md), and both Tenjiku profiles. Chess
and Xiangqi GUI startup scenarios are constructed by the QA host. The recorded
Chess and Tenjiku GUI observations retain their original provenance.

For Tenjiku, `write_tenjiku_frontend_fixtures` writes the current frontend
snapshots for Adams 1999 Modern and Muller Completed. The ordinary Load
generator, `write_tenjiku_gui_fixtures`, creates ten documents per profile and
the `tenjiku-game-fixtures.json` manifest. Its diagrams and manual expectations
come from the independently authored
[Tenjiku corpus](../../../docs/m4/tenjiku/fixtures.json). The royal-engine
diagram adds a remote Gote Pawn so a one-ply win must remove the King. The
package smoke test separately checks actual engine replies against those
documents. The document generator uses current settings and checks exact
decode and replay for all twenty files.

Observations use the current durable observation schema without a transport
wrapper.

Only frontend tests import these JSON files. Vite production imports do not
reference them, and the rules fixture generator is never a bundled sidecar.

The Xiangqi standard snapshot, capture/recapture played ledger and read-only Black-cannon inspection are generated
by the same authoritative shared controller (`xiangqi::Game::standard`), including
the settings shape of every game, 90 intersections and complete choices.
They are interaction/serialization inputs, not a second rules oracle. The frozen
XQ-B01 opening expectations and independently reviewed rules fixtures own rules
correctness. No fixture, verification controller or fake result enters a normal
product package.

The generator normalizes only the frontend fixture copy of an **untimed** Chess
MatchRecord to event-index nanosecond timestamps. The controller's real runtime
timestamps, product records and saved documents are unchanged. Each fixture
record is rebuilt by game replay: commands, actors, versions and every
reply must remain identical, as must canonical game history, conclusion and
active/clock state. Timed matches reject normalization. This keeps match
history visible while making snapshot fixtures reproducible across runs and OSes.
The generator example's two unit tests cover preservation, idempotence and timed
rejection; two consecutive generation runs should produce identical JSON bytes.
