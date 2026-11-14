//! Tori Shogi search position for the shared search core.
//!
//! A private fast copy of a native Tori position: 49-bit bitboards, hands,
//! and incremental hash and material. It applies the position-local rules of
//! `mshogi.tori.modern.v1` (compulsory promotion, the Swallow file, last-rank
//! and drop-mate rules, no move into check). The native `mshogi_core::tori`
//! rules stay the authority; `position_tests` compares this position with them.
//!
//! Squares use the native index: `row * 7 + column`, row 0 is rank `a`, and
//! Sente moves toward row 0.
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::tori::{Action, Face, Game, Piece, Position, Side, Square};
use std::sync::LazyLock;

const FACES: usize = 9;
const HAND_FACES: usize = 6;
const SQUARES: usize = 49;
const ALL: u64 = (1 << SQUARES) - 1;
const PHOENIX: usize = 0;
const FALCON: usize = 1;
const EAGLE: usize = 2;
const LEFT_QUAIL: usize = 5;
const RIGHT_QUAIL: usize = 6;
const SWALLOW: usize = 7;
const GOOSE: usize = 8;
/// First `from` value of a drop: `DROP + hand index`.
const DROP: usize = SQUARES;

/// Material in hundredths of a Swallow, in `Face::ALL` order.
const VALUE: [i32; FACES] = [0, 700, 1000, 400, 300, 500, 500, 100, 300];
/// Face of a hand index, in `Face::HANDS` order.
const HAND_FACE: [usize; HAND_FACES] = [FALCON, 3, 4, LEFT_QUAIL, RIGHT_QUAIL, SWALLOW];

/// Material value of a face; hand pieces have the value of their base face.
pub const fn piece_value(face: Face) -> i32 {
    VALUE[face_index(face)]
}
const fn face_index(face: Face) -> usize {
    match face {
        Face::Phoenix => 0,
        Face::Falcon => 1,
        Face::Eagle => 2,
        Face::Crane => 3,
        Face::Pheasant => 4,
        Face::LeftQuail => 5,
        Face::RightQuail => 6,
        Face::Swallow => 7,
        Face::Goose => 8,
    }
}
const fn base(face: usize) -> usize {
    match face {
        EAGLE => FALCON,
        GOOSE => SWALLOW,
        other => other,
    }
}
const fn hand_index(face: usize) -> usize {
    match base(face) {
        FALCON => 0,
        3 => 1,
        4 => 2,
        LEFT_QUAIL => 3,
        RIGHT_QUAIL => 4,
        _ => 5,
    }
}
const fn side_index(side: Side) -> usize {
    match side {
        Side::Sente => 0,
        Side::Gote => 1,
    }
}

/// Steps and jumps (one target each, no blocking) of a face, as
/// (right, forward) of the mover.
const LEAPS: [&[(i8, i8)]; FACES] = [
    &[
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ],
    &[(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (1, -1)],
    &[(0, 1), (-1, 0), (1, 0)],
    &[(-1, 1), (0, 1), (1, 1), (-1, -1), (0, -1), (1, -1)],
    &[(-1, -1), (1, -1), (0, 2)],
    &[(-1, -1)],
    &[(1, -1)],
    &[(0, 1)],
    &[(-2, 2), (2, 2), (0, -2)],
];
/// Rays of a face as (right, forward, maximum distance).
const RAYS: [&[(i8, i8, u8)]; FACES] = [
    &[],
    &[],
    &[(-1, 1, 6), (1, 1, 6), (0, -1, 6), (-1, -1, 2), (1, -1, 2)],
    &[],
    &[],
    &[(0, 1, 6), (1, -1, 6)],
    &[(0, 1, 6), (-1, -1, 6)],
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
    let column = (square % 7) as i8 + right;
    let row = (square / 7) as i8 - forward;
    ((0..7).contains(&column) && (0..7).contains(&row)).then(|| (row * 7 + column) as usize)
}
fn in_zone(side: usize, square: usize) -> bool {
    if side == 0 {
        square / 7 <= 1
    } else {
        square / 7 >= 5
    }
}

struct Tables {
    /// Leap targets of [side][face][square].
    leaps: [[[u64; SQUARES]; FACES]; 2],
    /// Squares from which a piece of [side][face] leaps to [square].
    leapers_to: [[[u64; SQUARES]; FACES]; 2],
    files: [u64; 7],
    piece: [[[u64; SQUARES]; FACES]; 2],
    hand: [[[u64; 17]; HAND_FACES]; 2],
    side: u64,
}
#[allow(clippy::needless_range_loop)]
static TABLES: LazyLock<Box<Tables>> = LazyLock::new(|| {
    let mut t = Box::new(Tables {
        leaps: [[[0; SQUARES]; FACES]; 2],
        leapers_to: [[[0; SQUARES]; FACES]; 2],
        files: [0; 7],
        piece: [[[0; SQUARES]; FACES]; 2],
        hand: [[[0; 17]; HAND_FACES]; 2],
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
        t.files[square % 7] |= 1 << square;
    }
    // Fixed-seed SplitMix64: the keys are the same in each process.
    let mut state = 0x7012_5EED_0000_0165_u64;
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
            for count in 1..17 {
                t.hand[side][face][count] = next();
            }
        }
    }
    t.side = next();
    t
});

/// Compact move: bits 0..6 target square, bits 6..12 source square, or
/// `49 + hand index` for a drop. Promotion is compulsory, so the position
/// gives the resulting face.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Move(u16);
impl Move {
    fn new(from: usize, to: usize) -> Self {
        Self((from << 6 | to) as u16)
    }
    fn from(self) -> usize {
        usize::from(self.0 >> 6)
    }
    fn to(self) -> usize {
        usize::from(self.0 & 63)
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
    reversible: u16,
}

/// Search-internal Tori position.
#[derive(Clone)]
pub struct ToriPosition {
    /// 0 for empty, else `1 + side * 9 + face`.
    board: [u8; SQUARES],
    pieces: [[u64; FACES]; 2],
    occupied: [u64; 2],
    hands: [[u8; HAND_FACES]; 2],
    side: usize,
    hash: u64,
    /// Board and hand material of each side.
    material: [i32; 2],
    /// True if the side to move is in check.
    check: bool,
    /// Plies since the last capture or drop.
    reversible: u16,
    stack: Vec<Undo>,
    /// Hash of each position from the start of the known game to this one.
    keys: Vec<u64>,
    /// Index in `keys` of the search root.
    root: usize,
    /// Departures that the native repetition rule forbids at the root.
    root_forbidden: Vec<Move>,
}

fn code(side: usize, face: usize) -> u8 {
    (1 + side * FACES + face) as u8
}
fn decode_piece(code: u8) -> (usize, usize) {
    let index = usize::from(code - 1);
    (index / FACES, index % FACES)
}

impl ToriPosition {
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
            stack: Vec::with_capacity(160),
            keys: Vec::with_capacity(700),
            root: 0,
            root_forbidden: Vec::new(),
        };
        for square in Square::all() {
            if let Some(Piece { side, face }) = position.piece(square) {
                p.put(side_index(side), face_index(face), square.index());
            }
        }
        for side in [Side::Sente, Side::Gote] {
            for (index, &face) in Face::HANDS.iter().enumerate() {
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
        p
    }

    /// Copy the current position of a native game with the hashes of its
    /// earlier positions and the departures that its repetition rule forbids.
    pub fn from_game(game: &Game) -> Self {
        let mut p = Self::from_position(game.position());
        let mut keys = Vec::with_capacity(game.events().len() + 1);
        let mut reversible = 0u16;
        let mut last: Option<(usize, u32)> = None;
        for count in 0..=game.events().len() {
            let Ok(snapshot) = game.event_prefix(count) else {
                break;
            };
            if last.is_some_and(|(ply, _)| ply == snapshot.board_ply) {
                continue;
            }
            let local = snapshot.local_position();
            let earlier = Self::from_position(local.position());
            let on_board = (earlier.occupied[0] | earlier.occupied[1]).count_ones();
            reversible = match last {
                Some((_, before)) if before == on_board => reversible.saturating_add(1),
                _ => 0,
            };
            last = Some((snapshot.board_ply, on_board));
            keys.push(earlier.hash);
        }
        if keys.last() == Some(&p.hash) {
            p.keys = keys;
            p.reversible = reversible;
        }
        p.root = p.keys.len() - 1;
        if let Some(obligation) = game.obligation() {
            p.root_forbidden = obligation
                .forbidden_departures
                .iter()
                .filter_map(|action| p.find(*action))
                .collect();
        }
        p
    }

    /// The native action of a move in this position.
    pub fn action(&self, m: Move) -> Action {
        let to = square(m.to());
        if m.is_drop() {
            Action::Drop {
                face: Face::ALL[HAND_FACE[m.from() - DROP]],
                to,
            }
        } else {
            let (side, face) = decode_piece(self.board[m.from()]);
            Action::Move {
                from: square(m.from()),
                to,
                result_face: Face::ALL[self.moved_face(side, face, m.from(), m.to())],
            }
        }
    }
    /// The move of a native action, if it is pseudo-legal here.
    pub fn find(&self, action: Action) -> Option<Move> {
        let m = match action {
            Action::Move {
                from,
                to,
                result_face,
            } => {
                let m = Move::new(from.index(), to.index());
                let code = self.board[from.index()];
                if code == 0 {
                    return None;
                }
                let (side, face) = decode_piece(code);
                if self.moved_face(side, face, from.index(), to.index()) != face_index(result_face)
                {
                    return None;
                }
                m
            }
            Action::Drop { face, to } => {
                let face = face_index(face);
                if face == PHOENIX || face != base(face) {
                    return None;
                }
                Move::new(DROP + hand_index(face), to.index())
            }
        };
        self.pseudo_legal(m).then_some(m)
    }
    /// All legal moves, without the root repetition restriction.
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
                face: Face::ALL[face],
            }
        })
    }
    /// Number of pieces of a face in the hand of a side.
    pub fn hand_count(&self, side: Side, face: Face) -> u8 {
        let face = face_index(face);
        if face == PHOENIX || face != base(face) {
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
        self.pieces[side][PHOENIX].trailing_zeros() as usize
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
    fn moved_face(&self, side: usize, face: usize, from: usize, to: usize) -> usize {
        if in_zone(side, from) || in_zone(side, to) {
            match face {
                FALCON => EAGLE,
                SWALLOW => GOOSE,
                other => other,
            }
        } else {
            face
        }
    }
    /// Ray targets of a piece: empty squares and the first occupied square.
    fn ray_targets(&self, side: usize, face: usize, from: usize) -> u64 {
        let all = self.occupied[0] | self.occupied[1];
        let mut targets = 0u64;
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
        for face in [EAGLE, LEFT_QUAIL, RIGHT_QUAIL] {
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
        let enemy = self.occupied[side ^ 1] & !self.pieces[side ^ 1][PHOENIX];
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
                targets &= !own & !self.pieces[side ^ 1][PHOENIX];
                while targets != 0 {
                    let to = targets.trailing_zeros() as usize;
                    targets &= targets - 1;
                    let capture = enemy & (1 << to) != 0;
                    let promotes = face == FALCON && (in_zone(side, from) || in_zone(side, to));
                    if (capture || promotes) == tactical {
                        moves.push(Move::new(from, to));
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
            if HAND_FACE[index] == SWALLOW {
                targets &= self.swallow_drop_squares(side);
            }
            while targets != 0 {
                let to = targets.trailing_zeros() as usize;
                targets &= targets - 1;
                moves.push(Move::new(DROP + index, to));
            }
        }
    }
    /// Squares where the static Swallow drop rules permit a drop.
    fn swallow_drop_squares(&self, side: usize) -> u64 {
        let mut allowed = ALL & !(0x7F << if side == 0 { 0 } else { 42 });
        for file in TABLES.files {
            if (self.pieces[side][SWALLOW] & file).count_ones() >= 2 {
                allowed &= !file;
            }
        }
        allowed
    }
    fn pseudo_legal(&self, m: Move) -> bool {
        let (from, to) = (m.from(), m.to());
        if to >= SQUARES {
            return false;
        }
        let side = self.side;
        if from >= DROP {
            let index = from - DROP;
            return index < HAND_FACES
                && self.hands[side][index] > 0
                && self.board[to] == 0
                && (HAND_FACE[index] != SWALLOW
                    || self.swallow_drop_squares(side) & (1 << to) != 0);
        }
        let code = self.board[from];
        if code == 0 {
            return false;
        }
        let (owner, face) = decode_piece(code);
        if owner != side
            || self.occupied[side] & (1 << to) != 0
            || self.pieces[side ^ 1][PHOENIX] & (1 << to) != 0
        {
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
        // Swallows gain value as they advance; they promote in the zone.
        let mut swallows = self.pieces[side][SWALLOW];
        while swallows != 0 {
            let square = swallows.trailing_zeros() as usize;
            swallows &= swallows - 1;
            let row = square / 7;
            score += 4 * if side == 0 { 6 - row } else { row } as i32;
        }
        // Own pieces next to the Phoenix protect it.
        let king = self.king(side);
        score += 8 * (TABLES.leaps[0][PHOENIX][king] & self.occupied[side]).count_ones() as i32;
        // A piece in hand can go to any empty square.
        let in_hand: i32 = self.hands[side].iter().map(|&n| i32::from(n)).sum();
        score + 10 * in_hand
    }
}

fn square(index: usize) -> Square {
    Square::new((7 - index % 7) as u8, (index / 7 + 1) as u8).expect("board index")
}

impl SearchPosition for ToriPosition {
    type Move = Move;
    const HISTORY_SLOTS: usize = 2 * FACES * SQUARES;

    fn hash(&self) -> u64 {
        self.hash
    }
    fn in_check(&self) -> bool {
        self.check
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<Move>) {
        let start = moves.len();
        self.generate_all(stage, moves);
        if self.ply() == 0 && !self.root_forbidden.is_empty() {
            let mut index = start;
            while index < moves.len() {
                if self.root_forbidden.contains(&moves[index]) {
                    moves.swap_remove(index);
                } else {
                    index += 1;
                }
            }
        }
    }
    fn make(&mut self, m: Move) -> bool {
        let side = self.side;
        let (from, to) = (m.from(), m.to());
        let mut undo = Undo {
            m,
            moved: 0,
            captured: self.board[to],
            hash: self.hash,
            check: self.check,
            reversible: self.reversible,
        };
        let mut swallow_drop = false;
        if from >= DROP {
            let index = from - DROP;
            self.remove_hand(side, index);
            self.put(side, HAND_FACE[index], to);
            swallow_drop = HAND_FACE[index] == SWALLOW;
            self.reversible = 0;
        } else {
            undo.moved = self.board[from];
            let (_, face) = decode_piece(undo.moved);
            if undo.captured != 0 {
                let (owner, victim) = decode_piece(undo.captured);
                self.take(owner, victim, to);
                self.add_hand(side, hand_index(victim));
                self.reversible = 0;
            } else {
                self.reversible = self.reversible.saturating_add(1);
            }
            self.take(side, face, from);
            let result = self.moved_face(side, face, from, to);
            self.put(side, result, to);
        }
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.stack.push(undo);
        self.keys.push(self.hash);
        if self.attacked(self.king(side), side ^ 1) {
            self.unmake();
            return false;
        }
        self.check = self.attacked(self.king(side ^ 1), side);
        // A Swallow drop must not give checkmate.
        if swallow_drop && self.check && !self.has_legal_move() {
            self.unmake();
            return false;
        }
        true
    }
    fn unmake(&mut self) {
        let undo = self.stack.pop().expect("a move to take back");
        self.keys.pop();
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
    }
    fn make_null(&mut self) {
        self.stack.push(Undo {
            m: Move::default(),
            moved: 0,
            captured: 0,
            hash: self.hash,
            check: self.check,
            reversible: self.reversible,
        });
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.keys.push(self.hash);
        self.check = false;
        // A pass is not a position of the game: no repetition across it.
        self.reversible = 0;
    }
    fn unmake_null(&mut self) {
        let undo = self.stack.pop().expect("a null move to take back");
        self.keys.pop();
        self.side ^= 1;
        self.hash = undo.hash;
        self.check = undo.check;
        self.reversible = undo.reversible;
    }
    fn null_move_safe(&self) -> bool {
        // With a piece in hand, or a piece other than Phoenix and Swallows,
        // the side to move nearly always has a useful move.
        let side = self.side;
        self.hands[side].iter().any(|&n| n > 0)
            || self.occupied[side] & !(self.pieces[side][PHOENIX] | self.pieces[side][SWALLOW]) != 0
    }
    fn rule_score(&self) -> Option<i32> {
        // Approximation of the Tori repetition rule. The native rule makes the
        // side to move leave a position at its third occurrence; it gives no
        // draw. The scan looks back only to the last capture or drop. In a drop
        // game a position can also come back through a capture and a drop, so
        // the search can miss a repetition; the root uses the exact native
        // forbidden departures. The search scores a position that occurs again as level, so a
        // side with the better position does not go around in a cycle.
        let current = self.keys.len() - 1;
        let limit = usize::from(self.reversible).min(current);
        let mut earlier = 0;
        let mut back = 2;
        while back <= limit {
            let index = current - back;
            if self.keys[index] == self.hash {
                if index >= self.root {
                    return Some(0);
                }
                earlier += 1;
                if earlier >= 2 {
                    return Some(0);
                }
            }
            back += 2;
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
        let (side, face) = decode_piece(self.board[m.from()]);
        face == FALCON && (in_zone(side, m.from()) || in_zone(side, m.to()))
    }
    fn tactical_order(&self, m: Move) -> i32 {
        let (_, face) = decode_piece(self.board[m.from()]);
        self.tactical_gain(m) * 16 - VALUE[face] / 16
    }
    fn tactical_gain(&self, m: Move) -> i32 {
        if m.is_drop() {
            return 0;
        }
        let (side, face) = decode_piece(self.board[m.from()]);
        let mut gain = VALUE[self.moved_face(side, face, m.from(), m.to())] - VALUE[face];
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
        let mover = VALUE[self.moved_face(side, face, m.from(), m.to())];
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
        self.pseudo_legal(m) && !(self.ply() == 0 && self.root_forbidden.contains(&m))
    }
}
