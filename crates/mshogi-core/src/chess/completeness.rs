//! Exact cooperative Article-3 reachability. Limits never prove impossibility.
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Operational work limits, independent of chess rules. None is unlimited.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReachabilityLimits {
    /// Maximum distinct positions retained, including the root.
    pub max_positions: Option<u64>,
    /// Maximum generated legal transitions examined.
    pub max_transitions: Option<u64>,
}
/// Recorded live-adjudication configuration. Legacy constructors retain M2.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum AdjudicationMode {
    /// Sound material subset; other cases remain unknown.
    #[default]
    M2,
    /// Exact finite graph algorithm; limits can interrupt it with Unknown.
    Full {
        /// Reproducible work limits. Unlimited requires potentially huge resources.
        limits: ReachabilityLimits,
    },
}
/// Which cooperative mating target is requested.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MateTarget {
    /// A mate delivered by this side.
    Side(Color),
    /// A mate delivered by either side; impossibility establishes global death.
    Either,
}
/// Why no decision was established.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReachabilityStop {
    /// Caller cancellation, checked between positions and transitions.
    Cancelled,
    /// Distinct-position budget exhausted.
    PositionLimit,
    /// Legal-transition budget exhausted.
    TransitionLimit,
    /// Work counters or platform address space cannot represent further work.
    ResourceLimit,
}
/// Exact proof result, or explicitly incomplete work.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum MateReachability {
    /// Checked complete actions ending in the requested mate.
    Possible {
        /// Cooperative witness, never played into the live game.
        witness: Vec<Action>,
        /// Reproducible work performed.
        work: ReachabilityWork,
    },
    /// Every reachable vertex was exhausted, with only sound proof pruning.
    Impossible {
        /// Reproducible exhaustion evidence, including static-proof leaf count.
        work: ReachabilityWork,
    },
    /// No chess score follows from unfinished exploration.
    Unknown {
        /// Resource/cancellation cause.
        reason: ReachabilityStop,
        /// Reproducible completed work.
        work: ReachabilityWork,
    },
}
/// Deterministic work observations, not timing or a trusted proof certificate.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReachabilityWork {
    /// Distinct exact identities discovered.
    pub positions: u64,
    /// Vertices classified/expanded completely or in progress.
    pub expanded: u64,
    /// Legal transitions examined.
    pub transitions: u64,
    /// Leaves proved impossible by bare-king/M2 material proofs.
    pub static_leaves: u64,
}
fn normalized(mut p: Position) -> Position {
    // Article-3 geometry is independent of counters and optional claim policy.
    p.halfmove_clock = 0;
    p.fullmove_number = 1;
    p
}
fn bare(position: &Position, side: Color) -> bool {
    !(0..64).any(|i| {
        position
            .piece_at(Square::new(i % 8, i / 8).unwrap())
            .is_some_and(|p| p.color == side && p.kind != PieceKind::King)
    })
}
fn static_impossible(p: &Position, target: MateTarget) -> bool {
    p.dead_recognition() == DeadRecognition::ProvenDead
        || matches!(target, MateTarget::Side(side) if bare(p, side))
}
struct Vertex {
    position: Position,
    parent: Option<(usize, Action)>,
}
impl Position {
    /// Decide cooperative mating reachability with exact collision-checked keys.
    /// Both players cooperate; no minimax, policy claims, clocks or future
    /// 50/75/repetition conclusions are executed. Checkmate/stalemate stop a path.
    /// Unlimited exploration is complete on the finite Article-3 state graph,
    /// given sufficient resources. Finite limits/cancellation always return
    /// Unknown unless an actual witness or exhaustive impossibility was found.
    /// Breadth-first order uses ascending coordinate strings, including promotions.
    pub fn mate_reachability(
        &self,
        target: MateTarget,
        limits: ReachabilityLimits,
        mut cancelled: impl FnMut() -> bool,
    ) -> MateReachability {
        let mut work = ReachabilityWork::default();
        let stop = |reason, work| MateReachability::Unknown { reason, work };
        if cancelled() {
            return stop(ReachabilityStop::Cancelled, work);
        }
        if limits.max_positions == Some(0) {
            return stop(ReachabilityStop::PositionLimit, work);
        }
        let root = normalized(self.clone());
        let mut seen = HashMap::from([(root.repetition_identity(), 0usize)]);
        let mut vertices = vec![Vertex {
            position: root,
            parent: None,
        }];
        work.positions = 1;
        let mut cursor = 0;
        while cursor < vertices.len() {
            if cancelled() {
                return stop(ReachabilityStop::Cancelled, work);
            }
            let p = &vertices[cursor].position;
            work.expanded += 1;
            let mut actions = p.legal_actions();
            if actions.is_empty() {
                if p.in_check()
                    && match target {
                        MateTarget::Either => true,
                        MateTarget::Side(c) => p.side_to_move().opponent() == c,
                    }
                {
                    let mut witness = Vec::new();
                    let mut node = cursor;
                    while let Some((parent, action)) = vertices[node].parent {
                        witness.push(action);
                        node = parent;
                    }
                    witness.reverse();
                    return MateReachability::Possible { witness, work };
                }
                cursor += 1;
                continue;
            }
            if static_impossible(p, target) {
                work.static_leaves += 1;
                cursor += 1;
                continue;
            }
            actions.sort_by_key(|a| a.to_string());
            for action in actions {
                if cancelled() {
                    return stop(ReachabilityStop::Cancelled, work);
                }
                if limits
                    .max_transitions
                    .is_some_and(|n| work.transitions >= n)
                {
                    return stop(ReachabilityStop::TransitionLimit, work);
                }
                let Some(transitions) = work.transitions.checked_add(1) else {
                    return stop(ReachabilityStop::ResourceLimit, work);
                };
                work.transitions = transitions;
                let mut next = vertices[cursor].position.clone();
                next.apply(action)
                    .expect("normalized legal transition cannot overflow");
                next = normalized(next);
                let key = next.repetition_identity();
                if seen.contains_key(&key) {
                    continue;
                }
                if limits.max_positions.is_some_and(|n| work.positions >= n) {
                    return stop(ReachabilityStop::PositionLimit, work);
                }
                if seen.try_reserve(1).is_err() || vertices.try_reserve(1).is_err() {
                    return stop(ReachabilityStop::ResourceLimit, work);
                }
                seen.insert(key, vertices.len());
                vertices.push(Vertex {
                    position: next,
                    parent: Some((cursor, action)),
                });
                work.positions += 1;
            }
            cursor += 1;
        }
        MateReachability::Impossible { work }
    }
}
