//! Full-width verifier for unlisted moves in the frozen checking-only Tori suite.
use mshogi_core::tori::{LocalPosition, RepetitionKey};
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

/// Exact identity: board, both hands and side to move. Memo entries are
/// filed under the key's 64-bit digest and confirmed on the full key.
fn key(position: &LocalPosition) -> RepetitionKey {
    RepetitionKey::from_position(position.position())
}

pub struct Prover {
    memo: HashMap<(u64, bool), Vec<(RepetitionKey, Known)>>,
    pub visits: u64,
    cap: u64,
}

impl Prover {
    pub fn full(cap: u64) -> Self {
        Self {
            memo: HashMap::new(),
            visits: 0,
            cap,
        }
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

    fn known(&self, key: &RepetitionKey, attacking: bool) -> Known {
        self.memo
            .get(&(key.digest(), attacking))
            .and_then(|v| v.iter().find(|(k, _)| k == key))
            .map_or(Known::default(), |(_, known)| *known)
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
        let actions = position.legal_actions();
        if !attacking {
            if actions.is_empty() {
                return Ok(true);
            }
            if plies == 0 {
                return Ok(false);
            }
            let key = key(position);
            let known = self.known(&key, false);
            if known.proven.is_some_and(|d| d <= plies) {
                return Ok(true);
            }
            if known.refuted.is_some_and(|d| d >= plies) {
                return Ok(false);
            }
            // Children are built one at a time: one escape settles the node.
            let mut result = true;
            for a in actions {
                let mut next = position.clone();
                next.apply(a).expect("legal action");
                if !self.node(&next, plies - 1, true)? {
                    result = false;
                    break;
                }
            }
            self.remember(key, false, plies, result);
            return Ok(result);
        }
        if plies == 0 {
            return Ok(false);
        }
        let key = key(position);
        let known = self.known(&key, true);
        if known.proven.is_some_and(|d| d <= plies) {
            return Ok(true);
        }
        if known.refuted.is_some_and(|d| d >= plies) {
            return Ok(false);
        }
        // Checks first (they decide most mates), then, in full mode, the
        // quiet actions; children are built one at a time.
        let mut result = false;
        let mut quiet = vec![];
        for a in actions {
            let mut next = position.clone();
            next.apply(a).expect("legal action");
            let side = next.position().side_to_move();
            if !next.is_in_check(side) {
                quiet.push(a);
                continue;
            }
            if self.node(&next, plies - 1, false)? {
                result = true;
                break;
            }
        }
        if !result {
            for a in quiet {
                let mut next = position.clone();
                next.apply(a).expect("legal action");
                if self.node(&next, plies - 1, false)? {
                    result = true;
                    break;
                }
            }
        }
        self.remember(key, true, plies, result);
        Ok(result)
    }

    fn remember(&mut self, key: RepetitionKey, attacking: bool, plies: u8, result: bool) {
        let entries = self.memo.entry((key.digest(), attacking)).or_default();
        let index = match entries.iter().position(|(k, _)| *k == key) {
            Some(i) => i,
            None => {
                entries.push((key, Known::default()));
                entries.len() - 1
            }
        };
        let entry = &mut entries[index].1;
        if result {
            entry.proven = Some(entry.proven.map_or(plies, |d| d.min(plies)));
        } else {
            entry.refuted = Some(entry.refuted.map_or(plies, |d| d.max(plies)));
        }
    }
}
