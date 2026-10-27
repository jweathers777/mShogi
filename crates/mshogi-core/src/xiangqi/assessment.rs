//! Sound fast recognition and the native assessment vocabulary; exact work stays explicit.
use super::{Action, LocalStatus, PieceKind, Position, Side};
use serde::{Deserialize, Serialize};
/// Owner-selected natural-course board-win goal. Penalty awards are excluded.
pub const NATURAL_WIN_GOAL: &str = "mshogi.xiangqi.natural-board-win.exists.v1";
/// Unknown is not proof of a possible win and never concludes a draw.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum NoWinAssessment {
    /// Two bare palace Generals have no legal check, capture or stalemate path.
    ProvenNeitherCanWin,
    /// A concrete finite natural board-win witness, not a forced-win claim.
    ProvenNaturalWin {
        /// Native side with the witness.
        winner: Side,
        /// Complete local actions; zero length for an already terminal board.
        witness: Vec<Action>,
    },
    /// No sound fast proof was found; a separate explicit exact request is needed.
    UnknownExactUnavailable,
    /// An explicitly requested exact run stopped without establishing either proof.
    UnknownExactIncomplete {
        /// Cancellation or the actual work/resource limit, never win capability.
        reason: super::ExactStop,
    },
    /// Local input/invariant failed; never a chess outcome.
    Failure,
}
/// Sound narrowly scoped fast facts under the owner-selected WXF3.1A goal.
/// No history penalty, resignation, agreement or failed proof is a win witness.
pub fn assess_natural_win(position: &Position) -> NoWinAssessment {
    let Ok(status) = position.local_status() else {
        return NoWinAssessment::Failure;
    };
    if matches!(status, LocalStatus::NoLegalAction { .. }) {
        return NoWinAssessment::ProvenNaturalWin {
            winner: position.side_to_move().opponent(),
            witness: vec![],
        };
    }
    if position
        .pieces()
        .all(|(_, piece)| piece.kind == PieceKind::General)
    {
        return NoWinAssessment::ProvenNeitherCanWin;
    }
    // One-ply positive proofs are checked by actual ordinary transitions, so a
    // representation failure is skipped, never mistaken for a mate or a proof.
    let Ok(actions) = position.legal_actions() else {
        return NoWinAssessment::Failure;
    };
    for action in actions {
        let mut after = position.clone();
        if after.apply(action).is_ok()
            && matches!(after.local_status(), Ok(LocalStatus::NoLegalAction { .. }))
        {
            return NoWinAssessment::ProvenNaturalWin {
                winner: position.side_to_move(),
                witness: vec![action],
            };
        }
    }
    NoWinAssessment::UnknownExactUnavailable
}

impl super::ExactReport {
    /// Native assessment result, preserving the distinction between a fast miss,
    /// completed proof, incomplete exact work and execution/invariant failure.
    /// This query is not permission to overwrite an actual history conclusion.
    pub fn assessment(&self) -> NoWinAssessment {
        match &self.verdict {
            super::ExactVerdict::NeitherCanWin => NoWinAssessment::ProvenNeitherCanWin,
            super::ExactVerdict::NaturalWin {
                winner, witness, ..
            } => NoWinAssessment::ProvenNaturalWin {
                winner: *winner,
                witness: witness.clone(),
            },
            super::ExactVerdict::Unknown { reason } => {
                NoWinAssessment::UnknownExactIncomplete { reason: *reason }
            }
            super::ExactVerdict::Failure { .. } => NoWinAssessment::Failure,
        }
    }
}
