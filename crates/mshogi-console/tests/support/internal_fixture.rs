//! Separate test helper: never copied into a desktop distributable.
use mshogi_engine::*;
use std::{
    io::{self, BufRead, Write},
    thread,
    time::Duration,
};
fn main() {
    let mut text = String::new();
    io::stdin().lock().read_line(&mut text).unwrap();
    // Explicit v3 verification path, not an execution capability. The shared
    // supervisor must reject forged echoes and retire old work across games.
    if serde_json::from_str::<serde_json::Value>(&text).unwrap()["protocol"]
        == SHARED_PROTOCOL_VERSION
    {
        let mut context: SharedRequest = serde_json::from_str(&text).unwrap();
        if context.request_id == 5 {
            thread::sleep(Duration::from_secs(30));
            return;
        }
        match context.request_id {
            20 => context.process_generation += 1,
            21 => context.request_id += 1,
            22 => context.session_version += 1,
            23 => context.limits.nodes += 1,
            24 => context.purpose = SharedPurpose::EngineTurn {},
            25 => {
                if let SharedPurpose::MoveAnalysis { target } = &mut context.purpose {
                    target.selection_id += 1
                }
            }
            26 => {
                context.game = mshogi_core::native::CanonicalRecord::shogi(
                    mshogi_core::shogi::GameRecord::from_game(&mshogi_core::shogi::Game::new()),
                )
            }
            // Same Xiangqi board and turn, different authoritative audit history.
            27 => {
                if let mshogi_core::native::CanonicalRecord::Xiangqi(record) = &context.game {
                    let mut game = record.to_game().unwrap();
                    game.submit(mshogi_core::xiangqi::GameRequest {
                        expected: game.context(),
                        actor: mshogi_core::xiangqi::Side::Red,
                        command: mshogi_core::xiangqi::GameCommand::ClaimNaturalCount,
                    })
                    .unwrap();
                    context.game = mshogi_core::native::CanonicalRecord::xiangqi(
                        mshogi_core::xiangqi::GameRecord::from_game(&game).unwrap(),
                    );
                }
            }
            _ => {}
        }
        let result = SharedResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                message: "Separate supervisor verification fixture; no native search performed"
                    .into(),
            },
        };
        println!(
            "{}",
            serde_json::to_string(&SharedResponse { context, result }).unwrap()
        );
        return;
    }
    let mut context: EngineRequest = serde_json::from_str(&text).unwrap();
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
            let bytes = vec![b'x'; MAX_RESPONSE_BYTES + 2];
            let _ = io::stdout().write_all(&bytes);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_secs(30));
            return;
        }
        5 => {
            thread::sleep(Duration::from_secs(30));
            return;
        }
        6 => context.process_generation += 1,
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
        _ => {}
    }
    let result = EngineResult::Incomplete {
        telemetry: Telemetry {
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::NodeLimit,
        },
    };
    println!(
        "{}",
        serde_json::to_string(&EngineResponse { context, result }).unwrap()
    );
}
