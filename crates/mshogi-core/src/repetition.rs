//! Root-indexed board history. Controls do not add a position.
use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    iter,
};

/// The map points to the newest ply with each hash. Earlier plies with the
/// same hash form a chain. Each hit is confirmed against the full key.
/// All positions share the same backing vectors, with no allocation per bucket.
#[derive(Clone, Debug)]
pub(crate) struct RepetitionIndex<K> {
    keys: Vec<K>,
    previous: Vec<Option<usize>>,
    latest: HashMap<u64, usize>,
}

// Different instances may hash the same history with different seeds.
impl<K: PartialEq> PartialEq for RepetitionIndex<K> {
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys
    }
}
impl<K: Eq> Eq for RepetitionIndex<K> {}

impl<K: Eq + Hash> RepetitionIndex<K> {
    pub(crate) fn new(root: K) -> Self {
        let mut index = Self {
            keys: Vec::new(),
            previous: Vec::new(),
            latest: HashMap::new(),
        };
        index.push(root);
        index
    }

    pub(crate) fn push(&mut self, key: K) {
        let hash = self.latest.hasher().hash_one(&key);
        self.previous
            .push(self.latest.insert(hash, self.keys.len()));
        self.keys.push(key);
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        assert!(len > 0, "the root remains in known history");
        while self.keys.len() > len {
            let key = self.keys.pop().expect("discarded position");
            let hash = self.latest.hasher().hash_one(&key);
            if let Some(previous) = self.previous.pop().expect("indexed position") {
                self.latest.insert(hash, previous);
            } else {
                self.latest.remove(&hash);
            }
        }
    }

    fn matches(&self, first: Option<usize>, key: &K) -> impl Iterator<Item = usize> {
        iter::successors(first, |&ply| self.previous[ply])
            .filter(move |&ply| &self.keys[ply] == key)
    }

    /// Exact matches, newest first.
    pub(crate) fn occurrences(&self, key: &K) -> impl Iterator<Item = usize> {
        let hash = self.latest.hasher().hash_one(key);
        self.matches(self.latest.get(&hash).copied(), key)
    }

    pub(crate) fn current(&self) -> &K {
        self.keys.last().expect("known root")
    }

    pub(crate) fn current_occurrences(&self) -> Vec<usize> {
        let mut found: Vec<_> = self
            .matches(Some(self.keys.len() - 1), self.current())
            .collect();
        found.reverse();
        found
    }

    pub(crate) fn keys(&self) -> &[K] {
        &self.keys
    }

    pub(crate) fn len(&self) -> usize {
        self.keys.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::Hasher;

    #[derive(Debug, Eq, PartialEq)]
    struct Collision(u8);
    impl Hash for Collision {
        fn hash<H: Hasher>(&self, state: &mut H) {
            0u8.hash(state);
        }
    }

    #[test]
    fn collisions_and_truncated_branches_keep_exact_indices() {
        let mut index = RepetitionIndex::new(Collision(0));
        for key in [1, 0, 1, 2, 0] {
            index.push(Collision(key));
        }
        assert_eq!(index.current_occurrences(), [0, 2, 5]);
        index.truncate(2);
        index.push(Collision(1));
        assert_eq!(index.current_occurrences(), [1, 2]);
        assert_eq!(index.occurrences(&Collision(0)).collect::<Vec<_>>(), [0]);
        assert!(index.occurrences(&Collision(2)).next().is_none());
    }
}
