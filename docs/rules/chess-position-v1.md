# Chess position profile v1

Profile ID: `mshogi.chess.position.v1`. Established by MSH-2 on 2026-10-23 for the M1 chess rules kernel. This document specifies an implementation target; it does not certify the retained Rust prototype. The six-game requirements in [the design](../design.md) remain unchanged.

## Scope and sources

The rule reference is the English [FIDE Laws effective 2023-01-01](https://handbook.fide.com/chapter/E012023), especially Articles 3 and 5. The interchange reference is Edwards's [PGN specification, section 16.1](https://www.saremba.de/chessgml/standards/pgn/pgn-complete.htm#c16.1). This is orthodox chess with the usual starting squares, not Chess960 or a historical variant.

M1 implements position-local legal actions, attacks, reversible transitions, FEN, perft/divide and a headless inspection console. Full game adjudication is deferred. `quiet`, `check`, `checkmate` and `stalemate` classify a position; `quiet` and `check` make no assertion about whether a match may continue. In particular, bare kings can have legal actions for these diagnostics even though a complete rules implementation would recognize a dead position. See the [coverage ledger](chess-coverage.md).

## Position and action semantics

A saved position comprises piece kind/color/square, side to move, four independent castling rights, the raw en passant target (if any), halfmove clock and fullmove number. There is exactly one king per color in this profile. There are no clocks, player assignments, result, claims, repetition occurrences or undo records in FEN.

Actions specify from/to and any promotion choice. Fixture strings use lowercase coordinate notation, e.g. `e2e4`, `e1g1`, `a7b8n`; this is a test adapter, not a required universal action encoding or a UCI protocol implementation. Castling names the king's destination (`e1c1`, `e8g8`), never its rook's square. Null moves and missing promotion choices are not legal player actions. Resolution determines all captures and rook movement, rather than trusting a caller's victim list.

Movement follows FIDE Article 3. Own pieces block; sliders stop at the first occupied square; knights jump. A pawn's double advance begins on its home rank and needs both squares empty. Its captures differ from advances. Promotion is part of the same action and requires exactly one of queen, rook, bishop or knight, including on captures. No king is ever captured.

Attacks are geometric with occupancy and pawn capture directions, not the attacker's legal-move list: pinned pieces and kings still constrain the opposing king. Legal actions leave the moving king safe after **all** effects, including removal of the en passant victim. Castling also requires the right, home king/rook, empty intervening squares (including b1/b8 for queenside), and safe king origin/transit/destination. The rook being attacked alone does not prohibit castling.

Castling rights are historical permissions, not inferred from home-square occupancy. Moving the king loses both rights; moving a home rook or capturing it on its home square loses the associated right; moving a replacement/promoted rook there never restores it. The en passant opportunity expires after the next action. A double pawn advance records its passed square even if no opponent pawn has a legal capture. A pawn move or capture resets the halfmove clock; other actions increment it. The fullmove number increments after Black's action. All position fields and any future derived cache must restore exactly on undo. An invalid action or counter-overflow failure leaves the caller's state unchanged.

Classify the side to move by `(in_check, has_legal_action)`: `(false,true)` = `quiet`, `(true,true)` = `check`, `(true,false)` = `checkmate`, `(false,false)` = `stalemate`. Perft counts legal action sequences of exactly the specified depth: depth zero is one, even at mate/stalemate; a node with no actions contributes zero at positive depth. Do not stop perft at a draw counter or an insufficient-material shortcut. Divide maps each legal root action to its perft count at the remaining depth and sums to perft at that root depth.

## FEN boundary contract

These are explicit project input-policy choices on top of FEN. They distinguish syntax/range/local structure from a proof of legal historical reachability. A successful parse is an **analysis position accepted by this profile**, not proof that a legal game led to it (FIDE 3.10.3).

Validation proceeds in this order; errors are deterministic categories `syntax`, `range`, then `structure`. Within a category an implementation may add a field/reason diagnostic. Validate into a temporary position and publish it only on success. When several faults coexist the earliest category wins. MSH-4 owns the parser and its tests.

1. **Syntax.** Accept exactly six nonempty ASCII fields after trimming/splitting ASCII whitespace (space, tab, CR, LF, vertical tab, form feed). Reject non-ASCII input and extra/missing fields. Piece placement has eight `/`-separated ranks, each expanding to eight squares, using only `PNBRQKpnbrqk` and `1` through `8`. Consecutive empty-run digits are rejected. Side is `w` or `b`. Rights are `-` or a nonempty subsequence of `KQkq` in that order without repetition; mixed `-`, Chess960 letters and reordered rights are rejected. Target is `-` or `[a-h][1-8]`. Counters contain only one or more ASCII digits: no sign, decimal point or exponent. Leading zeroes are accepted.
2. **Range.** Both counters use the unsigned 32-bit domain `0..4294967295`; fullmove additionally must be at least one. Never wrap, clamp or silently replace zero with one. Operations that would overflow must fail atomically. A syntactically valid but wrong-side EP rank belongs to the following structure checks.
3. **Local structure.** Exactly one king of each color, nonadjacent kings, at most eight pawns and sixteen pieces per color, and no pawns on rank one/eight. Each asserted castling right requires its color's king on e1/e8 and rook on a1/h1/a8/h8. Missing rights are allowed with home pieces. Retain asserted rights when paths are blocked or attacked; do not silently clean inconsistent rights.
4. **Local structure, EP.** If present, target is empty and on rank six with White to move or rank three with Black to move. The opposing pawn must occupy its double-push destination immediately beyond the target; its home-square origin must be empty and the halfmove clock must be zero. A capturing pawn need not exist, and any such capture need not be legal. Do not infer or require an entire last-move record. These local checks intentionally do not reconstruct the previous position or prove that its move was legal.
5. **Local structure, king safety.** The side **not** to move must not currently have its king attacked. The moving side may be in check, including double check. This rejects both-kings-in-check positions without confusing current check with malformed input.

No promotion-material budget, retrograde analysis, minimum fullmove count for piece placement, possible checker-origin analysis or full game reconstruction is required. For example three queens plus eight pawns can pass the local checks even though that material is unreachable. Do not label it a valid historical FIDE position. Rejection of additional diagrams requires a versioned policy change, not adoption of another library's stricter validity bitmask by accident. Unsupported extensions such as pockets, promoted-piece markers and Chess960 rights fail syntax; FEN is not the six-game save format.

### Serialization and identity

Output is canonical six-field FEN: eight ranks with maximal empty runs, a single ASCII space between fields, standard piece case, rights in `KQkq` order or `-`, lowercase target, minimal unsigned decimal counters and no surrounding whitespace. Preserve the raw target even when it cannot yield a legal en passant capture; use `fen(en_passant="fen")` when consulting python-chess, whose default output can omit it.

Two inputs are semantically equivalent for this boundary iff they expand to exactly the same tuple of board, side, rights, raw target and both numeric counters. Whitespace and leading numeric zeroes are insignificant. Equal boards alone, equal legal action sets, or equal repetition keys do not establish saved-position equality. A raw uncapturable target and `-` are distinct saved states here even if they permit the same actions. Repetition identity has its own rule-dependent semantics, and neither FEN nor counters establish repetition history, a claim entitlement or a safe search-cache identity.

## Fixtures and implementation handoff

[Corpus format and reproduction](../../tests/fixtures/chess/README.md) describes independently sourced cases, exact move sets, shallow perft, attack queries and FEN acceptance contracts. Checkmate/stalemate and both colors of promotion/castling/en passant are represented. Sparse cases isolate king safety; six maintained Stockfish reference positions exercise combinations.

MSH-4 consumes parser contracts; MSH-5 through MSH-9 consume transitions, attacks and move/classification expectations; MSH-10 consumes perft/divide; MSH-11 exposes the profile identity and limitations; MSH-12 adds sequence/differential evidence; MSH-13 remains a synthetic review of broader action semantics. Those tasks are not completed by this corpus. Profile or fixture corrections require a source/rationale and recorded review; do not regenerate expectations from mShogi to make a failure disappear.
