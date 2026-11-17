//! Game-independent mate search by continuous checks (tsume).
//!
//! Depth-first proof-number search (df-pn, Nagai 2002) with the 1+epsilon
//! threshold rule. The attacker plays only moves that give check; the
//! defender plays every legal reply. The defender is mated when it is in
//! check and has no legal move. The game supplies checking-move candidates
//! (`SearchPosition::generate_checks`, a superset is allowed) and evasions
//! (its ordinary move generation); `make` decides legality, so a game rule
//! such as the Shogi Pawn-drop mate is respected by construction.
//!
//! A position that a game rule already decides (`rule_score`), including a
//! repetition or a perpetual-check cycle, is not a mate. Such results depend
//! on the path, so they are kept out of the table. A proof is private
//! analysis; the native rules still admit the move and decide the result.
use super::{SearchPosition, Stage};
use std::time::Instant;

/// Proof or disproof number that stands for infinity.
const INFINITE: u32 = 1 << 30;
/// Longest checking sequence considered, in plies.
pub const MAX_MATE_PLY: usize = 64;

#[derive(Clone, Copy, Default)]
struct Entry {
    key: u64,
    pn: u32,
    dn: u32,
    /// For a proven node: plies to mate along the stored proof.
    len: u32,
}

/// Fixed-size proof table in buckets of four entries. A solved entry
/// (proven or disproven) is replaced only by its own key or when the whole
/// bucket is solved, so a proof tree is not lost to unrelated work.
pub struct MateTable {
    entries: Vec<Entry>,
    mask: usize,
}

const BUCKET: usize = 4;

impl MateTable {
    /// Table with `2^bits` entries (at least one bucket).
    pub fn new(bits: u32) -> Self {
        let size = (1usize << bits).max(BUCKET);
        Self {
            entries: vec![Entry::default(); size],
            mask: size / BUCKET - 1,
        }
    }
    /// Forget all entries.
    pub fn clear(&mut self) {
        self.entries.fill(Entry::default());
    }
    fn bucket(&self, key: u64) -> std::ops::Range<usize> {
        let first = (key as usize & self.mask) * BUCKET;
        first..first + BUCKET
    }
    fn find(&self, key: u64) -> Option<Entry> {
        if key == 0 {
            return None;
        }
        self.entries[self.bucket(key)]
            .iter()
            .copied()
            .find(|e| e.key == key)
    }
    fn get(&self, key: u64) -> (u32, u32) {
        self.find(key).map_or((1, 1), |e| (e.pn, e.dn))
    }
    /// Proof length of a proven entry, if stored.
    fn len(&self, key: u64) -> Option<u32> {
        self.find(key).filter(|e| e.pn == 0).map(|e| e.len)
    }
    fn put(&mut self, key: u64, pn: u32, dn: u32, len: u32) {
        if key == 0 {
            return;
        }
        let range = self.bucket(key);
        let bucket = &mut self.entries[range];
        let solved = |e: &Entry| e.key != 0 && (e.pn == 0 || e.dn == 0);
        let slot = bucket
            .iter()
            .position(|e| e.key == key)
            .or_else(|| bucket.iter().position(|e| e.key == 0))
            .or_else(|| {
                // The unsolved entry with the least work invested.
                (0..BUCKET)
                    .filter(|&i| !solved(&bucket[i]))
                    .min_by_key(|&i| bucket[i].pn.saturating_add(bucket[i].dn))
            });
        let slot = match slot {
            Some(slot) => slot,
            // Every entry is solved: an unsolved result does not evict one.
            None if pn != 0 && dn != 0 => return,
            None => 0,
        };
        bucket[slot] = Entry { key, pn, dn, len };
    }
}

/// Result of one mate search.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MateResult<M> {
    /// Forced mate; the sequence starts with the attacker's move and ends
    /// with the defender mated. It follows one proven line, not necessarily
    /// the shortest one.
    Mate(Vec<M>),
    /// No mate by continuous checks exists (within the ply bound).
    NoMate,
    /// The node or time bound stopped the search.
    Unknown,
}

/// Work bounds of one mate search.
#[derive(Clone, Copy, Debug)]
pub struct MateLimits {
    /// Node visits at most.
    pub nodes: u64,
    /// Stop at this instant.
    pub deadline: Option<Instant>,
}

#[derive(Clone, Copy)]
struct Child<M> {
    m: M,
    key: u64,
    /// The successor is decided by a game rule (no mate there).
    ruled: bool,
}

struct Mate<'a, P: SearchPosition> {
    pos: &'a mut P,
    table: &'a mut MateTable,
    limits: MateLimits,
    stop: &'a mut dyn FnMut() -> bool,
    nodes: u64,
    abort: bool,
    moves: Vec<P::Move>,
}

fn add(a: u32, b: u32) -> u32 {
    a.saturating_add(b).min(INFINITE)
}

impl<P: SearchPosition> Mate<'_, P> {
    fn exhausted(&mut self) -> bool {
        if self.abort {
            return true;
        }
        if self.nodes >= self.limits.nodes
            || (self.nodes & 1023 == 0
                && ((self.stop)() || self.limits.deadline.is_some_and(|d| Instant::now() >= d)))
        {
            self.abort = true;
        }
        self.abort
    }

    /// Legal successors: checking moves for the attacker, all moves for the
    /// defender.
    fn children(&mut self, attacker: bool) -> Vec<Child<P::Move>> {
        let mut moves = std::mem::take(&mut self.moves);
        moves.clear();
        if attacker {
            self.pos.generate_checks(&mut moves);
        } else {
            self.pos.generate_search(Stage::Tactical, &mut moves);
            self.pos.generate_search(Stage::Quiet, &mut moves);
        }
        let mut children = Vec::with_capacity(moves.len());
        for &m in &moves {
            if !self.pos.make(m) {
                continue;
            }
            if !attacker || self.pos.in_check() {
                children.push(Child {
                    m,
                    key: self.pos.hash(),
                    ruled: self.pos.rule_score().is_some(),
                });
            }
            self.pos.unmake();
        }
        self.moves = moves;
        children
    }

    fn numbers(&self, child: &Child<P::Move>) -> (u32, u32) {
        if child.ruled {
            // No mate through a rule-decided position.
            (INFINITE, 0)
        } else {
            self.table.get(child.key)
        }
    }

    /// Expand the node until its proof or disproof number reaches the
    /// threshold. Returns (pn, dn) of this node from the attacker's view.
    fn mid(&mut self, th_pn: u32, th_dn: u32, attacker: bool, ply: usize) -> (u32, u32) {
        self.nodes += 1;
        let key = self.pos.hash();
        if ply >= MAX_MATE_PLY {
            return (INFINITE, 0);
        }
        let children = self.children(attacker);
        if children.is_empty() {
            // No check: no mate. No legal reply to a check: mate.
            let result = if attacker {
                (INFINITE, 0)
            } else {
                (0, INFINITE)
            };
            self.table.put(key, result.0, result.1, 0);
            return result;
        }
        loop {
            // pn/dn of this node, and the best and second-best child.
            let (mut pn, mut dn) = if attacker {
                (INFINITE, 0)
            } else {
                (0, INFINITE)
            };
            let mut best = 0;
            let mut best_value = INFINITE;
            let mut second = INFINITE;
            let mut best_other = 0;
            for (i, child) in children.iter().enumerate() {
                let (cpn, cdn) = self.numbers(child);
                let (value, other) = if attacker { (cpn, cdn) } else { (cdn, cpn) };
                if attacker {
                    pn = pn.min(cpn);
                    dn = add(dn, cdn);
                } else {
                    pn = add(pn, cpn);
                    dn = dn.min(cdn);
                }
                if value < best_value {
                    second = best_value;
                    best_value = value;
                    best = i;
                    best_other = other;
                } else if value < second {
                    second = value;
                }
            }
            if pn >= th_pn || dn >= th_dn || pn == 0 || dn == 0 || self.exhausted() {
                let len = if pn == 0 {
                    self.proof_len(&children, attacker)
                } else {
                    0
                };
                self.table.put(key, pn, dn, len);
                return (pn, dn);
            }
            // 1+epsilon threshold for the selected child.
            let limit = second
                .saturating_add(second / 4)
                .saturating_add(1)
                .min(INFINITE);
            let (child_pn, child_dn) = if attacker {
                (th_pn.min(limit), th_dn - dn + best_other)
            } else {
                (th_pn - pn + best_other, th_dn.min(limit))
            };
            let child = children[best];
            if !self.pos.make(child.m) {
                // Generation and admission disagree: treat as no mate.
                self.table.put(key, INFINITE, 0, 0);
                return (INFINITE, 0);
            }
            self.mid(child_pn, child_dn, !attacker, ply + 1);
            self.pos.unmake();
        }
    }

    /// Proof length of a proven node: the shortest proven attacker move, or
    /// the longest defender reply, plus one.
    fn proof_len(&self, children: &[Child<P::Move>], attacker: bool) -> u32 {
        let lengths = children
            .iter()
            .filter(|c| !c.ruled)
            .filter_map(|c| self.table.len(c.key));
        1 + if attacker {
            lengths.min()
        } else {
            lengths.max()
        }
        .unwrap_or(MAX_MATE_PLY as u32)
    }

    /// Follow proven moves from the root to the mate along strictly
    /// decreasing stored proof lengths, so the line cannot cycle.
    fn proof_line(&mut self) -> Option<Vec<P::Move>> {
        let mut line = vec![];
        let mut attacker = true;
        let mut bound = self
            .table
            .len(self.pos.hash())
            .map_or(MAX_MATE_PLY as u32 + 1, |len| len + 1);
        let ok = loop {
            let children = self.children(attacker);
            if children.is_empty() {
                break !attacker;
            }
            if line.len() >= MAX_MATE_PLY {
                break false;
            }
            // Attacker: the shortest proven move. Defender: the longest
            // resistance; every reply must be proven.
            let proven = children.iter().filter(|c| !c.ruled).filter_map(|c| {
                self.table
                    .len(c.key)
                    .filter(|&len| len < bound)
                    .map(|len| (len, c))
            });
            let pick = if attacker {
                proven.min_by_key(|&(len, _)| len)
            } else if children
                .iter()
                .all(|c| !c.ruled && self.table.len(c.key).is_some())
            {
                proven.max_by_key(|&(len, _)| len)
            } else {
                None
            };
            let Some((len, &child)) = pick else {
                break false;
            };
            bound = len;
            if !self.pos.make(child.m) {
                break false;
            }
            line.push(child.m);
            attacker = !attacker;
        };
        for _ in 0..line.len() {
            self.pos.unmake();
        }
        ok.then_some(line)
    }
}

/// Search for a forced mate by continuous checks for the side to move.
/// The position is restored before return. Returns the result and the node
/// visits.
pub fn search<P: SearchPosition>(
    pos: &mut P,
    table: &mut MateTable,
    limits: MateLimits,
    stop: &mut dyn FnMut() -> bool,
) -> (MateResult<P::Move>, u64) {
    let mut run = Mate {
        pos,
        table,
        limits,
        stop,
        nodes: 0,
        abort: false,
        moves: Vec::with_capacity(256),
    };
    let (pn, dn) = run.mid(INFINITE - 1, INFINITE - 1, true, 0);
    let result = if pn == 0 {
        match run.proof_line() {
            Some(line) if !line.is_empty() => MateResult::Mate(line),
            // A proof whose line cannot be followed (for example a table
            // collision) is not reported as a mate.
            _ => MateResult::Unknown,
        }
    } else if dn == 0 && !run.abort {
        MateResult::NoMate
    } else {
        MateResult::Unknown
    };
    (result, run.nodes)
}
