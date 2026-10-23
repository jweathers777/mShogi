//! Repeatable perft timing baseline; a correctness-diagnostic speed record,
//! not a search-speed or playing-strength claim.
//!
//! Run: `cargo run --release --locked -p mshogi-core --example perft_baseline`

use std::time::Instant;

use mshogi_core::chess::Position;

// Totals are the independent expectations in tests/fixtures/chess/divide.json.
const CASES: [(&str, &str, u32, u64); 3] = [
    (
        "initial",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        4,
        197_281,
    ),
    (
        "kiwipete",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        4,
        4_085_603,
    ),
    (
        "promotion-castling",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        4,
        422_333,
    ),
];

// Optional deep totals published in Stockfish sf_17.1 tests/perft.sh (source
// registered with its SHA-256 in tests/fixtures/chess/corpus.json). Run with
// the `upstream` argument; each takes about a minute in a release build.
const UPSTREAM: [(&str, &str, u32, u64); 3] = [
    (
        "kiwipete-sf",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        5,
        193_690_690,
    ),
    (
        "tactical-sf",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        5,
        89_941_194,
    ),
    (
        "middlegame-sf",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        5,
        164_075_551,
    ),
];

fn main() {
    let upstream = std::env::args().any(|arg| arg == "upstream");
    println!("case depth nodes seconds nodes_per_second");
    let cases = CASES
        .iter()
        .chain(if upstream { &UPSTREAM[..] } else { &[] });
    for &(name, fen, depth, expected) in cases {
        let position = Position::from_fen(fen).expect("valid reference FEN");
        let start = Instant::now();
        let nodes = position.perft(depth).expect("perft");
        let seconds = start.elapsed().as_secs_f64();
        assert_eq!(nodes, expected, "{name}: independent total");
        println!(
            "{name} {depth} {nodes} {seconds:.3} {:.0}",
            nodes as f64 / seconds
        );
    }
}
