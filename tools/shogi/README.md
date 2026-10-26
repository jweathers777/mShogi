# Shogi board reference tooling

`reference.py` is an optional developer tool. It executes pinned cshogi in a
separate Python environment. No Python, cshogi source, GPL reference binary or
reference dependency is linked into or shipped with mShogi. See the
[actual admission evidence](../../docs/m4/reference/README.md) for the source,
license, compiler, flags, hashes and reproducible build.

Run the admitted reference fixtures and intentional failure demonstrations:

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --diagnostics-demo --report /private/tmp/shogi-reference-run.json
```

The expected facts in
[`board-rules.json`](../../tests/fixtures/shogi/board-rules.json) were drawn and
reviewed independently before the reference run. `contains` and `absent` assert
specific complete USI choices; `legal_moves` asserts an exact set;
`after_sfen` asserts individual transitions and `sfen` the final supplied
sequence. `in_check` asserts board check only. Reference observations are saved
separately. Reversal snapshots include SFEN, board pieces, both hands, turn,
next-ply, known history, check, full LegalAll choices and the reference's own
hash. Hash integers never cross the program boundary.

Each run checks the admitted extension hash, fixture integrity, all independent
assertions, exact nested push/pop, and four actual upstream API-gap sentinels.
The sentinel-only parser/is_legal/is_draw observations demonstrate why those
APIs cannot decide expected negative legality or game results. Board membership
always comes from `Board.legal_moves` (`MoveList<LegalAll>` at the source pin).
There is no search-pruned `Legal` or PV substitute. Game conclusions,
declarations, clocks and pre-root occurrences are outside this tool; MSH-67
owns their independent fixtures.

## Native probe contract

`--probe 'command args'` launches a subprocess for each compared prefix. The
subprocess reads one JSON line, reconstructs that root and the full ordered
prefix, then prints exactly one JSON response. Nonzero exits, stderr, timeout,
truncated JSON, wrong fields, malformed or duplicate choices are comparison
failures with retained diagnostics.

```json
{"root_sfen":"8k/9/9/9/9/9/9/9/K8 b - 1","moves":["9i9h"]}
```

```json
{"sfen":"8k/9/9/9/9/9/9/K8/9 w - 2","legal_moves":["1a1b","1a2a","1a2b"],"in_check":false,"undo_restored":true}
```

`undo_restored` is optional; an explicit false is a failure. Full USI identity
preserves source, destination and promotion suffix, or drop piece and target.
Choices are unordered and must be unique. SFEN compares canonical text and
reports a separate parsed semantic difference, including ownership, promotion,
hands, side and counter. No native/reference hash equality is expected.

```sh
/private/tmp/msh64-reference-venv/bin/python tools/shogi/reference.py \
  --probe 'target/debug/examples/shogi_probe' \
  --seed 20261026 --playouts 1 --plies 24 \
  --report /private/tmp/shogi-differential.json
```

Every unique fixture root contributes deterministic seeded play. Each move is
selected from the sorted LegalAll set using Python's seeded PRNG. Reports retain
root, seed, original sequence, canonical failing position, missing/extra choices,
and the shortest failing prefix: all earlier prefixes were checked. This is
prefix minimization, not a claim of globally minimal board material or a proof
that history can be deleted. A snapshot with zero board moves remains a valid
response; a transport failure does not masquerade as an empty legal set.

`--diagnostics-demo` starts real subprocess test doubles with missing choice,
extra choice, exit17, and truncated-response injections at ply2 of a retained
five-move seeded path. The run must detect each and reduce the evidence to the
two-move failing prefix. Those injected failures are expected diagnostic tests,
not observations of defects in mShogi. Native comparison is a separate command;
MSH-64's admission report contains zero native comparisons.
