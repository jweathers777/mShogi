//! Tori Shogi engine on the shared search core.
//!
//! The `Game` decides everything about the real game: a concluded
//! root, the legal actions (with the departures that the repetition rule
//! forbids) and each action of the reported line. The shared core searches a
//! private [`position::ToriPosition`] and proposes a move; this adapter returns
//! it only if the legal actions contain it.
/// Tunable classical evaluation weights and the Texel model.
pub mod eval;
pub mod position;

use crate::alphabeta::SearchPosition;
#[cfg(test)]
use crate::root::Status as SearchStatus;
use crate::root::{self, RootGame};
use mshogi_core::tori::{Action, Conclusion, Game, GameCommand, GameError, GameRequest, Side};
use position::ToriPosition;
pub use position::piece_value;
#[doc(hidden)]
pub use position::{ProfileRow, movegen_profile};

/// Evaluation identity: the linear Texel-fitted evaluation (`tori/eval.rs`).
/// MSH-206 kept the identity; shared-core scores have no exact score check.
pub const EVALUATOR_ID: &str = "mshogi.tori.material-placement.v1";
/// Search identity: the shared alpha-beta core.
pub const SEARCH_ID: &str = "mshogi.tori.shared-core.v1";

root::root_api!(300_000);

impl RootGame for Game {
    root::types!(ToriPosition, Action, fork, action);
    root::no_adjudication!();
    fn play(&mut self, action: &Action, _stop: &mut dyn FnMut() -> bool) -> Result<(), GameError> {
        let request = GameRequest {
            expected: self.context(),
            actor: self.side(),
            command: GameCommand::Play { action: *action },
        };
        self.submit(request).map(drop)
    }
    fn legal(&self, _: &mut dyn FnMut() -> bool) -> Result<Option<Vec<Action>>, SearchError> {
        Ok(Some(self.legal_actions()))
    }
    fn winner(c: Conclusion) -> Option<Side> {
        Some(match c {
            Conclusion::Checkmate { winner } | Conclusion::Resignation { winner } => winner,
            Conclusion::StalemateLoss { loser } | Conclusion::RepetitionForfeit { loser } => {
                loser.opponent()
            }
        })
    }
    fn position_for_search(&self, _: &[Action]) -> ToriPosition {
        ToriPosition::from_game(self)
    }
    fn replay_action(pos: &mut ToriPosition, m: position::Move) -> Option<Action> {
        let action = pos.action(m);
        pos.make(m).then_some(action)
    }
}
#[cfg(test)]
mod position_tests;
#[cfg(test)]
mod tests;
