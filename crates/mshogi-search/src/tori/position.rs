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

/// One ray of a ray piece: its squares up to the maximum distance, and
/// whether square indices grow along the ray (so the nearest blocker is the
/// lowest set bit) or fall (the highest set bit).
#[derive(Clone, Copy)]
struct Ray {
    mask: u64,
    up: bool,
}
const NO_RAY: Ray = Ray { mask: 0, up: false };
const MAX_RAYS: usize = 5;
/// The ray piece faces.
const RAY_FACES: [usize; 3] = [EAGLE, LEFT_QUAIL, RIGHT_QUAIL];

impl Ray {
    /// Squares the ray reaches: up to and including the nearest occupied one.
    fn targets(self, occupied: u64) -> u64 {
        let blockers = self.mask & occupied;
        if blockers == 0 {
            self.mask
        } else if self.up {
            let nearest = blockers.trailing_zeros();
            self.mask & ((2u64 << nearest) - 1)
        } else {
            let nearest = 63 - blockers.leading_zeros();
            self.mask & !((1u64 << nearest) - 1)
        }
    }
    /// The nearest occupied square on the ray, if any.
    fn nearest(self, occupied: u64) -> Option<usize> {
        let blockers = self.mask & occupied;
        (blockers != 0).then(|| {
            if self.up {
                blockers.trailing_zeros() as usize
            } else {
                63 - blockers.leading_zeros() as usize
            }
        })
    }
}

struct Tables {
    /// Rays of [side][face][square], in the order of `RAYS`.
    rays: [[[[Ray; MAX_RAYS]; SQUARES]; FACES]; 2],
    /// Reverse rays of [side][face][square]: the squares from which a piece
    /// of that side and face would reach the square along one of its rays.
    reverse: [[[[Ray; MAX_RAYS]; SQUARES]; FACES]; 2],
    /// Union of the reverse rays of [side][face][square], ignoring blockers.
    lines: [[[u64; SQUARES]; FACES]; 2],
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
        rays: [[[[NO_RAY; MAX_RAYS]; SQUARES]; FACES]; 2],
        reverse: [[[[NO_RAY; MAX_RAYS]; SQUARES]; FACES]; 2],
        lines: [[[0; SQUARES]; FACES]; 2],
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
    let walk = |side: usize, from: usize, right: i8, forward: i8, max: u8| {
        let mut ray = NO_RAY;
        let mut at = from;
        for _ in 0..max {
            let Some(next) = offset(at, side, right, forward) else {
                break;
            };
            ray.mask |= 1 << next;
            ray.up = next > from;
            at = next;
        }
        ray
    };
    for side in 0..2 {
        for face in 0..FACES {
            for square in 0..SQUARES {
                for (index, &(right, forward, max)) in RAYS[face].iter().enumerate() {
                    t.rays[side][face][square][index] = walk(side, square, right, forward, max);
                    let reverse = walk(side, square, -right, -forward, max);
                    t.reverse[side][face][square][index] = reverse;
                    t.lines[side][face][square] |= reverse.mask;
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
        if let Some(repetition) = game.repetition() {
            p.root_forbidden = repetition
                .forbidden
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
        let rays = &TABLES.rays[side][face][from];
        rays[..RAYS[face].len()]
            .iter()
            .fold(0, |targets, ray| targets | ray.targets(all))
    }
    /// Squares that a piece of `side` and `face` on `square` attacks.
    fn attacks(&self, side: usize, face: usize, square: usize) -> u64 {
        let leaps = TABLES.leaps[side][face][square];
        if RAYS[face].is_empty() {
            leaps
        } else {
            leaps | self.ray_targets(side, face, square)
        }
    }
    /// True if a piece of `by` attacks the square.
    fn attacked(&self, target: usize, by: usize) -> bool {
        let t = &TABLES;
        for face in 0..FACES {
            if t.leapers_to[by][face][target] & self.pieces[by][face] != 0 {
                return true;
            }
        }
        let all = self.occupied[0] | self.occupied[1];
        for face in RAY_FACES {
            let sliders = self.pieces[by][face];
            if sliders == 0 {
                continue;
            }
            // The nearest piece on a reverse ray is the only one that can
            // reach the target along that ray.
            for ray in &t.reverse[by][face][target][..RAYS[face].len()] {
                if let Some(nearest) = ray.nearest(all)
                    && sliders & (1 << nearest) != 0
                {
                    return true;
                }
            }
        }
        false
    }
    /// Squares on a ray line between `square` and a ray piece of `by`,
    /// ignoring blockers: only a piece leaving one of them can open a ray
    /// attack of `by` on the square.
    fn ray_lines(&self, square: usize, by: usize) -> u64 {
        RAY_FACES
            .iter()
            .filter(|&&face| self.pieces[by][face] != 0)
            .fold(0, |lines, &face| lines | TABLES.lines[by][face][square])
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
    /// True if the side to move can answer the check of a Swallow just
    /// dropped on `swallow`. A Swallow checks from an adjacent square, so no
    /// move can block it: only a Phoenix move or a capture of the Swallow can
    /// be legal.
    fn can_answer_swallow_check(&mut self, swallow: usize) -> bool {
        let side = self.side;
        let king = self.king(side);
        let mut candidates = Vec::new();
        let mut steps = TABLES.leaps[side][PHOENIX][king] & !self.occupied[side];
        while steps != 0 {
            let to = steps.trailing_zeros() as usize;
            steps &= steps - 1;
            candidates.push(Move::new(king, to));
        }
        for face in 0..FACES {
            let mut attackers = TABLES.leapers_to[side][face][swallow] & self.pieces[side][face];
            while attackers != 0 {
                let from = attackers.trailing_zeros() as usize;
                attackers &= attackers - 1;
                candidates.push(Move::new(from, swallow));
            }
        }
        let all = self.occupied[0] | self.occupied[1];
        for face in RAY_FACES {
            for ray in &TABLES.reverse[side][face][swallow][..RAYS[face].len()] {
                if let Some(from) = ray.nearest(all)
                    && self.pieces[side][face] & (1 << from) != 0
                {
                    candidates.push(Move::new(from, swallow));
                }
            }
        }
        candidates.into_iter().any(|m| {
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
        // The move can expose the own Phoenix only if the side is in check,
        // the Phoenix moves, or the piece leaves a ray line to the Phoenix.
        // A drop outside check adds a piece and never exposes the Phoenix.
        let own_king = self.king(side);
        let test_legality = self.check
            || (from < DROP
                && (from == own_king || self.ray_lines(own_king, side ^ 1) & (1 << from) != 0));
        let mut undo = Undo {
            m,
            moved: 0,
            captured: self.board[to],
            hash: self.hash,
            check: self.check,
            reversible: self.reversible,
        };
        let mut swallow_drop = false;
        let placed;
        if from >= DROP {
            let index = from - DROP;
            self.remove_hand(side, index);
            placed = HAND_FACE[index];
            self.put(side, placed, to);
            swallow_drop = placed == SWALLOW;
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
            placed = self.moved_face(side, face, from, to);
            self.put(side, placed, to);
        }
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.stack.push(undo);
        self.keys.push(self.hash);
        if test_legality && self.attacked(self.king(side), side ^ 1) {
            self.unmake();
            return false;
        }
        // The opponent was not in check before the move. Check now comes from
        // the moved piece itself, or from a ray that its departure opened (a
        // drop opens none; a capture leaves the target square occupied).
        let enemy_king = self.king(side ^ 1);
        self.check = self.attacks(side, placed, to) & (1 << enemy_king) != 0
            || (from < DROP
                && self.ray_lines(enemy_king, side) & (1 << from) != 0
                && self.attacked(enemy_king, side));
        // A Swallow drop must not give checkmate.
        if swallow_drop && self.check && !self.can_answer_swallow_check(to) {
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
        // Tori repetition rule (T06, MSH-208): a move that creates the third
        // occurrence of a position is illegal, so the side that moved here
        // loses. A position that occurred once before inside this search is a
        // cycle without progress and scores level. The scan looks back only to
        // the last capture or drop; in a drop game a position can also come
        // back through a capture and a drop, so the search can miss a
        // repetition. The root uses the exact native restriction.
        let current = self.keys.len() - 1;
        let limit = usize::from(self.reversible).min(current);
        let mut earlier = 0;
        let mut in_search = false;
        let mut back = 2;
        while back <= limit {
            let index = current - back;
            if self.keys[index] == self.hash {
                earlier += 1;
                in_search |= index >= self.root;
                if earlier >= 2 {
                    return Some(MATE_SCORE - self.ply() as i32);
                }
            }
            back += 2;
        }
        in_search.then_some(0)
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

/// One measured operation of [`movegen_profile`]: total nanoseconds over all
/// repeats and positions, and the number of operations (or generated moves).
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct ProfileRow {
    /// Operation name.
    pub label: String,
    /// Total time in nanoseconds.
    pub nanos: u128,
    /// Operations, or moves generated for a generation row.
    pub count: u64,
    /// Calls (pieces or hand kinds for generation rows).
    pub calls: u64,
}

/// Cost of the search-side move generation by piece kind, drops, make/unmake
/// by move class, the attack test and perft (MSH-200, Tori Move Generation
/// Deep Dive). A measurement tool, not used by the search.
#[doc(hidden)]
#[allow(clippy::needless_range_loop)]
pub fn movegen_profile(positions: &[Position], repeats: u32) -> Vec<ProfileRow> {
    use std::hint::black_box;
    use std::time::Instant;
    const NAMES: [&str; FACES] = [
        "Phoenix",
        "Falcon",
        "Eagle",
        "Crane",
        "Pheasant",
        "LeftQuail",
        "RightQuail",
        "Swallow",
        "Goose",
    ];
    let mut rows: Vec<ProfileRow> = Vec::new();
    let mut add = |label: &str, nanos: u128, count: u64, calls: u64| match rows
        .iter_mut()
        .find(|r| r.label == label)
    {
        Some(r) => {
            r.nanos += nanos;
            r.count += count;
            r.calls += calls;
        }
        None => rows.push(ProfileRow {
            label: label.into(),
            nanos,
            count,
            calls,
        }),
    };
    let mut copies: Vec<ToriPosition> = positions.iter().map(ToriPosition::from_position).collect();
    for p in &mut copies {
        let side = p.side;
        let own = p.occupied[side];
        let blocked = own | p.pieces[side ^ 1][PHOENIX];
        // Targets of each piece kind of the side to move.
        for face in 0..FACES {
            let pieces = p.pieces[side][face];
            if pieces == 0 {
                continue;
            }
            let mut generated = 0u64;
            let start = Instant::now();
            for _ in 0..repeats {
                let mut set = black_box(pieces);
                while set != 0 {
                    let from = set.trailing_zeros() as usize;
                    set &= set - 1;
                    let mut targets = TABLES.leaps[side][face][from];
                    if !RAYS[face].is_empty() {
                        targets |= p.ray_targets(side, face, from);
                    }
                    generated += u64::from(black_box(targets & !blocked).count_ones());
                }
            }
            add(
                &format!("targets {}", NAMES[face]),
                start.elapsed().as_nanos(),
                generated,
                u64::from(repeats) * u64::from(pieces.count_ones()),
            );
        }
        // Drop targets and moves.
        let kinds = p.hands[side].iter().filter(|&&n| n > 0).count() as u64;
        if kinds > 0 {
            let mut moves = Vec::with_capacity(256);
            let start = Instant::now();
            for _ in 0..repeats {
                moves.clear();
                let empty = ALL & !(p.occupied[0] | p.occupied[1]);
                for index in 0..HAND_FACES {
                    if p.hands[side][index] == 0 {
                        continue;
                    }
                    let mut targets = empty;
                    if HAND_FACE[index] == SWALLOW {
                        targets &= p.swallow_drop_squares(side);
                    }
                    while targets != 0 {
                        let to = targets.trailing_zeros() as usize;
                        targets &= targets - 1;
                        moves.push(Move::new(DROP + index, to));
                    }
                }
                black_box(&moves);
            }
            add(
                "drops",
                start.elapsed().as_nanos(),
                moves.len() as u64 * u64::from(repeats),
                kinds * u64::from(repeats),
            );
        }
        // Full generation of both stages.
        let mut moves = Vec::with_capacity(256);
        let start = Instant::now();
        for _ in 0..repeats {
            moves.clear();
            p.generate_all(Stage::Tactical, &mut moves);
            p.generate_all(Stage::Quiet, &mut moves);
            black_box(&moves);
        }
        add(
            "generate all",
            start.elapsed().as_nanos(),
            moves.len() as u64 * u64::from(repeats),
            u64::from(repeats),
        );
        // Attack test of the own Phoenix (the legality test of each move).
        let king = p.king(side);
        let start = Instant::now();
        for _ in 0..repeats {
            black_box(p.attacked(black_box(king), side ^ 1));
        }
        add(
            "attacked(Phoenix)",
            start.elapsed().as_nanos(),
            u64::from(repeats),
            u64::from(repeats),
        );
        // Make and unmake by move class.
        for &m in &moves {
            let class = if m.is_drop() {
                if HAND_FACE[m.from() - DROP] == SWALLOW {
                    "make swallow drop"
                } else {
                    "make drop"
                }
            } else if p.board[m.to()] != 0 {
                "make capture"
            } else {
                "make move"
            };
            let mut legal = 0u64;
            let start = Instant::now();
            for _ in 0..repeats {
                if p.make(black_box(m)) {
                    legal += 1;
                    p.unmake();
                }
            }
            add(class, start.elapsed().as_nanos(), legal, u64::from(repeats));
        }
    }
    // Perft to depth 3 (generation, make and unmake together).
    for p in &mut copies {
        let start = Instant::now();
        let nodes = p.perft(3);
        add("perft 3", start.elapsed().as_nanos(), nodes, 1);
    }
    rows
}
