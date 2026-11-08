//! Derived occupancy maps and compile-time rays for native Chess geometry.
use super::{Piece, PieceKind};

pub(super) const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const fn rays() -> [[u64; 8]; 64] {
    let mut maps = [[0; 8]; 64];
    let mut sq = 0;
    while sq < 64 {
        let mut dir = 0;
        while dir < 8 {
            let (df, dr) = DIRECTIONS[dir];
            let mut file = (sq % 8) as i8 + df;
            let mut rank = (sq / 8) as i8 + dr;
            while file >= 0 && file < 8 && rank >= 0 && rank < 8 {
                maps[sq][dir] |= 1 << (rank as usize * 8 + file as usize);
                file += df;
                rank += dr;
            }
            dir += 1;
        }
        sq += 1;
    }
    maps
}
const fn leaps(offsets: &[(i8, i8)]) -> [u64; 64] {
    let mut maps = [0; 64];
    let mut sq = 0;
    while sq < 64 {
        let mut offset = 0;
        while offset < offsets.len() {
            let file = (sq % 8) as i8 + offsets[offset].0;
            let rank = (sq / 8) as i8 + offsets[offset].1;
            if file >= 0 && file < 8 && rank >= 0 && rank < 8 {
                maps[sq] |= 1 << (rank as usize * 8 + file as usize);
            }
            offset += 1;
        }
        sq += 1;
    }
    maps
}
pub(super) const RAYS: [[u64; 8]; 64] = rays();
pub(super) const KNIGHTS: [u64; 64] = leaps(&[
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
]);
pub(super) const KINGS: [u64; 64] = leaps(&DIRECTIONS);
pub(super) const PAWNS: [[u64; 64]; 2] = [leaps(&[(-1, 1), (1, 1)]), leaps(&[(-1, -1), (1, -1)])];

pub(super) const fn kind_index(kind: PieceKind) -> usize {
    match kind {
        PieceKind::Pawn => 0,
        PieceKind::Knight => 1,
        PieceKind::Bishop => 2,
        PieceKind::Rook => 3,
        PieceKind::Queen => 4,
        PieceKind::King => 5,
    }
}
pub(super) fn nearest(blockers: u64, dir: usize) -> usize {
    debug_assert_ne!(blockers, 0);
    let (df, dr) = DIRECTIONS[dir];
    if dr > 0 || dr == 0 && df > 0 {
        blockers.trailing_zeros() as usize
    } else {
        63 - blockers.leading_zeros() as usize
    }
}

/// Redundant, derived data; canonical state and all native effects remain owned
/// by Position. Only construction and its one cell setter may update this cache.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct AttackCache {
    pub colors: [u64; 2],
    pub kinds: [u64; 6],
}
impl AttackCache {
    pub fn from_board(board: &[Option<Piece>; 64]) -> Self {
        let mut cache = Self::default();
        for (sq, &piece) in board.iter().enumerate() {
            cache.insert(sq, piece);
        }
        cache
    }
    pub fn insert(&mut self, sq: usize, piece: Option<Piece>) {
        if let Some(piece) = piece {
            self.colors[piece.color.index()] |= 1 << sq;
            self.kinds[kind_index(piece.kind)] |= 1 << sq;
        }
    }
    pub fn remove(&mut self, sq: usize, piece: Option<Piece>) {
        if let Some(piece) = piece {
            self.colors[piece.color.index()] &= !(1 << sq);
            self.kinds[kind_index(piece.kind)] &= !(1 << sq);
        }
    }
    pub fn occupied(&self) -> u64 {
        self.colors[0] | self.colors[1]
    }
}
