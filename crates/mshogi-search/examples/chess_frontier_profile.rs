//! Bounded full-history investigation, not an opponent match or strength estimate.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, STANDARD_START_FEN};
use mshogi_search::{
    chess_conventional::{TimeBudget, search_with_time},
    search::{MAX_DEPTH, SearchLimits},
};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let inputs: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&args[1]).unwrap()).unwrap();
    let depth = args.get(2).map_or(MAX_DEPTH, |x| x.parse().unwrap());
    let timed = args.get(3).is_none_or(|x| x == "timed");
    let nodes =
        args.get(4)
            .map(|x| x.parse().unwrap())
            .or(if timed { None } else { Some(300_000) });
    for input in inputs.as_array().unwrap() {
        let fen = input["fen"].as_str().unwrap_or(STANDARD_START_FEN);
        let mut game = AdjudicatedGame::from_fen(fen, ClaimPolicy::AutoClaim).unwrap();
        if let Some(history) = input["history"].as_array() {
            for action in history {
                game.apply(action.as_str().unwrap().parse().unwrap())
                    .unwrap();
            }
        }
        let before = game.clone();
        let start = Instant::now();
        let r = search_with_time(
            &game,
            SearchLimits { depth, nodes },
            timed.then(|| TimeBudget::from_hard_ms(1000)),
            || false,
        )
        .unwrap();
        assert_eq!(before, game);
        let mut replay = game.clone();
        for choice in &r.result.principal_variation {
            match choice {
                mshogi_search::search::SearchChoice::Move(a) => replay.apply(*a).unwrap(),
                mshogi_search::search::SearchChoice::Claim(c) => replay.claim(*c).unwrap(),
            }
        }
        println!(
            "{}",
            serde_json::json!({"id":input["id"], "fen":game.position().to_fen(), "elapsed_ms":start.elapsed().as_secs_f64()*1000.0, "depth_limit":depth,"node_limit":nodes,"timed":timed,"result":format!("{:?}",r.result),"seldepth":r.selective_depth,"stats":format!("{:?}",r.stats),"iterations":r.iterations.iter().map(|i|serde_json::json!({"depth":i.depth,"completed":i.completed,"elapsed_us":i.elapsed_us,"stats":format!("{:?}",i.stats)})).collect::<Vec<_>>()})
        );
    }
}
