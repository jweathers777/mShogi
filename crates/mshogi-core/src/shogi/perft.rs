use super::{Action, Position, Undo};
use crate::perft::{PerftPosition, TraversalError};

impl PerftPosition for Position {
    type Action = Action;
    type Undo = Undo;

    const COUNT_LEGAL_LEAVES: bool = true;

    fn perft_copy(&self) -> Result<Self, TraversalError> {
        Ok(self.normalized_for_board_queries())
    }

    fn perft_actions(
        &self,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Action>, TraversalError> {
        Ok(self.legal_actions())
    }

    fn perft_apply(
        &mut self,
        action: &Action,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<Undo, TraversalError> {
        self.apply(*action).map_err(TraversalError::native)
    }

    fn perft_undo(&mut self, undo: Undo) -> Result<(), String> {
        self.undo(undo).map_err(|error| error.to_string())
    }
}
