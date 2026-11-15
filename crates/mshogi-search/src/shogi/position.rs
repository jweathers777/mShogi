//! Private Shogi board, hands, move generation and evaluation for the shared core.
//! Native rules remain authoritative for real moves and outcomes.
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::shogi::{Action, Game, GameCommand, Piece, PieceKind, Position, Side, Square};
use std::sync::LazyLock;

const FACES: usize = 14;
const HAND_FACES: usize = 7;
const SQUARES: usize = 81;
const ALL: u128 = (1 << SQUARES) - 1;
const KING: usize = 0;
const PAWN: usize = 7;
const DROP: usize = SQUARES;
const VALUE: [i32; FACES] = [
    0, 800, 700, 500, 400, 300, 300, 100, 1000, 900, 500, 500, 500, 500,
];
const HAND_FACE: [usize; HAND_FACES] = [1, 2, 3, 4, 5, 6, 7];
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
fn face_index(kind: PieceKind, promoted: bool) -> usize {
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

struct Tables {
    /// Leap targets of [side][face][square].
    leaps: [[[u128; SQUARES]; FACES]; 2],
    /// Squares from which a piece of [side][face] leaps to [square].
    leapers_to: [[[u128; SQUARES]; FACES]; 2],
    files: [u128; 9],
    piece: [[[u64; SQUARES]; FACES]; 2],
    hand: [[[u64; 19]; HAND_FACES]; 2],
    side: u64,
}
#[allow(clippy::needless_range_loop)]
static TABLES: LazyLock<Box<Tables>> = LazyLock::new(|| {
    let mut t = Box::new(Tables {
        leaps: [[[0; SQUARES]; FACES]; 2],
        leapers_to: [[[0; SQUARES]; FACES]; 2],
        files: [0; 9],
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
        t.files[square % 9] |= 1 << square;
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
    reversible: usize,
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
    /// Board and hand material of each side.
    material: [i32; 2],
    /// True if the side to move is in check.
    check: bool,
    /// Plies since the latest null move, or all known plies.
    reversible: usize,
    completed: u32,
    stack: Vec<Undo>,
    /// Hash of each position from the start of the known game to this one.
    keys: Vec<u64>,
    /// Index in `keys` of the search root.
    root: usize,
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
            material: [0; 2],
            check: false,
            reversible: 0,
            completed: position.completed_actions(),
            stack: Vec::with_capacity(160),
            keys: Vec::with_capacity(700),
            root: 0,
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
        p.root = p.keys.len() - 1;
        p.reversible = p.root;
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
    /// Material of a side on the board and in hand.
    pub fn material(&self, side: Side) -> i32 {
        self.material[side_index(side)]
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
        self.board[square] = code(side, face);
        self.pieces[side][face] |= 1 << square;
        self.occupied[side] |= 1 << square;
        self.hash ^= TABLES.piece[side][face][square];
        self.material[side] += VALUE[face];
    }
    fn take(&mut self, side: usize, face: usize, square: usize) {
        self.board[square] = 0;
        self.pieces[side][face] &= !(1 << square);
        self.occupied[side] &= !(1 << square);
        self.hash ^= TABLES.piece[side][face][square];
        self.material[side] -= VALUE[face];
    }
    fn add_hand(&mut self, side: usize, index: usize) {
        let count = usize::from(self.hands[side][index]);
        self.hash ^= TABLES.hand[side][index][count] ^ TABLES.hand[side][index][count + 1];
        self.hands[side][index] += 1;
        self.material[side] += VALUE[HAND_FACE[index]];
    }
    fn remove_hand(&mut self, side: usize, index: usize) {
        let count = usize::from(self.hands[side][index]);
        self.hash ^= TABLES.hand[side][index][count] ^ TABLES.hand[side][index][count - 1];
        self.hands[side][index] -= 1;
        self.material[side] -= VALUE[HAND_FACE[index]];
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
        let all = self.occupied[0] | self.occupied[1];
        let mut targets = 0u128;
        for &(right, forward, max) in RAYS[face] {
            let mut at = from;
            for _ in 0..max {
                let Some(next) = offset(at, side, right, forward) else {
                    break;
                };
                targets |= 1 << next;
                if all & (1 << next) != 0 {
                    break;
                }
                at = next;
            }
        }
        targets
    }
    /// True if a piece of `by` attacks the square.
    fn attacked(&self, target: usize, by: usize) -> bool {
        let t = &TABLES;
        for face in 0..FACES {
            if t.leapers_to[by][face][target] & self.pieces[by][face] != 0 {
                return true;
            }
        }
        for face in [1, 2, 6, 8, 9] {
            let mut sliders = self.pieces[by][face];
            while sliders != 0 {
                let from = sliders.trailing_zeros() as usize;
                sliders &= sliders - 1;
                if self.ray_targets(by, face, from) & (1 << target) != 0 {
                    return true;
                }
            }
        }
        false
    }
    #[allow(clippy::needless_range_loop)]
    fn generate_all(&self, stage: Stage, moves: &mut Vec<Move>) {
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
        let empty = ALL & !(self.occupied[0] | self.occupied[1]);
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
        self.keys.len() - 1 - self.root
    }
    /// Static evaluation terms other than material, for one side.
    fn placement(&self, side: usize) -> i32 {
        let mut score = 0;
        // Pawns gain value as they advance; they promote in the zone.
        let mut pawns = self.pieces[side][PAWN];
        while pawns != 0 {
            let square = pawns.trailing_zeros() as usize;
            pawns &= pawns - 1;
            let row = square / 9;
            score += 4 * if side == 0 { 8 - row } else { row } as i32;
        }
        // Own pieces next to the King protect it.
        let king = self.king(side);
        score += 8 * (TABLES.leaps[0][KING][king] & self.occupied[side]).count_ones() as i32;
        // A piece in hand can go to any empty square.
        let in_hand: i32 = self.hands[side].iter().map(|&n| i32::from(n)).sum();
        score + 10 * in_hand
    }
}

fn square(index: usize) -> Square {
    Square::new((9 - index % 9) as u8, (index / 9 + 1) as u8).expect("board index")
}

impl SearchPosition for ShogiPosition {
    type Move = Move;
    const HISTORY_SLOTS: usize = 2 * FACES * SQUARES;

    fn hash(&self) -> u64 {
        self.hash
    }
    fn in_check(&self) -> bool {
        self.check
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<Move>) {
        self.generate_all(stage, moves);
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
            reversible: self.reversible,
            completed: self.completed,
        };
        let mut pawn_drop = false;
        if from >= DROP {
            let index = from - DROP;
            self.remove_hand(side, index);
            self.put(side, HAND_FACE[index], to);
            pawn_drop = HAND_FACE[index] == PAWN;
            self.reversible = self.reversible.saturating_add(1);
        } else {
            undo.moved = self.board[from];
            let (_, face) = decode_piece(undo.moved);
            if undo.captured != 0 {
                let (owner, victim) = decode_piece(undo.captured);
                self.take(owner, victim, to);
                self.add_hand(side, hand_index(victim));
                self.reversible = self.reversible.saturating_add(1);
            } else {
                self.reversible = self.reversible.saturating_add(1);
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
        self.reversible = undo.reversible;
        self.completed = undo.completed;
    }
    fn make_null(&mut self) {
        self.stack.push(Undo {
            m: Move::default(),
            moved: 0,
            captured: 0,
            hash: self.hash,
            check: self.check,
            reversible: self.reversible,
            completed: self.completed,
        });
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.keys.push(self.hash);
        self.checks.push(false);
        self.check = false;
        // A pass is not a position of the game: no repetition across it.
        self.reversible = 0;
    }
    fn unmake_null(&mut self) {
        let undo = self.stack.pop().expect("a null move to take back");
        self.keys.pop();
        self.checks.pop();
        self.side ^= 1;
        self.hash = undo.hash;
        self.check = undo.check;
        self.reversible = undo.reversible;
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
        let limit = self.reversible.min(current);
        let mut earlier = 0;
        for back in (2..=limit).step_by(2) {
            let index = current - back;
            if self.keys[index] != self.hash {
                continue;
            }
            earlier += 1;
            if index >= self.root || earlier >= 3 {
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
        self.material[side] - self.material[side ^ 1] + self.placement(side)
            - self.placement(side ^ 1)
            + 10
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
        // One-step exchange estimate. A capture of a piece that is worth at
        // least the capturing piece is good. If not, the capturing piece must
        // not be open to recapture.
        if m.is_drop() {
            return threshold <= 0;
        }
        let (side, face) = decode_piece(self.board[m.from()]);
        let gain = self.tactical_gain(m);
        if gain < threshold {
            return false;
        }
        let mover = VALUE[self.moved_face(face, m)];
        let victim = match self.board[m.to()] {
            0 => 0,
            code => VALUE[decode_piece(code).1],
        };
        if victim >= mover {
            return true;
        }
        !self.attacked(m.to(), side ^ 1) || gain - mover - VALUE[base(face)] >= threshold
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
