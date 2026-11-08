//! Fixed-work engineering diagnostic; no games, opponents or strength inference.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, STANDARD_START_FEN};
use mshogi_search::{
    chess_conventional::search,
    search::{SearchChoice, SearchLimits},
};
use std::time::Instant;
fn main() {
    let rounds = std::env::args()
        .nth(1)
        .map_or(3, |s| s.parse::<usize>().unwrap());
    assert!((1..=12).contains(&rounds));
    for round in 0..rounds {
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
            let result = search(
                &game,
                SearchLimits {
                    depth: 8,
                    nodes: Some(20_000),
                },
            )
            .unwrap();
            let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
            assert_eq!(game, original);
            let mut replay = game.clone();
            for choice in &result.result.principal_variation {
                match choice {
                    SearchChoice::Move(a) => replay.apply(*a).unwrap(),
                    SearchChoice::Claim(c) => replay.claim(*c).unwrap(),
                }
            }
            let s = &result.stats;
            println!(
                "{}",
                serde_json::json!({"round":round,"case":name,"fen":fen,
                "depth_limit":8,"node_limit":20000,"elapsed_ms":elapsed_ms,
                "result":format!("{:?}",result.result),"selective_depth":result.selective_depth,
                "main_nodes":s.main_nodes,"q_nodes":s.q_nodes,"see_probes":s.see_probes,
                "tt_probes":s.tt_probes,"tt_position_hits":s.tt_position_hits,
                "tt_context_hits":s.tt_context_hits,"tt_depth_hits":s.tt_hits,
                "tt_cutoffs":s.tt_cutoffs,"tt_stores":s.tt_stores})
            );
        }
    }
}
