//! Small fixed-size square sets for private large-board positions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SquareSet<const WORDS: usize>([u64; WORDS]);
impl<const WORDS: usize> Default for SquareSet<WORDS> {
    fn default() -> Self {
        Self([0; WORDS])
    }
}
impl<const WORDS: usize> SquareSet<WORDS> {
    pub(crate) fn add(&mut self, at: usize) {
        self.0[at / 64] |= 1 << (at % 64);
    }
    pub(crate) fn remove(&mut self, at: usize) {
        self.0[at / 64] &= !(1 << (at % 64));
    }
    pub(crate) fn contains(self, at: usize) -> bool {
        self.0[at / 64] & (1 << (at % 64)) != 0
    }
    pub(crate) fn count(self) -> u32 {
        self.0.iter().map(|n| n.count_ones()).sum()
    }
    pub(crate) fn pop(&mut self) -> Option<usize> {
        for (i, word) in self.0.iter_mut().enumerate() {
            if *word != 0 {
                let bit = word.trailing_zeros() as usize;
                *word &= *word - 1;
                return Some(i * 64 + bit);
            }
        }
        None
    }
}
