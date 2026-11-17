//! Measure the shared df-pn mate search alone on a Shogi tsume catalog.
//!
//! Usage: shogi_tsume_solve SUITE.json NODE_LIMIT OUT.json
//!
//! Each position gets a fresh table and the node limit. A reported mate is
//! replayed with the native rules: every move must be legal and the final
//! position must be checkmate. `first_in_expected` compares the first move
//! with the catalog's shortest-mate answer set (a longer forced mate can
//! still be correct).
use mshogi_core::shogi::Position;
use mshogi_search::alphabeta::{
    SearchPosition,
    mate::{self, MateLimits, MateResult, MateTable},
};
use mshogi_search::shogi::position::ShogiPosition;
use serde_json::{Value, json};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let suite: Value = serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let limit: u64 = args[2].parse().unwrap();
    let mut rows = vec![];
    for p in suite["positions"].as_array().unwrap() {
        let native = Position::from_sfen(p["sfen"].as_str().unwrap()).unwrap();
        let mut position = ShogiPosition::from_position(&native);
        let mut table = MateTable::new(20);
        let start = Instant::now();
        let (result, nodes) = mate::search(
            &mut position,
            &mut table,
            MateLimits {
                nodes: limit,
                deadline: None,
            },
            &mut || false,
        );
        let seconds = start.elapsed().as_secs_f64();
        let (status, first, plies, native_mate) = match &result {
            MateResult::Mate(line) => {
                let mut replay = native.clone();
                let mut ok = true;
                for &m in line {
                    ok &= replay.apply(position.action(m)).is_ok() && position.make(m);
                }
                for _ in 0..line.len() {
                    position.unmake();
                }
                ok &=
                    replay.legal_actions().is_empty() && replay.is_in_check(replay.side_to_move());
                let first = position.action(line[0]).to_string();
                ("mate", Some(first), line.len(), ok)
            }
            MateResult::NoMate => ("no_mate", None, 0, false),
            MateResult::Unknown => ("unknown", None, 0, false),
        };
        let expected: Vec<&str> = p["expected_moves"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m.as_str().unwrap())
            .collect();
        rows.push(json!({
            "id": p["id"], "split": p["split"], "mate_plies": p["mate_plies"], "status": status,
            "first": first, "first_in_expected": first.as_deref().is_some_and(|f| expected.contains(&f)),
            "line_plies": plies, "native_checkmate": native_mate, "nodes": nodes, "seconds": seconds,
        }));
    }
    let solved: Vec<&Value> = rows
        .iter()
        .filter(|r| r["native_checkmate"] == true)
        .collect();
    let mut times: Vec<f64> = rows
        .iter()
        .map(|r| r["seconds"].as_f64().unwrap())
        .collect();
    times.sort_by(f64::total_cmp);
    let summary = json!({
        "positions": rows.len(), "node_limit": limit,
        "native_checkmate": solved.len(),
        "first_in_expected": rows.iter().filter(|r| r["first_in_expected"] == true).count(),
        "reported_without_native_mate": rows.iter().filter(|r| r["status"] == "mate" && r["native_checkmate"] != true).count(),
        "no_mate": rows.iter().filter(|r| r["status"] == "no_mate").count(),
        "unknown": rows.iter().filter(|r| r["status"] == "unknown").count(),
        "median_seconds": times[times.len() / 2], "max_seconds": times[times.len() - 1],
    });
    println!("{summary}");
    std::fs::write(
        &args[3],
        serde_json::to_string_pretty(&json!({"summary": summary, "rows": rows})).unwrap(),
    )
    .unwrap();
}
