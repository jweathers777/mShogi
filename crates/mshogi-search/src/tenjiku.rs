//! Tenjiku adapter for the shared search core. Game owns actual play.
pub mod eval;
pub mod position;
#[cfg(test)]
mod position_tests;
#[cfg(test)]
use crate::root::Status as SearchStatus;
use crate::root::{self, RootGame};
use mshogi_core::tenjiku::{
    Action, Conclusion, Face, Game, GameCommand, GameError, GameRequest, Side,
};
use position::TenjikuPosition;

/// Classical material and placement evaluation.
pub const EVALUATOR_ID: &str = "mshogi.tenjiku.material-placement.v1";
/// Shared alpha-beta search identity.
pub const SEARCH_ID: &str = "mshogi.tenjiku.shared-core.v1";

/// Untrained material values in hundredths of a pawn.
pub const fn piece_value(face: Face) -> i32 {
    match face {
        Face::P => 100,
        Face::Dg | Face::N => 200,
        Face::L | Face::RC | Face::I | Face::C => 300,
        Face::S | Face::FL | Face::BT => 400,
        Face::G | Face::DE => 500,
        Face::Ky | Face::Ph => 600,
        Face::SM | Face::VM | Face::B | Face::MGn => 700,
        Face::R | Face::SSo | Face::VSo | Face::WH | Face::W | Face::FS => 900,
        Face::DK | Face::DH | Face::FO | Face::FB => 1100,
        Face::SE | Face::HF | Face::ChS | Face::HT => 1300,
        Face::FK => 1500,
        Face::FEg | Face::BGn | Face::RGn => 1800,
        Face::Ln | Face::WBf | Face::VGn => 2000,
        Face::LHk | Face::GGn => 2300,
        Face::FiD => 2500,
        Face::K | Face::CP => 5000,
    }
}
root::root_api!(50_000);

impl RootGame for Game {
    root::types!(TenjikuPosition, Action, clone, to_action);
    root::no_adjudication!();
    root::restricted_board!(TenjikuPosition);
    fn play(
        &mut self,
        action: &Action,
        mut stop: &mut dyn FnMut() -> bool,
    ) -> Result<(), GameError> {
        let request = GameRequest {
            expected: self.context(),
            actor: self.side(),
            command: GameCommand::Play {
                action: action.clone(),
            },
        };
        self.submit_with_stop(request, &mut stop).map(drop)
    }

    fn legal(
        &self,
        mut stop: &mut dyn FnMut() -> bool,
    ) -> Result<Option<Vec<Action>>, SearchError> {
        match self.legal_actions_with_stop(&mut stop) {
            Ok(legal) => Ok(Some(legal)),
            Err(GameError::Stopped) => Ok(None),
            Err(error) => Err(SearchError::Transition(error)),
        }
    }
    fn stopped(error: &GameError) -> bool {
        matches!(error, GameError::Stopped)
    }
    fn stop_before_replay(stop: &mut dyn FnMut() -> bool) -> bool {
        stop()
    }
}
#[cfg(test)]
mod tests;
