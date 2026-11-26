//! Earlier occurrences of the current position on a private search path.
use std::ops::RangeInclusive;

/// Indices of matching keys, newest first. `prior` excludes the current
/// position. The first lookback is an even number of plies. A side can
/// repeat only on its own turn.
pub(crate) fn earlier_occurrences(
    prior: &[u64],
    key: u64,
    lookback: RangeInclusive<usize>,
) -> impl Iterator<Item = usize> + '_ {
    let last = prior.len().min(*lookback.end());
    (*lookback.start()..=last)
        .step_by(2)
        .map(|back| prior.len() - back)
        .filter(move |&index| prior[index] == key)
}

/// A cycle in this search is enough. Before the root, the game specifies
/// how many earlier occurrences establish a repetition score.
pub(crate) fn cycle_start(
    occurrences: impl Iterator<Item = usize>,
    root: usize,
    required_before_root: usize,
) -> Option<usize> {
    occurrences
        .enumerate()
        .find(|&(count, index)| index >= root || count + 1 >= required_before_root)
        .map(|(_, index)| index)
}

/// The immutable part of a drop game's history before the search root.
/// A clear bit rules out every pre-root match. A set bit still requires
/// comparing the full keys, and the search path is always eligible.
#[derive(Clone, Default)]
pub(crate) struct GamePrefix {
    pub(crate) len: usize,
    buckets: [u64; 16],
}

impl GamePrefix {
    pub(crate) fn new(keys: &[u64]) -> Self {
        let mut prefix = Self {
            len: keys.len(),
            ..Self::default()
        };
        for &key in keys {
            let (bucket, bit) = Self::slot(key);
            prefix.buckets[bucket] |= bit;
        }
        prefix
    }

    fn slot(key: u64) -> (usize, u64) {
        ((key as usize >> 6) & 15, 1 << (key & 63))
    }

    pub(crate) fn occurrences<'a>(
        &self,
        keys: &'a [u64],
        after_null: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        let (&key, prior) = keys.split_last().expect("current position key");
        let (bucket, bit) = Self::slot(key);
        let first = if self.buckets[bucket] & bit == 0 {
            after_null.max(self.len)
        } else {
            after_null
        };
        earlier_occurrences(prior, key, 2..=prior.len() - first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bucket_collision_still_checks_full_keys_and_the_null_boundary() {
        let keys = [7, 8, 7 + 1024, 9, 7, 10, 7];
        let prefix = GamePrefix::new(&keys[..4]);
        assert_eq!(prefix.occurrences(&keys, 0).collect::<Vec<_>>(), [4, 0]);
        assert_eq!(prefix.occurrences(&keys, 2).collect::<Vec<_>>(), [4]);
        assert_eq!(prefix.occurrences(&keys, 5).next(), None);
        let keys = [7, 8, 9, 10, 11, 12, 11];
        assert_eq!(prefix.occurrences(&keys, 0).collect::<Vec<_>>(), [4]);
    }
}
