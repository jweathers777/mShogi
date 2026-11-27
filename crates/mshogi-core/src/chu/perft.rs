use super::{Action, LocalPosition, LocalUndo};
use crate::perft::{PerftPosition, TraversalError};

impl PerftPosition for LocalPosition {
    type Action = Action;
    type Undo = LocalUndo;

    fn perft_copy(&self) -> Result<Self, TraversalError> {
        Ok(self.clone())
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
    ) -> Result<LocalUndo, TraversalError> {
        self.apply(action)
            .map(|(_, undo)| undo)
            .map_err(TraversalError::rules_error)
    }

    fn perft_undo(&mut self, mut undo: LocalUndo) -> Result<(), String> {
        self.undo(&mut undo).map_err(|error| error.to_string())
    }
}
