//! Developer measurement of intrinsic proof, not a console/product capability.
use mshogi_core::xiangqi::*;
use serde::Serialize;
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};
#[derive(Serialize)]
struct Observation {
    name: &'static str,
    fen: &'static str,
    limits: ExactLimits,
    report: ExactReport,
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let states = args
        .get(1)
        .map(|s| s.parse().expect("state limit"))
        .unwrap_or(20_000);
    let millis = args
        .get(2)
        .map(|s| s.parse().expect("millisecond limit"))
        .unwrap_or(1000);
    let mut output = io::BufWriter::new(io::stdout().lock());
    for (name, fen) in [
        ("bare_generals", "5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1"),
        ("one_advisor", "5k3/9/9/9/9/9/9/9/4A4/3K5 w - - 0 1"),
        ("two_advisors", "5k3/9/9/9/9/9/9/9/4A4/3K1A3 w - - 0 1"),
        ("one_elephant", "5k3/9/9/9/9/9/9/9/9/2BK5 w - - 0 1"),
        ("two_elephants", "5k3/9/9/9/9/9/9/9/9/2BK2B2 w - - 0 1"),
        ("advisor_elephant", "5k3/9/9/9/9/9/9/9/4A4/2BK5 w - - 0 1"),
        ("one_cannon", "5k3/9/9/9/9/9/9/9/C8/3K5 w - - 0 1"),
        ("one_horse", "5k3/9/9/9/9/9/9/9/N8/3K5 w - - 0 1"),
        ("one_rook_two_ply", "5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1"),
        ("rook_advisor", "5k3/R8/9/9/9/9/9/9/4A4/3K5 b - - 0 1"),
        ("rook_horse", "5k3/R8/9/9/9/9/9/9/N8/3K5 b - - 0 1"),
        ("one_soldier", "5k3/9/9/9/9/P8/9/9/9/3K5 w - - 0 1"),
        ("two_cannons", "5k3/9/9/9/9/9/9/9/CC7/3K5 w - - 0 1"),
        (
            "four_defenders_each",
            "2b1kab2/9/5a3/9/9/9/9/5A3/9/2BK1AB2 w - - 0 1",
        ),
        ("standard", INITIAL_FEN),
    ] {
        for traversal in [ExactTraversal::BreadthFirst, ExactTraversal::DepthFirst] {
            let limits = ExactLimits {
                states,
                transitions: 20_000_000,
                millis,
                memory_bytes: 128 * 1024 * 1024,
                traversal,
                ..ExactLimits::default()
            };
            let position = Position::from_fen(fen).unwrap();
            position.validate_local().unwrap();
            let report = position.verify_natural_win(limits, || false);
            if let ExactVerdict::NaturalWin {
                winner,
                reason,
                witness,
            } = &report.verdict
            {
                let mut replay = Position::from_diagram(
                    &position.pieces().collect::<Vec<_>>(),
                    position.side_to_move(),
                    Counters::unknown(),
                )
                .unwrap();
                for &action in witness {
                    assert!(matches!(
                        replay.local_status(),
                        Ok(LocalStatus::Ongoing { .. })
                    ));
                    replay.apply(action).unwrap();
                }
                assert_eq!(replay.side_to_move().opponent(), *winner);
                assert_eq!(
                    replay.local_status().unwrap(),
                    LocalStatus::NoLegalAction { reason: *reason }
                );
            }
            serde_json::to_writer(
                &mut output,
                &Observation {
                    name,
                    fen,
                    limits,
                    report,
                },
            )
            .unwrap();
            writeln!(output).unwrap();
            output.flush().unwrap();
        }
    }
    let start = Instant::now();
    let report = Position::initial().verify_natural_win(
        ExactLimits {
            states: MAX_EXACT_STATES,
            millis: MAX_EXACT_MILLIS,
            memory_bytes: 128 * 1024 * 1024,
            ..ExactLimits::default()
        },
        || start.elapsed() >= Duration::from_millis(10),
    );
    serde_json::to_writer(
        &mut output,
        &serde_json::json!({"name":"cancel_after_10ms","report":report}),
    )
    .unwrap();
    writeln!(output).unwrap();
}
