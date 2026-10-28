//! Reproducible bounded Xiangqi workload; elapsed time is an observation, not a promise.
use mshogi_core::xiangqi::{
    ClaimPolicy, ExactLimits, Game, GameCommand, GameRequest, PositionRoot,
};
use mshogi_search::xiangqi::*;
use std::time::Instant;
fn game(fen: &str) -> Game {
    Game::from_root(PositionRoot::from_fen(fen).unwrap(), ClaimPolicy::Explicit).unwrap()
}
fn main() {
    println!(
        "evaluator={EVALUATOR_ID} search={SEARCH_ID} mode=sound_fast_only build=release-recommended"
    );
    let mut warning = game("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1");
    for text in ["e9f9", "f8e8", "f9e9", "e8f8"]
        .into_iter()
        .cycle()
        .take(10)
    {
        warning
            .submit(GameRequest {
                expected: warning.context(),
                actor: warning.position().side_to_move(),
                command: GameCommand::Play {
                    action: text.parse().unwrap(),
                },
            })
            .unwrap();
    }
    let cases = [
        (
            "standard-44",
            Game::standard(ClaimPolicy::Explicit).unwrap(),
        ),
        ("open-rook", game("5k3/9/9/9/9/4R4/9/9/9/4K4 w - - 0 1")),
        (
            "cannon-screen",
            game("5k3/9/2r6/9/2P6/9/2C6/9/9/3K5 w - - 0 1"),
        ),
        ("horse-leg", game("5k3/9/9/9/3r5/9/2N6/9/9/3K5 w - - 0 1")),
        (
            "check-evasions",
            game("4r4/5k3/9/9/9/9/9/9/9/4K4 w - - 0 1"),
        ),
        ("warning-10plies", warning),
        (
            "exact-negative-domain",
            game("5k3/9/9/9/9/9/9/9/9/3AK4 w - - 0 1"),
        ),
    ];
    for (name, g) in &cases {
        for limits in [
            DEFAULT_LIMITS,
            Limits {
                depth: MAX_DEPTH,
                nodes: MAX_NODES,
            },
        ] {
            let started = Instant::now();
            let result = search(g, limits).unwrap();
            println!(
                "case={name} actions={} limits={limits:?} nodes={} depth={} status={:?} score={:?} elapsed_ms={:.3}",
                g.position().legal_actions().unwrap().len(),
                result.nodes,
                result.completed_depth,
                result.status,
                result.score,
                started.elapsed().as_secs_f64() * 1000.0
            );
        }
    }
    for (name, g, states) in [
        ("exact-negative", &cases[6].1, 1000),
        ("exact-unknown", &cases[0].1, 0),
    ] {
        let started = Instant::now();
        let result = search_requiring_root_exact(
            g,
            DEFAULT_LIMITS,
            ExactLimits {
                states,
                millis: 5000,
                ..ExactLimits::default()
            },
            || false,
        )
        .unwrap();
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        let AssessmentEvidence::Exact(observation) = &result.assessment else {
            panic!("explicit exact evidence")
        };
        println!(
            "case={name} status={:?} decision={:?} score={:?} elapsed_ms={elapsed:.3} limits={:?} source_bytes={} report={:?}",
            result.status,
            result.decision,
            result.score,
            observation.request().limits(),
            observation.request().source_record().to_json().len(),
            observation.report()
        );
    }
}
