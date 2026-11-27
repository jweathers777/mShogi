//! Xiangqi adapter for the shared alpha-beta core; rules own actual play.
/// Tunable classical evaluation weights and the Texel model.
pub mod eval;
pub mod position;
#[cfg(test)]
mod position_tests;
use crate::alphabeta::SearchPosition;
pub use crate::root::Status;
use crate::root::{self, RootGame};
use mshogi_core::xiangqi::{
    Action, Conclusion, Game, GameCommand, GameError, GameRequest, NoWinAssessment,
    PendingAdjudication, PieceKind, Side,
};
use position::XiangqiPosition;

/// Untrained material and soldier advancement, independent of WXF adjudication.
pub const EVALUATOR_ID: &str = "mshogi.xiangqi.material-placement.v1";
/// Shared alpha-beta search identity.
pub const SEARCH_ID: &str = "mshogi.xiangqi.shared-core.v1";

/// Hundredths of an unadvanced soldier. A General is valued only by game outcomes.
pub const fn piece_value(kind: PieceKind) -> i32 {
    match kind {
        PieceKind::General => 0,
        PieceKind::Advisor | PieceKind::Elephant => 200,
        PieceKind::Horse => 400,
        PieceKind::Rook => 900,
        PieceKind::Cannon => 450,
        PieceKind::Soldier => 100,
    }
}
/// Complete board action versus optional formal count claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Choice {
    /// Complete action, including all history-layer consequences.
    Move(Action),
    /// Only an actually eligible claim is searched; admission remains.
    Claim {
        /// Claimant; ordinary turn search chooses for its mover.
        actor: Side,
    },
}
impl From<Action> for Choice {
    fn from(action: Action) -> Self {
        Self::Move(action)
    }
}
impl Choice {
    /// No outcome/score is applied by this conversion.
    pub const fn command(self) -> GameCommand {
        match self {
            Self::Move(action) => GameCommand::Play { action },
            Self::Claim { .. } => GameCommand::ClaimNaturalCount,
        }
    }
}
root::root_api!(300_000);

impl RootGame for Game {
    root::types!(XiangqiPosition, Choice, fork, action);
    root::timed_moves!(XiangqiPosition);

    const TABLE_MB: usize = 64;
    type Pending = PendingAdjudication;
    type Assessment = NoWinAssessment;
    fn assessment(&self) -> NoWinAssessment {
        self.no_win_assessment()
    }
    fn pending(&self) -> Option<&PendingAdjudication> {
        self.pending_adjudication()
    }
    fn root_decision(
        &self,
        result: &mut SearchResult,
        _: Limits,
        _: bool,
        _: &mut dyn FnMut() -> bool,
    ) -> Result<bool, SearchError> {
        if matches!(result.assessment, NoWinAssessment::Failure) {
            return Err(SearchError::Invariant);
        }
        Ok(false)
    }
    fn legal(&self, _: &mut dyn FnMut() -> bool) -> Result<Option<Vec<Action>>, SearchError> {
        self.position()
            .legal_actions()
            .map(Some)
            .map_err(|_| SearchError::Invariant)
    }
    fn draw_claim(&self) -> Option<Choice> {
        let actor = self.side();
        self.natural_count_eligibility(actor)
            .eligible
            .then_some(Choice::Claim { actor })
    }
    fn winner(c: Conclusion) -> Option<Side> {
        match c {
            Conclusion::Draw { .. } => None,
            Conclusion::Win { winner, .. } => Some(winner),
        }
    }
}
