//! Separate test helper: never copied into a desktop distributable.
use mshogi_core::native::{CanonicalRecord, GameKind};
use mshogi_engine::*;
use std::{
    io::{self, BufRead, Write},
    thread,
    time::Duration,
};
fn main() {
    let mut text = String::new();
    io::stdin().lock().read_line(&mut text).unwrap();
    let request: ExecutionRequest = serde_json::from_str(&text).unwrap();
    let mut context = request.context;
    // Separately compiled test helper, never a bundled product engine.
    if context.game.game_kind() == GameKind::Tori {
        if context.limits.nodes == 7 {
            if let (ExecutionPurpose::MoveAnalysis { .. }, CanonicalRecord::Tori(record)) =
                (&context.purpose, &context.game)
            {
                // This separately compiled fixture reads its explicit test-root
                // provenance. Real target labels need no verification-only PID.
                let game = record.to_game().unwrap();
                let host = game
                    .position()
                    .root()
                    .source()
                    .and_then(|source| source.strip_prefix("msh42-controller-"))
                    .and_then(|s| s.parse::<u32>().ok())
                    .expect("explicit test-root marker namespace");
                let marker = std::env::temp_dir()
                    .join(format!("msh131-started-{host}-{}.txt", context.request_id));
                let temporary = marker.with_extension("tmp");
                std::fs::write(&temporary, std::process::id().to_string()).unwrap();
                std::fs::rename(temporary, marker).unwrap();
            }
            thread::sleep(Duration::from_secs(30));
            return;
        }
        if context.limits.nodes == 1 {
            eprintln!("Tori supervised forced process failure");
            std::process::exit(17);
        }
    }

    match context.request_id {
        1 => return,
        2 => {
            eprintln!("fixture process failure");
            std::process::exit(17);
        }
        3 => {
            println!("not json");
            return;
        }
        4 => {
            let _ = io::stdout().write_all(&vec![b'x'; MAX_RESPONSE_BYTES + 2]);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_secs(30));
            return;
        }
        5 => {
            thread::sleep(Duration::from_secs(30));
            return;
        }
        6 | 20 => context.process_generation += 1,
        9 => {
            let _ = io::stderr().write_all(&vec![b'e'; MAX_STDERR_BYTES * 10]);
            std::process::exit(19);
        }
        10 => {
            print!("{{}}");
            return;
        }
        11 => {
            println!("{{}}\n{{}}");
            return;
        }
        21 => context.request_id += 1,
        22 => context.session_version += 1,
        23 => context.limits.nodes += 1,
        24 => context.purpose = ExecutionPurpose::EngineTurn {},
        25 => {
            if let ExecutionPurpose::MoveAnalysis { target } = &mut context.purpose {
                target.selection_id += 1;
            }
        }
        26 => {
            context.game = CanonicalRecord::shogi(mshogi_core::shogi::GameRecord::from_game(
                &mshogi_core::shogi::Game::new(),
            ))
        }
        // Same Xiangqi board and turn, different authoritative audit history.
        27 => {
            if let CanonicalRecord::Xiangqi(record) = &context.game {
                let mut game = record.to_game().unwrap();
                game.submit(mshogi_core::xiangqi::GameRequest {
                    expected: game.context(),
                    actor: mshogi_core::xiangqi::Side::Red,
                    command: mshogi_core::xiangqi::GameCommand::ClaimNaturalCount,
                })
                .unwrap();
                context.game = CanonicalRecord::xiangqi(
                    mshogi_core::xiangqi::GameRecord::from_game(&game).unwrap(),
                );
            }
        }
        _ => {}
    }
    let result = if context.game.game_kind() == GameKind::Chess && context.request_id < 20 {
        ExecutionResult::Chess {
            result: ChessResult::Incomplete {
                telemetry: Telemetry {
                    score: None,
                    completed_depth: 0,
                    nodes: 0,
                    principal_variation: vec![],
                    status: StopReason::NodeLimit,
                },
            },
        }
    } else {
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                message: "Separate supervisor verification fixture; no native search performed"
                    .into(),
            },
        }
    };
    println!(
        "{}",
        serde_json::to_string(&ExecutionResponse {
            protocol: PROTOCOL_VERSION,
            observation: ExecutionObservation::new(context, result),
        })
        .unwrap()
    );
}
