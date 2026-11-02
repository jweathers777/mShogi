//! Actual native Tori results. No declaration, chess mate or exact-proof adaptation.
use crate::{ExecutionContext, ExecutionPurpose, StopReason};
use mshogi_core::{
    native::CanonicalRecord,
    tori::{Action, Conclusion, Game, GameCommand, GameRecord, GameRequest, Side},
};
use mshogi_search::tori as search;
/// Current bounded interactive Tori defaults, shared by native consumers.
pub use mshogi_search::tori::DEFAULT_LIMITS as TORI_DEFAULT_LIMITS;
/// Maximum supported native Tori search depth.
pub use mshogi_search::tori::MAX_DEPTH as TORI_MAX_DEPTH;
/// Maximum supported native Tori visit budget.
pub use mshogi_search::tori::MAX_NODES as TORI_MAX_NODES;
use serde::{Deserialize, Serialize};
/// Measured finite native Tori interactive work settings.
pub const TORI_PLAY_LIMITS: crate::WorkLimits = crate::WorkLimits {
    depth: TORI_DEFAULT_LIMITS.depth,
    nodes: TORI_DEFAULT_LIMITS.nodes,
};
/// Actual bounded search observations, in native root-side units.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToriTelemetry {
    /// Fixed transparent evaluator identity.
    pub evaluator_id: String,
    /// Fixed deterministic search identity.
    pub search_id: String,
    /// Named analyzed native side.
    pub perspective: Side,
    /// Hundredths of a Swallow, or native terminal action distance; absent until completed work.
    #[serde(deserialize_with = "required_score")]
    pub score: Option<i32>,
    /// Last complete full-width iteration.
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
impl ToriTelemetry {
    /// Native win/loss distance is not necessarily checkmate; heuristic units are Swallows.
    pub fn score_label(&self) -> Option<String> {
        self.score.and_then(|score| {
            let n = i64::from(score).abs();
            if n > i64::from(search::WIN_SCORE) {
                return None;
            }
            Some(
                if n >= i64::from(search::WIN_SCORE) - i64::from(search::MAX_DEPTH) {
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
                    format!("{:+.2} Swallows", f64::from(score) / 100.0)
                },
            )
        })
    }
}
/// Complete native action, genuine conclusion, or no usable completed iteration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ToriResult {
    /// Proposed action; authoritative host admission is still required.
    Choice {
        /// Complete action.
        action: Action,
        /// Actual observations.
        telemetry: ToriTelemetry,
    },
    /// Exact native root conclusion.
    Finished {
        /// Native history-owned outcome.
        conclusion: Conclusion,
        /// Terminal-root observation.
        telemetry: ToriTelemetry,
    },
    /// No completed iteration, fallback move or score.
    Incomplete {
        /// Actual unsuccessful work.
        telemetry: ToriTelemetry,
    },
}
impl ToriResult {
    /// Borrow actual native telemetry without conflating result kinds.
    pub fn telemetry(&self) -> &ToriTelemetry {
        match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry } => telemetry,
        }
    }
    pub(crate) fn searched(r: search::SearchResult) -> Self {
        let telemetry = ToriTelemetry {
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
        let t = self.telemetry();
        if t.evaluator_id != search::EVALUATOR_ID
            || t.search_id != search::SEARCH_ID
            || t.perspective != game.position().side_to_move()
            || t.completed_depth > search::MAX_DEPTH
            || t.nodes > search::MAX_NODES
            || t.score
                .is_some_and(|s| !(-search::WIN_SCORE..=search::WIN_SCORE).contains(&s))
            || t.principal_variation.len() > usize::from(t.completed_depth)
        {
            return Err("Tori telemetry identity, work or score mismatch".into());
        }
        match self {
            Self::Incomplete { .. } => {
                if t.completed_depth != 0
                    || t.score.is_some()
                    || !t.principal_variation.is_empty()
                    || t.status == StopReason::Complete
                {
                    return Err("incomplete Tori work carried a completed result".into());
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
                    return Err("Tori terminal observation disagrees with native root".into());
                }
            }
            Self::Choice { action, .. } => {
                if game.conclusion().is_some()
                    || t.completed_depth == 0
                    || t.nodes < 2 * u64::from(t.completed_depth)
                    || t.score.is_none()
                    || t.principal_variation.first() != Some(action)
                {
                    return Err("Tori choice has no completed live-root iteration".into());
                }
                let mut branch = GameRecord::from_game(game)
                    .and_then(|r| r.to_game())
                    .map_err(|e| e.to_string())?;
                for action in &t.principal_variation {
                    branch
                        .submit(GameRequest {
                            expected: branch.context(),
                            actor: branch.position().side_to_move(),
                            command: GameCommand::Play { action: *action },
                        })
                        .map_err(|e| e.to_string())?;
                }
                if t.principal_variation.len() != usize::from(t.completed_depth)
                    && branch.conclusion().is_none()
                {
                    return Err("Tori continuation ends before its completed depth".into());
                }
                let expected = branch
                    .conclusion()
                    .map(|c| {
                        search::terminal_score(c, t.perspective, t.principal_variation.len() as u16)
                    })
                    .unwrap_or_else(|| {
                        search::evaluate(branch.position()).score_for(t.perspective)
                    });
                if t.score != Some(expected) {
                    return Err("Tori score disagrees with its native continuation".into());
                }
            }
        }
        Ok(())
    }
    pub(crate) fn matches(&self, context: &ExecutionContext) -> bool {
        let CanonicalRecord::Tori(record) = &context.game else {
            return false;
        };
        if !matches!(
            context.purpose,
            ExecutionPurpose::EngineTurn {} | ExecutionPurpose::MoveAnalysis { .. }
        ) {
            return false;
        }
        let t = self.telemetry();
        if !(search::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        })
        .valid()
        {
            return false;
        }
        if t.evaluator_id != search::EVALUATOR_ID
            || t.search_id != search::SEARCH_ID
            || t.perspective != record.side_to_move()
            || t.completed_depth > context.limits.depth
            || t.nodes > context.limits.nodes
            || (t.status == StopReason::NodeLimit && t.nodes != context.limits.nodes)
            || t.principal_variation.len() > usize::from(t.completed_depth)
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
                    && t.nodes >= 2 * u64::from(t.completed_depth)
                    && t.score.is_some()
                    && t.principal_variation.first() == Some(action)
                    && (t.status != StopReason::Complete
                        || t.completed_depth == context.limits.depth)
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
