//! Private Shogi board, hands, move generation and evaluation for the shared core.
//! Native rules remain authoritative for real moves and outcomes.
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use crate::repetition::{GamePrefix, cycle_start};
use mshogi_core::shogi::{Action, Game, GameCommand, Piece, PieceKind, Position, Side, Square};
use std::sync::LazyLock;

const FACES: usize = 14;
const HAND_FACES: usize = 7;
const SQUARES: usize = 81;
const ALL: u128 = (1 << SQUARES) - 1;
const KING: usize = 0;
const PAWN: usize = 7;
const DROP: usize = SQUARES;
/// Piece values in hundredths of a Pawn, in face order. Kings are scored by
/// rules-owned outcomes.
pub(super) const VALUE: [i32; FACES] = [
    0, 800, 700, 500, 400, 300, 300, 100, 1000, 900, 500, 500, 500, 500,
];
/// Face of a hand index.
pub(super) const HAND_FACE: [usize; HAND_FACES] = [1, 2, 3, 4, 5, 6, 7];
const KINDS: [PieceKind; 8] = [
    PieceKind::King,
    PieceKind::Rook,
    PieceKind::Bishop,
    PieceKind::Gold,
    PieceKind::Silver,
    PieceKind::Knight,
    PieceKind::Lance,
    PieceKind::Pawn,
];
pub(super) fn face_index(kind: PieceKind, promoted: bool) -> usize {
    let base = KINDS.iter().position(|&k| k == kind).unwrap();
    if promoted {
        match base {
            1 => 8,
            2 => 9,
            4..=7 => base + 6,
            _ => base,
        }
    } else {
        base
    }
}
const fn base(face: usize) -> usize {
    match face {
        8 => 1,
        9 => 2,
        10..=13 => face - 6,
        other => other,
    }
}
const fn hand_index(face: usize) -> usize {
    base(face) - 1
}
const fn side_index(side: Side) -> usize {
    match side {
        Side::Sente => 0,
        Side::Gote => 1,
    }
}

/// Steps and jumps (one target each, no blocking) of a face, as
/// (right, forward) of the mover.
const KING_STEPS: &[(i8, i8)] = &[
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const GOLD_STEPS: &[(i8, i8)] = &[(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (0, -1)];
const LEAPS: [&[(i8, i8)]; FACES] = [
    KING_STEPS,
    &[],
    &[],
    GOLD_STEPS,
    &[(-1, 1), (0, 1), (1, 1), (-1, -1), (1, -1)],
    &[(-1, 2), (1, 2)],
    &[],
    &[(0, 1)],
    &[(-1, 1), (1, 1), (-1, -1), (1, -1)],
    &[(0, 1), (-1, 0), (1, 0), (0, -1)],
    GOLD_STEPS,
    GOLD_STEPS,
    GOLD_STEPS,
    GOLD_STEPS,
];
const ROOK_RAYS: &[(i8, i8, u8)] = &[(0, 1, 8), (-1, 0, 8), (1, 0, 8), (0, -1, 8)];
const BISHOP_RAYS: &[(i8, i8, u8)] = &[(-1, 1, 8), (1, 1, 8), (-1, -1, 8), (1, -1, 8)];
const RAYS: [&[(i8, i8, u8)]; FACES] = [
    &[],
    ROOK_RAYS,
    BISHOP_RAYS,
    &[],
    &[],
    &[],
    &[(0, 1, 8)],
    &[],
    ROOK_RAYS,
    BISHOP_RAYS,
    &[],
    &[],
    &[],
    &[],
];
/// The square at (right, forward) of the mover, if it is on the board.
fn offset(square: usize, side: usize, right: i8, forward: i8) -> Option<usize> {
    let (right, forward) = if side == 0 {
        (right, forward)
    } else {
        (-right, -forward)
    };
    let column = (square % 9) as i8 + right;
    let row = (square / 9) as i8 - forward;
    ((0..9).contains(&column) && (0..9).contains(&row)).then(|| (row * 9 + column) as usize)
}
fn in_zone(side: usize, square: usize) -> bool {
    if side == 0 {
        square / 9 <= 2
    } else {
        square / 9 >= 6
    }
}

// North, west, east, south, then the four diagonals. Indices are monotone
// along each ray, so the nearest blocker is one bit scan.
const DIRECTIONS: [(i8, i8); 8] = [
    (0, 1),
    (-1, 0),
    (1, 0),
    (0, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
];
const DECREASING: [bool; 8] = [true, true, false, false, true, true, false, false];

fn ray(square: usize, direction: usize, occupied: u128) -> u128 {
    let mask = TABLES.rays[square][direction];
    let blockers = mask & occupied;
    if blockers == 0 {
        return mask;
    }
    let nearest = if DECREASING[direction] {
        127 - blockers.leading_zeros() as usize
    } else {
        blockers.trailing_zeros() as usize
    };
    mask & !TABLES.rays[nearest][direction]
}

/// All attackers of a square under the supplied exchange occupancy.
fn attackers(target: usize, by: usize, occupied: u128, pieces: &[[u128; FACES]; 2]) -> u128 {
    let mut result = 0;
    for (face, &sources) in pieces[by].iter().enumerate() {
        result |= TABLES.leapers_to[by][face][target] & sources;
    }
    let rooks = pieces[by][1] | pieces[by][8];
    let bishops = pieces[by][2] | pieces[by][9];
    for direction in 0..8 {
        let mut sliders = if direction < 4 { rooks } else { bishops };
        if direction == if by == 0 { 3 } else { 0 } {
            sliders |= pieces[by][6];
        }
        // Skip the bit scan if no compatible slider is on this ray.
        if sliders & TABLES.rays[target][direction] != 0 {
            result |= ray(target, direction, occupied) & sliders;
        }
    }
    result
}

struct Tables {
    /// Leap targets of [side][face][square].
    leaps: [[[u128; SQUARES]; FACES]; 2],
    /// Squares from which a piece of [side][face] leaps to [square].
    leapers_to: [[[u128; SQUARES]; FACES]; 2],
    rays: [[u128; 8]; SQUARES],
    files: [u128; 9],
    /// Squares within two king steps of a square, excluding it.
    near: [u128; SQUARES],
    /// Squares within three king steps of a square.
    near3: [u128; SQUARES],
    piece: [[[u64; SQUARES]; FACES]; 2],
    hand: [[[u64; 19]; HAND_FACES]; 2],
    side: u64,
}
#[allow(clippy::needless_range_loop)]
static TABLES: LazyLock<Box<Tables>> = LazyLock::new(|| {
    let mut t = Box::new(Tables {
        leaps: [[[0; SQUARES]; FACES]; 2],
        leapers_to: [[[0; SQUARES]; FACES]; 2],
        rays: [[0; 8]; SQUARES],
        files: [0; 9],
        near: [0; SQUARES],
        near3: [0; SQUARES],
        piece: [[[0; SQUARES]; FACES]; 2],
        hand: [[[0; 19]; HAND_FACES]; 2],
        side: 0,
    });
    for side in 0..2 {
        for face in 0..FACES {
            for from in 0..SQUARES {
                for &(right, forward) in LEAPS[face] {
                    if let Some(to) = offset(from, side, right, forward) {
                        t.leaps[side][face][from] |= 1 << to;
                        t.leapers_to[side][face][to] |= 1 << from;
                    }
                }
            }
        }
    }
    for square in 0..SQUARES {
        for (direction, &(right, forward)) in DIRECTIONS.iter().enumerate() {
            let mut at = square;
            while let Some(next) = offset(at, 0, right, forward) {
                t.rays[square][direction] |= 1 << next;
                at = next;
            }
        }
        t.files[square % 9] |= 1 << square;
        for other in 0..SQUARES {
            let rows = (square / 9).abs_diff(other / 9);
            let columns = (square % 9).abs_diff(other % 9);
            if other != square && rows <= 2 && columns <= 2 {
                t.near[square] |= 1 << other;
            }
            if rows <= 3 && columns <= 3 {
                t.near3[square] |= 1 << other;
            }
        }
    }
    // Fixed-seed SplitMix64: the keys are the same in each process.
    let mut state = 0x5091_5EED_0000_0180_u64;
    let mut next = || {
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };
    for side in 0..2 {
        for face in 0..FACES {
            for square in 0..SQUARES {
                t.piece[side][face][square] = next();
            }
        }
        for face in 0..HAND_FACES {
            // Count zero has key zero, so an empty hand adds nothing.
            for count in 1..19 {
                t.hand[side][face][count] = next();
            }
        }
    }
    t.side = next();
    t
});

/// Compact move: seven target bits, seven source bits, and a promotion bit.
/// A drop source is `81 + hand index`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Move(u16);
impl Move {
    fn new(from: usize, to: usize) -> Self {
        Self((from << 7 | to) as u16)
    }
    fn from(self) -> usize {
        usize::from((self.0 >> 7) & 127)
    }
    fn to(self) -> usize {
        usize::from(self.0 & 127)
    }
    fn promotes(self) -> bool {
        self.0 & (1 << 14) != 0
    }
    fn promoted(mut self) -> Self {
        self.0 |= 1 << 14;
        self
    }
    fn is_drop(self) -> bool {
        self.from() >= DROP
    }
}

#[derive(Clone, Copy)]
struct Undo {
    m: Move,
    /// Board code of the piece that moved, before the move (0 for a drop).
    moved: u8,
    /// Board code of the captured piece, or 0.
    captured: u8,
    hash: u64,
    check: bool,
    history_start: usize,
    completed: u32,
}

/// Search-internal Shogi position.
#[derive(Clone)]
pub struct ShogiPosition {
    /// 0 for empty, else `1 + side * 14 + face`.
    board: [u8; SQUARES],
    pieces: [[u128; FACES]; 2],
    occupied: [u128; 2],
    hands: [[u8; HAND_FACES]; 2],
    side: usize,
    hash: u64,
    /// Incremental evaluation terms of each side with the compiled weights:
    /// board values, rank tables, king rank and file, and hand terms.
    accumulated: [i32; 2],
    /// True if the side to move is in check.
    check: bool,
    /// Earliest key after the latest null move, or the start of the game.
    history_start: usize,
    completed: u32,
    stack: Vec<Undo>,
    /// Hash of each position from the start of the known game to this one.
    keys: Vec<u64>,
    /// Game keys before this search began.
    prefix: GamePrefix,
    /// Check status of the side to move at each key.
    checks: Vec<bool>,
}

fn code(side: usize, face: usize) -> u8 {
    (1 + side * FACES + face) as u8
}
fn decode_piece(code: u8) -> (usize, usize) {
    let index = usize::from(code - 1);
    (index / FACES, index % FACES)
}

impl ShogiPosition {
    /// Copy a native position. The copy has no earlier history.
    pub fn from_position(position: &Position) -> Self {
        let mut p = Self {
            board: [0; SQUARES],
            pieces: [[0; FACES]; 2],
            occupied: [0; 2],
            hands: [[0; HAND_FACES]; 2],
            side: side_index(position.side_to_move()),
            hash: 0,
            accumulated: [0; 2],
            check: false,
            history_start: 0,
            completed: position.completed_actions(),
            stack: Vec::with_capacity(160),
            keys: Vec::with_capacity(700),
            prefix: GamePrefix::default(),
            checks: Vec::new(),
        };
        for square in Square::all() {
            if let Some(Piece {
                side,
                kind,
                promoted,
            }) = position.piece(square)
            {
                p.put(side_index(side), face_index(kind, promoted), square.index());
            }
        }
        for side in [Side::Sente, Side::Gote] {
            for (index, &face) in PieceKind::HAND_KINDS.iter().enumerate() {
                for _ in 0..position.hand(side).count(face) {
                    p.add_hand(side_index(side), index);
                }
            }
        }
        if p.side == 1 {
            p.hash ^= TABLES.side;
        }
        p.check = p.attacked(p.king(p.side), p.side ^ 1);
        p.keys.push(p.hash);
        p.checks.push(p.check);
        p
    }

    /// Copy the current position of a native game with the hashes of its
    /// earlier positions and check observations.
    pub fn from_game(game: &Game) -> Self {
        let mut p = Self::from_position(game.position());
        let start = Self::from_position(game.root().position());
        p.keys = vec![start.hash];
        p.checks = vec![start.check];
        for event in game.events() {
            if let GameCommand::Play(_) = event.command {
                let native = Position::from_sfen(&event.after_sfen).expect("native event SFEN");
                let earlier = Self::from_position(&native);
                p.keys.push(earlier.hash);
                p.checks.push(earlier.check);
            }
        }
        p.prefix = GamePrefix::new(&p.keys[..p.keys.len() - 1]);
        p
    }
    /// Convert a compact move into a complete native action.
    pub fn action(&self, m: Move) -> Action {
        if m.is_drop() {
            Action::Drop {
                kind: KINDS[HAND_FACE[m.from() - DROP]],
                to: square(m.to()),
            }
        } else {
            Action::Move {
                from: square(m.from()),
                to: square(m.to()),
                promote: m.promotes(),
            }
        }
    }
    /// Encode a native action when it is pseudo-legal.
    pub fn find(&self, action: Action) -> Option<Move> {
        let m = match action {
            Action::Move { from, to, promote } => {
                let m = Move::new(from.index(), to.index());
                if promote { m.promoted() } else { m }
            }
            Action::Drop { kind, to } => {
                if kind == PieceKind::King {
                    return None;
                }
                Move::new(DROP + hand_index(face_index(kind, false)), to.index())
            }
        };
        self.pseudo_legal(m).then_some(m)
    }
    /// All position-local legal moves, independent of native outcomes.
    pub fn legal_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        self.generate_all(Stage::Tactical, &mut moves);
        self.generate_all(Stage::Quiet, &mut moves);
        moves.retain(|&m| {
            let legal = self.make(m);
            if legal {
                self.unmake();
            }
            legal
        });
        moves
    }
    /// Count the leaf nodes of the legal move tree.
    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut moves = Vec::new();
        self.generate_all(Stage::Tactical, &mut moves);
        self.generate_all(Stage::Quiet, &mut moves);
        let mut count = 0;
        for m in moves {
            if self.make(m) {
                count += self.perft(depth - 1);
                self.unmake();
            }
        }
        count
    }
    /// Side to move.
    pub fn side_to_move(&self) -> Side {
        if self.side == 0 {
            Side::Sente
        } else {
            Side::Gote
        }
    }
    /// Piece on a square.
    pub fn piece(&self, square: Square) -> Option<Piece> {
        let code = self.board[square.index()];
        (code != 0).then(|| {
            let (side, face) = decode_piece(code);
            Piece {
                side: if side == 0 { Side::Sente } else { Side::Gote },
                kind: KINDS[base(face)],
                promoted: face != base(face),
            }
        })
    }
    /// Number of pieces of a face in the hand of a side.
    pub fn hand_count(&self, side: Side, kind: PieceKind) -> u8 {
        let face = face_index(kind, false);
        if face == KING || face != base(face) {
            0
        } else {
            self.hands[side_index(side)][hand_index(face)]
        }
    }
    /// Hash computed from the full state; equal to `hash()` at all times.
    pub fn hash_from_scratch(&self) -> u64 {
        let mut hash = if self.side == 1 { TABLES.side } else { 0 };
        for (square, &code) in self.board.iter().enumerate() {
            if code != 0 {
                let (side, face) = decode_piece(code);
                hash ^= TABLES.piece[side][face][square];
            }
        }
        for side in 0..2 {
            for face in 0..HAND_FACES {
                hash ^= TABLES.hand[side][face][usize::from(self.hands[side][face])];
            }
        }
        hash
    }

    fn king(&self, side: usize) -> usize {
        self.pieces[side][KING].trailing_zeros() as usize
    }
    fn put(&mut self, side: usize, face: usize, square: usize) {
        self.accumulated[side] += square_weight(side, face, square);
        self.board[square] = code(side, face);
        self.pieces[side][face] |= 1 << square;
        self.occupied[side] |= 1 << square;
        self.hash ^= TABLES.piece[side][face][square];
    }
    fn take(&mut self, side: usize, face: usize, square: usize) {
        self.accumulated[side] -= square_weight(side, face, square);
        self.board[square] = 0;
        self.pieces[side][face] &= !(1 << square);
        self.occupied[side] &= !(1 << square);
        self.hash ^= TABLES.piece[side][face][square];
    }
    fn add_hand(&mut self, side: usize, index: usize) {
        let count = usize::from(self.hands[side][index]);
        self.accumulated[side] += hand_weight(index, count);
        self.hash ^= TABLES.hand[side][index][count] ^ TABLES.hand[side][index][count + 1];
        self.hands[side][index] += 1;
    }
    fn remove_hand(&mut self, side: usize, index: usize) {
        let count = usize::from(self.hands[side][index]);
        self.accumulated[side] -= hand_weight(index, count - 1);
        self.hash ^= TABLES.hand[side][index][count] ^ TABLES.hand[side][index][count - 1];
        self.hands[side][index] -= 1;
    }
    fn moved_face(&self, face: usize, m: Move) -> usize {
        if m.promotes() {
            match face {
                1 => 8,
                2 => 9,
                4..=7 => face + 6,
                _ => face,
            }
        } else {
            face
        }
    }
    fn dead_rank(side: usize, face: usize, to: usize) -> bool {
        let forward_rank = if side == 0 { to / 9 } else { 8 - to / 9 };
        match face {
            6 | 7 => forward_rank == 0,
            5 => forward_rank < 2,
            _ => false,
        }
    }
    /// Ray targets of a piece: empty squares and the first occupied square.
    fn ray_targets(&self, side: usize, face: usize, from: usize) -> u128 {
        let occupied = self.occupied[0] | self.occupied[1];
        match face {
            1 | 8 => (0..4).fold(0, |mask, d| mask | ray(from, d, occupied)),
            2 | 9 => (4..8).fold(0, |mask, d| mask | ray(from, d, occupied)),
            6 => ray(from, if side == 0 { 0 } else { 3 }, occupied),
            _ => 0,
        }
    }
    /// True if a piece of `by` attacks the square.
    fn attacked(&self, target: usize, by: usize) -> bool {
        attackers(
            target,
            by,
            self.occupied[0] | self.occupied[1],
            &self.pieces,
        ) != 0
    }
    fn generate_all(&self, stage: Stage, moves: &mut Vec<Move>) {
        self.generate_with_drops(stage, ALL, moves);
    }
    /// Generate one stage; drops go only to squares in `drop_mask`.
    #[allow(clippy::needless_range_loop)]
    fn generate_with_drops(&self, stage: Stage, drop_mask: u128, moves: &mut Vec<Move>) {
        let side = self.side;
        let own = self.occupied[side];
        let enemy = self.occupied[side ^ 1] & !self.pieces[side ^ 1][KING];
        let tactical = stage == Stage::Tactical;
        for face in 0..FACES {
            let mut from_set = self.pieces[side][face];
            while from_set != 0 {
                let from = from_set.trailing_zeros() as usize;
                from_set &= from_set - 1;
                let mut targets = TABLES.leaps[side][face][from];
                if !RAYS[face].is_empty() {
                    targets |= self.ray_targets(side, face, from);
                }
                targets &= !own & !self.pieces[side ^ 1][KING];
                while targets != 0 {
                    let to = targets.trailing_zeros() as usize;
                    targets &= targets - 1;
                    let capture = enemy & (1 << to) != 0;
                    let m = Move::new(from, to);
                    if !Self::dead_rank(side, face, to) && capture == tactical {
                        moves.push(m);
                    }
                    if tactical
                        && face == base(face)
                        && face != KING
                        && face != 3
                        && (in_zone(side, from) || in_zone(side, to))
                    {
                        moves.push(m.promoted());
                    }
                }
            }
        }
        if tactical {
            return;
        }
        let empty = drop_mask & !(self.occupied[0] | self.occupied[1]);
        for index in 0..HAND_FACES {
            if self.hands[side][index] == 0 {
                continue;
            }
            let mut targets = empty;
            targets &= self.drop_squares(side, HAND_FACE[index]);
            while targets != 0 {
                let to = targets.trailing_zeros() as usize;
                targets &= targets - 1;
                moves.push(Move::new(DROP + index, to));
            }
        }
    }
    fn drop_squares(&self, side: usize, face: usize) -> u128 {
        let mut allowed = ALL;
        if face == PAWN {
            for file in TABLES.files {
                if self.pieces[side][PAWN] & file != 0 {
                    allowed &= !file;
                }
            }
        }
        if face == PAWN || face == 6 || face == 5 {
            allowed &= !(0x1ffu128 << if side == 0 { 0 } else { 72 });
            if face == 5 {
                allowed &= !(0x1ffu128 << if side == 0 { 9 } else { 63 });
            }
        }
        allowed
    }
    fn pseudo_legal(&self, m: Move) -> bool {
        let (from, to) = (m.from(), m.to());
        if to >= SQUARES || m.0 & (1 << 15) != 0 {
            return false;
        }
        let side = self.side;
        if from >= DROP {
            let index = from - DROP;
            return index < HAND_FACES
                && self.hands[side][index] > 0
                && self.board[to] == 0
                && !m.promotes()
                && self.drop_squares(side, HAND_FACE[index]) & (1 << to) != 0;
        }
        let code = self.board[from];
        if code == 0 {
            return false;
        }
        let (owner, face) = decode_piece(code);
        if owner != side
            || self.occupied[side] & (1 << to) != 0
            || self.pieces[side ^ 1][KING] & (1 << to) != 0
        {
            return false;
        }
        if m.promotes() {
            if face != base(face)
                || face == KING
                || face == 3
                || !(in_zone(side, from) || in_zone(side, to))
            {
                return false;
            }
        } else if Self::dead_rank(side, face, to) {
            return false;
        }
        TABLES.leaps[side][face][from] & (1 << to) != 0
            || (!RAYS[face].is_empty() && self.ray_targets(side, face, from) & (1 << to) != 0)
    }
    fn has_legal_move(&mut self) -> bool {
        let mut moves = Vec::new();
        self.generate_all(Stage::Tactical, &mut moves);
        self.generate_all(Stage::Quiet, &mut moves);
        moves.into_iter().any(|m| {
            let legal = self.make(m);
            if legal {
                self.unmake();
            }
            legal
        })
    }
    fn ply(&self) -> usize {
        self.keys.len() - 1 - self.prefix.len
    }
    /// Squares from which a piece of `side` with `face` attacks `king`.
    fn check_squares(&self, side: usize, face: usize, king: usize, occupied: u128) -> u128 {
        let mut squares = TABLES.leapers_to[side][face][king];
        match face {
            1 | 8 => squares |= (0..4).fold(0, |mask, d| mask | ray(king, d, occupied)),
            2 | 9 => squares |= (4..8).fold(0, |mask, d| mask | ray(king, d, occupied)),
            6 => squares |= ray(king, if side == 0 { 3 } else { 0 }, occupied),
            _ => {}
        }
        squares
    }
    /// Own pieces of `side` that block one of its own sliders from `king`:
    /// moving one off the line can give a discovered check.
    fn discoverers(&self, side: usize, king: usize, occupied: u128) -> u128 {
        let rooks = self.pieces[side][1] | self.pieces[side][8];
        let bishops = self.pieces[side][2] | self.pieces[side][9];
        let mut result = 0;
        for direction in 0..8 {
            let mut sliders = if direction < 4 { rooks } else { bishops };
            if direction == if side == 0 { 3 } else { 0 } {
                sliders |= self.pieces[side][6];
            }
            if sliders & TABLES.rays[king][direction] == 0 {
                continue;
            }
            let blocker = ray(king, direction, occupied) & occupied & self.occupied[side];
            if blocker == 0 {
                continue;
            }
            let at = blocker.trailing_zeros() as usize;
            if ray(at, direction, occupied) & occupied & sliders != 0 {
                result |= blocker;
            }
        }
        result
    }
    /// Empty squares where a drop can block the single sliding check of the
    /// side to move; zero for a double or adjacent check.
    fn interpositions(&self) -> u128 {
        let side = self.side;
        let king = self.king(side);
        let occupied = self.occupied[0] | self.occupied[1];
        let checkers = attackers(king, side ^ 1, occupied, &self.pieces);
        if checkers.count_ones() != 1 {
            return 0;
        }
        for direction in 0..8 {
            let line = ray(king, direction, occupied);
            if line & checkers != 0 {
                return line & !checkers;
            }
        }
        0
    }
    /// True if the move (not a discovered check) attacks the enemy king.
    fn gives_direct_check(&self, m: Move) -> bool {
        let side = self.side;
        let king = self.king(side ^ 1);
        let occupied = self.occupied[0] | self.occupied[1];
        let face = if m.is_drop() {
            HAND_FACE[m.from() - DROP]
        } else {
            self.moved_face(decode_piece(self.board[m.from()]).1, m)
        };
        let after = if m.is_drop() {
            occupied | 1 << m.to()
        } else {
            (occupied & !(1 << m.from())) | 1 << m.to()
        };
        self.check_squares(side, face, king, after) & (1 << m.to()) != 0
    }
    /// Static exchange evaluation of a move on its target square, with
    /// board and hand values: a captured piece leaves the board and its base
    /// face enters the capturer's hand. Each recapture uses the least
    /// valuable attacker and promotes when it may. Pins are ignored.
    pub fn see(&self, m: Move) -> i32 {
        const ORDER: [usize; FACES] = [7, 6, 5, 4, 3, 10, 11, 12, 13, 2, 1, 9, 8, KING];
        let side = self.side;
        let to = m.to();
        let mut occupied = self.occupied[0] | self.occupied[1];
        let mut gains = [0i32; 40];
        let mut on_target;
        if m.is_drop() {
            on_target = HAND_FACE[m.from() - DROP];
            occupied |= 1 << to;
        } else {
            let face = decode_piece(self.board[m.from()]).1;
            on_target = self.moved_face(face, m);
            gains[0] = VALUE[on_target] - VALUE[face];
            if self.board[to] != 0 {
                let victim = decode_piece(self.board[to]).1;
                gains[0] += VALUE[victim] + VALUE[base(victim)];
            }
            occupied &= !(1 << m.from());
        }
        let mut mover = side ^ 1;
        let mut depth = 0;
        loop {
            let found = attackers(to, mover, occupied, &self.pieces) & occupied;
            if found == 0 {
                break;
            }
            let Some((face, from)) = ORDER.iter().find_map(|&face| {
                let set = found & self.pieces[mover][face];
                (set != 0).then(|| (face, set.trailing_zeros() as usize))
            }) else {
                break;
            };
            if face == KING
                && attackers(to, mover ^ 1, occupied & !(1 << from), &self.pieces)
                    & occupied
                    & !(1 << from)
                    != 0
            {
                break;
            }
            let promotable = face == base(face) && face != KING && face != 3;
            let promoted = if promotable && (in_zone(mover, from) || in_zone(mover, to)) {
                match face {
                    1 => 8,
                    2 => 9,
                    _ => face + 6,
                }
            } else {
                face
            };
            depth += 1;
            if depth >= gains.len() {
                break;
            }
            gains[depth] = VALUE[on_target] + VALUE[base(on_target)] + VALUE[promoted]
                - VALUE[face]
                - gains[depth - 1];
            on_target = promoted;
            occupied &= !(1 << from);
            mover ^= 1;
        }
        while depth > 0 {
            gains[depth - 1] = -(-gains[depth - 1]).max(gains[depth]);
            depth -= 1;
        }
        gains[0]
    }
    /// Evaluation features (`super::eval`): each is the side to move's
    /// value minus the opponent's, plus the tempo. `sink(index, value)`
    /// receives every nonzero contribution. The search computes the same
    /// sum with incremental static terms (`evaluate`).
    pub fn features(&self, sink: &mut impl FnMut(usize, i32)) {
        use super::eval::*;
        for side in 0..2 {
            let sign = if side == self.side { 1 } else { -1 };
            for face in 0..FACES {
                let mut set = self.pieces[side][face];
                while set != 0 {
                    let square = set.trailing_zeros() as usize;
                    set &= set - 1;
                    for (index, value) in square_features(side, face, square) {
                        sink(index, sign * value);
                    }
                }
            }
            for index in 0..HAND_FACES {
                let count = i32::from(self.hands[side][index]);
                if count > 0 {
                    sink(HAND + index, sign * count);
                    sink(HAND_FIRST + index, sign);
                }
            }
            self.dynamic_features(side, &mut |index, value| sink(index, sign * value));
        }
        sink(TEMPO, 1);
    }
    /// Terms that depend on more than one piece: slider mobility and king
    /// safety, for one side.
    #[allow(clippy::needless_range_loop)]
    fn dynamic_features(&self, side: usize, sink: &mut impl FnMut(usize, i32)) {
        use super::eval::*;
        let occupied = self.occupied[0] | self.occupied[1];
        let enemy = side ^ 1;
        for (slot, face) in [6, 2, 1, 9, 8].into_iter().enumerate() {
            let mut set = self.pieces[side][face];
            while set != 0 {
                let square = set.trailing_zeros() as usize;
                set &= set - 1;
                let reach = self.ray_targets(side, face, square) & !self.occupied[side];
                let count = reach.count_ones() as i32;
                if count != 0 {
                    sink(MOBILITY + slot, count);
                }
            }
        }
        let king = self.king(side);
        let around = TABLES.leaps[0][KING][king];
        let own = (around & self.occupied[side]).count_ones() as i32;
        if own != 0 {
            sink(KING_ADJACENT_OWN, own);
        }
        // Enemy attacks on the squares next to the king: leapers reach at
        // most two ranks, so only pieces within three squares matter.
        let mut map = 0u128;
        let reach = TABLES.near3[king];
        for face in 0..FACES {
            let mut set = self.pieces[enemy][face]
                & if matches!(face, 1 | 2 | 6 | 8 | 9) {
                    ALL
                } else {
                    reach
                };
            while set != 0 {
                let from = set.trailing_zeros() as usize;
                set &= set - 1;
                map |= TABLES.leaps[enemy][face][from];
                if !RAYS[face].is_empty() {
                    map |= self.ray_targets(enemy, face, from);
                }
            }
        }
        let attacked = (around & map).count_ones() as i32;
        if attacked != 0 {
            sink(KING_ADJACENT_ATTACKED, attacked);
        }
        let near = TABLES.near[king];
        let enemies = (near & self.occupied[enemy] & !self.pieces[enemy][KING]).count_ones() as i32;
        if enemies != 0 {
            sink(KING_NEAR_ENEMY, enemies);
        }
        let defenders = self.pieces[side][3]
            | self.pieces[side][4]
            | self.pieces[side][10]
            | self.pieces[side][11]
            | self.pieces[side][12]
            | self.pieces[side][13];
        let defenders = (near & defenders).count_ones() as i32;
        if defenders != 0 {
            sink(KING_NEAR_DEFENDERS, defenders);
        }
        let held: i32 = (0..HAND_FACES - 1)
            .map(|index| i32::from(self.hands[enemy][index]))
            .sum();
        let empty = (around & !occupied).count_ones() as i32;
        if held * empty != 0 {
            sink(KING_DROP_THREAT, held * empty);
        }
    }
    /// Evaluation with explicit weights through the incremental path shape:
    /// static terms for the compiled weights must match `accumulated`.
    fn dynamic_score(&self, side: usize) -> i32 {
        let weights = &super::eval::DEFAULT;
        let mut total = 0;
        self.dynamic_features(side, &mut |index, value| total += weights[index] * value);
        total
    }
}

/// Static features of one piece on a square: board value and rank table,
/// or the king's rank and file.
fn square_features(side: usize, face: usize, square: usize) -> [(usize, i32); 2] {
    use super::eval::*;
    // Rank-table group of a face: Pawn, Lance, Knight, Silver, Gold-like,
    // Bishop, Rook, Horse, Dragon.
    const GROUP: [usize; FACES] = [0, 6, 5, 4, 3, 2, 1, 0, 8, 7, 4, 4, 4, 4];
    let advanced = if side == 0 {
        8 - square / 9
    } else {
        square / 9
    };
    if face == KING {
        [
            (KING_RANK + advanced, 1),
            (KING_FILE + (square % 9).abs_diff(4), 1),
        ]
    } else {
        [(BOARD + face - 1, 1), (PST + GROUP[face] * 9 + advanced, 1)]
    }
}
fn square_weight(side: usize, face: usize, square: usize) -> i32 {
    square_features(side, face, square)
        .iter()
        .map(|&(index, value)| super::eval::DEFAULT[index] * value)
        .sum()
}
/// Change of the hand terms when a hand count rises from `count`.
fn hand_weight(index: usize, count: usize) -> i32 {
    use super::eval::*;
    DEFAULT[HAND + index]
        + if count == 0 {
            DEFAULT[HAND_FIRST + index]
        } else {
            0
        }
}

fn square(index: usize) -> Square {
    Square::new((9 - index % 9) as u8, (index / 9 + 1) as u8).expect("board index")
}

impl SearchPosition for ShogiPosition {
    type Move = Move;
    const HISTORY_SLOTS: usize = 2 * FACES * SQUARES;
    /// A capture moves a piece from the board to the hand: the smallest
    /// capture changes material by two Pawn values.
    const MARGIN_UNIT: i32 = 200;
    const QUIESCENCE_CHECKS: usize = 8;
    const MATE_SEARCH: bool = true;
    fn hash(&self) -> u64 {
        self.hash
    }
    fn in_check(&self) -> bool {
        self.check
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<Move>) {
        self.generate_all(stage, moves);
    }
    fn generate_search(&self, stage: Stage, moves: &mut Vec<Move>) {
        // In check, only a drop between the king and a single slider can be
        // legal; other drops are left out. Board moves are unchanged.
        let mask = if stage == Stage::Quiet && self.check {
            self.interpositions()
        } else {
            ALL
        };
        self.generate_with_drops(stage, mask, moves);
    }
    #[allow(clippy::needless_range_loop)]
    fn generate_checks(&self, moves: &mut Vec<Move>) {
        let side = self.side;
        let king = self.king(side ^ 1);
        let occupied = self.occupied[0] | self.occupied[1];
        let open = !self.occupied[side] & !self.pieces[side ^ 1][KING];
        let mut squares = [0u128; FACES];
        for face in 1..FACES {
            squares[face] = self.check_squares(side, face, king, occupied);
        }
        let discoverers = self.discoverers(side, king, occupied);
        for face in 0..FACES {
            let mut from_set = self.pieces[side][face];
            while from_set != 0 {
                let from = from_set.trailing_zeros() as usize;
                from_set &= from_set - 1;
                let discovers = discoverers & (1 << from) != 0;
                if face == KING && !discovers {
                    continue;
                }
                let mut targets = TABLES.leaps[side][face][from];
                if !RAYS[face].is_empty() {
                    targets |= self.ray_targets(side, face, from);
                }
                targets &= open;
                let promotable = face == base(face) && face != KING && face != 3;
                while targets != 0 {
                    let to = targets.trailing_zeros() as usize;
                    targets &= targets - 1;
                    let m = Move::new(from, to);
                    if !Self::dead_rank(side, face, to)
                        && (discovers || squares[face] & (1 << to) != 0)
                    {
                        moves.push(m);
                    }
                    if promotable && (in_zone(side, from) || in_zone(side, to)) {
                        let promoted = self.moved_face(face, m.promoted());
                        if discovers || squares[promoted] & (1 << to) != 0 {
                            moves.push(m.promoted());
                        }
                    }
                }
            }
        }
        let empty = ALL & !occupied;
        for index in 0..HAND_FACES {
            if self.hands[side][index] == 0 {
                continue;
            }
            let face = HAND_FACE[index];
            let mut targets = empty & squares[face] & self.drop_squares(side, face);
            while targets != 0 {
                let to = targets.trailing_zeros() as usize;
                targets &= targets - 1;
                moves.push(Move::new(DROP + index, to));
            }
        }
    }
    fn make(&mut self, m: Move) -> bool {
        if !self.pseudo_legal(m) {
            return false;
        }
        let side = self.side;
        let (from, to) = (m.from(), m.to());
        let mut undo = Undo {
            m,
            moved: 0,
            captured: self.board[to],
            hash: self.hash,
            check: self.check,
            history_start: self.history_start,
            completed: self.completed,
        };
        let mut pawn_drop = false;
        if from >= DROP {
            let index = from - DROP;
            self.remove_hand(side, index);
            self.put(side, HAND_FACE[index], to);
            pawn_drop = HAND_FACE[index] == PAWN;
        } else {
            undo.moved = self.board[from];
            let (_, face) = decode_piece(undo.moved);
            if undo.captured != 0 {
                let (owner, victim) = decode_piece(undo.captured);
                self.take(owner, victim, to);
                self.add_hand(side, hand_index(victim));
            }
            self.take(side, face, from);
            let result = self.moved_face(face, m);
            self.put(side, result, to);
        }
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.completed = self.completed.saturating_add(1);
        self.stack.push(undo);
        self.keys.push(self.hash);
        self.checks.push(false);
        if self.attacked(self.king(side), side ^ 1) {
            self.unmake();
            return false;
        }
        self.check = self.attacked(self.king(side ^ 1), side);
        *self.checks.last_mut().unwrap() = self.check;
        // A checking Pawn drop must leave a legal reply.
        if pawn_drop
            && TABLES.leaps[side][PAWN][to] & (1 << self.king(side ^ 1)) != 0
            && !self.has_legal_move()
        {
            self.unmake();
            return false;
        }
        true
    }
    fn unmake(&mut self) {
        let undo = self.stack.pop().expect("a move to take back");
        self.keys.pop();
        self.checks.pop();
        self.side ^= 1;
        let side = self.side;
        let (from, to) = (undo.m.from(), undo.m.to());
        if from >= DROP {
            let index = from - DROP;
            self.take(side, HAND_FACE[index], to);
            self.add_hand(side, index);
        } else {
            let (_, result) = decode_piece(self.board[to]);
            self.take(side, result, to);
            let (_, face) = decode_piece(undo.moved);
            self.put(side, face, from);
            if undo.captured != 0 {
                let (owner, victim) = decode_piece(undo.captured);
                self.remove_hand(side, hand_index(victim));
                self.put(owner, victim, to);
            }
        }
        self.hash = undo.hash;
        self.check = undo.check;
        self.history_start = undo.history_start;
        self.completed = undo.completed;
    }
    fn make_null(&mut self) {
        self.stack.push(Undo {
            m: Move::default(),
            moved: 0,
            captured: 0,
            hash: self.hash,
            check: self.check,
            history_start: self.history_start,
            completed: self.completed,
        });
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.keys.push(self.hash);
        self.checks.push(false);
        self.check = false;
        // A pass is not a position of the game: no repetition across it.
        self.history_start = self.keys.len() - 1;
    }
    fn unmake_null(&mut self) {
        let undo = self.stack.pop().expect("a null move to take back");
        self.keys.pop();
        self.checks.pop();
        self.side ^= 1;
        self.hash = undo.hash;
        self.check = undo.check;
        self.history_start = undo.history_start;
        self.completed = undo.completed;
    }
    fn null_move_safe(&self) -> bool {
        // With a piece in hand, or a piece other than King and Pawns,
        // the side to move nearly always has a useful move.
        let side = self.side;
        self.hands[side].iter().any(|&n| n > 0)
            || self.occupied[side] & !(self.pieces[side][KING] | self.pieces[side][PAWN]) != 0
    }
    fn rule_score(&self) -> Option<i32> {
        let current = self.keys.len() - 1;
        let occurrences = self.prefix.occurrences(&self.keys, self.history_start);
        if let Some(index) = cycle_start(occurrences, self.prefix.len, 3) {
            // On a repeated search cycle, a side checking on every one
            // of its moves loses. Board/hand cycles can cross captures.
            for side in 0..2 {
                let all_check = (index + 1..=current)
                    .filter(|&i| {
                        let mover = self.side ^ ((current - i) & 1) ^ 1;
                        mover == side
                    })
                    .all(|i| self.checks[i]);
                if all_check {
                    return Some(if side == self.side {
                        -MATE_SCORE + self.ply() as i32
                    } else {
                        MATE_SCORE - self.ply() as i32
                    });
                }
            }
            return Some(0);
        }
        if self.completed >= 320 {
            // The native length rule follows mate/stalemate in priority.
            let mut copy = self.clone();
            return Some(if copy.has_legal_move() {
                0
            } else {
                -MATE_SCORE + self.ply() as i32
            });
        }
        None
    }
    fn draw_claimable(&self) -> bool {
        false
    }
    fn no_move_score(&self, ply: i32) -> i32 {
        // Checkmate and a position without a legal action both lose.
        -(MATE_SCORE - ply)
    }
    fn evaluate(&self) -> i32 {
        let side = self.side;
        self.accumulated[side] - self.accumulated[side ^ 1] + self.dynamic_score(side)
            - self.dynamic_score(side ^ 1)
            + super::eval::DEFAULT[super::eval::TEMPO]
    }
    fn is_tactical(&self, m: Move) -> bool {
        if m.is_drop() {
            return false;
        }
        if self.board[m.to()] != 0 {
            return true;
        }
        m.promotes()
    }
    fn tactical_order(&self, m: Move) -> i32 {
        let (_, face) = decode_piece(self.board[m.from()]);
        self.tactical_gain(m) * 16 - VALUE[face] / 16
    }
    fn tactical_gain(&self, m: Move) -> i32 {
        if m.is_drop() {
            return 0;
        }
        let (_, face) = decode_piece(self.board[m.from()]);
        let mut gain = VALUE[self.moved_face(face, m)] - VALUE[face];
        if self.board[m.to()] != 0 {
            let (_, victim) = decode_piece(self.board[m.to()]);
            // The victim leaves the board and its base face enters the hand.
            gain += VALUE[victim] + VALUE[base(victim)];
        }
        gain.max(0)
    }
    fn exchange_at_least(&self, m: Move, threshold: i32) -> bool {
        // The first move's gain bounds the exchange from above.
        if !m.is_drop() && self.tactical_gain(m) < threshold {
            return false;
        }
        self.see(m) >= threshold
    }
    fn quiet_order(&self, m: Move) -> i32 {
        let side = self.side;
        let mut bonus = 0;
        if !m.is_drop() {
            let face = decode_piece(self.board[m.from()]).1;
            // Declining to promote a Pawn, Bishop or Rook is almost never
            // better; the promotion is ordered among the tactical moves.
            if matches!(face, 1 | 2 | PAWN)
                && !m.promotes()
                && (in_zone(side, m.from()) || in_zone(side, m.to()))
            {
                bonus -= 20_000;
            }
        }
        if self.gives_direct_check(m) {
            bonus += 6_000;
        } else if m.is_drop() {
            let king = self.king(side ^ 1);
            let (row, column) = (m.to() / 9, m.to() % 9);
            let distance = row.abs_diff(king / 9).max(column.abs_diff(king % 9));
            if distance <= 2 {
                bonus += 2_000;
            }
        }
        bonus
    }
    fn history_slot(&self, m: Move) -> usize {
        let face = if m.is_drop() {
            HAND_FACE[m.from() - DROP]
        } else {
            decode_piece(self.board[m.from()]).1
        };
        (self.side * FACES + face) * SQUARES + m.to()
    }
    fn encode(m: Move) -> u32 {
        u32::from(m.0) + 1
    }
    fn decode(&self, bits: u32) -> Option<Move> {
        let m = Move(u16::try_from(bits.checked_sub(1)?).ok()?);
        self.is_pseudo_legal(m).then_some(m)
    }
    fn is_pseudo_legal(&self, m: Move) -> bool {
        self.pseudo_legal(m)
    }
}
