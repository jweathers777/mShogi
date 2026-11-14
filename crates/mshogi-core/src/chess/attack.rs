//! Geometric chess attack queries for `mshogi.chess.position.v1`.
//!
//! An attack is decided by piece kind, color, board edges and current
//! occupancy only. It is not legal-move membership: pinned pieces and kings
//! still attack, a target occupied by the attacker's own piece is attacked
//! (defended), and no castling or en passant effect is considered.

use super::{
    Color, Piece, PieceKind, Position, Square,
    attack_cache::{DIRECTIONS, KINGS, KNIGHTS, PAWNS, RAYS, kind_index, nearest},
};

impl Position {
    pub(super) fn king_square(&self, side: Color) -> Square {
        let kings =
            self.attacks.kinds[kind_index(PieceKind::King)] & self.attacks.colors[side.index()];
        debug_assert_eq!(kings.count_ones(), 1);
        Square(kings.trailing_zeros() as u8)
    }
    /// Geometric squares attacked by the piece on `from`, as an a1-bit0 map.
    ///
    /// This has exactly the attack/defence semantics of [`Position::attackers`]:
    /// slider rays include their first occupied square and stop there, friendly
    /// blockers are defended, and pins do not remove attacks. Empty squares
    /// return zero. Castling, pawn pushes and en passant are not attack effects.
    /// The map is read-only position data, never permission to play an action.
    pub fn attacks_from(&self, from: Square) -> u64 {
        let Some(piece) = self.piece_at(from) else {
            return 0;
        };
        self.attacks_as(piece, from)
    }

    /// Geometric attack map for a hypothetical `piece` at `from`, using this
    /// board's blockers. The origin occupant is ignored; every ray includes its
    /// first occupied target. This never changes the board, resolves a move or
    /// establishes legal admission. It shares all geometry with `attacks_from`.
    pub fn attacks_as(&self, piece: Piece, from: Square) -> u64 {
        match piece.kind {
            PieceKind::Pawn => PAWNS[piece.color.index()][from.index()],
            PieceKind::Knight => KNIGHTS[from.index()],
            PieceKind::King => KINGS[from.index()],
            kind => {
                let mut attacked = 0;
                for (dir, &(df, dr)) in DIRECTIONS.iter().enumerate() {
                    let diagonal = df != 0 && dr != 0;
                    if kind == PieceKind::Bishop && !diagonal || kind == PieceKind::Rook && diagonal
                    {
                        continue;
                    }
                    let ray = RAYS[from.index()][dir];
                    let blockers = ray & self.attacks.occupied();
                    attacked |= if blockers == 0 {
                        ray
                    } else {
                        ray & !RAYS[nearest(blockers, dir)][dir]
                    };
                }
                attacked
            }
        }
    }

    /// Squares holding a piece of color `by` that attacks `target`, a1 to h8.
    ///
    /// Pawns attack the two forward diagonals for their color whether or not
    /// the target is occupied; they never attack straight ahead. Knights and
    /// kings attack their fixed offsets. Bishops, rooks and queens attack along
    /// their lines up to and including the first occupied square in each
    /// direction. Moves never wrap across a board edge. Whether the attacker is
    /// pinned, whose piece stands on `target`, and whether moving the attacker
    /// would be legal are all irrelevant. A piece never attacks its own square.
    ///
    /// ```
    /// use mshogi_core::chess::{Color, Position, Square};
    /// let position = Position::from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1")?;
    /// let a8 = Square::new(0, 7).unwrap();
    /// let attackers: Vec<Square> = position.attackers(a8, Color::White).collect();
    /// assert_eq!(attackers, [Square::new(0, 0).unwrap()]);
    /// assert!(!position.is_square_attacked(a8, Color::Black));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn attackers(&self, target: Square, by: Color) -> impl Iterator<Item = Square> + '_ {
        (0..64u8)
            .map(Square)
            .filter(move |&from| self.piece_attacks(from, target, by))
    }

    /// Whether any piece of color `by` attacks `target`; see [`Position::attackers`].
    ///
    /// For example, the side to move is in check exactly when its king's square
    /// is attacked by the opponent. This query alone does not decide legality.
    pub fn is_square_attacked(&self, target: Square, by: Color) -> bool {
        self.attacker_bits(target, by, true) != 0
    }

    pub(super) fn attacker_mask(&self, target: Square, by: Color) -> u64 {
        self.attacker_bits(target, by, false)
    }

    fn attacker_bits(&self, target: Square, by: Color, first_only: bool) -> u64 {
        // Cached occupancy is derived by the same native effect-cell setter.
        // Leapers are reverse maps; a slider is visible iff it is the nearest
        // occupied square on its ray. Pins and target ownership do not enter.
        let ours = self.attacks.colors[by.index()];
        let pieces = |kind| ours & self.attacks.kinds[kind_index(kind)];
        let mut found = (PAWNS[by.opponent().index()][target.index()] & pieces(PieceKind::Pawn))
            | (KNIGHTS[target.index()] & pieces(PieceKind::Knight))
            | (KINGS[target.index()] & pieces(PieceKind::King));
        if first_only && found != 0 {
            return found;
        }
        let queens = pieces(PieceKind::Queen);
        let diagonal = queens | pieces(PieceKind::Bishop);
        let straight = queens | pieces(PieceKind::Rook);
        for (dir, &(df, dr)) in DIRECTIONS.iter().enumerate() {
            let attackers = if df != 0 && dr != 0 {
                diagonal
            } else {
                straight
            };
            if attackers == 0 {
                continue;
            }
            let blockers = RAYS[target.index()][dir] & self.attacks.occupied();
            if blockers != 0 && attackers & (1 << nearest(blockers, dir)) != 0 {
                found |= 1 << nearest(blockers, dir);
                if first_only {
                    return found;
                }
            }
        }
        found
    }

    fn piece_attacks(&self, from: Square, target: Square, by: Color) -> bool {
        let Some(piece) = self.piece_at(from) else {
            return false;
        };
        if piece.color != by {
            return false;
        }
        let df = target.file() as i8 - from.file() as i8;
        let dr = target.rank() as i8 - from.rank() as i8;
        let diagonal = df != 0 && df.abs() == dr.abs();
        let orthogonal = (df == 0) != (dr == 0);
        match piece.kind {
            PieceKind::Pawn => df.abs() == 1 && dr == if by == Color::White { 1 } else { -1 },
            PieceKind::Knight => matches!((df.abs(), dr.abs()), (1, 2) | (2, 1)),
            PieceKind::King => df.abs().max(dr.abs()) == 1,
            PieceKind::Bishop => diagonal && self.ray_is_clear(from, target),
            PieceKind::Rook => orthogonal && self.ray_is_clear(from, target),
            PieceKind::Queen => (diagonal || orthogonal) && self.ray_is_clear(from, target),
        }
    }

    // Called only for aligned, distinct, bounded squares. Every intervening
    // coordinate lies on the segment between them and is therefore in 0..8.
    pub(super) fn ray_is_clear(&self, from: Square, target: Square) -> bool {
        let step_file = (target.file() as i8 - from.file() as i8).signum();
        let step_rank = (target.rank() as i8 - from.rank() as i8).signum();
        let mut file = from.file() as i8 + step_file;
        let mut rank = from.rank() as i8 + step_rank;
        while (file, rank) != (target.file() as i8, target.rank() as i8) {
            if self.board[rank as usize * 8 + file as usize].is_some() {
                return false;
            }
            file += step_file;
            rank += step_rank;
        }
        true
    }
}
