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

## Rust position API

MSH-4 implements `mshogi_core::chess::Position::from_fen`, the equivalent `FromStr`, canonical `to_fen`/`Display`, and atomic `set_fen`. Read-only accessors expose the piece at a bounded `Square`, side, historical `CastlingRights`, raw target and both counters. `Color`, `PieceKind` and `Piece` describe this chess profile only. A direct private 64-cell board uses a1 index 0 through h8 index 63; no representation compression or incremental cache is introduced. Fields cannot be mutated through the public API, so construction/replacement preserve the documented invariants.

`FenError::category()` returns `Syntax`, `Range` or `Structure`; display adds a stable field/reason diagnostic. All syntax fields are validated before numeric ranges, then local structure. Rejected input publishes no partial position. The nonmoving-king invariant uses the public geometric attack query described below, so pinned attackers and blockers are handled by one implementation.

Derived position equality compares the entire saved-state tuple, including raw target and both counters. No match history, repetition/cache key, draw claim, result or legal-reachability proof is created by parsing or equality. The 68 unchanged MSH-2 inputs are consumed by `cargo test -p mshogi-core --test fen_corpus --locked`: 41 accepted inputs with exact canonical expectations and 27 rejected contracts with expected categories and unchanged caller state. Move/perft/classification expectations remain untested by this parser suite.

## Rust ordinary transition and undo API

MSH-5 adds the deliberately chess-specific `Action { from, to, promotion }`, with bounded squares and a four-way `Promotion` enum. It has no moving-piece or victim/effect list supplied by the client. `Position::apply_ordinary` is the checked public boundary: validate source/side, occupancy, piece/pawn geometry and path, promotion choice and u32 counter bounds; derive capture/placement from the board; apply to a temporary candidate; validate resulting local invariants/own-king safety; publish only on success. Being in check before an action is allowed if the action resolves it. This is not a move generator or match-outcome adjudicator.

Ordinary king moves lose both rights; moving/capturing a home rook loses its associated right. A double pawn advance records the passed square even without a legal EP capturer; every other action clears the old target. Pawn actions/captures reset the halfmove clock, other moves increment it, and Black increments the fullmove number. Resets can succeed at maximum halfmove; required increments beyond u32 reject without mutation. Every action error leaves the entire input position unchanged. `apply_ordinary` remains the ordinary-only subset: castling-shaped standard king actions and EP captures return `CastlingNotOrdinary`/`EnPassantNotOrdinary` without judging their legality. MSH-8 executes them through `Position::apply` (below).

Resolution and `apply_resolved` are private: only the checked boundaries (`apply_ordinary`, and since MSH-8 `apply`) can construct resolved effects, and the trusted primitive runs on its temporary clone. Path checks and the structural king-safety check share the attack module's ray geometry. No unchecked public mutator is provided.

On success, opaque `Undo` stores the full before/after positions. `Position::undo(&record)` compares **all** current saved-state fields with the stored post-state and restores the full prior snapshot; mismatched/out-of-order/immediately repeated undo fails atomically. The borrowed record may be used again if the exact same post-state is reached. It is not an origin nonce or proof of game history; callers own correct LIFO order and game context, and identical saved values in different contexts are not distinguished. This record is private in-memory implementation state, not a canonical save/notation format. There are no derived caches yet; full-state snapshots avoid missed-field restoration without speculative compression.

The transition fixture family is separate from the original corpus: [ordinary-transitions.json](../../tests/fixtures/chess/ordinary-transitions.json), format `mshogi.chess.ordinary-transitions.v1`, has 49 independently checked sequences and 74 exact after-FEN expectations. Rust tests verify each forward state and every reverse-step full-state restoration, plus invalid and non-ordinary actions, counter overflow and undo mismatch. These checks alone do not establish castling/EP execution, attack queries, generated move sets, full adjudication or the larger games. MSH-13 reviews how these concrete chess-only boundaries map to broader complete actions; no universal route/effect representation is claimed here.

## Rust attack query API

MSH-6 adds `Position::attackers(square, by)`, which yields every square holding a piece of color `by` that attacks `square` in a1-to-h8 order, and `Position::is_square_attacked(square, by)`. Both are read-only and depend only on the current board; side to move, castling rights, raw target and counters are ignored.

Semantics, per FIDE Articles 3.2 to 3.8 and this profile:

- A pawn attacks the one or two squares diagonally forward for its color, whether empty, occupied by an opponent piece or occupied by its own piece. It never attacks straight ahead or backward.
- Knights and kings attack their fixed offsets; a knight is never blocked.
- Bishops, rooks and queens attack along their lines up to **and including** the first occupied square of either color, and nothing beyond it. There is no x-ray through any piece, including a friendly slider or a king.
- Squares never wrap across a board edge, and a piece never attacks its own square.
- An attack is not legal-move membership. A pinned piece still attacks, a king attacks squares adjacent to it even when they are defended, and a target occupied by the attacker's own piece is attacked (defended). Attacks on a king square are allowed and mean check.
- En passant and castling are not attacks. A pawn attacks a raw target square diagonally in front of it like any other square. En passant adds no attack on the victim's square; that square is attacked only by pieces that geometrically attack it. Castling does not attack anything.

Legality tasks evaluate attacks on the position **after** all effects of a candidate action. In particular a king stepping along a checking line, castling transit and an en passant capture that clears two squares from a rank each need the post-action occupancy; querying the pre-action board answers a different question. No incremental attack table, bitboard or cache is maintained.

The separate fixture family [attacks.json](../../tests/fixtures/chess/attacks.json), format `mshogi.chess.attacks.v1`, has 29 curated diagrams with 155 exact attacker sets and complete attacked-square maps of both colors. These verify attack queries only. They do not establish legal move generation, castling/en passant execution, check classification or the larger games.

## Rust ordinary candidate API

MSH-7 adds `Position::ordinary_candidates()`, returning a `Vec<Action>` for the side to move. These are **pseudo-legal ordinary candidates, not legal moves**. MSH-9 adds the public legal-action list and classification; MSH-8 adds the castling and en passant candidates (below).

The set contains every ordinary piece move or capture, pawn single advance, pawn double advance from the home rank through an empty square, and diagonal pawn capture of an opponent piece. Every last-rank pawn destination appears as four distinct actions, one for each of queen, rook, bishop and knight; a last-rank action without a choice never appears. Movement shape, own-piece blocking, slider rays up to the first occupied square, board edges and destination occupancy are already correct. Castling, en passant, own-piece captures and king captures are never included, and there are no duplicates. Order is deterministic but unspecified and is not a search ordering.

Candidates are **not** filtered for the moving side's king safety or counter bounds. A pinned piece's moves, king steps onto attacked squares and moves that ignore a check are all candidates. The contract with the checked boundary is exact:

- every candidate is either accepted by `apply_ordinary` or rejected only as `IllegalPosition` (the moving king would be unsafe) or `CounterOverflow`;
- `apply_ordinary` accepts no action outside the candidate set.

Final filtering must therefore still establish own-king safety after all effects, and add the special actions, before any set is called legal. Counter bounds do not affect legality (see the legal action API below); they only make `apply` fail atomically at the u32 limit. The `ordinary_candidates` contract with `apply_ordinary` is unchanged by MSH-8.

The separate fixture family [ordinary-candidates.json](../../tests/fixtures/chess/ordinary-candidates.json), format `mshogi.chess.ordinary-candidates.v1`, has 17 curated positions with exact candidate sets (274 actions) and exact legal ordinary subsets (241 actions). Tests also check every bounded action shape against this contract on those positions and on the 35 corpus positions. They do not establish castling, en passant, the full legal-move list, perft or classification.

## Rust special action API

MSH-8 adds `Position::apply(action)`, the checked boundary for every complete chess action: ordinary moves, captures and promotions exactly as `apply_ordinary`, plus castling and en passant. It returns the same full-snapshot `Undo`, so every special mutation (rook placement, victim removal, rights, raw target, counters) restores exactly. Every rejection leaves the caller's state unchanged.

**Castling** is named by the king's two-file move from e1/e8 (`e1g1`, `e1c1`, `e8g8`, `e8c8`). Checks, in order:

1. The color holds the right for that side, its rook stands on the home corner, and every square between king and rook is empty, including b1/b8 for queenside. Otherwise the error is `CastlingUnavailable`. Castling never captures.
2. No promotion choice is supplied (`UnexpectedPromotion`).
3. The king is not in check, and the square it crosses (f or d file) is not attacked. Otherwise the error is `CastlingKingAttacked`.
4. The counters can advance (`CounterOverflow`).
5. The king's destination is judged on the resulting board by the same structural king-safety check every action uses, and an attacked destination is rejected as `IllegalPosition`. An attacked rook, or an attacked b1/b8, does not prevent castling.

The rook moves to the square the king crossed. Both of the color's rights are lost, the raw target clears, and the halfmove clock increments, because castling is neither a pawn move nor a capture.

**En passant** is named by the capturing pawn's diagonal move onto the raw target. The victim is the opposing pawn on the capturer's rank in the destination file, which FEN and transition invariants guarantee. The capture removes the victim, resets the halfmove clock and clears the target. Own-king safety is judged after both the mover and the victim have left their squares, so rank discoveries, diagonal discoveries and pinned capturers are rejected as `IllegalPosition`. Without a raw target the same move is `IllegalGeometry`; with a promotion choice it is `UnexpectedPromotion`.

`Position::special_candidates()` returns the pseudo-legal special actions:

- castling for each side whose right, home rook and empty path are present, whether or not the king is in check or would cross or land on an attacked square;
- en passant for each own pawn diagonally behind the raw target, whether or not the capture exposes its king.

`Position::candidates()` is the ordinary candidates followed by the special ones. The contract with `apply` is exact:

- `apply` accepts no action outside `candidates()`;
- `apply` rejects a candidate only as `CastlingKingAttacked`, `IllegalPosition` or `CounterOverflow`.

These are still **not** a legal-move list or a classification; see the legal action API below. Tests show that filtering `candidates()` through `apply` reproduces the complete independent legal set of all 35 corpus positions.

The separate fixture family [special-actions.json](../../tests/fixtures/chess/special-actions.json), format `mshogi.chess.special-actions.v1`, holds:

- 21 positions with hand-curated special candidates, legal specials and the complete independent legal set;
- 6 sequences (25 exact after-states) mixing castling, en passant and ordinary actions;
- 18 rejections with expected error categories.

## Rust legal action and classification API

MSH-9 adds `Position::legal_actions()`, `Position::in_check()` and `Position::classify()`.

`legal_actions()` returns every legal action of the side to move: ordinary moves and captures, every promotion choice, castling and en passant. Each must leave the moving king safe after all effects. It is exactly `candidates()` filtered by the checked `apply`, evaluated on a private copy whose counters are normalized.

**Decision: legality does not depend on the u32 counter domain.** Counters never affect a chess rule's legality. The profile's counter domain is a representation limit that makes an overflowing operation fail atomically; it is not a rule that removes legal actions. Consequences:

- at the limit, `apply` can reject a listed action as `CounterOverflow`. That is a representation failure (the resulting state cannot be represented), not a finding of illegality, and it leaves the whole position unchanged;
- `apply` accepts every other listed action, and nothing outside the list;
- positions that differ only in counters have identical legal sets and classifications, so bare kings at halfmove 4294967295 remain `quiet` rather than becoming a false stalemate, and a check whose every reply would overflow stays `check`, not checkmate.

Approved by John on 2026-10-23 (MSH-9), including the corresponding wording of the candidate contract above.

Order is deterministic but unspecified. It is not a search ordering.

`in_check()` is whether the side to move's king is attacked. `classify()` returns `Classification::{Quiet, Check, Checkmate, Stalemate}` from `(in_check, has_legal_action)`, as specified above.

This is the limited M1 position-level classifier, not complete game adjudication. A `quiet` result means only "not in check, with a legal action"; for example bare kings are `quiet` although FIDE 5.2.2 would end that game as a dead position. `Quiet` and `Check` do not mean a game may continue, and there is no dead-position, repetition, 50/75-move, agreed-draw, claim, resignation or clock logic; the coverage ledger keeps those obligations open.

Evidence:

- exact legal sets and classifications for all 35 corpus positions and for the separate fixture family [classification.json](../../tests/fixtures/chess/classification.json), format `mshogi.chess.classification.v1` (8 diagrams);
- exact legal sets for the 21 special-action positions;
- for every one of those positions, every listed action applies and undoes exactly (except a `CounterOverflow` rejection at the u32 limit), and every other bounded action is rejected without mutation.

Perft/divide (MSH-10) and console exposure (MSH-11) are separate work. MSH-10 must define perft from a position at the counter limit, where depth-2 counts cannot be reached by applying listed actions; normalizing counters as `legal_actions` does is the consistent choice.
