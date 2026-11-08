//! Fixed-work engineering diagnostic; no games, opponents or strength inference.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, STANDARD_START_FEN};
use mshogi_search::{
    chess_conventional::{SearchOptions, TimeBudget, search_with_options},
    search::{MAX_DEPTH, SearchChoice, SearchLimits},
};
use std::time::Instant;
fn main() {
    let rounds = std::env::args()
        .nth(1)
        .map_or(3, |s| s.parse::<usize>().unwrap());
    assert!((1..=12).contains(&rounds));
    let mut timed = false;
    let mut depth = None;
    let mut nodes = None;
    let mut table_entries = SearchOptions::default().table_entries;
    for arg in std::env::args().skip(2) {
        if arg == "--timed" {
            timed = true;
        } else if let Some(n) = arg.strip_prefix("--table=") {
            table_entries = n.parse().unwrap();
        } else if let Some(n) = arg.strip_prefix("--depth=") {
            depth = Some(n.parse().unwrap());
        } else if let Some(n) = arg.strip_prefix("--nodes=") {
            nodes = Some(n.parse().unwrap());
        } else {
            panic!("unknown diagnostic option: {arg}");
        }
    }
    let depth = depth.unwrap_or(if timed { MAX_DEPTH } else { 8 });
    let nodes = nodes.unwrap_or(if timed { 1_000_000 } else { 20_000 });
    assert!(
        table_entries <= 16_384
            && (1..=MAX_DEPTH).contains(&depth)
            && (1..=1_000_000).contains(&nodes)
    );
    let mut cases = vec![
        ("standard", STANDARD_START_FEN),
        (
            "developed",
            "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 w - - 0 1",
        ),
        ("recapture", "r6k/8/8/8/8/8/q7/R6K w - - 0 1"),
        ("endgame", "7k/8/2K5/8/8/8/P7/R7 w - - 0 1"),
    ];
    if timed {
        cases.push((
            "historical-queen-grab",
            "rn3rk1/p4ppp/3bpP2/1Q6/3P4/2q2N2/5PPP/R1B2RK1 b - - 0 15",
        ));
    }
    for round in 0..rounds {
        for &(name, fen) in &cases {
            let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::AutoClaim).unwrap();
            let original = game.clone();
            let start = Instant::now();
            let result = search_with_options(
                &game,
                SearchLimits {
                    depth,
                    nodes: Some(nodes),
                },
                SearchOptions {
                    table_entries,
                    time: timed.then(|| TimeBudget::from_hard_ms(1000)),
                    ..SearchOptions::default()
                },
                || false,
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
                "depth_limit":depth,"node_limit":nodes,"hard_ms":if timed {1000} else {0},"elapsed_ms":elapsed_ms,
                "table_entries":table_entries,
                "result":format!("{:?}",result.result),"selective_depth":result.selective_depth,
                "main_nodes":s.main_nodes,"q_nodes":s.q_nodes,"see_probes":s.see_probes,
                "tt_probes":s.tt_probes,"tt_position_hits":s.tt_position_hits,
                "tt_context_hits":s.tt_context_hits,"tt_depth_hits":s.tt_hits,
                "tt_cutoffs":s.tt_cutoffs,"tt_stores":s.tt_stores,
                "q_tt_probes":s.q_tt_probes,"q_tt_hits":s.q_tt_hits,
                "q_tt_cutoffs":s.q_tt_cutoffs,"q_tt_stores":s.q_tt_stores,
                "tt_replacements":s.tt_replacements,"tt_store_refusals":s.tt_store_refusals,
                "stats":format!("{s:?}")})
            );
        }
    }
}
