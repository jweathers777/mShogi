//! Reproducible shallow search workload; wall time is an observation only.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, STANDARD_START_FEN};
use mshogi_search::{chess::EVALUATOR_ID, search::*};
use std::time::Instant;

fn main() {
    let limits = SearchLimits {
        depth: 3,
        nodes: None,
    };
    println!(
        "search={SEARCH_ID} evaluator={EVALUATOR_ID} policy=explicit_only depth=3 nodes=unlimited build=release-recommended"
    );
    for (name, fen) in [
        ("standard", STANDARD_START_FEN),
        ("mate-two", "7k/8/4QK2/8/8/8/8/8 w - - 0 1"),
        ("capture", "7k/8/8/8/8/8/q7/R6K w - - 0 1"),
    ] {
        let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        let started = Instant::now();
        let result = search(&game, limits).unwrap();
        let elapsed = started.elapsed();
        println!(
            "case={name} fen={fen:?} nodes={} elapsed_ms={:.3} nodes_per_second={:.0} depth={} score={:?} decision={:?} status={:?}",
            result.nodes,
            elapsed.as_secs_f64() * 1000.0,
            result.nodes as f64 / elapsed.as_secs_f64(),
            result.completed_depth,
            result.score,
            result.decision,
            result.status
        );
    }
}
