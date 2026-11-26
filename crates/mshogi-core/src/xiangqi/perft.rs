use super::{Action, Counters, Position, Undo};
use crate::perft::{PerftPosition, TraversalError};

impl PerftPosition for Position {
    type Action = Action;
    type Undo = Undo;

    const MAX_DEPTH: u32 = 3;

    fn perft_copy(&self) -> Result<Self, TraversalError> {
        self.validate_local().map_err(TraversalError::native)?;
        let mut position = self.clone();
        position.counters = Counters::new(Some(0), Some(1)).expect("normalized");
        Ok(position)
    }

    fn perft_actions(
        &self,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Action>, TraversalError> {
        self.legal_actions().map_err(TraversalError::native)
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

    fn sort_divide_actions(actions: &mut [Action]) {
        actions.sort_by_cached_key(ToString::to_string);
    }
}
