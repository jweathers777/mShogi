//! Bounded ordering statistics. Collisions affect ordering, never move legality.
use std::ops::{Index, IndexMut};

const HISTORY_ENTRIES: usize = 1 << 16;
const CONTINUATION_ENTRIES: usize = 1 << 23;

pub(super) struct History {
    entries: Vec<i32>,
    mask: usize,
}

impl History {
    pub(super) fn new(slots: usize) -> Self {
        assert!(slots > 0, "a position must supply history slots");
        Self {
            entries: vec![0; slots.min(HISTORY_ENTRIES)],
            mask: if slots <= HISTORY_ENTRIES {
                usize::MAX
            } else {
                HISTORY_ENTRIES - 1
            },
        }
    }

    pub(super) fn clear(&mut self) {
        self.entries.fill(0);
    }

    pub(super) fn decay(&mut self) {
        for value in &mut self.entries {
            *value /= 2;
        }
    }
}

impl Index<usize> for History {
    type Output = i32;
    fn index(&self, slot: usize) -> &i32 {
        &self.entries[slot & self.mask]
    }
}

impl IndexMut<usize> for History {
    fn index_mut(&mut self, slot: usize) -> &mut i32 {
        let index = slot & self.mask;
        &mut self.entries[index]
    }
}

pub(super) struct ContinuationHistory {
    entries: Vec<i32>,
    // Some(stride) means the complete dense table fits. Otherwise hash pairs.
    stride: Option<usize>,
}

impl ContinuationHistory {
    pub(super) fn new(slots: usize) -> Self {
        assert!(slots > 0, "a position must supply history slots");
        let dense = slots
            .checked_mul(slots)
            .filter(|&n| n <= CONTINUATION_ENTRIES);
        Self {
            entries: vec![0; dense.unwrap_or(CONTINUATION_ENTRIES)],
            stride: dense.map(|_| slots),
        }
    }

    pub(super) fn clear(&mut self) {
        self.entries.fill(0);
    }

    fn index_of(&self, (previous, current): (usize, usize)) -> usize {
        if let Some(stride) = self.stride {
            previous * stride + current
        } else {
            // Mix the slots separately: no product involving the game's slot
            // count is formed, even for the largest representable count.
            let key = mix(previous as u64) ^ mix(current as u64).rotate_left(32);
            mix(key) as usize & (CONTINUATION_ENTRIES - 1)
        }
    }
}

fn mix(mut key: u64) -> u64 {
    key = (key ^ (key >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    key = (key ^ (key >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    key ^ (key >> 31)
}

impl Index<(usize, usize)> for ContinuationHistory {
    type Output = i32;
    fn index(&self, pair: (usize, usize)) -> &i32 {
        &self.entries[self.index_of(pair)]
    }
}

impl IndexMut<(usize, usize)> for ContinuationHistory {
    fn index_mut(&mut self, pair: (usize, usize)) -> &mut i32 {
        let index = self.index_of(pair);
        &mut self.entries[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphabeta::{HISTORY_LIMIT, gravity};

    #[test]
    fn all_current_games_keep_dense_indices_and_updates() {
        for slots in [768, 882, 2268, 1260] {
            let mut table = ContinuationHistory::new(slots);
            assert_eq!(table.entries.len(), slots * slots);
            // Exhaustive pair mapping, not a sample that can miss aliasing.
            for previous in 0..slots {
                for current in 0..slots {
                    assert_eq!(
                        table.index_of((previous, current)),
                        previous * slots + current
                    );
                }
            }
            let mut history = History::new(slots);
            assert_eq!(history.entries.len(), slots);
            for slot in 0..slots {
                history[slot] = slot as i32;
            }
            for slot in 0..slots {
                assert_eq!(history[slot], slot as i32);
            }
            let pair = (slots - 1, slots - 2);
            let mut expected = 0;
            for bonus in [16_i32, 64, 2000, -2000, -64, 144] {
                expected += bonus - expected * bonus.abs() / 16384;
                gravity(&mut table[pair], bonus);
                assert_eq!(table[pair], expected);
            }
            table.clear();
            assert!(table.entries.iter().all(|&n| n == 0));
            history.decay();
            assert_eq!(history[slots - 1], (slots as i32 - 1) / 2);
            history.clear();
            assert!(history.entries.iter().all(|&n| n == 0));
        }
    }

    #[test]
    fn large_and_extreme_slot_counts_have_fixed_storage() {
        for slots in [2896, 2897, 8352, 22528, usize::MAX] {
            let mut table = ContinuationHistory::new(slots);
            let expected = slots.saturating_mul(slots).min(CONTINUATION_ENTRIES);
            assert_eq!(table.entries.len(), expected);
            assert_eq!(
                History::new(slots).entries.len(),
                slots.min(HISTORY_ENTRIES)
            );
            for pair in [
                (0, 0),
                (slots - 1, 0),
                (0, slots - 1),
                (slots - 1, slots - 1),
            ] {
                table[pair] = 123;
                assert_eq!(table[pair], 123);
            }
        }
    }

    #[test]
    fn colliding_pairs_share_bounded_statistics_and_clear() {
        let mut table = ContinuationHistory::new(usize::MAX);
        let mut seen = std::collections::HashMap::new();
        let (first, second) = (0..100_000)
            .find_map(|slot| {
                let pair = (slot, slot + 1);
                seen.insert(table.index_of(pair), pair)
                    .map(|old| (old, pair))
            })
            .expect("birthday collision in fixed table");
        for bonus in [2000, -2000] {
            for _ in 0..1000 {
                gravity(&mut table[first], bonus);
                assert_eq!(table[first], table[second]);
                assert!(table[first].abs() <= HISTORY_LIMIT);
            }
        }
        table.clear();
        assert_eq!(table[first], 0);
        assert_eq!(table[second], 0);
        let mut history = History::new(usize::MAX);
        history[0] = -99;
        assert_eq!(history[HISTORY_ENTRIES], -99);
        history.decay();
        assert_eq!(history[HISTORY_ENTRIES], -49);
        history.clear();
        assert_eq!(history[HISTORY_ENTRIES], 0);
    }

    #[test]
    #[should_panic(expected = "a position must supply history slots")]
    fn empty_history_is_rejected() {
        History::new(0);
    }

    #[test]
    #[should_panic(expected = "a position must supply history slots")]
    fn empty_continuation_is_rejected() {
        ContinuationHistory::new(0);
    }
}
