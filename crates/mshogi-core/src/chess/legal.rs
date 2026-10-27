//! Legal chess actions and position-level classification.
//!
//! This is the public legal-move list for `mshogi.chess.position.v1`. The
//! classification describes one position only; it is not match adjudication.

use std::fmt;

use super::{Action, Color, PieceKind, Position, Square};

/// Position-level classification of the side to move, per the profile's
/// `(in_check, has_legal_action)` rule. This is the limited M1 classifier,
/// not complete game adjudication.
///
/// `Quiet` and `Check` make no claim that a game may continue: dead positions,
/// repetition, the 50/75-move rules, agreed draws, claims, resignation and
/// clocks are outside this classification (see the coverage ledger).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Classification {
    /// Not in check, with at least one legal action.
    Quiet,
    /// In check, with at least one legal action.
    Check,
    /// In check, with no legal action.
    Checkmate,
    /// Not in check, with no legal action.
    Stalemate,
}

impl fmt::Display for Classification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Quiet => "quiet",
            Self::Check => "check",
            Self::Checkmate => "checkmate",
            Self::Stalemate => "stalemate",
        })
    }
}

impl Position {
    /// Read-only local movement inspection for a hypothetical turn by `side`.
    /// This is educational position data, not permission to commit a move or a
    /// game/history conclusion. All rights stay on a private copy; en passant
    /// is available only to the actual entitled side. The original is unchanged.
    pub fn inspection_actions(&self, side: Color) -> Vec<Action> {
        let mut copy = self.clone();
        if side != self.side_to_move {
            copy.en_passant_target = None;
        }
        copy.side_to_move = side;
        copy.legal_actions()
    }

    /// Every legal action for the side to move: ordinary moves and captures,
    /// each promotion choice, castling and en passant, after king safety.
    ///
    /// Legality follows the profile's rules and does **not** depend on the u32
    /// counter domain. At the counter limit an action in this list can still be
    /// rejected by [`Position::apply`] with
    /// [`ActionError::CounterOverflow`](super::ActionError::CounterOverflow),
    /// atomically; every other legal action is accepted by `apply`, and `apply`
    /// accepts nothing outside this list. Order is deterministic but
    /// unspecified and carries no search meaning.
    ///
    /// ```
    /// use mshogi_core::chess::{Classification, Position};
    /// let position = Position::from_fen("7k/8/8/8/8/8/5PPP/r5K1 w - - 0 1")?;
    /// assert!(position.legal_actions().is_empty());
    /// assert_eq!(position.classify(), Classification::Checkmate);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn legal_actions(&self) -> Vec<Action> {
        // Counters never affect rule legality; normalize them on a private
        // clone so that only representation overflow is excluded from `apply`.
        // Sound while resolution reads counters only for overflow and structure
        // validation reads halfmove only for a post-state EP target (which a
        // double push always pairs with a reset). Revisit if that changes.
        let mut base = self.clone();
        base.halfmove_clock = 0;
        base.fullmove_number = 1;
        base.candidates()
            .into_iter()
            .filter(|&action| base.clone().apply(action).is_ok())
            .collect()
    }

    /// Whether the side to move's king is attacked by the opponent.
    pub fn in_check(&self) -> bool {
        let side = self.side_to_move;
        let king = (0..64u8).map(Square).find(|&square| {
            self.piece_at(square)
                .is_some_and(|piece| piece.color == side && piece.kind == PieceKind::King)
        });
        // Validated positions always contain exactly one king of each color.
        king.is_some_and(|square| self.is_square_attacked(square, side.opponent()))
    }

    /// Classify the side to move as quiet, check, checkmate or stalemate.
    ///
    /// The limited M1 position-level classifier: this is not a game result, a
    /// draw claim or a dead-position finding. For example bare kings are `Quiet`
    /// although complete chess rules would end that game. Counter values never
    /// change the result.
    pub fn classify(&self) -> Classification {
        match (self.in_check(), !self.legal_actions().is_empty()) {
            (false, true) => Classification::Quiet,
            (true, true) => Classification::Check,
            (true, false) => Classification::Checkmate,
            (false, false) => Classification::Stalemate,
        }
    }
}

#[cfg(test)]
mod inspection_tests {
    use super::*;
    fn actions(position: &Position, side: Color, origin: &str) -> Vec<String> {
        let mut moves = position
            .inspection_actions(side)
            .into_iter()
            .filter(|a| a.from.to_string() == origin)
            .map(|a| a.to_string())
            .collect::<Vec<_>>();
        moves.sort();
        moves
    }
    #[test]
    fn hypothetical_knight_and_pin_are_local_and_immutable() {
        let p = Position::from_fen("4k3/4r1n1/8/8/8/8/8/K3R3 w - - 37 18").unwrap();
        let before = p.clone();
        assert_eq!(actions(&p, Color::Black, "g7"), ["g7e6", "g7f5", "g7h5"]);
        let rook = actions(&p, Color::Black, "e7");
        assert!(rook.contains(&"e7e1".into()));
        assert!(!rook.contains(&"e7d7".into()));
        assert_eq!(p, before);
    }
    #[test]
    fn inspection_preserves_actual_en_passant_without_transferring_it() {
        let p = Position::from_fen("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9").unwrap();
        let before = p.clone();
        assert!(actions(&p, Color::White, "e5").contains(&"e5d6".into()));
        assert!(actions(&p, Color::Black, "d5").contains(&"d5d4".into()));
        assert!(!actions(&p, Color::Black, "d5").contains(&"d5e4".into()));
        assert_eq!(p, before);
    }
    #[test]
    fn hypothetical_promotion_contains_all_four_completions() {
        let p = Position::from_fen("4k3/8/8/8/8/8/p7/4K3 w - - 0 1").unwrap();
        assert_eq!(
            actions(&p, Color::Black, "a2"),
            ["a2a1b", "a2a1n", "a2a1q", "a2a1r"]
        );
    }
    #[test]
    fn hypothetical_castling_uses_existing_rights_without_mutation() {
        let p = Position::from_fen("r3k2r/8/8/8/8/8/8/4K3 w kq - 12 24").unwrap();
        let before = p.clone();
        let choices = actions(&p, Color::Black, "e8");
        assert!(choices.contains(&"e8c8".into()));
        assert!(choices.contains(&"e8g8".into()));
        assert_eq!(p, before);
    }
}
