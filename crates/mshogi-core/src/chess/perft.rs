use super::{Action, Position, Promotion, Undo};
use crate::perft::{PerftPosition, TraversalError};

impl PerftPosition for Position {
    type Action = Action;
    type Undo = Undo;

    const COUNT_LEGAL_LEAVES: bool = true;

    fn perft_copy(&self) -> Result<Self, TraversalError> {
        // Counters never affect board legality or the admitted en passant right.
        let mut copy = self.clone();
        copy.halfmove_clock = 0;
        copy.fullmove_number = 1;
        Ok(copy)
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
        self.apply(*action).map_err(TraversalError::rules_error)
    }

    fn perft_undo(&mut self, undo: Undo) -> Result<(), String> {
        self.undo(&undo).map_err(|error| error.to_string())
    }

    fn sort_divide_actions(actions: &mut [Action]) {
        sort_divide_actions(actions);
    }
}

/// Sort by source, destination (file fastest), then promotion Q, R, B, N.
pub fn sort_divide_actions(actions: &mut [Action]) {
    actions.sort_by_key(|action| {
        let promotion = match action.promotion {
            None => 0,
            Some(Promotion::Queen) => 1,
            Some(Promotion::Rook) => 2,
            Some(Promotion::Bishop) => 3,
            Some(Promotion::Knight) => 4,
        };
        (
            action.from.rank() * 8 + action.from.file(),
            action.to.rank() * 8 + action.to.file(),
            promotion,
        )
    });
}
