//! Short native search diagnostic. No matches, opponents, Elo or acceptance inference.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, STANDARD_START_FEN};
use mshogi_search::{
    chess_conventional::{TimeBudget, search_with_time},
    search::SearchLimits,
};
use std::time::Instant;

fn main() {
    for (name, fen) in [
        ("standard", STANDARD_START_FEN),
        (
            "developed",
            "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 w - - 0 1",
        ),
        ("recapture", "r6k/8/8/8/8/8/q7/R6K w - - 0 1"),
        ("endgame", "7k/8/2K5/8/8/8/P7/R7 w - - 0 1"),
    ] {
        let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::AutoClaim).unwrap();
        let original = game.clone();
        let start = Instant::now();
        let result = search_with_time(
            &game,
            SearchLimits {
                depth: 8,
                nodes: Some(20_000),
            },
            Some(TimeBudget::from_hard_ms(1000)),
            || false,
        )
        .unwrap();
        assert_eq!(game, original);
        let mut replay = game.clone();
        for c in &result.result.principal_variation {
            match c {
                mshogi_search::search::SearchChoice::Move(a) => replay.apply(*a).unwrap(),
                mshogi_search::search::SearchChoice::Claim(c) => replay.claim(*c).unwrap(),
            }
        }
        let s = &result.stats;
        let r = &result.result;
        println!(
            "{}",
            serde_json::json!({"case":name,"fen":fen,"depth_limit":8,
            "node_limit":20000,"hard_ms":1000,"soft_ms":700,"elapsed_ms":start.elapsed().as_secs_f64()*1000.0,
            "completed_depth":r.completed_depth,"selective_depth":result.selective_depth,
            "nodes":r.nodes,"score":r.score,"status":format!("{:?}",r.status),
            "pv":r.principal_variation.iter().map(|c|format!("{c:?}")).collect::<Vec<_>>(),
            "stats":{"main_nodes":s.main_nodes,"q_nodes":s.q_nodes,"tt_hits":s.tt_hits,
                "tt_cutoffs":s.tt_cutoffs,"researches":s.researches,"reductions":s.reductions,
                "aspiration_researches":s.aspiration_researches,"null_probes":s.null_probes,
                "null_verified":s.null_verified,"extensions":s.extensions,
                "futility_pruned":s.futility_pruned,"see_pruned":s.see_pruned,"see_probes":s.see_probes}})
        );
    }
}
