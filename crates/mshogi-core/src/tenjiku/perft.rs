use super::{Action, ActionError, Game, GameCommand, GameError, GameRequest, LocalPosition};
use crate::{
    native::history::GameUndo,
    perft::{PerftPosition, STANDARD_VISIT_LIMIT, TraversalError},
};

impl PerftPosition for Game {
    type Action = Action;
    type Undo = GameUndo;

    const MAX_DEPTH: u32 = 3;
    const MAX_VISITS: u64 = STANDARD_VISIT_LIMIT;

    fn perft_copy(&self) -> Result<Self, TraversalError> {
        Ok(self.clone())
    }

    fn perft_actions(
        &self,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Action>, TraversalError> {
        self.legal_actions_with_stop(stop).map_err(game_error)
    }

    fn perft_apply(
        &mut self,
        action: &Action,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, TraversalError> {
        self.submit_with_stop(
            GameRequest {
                expected: self.context(),
                actor: self.position().side_to_move(),
                command: GameCommand::Play {
                    action: action.clone(),
                },
            },
            stop,
        )
        .map_err(game_error)
    }

    fn perft_undo(&mut self, mut undo: GameUndo) -> Result<(), String> {
        self.undo(&mut undo).map_err(|error| error.to_string())
    }
}

fn game_error(error: GameError) -> TraversalError {
    match error {
        GameError::Stopped => TraversalError::Stopped,
        error => TraversalError::native(error),
    }
}

impl PerftPosition for LocalPosition {
    type Action = Action;
    type Undo = Self;

    fn perft_copy(&self) -> Result<Self, TraversalError> {
        Ok(self.clone())
    }

    fn perft_actions(
        &self,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Action>, TraversalError> {
        self.legal_actions_with_stop(stop).map_err(local_error)
    }

    fn perft_apply(
        &mut self,
        action: &Action,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, TraversalError> {
        let (next, _) = self.simulate_with_stop(action, stop).map_err(local_error)?;
        Ok(std::mem::replace(self, next))
    }

    fn perft_undo(&mut self, undo: Self) -> Result<(), String> {
        *self = undo;
        Ok(())
    }
}

fn local_error(error: ActionError) -> TraversalError {
    match error {
        ActionError::Stopped => TraversalError::Stopped,
        error => TraversalError::native(error),
    }
}

#[cfg(test)]
#[path = "perft_tests.rs"]
mod tests;
