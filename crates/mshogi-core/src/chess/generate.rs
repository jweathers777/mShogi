//! Pseudo-legal chess candidates; not a legal-move list.

use std::ops::ControlFlow;

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
pub(super) const PROMOTIONS: [Promotion; 4] = [
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
        let _ = self.visit_ordinary_candidates(&mut |action| {
            candidates.push(action);
            ControlFlow::Continue(())
        });
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
        let _ = self.visit_special_candidates(&mut |action| {
            candidates.push(action);
            ControlFlow::Continue(())
        });
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
        let mut candidates = Vec::new();
        let _ = self.visit_candidates(&mut |action| {
            candidates.push(action);
            ControlFlow::Continue(())
        });
        candidates
    }

    // The one candidate traversal behind both complete lists and early-exit
    // native queries. A break propagates through every piece and phase.
    pub(super) fn visit_candidates(
        &self,
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        self.visit_ordinary_candidates(visit)?;
        self.visit_special_candidates(visit)
    }

    fn visit_special_candidates(
        &self,
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        self.visit_castling_candidates(visit)?;
        self.visit_en_passant_candidates(visit)
    }

    fn visit_castling_candidates(
        &self,
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
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
                visit(plain(at(4), at(king_to)))?;
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_en_passant_candidates(
        &self,
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let Some(target) = self.en_passant_target else {
            return ControlFlow::Continue(());
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
                visit(plain(from, target))?;
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_ordinary_candidates(
        &self,
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        for index in 0..64u8 {
            let from = Square(index);
            let Some(piece) = self.piece_at(from) else {
                continue;
            };
            if piece.color != self.side_to_move {
                continue;
            }
            match piece.kind {
                PieceKind::Pawn => self.visit_pawn_candidates(from, visit),
                PieceKind::Knight => self.visit_step_candidates(from, &KNIGHT_OFFSETS, visit),
                PieceKind::King => self.visit_step_candidates(from, &KING_OFFSETS, visit),
                PieceKind::Bishop => self.visit_ray_candidates(from, &DIAGONALS, visit),
                PieceKind::Rook => self.visit_ray_candidates(from, &ORTHOGONALS, visit),
                PieceKind::Queen => {
                    self.visit_ray_candidates(from, &DIAGONALS, visit)?;
                    self.visit_ray_candidates(from, &ORTHOGONALS, visit)
                }
            }?;
        }
        ControlFlow::Continue(())
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

    fn visit_step_candidates(
        &self,
        from: Square,
        offsets: &[(i8, i8)],
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        for &(df, dr) in offsets {
            if let Some(to) = offset(from, df, dr)
                && self.may_land_on(to)
            {
                visit(plain(from, to))?;
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_ray_candidates(
        &self,
        from: Square,
        directions: &[(i8, i8)],
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        for &(df, dr) in directions {
            let mut current = from;
            while let Some(to) = offset(current, df, dr) {
                if self.may_land_on(to) {
                    visit(plain(from, to))?;
                }
                if self.piece_at(to).is_some() {
                    break;
                }
                current = to;
            }
        }
        ControlFlow::Continue(())
    }

    fn visit_pawn_candidates(
        &self,
        from: Square,
        visit: &mut impl FnMut(Action) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let (forward, start_rank, last_rank) = match self.side_to_move {
            Color::White => (1, 1, 7),
            Color::Black => (-1, 6, 0),
        };
        let mut push = |to: Square| {
            if to.rank() == last_rank {
                for promotion in PROMOTIONS {
                    visit(Action {
                        from,
                        to,
                        promotion: Some(promotion),
                    })?;
                }
            } else {
                visit(plain(from, to))?;
            }
            ControlFlow::Continue(())
        };
        if let Some(one) = offset(from, 0, forward)
            && self.piece_at(one).is_none()
        {
            push(one)?;
            if from.rank() == start_rank
                && let Some(two) = offset(one, 0, forward)
                && self.piece_at(two).is_none()
            {
                push(two)?;
            }
        }
        for df in [-1, 1] {
            // Captures need an opponent piece; an empty raw EP target is not one.
            if let Some(to) = offset(from, df, forward)
                && self.piece_at(to).is_some()
                && self.may_land_on(to)
            {
                push(to)?;
            }
        }
        ControlFlow::Continue(())
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

#[cfg(test)]
mod visitor_tests {
    use super::*;

    fn texts(actions: Vec<Action>) -> Vec<String> {
        actions.into_iter().map(|a| a.to_string()).collect()
    }

    fn visited_until(p: &Position, stop: &str) -> Vec<String> {
        let mut visited = Vec::new();
        let result = p.visit_candidates(&mut |action| {
            let text = action.to_string();
            assert!(
                visited.last().is_none_or(|last| last != stop),
                "visited after break"
            );
            let end = text == stop;
            visited.push(text);
            if end {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        assert!(result.is_break());
        visited
    }

    #[test]
    fn candidate_visitor_breaks_inside_rays_and_promotion_choices() {
        let p = Position::from_fen("7k/8/8/8/8/8/8/R6K w - - 0 1").unwrap();
        let before = p.clone();
        assert_eq!(visited_until(&p, "a1a3"), ["a1a2", "a1a3"]);
        assert_eq!(p, before);
        // Queen traversal invokes diagonal rays before a separate orthogonal
        // phase. A break inside the first phase must skip the second entirely.
        let p = Position::from_fen("8/7k/8/8/8/8/8/Q6K w - - 0 1").unwrap();
        let before = p.clone();
        assert_eq!(visited_until(&p, "a1c3"), ["a1b2", "a1c3"]);
        assert_eq!(p, before);
        let p = Position::from_fen("2r1k3/1P6/8/8/8/8/8/K7 w - - 0 1").unwrap();
        let before = p.clone();
        assert_eq!(
            visited_until(&p, "b7b8r"),
            ["a1a2", "a1b2", "a1b1", "b7b8q", "b7b8r"]
        );
        assert_eq!(p, before);
    }

    #[test]
    fn candidate_visitor_break_propagates_between_ordinary_castling_and_ep_phases() {
        let p = Position::from_fen("4k3/8/8/2PpP3/8/8/8/R3K2R w KQ d6 0 9").unwrap();
        let before = p.clone();
        let ordinary = texts(p.ordinary_candidates());
        assert_eq!(ordinary.last().map(String::as_str), Some("e5e6"));
        assert_eq!(visited_until(&p, "e5e6"), ordinary);
        let mut expected = ordinary;
        expected.push("e1g1".into());
        assert_eq!(visited_until(&p, "e1g1"), expected);
        expected.extend(["e1c1".into(), "c5d6".into()]);
        assert_eq!(visited_until(&p, "c5d6"), expected);
        expected.push("e5d6".into());
        let mut complete = Vec::new();
        let result = p.visit_candidates(&mut |action| {
            complete.push(action.to_string());
            ControlFlow::Continue(())
        });
        assert!(result.is_continue());
        assert_eq!(complete, expected);
        assert_eq!(p, before);
    }

    #[test]
    fn prechange_ordered_candidate_snapshots() {
        // Recorded against the pre-visitor generator. These are compatibility
        // snapshots, not substitutes for the independent legal-set fixtures.
        for (fen, expected) in [
            (
                super::super::STANDARD_START_FEN,
                "b1c3 b1a3 g1h3 g1f3 a2a3 a2a4 b2b3 b2b4 c2c3 c2c4 d2d3 d2d4 e2e3 e2e4 f2f3 f2f4 g2g3 g2g4 h2h3 h2h4",
            ),
            (
                "2r1k3/1P6/8/8/8/8/8/K7 w - - 0 1",
                "a1a2 a1b2 a1b1 b7b8q b7b8r b7b8b b7b8n b7c8q b7c8r b7c8b b7c8n",
            ),
            (
                "7k/8/8/8/8/P7/8/R1b4K w - - 0 1",
                "a1a2 a1b1 a1c1 h1h2 h1g1 h1g2 a3a4",
            ),
        ] {
            let p = Position::from_fen(fen).unwrap();
            let before = p.clone();
            assert_eq!(
                texts(p.ordinary_candidates()),
                expected.split_whitespace().collect::<Vec<_>>()
            );
            assert_eq!(p, before);
        }
        let p = Position::from_fen("4k3/8/8/2PpP3/8/8/8/R3K2R w KQ d6 0 9").unwrap();
        assert_eq!(
            texts(p.special_candidates()),
            ["e1g1", "e1c1", "c5d6", "e5d6"]
        );
        let mut expected = p.ordinary_candidates();
        expected.extend(p.special_candidates());
        assert_eq!(p.candidates(), expected);
    }
}
