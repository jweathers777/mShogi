# Xiangqi board-reference tools

These are developer-only, isolated comparison tools for MSH-91/MSH-93. They
are not linked into mShogi, packaged with the application, or used to decide
WXF game outcomes. Independently expected fixture facts remain authoritative
inputs for investigation; an engine agreement is an observation.

## Reproduce a build

Obtain the official Pikafish repository at exactly
`9d7833aa113626b3ea63ad83f49fb9b456bd5839`, with a clean tracked tree. Then:

```sh
python3 tools/xiangqi/build_reference.py \
  --source /absolute/path/to/pinned/pikafish \
  --output /private/tmp/msh91-reference/rebuilt
python3 tools/xiangqi/test_reference.py \
  --manifest /private/tmp/msh91-reference/rebuilt/admission.json
python3 tools/xiangqi/compare.py \
  --manifest /private/tmp/msh91-reference/rebuilt/admission.json \
  --fixtures tests/fixtures/xiangqi/board.json --depth 2 \
  --report /private/tmp/msh91-reference/rebuilt/fixture-comparison.json
```

The output directory must not exist. The builder clones only the clean pinned
Git tree into it, applies the retained add-only harness patch, and compiles
specific board source units directly. It does not run the upstream Makefile,
download a network, call `Engine::perft`/`verify_network`, or evaluate NNUE.
`k6r2nc2.cpp` is needed for native incremental feature-bookkeeping helpers in
`do_move`; no weights or search are used. Assertions remain enabled.

The build records source/tree hashes, the entire patch, actual compiler
commands and log, compiler/platform/flags and binary SHA-256. A locally
reproduced binary gets its own manifest: do not replace its identity with an
old binary hash. GPL-3.0-or-later source/license provenance is retained. No
binary or external model is committed or distributed by this tool.

## Operation and domain

The admitted operation is `Position::set`, complete `MoveList<ALL>` (including
`EVASIONS` under check), `do_move`/`undo_move`, board-only perft/divide, and
native check state. The adapter verifies FEN round trips and restoration of
board, native state bytes and pointer, hashes, and complete legal choices
after every enumerated make/unmake and every replay-prefix undo. These checks
establish the comparator's restoration behavior; native mShogi restoration
has its own tests.

The public JSONL request is:

```json
{"root_fen":"rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1","moves":[],"perft_depth":2}
```

Only `root_fen` is mandatory. The other two fields default to `[]` and `0`.
Unknown/duplicate fields, nonfinite values, malformed actions and unsupported
domains are rejected. At most 256 replay actions and depths 0–3 are accepted.
Every replay action must belong to the complete enumerated set.

FEN is six fields, canonical single spaces, ranks 9 down to 0, files a–i,
literal `- -`, `RACPNBK/racpnbk`, `w`=Red and `b`=Black. Complete action
coordinates are a0–i9, four characters, no promotion/drop suffix. The parser
domain requires exactly one General each, per-side original material maxima,
native palace/Advisor/Elephant/Soldier square restrictions, fullmove 1–100000
and root declared counter 0–119. The native parser additionally rejects a
position where the moving side already attacks the non-moving General.
Successful admission is not proof of historical reachability. Explicit native
structural diagnostics outside this domain are skipped, never normalized into
different positions. Native record/history and counter support are separate.

Response fields are `fen`, sorted unique `legal_moves`, `in_check` and
`undo_restored`. Positive depth adds `perft` and sorted
`divide:[{"action":"a0a1","nodes":1},...]`. Depth0 does not add count fields.
Use `python3 tools/xiangqi/reference.py --manifest ...` for one JSON request
on stdin. POSIX subprocess control has a default20s timeout (maximum120s),
4096-byte request, 65536-byte response and 4096-byte diagnostic caps. Timeouts,
process failures, malformed responses, resource limits and admission failures
are explicit diagnostic categories. Failure/success retirement kills the
process group, including descendants whose leader has already exited.

All history/repetition/chase helpers, natural-count adjudication, no-win
outcomes, royal-capture continuations, evaluation, search and playing strength
are excluded. Perft does not stop for historical or impossible-to-win outcomes.

## Native comparison

The native developer probe may consume the same JSONL contract. Add:

```sh
--native-probe '/absolute/path/to/native-probe' --walks 8 --length 64
```

to `compare.py`. It compares both implementations with independently expected
facts, and then compares complete response sets/check/board-side-fullmove and optional
perft/divide. Deterministic seeded sequence observations are explicitly kept
apart from fixture expectations. A mismatch report retains missing and extra
actions, expected facts and actual observations, along with tool/fixture/
manifest/binary hashes. It never edits fixtures. Unproven count observations
must not be relabeled independently established perft expectations.

Pikafish's emitted FEN counter is its rule60/check10-adjusted clock. It is
explicitly excluded from native counter agreement, while both original FEN
observations are retained. The native raw/declared counter and WXF history
cannot be redefined by this comparator. A 24-ply, 12-check cycle sentinel
demonstrates the foreign clock reporting20 while the raw no-capture count
is24. Placement, side and fullmove number remain comparable.

## Evidence

`docs/m4/xiangqi/reference/` retains the executed macOS/arm64 build manifest,
patch and compiler log, 15 admission/failure sentinel results, a deliberate
mismatch example, the actual B38/mirror disagreement found on first execution,
and independently derived perft evidence. The B38 expected-data correction was
made only after separate rule review established that the checking Horse also
blocked the facing-General line. Initial and renewed fixture hashes remain
recorded; the harness did not infer or rewrite the expected answer.

The earlier `docs/m4/xiangqi/comparator-audit.md` remains historical source-only
research. This runtime admission supersedes its *not yet built/admitted* status
only for the restricted operation above, not its outcome exclusions. The
alternative Fairy-Stockfish pin remains unbuilt and unadmitted.

## Reviewed MSH-94 history board paths

The four frozen cycle/count/mutual-check/precedence corpora are also exercised
through the existing board-only native probe and admitted reference. This does
not make reference repetition/count outcomes authoritative. To rebuild the
native executable from the checked-out source and collect a compact report:

```sh
cargo build --locked -p mshogi-core --example xiangqi_probe
python3 tools/xiangqi/collect_history_boards.py --report /tmp/history-board-summary.json
python3 tools/xiangqi/review_count_history.py --output /tmp/count-arithmetic.json
```

The collector defaults to current directory as repository; `--repo` and
`--native-probe` can name explicit paths. It checks every prefix (492 over17
histories), all complete legal choices, check, exact undo and comparable FEN
fields. `--native-only` is explicitly labeled and makes no reference claim.
Each process uses the existing bounded adapter; failure/mismatch names the
fixture/prefix. Input/source/binary mutations during collection refuse a report.
Expected fixtures are never regenerated. Reports retain hashes and per-fixture
counts rather than bulky raw streams; deterministic ordered observation frames
can be reproduced with the same tools/binaries. Source hashes identify observed
files; the preceding Cargo build, not the collector, establishes their build
relationship to the native executable. Independent expected outcomes and their
source review remain in `docs/m4/xiangqi/history-evidence.md`.

`review_count_history.py` is a deliberately limited independent arithmetic check
for the four reviewed Rook/General count fixtures, with straight-line placement
and capture/check evidence. It is not a reusable rules engine or product dependency.
