//! Chu adapter for the shared search core. Native Game owns actual play.
pub mod eval;
pub mod position;
#[cfg(test)]
mod position_tests;
#[cfg(test)]
use crate::root::Status as SearchStatus;
use crate::root::{self, RootGame};
use mshogi_core::chu::{Action, Conclusion, Face, Game, GameCommand, GameError, GameRequest, Side};
use position::ChuPosition;

/// Classical material and placement evaluation.
pub const EVALUATOR_ID: &str = "mshogi.chu.material-placement.v1";
/// Shared alpha-beta search identity.
pub const SEARCH_ID: &str = "mshogi.chu.shared-core.v1";

/// Material values shared by evaluation, capture ordering and static exchange.
pub const fn piece_value(face: Face) -> i32 {
    eval::DEFAULT[eval::index(face)]
}
root::root_api!(300_000);

impl RootGame for Game {
    root::native_types!(ChuPosition, Action, fork, to_native);
    root::no_adjudication!();
    root::restricted_board!(ChuPosition);
    fn choice(action: Action) -> Action {
        action
    }
    fn play(&mut self, action: &Action, _stop: &mut dyn FnMut() -> bool) -> Result<(), GameError> {
        let request = GameRequest {
            expected: self.context(),
            actor: self.side(),
            command: GameCommand::Play {
                action: action.clone(),
            },
        };
        self.submit(request).map(drop)
    }
    fn legal(&self, _: &mut dyn FnMut() -> bool) -> Result<Option<Vec<Action>>, SearchError> {
        Ok(Some(self.legal_actions()))
    }
}
#[cfg(test)]
mod tests;
