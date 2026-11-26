//! Private Xiangqi board and classical evaluation for the shared search core.
//! Real move admission and WXF adjudication remain native responsibilities.
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use crate::repetition::{cycle_start, earlier_occurrences};
use mshogi_core::xiangqi::{
    Action, EventKind, Game, Intersection, Piece, PieceKind, Position, Side,
};
use std::sync::LazyLock;

const N: usize = 90;
const KINDS: [PieceKind; 7] = [
    PieceKind::General,
    PieceKind::Advisor,
    PieceKind::Elephant,
    PieceKind::Horse,
    PieceKind::Rook,
    PieceKind::Cannon,
    PieceKind::Soldier,
];
/// Exchange values (SEE, capture gain and order): the compiled material
/// weights, so that exchanges and the evaluation agree.
const VALUE: [i32; 7] = {
    use super::eval::{DEFAULT, MATERIAL};
    [
        0,
        DEFAULT[MATERIAL],
        DEFAULT[MATERIAL + 1],
        DEFAULT[MATERIAL + 2],
        DEFAULT[MATERIAL + 3],
        DEFAULT[MATERIAL + 4],
        DEFAULT[MATERIAL + 5],
    ]
};
const GENERAL: usize = 0;
const ADVISOR: usize = 1;
const ELEPHANT: usize = 2;
const HORSE: usize = 3;
const ROOK: usize = 4;
const CANNON: usize = 5;
const SOLDIER: usize = 6;
const ALL: u128 = (1 << N) - 1;
const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
fn owner(side: Side) -> usize {
    if side == Side::Red { 0 } else { 1 }
}
fn native_side(side: usize) -> Side {
    if side == 0 { Side::Red } else { Side::Black }
}
fn kind_index(kind: PieceKind) -> usize {
    KINDS.iter().position(|&k| k == kind).unwrap()
}
fn point(at: usize) -> Intersection {
    Intersection::new((at % 9) as u8, (at / 9) as u8).expect("board index")
}
fn code(side: usize, kind: usize) -> u8 {
    (1 + side * 7 + kind) as u8
}
fn piece_parts(piece: u8) -> (usize, usize) {
    let p = usize::from(piece - 1);
    (p / 7, p % 7)
}
fn offset(at: usize, dx: i8, dy: i8) -> Option<usize> {
    let x = (at % 9) as i8 + dx;
    let y = (at / 9) as i8 + dy;
    ((0..9).contains(&x) && (0..10).contains(&y)).then_some((y * 9 + x) as usize)
}
fn palace(side: usize, at: usize) -> bool {
    (3..=5).contains(&(at % 9)) && if side == 0 { at / 9 <= 2 } else { at / 9 >= 7 }
}
fn crossed(side: usize, at: usize) -> bool {
    if side == 0 { at / 9 >= 5 } else { at / 9 <= 4 }
}

#[derive(Clone, Copy, Default)]
struct Jump {
    to: u8,
    blocker: u8,
}
struct Tables {
    jumps: [[[[Jump; 8]; N]; 7]; 2],
    counts: [[[u8; N]; 7]; 2],
    pieces: [[[u64; N]; 7]; 2],
    side: u64,
}
/// Squares on the rank and file of each square (excluding it).
static LINE_MASK: LazyLock<[u128; N]> = LazyLock::new(|| {
    std::array::from_fn(|at| {
        (0..N)
            .filter(|&o| o != at && (o % 9 == at % 9 || o / 9 == at / 9))
            .fold(0, |m, o| m | 1 << o)
    })
});
/// Squares a Horse move away from each square.
static HORSE_SQUARES: LazyLock<[u128; N]> = LazyLock::new(|| {
    std::array::from_fn(|at| {
        [
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
            (-1, -2),
            (-2, -1),
        ]
        .into_iter()
        .filter_map(|(dx, dy)| offset(at, dx, dy))
        .fold(0, |m, o| m | 1 << o)
    })
});
/// Horses that can attack each square: (source, leg) pairs; `N` marks unused.
static HORSE_REVERSE: LazyLock<[[(u8, u8); 8]; N]> = LazyLock::new(|| {
    std::array::from_fn(|target| {
        let mut out = [(N as u8, N as u8); 8];
        for (i, (dx, dy)) in [
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
            (-1, -2),
            (-2, -1),
        ]
        .into_iter()
        .enumerate()
        {
            if let Some(from) = offset(target, dx, dy) {
                // The Horse moves from `from` by (-dx, -dy); its leg is the
                // first step along the two-square axis.
                let leg = offset(
                    from,
                    if dx.abs() == 2 { -dx / 2 } else { 0 },
                    if dy.abs() == 2 { -dy / 2 } else { 0 },
                )
                .expect("on board");
                out[i] = (from as u8, leg as u8);
            }
        }
        out
    })
});
/// Orthogonally adjacent squares of each square.
static ADJACENT: LazyLock<[u128; N]> = LazyLock::new(|| {
    std::array::from_fn(|at| {
        DIRECTIONS
            .into_iter()
            .filter_map(|(dx, dy)| offset(at, dx, dy))
            .fold(0, |m, o| m | 1 << o)
    })
});
/// Static evaluation of each piece on each square with the compiled weights.
static STATIC: LazyLock<Box<[[[i32; N]; 7]; 2]>> = LazyLock::new(|| {
    let mut t = Box::new([[[0; N]; 7]; 2]);
    for side in 0..2 {
        for kind in 0..7 {
            for at in 0..N {
                XiangqiPosition::static_features(side, kind, at, &mut |index, value| {
                    t[side][kind][at] += super::eval::DEFAULT[index] * value
                });
            }
        }
    }
    t
});
/// Chariot and Cannon targets along one rank (9 squares) or file (10
/// squares), indexed by mode, position and line occupancy. Modes: 0 Chariot,
/// 1 Cannon quiet moves, 2 Cannon capture influence (squares beyond one
/// screen up to and including the next piece), as in the ray walk.
struct Lines {
    rank: Vec<u16>,
    file: Vec<u16>,
    /// File-0 squares of a 10-bit file mask; shift by the file index.
    spread: Vec<u128>,
}
fn line_targets(len: usize, pos: usize, occ: u32, mode: usize) -> u16 {
    let mut out = 0u16;
    for dir in [1i32, -1] {
        let mut at = pos as i32 + dir;
        let mut screened = false;
        while (0..len as i32).contains(&at) {
            let occupied = occ & (1 << at) != 0;
            match mode {
                0 => {
                    out |= 1 << at;
                    if occupied {
                        break;
                    }
                }
                1 => {
                    if occupied {
                        break;
                    }
                    out |= 1 << at;
                }
                _ => {
                    if !screened {
                        screened = occupied;
                    } else {
                        out |= 1 << at;
                        if occupied {
                            break;
                        }
                    }
                }
            }
            at += dir;
        }
    }
    out
}
static LINES: LazyLock<Lines> = LazyLock::new(|| {
    let mut rank = vec![0; 3 * 9 * 512];
    let mut file = vec![0; 3 * 10 * 1024];
    for mode in 0..3 {
        for pos in 0..9 {
            for occ in 0..512u32 {
                rank[(mode * 9 + pos) * 512 + occ as usize] = line_targets(9, pos, occ, mode);
            }
        }
        for pos in 0..10 {
            for occ in 0..1024u32 {
                file[(mode * 10 + pos) * 1024 + occ as usize] = line_targets(10, pos, occ, mode);
            }
        }
    }
    let spread = (0..1024u32)
        .map(|mask| {
            (0..10)
                .filter(|r| mask & (1 << r) != 0)
                .fold(0u128, |acc, r| acc | 1 << (r * 9))
        })
        .collect();
    Lines { rank, file, spread }
});
#[allow(clippy::needless_range_loop)]
static TABLES: LazyLock<Box<Tables>> = LazyLock::new(|| {
    let mut t = Box::new(Tables {
        jumps: [[[[Jump::default(); 8]; N]; 7]; 2],
        counts: [[[0; N]; 7]; 2],
        pieces: [[[0; N]; 7]; 2],
        side: 0,
    });
    for side in 0..2 {
        for kind in 0..7 {
            for from in 0..N {
                let steps: &[(i8, i8)] = match kind {
                    GENERAL => &DIRECTIONS,
                    ADVISOR => &[(1, 1), (1, -1), (-1, 1), (-1, -1)],
                    ELEPHANT => &[(2, 2), (2, -2), (-2, 2), (-2, -2)],
                    HORSE => &[
                        (2, 1),
                        (2, -1),
                        (-2, 1),
                        (-2, -1),
                        (1, 2),
                        (1, -2),
                        (-1, 2),
                        (-1, -2),
                    ],
                    SOLDIER => {
                        if side == 0 {
                            &[(0, 1), (-1, 0), (1, 0)]
                        } else {
                            &[(0, -1), (-1, 0), (1, 0)]
                        }
                    }
                    _ => &[],
                };
                for &(dx, dy) in steps {
                    let Some(to) = offset(from, dx, dy) else {
                        continue;
                    };
                    if (kind == GENERAL || kind == ADVISOR) && !palace(side, to) {
                        continue;
                    }
                    if kind == ELEPHANT && crossed(side, to) {
                        continue;
                    }
                    if kind == SOLDIER && dx != 0 && !crossed(side, from) {
                        continue;
                    }
                    let blocker = match kind {
                        ELEPHANT => offset(from, dx / 2, dy / 2).unwrap() as u8,
                        HORSE => offset(
                            from,
                            if dx.abs() == 2 { dx / 2 } else { 0 },
                            if dy.abs() == 2 { dy / 2 } else { 0 },
                        )
                        .unwrap() as u8,
                        _ => N as u8,
                    };
                    let count = &mut t.counts[side][kind][from];
                    t.jumps[side][kind][from][usize::from(*count)] = Jump {
                        to: to as u8,
                        blocker,
                    };
                    *count += 1;
                }
            }
        }
    }
    let mut state = 0x581a_5eed_0000_0181u64;
    let mut next = || {
        state = state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    };
    for side in 0..2 {
        for kind in 0..7 {
            for at in 0..N {
                t.pieces[side][kind][at] = next();
            }
        }
    }
    t.side = next();
    t
});

/// Seven target bits and seven source bits, no promotion or drops.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Move(u16);
impl Move {
    fn new(from: usize, to: usize) -> Self {
        Self(((from << 7) | to) as u16)
    }
    fn from(self) -> usize {
        usize::from(self.0 >> 7)
    }
    fn to(self) -> usize {
        usize::from(self.0 & 127)
    }
}
#[derive(Clone, Copy)]
struct Undo {
    m: Move,
    captured: u8,
    hash: u64,
    check: bool,
    reversible: usize,
}
/// Array board with 128-bit piece sets and incremental hash/evaluation.
#[derive(Clone)]
pub struct XiangqiPosition {
    board: [u8; N],
    pieces: [[u128; 7]; 2],
    occupied: [u128; 2],
    /// Occupancy of each file, bit r = rank r.
    files: [u16; 9],
    side: usize,
    hash: u64,
    material: [i32; 2],
    /// Static evaluation terms (material and placement) with the compiled weights.
    accumulated: [i32; 2],
    check: bool,
    stack: Vec<Undo>,
    keys: Vec<u64>,
    checks: Vec<bool>,
    /// Per position up to the root: the move into it chased (search
    /// approximation of WXF). Positions after the root are computed only
    /// when a repetition needs them.
    pub(crate) chases: Vec<bool>,
    /// Compute chase flags in `make` (only while replaying game history).
    track_chases: bool,
    root: usize,
    /// Known plies after the latest capture or null move.
    reversible: usize,
}
impl XiangqiPosition {
    /// Copy the native board. No earlier history is available.
    pub fn from_position(native: &Position) -> Self {
        let mut p = Self {
            board: [0; N],
            pieces: [[0; 7]; 2],
            occupied: [0; 2],
            files: [0; 9],
            side: owner(native.side_to_move()),
            hash: 0,
            material: [0; 2],
            accumulated: [0; 2],
            check: false,
            stack: Vec::with_capacity(160),
            keys: Vec::with_capacity(700),
            checks: Vec::with_capacity(700),
            chases: Vec::with_capacity(700),
            track_chases: false,
            root: 0,
            reversible: 0,
        };
        for (at, piece) in native.pieces() {
            p.put(owner(piece.side), kind_index(piece.kind), at.index());
        }
        if p.side == 1 {
            p.hash ^= TABLES.side;
        }
        p.check = p.attacked(p.general(p.side), p.side ^ 1);
        p.keys.push(p.hash);
        p.checks.push(p.check);
        p.chases.push(false);
        p
    }
    /// Replay only recorded board actions on the private board, once per request.
    /// Warnings, claims and other non-board events do not add repetition keys.
    pub fn from_game(game: &Game) -> Self {
        let mut p = Self::from_position(game.root().position());
        p.track_chases = true;
        for event in game.events() {
            if let EventKind::BoardAction { action, .. } = event.event {
                let m = p.find(action).expect("native action is pseudo-legal");
                assert!(p.make(m), "native action is legal");
            }
        }
        p.stack.clear();
        p.track_chases = false;
        p.root = p.keys.len() - 1;
        p
    }
    /// Complete native action of a compact move.
    pub fn action(&self, m: Move) -> Action {
        Action {
            from: point(m.from()),
            to: point(m.to()),
        }
    }
    /// Encode a pseudo-legal native action.
    pub fn find(&self, action: Action) -> Option<Move> {
        let m = Move::new(action.from.index(), action.to.index());
        self.is_pseudo_legal(m).then_some(m)
    }
    /// All local legal moves, without history adjudication.
    pub fn legal_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        self.generate(Stage::Tactical, &mut moves);
        self.generate(Stage::Quiet, &mut moves);
        moves.retain(|&m| {
            let ok = self.make(m);
            if ok {
                self.unmake();
            }
            ok
        });
        moves
    }
    /// Exact-depth local board leaves, for diagnostics.
    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut moves = Vec::new();
        self.generate(Stage::Tactical, &mut moves);
        self.generate(Stage::Quiet, &mut moves);
        let mut leaves = 0;
        for m in moves {
            if self.make(m) {
                leaves += self.perft(depth - 1);
                self.unmake();
            }
        }
        leaves
    }
    /// Native side to move.
    pub fn side_to_move(&self) -> Side {
        native_side(self.side)
    }
    /// Piece at an intersection.
    pub fn piece(&self, at: Intersection) -> Option<Piece> {
        (self.board[at.index()] != 0).then(|| {
            let (s, k) = piece_parts(self.board[at.index()]);
            Piece::new(native_side(s), KINDS[k])
        })
    }
    /// Material without placement terms.
    pub fn material(&self, side: Side) -> i32 {
        self.material[owner(side)]
    }
    /// Recompute board hash independently of the incremental update.
    pub fn hash_from_scratch(&self) -> u64 {
        let mut hash = if self.side == 1 { TABLES.side } else { 0 };
        for (at, &p) in self.board.iter().enumerate() {
            if p != 0 {
                let (s, k) = piece_parts(p);
                hash ^= TABLES.pieces[s][k][at];
            }
        }
        hash
    }
    fn general(&self, side: usize) -> usize {
        self.pieces[side][GENERAL].trailing_zeros() as usize
    }
    /// Static features of one piece: material and placement, value +1.
    fn static_features(side: usize, kind: usize, at: usize, sink: &mut impl FnMut(usize, i32)) {
        use super::eval::*;
        let rank = if side == 0 { at / 9 } else { 9 - at / 9 };
        let distance = (at % 9).abs_diff(4);
        match kind {
            GENERAL => {
                if rank == 1 {
                    sink(GENERAL_RANK1, 1);
                } else if rank == 2 {
                    sink(GENERAL_RANK2, 1);
                }
                if distance != 0 {
                    sink(GENERAL_OFF_FILE, 1);
                }
            }
            ADVISOR => {
                sink(MATERIAL, 1);
                if rank == 1 && distance == 0 {
                    sink(ADVISOR_CENTER, 1);
                }
            }
            ELEPHANT => {
                sink(MATERIAL + 1, 1);
                if rank == 2 && distance == 0 {
                    sink(ELEPHANT_CENTER, 1);
                }
            }
            HORSE => {
                sink(MATERIAL + 2, 1);
                sink(HORSE_RANK + rank, 1);
                sink(HORSE_FILE + distance, 1);
            }
            ROOK => {
                sink(MATERIAL + 3, 1);
                sink(ROOK_RANK + rank, 1);
                sink(ROOK_FILE + distance, 1);
            }
            CANNON => {
                sink(MATERIAL + 4, 1);
                sink(CANNON_RANK + rank, 1);
                sink(CANNON_FILE + distance, 1);
            }
            _ => {
                sink(MATERIAL + 5, 1);
                sink(SOLDIER_RANK + rank, 1);
                if rank >= 5 {
                    sink(SOLDIER_FILE + distance, 1);
                }
            }
        }
    }
    fn put(&mut self, side: usize, kind: usize, at: usize) {
        self.board[at] = code(side, kind);
        self.pieces[side][kind] |= 1 << at;
        self.occupied[side] |= 1 << at;
        self.files[at % 9] |= 1 << (at / 9);
        self.hash ^= TABLES.pieces[side][kind][at];
        self.material[side] += super::piece_value(KINDS[kind]);
        self.accumulated[side] += STATIC[side][kind][at];
    }
    fn take(&mut self, side: usize, kind: usize, at: usize) {
        self.board[at] = 0;
        self.pieces[side][kind] &= !(1 << at);
        self.occupied[side] &= !(1 << at);
        self.files[at % 9] &= !(1 << (at / 9));
        self.hash ^= TABLES.pieces[side][kind][at];
        self.material[side] -= super::piece_value(KINDS[kind]);
        self.accumulated[side] -= STATIC[side][kind][at];
    }
    /// All local targets before friendly occupancy and royal-capture filtering.
    /// For Cannons, quiet targets precede the screen; capture targets follow it.
    fn targets(&self, side: usize, kind: usize, from: usize, capture: bool) -> u128 {
        if kind == ROOK || kind == CANNON {
            let mode = if kind == ROOK {
                0
            } else if capture {
                2
            } else {
                1
            };
            let (x, y) = (from % 9, from / 9);
            let occ = self.occupied[0] | self.occupied[1];
            let rank_occ = ((occ >> (9 * y)) & 0x1FF) as usize;
            let lines = &*LINES;
            let rank = u128::from(lines.rank[(mode * 9 + x) * 512 + rank_occ]) << (9 * y);
            let file_mask = lines.file[(mode * 10 + y) * 1024 + usize::from(self.files[x])];
            return rank | lines.spread[usize::from(file_mask)] << x;
        }
        let mut targets = 0;
        let t = &TABLES;
        for jump in &t.jumps[side][kind][from][..usize::from(t.counts[side][kind][from])] {
            if usize::from(jump.blocker) == N || self.board[usize::from(jump.blocker)] == 0 {
                targets |= 1 << jump.to;
            }
        }
        targets
    }
    /// Table targets of a Chariot (kind 4) or Cannon (kind 5), for tests.
    #[cfg(test)]
    pub(crate) fn slider_targets(&self, kind: usize, from: usize, capture: bool) -> u128 {
        self.targets(0, kind, from, capture)
    }
    /// Reference ray walk for Chariot and Cannon targets, for differential tests.
    #[cfg(test)]
    pub(crate) fn slider_targets_by_walk(&self, kind: usize, from: usize, capture: bool) -> u128 {
        {
            let mut targets = 0;
            for (dx, dy) in DIRECTIONS {
                let mut at = from;
                let mut screened = false;
                while let Some(to) = offset(at, dx, dy) {
                    let occupied = self.board[to] != 0;
                    if kind == ROOK {
                        targets |= 1 << to;
                        if occupied {
                            break;
                        }
                    } else if !screened {
                        if occupied {
                            screened = true;
                            if !capture {
                                break;
                            }
                        } else if !capture {
                            targets |= 1 << to;
                        }
                    } else {
                        // Capture-mode influence covers empty squares beyond
                        // one screen too, as required by geometric defense.
                        targets |= 1 << to;
                        if occupied {
                            break;
                        }
                    }
                    at = to;
                }
            }
            targets
        }
    }
    fn attacked(&self, target: usize, by: usize) -> bool {
        // A flying General attacks only the opposing General, never an
        // arbitrary distant square. Ordinary palace steps use the tables.
        let their = self.general(by);
        if target % 9 == their % 9 && self.board[target] == code(by ^ 1, GENERAL) {
            let (low, high) = if their < target {
                (their, target)
            } else {
                (target, their)
            };
            if (low + 9..high).step_by(9).all(|at| self.board[at] == 0) {
                return true;
            }
        }
        // Chariots and Cannons: lines are symmetric, so the Chariot and Cannon
        // capture lines from the target find every attacker.
        if self.pieces[by][ROOK] | self.pieces[by][CANNON] != 0 {
            let rook_line = self.targets(by, ROOK, target, true);
            if rook_line & self.pieces[by][ROOK] != 0 {
                return true;
            }
            let cannon_line = self.targets(by, CANNON, target, true);
            if cannon_line & self.pieces[by][CANNON] != 0 {
                return true;
            }
        }
        // Jumping and stepping pieces, found from the target with their
        // legs, eyes, palace and river rules checked directly.
        let general = code(by, GENERAL);
        let advisor = code(by, ADVISOR);
        if palace(by, target) {
            for (dx, dy) in DIRECTIONS {
                if offset(target, dx, dy).is_some_and(|at| self.board[at] == general) {
                    return true;
                }
            }
            if self.pieces[by][ADVISOR] != 0 {
                for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                    if offset(target, dx, dy).is_some_and(|at| self.board[at] == advisor) {
                        return true;
                    }
                }
            }
        }
        if self.pieces[by][ELEPHANT] != 0 && !crossed(by, target) {
            let elephant = code(by, ELEPHANT);
            for (dx, dy) in [(2, 2), (2, -2), (-2, 2), (-2, -2)] {
                if let Some(at) = offset(target, dx, dy)
                    && self.board[at] == elephant
                    && self.board[offset(target, dx / 2, dy / 2).expect("on board")] == 0
                {
                    return true;
                }
            }
        }
        if self.pieces[by][HORSE] != 0 {
            let horse = code(by, HORSE);
            for &(from, leg) in &HORSE_REVERSE[target] {
                if from != N as u8
                    && self.board[usize::from(from)] == horse
                    && self.board[usize::from(leg)] == 0
                {
                    return true;
                }
            }
        }
        if self.pieces[by][SOLDIER] != 0 {
            let soldier = code(by, SOLDIER);
            let back: i8 = if by == 0 { -1 } else { 1 };
            if offset(target, 0, back).is_some_and(|at| self.board[at] == soldier) {
                return true;
            }
            for dx in [-1, 1] {
                if let Some(at) = offset(target, dx, 0)
                    && self.board[at] == soldier
                    && crossed(by, at)
                {
                    return true;
                }
            }
        }
        false
    }
    /// Reference attack test over every piece's targets, for differential tests.
    #[cfg(test)]
    pub(crate) fn attacked_by_targets(&self, target: usize, by: usize) -> bool {
        let their = self.general(by);
        if target % 9 == their % 9 && self.board[target] == code(by ^ 1, GENERAL) {
            let (low, high) = if their < target {
                (their, target)
            } else {
                (target, their)
            };
            if (low + 9..high).step_by(9).all(|at| self.board[at] == 0) {
                return true;
            }
        }
        (0..7).any(|kind| {
            let mut pieces = self.pieces[by][kind];
            while pieces != 0 {
                let from = pieces.trailing_zeros() as usize;
                pieces &= pieces - 1;
                if self.targets(by, kind, from, true) & (1 << target) != 0 {
                    return true;
                }
            }
            false
        })
    }
    /// Fast attack test, exposed for differential tests.
    #[cfg(test)]
    pub(crate) fn attacked_fast(&self, target: usize, by: usize) -> bool {
        self.attacked(target, by)
    }
    /// Chase flags of the positions `start..=current`: stored flags up to the
    /// root, and for later positions the search moves walked back on a
    /// scratch board.
    pub(crate) fn chase_flags(&self, start: usize) -> Vec<bool> {
        let current = self.keys.len() - 1;
        let mut flags = vec![false; current + 1 - start];
        let mut b = self.scratch();
        for i in (start..=current).rev() {
            if i <= self.root {
                flags[i - start] = self.chases[i];
                continue;
            }
            // No null move lies in the window: a null move resets
            // `reversible`, which bounds the repetition scan.
            let undo = self.stack[i - self.root - 1];
            let (from, to) = (undo.m.from(), undo.m.to());
            debug_assert!(b.board[to] != 0, "board move in the repetition window");
            let (side, kind) = piece_parts(b.board[to]);
            flags[i - start] = undo.captured == 0 && b.chases_after(side, kind, to);
            b.take(side, kind, to);
            b.put(side, kind, from);
            if undo.captured != 0 {
                let (s, k) = piece_parts(undo.captured);
                b.put(s, k, to);
            }
        }
        flags
    }
    /// Leg squares of `side`'s opponent Horses that attack `general` once the
    /// leg is empty (a piece leaving one exposes the General).
    fn enemy_horse_legs(&self, side: usize, general: usize) -> u128 {
        self.horse_legs(side ^ 1, general)
    }
    /// Leg squares of `side`'s Horses aimed at the opposing `general`.
    fn own_horse_legs(&self, side: usize, general: usize) -> u128 {
        self.horse_legs(side, general)
    }
    fn horse_legs(&self, horses: usize, general: usize) -> u128 {
        let wanted = code(horses, HORSE);
        let mut legs = 0;
        if self.pieces[horses][HORSE] == 0 {
            return 0;
        }
        for (dx, dy) in [
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
            (-1, -2),
            (-2, -1),
        ] {
            if let Some(at) = offset(general, dx, dy)
                && self.board[at] == wanted
            {
                let leg = offset(
                    at,
                    if dx.abs() == 2 { -dx / 2 } else { 0 },
                    if dy.abs() == 2 { -dy / 2 } else { 0 },
                )
                .expect("on board");
                legs |= 1 << leg;
            }
        }
        legs
    }
    /// Search approximation of a WXF chase by the piece that just moved to
    /// `to`: it attacks an opposing piece other than the General and an
    /// uncrossed Soldier that is not protected, or a Horse or Cannon attacks
    /// a Chariot. Generals and Soldiers may chase freely.
    fn chases_after(&self, side: usize, kind: usize, to: usize) -> bool {
        if kind == GENERAL || kind == SOLDIER {
            return false;
        }
        let them = side ^ 1;
        let mut victims =
            self.targets(side, kind, to, true) & self.occupied[them] & !self.pieces[them][GENERAL];
        while victims != 0 {
            let at = victims.trailing_zeros() as usize;
            victims &= victims - 1;
            let (_, victim) = piece_parts(self.board[at]);
            if victim == SOLDIER && !crossed(them, at) {
                continue;
            }
            if victim == ROOK && (kind == HORSE || kind == CANNON) {
                return true;
            }
            if !self.attacked(at, them) {
                return true;
            }
        }
        false
    }
    /// Static exchange evaluation of a move: the material balance of the
    /// best sequence of captures on its target square, least valuable
    /// attacker first. Cannon screens and Horse legs change as pieces leave,
    /// so attackers are found again on a scratch board after each capture.
    /// Pins and General exposure are ignored, as in the usual SEE.
    pub fn see(&self, m: Move) -> i32 {
        let value = |kind: usize| if kind == GENERAL { 10_000 } else { VALUE[kind] };
        let mut b = self.scratch();
        let (from, to) = (m.from(), m.to());
        let (side, mut kind) = piece_parts(self.board[from]);
        let mut gain = [0i32; 32];
        if self.board[to] != 0 {
            let (s, k) = piece_parts(self.board[to]);
            gain[0] = value(k);
            b.take(s, k, to);
        }
        b.take(side, kind, from);
        b.put(side, kind, to);
        let mut depth = 0;
        let mut turn = side ^ 1;
        loop {
            let attacker = b.least_attacker(to, turn);
            let Some((next, at)) = attacker else { break };
            if depth + 1 >= gain.len() {
                break;
            }
            depth += 1;
            gain[depth] = value(kind) - gain[depth - 1];
            b.take(turn ^ 1, kind, to);
            b.take(turn, next, at);
            b.put(turn, next, to);
            kind = next;
            turn ^= 1;
        }
        while depth > 0 {
            gain[depth - 1] = -(-gain[depth - 1]).max(gain[depth]);
            depth -= 1;
        }
        gain[0]
    }
    /// Least valuable piece of `by` that attacks `target`, with its square,
    /// found from the target as in `attacked`.
    fn least_attacker(&self, target: usize, by: usize) -> Option<(usize, usize)> {
        // Same direct checks as `attacked`, in value order.
        if self.pieces[by][SOLDIER] != 0 {
            let soldier = code(by, SOLDIER);
            let back: i8 = if by == 0 { -1 } else { 1 };
            if let Some(at) = offset(target, 0, back)
                && self.board[at] == soldier
            {
                return Some((SOLDIER, at));
            }
            for dx in [-1, 1] {
                if let Some(at) = offset(target, dx, 0)
                    && self.board[at] == soldier
                    && crossed(by, at)
                {
                    return Some((SOLDIER, at));
                }
            }
        }
        if palace(by, target) && self.pieces[by][ADVISOR] != 0 {
            let advisor = code(by, ADVISOR);
            for (dx, dy) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                if let Some(at) = offset(target, dx, dy)
                    && self.board[at] == advisor
                {
                    return Some((ADVISOR, at));
                }
            }
        }
        if self.pieces[by][ELEPHANT] != 0 && !crossed(by, target) {
            let elephant = code(by, ELEPHANT);
            for (dx, dy) in [(2, 2), (2, -2), (-2, 2), (-2, -2)] {
                if let Some(at) = offset(target, dx, dy)
                    && self.board[at] == elephant
                    && self.board[offset(target, dx / 2, dy / 2).expect("on board")] == 0
                {
                    return Some((ELEPHANT, at));
                }
            }
        }
        if self.pieces[by][HORSE] != 0 {
            let horse = code(by, HORSE);
            for &(from, leg) in &HORSE_REVERSE[target] {
                if from != N as u8
                    && self.board[usize::from(from)] == horse
                    && self.board[usize::from(leg)] == 0
                {
                    return Some((HORSE, usize::from(from)));
                }
            }
        }
        if self.pieces[by][CANNON] != 0 {
            let cannons = self.targets(by, CANNON, target, true) & self.pieces[by][CANNON];
            if cannons != 0 {
                return Some((CANNON, cannons.trailing_zeros() as usize));
            }
        }
        if self.pieces[by][ROOK] != 0 {
            let rooks = self.targets(by, ROOK, target, true) & self.pieces[by][ROOK];
            if rooks != 0 {
                return Some((ROOK, rooks.trailing_zeros() as usize));
            }
        }
        if self.pieces[by][GENERAL] == 0 {
            // A General captured earlier in the exchange.
            return None;
        }
        let general = self.general(by);
        (palace(by, target) && ADJACENT[general] & (1 << target) != 0).then_some((GENERAL, general))
    }
    /// Reference SEE whose attacker search scans every piece's targets in
    /// value order, for differential tests.
    #[cfg(test)]
    pub(crate) fn see_by_target_scan(&self, m: Move) -> i32 {
        const ORDER: [usize; 7] = [SOLDIER, ADVISOR, ELEPHANT, HORSE, CANNON, ROOK, GENERAL];
        let value = |kind: usize| if kind == GENERAL { 10_000 } else { VALUE[kind] };
        let mut b = self.scratch();
        let (from, to) = (m.from(), m.to());
        let (side, mut kind) = piece_parts(self.board[from]);
        let mut gain = vec![0i32];
        if self.board[to] != 0 {
            let (s, k) = piece_parts(self.board[to]);
            gain[0] = value(k);
            b.take(s, k, to);
        }
        b.take(side, kind, from);
        b.put(side, kind, to);
        let mut turn = side ^ 1;
        while let Some((next, at)) = ORDER.iter().find_map(|&k| {
            let mut pieces = b.pieces[turn][k];
            while pieces != 0 {
                let at = pieces.trailing_zeros() as usize;
                pieces &= pieces - 1;
                if b.targets(turn, k, at, true) & (1 << to) != 0 {
                    return Some((k, at));
                }
            }
            None
        }) {
            gain.push(value(kind) - gain[gain.len() - 1]);
            b.take(turn ^ 1, kind, to);
            b.take(turn, next, at);
            b.put(turn, next, to);
            kind = next;
            turn ^= 1;
        }
        for d in (1..gain.len()).rev() {
            gain[d - 1] = -(-gain[d - 1]).max(gain[d]);
        }
        gain[0]
    }
    fn scratch(&self) -> Self {
        Self {
            board: self.board,
            pieces: self.pieces,
            occupied: self.occupied,
            files: self.files,
            side: self.side,
            hash: self.hash,
            material: self.material,
            accumulated: self.accumulated,
            check: self.check,
            stack: Vec::new(),
            keys: Vec::new(),
            checks: Vec::new(),
            chases: Vec::new(),
            track_chases: false,
            root: 0,
            reversible: 0,
        }
    }
    /// All evaluation features, side to move minus opponent: static terms
    /// of every piece, the dynamic terms and the tempo.
    pub fn features(&self, sink: &mut impl FnMut(usize, i32)) {
        for side in 0..2 {
            let sign = if side == self.side { 1 } else { -1 };
            for kind in 0..7 {
                let mut pieces = self.pieces[side][kind];
                while pieces != 0 {
                    let at = pieces.trailing_zeros() as usize;
                    pieces &= pieces - 1;
                    Self::static_features(side, kind, at, &mut |i, v| sink(i, sign * v));
                }
            }
        }
        self.dynamic_features(self.side, true, sink);
        sink(super::eval::TEMPO, 1);
    }
    /// Chariots, Horses and Cannons of both sides.
    pub fn majors(&self) -> u8 {
        (0..2)
            .map(|s| {
                (self.pieces[s][ROOK] | self.pieces[s][HORSE] | self.pieces[s][CANNON]).count_ones()
            })
            .sum::<u32>() as u8
    }
    /// Per-node terms (mobility, piece value by phase, General safety),
    /// `side` minus the opponent.
    /// With `all` false, terms whose compiled weight is zero are skipped
    /// (the search evaluation); fitting needs every term.
    fn dynamic_features(&self, side: usize, all: bool, sink: &mut impl FnMut(usize, i32)) {
        use super::eval::*;
        // The compiled weights keep every per-node term at zero (MSH-196):
        // the search then skips this work entirely.
        const ACTIVE: bool = {
            let mut any = false;
            let mut i = MOBILITY_HORSE;
            while i < TEMPO {
                any |= DEFAULT[i] != 0;
                i += 1;
            }
            any
        };
        if !all && !ACTIVE {
            return;
        }
        let want = |index: usize| all || DEFAULT[index] != 0;
        let majors = i32::from(self.majors());
        let empty = ALL & !(self.occupied[0] | self.occupied[1]);
        for s in 0..2 {
            let sign = if s == side { 1 } else { -1 };
            let them = s ^ 1;
            let mut mobility = [0i32; 3];
            for (slot, kind) in [(0, HORSE), (1, ROOK), (2, CANNON)] {
                if !want(MOBILITY_HORSE + slot) {
                    continue;
                }
                let mut pieces = self.pieces[s][kind];
                while pieces != 0 {
                    let at = pieces.trailing_zeros() as usize;
                    pieces &= pieces - 1;
                    let reach = if kind == CANNON {
                        (self.targets(s, kind, at, false) & empty)
                            | (self.targets(s, kind, at, true) & self.occupied[them])
                    } else {
                        self.targets(s, kind, at, true) & !self.occupied[s]
                    };
                    mobility[slot] += reach.count_ones() as i32;
                }
            }
            sink(MOBILITY_HORSE, sign * mobility[0]);
            sink(MOBILITY_ROOK, sign * mobility[1]);
            sink(MOBILITY_CANNON, sign * mobility[2]);
            sink(
                HORSE_PHASE,
                sign * self.pieces[s][HORSE].count_ones() as i32 * majors,
            );
            sink(
                CANNON_PHASE,
                sign * self.pieces[s][CANNON].count_ones() as i32 * majors,
            );
            let general = self.general(s);
            let file = general % 9;
            let on_file = |bits: u128| {
                let mut count = 0;
                let mut b = bits;
                while b != 0 {
                    let at = b.trailing_zeros() as usize;
                    b &= b - 1;
                    count += i32::from(at % 9 == file);
                }
                count
            };
            sink(ENEMY_ROOK_FILE, sign * on_file(self.pieces[them][ROOK]));
            sink(ENEMY_CANNON_FILE, sign * on_file(self.pieces[them][CANNON]));
            let mut attacked = 0;
            for (dx, dy) in DIRECTIONS {
                if !want(GENERAL_ADJACENT_ATTACKED) {
                    break;
                }
                if let Some(at) = offset(general, dx, dy)
                    && palace(s, at)
                    && self.attacked(at, them)
                {
                    attacked += 1;
                }
            }
            sink(GENERAL_ADJACENT_ATTACKED, sign * attacked);
        }
    }
    // Fixed-size scratch state only: no history clone or allocation per node.
    fn has_legal_move(&self) -> bool {
        let mut scratch = Self {
            board: self.board,
            pieces: self.pieces,
            occupied: self.occupied,
            files: self.files,
            side: self.side,
            hash: self.hash,
            material: self.material,
            accumulated: self.accumulated,
            check: self.check,
            stack: Vec::new(),
            keys: Vec::new(),
            checks: Vec::new(),
            chases: Vec::new(),
            track_chases: false,
            root: 0,
            reversible: 0,
        };
        let side = self.side;
        let enemy = self.occupied[side ^ 1] & !self.pieces[side ^ 1][GENERAL];
        let empty = ALL & !(self.occupied[0] | self.occupied[1]);
        for kind in 0..7 {
            let mut pieces = self.pieces[side][kind];
            while pieces != 0 {
                let from = pieces.trailing_zeros() as usize;
                pieces &= pieces - 1;
                let mut targets = (self.targets(side, kind, from, true) & enemy)
                    | (self.targets(side, kind, from, false) & empty);
                while targets != 0 {
                    let to = targets.trailing_zeros() as usize;
                    targets &= targets - 1;
                    let victim = self.board[to];
                    scratch.take(side, kind, from);
                    if victim != 0 {
                        let (s, k) = piece_parts(victim);
                        scratch.take(s, k, to);
                    }
                    scratch.put(side, kind, to);
                    if !scratch.attacked(scratch.general(side), side ^ 1) {
                        return true;
                    }
                    scratch.take(side, kind, to);
                    scratch.put(side, kind, from);
                    if victim != 0 {
                        let (s, k) = piece_parts(victim);
                        scratch.put(s, k, to);
                    }
                }
            }
        }
        false
    }
    fn ply(&self) -> usize {
        self.keys.len() - 1 - self.root
    }
}
impl SearchPosition for XiangqiPosition {
    type Move = Move;
    const HISTORY_SLOTS: usize = 2 * 7 * N;
    const QUIESCENCE_CHECKS: usize = 8;
    const MATE_SEARCH: bool = true;
    fn hash(&self) -> u64 {
        self.hash
    }
    fn in_check(&self) -> bool {
        self.check
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<Move>) {
        let capture = stage == Stage::Tactical;
        let mask = if capture {
            self.occupied[self.side ^ 1] & !self.pieces[self.side ^ 1][GENERAL]
        } else {
            ALL & !(self.occupied[0] | self.occupied[1])
        };
        for kind in 0..7 {
            let mut pieces = self.pieces[self.side][kind];
            while pieces != 0 {
                let from = pieces.trailing_zeros() as usize;
                pieces &= pieces - 1;
                let mut targets = self.targets(self.side, kind, from, capture) & mask;
                while targets != 0 {
                    let to = targets.trailing_zeros() as usize;
                    targets &= targets - 1;
                    moves.push(Move::new(from, to));
                }
            }
        }
    }
    /// Superset of every legal checking move. A check needs one of: a
    /// Chariot or Cannon landing on the General's file or rank, any piece
    /// landing there (new Cannon screen) or leaving it (uncovered Chariot or
    /// Cannon line, removed second screen), a Horse landing a Horse move from
    /// the General, a Soldier landing next to it, or a piece leaving the leg
    /// square of a Horse aimed at the General.
    fn generate_checks(&self, moves: &mut Vec<Move>) {
        let side = self.side;
        let g = self.general(side ^ 1);
        let (gx, gy) = (g % 9, g / 9);
        let mut line: u128 = 0;
        for at in 0..N {
            if at != g && (at % 9 == gx || at / 9 == gy) {
                line |= 1 << at;
            }
        }
        let mut horse_squares: u128 = 0;
        let mut legs: u128 = 0;
        for (dx, dy) in [
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
            (-1, -2),
            (-2, -1),
        ] {
            if let Some(at) = offset(g, dx, dy) {
                horse_squares |= 1 << at;
                if self.board[at] == code(side, HORSE) {
                    // The leg is next to the Horse along its two-step axis.
                    let leg = offset(
                        at,
                        if dx.abs() == 2 { -dx / 2 } else { 0 },
                        if dy.abs() == 2 { -dy / 2 } else { 0 },
                    )
                    .expect("on board");
                    legs |= 1 << leg;
                }
            }
        }
        let mut adjacent: u128 = 0;
        for (dx, dy) in DIRECTIONS {
            if let Some(at) = offset(g, dx, dy) {
                adjacent |= 1 << at;
            }
        }
        let enemy = self.occupied[side ^ 1] & !self.pieces[side ^ 1][GENERAL];
        let empty = ALL & !(self.occupied[0] | self.occupied[1]);
        for kind in 0..7 {
            let mut pieces = self.pieces[side][kind];
            while pieces != 0 {
                let from = pieces.trailing_zeros() as usize;
                pieces &= pieces - 1;
                let all = (self.targets(side, kind, from, true) & enemy)
                    | (self.targets(side, kind, from, false) & empty);
                let wanted = if (line | legs) & (1 << from) != 0 {
                    all
                } else {
                    let mut w = line;
                    if kind == HORSE {
                        w |= horse_squares;
                    }
                    if kind == SOLDIER {
                        w |= adjacent;
                    }
                    all & w
                };
                let mut targets = wanted;
                while targets != 0 {
                    let to = targets.trailing_zeros() as usize;
                    targets &= targets - 1;
                    moves.push(Move::new(from, to));
                }
            }
        }
    }
    fn make(&mut self, m: Move) -> bool {
        // Every caller passes a pseudo-legal move: generated moves, killers
        // checked with `is_pseudo_legal`, table moves through `decode`.
        debug_assert!(self.is_pseudo_legal(m), "pseudo-legal move");
        let side = self.side;
        let (from, to) = (m.from(), m.to());
        let (_, kind) = piece_parts(self.board[from]);
        // Before the move: can it expose the own General, and can it check?
        let own = self.general(side);
        let needs_legality = kind == GENERAL
            || self.check
            || (LINE_MASK[own] | self.enemy_horse_legs(side, own)) & (1 << from) != 0
            || LINE_MASK[own] & (1 << to) != 0;
        let them = self.general(side ^ 1);
        let may_check = kind == GENERAL
            || (LINE_MASK[them] | self.own_horse_legs(side, them)) & (1 << from) != 0
            || LINE_MASK[them] & (1 << to) != 0
            || (kind == HORSE && HORSE_SQUARES[them] & (1 << to) != 0)
            || (kind == SOLDIER && ADJACENT[them] & (1 << to) != 0);
        let undo = Undo {
            m,
            captured: self.board[to],
            hash: self.hash,
            check: self.check,
            reversible: self.reversible,
        };
        self.reversible = self.reversible.saturating_add(1);
        if undo.captured != 0 {
            let (s, k) = piece_parts(undo.captured);
            self.take(s, k, to);
            self.reversible = 0;
        }
        self.take(side, kind, from);
        self.put(side, kind, to);
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.stack.push(undo);
        self.keys.push(self.hash);
        self.checks.push(false);
        self.chases.push(false);
        if needs_legality && self.attacked(self.general(side), side ^ 1) {
            self.unmake();
            return false;
        }
        self.check = may_check && self.attacked(self.general(side ^ 1), side);
        *self.checks.last_mut().unwrap() = self.check;
        let chase = self.track_chases && undo.captured == 0 && self.chases_after(side, kind, to);
        *self.chases.last_mut().unwrap() = chase;
        true
    }
    fn unmake(&mut self) {
        let undo = self.stack.pop().expect("balanced make/unmake");
        self.keys.pop();
        self.checks.pop();
        self.chases.pop();
        self.side ^= 1;
        let (from, to) = (undo.m.from(), undo.m.to());
        let (_, kind) = piece_parts(self.board[to]);
        self.take(self.side, kind, to);
        self.put(self.side, kind, from);
        if undo.captured != 0 {
            let (s, k) = piece_parts(undo.captured);
            self.put(s, k, to);
        }
        self.hash = undo.hash;
        self.check = undo.check;
        self.reversible = undo.reversible;
    }
    fn make_null(&mut self) {
        self.stack.push(Undo {
            m: Move::default(),
            captured: 0,
            hash: self.hash,
            check: self.check,
            reversible: self.reversible,
        });
        self.side ^= 1;
        self.hash ^= TABLES.side;
        self.keys.push(self.hash);
        self.checks.push(false);
        self.chases.push(false);
        self.check = false;
        self.reversible = 0;
    }
    fn unmake_null(&mut self) {
        let undo = self.stack.pop().expect("balanced null move");
        self.keys.pop();
        self.checks.pop();
        self.chases.pop();
        self.side ^= 1;
        self.hash = undo.hash;
        self.check = undo.check;
        self.reversible = undo.reversible;
    }
    fn null_move_safe(&self) -> bool {
        self.pieces[self.side][ROOK]
            | self.pieces[self.side][HORSE]
            | self.pieces[self.side][CANNON]
            != 0
    }
    fn rule_score(&self) -> Option<i32> {
        // Quiescence can reach an unchecked stalemate: Xiangqi has no draw
        // for stalemate. Detect it before repetition or bare-General draws.
        if !self.has_legal_move() {
            return Some(-MATE_SCORE + self.ply() as i32);
        }
        if (self.occupied[0] | self.occupied[1]).count_ones() == 2 {
            return Some(0);
        }

        let current = self.keys.len() - 1;
        let occurrences =
            earlier_occurrences(&self.keys[..current], self.hash, 2..=self.reversible);
        if let Some(index) = cycle_start(occurrences, self.root, 2) {
            // Approximate WXF conduct. A side whose every move in the
            // cycle checks loses; otherwise a side whose every move
            // checks or chases loses; both or neither: draw. Native
            // rules adjudicate the real game.
            let moves_of = |side: usize| {
                (index + 1..=current).filter(move |&i| self.side ^ ((current - i) & 1) ^ 1 == side)
            };
            let checker = [0, 1].map(|side| moves_of(side).all(|i| self.checks[i]));
            let loser = if checker[0] != checker[1] {
                Some(usize::from(checker[1]))
            } else {
                let chases = self.chase_flags(index + 1);
                let violator = [0, 1]
                    .map(|side| moves_of(side).all(|i| self.checks[i] || chases[i - index - 1]));
                (violator[0] != violator[1]).then(|| usize::from(violator[1]))
            };
            if let Some(side) = loser {
                return Some(if side == self.side {
                    -MATE_SCORE + self.ply() as i32
                } else {
                    MATE_SCORE - self.ply() as i32
                });
            }
            return Some(0);
        }
        None
    }
    fn draw_claimable(&self) -> bool {
        false
    }
    fn no_move_score(&self, ply: i32) -> i32 {
        -MATE_SCORE + ply
    }
    fn evaluate(&self) -> i32 {
        let side = self.side;
        let mut dynamic = 0;
        self.dynamic_features(side, false, &mut |index, value| {
            dynamic += super::eval::DEFAULT[index] * value
        });
        self.accumulated[side] - self.accumulated[side ^ 1]
            + dynamic
            + super::eval::DEFAULT[super::eval::TEMPO]
    }
    fn is_tactical(&self, m: Move) -> bool {
        self.board[m.to()] != 0
    }
    fn tactical_order(&self, m: Move) -> i32 {
        self.tactical_gain(m) * 16 - VALUE[piece_parts(self.board[m.from()]).1] / 16
    }
    fn tactical_gain(&self, m: Move) -> i32 {
        let p = self.board[m.to()];
        if p == 0 { 0 } else { VALUE[piece_parts(p).1] }
    }
    fn exchange_at_least(&self, m: Move, threshold: i32) -> bool {
        let gain = self.tactical_gain(m);
        if gain < threshold {
            return false;
        }
        let mover = VALUE[piece_parts(self.board[m.from()]).1];
        gain - mover >= threshold || self.see(m) >= threshold
    }
    fn history_slot(&self, m: Move) -> usize {
        (self.side * 7 + piece_parts(self.board[m.from()]).1) * N + m.to()
    }
    fn encode(m: Move) -> u32 {
        u32::from(m.0) + 1
    }
    fn decode(&self, bits: u32) -> Option<Move> {
        let m = Move(u16::try_from(bits.checked_sub(1)?).ok()?);
        self.is_pseudo_legal(m).then_some(m)
    }
    fn is_pseudo_legal(&self, m: Move) -> bool {
        let (from, to) = (m.from(), m.to());
        if from >= N || to >= N || from == to || self.board[from] == 0 {
            return false;
        }
        let (side, kind) = piece_parts(self.board[from]);
        if side != self.side {
            return false;
        }
        if self.board[to] != 0 {
            let (s, k) = piece_parts(self.board[to]);
            if s == side || k == GENERAL {
                return false;
            }
        }
        self.targets(side, kind, from, self.board[to] != 0) & (1 << to) != 0
    }
}
