//! Transposition table keyed on the board-state hash only.
//!
//! The table is private engine analysis. An entry can be reused for a position
//! that was reached through a different history, so a stored score can be
//! inexact in rare history-dependent cases (design A-03). It never decides a
//! real game result. The owner keeps one table for a full game.

/// Kind of score bound in an entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Bound {
    /// The score is exact.
    Exact,
    /// The true score is at least the stored score (fail high).
    Lower,
    /// The true score is at most the stored score (fail low).
    Upper,
}

/// One probe result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hit {
    /// Encoded best move, zero when the entry has none.
    pub move_bits: u32,
    /// Stored score, already adjusted to the probing ply for mate scores.
    pub score: i32,
    /// Static evaluation of the position, when the entry has one.
    pub eval: Option<i32>,
    /// Remaining depth of the stored search. Quiescence entries have depth 0.
    pub depth: i32,
    /// Bound kind of the score.
    pub bound: Bound,
}

#[derive(Clone, Copy, Default)]
struct Entry {
    key: u32,
    move_bits: u32,
    score: i32,
    eval: i16,
    depth: i8,
    // Bits 0-1: bound (0 empty, 1 exact, 2 lower, 3 upper). Bits 2-7: generation.
    meta: u8,
}

const NO_EVAL: i16 = i16::MIN;
const BUCKET: usize = 4;

/// Fixed-size table with buckets of four entries and generation aging.
pub struct TranspositionTable {
    entries: Vec<Entry>,
    buckets: usize,
    generation: u8,
    mate_bound: i32,
}

impl TranspositionTable {
    /// Make a table of about `megabytes` (at least one bucket). `mate_bound` is
    /// the smallest absolute score that is a mate score; such scores are stored
    /// relative to the node and not to the root.
    pub fn new(megabytes: usize, mate_bound: i32) -> Self {
        let bytes = megabytes.max(1) * 1024 * 1024;
        let buckets = (bytes / (BUCKET * std::mem::size_of::<Entry>())).max(1);
        Self {
            entries: vec![Entry::default(); buckets * BUCKET],
            buckets,
            generation: 0,
            mate_bound,
        }
    }
    /// Size in megabytes, rounded down.
    pub fn megabytes(&self) -> usize {
        self.entries.len() * std::mem::size_of::<Entry>() / (1024 * 1024)
    }
    /// Remove all entries.
    pub fn clear(&mut self) {
        self.entries.fill(Entry::default());
        self.generation = 0;
    }
    /// Start a new search: older entries become preferred replacement victims.
    pub fn new_search(&mut self) {
        self.generation = (self.generation + 1) & 63;
    }
    /// Permille of sampled entries written in the current search.
    pub fn hashfull(&self) -> u32 {
        let sample = self.entries.len().min(1000);
        let used = self.entries[..sample]
            .iter()
            .filter(|e| e.meta & 3 != 0 && e.meta >> 2 == self.generation)
            .count();
        (used * 1000 / sample.max(1)) as u32
    }
    #[inline]
    fn bucket(&self, hash: u64) -> usize {
        // Multiply-shift maps the low 32 hash bits to a bucket without modulo.
        (((hash & 0xffff_ffff) * self.buckets as u64) >> 32) as usize * BUCKET
    }
    /// Find the entry for this hash.
    #[inline]
    pub fn probe(&self, hash: u64, ply: i32) -> Option<Hit> {
        let start = self.bucket(hash);
        let key = (hash >> 32) as u32;
        for e in &self.entries[start..start + BUCKET] {
            if e.key == key && e.meta & 3 != 0 {
                let mut score = e.score;
                if score >= self.mate_bound {
                    score -= ply;
                } else if score <= -self.mate_bound {
                    score += ply;
                }
                return Some(Hit {
                    move_bits: e.move_bits,
                    score,
                    eval: (e.eval != NO_EVAL).then_some(i32::from(e.eval)),
                    depth: i32::from(e.depth),
                    bound: match e.meta & 3 {
                        1 => Bound::Exact,
                        2 => Bound::Lower,
                        _ => Bound::Upper,
                    },
                });
            }
        }
        None
    }
    /// Store a result. An entry for the same position keeps its move when the new
    /// result has none, and is kept when it is much deeper and from this search.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn store(
        &mut self,
        hash: u64,
        ply: i32,
        move_bits: u32,
        mut score: i32,
        eval: Option<i32>,
        depth: i32,
        bound: Bound,
    ) {
        let start = self.bucket(hash);
        let key = (hash >> 32) as u32;
        if score >= self.mate_bound {
            score += ply;
        } else if score <= -self.mate_bound {
            score -= ply;
        }
        let generation = self.generation;
        let mut victim = start;
        let mut worst = i32::MAX;
        for i in start..start + BUCKET {
            let e = &self.entries[i];
            if e.meta & 3 == 0 || e.key == key {
                victim = i;
                break;
            }
            let age = i32::from(generation.wrapping_sub(e.meta >> 2) & 63);
            let worth = i32::from(e.depth) - 8 * age;
            if worth < worst {
                worst = worth;
                victim = i;
            }
        }
        let e = &mut self.entries[victim];
        let same = e.key == key && e.meta & 3 != 0;
        // Keep a much deeper result of this search for the same position. A
        // quiescence result (depth 0) never replaces a main-search result.
        if same
            && e.meta >> 2 == generation
            && (depth == 0 && e.depth > 0
                || bound != Bound::Exact && i32::from(e.depth) > depth + 3)
        {
            return;
        }
        let move_bits = if move_bits == 0 && same {
            e.move_bits
        } else {
            move_bits
        };
        *e = Entry {
            key,
            move_bits,
            score,
            eval: eval.map_or(NO_EVAL, |v| {
                v.clamp(i32::from(i16::MIN) + 1, i32::from(i16::MAX)) as i16
            }),
            depth: depth.clamp(0, 127) as i8,
            meta: (generation << 2)
                | match bound {
                    Bound::Exact => 1,
                    Bound::Lower => 2,
                    Bound::Upper => 3,
                },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_is_sixteen_bytes_and_size_follows_megabytes() {
        assert_eq!(std::mem::size_of::<Entry>(), 16);
        let t = TranspositionTable::new(8, 900_000);
        assert_eq!(t.megabytes(), 8);
        assert_eq!(t.entries.len(), 8 * 1024 * 1024 / 16);
    }

    #[test]
    fn store_probe_mate_adjustment_and_move_retention() {
        let mut t = TranspositionTable::new(1, 900_000);
        t.new_search();
        let h = 0x1234_5678_9abc_def0;
        assert_eq!(t.probe(h, 0), None);
        t.store(h, 3, 77, 999_990, Some(25), 6, Bound::Lower);
        let hit = t.probe(h, 5).unwrap();
        // Mate in 10 plies from ply 3 is stored as node-relative 999_993.
        assert_eq!(hit.score, 999_988);
        assert_eq!((hit.move_bits, hit.depth, hit.bound), (77, 6, Bound::Lower));
        assert_eq!(hit.eval, Some(25));
        // A shallower result without a move keeps the stored move.
        t.store(h, 0, 0, -40, None, 7, Bound::Upper);
        let hit = t.probe(h, 0).unwrap();
        assert_eq!((hit.move_bits, hit.score, hit.eval), (77, -40, None));
        // A much shallower bound does not replace a deep entry of this search.
        t.store(h, 0, 5, 10, None, 1, Bound::Lower);
        assert_eq!(t.probe(h, 0).unwrap().depth, 7);
        // A quiescence result does not replace a main-search entry of this search.
        t.store(h, 0, 5, 10, None, 0, Bound::Exact);
        assert_eq!(t.probe(h, 0).unwrap().depth, 7);
        // A different position with the same bucket index and another key coexists.
        let other = h ^ (1 << 40);
        t.store(other, 0, 9, 1, None, 2, Bound::Exact);
        assert_eq!(t.probe(other, 0).unwrap().move_bits, 9);
        assert_eq!(t.probe(h, 0).unwrap().move_bits, 77);
        t.clear();
        assert_eq!(t.probe(h, 0), None);
    }

    #[test]
    fn full_bucket_replaces_the_old_shallow_entry() {
        let mut t = TranspositionTable::new(1, 900_000);
        t.new_search();
        let base = 42u64;
        for i in 0..4u64 {
            t.store(
                base | ((i + 1) << 32),
                0,
                i as u32 + 1,
                0,
                None,
                10,
                Bound::Exact,
            );
        }
        t.new_search();
        t.store(base | (2 << 32), 0, 2, 0, None, 10, Bound::Exact);
        t.store(base | (9 << 32), 0, 99, 0, None, 1, Bound::Exact);
        assert!(t.probe(base | (9 << 32), 0).is_some());
        assert!(t.probe(base | (2 << 32), 0).is_some());
        let left = (1..=4u64)
            .filter(|i| t.probe(base | (i << 32), 0).is_some())
            .count();
        assert_eq!(left, 3);
    }
}
