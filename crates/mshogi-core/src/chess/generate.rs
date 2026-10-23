//! Pseudo-legal chess candidates; not a legal-move list.

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
        self.push_ordinary_candidates(&mut candidates);
        candidates
    }

    /// Pseudo-legal castling and en passant candidates. **Not legal moves.**
    ///
    /// Castling appears when the side holds the right, its home rook is in
    /// place and every square between king and rook is empty. Whether the king
    /// is in check, crosses an attacked square or lands on one is **not**
    /// checked. En passant appears for each own pawn diagonally behind the raw
    /// target, whether or not the capture would expose its own king.
    ///
    /// Every special candidate is either accepted by [`Position::apply`] or
    /// rejected by it only with
    /// [`ActionError::CastlingKingAttacked`](super::ActionError::CastlingKingAttacked),
    /// [`ActionError::IllegalPosition`](super::ActionError::IllegalPosition) or
    /// [`ActionError::CounterOverflow`](super::ActionError::CounterOverflow).
    pub fn special_candidates(&self) -> Vec<Action> {
        let mut candidates = Vec::new();
        self.push_castling_candidates(&mut candidates);
        self.push_en_passant_candidates(&mut candidates);
        candidates
    }

    /// Every pseudo-legal candidate: [`Position::ordinary_candidates`] followed
    /// by [`Position::special_candidates`]. **Not legal moves.**
    ///
    /// [`Position::apply`] accepts no action outside this set and rejects a
    /// candidate only with `CastlingKingAttacked`, `IllegalPosition` or
    /// `CounterOverflow`. Filtering by `apply` therefore yields exactly the
    /// profile-legal actions (the u32 counter domain can exclude a FIDE-legal
    /// action at the limit), but this method itself makes no legality claim.
    pub fn candidates(&self) -> Vec<Action> {
        let mut candidates = self.ordinary_candidates();
        candidates.extend(self.special_candidates());
        candidates
    }

    fn push_castling_candidates(&self, out: &mut Vec<Action>) {
        let color = self.side_to_move;
        let rank = if color == Color::White { 0 } else { 7 };
        let at = |file: u8| Square(rank * 8 + file);
        let home_rook = Some(super::Piece {
            color,
            kind: PieceKind::Rook,
        });
        for (has_right, rook_file, king_to, between) in [
            (self.castling_rights.kingside(color), 7, 6, &[5u8, 6][..]),
            (self.castling_rights.queenside(color), 0, 2, &[1, 2, 3][..]),
        ] {
            if has_right
                && self.piece_at(at(rook_file)) == home_rook
                && between
                    .iter()
                    .all(|&file| self.piece_at(at(file)).is_none())
            {
                out.push(plain(at(4), at(king_to)));
            }
        }
    }

    fn push_en_passant_candidates(&self, out: &mut Vec<Action>) {
        let Some(target) = self.en_passant_target else {
            return;
        };
        let back = if self.side_to_move == Color::White {
            -1
        } else {
            1
        };
        let own_pawn = Some(super::Piece {
            color: self.side_to_move,
            kind: PieceKind::Pawn,
        });
        for df in [-1, 1] {
            if let Some(from) = offset(target, df, back)
                && self.piece_at(from) == own_pawn
            {
                out.push(plain(from, target));
            }
        }
    }

    fn push_ordinary_candidates(&self, candidates: &mut Vec<Action>) {
        for index in 0..64u8 {
            let from = Square(index);
            let Some(piece) = self.piece_at(from) else {
                continue;
            };
            if piece.color != self.side_to_move {
                continue;
            }
            match piece.kind {
                PieceKind::Pawn => self.pawn_candidates(from, candidates),
                PieceKind::Knight => self.step_candidates(from, &KNIGHT_OFFSETS, candidates),
                PieceKind::King => self.step_candidates(from, &KING_OFFSETS, candidates),
                PieceKind::Bishop => self.ray_candidates(from, &DIAGONALS, candidates),
                PieceKind::Rook => self.ray_candidates(from, &ORTHOGONALS, candidates),
                PieceKind::Queen => {
                    self.ray_candidates(from, &DIAGONALS, candidates);
                    self.ray_candidates(from, &ORTHOGONALS, candidates);
                }
            }
        }
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
