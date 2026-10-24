//! Rules-owned action history. Adjudication is added by MSH-17/18.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    Action, ActionError, CastlingRights, Color, FenError, Piece, PieceKind, Position, Square,
};

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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RepetitionIdentity {
    board: [Option<Piece>; 64],
    side: Color,
    castling: CastlingRights,
    legal_ep: Option<Square>,
}

impl Position {
    /// Rule-equivalent identity using **legal**, counter-independent en passant.
    pub fn repetition_identity(&self) -> RepetitionIdentity {
        let legal_ep = self.en_passant_target.filter(|&target| {
            self.legal_actions().iter().any(|action| {
                action.to == target
                    && action.from.file() != target.file()
                    && self.piece_at(target).is_none()
                    && self
                        .piece_at(action.from)
                        .is_some_and(|p| p.kind == PieceKind::Pawn)
            })
        });
        RepetitionIdentity {
            board: self.board,
            side: self.side_to_move,
            castling: self.castling_rights,
            legal_ep,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Step {
    action: Action,
    before: Position,
    previous_identity: RepetitionIdentity,
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

    /// Check one action and atomically update position, history and occurrences.
    /// Failure, including counter overflow, leaves the entire game unchanged.
    pub fn apply(&mut self, action: Action) -> Result<(), ActionError> {
        let mut after = self.position.clone();
        after.apply(action)?;
        let identity = after.repetition_identity();
        self.steps.push(Step {
            action,
            before: std::mem::replace(&mut self.position, after),
            previous_identity: std::mem::replace(&mut self.current_identity, identity.clone()),
        });
        *self.occurrences.entry(identity).or_default() += 1;
        Ok(())
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
        Some(step.action)
    }
}
