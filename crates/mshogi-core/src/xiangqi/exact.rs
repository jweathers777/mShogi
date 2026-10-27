//! Intrinsic natural-board reachability, deliberately not a search evaluation cache.
use super::{Action, Counters, Intersection, NoActionReason, Piece, PieceKind, Position, Side};
use serde::{Deserialize, Serialize};
use std::{mem::size_of, time::Instant};

/// Hard ceilings bound allocations, work and diagnostic output even for untrusted callers.
pub const MAX_EXACT_STATES: u32 = 1_000_000;
/// Maximum supported work duration; cancellation is polled between every bounded native operation.
pub const MAX_EXACT_MILLIS: u32 = 60_000;
const SCRATCH_BYTES: usize = 32 * 1024;
const NONE: u32 = u32::MAX;
/// Exact traversal choice; neither traversal is minimax.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExactTraversal {
    /// Shortest witness first; finite exhausted queue proves negative reachability.
    BreadthFirst,
    /// Alternative measured exploration order, with identical proof obligations.
    DepthFirst,
}
/// Explicit finite work, heap-capacity and witness limits. Zero work is allowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactLimits {
    /// Discovered states, including root.
    pub states: u32,
    /// Generated edges, including cache hits.
    pub transitions: u64,
    /// Wall-clock ceiling; timed reruns need not do identical work.
    pub millis: u32,
    /// Verifier-owned heap capacity plus conservative bounded scratch allowance.
    pub memory_bytes: usize,
    /// Maximum returned complete witness actions.
    pub witness_plies: u32,
    /// Deterministic traversal order in the absence of time/cancellation limits.
    pub traversal: ExactTraversal,
}
impl Default for ExactLimits {
    fn default() -> Self {
        Self {
            states: 20_000,
            transitions: 200_000,
            millis: 250,
            memory_bytes: 4 * 1024 * 1024,
            witness_plies: 4096,
            traversal: ExactTraversal::DepthFirst,
        }
    }
}
/// Incomplete work is neither a draw nor a positive capability proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExactStop {
    /// Caller cancelled this computation.
    Cancelled,
    /// Discovered-state bound.
    States,
    /// Generated-edge bound.
    Transitions,
    /// Allocated-capacity bound or allocator refusal.
    Memory,
    /// Wall-clock bound.
    Deadline,
    /// A positive path exceeded the bounded witness output.
    Witness,
}
/// Explicit refusal/invariant failure, not an incomplete proof or chess outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExactFailure {
    /// Unsupported limits, rejected before work/allocation.
    UnsupportedLimits,
    /// Input is outside the admitted local-position domain.
    InvalidPosition,
    /// A native legal action failed its normalized transition.
    TransitionInvariant,
}
/// Measurements are observations, never a score or a promised timed rerun.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactWork {
    /// Unique discovered states.
    pub states: u32,
    /// Fully expanded states, including natural terminal boards.
    pub expanded: u32,
    /// Edges generated from nonterminal boards.
    pub transitions: u64,
    /// Edges reusing an already discovered exact state.
    pub cache_hits: u64,
    /// Actual open-addressed key comparisons.
    pub cache_probes: u64,
    /// Largest local choice set observed.
    pub max_branching: u32,
    /// Owned allocation capacity plus conservative scratch allowance, not process RSS.
    pub memory_bytes: usize,
    /// Observed elapsed microseconds.
    pub elapsed_micros: u64,
}
/// Exact reasoning over the complete reachable graph, or an honest incomplete/failure result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExactVerdict {
    /// Every reachable state expanded, with no natural terminal.
    NeitherCanWin,
    /// Finite ordinary board path reaches a native terminal. No adversarial guarantee.
    NaturalWin {
        /// Actual native winner.
        winner: Side,
        /// Checked versus unchecked terminal loss.
        reason: NoActionReason,
        /// Complete path from submitted target (empty only if target already terminal).
        witness: Vec<Action>,
    },
    /// Graph closure or bounded witness was not completed.
    Unknown {
        /// Precise incomplete-work reason.
        reason: ExactStop,
    },
    /// Execution/input invariant failed.
    Failure {
        /// Precise non-outcome failure.
        reason: ExactFailure,
    },
}
/// Bounded result whose target/purpose/runtime provenance is owned by the native request adapter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactReport {
    /// Actual verdict; fast recognition is not substituted for exact execution.
    pub verdict: ExactVerdict,
    /// Actual work and capacity observations.
    pub work: ExactWork,
}
// 90 nibbles encode empty/7 native kinds x2 sides; byte45 is side-to-move.
// No material hashing collisions, symmetry quotient or history-sensitive score reuse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Key([u8; 46]);
impl Key {
    fn of(p: &Position) -> Self {
        let mut bytes = [0; 46];
        for (at, piece) in p.pieces() {
            let code = 1 + piece.kind.index() as u8 + 7 * piece.side.index() as u8;
            bytes[at.index() / 2] |= code << (4 * (at.index() % 2));
        }
        bytes[45] = p.side.index() as u8;
        Self(bytes)
    }
    fn position(self) -> Position {
        const KINDS: [PieceKind; 7] = [
            PieceKind::General,
            PieceKind::Advisor,
            PieceKind::Elephant,
            PieceKind::Horse,
            PieceKind::Rook,
            PieceKind::Cannon,
            PieceKind::Soldier,
        ];
        let mut board = [None; 90];
        for (i, slot) in board.iter_mut().enumerate() {
            let code = (self.0[i / 2] >> (4 * (i % 2))) & 15;
            if code != 0 {
                *slot = Some(Piece::new(
                    if code <= 7 { Side::Red } else { Side::Black },
                    KINDS[(code as usize - 1) % 7],
                ));
            }
        }
        // Keys are private, exclusively produced from admitted positions.
        Position {
            board,
            side: if self.0[45] == 0 {
                Side::Red
            } else {
                Side::Black
            },
            counters: Counters::unknown(),
        }
    }
    fn hash(self) -> usize {
        let mut hash = 0xcbf29ce484222325_u64;
        for byte in self.0 {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
        hash as usize
    }
}
struct Node {
    key: Key,
    parent: u32,
    action: Action,
}

impl Position {
    /// Explore the finite intrinsic board graph. History penalties, count claims,
    /// counters and voluntary/administrative commands are not proof edges/terminals.
    /// Never mutates the submitted position. Each native move-generation/transition
    /// call is bounded by the 90-intersection/32-piece board; polling surrounds them.
    pub fn verify_natural_win(
        &self,
        limits: ExactLimits,
        mut cancelled: impl FnMut() -> bool,
    ) -> ExactReport {
        let started = Instant::now();
        let mut work = ExactWork::default();
        let finish = |verdict, mut work: ExactWork| {
            work.elapsed_micros = started.elapsed().as_micros().min(u128::from(u64::MAX)) as u64;
            ExactReport { verdict, work }
        };
        macro_rules! unknown {
            ($r:expr) => {
                return finish(ExactVerdict::Unknown { reason: $r }, work)
            };
        }
        macro_rules! failure {
            ($r:expr) => {
                return finish(ExactVerdict::Failure { reason: $r }, work)
            };
        }
        macro_rules! poll {
            () => {
                if cancelled() {
                    unknown!(ExactStop::Cancelled);
                }
                if started.elapsed().as_millis() >= u128::from(limits.millis) {
                    unknown!(ExactStop::Deadline);
                }
            };
        }
        if limits.states > MAX_EXACT_STATES
            || limits.transitions > 20_000_000
            || limits.millis > MAX_EXACT_MILLIS
            || limits.memory_bytes > 128 * 1024 * 1024
            || limits.witness_plies > 4096
        {
            failure!(ExactFailure::UnsupportedLimits);
        }
        poll!();
        if self.validate_local().is_err() {
            failure!(ExactFailure::InvalidPosition);
        }
        if limits.states == 0 {
            unknown!(ExactStop::States);
        }
        let count = limits.states as usize;
        let slots = (count * 2).next_power_of_two();
        let required = SCRATCH_BYTES
            + count * (size_of::<Node>() + size_of::<u32>())
            + slots * size_of::<u32>()
            + limits.witness_plies as usize * size_of::<Action>();
        if required > limits.memory_bytes {
            unknown!(ExactStop::Memory);
        }
        let mut nodes = Vec::<Node>::new();
        let mut table = Vec::<u32>::new();
        let mut frontier = Vec::<u32>::new();
        let mut witness = Vec::<Action>::new();
        if nodes.try_reserve_exact(count).is_err()
            || table.try_reserve_exact(slots).is_err()
            || frontier.try_reserve_exact(count).is_err()
            || witness
                .try_reserve_exact(limits.witness_plies as usize)
                .is_err()
        {
            unknown!(ExactStop::Memory);
        }
        work.memory_bytes = SCRATCH_BYTES
            + nodes.capacity() * size_of::<Node>()
            + (table.capacity() + frontier.capacity()) * size_of::<u32>()
            + witness.capacity() * size_of::<Action>();
        if work.memory_bytes > limits.memory_bytes {
            unknown!(ExactStop::Memory);
        }
        table.resize(slots, NONE);
        let root = Key::of(self);
        let dummy = Action {
            from: Intersection::new(0, 0).unwrap(),
            to: Intersection::new(0, 0).unwrap(),
        };
        table[root.hash() & (slots - 1)] = 0;
        nodes.push(Node {
            key: root,
            parent: NONE,
            action: dummy,
        });
        frontier.push(0);
        work.states = 1;
        let mut head = 0;
        loop {
            poll!();
            let index = match limits.traversal {
                ExactTraversal::BreadthFirst => {
                    if head == frontier.len() {
                        break;
                    }
                    let index = frontier[head];
                    head += 1;
                    index
                }
                ExactTraversal::DepthFirst => match frontier.pop() {
                    Some(index) => index,
                    None => break,
                },
            };
            let position = nodes[index as usize].key.position();
            let Ok(actions) = position.legal_actions() else {
                failure!(ExactFailure::TransitionInvariant);
            };
            poll!();
            work.expanded += 1;
            work.max_branching = work.max_branching.max(actions.len() as u32);
            if actions.is_empty() {
                let mut cursor = index;
                while nodes[cursor as usize].parent != NONE {
                    poll!();
                    if witness.len() >= limits.witness_plies as usize {
                        unknown!(ExactStop::Witness);
                    }
                    witness.push(nodes[cursor as usize].action);
                    cursor = nodes[cursor as usize].parent;
                }
                witness.reverse();
                return finish(
                    ExactVerdict::NaturalWin {
                        winner: position.side.opponent(),
                        reason: if position.is_in_check(position.side) {
                            NoActionReason::Checkmate
                        } else {
                            NoActionReason::Stalemate
                        },
                        witness,
                    },
                    work,
                );
            }
            for action in actions {
                poll!();
                if work.transitions == limits.transitions {
                    unknown!(ExactStop::Transitions);
                }
                work.transitions += 1;
                let mut after = position.clone();
                if after.apply(action).is_err() {
                    failure!(ExactFailure::TransitionInvariant);
                }
                let key = Key::of(&after);
                let mut slot = key.hash() & (slots - 1);
                while table[slot] != NONE {
                    poll!();
                    work.cache_probes += 1;
                    if nodes[table[slot] as usize].key == key {
                        break;
                    }
                    slot = (slot + 1) & (slots - 1);
                }
                if table[slot] != NONE {
                    work.cache_hits += 1;
                    continue;
                }
                if nodes.len() == count {
                    unknown!(ExactStop::States);
                }
                let next = nodes.len() as u32;
                table[slot] = next;
                nodes.push(Node {
                    key,
                    parent: index,
                    action,
                });
                frontier.push(next);
                work.states += 1;
            }
        }
        finish(ExactVerdict::NeitherCanWin, work)
    }
}
