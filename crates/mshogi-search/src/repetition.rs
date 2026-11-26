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

/// Search approximation for drop games: a capture or drop resets the scan.
/// The native rules still check the full known history of the game.
#[derive(Clone, Copy, Default)]
pub(crate) struct DropGameWindow(u16);

impl DropGameWindow {
    pub(crate) fn advance(&mut self, capture_or_drop: bool) {
        self.0 = if capture_or_drop {
            0
        } else {
            self.0.saturating_add(1)
        };
    }

    pub(crate) fn occurrences(self, keys: &[u64]) -> impl Iterator<Item = usize> + '_ {
        let (&key, prior) = keys.split_last().expect("current position key");
        earlier_occurrences(prior, key, 2..=usize::from(self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_or_drop_excludes_old_keys_and_quiet_plies_include_the_new_boundary() {
        let mut window = DropGameWindow::default();
        for _ in 0..4 {
            window.advance(false);
        }
        assert_eq!(window.occurrences(&[5, 7, 8, 9, 5]).next(), Some(0));
        window.advance(true);
        assert_eq!(window.occurrences(&[5, 7, 8, 9, 5, 7]).next(), None);
        for _ in 0..4 {
            window.advance(false);
        }
        assert_eq!(
            window
                .occurrences(&[5, 7, 8, 9, 5, 7, 1, 2, 3, 7])
                .collect::<Vec<_>>(),
            [5]
        );
    }
}
