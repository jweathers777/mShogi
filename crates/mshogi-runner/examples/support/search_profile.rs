//! Diagnostic wrapper: coarse, non-overlapping search-operation timings.
//! Only the benchmark uses this type; normal searches have no timers.
use mshogi_search::alphabeta::{SearchPosition, Stage};
use std::{cell::RefCell, time::Instant};

pub struct Profiled<P> {
    pub inner: P,
    pub totals: RefCell<[(u64, u128); 5]>,
}
impl<P> Profiled<P> {
    pub fn new(inner: P) -> Self {
        Self {
            inner,
            totals: RefCell::new([(0, 0); 5]),
        }
    }
    fn record(&self, index: usize, start: Instant) {
        let nanos = start.elapsed().as_nanos();
        let mut totals = self.totals.borrow_mut();
        totals[index].0 += 1;
        totals[index].1 += nanos;
    }
}
macro_rules! timed {
    ($self:ident, $index:expr, $call:expr) => {{
        let at = Instant::now();
        let result = $call;
        $self.record($index, at);
        result
    }};
}
macro_rules! forward {
    ($(fn $name:ident(&$self:ident $(, $arg:ident: $ty:ty)*) -> $result:ty;)*) => {
        $(fn $name(&$self, $($arg: $ty),*) -> $result { $self.inner.$name($($arg),*) })*
    };
}
impl<P: SearchPosition> SearchPosition for Profiled<P> {
    type Move = P::Move;
    const HISTORY_SLOTS: usize = P::HISTORY_SLOTS;
    const QUIESCENCE_PLIES: usize = P::QUIESCENCE_PLIES;
    const MARGIN_UNIT: i32 = P::MARGIN_UNIT;
    const QUIESCENCE_CHECKS: usize = P::QUIESCENCE_CHECKS;
    const MATE_SEARCH: bool = P::MATE_SEARCH;
    forward! {
        fn hash(&self) -> u64;
        fn in_check(&self) -> bool;
        fn null_move_safe(&self) -> bool;
        fn draw_claimable(&self) -> bool;
        fn no_move_score(&self, ply: i32) -> i32;
        fn evaluate(&self) -> i32;
        fn is_tactical(&self, m: Self::Move) -> bool;
        fn tactical_order(&self, m: Self::Move) -> i32;
        fn tactical_gain(&self, m: Self::Move) -> i32;
        fn quiet_order(&self, m: Self::Move) -> i32;
        fn history_slot(&self, m: Self::Move) -> usize;
        fn decode(&self, bits: u32) -> Option<Self::Move>;
        fn is_pseudo_legal(&self, m: Self::Move) -> bool;
    }
    fn encode(m: Self::Move) -> u32 {
        P::encode(m)
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<Self::Move>) {
        self.inner.generate(stage, moves);
    }
    fn generate_checks(&self, moves: &mut Vec<Self::Move>) {
        self.inner.generate_checks(moves);
    }
    fn generate_search(&self, stage: Stage, moves: &mut Vec<Self::Move>) {
        timed!(self, 0, self.inner.generate_search(stage, moves))
    }
    fn make(&mut self, m: Self::Move) -> bool {
        timed!(self, 1, self.inner.make(m))
    }
    fn unmake(&mut self) {
        timed!(self, 2, self.inner.unmake())
    }
    fn rule_score(&self) -> Option<i32> {
        timed!(self, 3, self.inner.rule_score())
    }
    fn exchange_at_least(&self, m: Self::Move, threshold: i32) -> bool {
        timed!(self, 4, self.inner.exchange_at_least(m, threshold))
    }
    fn make_null(&mut self) {
        self.inner.make_null();
    }
    fn unmake_null(&mut self) {
        self.inner.unmake_null();
    }
}
