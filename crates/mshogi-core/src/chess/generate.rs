//! Pseudo-legal ordinary chess candidates; not a legal-move list.

use super::{Action, Color, PieceKind, Position, Promotion, Square};

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
    (-1, 2),
];
const KING_OFFSETS: [(i8, i8); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];
const DIAGONALS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
const ORTHOGONALS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const PROMOTIONS: [Promotion; 4] = [
    Promotion::Queen,
    Promotion::Rook,
    Promotion::Bishop,
    Promotion::Knight,
];

impl Position {
    /// Pseudo-legal ordinary candidates for the side to move. **Not legal moves.**
    ///
    /// Each candidate is an ordinary piece move or capture, a pawn single or
    /// double advance, a pawn capture, or one of the four promotion choices,
    /// each a distinct [`Action`]. Movement shape, own-piece blocking, the
    /// destination's occupancy and promotion are already correct. Castling and
    /// en passant are never included. A king is never a capture target.
    ///
    /// Candidates are **not** filtered for the moving side's king safety or for
    /// counter overflow: a pinned piece's move, a king step onto an attacked
    /// square and a move that ignores check are all included. Every candidate
    /// is either accepted by [`Position::apply_ordinary`] or rejected by it
    /// only with [`ActionError::IllegalPosition`](super::ActionError::IllegalPosition)
    /// or [`ActionError::CounterOverflow`](super::ActionError::CounterOverflow);
    /// `apply_ordinary` accepts no action outside this set. Order is
    /// deterministic but unspecified and carries no search meaning.
    ///
    /// ```
    /// use mshogi_core::chess::Position;
    /// let position = Position::from_fen(
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// )?;
    /// assert_eq!(position.ordinary_candidates().len(), 20);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn ordinary_candidates(&self) -> Vec<Action> {
        let mut candidates = Vec::new();
        for index in 0..64u8 {
            let from = Square(index);
            let Some(piece) = self.piece_at(from) else {
                continue;
            };
            if piece.color != self.side_to_move {
                continue;
            }
            match piece.kind {
                PieceKind::Pawn => self.pawn_candidates(from, &mut candidates),
                PieceKind::Knight => self.step_candidates(from, &KNIGHT_OFFSETS, &mut candidates),
                PieceKind::King => self.step_candidates(from, &KING_OFFSETS, &mut candidates),
                PieceKind::Bishop => self.ray_candidates(from, &DIAGONALS, &mut candidates),
                PieceKind::Rook => self.ray_candidates(from, &ORTHOGONALS, &mut candidates),
                PieceKind::Queen => {
                    self.ray_candidates(from, &DIAGONALS, &mut candidates);
                    self.ray_candidates(from, &ORTHOGONALS, &mut candidates);
                }
            }
        }
        candidates
    }

    // An empty square or an opponent non-king piece may be the destination. In a
    // valid position the side to move never attacks the opposing king, so the
    // king exclusion is a defensive guard that fixtures cannot reach.
    fn may_land_on(&self, to: Square) -> bool {
        match self.piece_at(to) {
            None => true,
            Some(piece) => piece.color != self.side_to_move && piece.kind != PieceKind::King,
        }
    }

    fn step_candidates(&self, from: Square, offsets: &[(i8, i8)], out: &mut Vec<Action>) {
        for &(df, dr) in offsets {
            if let Some(to) = offset(from, df, dr)
                && self.may_land_on(to)
            {
                out.push(plain(from, to));
            }
        }
    }

    fn ray_candidates(&self, from: Square, directions: &[(i8, i8)], out: &mut Vec<Action>) {
        for &(df, dr) in directions {
            let mut current = from;
            while let Some(to) = offset(current, df, dr) {
                if self.may_land_on(to) {
                    out.push(plain(from, to));
                }
                if self.piece_at(to).is_some() {
                    break;
                }
                current = to;
            }
        }
    }

    fn pawn_candidates(&self, from: Square, out: &mut Vec<Action>) {
        let (forward, start_rank, last_rank) = match self.side_to_move {
            Color::White => (1, 1, 7),
            Color::Black => (-1, 6, 0),
        };
        let mut push = |to: Square| {
            if to.rank() == last_rank {
                for promotion in PROMOTIONS {
                    out.push(Action {
                        from,
                        to,
                        promotion: Some(promotion),
                    });
                }
            } else {
                out.push(plain(from, to));
            }
        };
        if let Some(one) = offset(from, 0, forward)
            && self.piece_at(one).is_none()
        {
            push(one);
            if from.rank() == start_rank
                && let Some(two) = offset(one, 0, forward)
                && self.piece_at(two).is_none()
            {
                push(two);
            }
        }
        for df in [-1, 1] {
            // Captures need an opponent piece; an empty raw EP target is not one.
            if let Some(to) = offset(from, df, forward)
                && self.piece_at(to).is_some()
                && self.may_land_on(to)
            {
                push(to);
            }
        }
    }
}

fn offset(from: Square, df: i8, dr: i8) -> Option<Square> {
    let file = u8::try_from(from.file() as i8 + df).ok()?;
    let rank = u8::try_from(from.rank() as i8 + dr).ok()?;
    Square::new(file, rank)
}

const fn plain(from: Square, to: Square) -> Action {
    Action {
        from,
        to,
        promotion: None,
    }
}
