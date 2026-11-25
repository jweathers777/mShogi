//! Actual native Chu results. No declaration, chess mate or exact-proof adaptation.
use crate::{ExecutionContext, ExecutionPurpose, StopReason};
use mshogi_core::{
    chu::{Action, Conclusion, Game, GameCommand, GameRequest, Side},
    native::CanonicalRecord,
};
use mshogi_search::chu as search;
/// Current bounded interactive Chu defaults, shared by native consumers.
pub use mshogi_search::chu::DEFAULT_LIMITS as CHU_DEFAULT_LIMITS;
/// Maximum supported native Chu search depth.
pub use mshogi_search::chu::MAX_DEPTH as CHU_MAX_DEPTH;
/// Maximum supported native Chu visit budget.
pub use mshogi_search::chu::MAX_NODES as CHU_MAX_NODES;
use serde::{Deserialize, Serialize};
/// Measured finite native Chu interactive work settings.
pub const CHU_PLAY_LIMITS: crate::WorkLimits = crate::WorkLimits {
    time_ms: None,
    depth: CHU_DEFAULT_LIMITS.depth,
    nodes: CHU_DEFAULT_LIMITS.nodes,
};
/// Actual bounded search observations, in native root-side units.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChuTelemetry {
    /// Fixed transparent evaluator identity.
    pub evaluator_id: String,
    /// Fixed deterministic search identity.
    pub search_id: String,
    /// Named analyzed native side.
    pub perspective: Side,
    /// Hundredths of a Pawn, or native terminal action distance; absent until completed work.
    #[serde(deserialize_with = "required_score")]
    pub score: Option<i32>,
    /// Last complete nominal iteration.
    pub completed_depth: u16,
    /// Total admitted visits, including discarded work.
    pub nodes: u64,
    /// Native complete continuation from completed work.
    pub principal_variation: Vec<Action>,
    /// Completion, resource exhaustion or cooperative stop; never a game outcome.
    pub status: StopReason,
}
fn required_score<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    Option::<i32>::deserialize(d)
}
impl ChuTelemetry {
    fn is_current(&self) -> bool {
        self.evaluator_id == search::EVALUATOR_ID && self.search_id == search::SEARCH_ID
    }
    /// Native win/loss distance is not necessarily checkmate; heuristic units are Pawns.
    pub fn score_label(&self) -> Option<String> {
        if !self.is_current() {
            return None;
        }
        self.score.and_then(|score| {
            let n = i64::from(score).abs();
            if n > i64::from(search::WIN_SCORE) {
                return None;
            }
            Some(if n >= i64::from(mshogi_search::alphabeta::MATE_BOUND) {
                let d = i64::from(search::WIN_SCORE) - n;
                let outcome = if score >= 0 { "Win" } else { "Loss" };
                if d == 0 {
                    format!("{outcome} at the analyzed position")
                } else {
                    format!(
                        "{outcome} in {d} board {}",
                        if d == 1 { "action" } else { "actions" }
                    )
                }
            } else {
                format!("{:+.2} Pawns", f64::from(score) / 100.0)
            })
        })
    }
}
/// Complete native action, genuine conclusion, or no usable completed iteration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ChuResult {
    /// Proposed action; authoritative host admission is still required.
    Choice {
        /// Complete action.
        action: Action,
        /// Actual observations.
        telemetry: ChuTelemetry,
    },
    /// Exact native root conclusion.
    Finished {
        /// Native history-owned outcome.
        conclusion: Conclusion,
        /// Terminal-root observation.
        telemetry: ChuTelemetry,
    },
    /// No completed iteration, fallback move or score.
    Incomplete {
        /// Actual unsuccessful work.
        telemetry: ChuTelemetry,
    },
}
impl ChuResult {
    /// Borrow actual native telemetry without conflating result kinds.
    pub fn telemetry(&self) -> &ChuTelemetry {
        match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry } => telemetry,
        }
    }
    pub(crate) fn searched(r: search::SearchResult) -> Self {
        let telemetry = ChuTelemetry {
            evaluator_id: search::EVALUATOR_ID.into(),
            search_id: search::SEARCH_ID.into(),
            perspective: r.perspective,
            score: r.score,
            completed_depth: r.completed_depth,
            nodes: r.nodes,
            principal_variation: r.principal_variation,
            status: match r.status {
                mshogi_search::search::SearchStatus::Complete => StopReason::Complete,
                mshogi_search::search::SearchStatus::NodeLimit => StopReason::NodeLimit,
                mshogi_search::search::SearchStatus::Stopped => StopReason::Stopped,
            },
        };
        match r.decision {
            Some(search::Decision::Choose(action)) => Self::Choice { action, telemetry },
            Some(search::Decision::Finished(conclusion)) => Self::Finished {
                conclusion,
                telemetry,
            },
            None => Self::Incomplete { telemetry },
        }
    }
    /// Re-admit the complete continuation against authoritative native history.
    /// This validates consistency, not a claim that a heuristic score is optimal.
    /// Runtime identities and submitted limits remain the observation owner's check.
    pub fn validate(&self, game: &Game) -> Result<(), String> {
        self.validate_with_stop(game, &mut || false)
    }
    /// Validate native continuation under an admission worker's stop/deadline.
    pub fn validate_with_stop(
        &self,
        game: &Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        if stop() {
            return Err("Chu observation validation stopped".into());
        }
        let t = self.telemetry();
        if !t.is_current()
            || t.perspective != game.position().side_to_move()
            || t.completed_depth > search::MAX_DEPTH
            || t.nodes > search::MAX_NODES
            || t.score
                .is_some_and(|s| !(-search::WIN_SCORE..=search::WIN_SCORE).contains(&s))
            || t.principal_variation.len() > search::MAX_LINE
        {
            return Err("Chu telemetry identity, work or score mismatch".into());
        }
        match self {
            Self::Incomplete { .. } => {
                if t.completed_depth != 0
                    || t.score.is_some()
                    || !t.principal_variation.is_empty()
                    || t.status == StopReason::Complete
                {
                    return Err("incomplete Chu work carried a completed result".into());
                }
            }
            Self::Finished { conclusion, .. } => {
                if game.conclusion() != Some(*conclusion)
                    || t.completed_depth != 1
                    || t.nodes != 1
                    || t.status != StopReason::Complete
                    || t.score != Some(search::terminal_score(*conclusion, t.perspective, 0))
                    || !t.principal_variation.is_empty()
                {
                    return Err("Chu terminal observation disagrees with native root".into());
                }
            }
            Self::Choice { action, .. } => {
                if game.conclusion().is_some()
                    || t.completed_depth == 0
                    || t.nodes < u64::from(t.completed_depth)
                    || t.score.is_none()
                    || t.principal_variation.first() != Some(action)
                {
                    return Err("Chu choice has no completed live-root iteration".into());
                }
                let mut branch = game.fork().map_err(|e| e.to_string())?;
                for action in &t.principal_variation {
                    if stop() {
                        return Err("Chu observation validation stopped".into());
                    }
                    branch
                        .submit(GameRequest {
                            expected: branch.context(),
                            actor: branch.position().side_to_move(),
                            command: GameCommand::Play {
                                action: action.clone(),
                            },
                        })
                        .map_err(|e| e.to_string())?;
                }
                if stop() {
                    return Err("Chu observation validation stopped".into());
                }
                // Selective search scores are not the static value at the end
                // of a bounded reported line. Native replay above remains mandatory.
                if t.score
                    .is_some_and(|s| s.unsigned_abs() >= search::WIN_SCORE as u32)
                {
                    return Err("Chu choice cannot be a win at the analyzed root".into());
                }
            }
        }
        Ok(())
    }
    pub(crate) fn matches(&self, context: &ExecutionContext) -> bool {
        let CanonicalRecord::Chu(record) = &context.game else {
            return false;
        };
        if !matches!(
            context.purpose,
            ExecutionPurpose::EngineTurn {} | ExecutionPurpose::MoveAnalysis { .. }
        ) {
            return false;
        }
        let t = self.telemetry();
        if !t.is_current()
            || context.limits.depth == 0
            || context.limits.depth > search::MAX_DEPTH
            || context.limits.nodes > search::MAX_NODES
        {
            return false;
        }
        if t.perspective != record.side_to_move()
            || t.completed_depth > context.limits.depth
            || t.nodes > context.limits.nodes
            || (t.status == StopReason::NodeLimit && t.nodes != context.limits.nodes)
            || t.principal_variation.len() > search::MAX_LINE
            || t.score
                .is_some_and(|s| !(-search::WIN_SCORE..=search::WIN_SCORE).contains(&s))
        {
            return false;
        }
        // This path is called while polling the supervised child on the host.
        // Cached/shape checks only: native replay/PV/leaf validation belongs to
        // the bounded off-path admission worker via validate, before mutation.
        match self {
            Self::Choice { action, .. } => {
                record.conclusion().is_none()
                    && t.completed_depth > 0
                    && t.nodes >= u64::from(t.completed_depth)
                    && t.score.is_some()
                    && t.principal_variation.first() == Some(action)
            }
            Self::Finished { conclusion, .. } => {
                record.conclusion() == Some(*conclusion)
                    && t.completed_depth == 1
                    && t.nodes == 1
                    && t.status == StopReason::Complete
                    && t.score == Some(search::terminal_score(*conclusion, t.perspective, 0))
                    && t.principal_variation.is_empty()
            }
            Self::Incomplete { .. } => {
                t.completed_depth == 0
                    && t.score.is_none()
                    && t.principal_variation.is_empty()
                    && t.status != StopReason::Complete
            }
        }
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    #[test]
    fn identity_bounds_and_selective_distance_labels() {
        let game = Game::standard();
        let mut context = ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            game: CanonicalRecord::chu(mshogi_core::chu::GameRecord::from_game(&game).unwrap()),
            limits: CHU_PLAY_LIMITS,
        };
        let mut telemetry = ChuTelemetry {
            evaluator_id: "mshogi.chu.unknown.v1".into(),
            search_id: search::SEARCH_ID.into(),
            perspective: Side::Sente,
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::Stopped,
        };
        let incomplete = |telemetry: &ChuTelemetry| ChuResult::Incomplete {
            telemetry: telemetry.clone(),
        };
        assert!(!incomplete(&telemetry).matches(&context));
        assert!(incomplete(&telemetry).validate(&game).is_err());
        telemetry.evaluator_id = search::EVALUATOR_ID.into();
        assert!(incomplete(&telemetry).matches(&context));
        incomplete(&telemetry).validate(&game).unwrap();
        context.limits.depth = search::MAX_DEPTH + 1;
        assert!(!incomplete(&telemetry).matches(&context));
        telemetry.score = Some(search::WIN_SCORE - 20);
        assert_eq!(
            telemetry.score_label().as_deref(),
            Some("Win in 20 board actions")
        );
        telemetry.score = Some(i32::MIN);
        assert_eq!(telemetry.score_label(), None);
    }
}
