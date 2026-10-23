//! Legal chess actions and position-level classification.
//!
//! This is the public legal-move list for `mshogi.chess.position.v1`. The
//! classification describes one position only; it is not match adjudication.

use std::fmt;

use super::{Action, PieceKind, Position, Square};

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
