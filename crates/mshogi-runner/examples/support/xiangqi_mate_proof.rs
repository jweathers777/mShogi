//! Reference checking-mate prover over the native Xiangqi rules.
//!
//! This is the answer source of the Xiangqi tactical suite. It uses only
//! `mshogi_core::xiangqi::Position` (native legality, including General
//! safety and facing Generals) and is independent of the search crate. The
//! attacker plays only checking moves; the defender plays every legal reply.
//! The defender loses when it has no legal action (checkmate; stalemate is
//! also a loss in Xiangqi, but a checked defender is always mated here).
//!
//! Roots are diagrams without earlier history. Native rules act on a
//! repetition only from its third occurrence (warning or pending
//! adjudication) and decide it from the fourth. A cycle needs at least four
//! plies, so a line of at most seven plies has at most two occurrences of any
//! position, and repetition cannot change an answer within this bound.
use mshogi_core::xiangqi::{Action, Position};
use std::collections::HashMap;

/// Largest supported proof length in plies.
pub const MAX_PROOF_PLIES: u8 = 7;

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

/// Board and side to move; counters do not affect a short mate.
fn key(position: &Position) -> String {
    let fen = position.to_fen().expect("native position");
    fen.split(' ').take(2).collect::<Vec<_>>().join(" ")
}

/// Memoized bounded AND-OR proof search.
pub struct Prover {
    memo: HashMap<(String, bool), Known>,
    /// Node visits so far.
    pub visits: u64,
    /// Visit bound.
    pub cap: u64,
}

/// Legal actions of the side to move with their native successors.
pub fn successors(position: &Position, only_checks: bool) -> Vec<(Action, Position)> {
    position
        .legal_actions()
        .expect("native position")
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
    pub fn attacker_mates(&mut self, position: &Position, plies: u8) -> Result<bool, Exhausted> {
        assert!(plies <= MAX_PROOF_PLIES);
        self.node(position, plies, true)
    }

    /// True if every legal reply of the side to move is mated within `plies`
    /// further plies.
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
                return Ok(true);
            }
            if plies == 0 {
                return Ok(false);
            }
            let key = (key(position), false);
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
        let key = (key(position), true);
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

    fn remember(&mut self, key: (String, bool), plies: u8, result: bool) {
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
    pub fn shortest(&mut self, position: &Position, max_plies: u8) -> Result<Option<u8>, Exhausted> {
        for plies in (1..=max_plies).step_by(2) {
            if self.attacker_mates(position, plies)? {
                return Ok(Some(plies));
            }
        }
        Ok(None)
    }
}

/// Plain recursion without memo, for cross-checking the prover in tests.
#[allow(dead_code)]
pub fn plain_defender_mated(position: &Position, plies: u8) -> bool {
    let replies = successors(position, false);
    if replies.is_empty() {
        return true;
    }
    if plies == 0 {
        return false;
    }
    replies.iter().all(|(_, next)| {
        plies >= 2
            && successors(next, true)
                .iter()
                .any(|(_, n)| plain_defender_mated(n, plies - 2))
    })
}
