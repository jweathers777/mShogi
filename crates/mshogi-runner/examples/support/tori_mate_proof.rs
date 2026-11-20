//! Reference mate prover over the native local Tori rules (MSH-204, Tori
//! Strength Harness).
//!
//! This is the answer source of the Tori tactical suite. It uses only
//! `mshogi_core::tori::LocalPosition` (native local legality, including
//! Phoenix safety, drops, compulsory promotion and the Swallow-drop rules) and
//! is independent of the search crate. In checking mode the attacker plays only
//! checking actions, drops included (used to find positions); in full mode it
//! plays every legal action (used for complete answer sets). The defender
//! plays every legal reply and loses when it has none: checkmate or quiet
//! immobility, both losses in Tori (T05).
//!
//! Roots are diagrams without earlier history. A position occurs at most once
//! more within four plies, so a line of at most seven plies has at most two
//! occurrences of any position; the repetition rule (T06, refusal of a third
//! occurrence) cannot change an answer within this bound.
use mshogi_core::tori::{Action, LocalPosition, RepetitionKey};
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

/// Exact identity: board, both hands and side to move (Debug form of the key).
fn key(position: &LocalPosition) -> String {
    format!("{:?}", RepetitionKey::from_position(position.position()))
}

/// Legal actions of the side to move with their native successors.
pub fn successors(position: &LocalPosition, only_checks: bool) -> Vec<(Action, LocalPosition)> {
    position
        .legal_actions()
        .into_iter()
        .filter_map(|a| {
            let mut next = position.clone();
            next.apply(a).ok()?;
            let side = next.position().side_to_move();
            (!only_checks || next.is_in_check(side)).then_some((a, next))
        })
        .collect()
}

/// Memoized bounded AND-OR proof search.
pub struct Prover {
    memo: HashMap<(String, bool), Known>,
    /// Attacker plays every legal action instead of only checks.
    pub full: bool,
    /// Node visits so far.
    pub visits: u64,
    /// Visit bound.
    pub cap: u64,
}

impl Prover {
    /// New checking prover with a visit bound.
    pub fn new(cap: u64) -> Self {
        Self {
            memo: HashMap::new(),
            full: false,
            visits: 0,
            cap,
        }
    }

    /// New full-width prover: the attacker may play any legal action.
    pub fn full(cap: u64) -> Self {
        Self {
            full: true,
            ..Self::new(cap)
        }
    }

    /// True if the side to move wins within `plies` (odd) plies.
    pub fn attacker_mates(
        &mut self,
        position: &LocalPosition,
        plies: u8,
    ) -> Result<bool, Exhausted> {
        assert!(plies <= MAX_PROOF_PLIES);
        self.node(position, plies, true)
    }

    /// True if every legal reply of the side to move loses within `plies`
    /// further plies (or it has no legal reply).
    pub fn defender_mated(
        &mut self,
        position: &LocalPosition,
        plies: u8,
    ) -> Result<bool, Exhausted> {
        self.node(position, plies, false)
    }

    fn node(
        &mut self,
        position: &LocalPosition,
        plies: u8,
        attacking: bool,
    ) -> Result<bool, Exhausted> {
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
        for (_, next) in successors(position, !self.full) {
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

    /// All first actions (checking only, unless full) that win within `plies`.
    pub fn answer_set(
        &mut self,
        position: &LocalPosition,
        plies: u8,
    ) -> Result<Vec<Action>, Exhausted> {
        let mut found = vec![];
        for (a, next) in successors(position, !self.full) {
            if self.defender_mated(&next, plies - 1)? {
                found.push(a);
            }
        }
        Ok(found)
    }

    /// Shortest odd win length up to `max_plies`, or None if none exists.
    pub fn shortest(
        &mut self,
        position: &LocalPosition,
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

/// Plain recursion without memo (checking attacker), for cross-checking.
#[allow(dead_code)]
pub fn plain_defender_mated(position: &LocalPosition, plies: u8) -> bool {
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
