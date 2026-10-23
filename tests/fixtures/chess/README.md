# M1 chess reference corpus

Contract: [`mshogi.chess.position.v1`](../../../docs/rules/chess-position-v1.md). Data format `schema_version: 1`; file [corpus.json](corpus.json). Added by MSH-2, 2026-10-23. This is input and independently expected output for future product tests, not evidence that mShogi passes them.

## Format

The root has `schema_version`, `rules_profile`, `sources` and `cases`. Sources have stable IDs, URL, exact version, retrieval date, and where available SHA-256 of the retrieved bytes (for python-chess, the installed `chess/__init__.py` bytes). Raw downloaded documents and installed tools are outside the repository. Each case has:

| Field | Meaning |
| --- | --- |
| `id` | Stable unique lowercase identifier; preserve IDs in regression reports |
| `rules_profile` | Exact profile ID, repeated so an extracted case is self-identifying |
| `tags` | Rule/coverage categories; not a completion claim |
| `input.fen` | Six-field starting state, or the exact malformed parser input |
| `operation` | `position` or `parse_fen` |
| `expected` | Exact answers described below |
| `provenance` | Registry source IDs, precise locator, establishment method and rule rationale |

For `position`, compare the **entire** sorted unique `legal_moves` array, including explicit underpromotion suffixes. Order in an implementation is irrelevant; normalize before comparison. An empty array is intentional. `classification` is one of `quiet`, `check`, `checkmate`, `stalemate`. Optional `perft` maps decimal depths to exact nonnegative node counts; this bounded seed uses depths 0 through 3. Optional `attacks` asks whether color `by` attacks `square`; occupancy, pinned attackers and pawn direction follow the profile. These queries are geometric, not legal-move membership.

For `parse_fen`, `accepted: true` has an exact `canonical_fen`; `accepted: false` has error category `syntax`, `range` or `structure`. Parser cases are independent **contract examples** under the documented project validation policy. They do not claim another library has the same input policy. Implementers must test rejection without caller-state mutation in MSH-4. No executable mShogi parser is added here.

New operations or incompatible meanings require a format version change. Do not silently drop unknown operations, compare only a move subset, replace exact counts with ranges, or auto-update answers from the implementation being tested. Position fixtures may later also serve as transition/undo starting states; passing move generation alone does not establish undo correctness.

## Coverage and provenance

The corpus contains 35 position cases and 33 FEN contracts (6 accepted, 27 rejected). The [coverage ledger](../../../docs/rules/chess-coverage.md) distinguishes the contract from implementation status.

- The first six positions come from the **standard** section of [Stockfish sf_17.1's maintained perft test](https://github.com/official-stockfish/Stockfish/blob/sf_17.1/tests/perft.sh). Its own comment attributes the positions to Chess Programming Wiki Perft Results. The four-field Kiwipete and rook/pawn records receive explicit `0 1` counters to meet this profile; all other positions are unchanged. Stored depth 0..3 counts were independently enumerated with `chess==1.11.2`; they are **not** misrepresented as the deeper counts printed in that upstream file. No Stockfish engine run or deep upstream perft verification is claimed here.
- Other positions are small, purpose-built diagrams whose rationale and exact FIDE articles are recorded. Both colors have castling, en passant, quiet/capturing four-way promotion, mate and stalemate cases. Separate diagrams exercise pins, a pinned attacker, interposition, double check, king adjacency, capture x-rays, attacked castling paths and discovered check through en passant.
- Rule rationale was curated before oracle enumeration. Exact legal move sets, classification, attacks and shallow counts are established with the independently maintained [python-chess v1.11.2](https://github.com/niklasf/python-chess/tree/v1.11.2), not either mShogi implementation. Source/module identity and retrieval date are in the data. Agreement with one oracle is bounded evidence, not a proof of all chess rules.
- Parser fixtures derive from [Edwards's FEN definition, section 16.1](https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c16.1) plus explicit profile policy. `fen-uncapturable-ep` is its position after `1. e4`; other synthetic inputs isolate policy boundaries. The accepted three-queen/eight-pawn diagram deliberately illustrates analysis acceptance without historical reachability.

Oracle discrepancies are resolved explicitly: use `board.fen(en_passant="fen")` to retain raw EP; do not use the oracle's permissive FEN parser, validity bitmask or `outcome()` as mShogi policy. The checker skips rejected parser inputs rather than falsely claiming they pass product tests. No outstanding expectation disagreement was found in the initial enumeration; later discrepancies must preserve the case, source, rule reasoning and competing outputs for review.

## Reproduce

Run from the repository root. Structural verification uses Python 3's standard library only:

```sh
python3 tools/check_chess_corpus.py
```

An optional isolated research environment rechecks independent reference answers. Choose a disposable directory outside the checkout; this POSIX example leaves product dependencies and global packages untouched:

```sh
python3 -m venv /tmp/mshogi-chess-oracle
/tmp/mshogi-chess-oracle/bin/python -m pip install chess==1.11.2
/tmp/mshogi-chess-oracle/bin/python tools/check_chess_corpus.py --oracle
```

On Windows use the equivalent virtualenv `Scripts/python.exe`. A platform-specific reproduction example is not evidence of an execution there. The MSH-2 ticket records the actual host, commands, versions, outcomes and review.

The checker reads committed expectations and never rewrites them. It verifies the oracle version and module hash before comparing complete answers. Perft is bounded to depth 3; the six reference cases total 271,312 depth-three leaves. Invalid fixture metadata or mismatched expectations exits nonzero with the case ID. `--corpus PATH` supports reviewing candidate data without changing the committed file.

Licensing boundary: [python-chess is GPL-3.0-or-later](https://github.com/niklasf/python-chess/blob/v1.11.2/README.rst#license), and [its license](https://github.com/niklasf/python-chess/blob/v1.11.2/LICENSE.txt) was inspected before use. It is an optional, separately installed research/checking tool; no upstream code or package is vendored, linked into Rust, added to product dependencies, or shipped by this change. The small perft positions and numerical answers are attributed factual test data, not copied test program code. Existing BSD notice and deferred release-licensing decision remain unchanged. Reassess licensing before any future distribution or code reuse.
