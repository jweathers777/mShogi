//! Reference checking-mate prover over the native Shogi rules.
//!
//! This is the answer source of the tactical suites. It uses only
//! `mshogi_core::shogi::Position` (native legality, including king safety and
//! the Pawn-drop-mate rule) and is independent of the search crate. The
//! attacker plays only checking moves; the defender plays every legal reply.
//! The defender loses when it is in check and has no legal action.
//!
//! Roots are diagrams without earlier history. A minimal mate of at most 11
//! plies cannot contain a fourfold repetition (it would need three complete
//! cycles of at least four plies each, and removing a cycle gives a shorter
//! mate), so repetition does not change the answer within this bound.
use mshogi_core::shogi::{Action, Position, RepetitionIdentity};
use std::collections::HashMap;

/// Largest supported proof length in plies.
#[allow(dead_code)]
pub const MAX_PROOF_PLIES: u8 = 11;

/// The node bound was exhausted: the result is unknown, not a failed tactic.
#[derive(Debug)]
pub struct Exhausted;

#[derive(Clone, Copy, Default)]
struct Known {
    /// Smallest remaining depth at which a mate is proven.
    proven: Option<u8>,
    /// Largest remaining depth at which no mate exists.
    refuted: Option<u8>,
}

/// Memoized bounded AND-OR proof search.
pub struct Prover {
    memo: HashMap<(RepetitionIdentity, bool), Known>,
    /// Node visits so far.
    pub visits: u64,
    /// Visit bound.
    pub cap: u64,
}

/// Legal actions of the side to move with their native successors. Each
/// pseudo-legal action is admitted by native `apply` exactly once.
pub fn successors(position: &Position, only_checks: bool) -> Vec<(Action, Position)> {
    position
        .pseudo_legal_actions()
        .into_iter()
        .filter_map(|a| {
            let mut next = position.clone();
            next.apply(a).ok()?;
            (!only_checks || next.is_in_check(next.side_to_move())).then_some((a, next))
        })
        .collect()
}

impl Prover {
    /// New prover with a visit bound.
    pub fn new(cap: u64) -> Self {
        Self {
            memo: HashMap::new(),
            visits: 0,
            cap,
        }
    }

    /// True if the side to move mates by checks within `plies` (odd) plies.
    #[allow(dead_code)]
    pub fn attacker_mates(&mut self, position: &Position, plies: u8) -> Result<bool, Exhausted> {
        self.node(position, plies, true)
    }

    /// True if every legal reply of the side to move (in check) is mated
    /// within `plies` further plies.
    pub fn defender_mated(&mut self, position: &Position, plies: u8) -> Result<bool, Exhausted> {
        self.node(position, plies, false)
    }

    fn node(&mut self, position: &Position, plies: u8, attacking: bool) -> Result<bool, Exhausted> {
        self.visits += 1;
        if self.visits > self.cap {
            return Err(Exhausted);
        }
        if !attacking {
            let replies = successors(position, false);
            if replies.is_empty() {
                return Ok(position.is_in_check(position.side_to_move()));
            }
            if plies == 0 {
                return Ok(false);
            }
            let key = (position.repetition_identity(), false);
            let known = self.memo.get(&key).copied().unwrap_or_default();
            if known.proven.is_some_and(|d| d <= plies) {
                return Ok(true);
            }
            if known.refuted.is_some_and(|d| d >= plies) {
                return Ok(false);
            }
            let mut result = true;
            for (_, next) in replies {
                if !self.node(&next, plies - 1, true)? {
                    result = false;
                    break;
                }
            }
            self.remember(key, plies, result);
            return Ok(result);
        }
        if plies == 0 {
            return Ok(false);
        }
        let key = (position.repetition_identity(), true);
        let known = self.memo.get(&key).copied().unwrap_or_default();
        if known.proven.is_some_and(|d| d <= plies) {
            return Ok(true);
        }
        if known.refuted.is_some_and(|d| d >= plies) {
            return Ok(false);
        }
        let mut result = false;
        for (_, next) in successors(position, true) {
            if self.node(&next, plies - 1, false)? {
                result = true;
                break;
            }
        }
        self.remember(key, plies, result);
        Ok(result)
    }

    fn remember(&mut self, key: (RepetitionIdentity, bool), plies: u8, result: bool) {
        let entry = self.memo.entry(key).or_default();
        if result {
            entry.proven = Some(entry.proven.map_or(plies, |d| d.min(plies)));
        } else {
            entry.refuted = Some(entry.refuted.map_or(plies, |d| d.max(plies)));
        }
    }

    /// All checking first moves that mate within `plies` plies.
    pub fn answer_set(&mut self, position: &Position, plies: u8) -> Result<Vec<Action>, Exhausted> {
        let mut found = vec![];
        for (a, next) in successors(position, true) {
            if self.defender_mated(&next, plies - 1)? {
                found.push(a);
            }
        }
        Ok(found)
    }

    /// Shortest odd mate length up to `max_plies`, or None if none exists.
    #[allow(dead_code)]
    pub fn shortest(
        &mut self,
        position: &Position,
        max_plies: u8,
    ) -> Result<Option<u8>, Exhausted> {
        for plies in (1..=max_plies).step_by(2) {
            if self.attacker_mates(position, plies)? {
                return Ok(Some(plies));
            }
        }
        Ok(None)
    }
}
