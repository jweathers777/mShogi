# M1 chess reference corpus

Contract: [`mshogi.chess.position.v1`](../../../docs/rules/chess-position-v1.md). Data format `schema_version: 1`; file [corpus.json](corpus.json). Added by MSH-2, 2026-10-23. This is input and independently expected output for product tests. MSH-4 consumes the FEN contracts and starting states; MSH-6 the attack queries; MSH-9 the exact legal move sets and classifications. Perft expectations await MSH-10.

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

For `parse_fen`, `accepted: true` has an exact `canonical_fen`; `accepted: false` has error category `syntax`, `range` or `structure`. Parser cases are independent **contract examples** under the documented project validation policy. They do not claim another library has the same input policy. MSH-4 executes rejection without caller-state mutation in the core Rust integration tests. The Python oracle checker intentionally does not act as this profile's product parser.

New operations or incompatible meanings require a format version change. Do not silently drop unknown operations, compare only a move subset, replace exact counts with ranges, or auto-update answers from the implementation being tested. Position fixtures may later also serve as transition/undo starting states; passing move generation alone does not establish undo correctness.

## Coverage and provenance

The corpus contains 35 position cases and 33 FEN contracts (6 accepted, 27 rejected). The [coverage ledger](../../../docs/rules/chess-coverage.md) distinguishes the contract from implementation status.

- The first six positions come from the **standard** section of [Stockfish sf_17.1's maintained perft test](https://github.com/official-stockfish/Stockfish/blob/sf_17.1/tests/perft.sh). Its own comment attributes the positions to Chess Programming Wiki Perft Results. The four-field Kiwipete and rook/pawn records receive explicit `0 1` counters to meet this profile; all other positions are unchanged. Stored depth 0..3 counts were independently enumerated with `chess==1.11.2`; they are **not** misrepresented as the deeper counts printed in that upstream file. No Stockfish engine run or deep upstream perft verification is claimed here.
- Other positions are small, purpose-built diagrams whose rationale and exact FIDE articles are recorded. Both colors have castling, en passant, quiet/capturing four-way promotion, mate and stalemate cases. Separate diagrams exercise pins, a pinned attacker, interposition, double check, king adjacency, capture x-rays, attacked castling paths and discovered check through en passant.
- Rule rationale was curated before oracle enumeration. Exact legal move sets, classification, attacks and shallow counts are established with the independently maintained [python-chess v1.11.2](https://github.com/niklasf/python-chess/tree/v1.11.2), not either mShogi implementation. Source/module identity and retrieval date are in the data. Agreement with one oracle is bounded evidence, not a proof of all chess rules.
- Parser fixtures derive from [Edwards's FEN definition, section 16.1](https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c16.1) plus explicit profile policy. `fen-uncapturable-ep` is its position after `1. e4`; other synthetic inputs isolate policy boundaries. The accepted three-queen/eight-pawn diagram deliberately illustrates analysis acceptance without historical reachability.

Oracle discrepancies are resolved explicitly: use `board.fen(en_passant="fen")` to retain raw EP; do not use the oracle's permissive FEN parser, validity bitmask or `outcome()` as mShogi policy. The checker skips rejected parser inputs rather than falsely claiming they pass product tests. No outstanding expectation disagreement was found in the initial enumeration; later discrepancies must preserve the case, source, rule reasoning and competing outputs for review.

## Reproduce

The product parser suite consumes the unchanged corpus directly:

```sh
cargo test -p mshogi-core --test fen_corpus --locked
```

It tests all 33 parser contracts (6 accepted, 27 rejected) and canonical parsing of the other 35 starting positions. This parser suite alone does not verify those positions' legal moves, attacks, classification, perft or apply/undo behavior; the later suites below do, except perft.


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

## Ordinary before/after fixture family

MSH-5 adds [ordinary-transitions.json](ordinary-transitions.json), a separate family named `mshogi.chess.ordinary-transitions.v1` with `schema_version: 1`. This does not change or add operations to the original corpus format. Each case has a stable ID, the exact rules-profile ID, `before_fen`, nonempty ordered `steps` of `{ action, after_fen }`, coverage tags and source/locator/method/rationale provenance. Action text uses the same coordinate/promotion test adapter; it is not a new production wire protocol. Both FENs and the complete action, not caller-supplied effects, make a step reproducible.

There are 49 cases and 74 ordinary steps. Hand-reviewed cases exercise initial doubles and raw targets, expiry, ordinary quiet/capturing piece geometry, check evasions, all four quiet/capturing promotions of both colors, king and home-rook rights loss (including promoting captures), return-without-right-restoration, and representable counter boundaries. Expected after-FENs were independently enumerated with pinned `chess==1.11.2` after first checking each curated action's legality and excluding castling/EP. Source version/module hash and FIDE/profile rationale are recorded; no mShogi output established an expected answer.

Run the product transitions and complete reversal/atomic rejection tests:

```sh
cargo test -p mshogi-core --test ordinary_actions --locked
```

Optional oracle recheck, in the separately installed environment described above:

```sh
/tmp/mshogi-chess-oracle/bin/python tools/check_chess_transitions.py
```

The oracle checker verifies every expected after-state and never rewrites it; Rust tests additionally compare the full saved tuple at every reverse step. Neither implies special-move execution or full match adjudication. Invalid-action, non-ordinary special, overflow and bad-undo tests are explicit Rust cases under the documented API/profile policy; they do not substitute agreement with a permissive external parser for the project boundary.

## Attack fixture family

MSH-6 adds [attacks.json](attacks.json), a separate family named `mshogi.chess.attacks.v1` with `schema_version: 1`. The original corpus and its three attack queries are unchanged. Each case has a stable ID, the exact rules-profile ID, a canonical `fen`, coverage `tags`, curated `queries` of `{ by, square, attackers }`, `attacked_squares` mapping `white` and `black` to every attacked square, and source/method/rationale provenance. Square lists use a1-to-h8 order (file fastest), matching the Rust `attackers` order; an empty list is intentional.

There are 29 positions, 155 exact attacker sets and 3,712 implied per-square map answers. Diagrams isolate pawn direction/edges, occupancy independence and last-rank pawns checking the back rank, knights in the centre, corners and surrounded, adjacent and corner kings, first-blocker sliders with no x-ray (including a friendly battery), facing queens, pinned attackers of both colors (black bishop, rook, and a white knight covering the black king's flight squares), a king blocking its own check ray, castling-path coverage for both colors by bishop, knight, pawn and rook, en passant before/after occupancy opening a rank for both colors and a diagonal for White, double check and edge pieces. Some positions deliberately show the occupancy after a candidate action with the side to move unchanged, because legality is judged on that post-action board.

Each diagram, query and attacker set was first written by hand from the recorded FIDE/profile rationale. The pinned `chess==1.11.2` `Board.attackers` then confirmed every set, and `Board.is_attacked_by` enumerated the complete maps. No mShogi output established an answer. Agreement with one oracle remains bounded evidence.

```sh
cargo test -p mshogi-core --test attacks --locked
/tmp/mshogi-chess-oracle/bin/python tools/check_chess_attacks.py
```

The second, optional command uses the separately installed environment described above; it verifies the committed answers and never rewrites them.

## Ordinary candidate fixture family

MSH-7 adds [ordinary-candidates.json](ordinary-candidates.json), a separate family named `mshogi.chess.ordinary-candidates.v1` with `schema_version: 1`. Each case has a stable ID, the exact rules-profile ID, a canonical `fen`, coverage `tags`, the sorted unique `candidates` (pseudo-legal ordinary actions in the coordinate test adapter, castling and en passant excluded), the sorted `legal_ordinary` subset, the `hand_expectations` written before oracle enumeration (candidate count, required and excluded actions, legal-ordinary count) and source/method/rationale provenance. Compare entire sets; order is irrelevant.

There are 17 positions, 274 exact candidates and 241 legal ordinary actions. Diagrams cover both initial sides, blocked single and double pawn advances of both colors, edge pawn captures, quiet/capturing/blocked promotions of both colors with all four choices, corner knights, bishop and queen, slider blockers and captures, pinned pieces and sides in check of both colors, and positions where en passant or castling is legal but excluded.

For each case the diagram, candidate count, required and excluded actions and legal-ordinary count were first written by hand from the rationale. The pinned `chess==1.11.2` `generate_pseudo_legal_moves` and `legal_moves`, minus `is_castling`/`is_en_passant`, then enumerated the exact sets and had to agree with every hand expectation. The stored hand expectations are rechecked by the Rust suite and the oracle checker. Two hand legal-ordinary counts were miscounted at first. In `pawn-edge-captures` only e1f2 is unsafe (the b3 pawn attacks a2/c2, not d2), and in `corner-bishop-and-queen` every king step is safe. Both were rechecked by hand against the board and corrected; no mShogi output was consulted. Agreement with one oracle remains bounded evidence.

```sh
cargo test -p mshogi-core --test ordinary_candidates --locked
/tmp/mshogi-chess-oracle/bin/python tools/check_chess_ordinary_candidates.py
```

## Special action fixture family

MSH-8 adds [special-actions.json](special-actions.json), a separate family named `mshogi.chess.special-actions.v1` with `schema_version: 1`. Its root has three arrays, and IDs are unique across all three:

- `positions`: a canonical `fen`, the hand-curated `special_candidates` and `legal_special` sets, and the complete oracle `legal` set;
- `sequences`: `before_fen` and ordered `steps` of `{ action, after_fen }`, each containing at least one castling or en passant action;
- `rejections`: a `fen`, an `action`, the expected profile `error` category, and `oracle_legal`, which is true only when the action is FIDE-legal but rejected by the profile's u32 counter domain.

Every case carries tags and source/method/rationale provenance. Actions use the coordinate test adapter, with castling named by the king's move.

There are 21 positions, 6 sequences with 25 exact steps, and 18 rejections. They cover:

- both castlings of both colors;
- missing rights, own and opponent blockers;
- check, an attacked transit or destination, an attacked rook and an attacked b1;
- en passant for both colors, two capturers, expiry and an uncapturable raw target;
- rank discoveries of both colors, a diagonal discovery, a pinned capturer and en passant as a check evasion;
- a rook moved away and back, which does not restore the right;
- castling at the counter limit.

All special candidate sets, legal specials and rejection errors were written by hand from the FIDE/profile rationale first. The pinned `chess==1.11.2` `legal_moves` with `is_castling`/`is_en_passant` then had to agree, before the complete legal sets and after-FENs were enumerated. Every hand expectation matched on the first run. Special candidates and error categories are profile contracts that the oracle cannot supply; they are checked by hand and by the Rust contract test.

```sh
cargo test -p mshogi-core --test special_actions --locked
/tmp/mshogi-chess-oracle/bin/python tools/check_chess_special_actions.py
```

## Classification fixture family

MSH-9 adds [classification.json](classification.json), a separate family named `mshogi.chess.classification.v1` with `schema_version: 1`. It fills gaps in the corpus's classification coverage, which has five White checks, no Black check and one mate/stalemate per color.

Each case has a stable ID, the exact rules-profile ID, a canonical `fen`, tags, the `hand_expectations` written before oracle enumeration (classification, legal count and required legal actions), the exact `classification`, the complete sorted `legal` set, and provenance.

The 8 positions are: a Black single check, a Black double check, a smothered mate, a mate by a protected queen, a White back-rank mate, a stalemate with a pinned knight, a Black en passant check evasion, and bare kings at the u32 counter limit. The last records the profile decision that legality ignores the counter domain.

Every hand expectation matched pinned `chess==1.11.2` `legal_moves`/`is_check`/`is_checkmate`/`is_stalemate` on the first run.

```sh
cargo test -p mshogi-core --test legal_actions --locked
/tmp/mshogi-chess-oracle/bin/python tools/check_chess_classification.py
```
