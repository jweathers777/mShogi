//! Orthodox draw/automatic adjudication over known history; configured dead proofs.
use super::*;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// Sound recognition result. Unknown never proves mating possibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeadRecognition {
    /// Neither side can mate, proven by the documented sufficient subset.
    ProvenDead,
    /// Outside this bounded recognizer; full reachability is a separate opt-in API.
    Unknown,
}

impl Position {
    /// Sufficient proofs only: kings; one minor total; or bishops exclusively
    /// on one square color. Two knights and opposite-color bishops stay unknown.
    pub fn dead_recognition(&self) -> DeadRecognition {
        let mut minor_count = 0;
        let mut bishops_only = true;
        let mut bishop_color = None;
        for rank in 0..8 {
            for file in 0..8 {
                let Some(p) = self.piece_at(Square::new(file, rank).unwrap()) else {
                    continue;
                };
                match p.kind {
                    PieceKind::King => {}
                    PieceKind::Pawn | PieceKind::Rook | PieceKind::Queen => {
                        return DeadRecognition::Unknown;
                    }
                    PieceKind::Knight => {
                        minor_count += 1;
                        bishops_only = false;
                    }
                    PieceKind::Bishop => {
                        minor_count += 1;
                        let color = (rank + file) % 2;
                        if bishop_color.is_some_and(|old| old != color) {
                            bishops_only = false;
                        }
                        bishop_color = Some(color);
                    }
                }
            }
        }
        if minor_count <= 1 || bishops_only {
            DeadRecognition::ProvenDead
        } else {
            DeadRecognition::Unknown
        }
    }
}

/// Exact draw reason; opportunities are never represented as results.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawReason {
    /// Side to move has no legal action and is not in check.
    Stalemate,
    /// Sound material proof or full reachable-graph exhaustion.
    DeadPosition,
    /// Five known occurrences of the current repetition identity.
    Fivefold,
    /// At least 150 halfmoves, with checkmate taking precedence.
    SeventyFiveMove,
    /// A valid optional claim was actually exercised.
    Claimed(DrawClaim),
}

/// Concluded chess result. None at the API boundary means ongoing, even if a
/// claim is available. Match commands/clock outcomes are added separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ChessOutcome {
    /// Checkmate wins for the opponent of the side to move.
    Checkmate {
        /// Winning color.
        winner: Color,
    },
    /// Concluded draw with its established primary reason.
    Draw(DrawReason),
}

/// Optional claim request. Intended actions are tested without being played.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClaimRequest {
    /// Must be the player to move.
    pub actor: Color,
    /// Requested rule.
    pub reason: DrawClaim,
    /// Intended complete action, or None for a current-position claim.
    pub intended: Option<Action>,
}

/// Rejected claim; neither board, history nor outcome changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimError {
    /// The game is already concluded.
    GameOver,
    /// Actor is not the side to move.
    WrongActor,
    /// The named threshold is not reached.
    NotAvailable,
    /// Intended action is illegal.
    InvalidIntended(ActionError),
}
impl fmt::Display for ClaimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "draw claim rejected: {self:?}")
    }
}
impl Error for ClaimError {}

impl Game {
    /// Automatic outcome of the current history snapshot, not proof that every
    /// past action was valid live play. AdjudicatedGame validates the live path.
    pub fn automatic_outcome(&self) -> Option<ChessOutcome> {
        match self.position().classify() {
            Classification::Checkmate => {
                return Some(ChessOutcome::Checkmate {
                    winner: self.position().side_to_move().opponent(),
                });
            }
            Classification::Stalemate => return Some(ChessOutcome::Draw(DrawReason::Stalemate)),
            _ => {}
        }
        if self.position().dead_recognition() == DeadRecognition::ProvenDead {
            return Some(ChessOutcome::Draw(DrawReason::DeadPosition));
        }
        if self.current_occurrences() >= 5 {
            return Some(ChessOutcome::Draw(DrawReason::Fivefold));
        }
        if self.position().halfmove_clock() >= 150 {
            return Some(ChessOutcome::Draw(DrawReason::SeventyFiveMove));
        }
        None
    }

    fn validate_claim_without_outcome(&self, request: ClaimRequest) -> Result<(), ClaimError> {
        if request.actor != self.position().side_to_move() {
            return Err(ClaimError::WrongActor);
        }
        let (identity, count) = if let Some(action) = request.intended {
            let mut projected = self.position().clone();
            // A claim does not play a move. Counter representability is not
            // legality; fullmove numbers are irrelevant to either threshold.
            projected.fullmove_number = 1;
            projected
                .apply(action)
                .map_err(ClaimError::InvalidIntended)?;
            (projected.repetition_identity(), projected.halfmove_clock())
        } else {
            (
                self.repetition_identity().clone(),
                self.position().halfmove_clock(),
            )
        };
        let valid = match request.reason {
            DrawClaim::Threefold => {
                self.occurrences(&identity) + usize::from(request.intended.is_some()) >= 3
            }
            DrawClaim::FiftyMove => count >= 100,
        };
        if valid {
            Ok(())
        } else {
            Err(ClaimError::NotAvailable)
        }
    }

    /// Validate a current/intended claim without mutating or concluding a game.
    pub fn validate_claim(&self, request: ClaimRequest) -> Result<(), ClaimError> {
        if self.automatic_outcome().is_some() {
            return Err(ClaimError::GameOver);
        }
        self.validate_claim_without_outcome(request)
    }

    /// All available claims in profile policy order: current threefold/fifty,
    /// then intended threefold/fifty, with actions sorted by square and q/r/b/n.
    pub fn available_claims(&self) -> Vec<ClaimRequest> {
        if self.automatic_outcome().is_some() {
            return Vec::new();
        }
        let actor = self.position().side_to_move();
        let mut claims = Vec::new();
        for reason in [DrawClaim::Threefold, DrawClaim::FiftyMove] {
            let r = ClaimRequest {
                actor,
                reason,
                intended: None,
            };
            if self.validate_claim_without_outcome(r).is_ok() {
                claims.push(r);
            }
        }
        let mut actions = self.position().legal_actions();
        actions.sort_by_key(|a| {
            (
                a.from.rank() * 8 + a.from.file(),
                a.to.rank() * 8 + a.to.file(),
                match a.promotion {
                    None => 0,
                    Some(Promotion::Queen) => 1,
                    Some(Promotion::Rook) => 2,
                    Some(Promotion::Bishop) => 3,
                    Some(Promotion::Knight) => 4,
                },
            )
        });
        for reason in [DrawClaim::Threefold, DrawClaim::FiftyMove] {
            for &action in &actions {
                let r = ClaimRequest {
                    actor,
                    reason,
                    intended: Some(action),
                };
                if self.validate_claim_without_outcome(r).is_ok() {
                    claims.push(r);
                }
            }
        }
        claims
    }
}

/// Live-game rejection, separate from position-only ActionError.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayError {
    /// Previous result is final.
    GameOver,
    /// Invalid or unrepresentable action; game unchanged.
    Action(ActionError),
    /// Invalid optional claim; game unchanged.
    Claim(ClaimError),
}
impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameOver => f.write_str("game is already concluded"),
            Self::Action(e) => e.fmt(f),
            Self::Claim(e) => e.fmt(f),
        }
    }
}
impl Error for PlayError {}

/// One whole live event reversed for analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdjudicationUndo {
    /// Played board action was reversed.
    Move(Action),
    /// Accepted claim was reversed; the intended action was never played.
    Claim(ClaimRequest),
}

/// Rules-owned live boundary over Game's diagnostic history. Construction from
/// a history validates every past action against automatic outcomes. Claims are
/// optional until explicitly exercised; stored policy alone does not execute.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdjudicatedGame {
    history: Game,
    claimed: Option<ClaimRequest>,
    mode: AdjudicationMode,
    dead_analysis: Option<MateReachability>,
}
impl AdjudicatedGame {
    /// Known standard start.
    pub fn new(policy: ClaimPolicy) -> Self {
        Self {
            history: Game::new(policy),
            claimed: None,
            mode: AdjudicationMode::M2,
            dead_analysis: None,
        }
    }
    /// Imported start, with unknown earlier repetition history.
    pub fn from_fen(fen: &str, policy: ClaimPolicy) -> Result<Self, FenError> {
        Ok(Self {
            history: Game::from_fen(fen, policy)?,
            claimed: None,
            mode: AdjudicationMode::M2,
            dead_analysis: None,
        })
    }
    /// Validate a historical action path as live play without exercising policy.
    pub fn from_history(history: Game) -> Result<Self, PlayError> {
        let mut game = Self {
            history: Game::from_initial(
                history.initial_position().clone(),
                history.history_origin(),
                history.claim_policy(),
            ),
            claimed: None,
            mode: AdjudicationMode::M2,
            dead_analysis: None,
        };
        for a in history.actions() {
            game.apply(a)?;
        }
        Ok(game)
    }
    /// Construct an imported live game with explicitly selected completeness.
    /// Full mode may be expensive; use limits for operational calls.
    pub fn from_fen_with_mode(
        fen: &str,
        policy: ClaimPolicy,
        mode: AdjudicationMode,
    ) -> Result<Self, FenError> {
        let mut game = Self::from_fen(fen, policy)?;
        game.mode = mode;
        game.refresh_dead_analysis();
        Ok(game)
    }
    /// Validate all historical moves under the recorded completeness mode.
    pub fn from_history_with_mode(
        history: Game,
        mode: AdjudicationMode,
    ) -> Result<Self, PlayError> {
        let mut game = Self {
            history: Game::from_initial(
                history.initial_position().clone(),
                history.history_origin(),
                history.claim_policy(),
            ),
            claimed: None,
            mode,
            dead_analysis: None,
        };
        game.refresh_dead_analysis();
        for action in history.actions() {
            game.apply(action)?;
        }
        Ok(game)
    }
    /// Recorded semantics/work configuration; legacy constructors select M2.
    pub const fn adjudication_mode(&self) -> AdjudicationMode {
        self.mode
    }
    /// Cached current full-dead analysis. It is not rerun by outcome/claim reads.
    pub fn dead_analysis(&self) -> Option<&MateReachability> {
        self.dead_analysis.as_ref()
    }
    fn refresh_dead_analysis(&mut self) {
        self.dead_analysis = match self.mode {
            AdjudicationMode::M2 => None,
            AdjudicationMode::Full { limits } => Some(self.position().mate_reachability(
                MateTarget::Either,
                limits,
                || false,
            )),
        };
    }
    /// Immutable full game context for search and adapters.
    pub const fn history(&self) -> &Game {
        &self.history
    }
    /// Current position, never mutable through the adapter.
    pub fn position(&self) -> &Position {
        self.history.position()
    }
    /// Actual concluded result, independent of optional opportunities.
    pub fn outcome(&self) -> Option<ChessOutcome> {
        // Checkmate/stalemate retain precedence. Full death precedes history draws.
        if self.claimed.is_none()
            && matches!(
                self.dead_analysis,
                Some(MateReachability::Impossible { .. })
            )
            && !matches!(
                self.position().classify(),
                Classification::Checkmate | Classification::Stalemate
            )
        {
            return Some(ChessOutcome::Draw(DrawReason::DeadPosition));
        }
        self.claimed
            .map(|c| ChessOutcome::Draw(DrawReason::Claimed(c.reason)))
            .or_else(|| self.history.automatic_outcome())
    }
    /// Available optional claims; empty after any conclusion.
    pub fn available_claims(&self) -> Vec<ClaimRequest> {
        if self.outcome().is_some() {
            Vec::new()
        } else {
            self.history.available_claims()
        }
    }
    /// Claim the selected policy would exercise at a turn boundary, if any.
    pub fn policy_claim(&self) -> Option<ClaimRequest> {
        if self.history.claim_policy() == ClaimPolicy::AutoClaim {
            self.available_claims().into_iter().next()
        } else {
            None
        }
    }
    /// Checked live action. Automatic conclusions take effect before any later event.
    pub fn apply(&mut self, action: Action) -> Result<(), PlayError> {
        if self.outcome().is_some() {
            return Err(PlayError::GameOver);
        }
        self.history.apply(action).map_err(PlayError::Action)?;
        self.refresh_dead_analysis();
        Ok(())
    }
    /// Exercise a valid claim atomically. Intended actions are not committed.
    pub fn claim(&mut self, request: ClaimRequest) -> Result<(), ClaimError> {
        if self.outcome().is_some() {
            return Err(ClaimError::GameOver);
        }
        self.history.validate_claim(request)?;
        self.claimed = Some(request);
        Ok(())
    }
    /// Exact event undo for private analysis; never a live match command.
    pub fn undo(&mut self) -> Option<AdjudicationUndo> {
        if let Some(c) = self.claimed.take() {
            Some(AdjudicationUndo::Claim(c))
        } else {
            let action = self.history.undo()?;
            self.refresh_dead_analysis();
            Some(AdjudicationUndo::Move(action))
        }
    }
    /// Canonical actions plus the exercised claim, if any.
    pub fn record(&self) -> GameRecord {
        let mut r = self.history.record();
        r.set_adjudication_mode(self.mode);
        if let Some(c) = self.claimed {
            r.push_event(RecordEvent {
                ply: self.history.ply_count() as u64,
                kind: RecordEventKind::Observation(MatchObservation::Command {
                    actor: c.actor,
                    command: RecordedCommand::ClaimDraw {
                        reason: c.reason,
                        intended: c.intended,
                    },
                }),
            });
        }
        r
    }
    /// Replay live actions and accepted claims. Other observations are explicitly
    /// unsupported here; match commands and clock validation belong to MSH-18.
    pub fn replay_record(record: &GameRecord) -> Result<Self, LiveReplayError> {
        let mut game = Self {
            history: Game::from_initial(
                record.initial_position().clone(),
                record.history_origin(),
                record.claim_policy(),
            ),
            claimed: None,
            mode: record.adjudication_mode(),
            dead_analysis: None,
        };
        game.refresh_dead_analysis();
        for (i, event) in record.events().iter().enumerate() {
            if event.ply != game.history.ply_count() as u64 {
                return Err(LiveReplayError::Context(i + 1));
            }
            let result = match event.kind {
                RecordEventKind::Move(a) => game.apply(a),
                RecordEventKind::Observation(MatchObservation::Command {
                    actor,
                    command: RecordedCommand::ClaimDraw { reason, intended },
                }) => game
                    .claim(ClaimRequest {
                        actor,
                        reason,
                        intended,
                    })
                    .map_err(PlayError::Claim),
                _ => return Err(LiveReplayError::Unsupported(i + 1)),
            };
            result.map_err(|e| LiveReplayError::Play(i + 1, e))?;
        }
        Ok(game)
    }
}
/// Live replay failures never return a partial game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveReplayError {
    /// Declared ply is inconsistent with actions.
    Context(usize),
    /// This layer cannot interpret the observed event.
    Unsupported(usize),
    /// Action or claim is invalid at this one-based event.
    Play(usize, PlayError),
}
impl fmt::Display for LiveReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "live replay rejected: {self:?}")
    }
}
impl Error for LiveReplayError {}
