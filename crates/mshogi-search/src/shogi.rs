//! Shogi adapter for the game-independent alpha-beta core.
pub mod eval;
pub mod position;
#[cfg(test)]
mod position_tests;
use crate::alphabeta::SearchPosition;
use crate::root::{self, RootGame};
use mshogi_core::shogi::{Action, Conclusion, Game, GameCommand, GameError, GameRequest, Side};
use position::ShogiPosition;

/// Classical material and placement evaluation identity.
pub const EVALUATOR_ID: &str = "mshogi.shogi.material-placement.v1";
/// Shared alpha-beta search identity.
pub const SEARCH_ID: &str = "mshogi.shogi.shared-core.v1";

/// Native complete action versus explicit terminal declaration choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Choice {
    /// A complete legal board move/drop/promotion.
    Move(Action),
    /// Formal entering-king declaration for the native actor; not a board action.
    Declare {
        /// Native declarer; submitting a declaration does not change board turn.
        actor: Side,
    },
}
impl From<Action> for Choice {
    fn from(action: Action) -> Self {
        Self::Move(action)
    }
}
impl Choice {
    /// Retain native command semantics; admission still belongs to Game::submit.
    pub const fn command(self) -> GameCommand {
        match self {
            Self::Move(action) => GameCommand::Play(action),
            Self::Declare { .. } => GameCommand::DeclareEnteringKing,
        }
    }
}
root::root_api!(300_000);

impl RootGame for Game {
    root::native_types!(ShogiPosition, Choice, clone, action);
    root::no_adjudication!();
    root::timed_moves!(ShogiPosition);
    fn legal(&self, _: &mut dyn FnMut() -> bool) -> Result<Option<Vec<Action>>, SearchError> {
        Ok(Some(self.position().legal_actions()))
    }
    const TIMED_ZERO_NODES: bool = true;
    fn winner(c: Conclusion) -> Option<Side> {
        match c {
            Conclusion::Draw(_) => None,
            Conclusion::Win { winner, .. } => Some(winner),
        }
    }
    fn root_decision(
        &self,
        result: &mut SearchResult,
        limits: Limits,
        timed: bool,
        stop: &mut dyn FnMut() -> bool,
    ) -> Result<bool, SearchError> {
        let root_side = result.perspective;
        if self.declaration_eligibility(root_side).eligible() && (!timed || limits.nodes >= 2) {
            result.nodes = 1;
            if stop() {
                result.status = root::Status::Stopped;
                return Ok(true);
            }
            if limits.nodes < 2 {
                result.status = root::Status::NodeLimit;
                return Ok(true);
            }
            let choice = Choice::Declare { actor: root_side };
            let mut native = self.clone();
            native
                .play(&choice, stop)
                .map_err(SearchError::Transition)?;
            let conclusion = native.conclusion().ok_or(SearchError::Invariant)?;
            result.decision = Some(Decision::Choose(choice));
            result.score = Some(terminal_score(conclusion, root_side, 1));
            result.completed_depth = 1;
            result.nodes = 2;
            result.principal_variation.push(choice);
            return Ok(true);
        }
        Ok(false)
    }
}
