//! Game-independent Texel model interface.
//!
//! A game supplies a [`TexelModel`]: parameter names and bounds, position
//! preparation and the exact integer score that its search uses. The shared
//! fitting tool (`examples/support/texel.rs`) owns the corpus format, K
//! calibration, bounded coordinate descent, validation selection and the
//! separate held-out check. Corpus rows are JSON lines `{POSITION_FIELD,
//! "result", "split", "game_id"}`; `result` is the first player's
//! outcome, never a search score.

/// A game's tunable evaluation.
pub trait TexelModel {
    /// Prepared, weight-independent position data.
    type Prepared: Sync;
    /// Short game name used in output kinds (`mshogi-{KIND}-texel-fit`).
    const KIND: &'static str;
    /// Corpus field that holds the position text (for example `fen`).
    const POSITION_FIELD: &'static str;
    /// K calibration range; 36 evenly spaced values are tried.
    const K_RANGE: (f64, f64) = (0.25, 2.0);
    /// Coordinate descent step sizes, largest first.
    const STEPS: &'static [i32];
    /// Parameter names in stable order.
    fn names() -> &'static [&'static str];
    /// Untuned starting point.
    fn initial() -> Vec<i32>;
    /// Inclusive lower bounds.
    fn lower() -> Vec<i32>;
    /// Inclusive upper bounds.
    fn upper() -> Vec<i32>;
    /// Identity used to reject duplicate positions (board, side, rights).
    fn key(position: &str) -> String;
    /// Prepare a position. `Ok(None)` skips a row that the model does not fit
    /// (for example a position that is not quiet); skipped rows are counted.
    fn prepare(position: &str) -> std::result::Result<Option<Self::Prepared>, String>;
    /// Exact search score from the first player's view.
    fn score(prepared: &Self::Prepared, weights: &[i32]) -> i32;
    /// Report bucket labels.
    fn buckets() -> &'static [&'static str];
    /// Report bucket of a position.
    fn bucket(prepared: &Self::Prepared) -> usize;
}

/// Logistic win probability of a score.
pub fn probability(score: i32, k: f64) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf(-k * f64::from(score) / 400.0))
}
