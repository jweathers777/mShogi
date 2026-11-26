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
