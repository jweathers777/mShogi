//! Rules-owned action history. Adjudication is added by MSH-17/18.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use super::{Action, ActionError, Color, FenError, PieceKind, Position, Square};

/// Standard initial chess state, shared by the game constructor and adapters.
pub const STANDARD_START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// What is known before the first recorded position. Never inferred from FEN.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryOrigin {
    /// The exact orthodox start, before any action.
    StandardStart,
    /// An imported FEN; earlier repetitions are unknown, even at fullmove one.
    ImportedFen,
}

/// Claim handling selected by the consumer, recorded without executing claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPolicy {
    /// Valid claims remain optional player commands.
    ExplicitOnly,
    /// The unattended controller exercises valid claims at the turn boundary.
    AutoClaim,
}

/// Exact repetition equivalence, not saved-position equality or a search key.
///
/// Counters and uncapturable raw EP are excluded; all castling permissions stay.
/// Hashing is only an index: the entire identity is compared on collisions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepetitionIdentity {
    // Lossless canonical34bytes: two piece codes per byte, side/rights flags,
    // legal EP (64=None). This is the identity itself, not a probabilistic hash
    // or public saved-state encoding. Full equality checks every encoded field.
    packed: [u8; 34],
}
impl Hash for RepetitionIdentity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.packed);
    }
}
impl RepetitionIdentity {
    fn from_position(position: &Position, legal_ep: Option<Square>) -> Self {
        let mut bytes = [0u8; 34];
        for (index, occupant) in position.board.iter().enumerate() {
            let code = occupant.map_or(0, |piece| {
                let kind = match piece.kind {
                    PieceKind::Pawn => 1,
                    PieceKind::Knight => 2,
                    PieceKind::Bishop => 3,
                    PieceKind::Rook => 4,
                    PieceKind::Queen => 5,
                    PieceKind::King => 6,
                };
                kind + if piece.color == Color::Black { 6 } else { 0 }
            });
            bytes[index / 2] |= code << ((index % 2) * 4);
        }
        bytes[32] = u8::from(position.side_to_move == Color::Black)
            | (u8::from(position.castling_rights.kingside(Color::White)) << 1)
            | (u8::from(position.castling_rights.queenside(Color::White)) << 2)
            | (u8::from(position.castling_rights.kingside(Color::Black)) << 3)
            | (u8::from(position.castling_rights.queenside(Color::Black)) << 4);
        bytes[33] = legal_ep.map_or(64, |square| square.0);
        Self { packed: bytes }
    }
}

impl Position {
    /// Rule-equivalent identity using **legal**, counter-independent en passant.
    pub fn repetition_identity(&self) -> RepetitionIdentity {
        let legal_ep = self
            .en_passant_target
            .filter(|&target| !self.legal_captures_to(target).is_empty());
        RepetitionIdentity::from_position(self, legal_ep)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Step {
    action: Action,
    before: Position,
    previous_identity: RepetitionIdentity,
    previous_reversible_repeat: bool,
}

/// Current position, known action history and exact repetition occurrence table.
///
/// Mutable position access is deliberately absent. Apply/undo owns every affected
/// history field. This foundation records claim policy but does not yet conclude
/// games or execute match commands (MSH-17/18). Clone supports private analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Game {
    initial: Position,
    origin: HistoryOrigin,
    policy: ClaimPolicy,
    position: Position,
    steps: Vec<Step>,
    current_identity: RepetitionIdentity,
    occurrences: HashMap<RepetitionIdentity, usize>,
    // Exact prerequisite for a current/intended repetition claim. The public
    // occurrence table remains complete, including unreachable old identities.
    reversible_repeat: bool,
}

impl Game {
    /// Begin at the known standard start, counted as occurrence one.
    pub fn new(policy: ClaimPolicy) -> Self {
        Self::from_initial(
            Position::from_fen(STANDARD_START_FEN).expect("valid standard start"),
            HistoryOrigin::StandardStart,
            policy,
        )
    }

    /// Import a FEN with its counters intact and unknown earlier history.
    pub fn from_fen(fen: &str, policy: ClaimPolicy) -> Result<Self, FenError> {
        Ok(Self::from_initial(
            Position::from_fen(fen)?,
            HistoryOrigin::ImportedFen,
            policy,
        ))
    }

    pub(super) fn from_initial(
        initial: Position,
        origin: HistoryOrigin,
        policy: ClaimPolicy,
    ) -> Self {
        let key = initial.repetition_identity();
        Self {
            position: initial.clone(),
            initial,
            origin,
            policy,
            steps: Vec::new(),
            current_identity: key.clone(),
            occurrences: HashMap::from([(key, 1)]),
            reversible_repeat: false,
        }
    }

    /// Immutable current position; adapters cannot bypass history bookkeeping.
    pub const fn position(&self) -> &Position {
        &self.position
    }

    /// Exact initial position, including imported raw EP and counters.
    pub const fn initial_position(&self) -> &Position {
        &self.initial
    }

    /// Known standard start or imported FEN with unknown preceding history.
    pub const fn history_origin(&self) -> HistoryOrigin {
        self.origin
    }

    /// Selected consumer policy; reconstruction never executes it implicitly.
    pub const fn claim_policy(&self) -> ClaimPolicy {
        self.policy
    }

    /// Recorded action count, not a unique version or search request identity.
    pub fn ply_count(&self) -> usize {
        self.steps.len()
    }

    /// Complete player actions in order, with no client-authored effects/undo.
    pub fn actions(&self) -> impl ExactSizeIterator<Item = Action> + '_ {
        self.steps.iter().map(|step| step.action)
    }

    /// Exact current repetition identity.
    pub const fn repetition_identity(&self) -> &RepetitionIdentity {
        &self.current_identity
    }

    /// Number of occurrences in known history only, including the initial state.
    pub fn occurrences(&self, identity: &RepetitionIdentity) -> usize {
        self.occurrences.get(identity).copied().unwrap_or(0)
    }

    /// Occurrences of the current rule-equivalent position in known history.
    pub fn current_occurrences(&self) -> usize {
        self.occurrences(&self.current_identity)
    }

    // Necessary, not sufficient, for any current or intended threefold claim.
    // Only repetitions since the last irreversible action can recur.
    pub(super) fn has_repeated_identity(&self) -> bool {
        self.reversible_repeat
    }

    /// Check one action and atomically update position, history and occurrences.
    /// Failure, including counter overflow, leaves the entire game unchanged.
    pub fn apply(&mut self, action: Action) -> Result<(), ActionError> {
        let after = self.position.checked_after(action)?;
        self.commit_after(action, after);
        Ok(())
    }

    // Only native checked resolution/prepared tokens supply `after`.
    pub(super) fn commit_after(&mut self, action: Action, after: Position) {
        let identity = after.repetition_identity();
        let irreversible =
            after.halfmove_clock == 0 || self.position.castling_rights != after.castling_rights;
        self.steps.push(Step {
            action,
            before: std::mem::replace(&mut self.position, after),
            previous_identity: std::mem::replace(&mut self.current_identity, identity.clone()),
            previous_reversible_repeat: self.reversible_repeat,
        });
        let count = self.occurrences.entry(identity).or_default();
        *count += 1;
        // No identity after a pawn move, capture or rights loss can equal an
        // identity before it. Thus this complete-map count is also the count
        // within the current reversible segment; no second map is needed.
        self.reversible_repeat = !irreversible && (self.reversible_repeat || *count >= 2);
    }

    /// Restore the last whole action's position and every history field.
    /// Empty history returns None without changing the initial occurrence.
    /// This is analysis branching, not a live-match command retracting results.
    pub fn undo(&mut self) -> Option<Action> {
        let step = self.steps.pop()?;
        let count = self
            .occurrences
            .get_mut(&self.current_identity)
            .expect("owned occurrence");
        *count -= 1;
        if *count == 0 {
            self.occurrences.remove(&self.current_identity);
        }
        self.position = step.before;
        self.current_identity = step.previous_identity;
        self.reversible_repeat = step.previous_reversible_repeat;
        Some(step.action)
    }
}

#[cfg(test)]
mod reversible_repeat_tests {
    use super::*;

    fn action(text: &str) -> Action {
        text.parse().unwrap()
    }

    #[test]
    fn irreversible_boundaries_retain_archive_and_restore_repeat_prerequisite() {
        for (fen, cycle, irreversible) in [
            (STANDARD_START_FEN, ["g1f3", "g8f6", "f3g1", "f6g8"], "e2e4"),
            (
                "7k/8/8/8/8/p7/8/R5K1 w - - 0 1",
                ["a1b1", "h8g8", "b1a1", "g8h8"],
                "a1a3",
            ),
            (
                "4k1n1/8/8/8/8/8/8/4K1NR w K - 0 1",
                ["g1f3", "g8f6", "f3g1", "f6g8"],
                "h1h2",
            ),
        ] {
            let mut game = Game::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
            let initial = game.clone();
            let identity = game.repetition_identity().clone();
            assert!(!game.has_repeated_identity());
            for (index, text) in cycle.into_iter().enumerate() {
                game.apply(action(text)).unwrap();
                assert_eq!(game.has_repeated_identity(), index == 3);
            }
            assert_eq!(game.occurrences(&identity), 2);
            let repeated = game.clone();
            game.apply(action(irreversible)).unwrap();
            assert!(!game.has_repeated_identity(), "{irreversible}");
            assert_eq!(
                game.occurrences(&identity),
                2,
                "archival occurrences retained"
            );
            assert_eq!(game.ply_count(), 5);
            assert_eq!(game.undo(), Some(action(irreversible)));
            assert_eq!(game, repeated);
            for text in cycle.into_iter().rev() {
                assert_eq!(game.undo(), Some(action(text)));
                assert!(!game.has_repeated_identity());
            }
            assert_eq!(game, initial);
        }
    }

    #[test]
    fn fresh_repetitions_after_boundary_and_failed_actions_restore_exactly() {
        let mut game = Game::new(ClaimPolicy::ExplicitOnly);
        for text in ["g1f3", "g8f6", "f3g1", "f6g8", "e2e4"] {
            game.apply(action(text)).unwrap();
        }
        let boundary = game.clone();
        assert!(!game.has_repeated_identity());
        // The initial post-e4 position has a raw EP target; it is not legally
        // capturable, so completing this cycle genuinely repeats its identity.
        for (index, text) in ["g8f6", "g1f3", "f6g8", "f3g1"].into_iter().enumerate() {
            game.apply(action(text)).unwrap();
            assert_eq!(game.has_repeated_identity(), index == 3);
        }
        let before_error = game.clone();
        assert!(game.apply(action("e4e6")).is_err());
        assert_eq!(game, before_error);
        for _ in 0..4 {
            game.undo().unwrap();
        }
        assert_eq!(game, boundary);
        let mut overflow = Game::from_fen(
            "7k/8/8/8/8/8/8/KR6 b - - 0 4294967295",
            ClaimPolicy::ExplicitOnly,
        )
        .unwrap();
        let before = overflow.clone();
        assert_eq!(
            overflow.apply(action("h8g8")),
            Err(ActionError::CounterOverflow)
        );
        assert_eq!(overflow, before);
    }
}
