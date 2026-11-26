use super::{Action, LocalPosition, LocalUndo};
use crate::perft::{PerftPosition, STANDARD_VISIT_LIMIT, TraversalError};

impl PerftPosition for LocalPosition {
    type Action = Action;
    type Undo = LocalUndo;

    const MAX_DEPTH: u32 = 8;
    const MAX_VISITS: u64 = STANDARD_VISIT_LIMIT;

    fn perft_copy(&self) -> Result<Self, TraversalError> {
        Ok(self.board_probe())
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
        self.apply(*action)
            .map(|(_, undo)| undo)
            .map_err(TraversalError::native)
    }

    fn perft_undo(&mut self, mut undo: LocalUndo) -> Result<(), String> {
        self.undo(&mut undo).map_err(|error| error.to_string())
    }
}
