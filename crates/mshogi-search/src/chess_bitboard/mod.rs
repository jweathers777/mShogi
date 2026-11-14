//! Search-internal Chess position: bitboards, make/unmake in place, incremental
//! hash and incremental material/placement.
//!
//! This is the engine's private analysis board. The native rules in
//! `mshogi_core::chess` stay the authority for each real game, result, claim
//! and history. Perft and differential tests prove that this board agrees with
//! the native legal actions and transitions.
pub(crate) mod attacks;
#[cfg(test)]
pub(crate) mod tests;

use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use attacks::{BLACK, TABLES, WHITE};
use mshogi_core::chess::{Action, AdjudicatedGame, Color, PieceKind, Position, Promotion, Square};
use std::sync::LazyLock;

pub(crate) const PAWN: usize = 0;
pub(crate) const KNIGHT: usize = 1;
pub(crate) const BISHOP: usize = 2;
pub(crate) const ROOK: usize = 3;
pub(crate) const QUEEN: usize = 4;
pub(crate) const KING: usize = 5;
const EMPTY: u8 = 12;
const NO_SQUARE: u8 = 64;

/// Material values in centipawns: pawn 100, knight 320, bishop 330, rook 500,
/// queen 900, king 0.
pub(crate) const VALUE: [i32; 6] = [100, 320, 330, 500, 900, 0];

const FLAG_NORMAL: u16 = 0;
const FLAG_DOUBLE: u16 = 1;
const FLAG_EN_PASSANT: u16 = 2;
const FLAG_CASTLE: u16 = 3;
const FLAG_PROMOTE_KNIGHT: u16 = 4;
const FLAG_PROMOTE_QUEEN: u16 = 7;

/// Compact move: origin, destination and a kind flag. Zero is never a move.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Move(u16);
impl Move {
    #[inline]
    fn new(from: usize, to: usize, flag: u16) -> Self {
        Self(from as u16 | (to as u16) << 6 | flag << 12)
    }
    #[inline]
    pub(crate) fn from(self) -> usize {
        usize::from(self.0 & 63)
    }
    #[inline]
    pub(crate) fn to(self) -> usize {
        usize::from(self.0 >> 6 & 63)
    }
    #[inline]
    fn flag(self) -> u16 {
        self.0 >> 12
    }
    /// Promotion piece kind index, if this move promotes.
    #[inline]
    fn promotion(self) -> Option<usize> {
        (self.flag() >= FLAG_PROMOTE_KNIGHT).then(|| usize::from(self.flag() - 3))
    }
}

struct Keys {
    piece: [[u64; 64]; 12],
    castling: [u64; 16],
    en_passant: [u64; 8],
    side: u64,
}
static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let mut state = 0x0123_4567_89AB_CDEFu64;
    let mut next = || {
        // splitmix64
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };
    let mut piece = [[0; 64]; 12];
    for row in &mut piece {
        for key in row.iter_mut() {
            *key = next();
        }
    }
    let mut castling = [0; 16];
    for key in castling.iter_mut().skip(1) {
        *key = next();
    }
    let mut en_passant = [0; 8];
    for key in &mut en_passant {
        *key = next();
    }
    Keys {
        piece,
        castling,
        en_passant,
        side: next(),
    }
});

/// Placement values indexed by color, piece kind and square.
type PlacementTable = [[[(i32, i32); 64]; 6]; 2];

/// Middle-game and endgame placement value of each piece on each square, the
/// same arithmetic as the earlier conventional evaluation.
pub(crate) static PLACEMENT: LazyLock<PlacementTable> = LazyLock::new(|| {
    let mut table = [[[(0, 0); 64]; 6]; 2];
    for (color, by_kind) in table.iter_mut().enumerate() {
        for (kind, by_square) in by_kind.iter_mut().enumerate() {
            for (sq, value) in by_square.iter_mut().enumerate() {
                let (file, rank) = ((sq % 8) as i32, (sq / 8) as i32);
                let relative = if color == WHITE { rank } else { 7 - rank };
                let advance = (relative - 1).max(0);
                let center = (14 - (2 * file - 7).abs() - (2 * rank - 7).abs()) / 2;
                let file_center = 3 - (file - 3).abs().min((file - 4).abs());
                let home = i32::from(relative == 0);
                *value = match kind {
                    PAWN => (
                        4 * advance + 4 * file_center,
                        10 * advance + 2 * file_center,
                    ),
                    KNIGHT => (12 * center - 8 * home, 8 * center),
                    BISHOP => (7 * center + 3 * advance - 8 * home, 6 * center),
                    ROOK => (2 * center, 4 * center),
                    QUEEN => (3 * center - 6 * i32::from(relative > 1), 5 * center),
                    _ => (-10 * center - 8 * advance, 14 * center),
                };
            }
        }
    }
    table
});

/// Castling-right bits that stay after a move touches a square.
const fn rights_mask(sq: usize) -> u8 {
    match sq {
        0 => !2,
        4 => !3,
        7 => !1,
        56 => !8,
        60 => !12,
        63 => !4,
        _ => !0,
    }
}

#[derive(Clone, Copy)]
struct Undo {
    mv: Move,
    captured: u8,
    castling: u8,
    en_passant: u8,
    halfmove: u16,
    since_null: u16,
    hash: u64,
    checkers: u64,
}

/// Chess position for the shared search core.
#[derive(Clone)]
pub struct ChessPosition {
    pub(crate) pieces: [[u64; 6]; 2],
    pub(crate) occupied: [u64; 2],
    board: [u8; 64],
    pub(crate) side: usize,
    castling: u8,
    en_passant: u8,
    halfmove: u16,
    since_null: u16,
    hash: u64,
    checkers: u64,
    pub(crate) material: [i32; 2],
    pub(crate) placement: [(i32, i32); 2],
    pub(crate) phase: i32,
    stack: Vec<Undo>,
    /// Hashes of all earlier positions: the game before the root, then the path.
    keys: Vec<u64>,
    /// Number of `keys` that are from the game before the search root.
    root_keys: usize,
}

#[inline]
fn bit(sq: usize) -> u64 {
    1u64 << sq
}
#[inline]
fn lsb(map: u64) -> usize {
    map.trailing_zeros() as usize
}

fn square(index: usize) -> Square {
    Square::new((index % 8) as u8, (index / 8) as u8).expect("index below 64")
}
fn index(square: Square) -> usize {
    usize::from(square.rank()) * 8 + usize::from(square.file())
}
fn kind_index(kind: PieceKind) -> usize {
    match kind {
        PieceKind::Pawn => PAWN,
        PieceKind::Knight => KNIGHT,
        PieceKind::Bishop => BISHOP,
        PieceKind::Rook => ROOK,
        PieceKind::Queen => QUEEN,
        PieceKind::King => KING,
    }
}

impl ChessPosition {
    /// Copy a native position. The result has no earlier history.
    pub fn from_position(position: &Position) -> Self {
        let mut result = Self {
            pieces: [[0; 6]; 2],
            occupied: [0; 2],
            board: [EMPTY; 64],
            side: usize::from(position.side_to_move() == Color::Black),
            castling: 0,
            en_passant: position
                .en_passant_target()
                .map_or(NO_SQUARE, |sq| index(sq) as u8),
            halfmove: position.halfmove_clock().min(u32::from(u16::MAX)) as u16,
            since_null: u16::MAX,
            hash: 0,
            checkers: 0,
            material: [0; 2],
            placement: [(0, 0); 2],
            phase: 0,
            stack: Vec::with_capacity(256),
            keys: Vec::with_capacity(512),
            root_keys: 0,
        };
        for sq in 0..64 {
            if let Some(piece) = position.piece_at(square(sq)) {
                result.put(
                    usize::from(piece.color == Color::Black),
                    kind_index(piece.kind),
                    sq,
                );
            }
        }
        let rights = position.castling_rights();
        result.castling = u8::from(rights.kingside(Color::White))
            | u8::from(rights.queenside(Color::White)) << 1
            | u8::from(rights.kingside(Color::Black)) << 2
            | u8::from(rights.queenside(Color::Black)) << 3;
        result.hash = result.computed_hash();
        result.checkers = result.attackers_of_king(result.side);
        result
    }

    /// Build the current position of a game with the hashes of its earlier
    /// positions, so that the search can see repetitions of game positions.
    /// If this board cannot replay the native history, the result is the current
    /// position without history.
    pub fn from_game(game: &AdjudicatedGame) -> Self {
        let history = game.history();
        let mut result = Self::from_position(history.initial_position());
        let mut replayed = true;
        for action in history.actions() {
            match result.find(action) {
                Some(m) if result.make(m) => {}
                _ => {
                    replayed = false;
                    break;
                }
            }
        }
        if !replayed {
            debug_assert!(false, "search board could not replay the native history");
            result = Self::from_position(game.position());
        }
        result.stack.clear();
        result.root_keys = result.keys.len();
        result.since_null = u16::MAX;
        result
    }

    /// Native action for a move of this board.
    pub fn action(&self, m: Move) -> Action {
        Action {
            from: square(m.from()),
            to: square(m.to()),
            promotion: m.promotion().map(|kind| match kind {
                KNIGHT => Promotion::Knight,
                BISHOP => Promotion::Bishop,
                ROOK => Promotion::Rook,
                _ => Promotion::Queen,
            }),
        }
    }

    /// Pseudo-legal move of this board for a native action, if one matches.
    pub fn find(&self, action: Action) -> Option<Move> {
        let mut moves = Vec::with_capacity(64);
        self.generate(Stage::Tactical, &mut moves);
        self.generate(Stage::Quiet, &mut moves);
        moves.into_iter().find(|&m| self.action(m) == action)
    }

    /// All legal moves, for tests and root checks.
    pub fn legal_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);
        self.generate(Stage::Tactical, &mut moves);
        self.generate(Stage::Quiet, &mut moves);
        moves.retain(|&m| {
            let legal = self.make(m);
            if legal {
                self.unmake();
            }
            legal
        });
        moves
    }

    /// Count leaf nodes of the legal move tree to `depth`.
    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut moves = Vec::with_capacity(64);
        self.generate(Stage::Tactical, &mut moves);
        self.generate(Stage::Quiet, &mut moves);
        let mut total = 0;
        for m in moves {
            if self.make(m) {
                total += self.perft(depth - 1);
                self.unmake();
            }
        }
        total
    }

    /// Side to move as a native color.
    pub fn side_to_move(&self) -> Color {
        if self.side == WHITE {
            Color::White
        } else {
            Color::Black
        }
    }

    #[inline]
    pub(crate) fn all(&self) -> u64 {
        self.occupied[0] | self.occupied[1]
    }
    #[inline]
    pub(crate) fn kind_at(&self, sq: usize) -> Option<(usize, usize)> {
        let code = self.board[sq];
        (code != EMPTY).then(|| (usize::from(code) / 6, usize::from(code) % 6))
    }
    #[inline]
    pub(crate) fn king_square(&self, color: usize) -> usize {
        lsb(self.pieces[color][KING])
    }
    pub(crate) fn has_castling_rights(&self) -> bool {
        self.castling != 0
    }

    #[inline]
    fn put(&mut self, color: usize, kind: usize, sq: usize) {
        self.pieces[color][kind] |= bit(sq);
        self.occupied[color] |= bit(sq);
        self.board[sq] = (color * 6 + kind) as u8;
        self.material[color] += VALUE[kind];
        let (middle, end) = PLACEMENT[color][kind][sq];
        self.placement[color].0 += middle;
        self.placement[color].1 += end;
        if kind != PAWN && kind != KING {
            self.phase += VALUE[kind];
        }
    }
    #[inline]
    fn remove(&mut self, color: usize, kind: usize, sq: usize) {
        self.pieces[color][kind] &= !bit(sq);
        self.occupied[color] &= !bit(sq);
        self.board[sq] = EMPTY;
        self.material[color] -= VALUE[kind];
        let (middle, end) = PLACEMENT[color][kind][sq];
        self.placement[color].0 -= middle;
        self.placement[color].1 -= end;
        if kind != PAWN && kind != KING {
            self.phase -= VALUE[kind];
        }
    }

    /// The en passant file is part of the hash only when a capture is possible.
    fn en_passant_key(&self) -> u64 {
        if self.en_passant == NO_SQUARE {
            return 0;
        }
        let target = usize::from(self.en_passant);
        // Pawns of the side to move that attack the target stand on the squares
        // that an opposing pawn on the target would attack.
        if TABLES.pawn[1 - self.side][target] & self.pieces[self.side][PAWN] != 0 {
            KEYS.en_passant[target % 8]
        } else {
            0
        }
    }

    fn computed_hash(&self) -> u64 {
        let mut hash = 0;
        for sq in 0..64 {
            if self.board[sq] != EMPTY {
                hash ^= KEYS.piece[usize::from(self.board[sq])][sq];
            }
        }
        hash ^= KEYS.castling[usize::from(self.castling)];
        hash ^= self.en_passant_key();
        if self.side == BLACK {
            hash ^= KEYS.side;
        }
        hash
    }

    /// Pieces of both colors that attack a square, with the given occupancy.
    #[inline]
    pub(crate) fn attackers_to(&self, sq: usize, occupied: u64) -> u64 {
        let t = &*TABLES;
        let diagonal = self.pieces[0][BISHOP]
            | self.pieces[1][BISHOP]
            | self.pieces[0][QUEEN]
            | self.pieces[1][QUEEN];
        let straight = self.pieces[0][ROOK]
            | self.pieces[1][ROOK]
            | self.pieces[0][QUEEN]
            | self.pieces[1][QUEEN];
        (t.pawn[BLACK][sq] & self.pieces[WHITE][PAWN])
            | (t.pawn[WHITE][sq] & self.pieces[BLACK][PAWN])
            | (t.knight[sq] & (self.pieces[0][KNIGHT] | self.pieces[1][KNIGHT]))
            | (t.king[sq] & (self.pieces[0][KING] | self.pieces[1][KING]))
            | (t.bishop(sq, occupied) & diagonal)
            | (t.rook(sq, occupied) & straight)
    }
    #[inline]
    fn attacked(&self, sq: usize, by: usize) -> bool {
        let t = &*TABLES;
        let occupied = self.all();
        t.pawn[1 - by][sq] & self.pieces[by][PAWN] != 0
            || t.knight[sq] & self.pieces[by][KNIGHT] != 0
            || t.king[sq] & self.pieces[by][KING] != 0
            || t.bishop(sq, occupied) & (self.pieces[by][BISHOP] | self.pieces[by][QUEEN]) != 0
            || t.rook(sq, occupied) & (self.pieces[by][ROOK] | self.pieces[by][QUEEN]) != 0
    }
    fn attackers_of_king(&self, color: usize) -> u64 {
        self.attackers_to(self.king_square(color), self.all()) & self.occupied[1 - color]
    }

    /// Attack map of a piece of `kind` and `color` on `sq` with this occupancy:
    /// the same geometry as the native `attacks_as`.
    #[inline(always)]
    pub(crate) fn attacks_as(&self, color: usize, kind: usize, sq: usize) -> u64 {
        let t = &*TABLES;
        match kind {
            PAWN => t.pawn[color][sq],
            KNIGHT => t.knight[sq],
            BISHOP => t.bishop(sq, self.all()),
            ROOK => t.rook(sq, self.all()),
            QUEEN => t.bishop(sq, self.all()) | t.rook(sq, self.all()),
            _ => t.king[sq],
        }
    }

    fn castle_path_clear(&self, kingside: bool) -> bool {
        let us = self.side;
        let home = if us == WHITE { 4 } else { 60 };
        let right = if kingside { 1 } else { 2 } << (2 * us);
        if self.castling & right == 0 || self.checkers != 0 {
            return false;
        }
        let (empty, pass): (&[usize], [usize; 2]) = if kingside {
            (&[home + 1, home + 2], [home + 1, home + 2])
        } else {
            (&[home - 1, home - 2, home - 3], [home - 1, home - 2])
        };
        let occupied = self.all();
        empty.iter().all(|&sq| occupied & bit(sq) == 0)
            && pass.iter().all(|&sq| !self.attacked(sq, 1 - us))
    }

    fn pawn_moves(&self, stage: Stage, moves: &mut Vec<Move>) {
        let us = self.side;
        let them = 1 - us;
        let occupied = self.all();
        let (forward, start_rank, last_rank): (isize, usize, usize) =
            if us == WHITE { (8, 1, 7) } else { (-8, 6, 0) };
        let mut pawns = self.pieces[us][PAWN];
        while pawns != 0 {
            let from = lsb(pawns);
            pawns &= pawns - 1;
            let ahead = (from as isize + forward) as usize;
            let promotes = ahead / 8 == last_rank;
            if stage == Stage::Tactical {
                let mut targets = TABLES.pawn[us][from] & self.occupied[them];
                while targets != 0 {
                    let to = lsb(targets);
                    targets &= targets - 1;
                    if promotes {
                        for flag in (FLAG_PROMOTE_KNIGHT..=FLAG_PROMOTE_QUEEN).rev() {
                            moves.push(Move::new(from, to, flag));
                        }
                    } else {
                        moves.push(Move::new(from, to, FLAG_NORMAL));
                    }
                }
                if self.en_passant != NO_SQUARE
                    && TABLES.pawn[us][from] & bit(usize::from(self.en_passant)) != 0
                {
                    moves.push(Move::new(
                        from,
                        usize::from(self.en_passant),
                        FLAG_EN_PASSANT,
                    ));
                }
                if promotes && occupied & bit(ahead) == 0 {
                    moves.push(Move::new(from, ahead, FLAG_PROMOTE_QUEEN));
                }
            } else if occupied & bit(ahead) == 0 {
                if promotes {
                    for flag in FLAG_PROMOTE_KNIGHT..FLAG_PROMOTE_QUEEN {
                        moves.push(Move::new(from, ahead, flag));
                    }
                } else {
                    moves.push(Move::new(from, ahead, FLAG_NORMAL));
                    if from / 8 == start_rank {
                        let two = (ahead as isize + forward) as usize;
                        if occupied & bit(two) == 0 {
                            moves.push(Move::new(from, two, FLAG_DOUBLE));
                        }
                    }
                }
            }
        }
    }

    /// Static exchange evaluation: true if the exchange on the destination
    /// gains at least `threshold` for the mover. Pins are ignored.
    fn see_at_least(&self, m: Move, threshold: i32) -> bool {
        if m.flag() != FLAG_NORMAL && m.flag() != FLAG_DOUBLE {
            return threshold <= 0;
        }
        let (from, to) = (m.from(), m.to());
        let t = &*TABLES;
        let mut swap = self.kind_at(to).map_or(0, |(_, kind)| VALUE[kind]) - threshold;
        if swap < 0 {
            return false;
        }
        let mover = self.kind_at(from).map_or(0, |(_, kind)| kind);
        swap = if mover == KING { 20_000 } else { VALUE[mover] } - swap;
        if swap <= 0 {
            return true;
        }
        let mut occupied = self.all() ^ bit(from) ^ bit(to);
        let mut attackers = self.attackers_to(to, occupied);
        let diagonal = self.pieces[0][BISHOP]
            | self.pieces[1][BISHOP]
            | self.pieces[0][QUEEN]
            | self.pieces[1][QUEEN];
        let straight = self.pieces[0][ROOK]
            | self.pieces[1][ROOK]
            | self.pieces[0][QUEEN]
            | self.pieces[1][QUEEN];
        let mut side = self.side;
        let mut result = 1;
        loop {
            side = 1 - side;
            attackers &= occupied;
            let ours = attackers & self.occupied[side];
            if ours == 0 {
                break;
            }
            result ^= 1;
            let mut taken = false;
            for (kind, &value) in VALUE.iter().enumerate().take(QUEEN + 1) {
                let candidates = ours & self.pieces[side][kind];
                if candidates == 0 {
                    continue;
                }
                swap = value - swap;
                if swap < result {
                    return result != 0;
                }
                occupied ^= candidates.isolate_lowest_one();
                if kind == PAWN || kind == BISHOP || kind == QUEEN {
                    attackers |= t.bishop(to, occupied) & diagonal;
                }
                if kind == ROOK || kind == QUEEN {
                    attackers |= t.rook(to, occupied) & straight;
                }
                taken = true;
                break;
            }
            if !taken {
                // Only the king can capture: legal only if no enemy attacker stays.
                return if attackers & occupied & self.occupied[1 - side] != 0 {
                    result ^ 1 != 0
                } else {
                    result != 0
                };
            }
        }
        result != 0
    }

    fn repetition(&self) -> bool {
        let limit = usize::from(self.halfmove.min(self.since_null)).min(self.keys.len());
        let mut earlier = false;
        let mut back = 4;
        // The same side is to move each two plies; a repetition needs four.
        while back <= limit {
            let at = self.keys.len() - back;
            if self.keys[at] == self.hash {
                if at >= self.root_keys || earlier {
                    return true;
                }
                earlier = true;
            }
            back += 2;
        }
        false
    }

    /// Legality of a pseudo-legal quiet move of a piece that is not a pawn,
    /// without a change of the position. Such a move captures nothing, so only
    /// the occupancy changes.
    fn quiet_piece_move_is_legal(&self, m: Move) -> bool {
        let us = self.side;
        let (from, to) = (m.from(), m.to());
        let Some((_, kind)) = self.kind_at(from) else {
            return false;
        };
        if kind == PAWN {
            return false;
        }
        if m.flag() == FLAG_CASTLE {
            // Castling moves are generated fully legal.
            return true;
        }
        let enemies = self.occupied[1 - us];
        if kind == KING {
            self.attackers_to(to, self.all() ^ bit(from)) & enemies == 0
        } else {
            let occupied = self.all() ^ bit(from) | bit(to);
            self.attackers_to(self.king_square(us), occupied) & enemies == 0
        }
    }

    fn has_legal_move(&mut self) -> bool {
        let mut moves = Vec::with_capacity(64);
        self.generate(Stage::Tactical, &mut moves);
        self.generate(Stage::Quiet, &mut moves);
        moves.into_iter().any(|m| {
            let legal = self.make(m);
            if legal {
                self.unmake();
            }
            legal
        })
    }

    fn restore(&mut self, undo: Undo) {
        let m = undo.mv;
        let us = 1 - self.side;
        let them = self.side;
        let (from, to) = (m.from(), m.to());
        let (_, moved) = self.kind_at(to).expect("moved piece on destination");
        self.remove(us, moved, to);
        self.put(us, if m.promotion().is_some() { PAWN } else { moved }, from);
        if m.flag() == FLAG_CASTLE {
            let (rook_from, rook_to) = if to > from {
                (from + 3, from + 1)
            } else {
                (from - 4, from - 1)
            };
            self.remove(us, ROOK, rook_to);
            self.put(us, ROOK, rook_from);
        }
        if undo.captured != EMPTY {
            let at = if m.flag() == FLAG_EN_PASSANT {
                if us == WHITE { to - 8 } else { to + 8 }
            } else {
                to
            };
            self.put(them, usize::from(undo.captured) % 6, at);
        }
        self.side = us;
        self.castling = undo.castling;
        self.en_passant = undo.en_passant;
        self.halfmove = undo.halfmove;
        self.since_null = undo.since_null;
        self.hash = undo.hash;
        self.checkers = undo.checkers;
    }
}

impl SearchPosition for ChessPosition {
    type Move = Move;
    const HISTORY_SLOTS: usize = 2 * 6 * 64;

    #[inline]
    fn hash(&self) -> u64 {
        self.hash
    }
    #[inline]
    fn in_check(&self) -> bool {
        self.checkers != 0
    }

    fn generate(&self, stage: Stage, moves: &mut Vec<Move>) {
        let us = self.side;
        let occupied = self.all();
        let targets = if stage == Stage::Tactical {
            self.occupied[1 - us]
        } else {
            !occupied
        };
        self.pawn_moves(stage, moves);
        for kind in KNIGHT..=KING {
            let mut from_map = self.pieces[us][kind];
            while from_map != 0 {
                let from = lsb(from_map);
                from_map &= from_map - 1;
                let mut to_map = self.attacks_as(us, kind, from) & targets;
                while to_map != 0 {
                    let to = lsb(to_map);
                    to_map &= to_map - 1;
                    moves.push(Move::new(from, to, FLAG_NORMAL));
                }
            }
        }
        if stage == Stage::Quiet && self.castling != 0 {
            let home = if us == WHITE { 4 } else { 60 };
            if self.castle_path_clear(true) {
                moves.push(Move::new(home, home + 2, FLAG_CASTLE));
            }
            if self.castle_path_clear(false) {
                moves.push(Move::new(home, home - 2, FLAG_CASTLE));
            }
        }
    }

    fn make(&mut self, m: Move) -> bool {
        let us = self.side;
        let them = 1 - us;
        let (from, to, flag) = (m.from(), m.to(), m.flag());
        let (_, kind) = self.kind_at(from).expect("mover on origin");
        let capture_at = if flag == FLAG_EN_PASSANT {
            if us == WHITE { to - 8 } else { to + 8 }
        } else {
            to
        };
        let captured = self.board[capture_at];
        let undo = Undo {
            mv: m,
            captured,
            castling: self.castling,
            en_passant: self.en_passant,
            halfmove: self.halfmove,
            since_null: self.since_null,
            hash: self.hash,
            checkers: self.checkers,
        };
        let keys = &*KEYS;
        let mut hash =
            self.hash ^ self.en_passant_key() ^ keys.castling[usize::from(self.castling)];
        if captured != EMPTY {
            self.remove(them, usize::from(captured) % 6, capture_at);
            hash ^= keys.piece[usize::from(captured)][capture_at];
        }
        let placed = m.promotion().unwrap_or(kind);
        self.remove(us, kind, from);
        self.put(us, placed, to);
        hash ^= keys.piece[us * 6 + kind][from] ^ keys.piece[us * 6 + placed][to];
        if flag == FLAG_CASTLE {
            let (rook_from, rook_to) = if to > from {
                (from + 3, from + 1)
            } else {
                (from - 4, from - 1)
            };
            self.remove(us, ROOK, rook_from);
            self.put(us, ROOK, rook_to);
            hash ^= keys.piece[us * 6 + ROOK][rook_from] ^ keys.piece[us * 6 + ROOK][rook_to];
        }
        self.castling &= rights_mask(from) & rights_mask(to);
        self.en_passant = if flag == FLAG_DOUBLE {
            ((from + to) / 2) as u8
        } else {
            NO_SQUARE
        };
        self.halfmove = if kind == PAWN || captured != EMPTY {
            0
        } else {
            self.halfmove.saturating_add(1)
        };
        self.since_null = self.since_null.saturating_add(1);
        self.side = them;
        hash ^= keys.castling[usize::from(self.castling)] ^ keys.side;
        self.hash = hash ^ self.en_passant_key();
        if self.attacked(self.king_square(us), them) {
            self.restore(undo);
            return false;
        }
        self.checkers = self.attackers_of_king(them);
        self.stack.push(undo);
        self.keys.push(undo.hash);
        true
    }

    fn unmake(&mut self) {
        let undo = self.stack.pop().expect("a move to take back");
        self.keys.pop();
        self.restore(undo);
    }

    fn make_null(&mut self) {
        let undo = Undo {
            mv: Move(0),
            captured: EMPTY,
            castling: self.castling,
            en_passant: self.en_passant,
            halfmove: self.halfmove,
            since_null: self.since_null,
            hash: self.hash,
            checkers: self.checkers,
        };
        self.hash ^= self.en_passant_key() ^ KEYS.side;
        self.en_passant = NO_SQUARE;
        self.side = 1 - self.side;
        self.since_null = 0;
        self.stack.push(undo);
        self.keys.push(undo.hash);
    }

    fn unmake_null(&mut self) {
        let undo = self.stack.pop().expect("a null move to take back");
        self.keys.pop();
        self.side = 1 - self.side;
        self.en_passant = undo.en_passant;
        self.since_null = undo.since_null;
        self.hash = undo.hash;
        self.checkers = undo.checkers;
    }

    fn null_move_safe(&self) -> bool {
        let us = self.side;
        self.occupied[us] & !(self.pieces[us][PAWN] | self.pieces[us][KING]) != 0
    }

    fn rule_score(&self) -> Option<i32> {
        if self.halfmove >= 100 {
            // Checkmate on the hundredth halfmove takes precedence.
            if self.checkers == 0 || self.clone().has_legal_move() {
                return Some(0);
            }
            return None;
        }
        if self.halfmove >= 4 && self.repetition() {
            return Some(0);
        }
        let pawns_and_majors = self.pieces[0][PAWN]
            | self.pieces[1][PAWN]
            | self.pieces[0][ROOK]
            | self.pieces[1][ROOK]
            | self.pieces[0][QUEEN]
            | self.pieces[1][QUEEN];
        if pawns_and_majors == 0 && self.all().count_ones() <= 3 {
            return Some(0);
        }
        None
    }

    fn draw_claimable(&self) -> bool {
        // Fifty-move claim with an intended move: the hundredth halfmove must
        // be a legal move that is not a capture and not a pawn move.
        if self.halfmove != 99 {
            return false;
        }
        let mut moves = Vec::with_capacity(48);
        self.generate(Stage::Quiet, &mut moves);
        moves.into_iter().any(|m| self.quiet_piece_move_is_legal(m))
    }

    fn no_move_score(&self, ply: i32) -> i32 {
        if self.checkers != 0 {
            -MATE_SCORE + ply
        } else {
            0
        }
    }

    fn evaluate(&self) -> i32 {
        crate::chess_eval::evaluate(self)
    }

    #[inline]
    fn is_tactical(&self, m: Move) -> bool {
        self.board[m.to()] != EMPTY || m.flag() == FLAG_EN_PASSANT || m.flag() == FLAG_PROMOTE_QUEEN
    }

    fn tactical_order(&self, m: Move) -> i32 {
        let victim = if m.flag() == FLAG_EN_PASSANT {
            VALUE[PAWN]
        } else {
            self.kind_at(m.to()).map_or(0, |(_, kind)| VALUE[kind])
        };
        let attacker = self.kind_at(m.from()).map_or(0, |(_, kind)| kind as i32);
        let promotion = m.promotion().map_or(0, |kind| VALUE[kind] - VALUE[PAWN]);
        (victim + promotion) * 8 - attacker
    }

    fn tactical_gain(&self, m: Move) -> i32 {
        let victim = if m.flag() == FLAG_EN_PASSANT {
            VALUE[PAWN]
        } else {
            self.kind_at(m.to()).map_or(0, |(_, kind)| VALUE[kind])
        };
        victim + m.promotion().map_or(0, |kind| VALUE[kind] - VALUE[PAWN])
    }

    fn exchange_at_least(&self, m: Move, threshold: i32) -> bool {
        self.see_at_least(m, threshold)
    }

    #[inline]
    fn history_slot(&self, m: Move) -> usize {
        let kind = usize::from(self.board[m.from()]) % 6;
        (self.side * 6 + kind) * 64 + m.to()
    }

    #[inline]
    fn encode(m: Move) -> u32 {
        u32::from(m.0)
    }

    fn decode(&self, bits: u32) -> Option<Move> {
        let m = Move(u16::try_from(bits).ok()?);
        self.is_pseudo_legal(m).then_some(m)
    }

    fn is_pseudo_legal(&self, m: Move) -> bool {
        let (from, to, flag) = (m.from(), m.to(), m.flag());
        if flag > FLAG_PROMOTE_QUEEN {
            return false;
        }
        let us = self.side;
        let Some((color, kind)) = self.kind_at(from) else {
            return false;
        };
        if color != us
            || self.occupied[us] & bit(to) != 0
            || self.pieces[1 - us][KING] & bit(to) != 0
        {
            return false;
        }
        if kind != PAWN {
            return match flag {
                FLAG_NORMAL => self.attacks_as(us, kind, from) & bit(to) != 0,
                FLAG_CASTLE => {
                    let home = if us == WHITE { 4 } else { 60 };
                    kind == KING
                        && from == home
                        && (to == home + 2 && self.castle_path_clear(true)
                            || to + 2 == home && self.castle_path_clear(false))
                }
                _ => false,
            };
        }
        let (forward, start_rank, last_rank): (isize, usize, usize) =
            if us == WHITE { (8, 1, 7) } else { (-8, 6, 0) };
        let ahead = (from as isize + forward) as usize;
        let occupied = self.all();
        let promotes = to / 8 == last_rank;
        match flag {
            FLAG_EN_PASSANT => {
                self.en_passant != NO_SQUARE
                    && to == usize::from(self.en_passant)
                    && TABLES.pawn[us][from] & bit(to) != 0
            }
            FLAG_DOUBLE => {
                from / 8 == start_rank
                    && to as isize == from as isize + 2 * forward
                    && occupied & (bit(ahead) | bit(to)) == 0
            }
            FLAG_CASTLE => false,
            _ => {
                if promotes != (flag >= FLAG_PROMOTE_KNIGHT) {
                    return false;
                }
                if to == ahead {
                    occupied & bit(to) == 0
                } else {
                    TABLES.pawn[us][from] & bit(to) & self.occupied[1 - us] != 0
                }
            }
        }
    }
}
