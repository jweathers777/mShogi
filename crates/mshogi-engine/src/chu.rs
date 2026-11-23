//! Actual native Chu results. No declaration, chess mate or exact-proof adaptation.
use crate::observation_identity::{Identity, Profile, SearchKind};
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
    fn identity(&self) -> Option<Identity> {
        Identity::classify(
            &self.evaluator_id,
            &self.search_id,
            Profile {
                evaluator: search::EVALUATOR_ID,
                search: search::SEARCH_ID,
                max_depth: search::MAX_DEPTH,
                max_nodes: search::MAX_NODES,
                win_score: search::WIN_SCORE,
                line_bound: search::MAX_LINE,
            },
            Profile {
                evaluator: search::LEGACY_EVALUATOR_ID,
                search: search::LEGACY_SEARCH_ID,
                max_depth: search::LEGACY_MAX_DEPTH,
                max_nodes: search::LEGACY_MAX_NODES,
                win_score: search::LEGACY_WIN_SCORE,
                line_bound: 0,
            },
        )
    }
    /// Native win/loss distance is not necessarily checkmate; heuristic units are Pawns.
    pub fn score_label(&self) -> Option<String> {
        let identity = self.identity()?;
        let win_score = identity.profile.win_score;
        let win_bound = match identity.kind {
            SearchKind::SharedCore => mshogi_search::alphabeta::MATE_BOUND,
            SearchKind::FullWidth => win_score - i32::from(identity.max_depth()),
        };
        self.score.and_then(|score| {
            let n = i64::from(score).abs();
            if n > i64::from(win_score) {
                return None;
            }
            Some(if n >= i64::from(win_bound) {
                let d = i64::from(win_score) - n;
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
        self.validate_with_work(game, &mut || false).map(|_| ())
    }
    /// Validate native continuation under an admission worker's stop/deadline.
    /// Returns actual native PV transition work; the admitted history is copied, not replayed;
    /// the document owner must charge these units to its aggregate budget.
    pub fn validate_with_work(
        &self,
        game: &Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<u64, String> {
        if stop() {
            return Err("Chu observation validation stopped".into());
        }
        let mut work = 0u64;
        let t = self.telemetry();
        let Some(identity) = t.identity() else {
            return Err("Chu telemetry identity, work or score mismatch".into());
        };
        if t.perspective != game.position().side_to_move()
            || t.completed_depth > identity.max_depth()
            || t.nodes > identity.max_nodes()
            || t.score
                .is_some_and(|s| !(-search::WIN_SCORE..=search::WIN_SCORE).contains(&s))
            || t.principal_variation.len() > identity.max_line(t.completed_depth)
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
                    || t.nodes < identity.least_nodes(t.completed_depth)
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
                    let before = branch.replay_work();
                    branch
                        .submit(GameRequest {
                            expected: branch.context(),
                            actor: branch.position().side_to_move(),
                            command: GameCommand::Play {
                                action: action.clone(),
                            },
                        })
                        .map_err(|e| e.to_string())?;
                    work = work
                        .checked_add(branch.replay_work().saturating_sub(before))
                        .ok_or_else(|| "Chu observation validation work overflow".to_string())?;
                }
                if stop() {
                    return Err("Chu observation validation stopped".into());
                }
                // Selective search scores are not the static value at the end
                // of a bounded reported line. Native replay above remains mandatory.
                if identity.kind == SearchKind::SharedCore {
                    return if t
                        .score
                        .is_some_and(|s| s.unsigned_abs() >= search::WIN_SCORE as u32)
                    {
                        Err("Chu choice cannot be a win at the analyzed root".into())
                    } else {
                        Ok(work)
                    };
                }
                if t.principal_variation.len() != usize::from(t.completed_depth)
                    && branch.conclusion().is_none()
                {
                    return Err("Chu continuation ends before its completed depth".into());
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
                    return Err("Chu score disagrees with its native continuation".into());
                }
            }
        }
        Ok(work)
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
        let Some(identity) = t.identity() else {
            return false;
        };
        if context.limits.depth == 0
            || context.limits.depth > identity.max_depth()
            || context.limits.nodes > identity.max_nodes()
        {
            return false;
        }
        if t.perspective != record.side_to_move()
            || t.completed_depth > context.limits.depth
            || t.nodes > context.limits.nodes
            || (t.status == StopReason::NodeLimit && t.nodes != context.limits.nodes)
            || t.principal_variation.len() > identity.max_line(t.completed_depth)
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
                    && t.nodes >= identity.least_nodes(t.completed_depth)
                    && t.score.is_some()
                    && t.principal_variation.first() == Some(action)
                    && (identity.kind == SearchKind::SharedCore
                        || t.status != StopReason::Complete
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

#[cfg(test)]
mod identity_tests {
    use super::*;
    #[test]
    fn historical_identity_bounds_and_selective_distance_labels_are_distinct() {
        let game = Game::standard();
        let mut context = ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            process_generation: 1,
            game: CanonicalRecord::chu(mshogi_core::chu::GameRecord::from_game(&game).unwrap()),
            limits: crate::WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 512,
            },
        };
        let mut telemetry = ChuTelemetry {
            evaluator_id: search::LEGACY_EVALUATOR_ID.into(),
            search_id: search::LEGACY_SEARCH_ID.into(),
            perspective: Side::Sente,
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::Stopped,
        };
        let old = ChuResult::Incomplete {
            telemetry: telemetry.clone(),
        };
        assert!(old.matches(&context));
        old.validate(&game).unwrap();
        context.limits = CHU_PLAY_LIMITS;
        assert!(!old.matches(&context));
        telemetry.evaluator_id = search::EVALUATOR_ID.into();
        assert!(
            !ChuResult::Incomplete {
                telemetry: telemetry.clone()
            }
            .matches(&context)
        );
        telemetry.search_id = search::SEARCH_ID.into();
        assert!(
            ChuResult::Incomplete {
                telemetry: telemetry.clone()
            }
            .matches(&context)
        );
        telemetry.score = Some(search::WIN_SCORE - 20);
        assert_eq!(
            telemetry.score_label().as_deref(),
            Some("Win in 20 board actions")
        );
        telemetry.score = Some(i32::MIN);
        assert_eq!(telemetry.score_label(), None);
    }
}
