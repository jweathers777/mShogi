//! Actual bundled native entry point rejects Tori execution explicitly and unscored.
use mshogi_core::{native::CanonicalRecord, tori};
use mshogi_engine::*;
use std::{
    io::Write,
    process::{Command, Stdio},
};
#[test]
fn real_subprocess_preserves_tori_context_but_supplies_no_move_score_or_outcome() {
    let mut game = tori::Game::standard();
    let action = game.legal_actions()[0];
    game.submit(tori::GameRequest {
        expected: game.context(),
        actor: tori::Side::Sente,
        command: tori::GameCommand::Play { action },
    })
    .unwrap();
    let request = ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 129,
            session_version: 9,
            process_generation: 2,
            game: CanonicalRecord::tori(tori::GameRecord::from_game(&game).unwrap()),
            limits: WorkLimits {
                depth: 1,
                nodes: 100,
            },
        },
    };
    let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi"))
        .arg("internal-search")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    writeln!(
        child.stdin.take().unwrap(),
        "{}",
        serde_json::to_string(&request).unwrap()
    )
    .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{:?}", output.stderr);
    let response: ExecutionResponse = serde_json::from_slice(&output.stdout).unwrap();
    assert!(response.matches(&request));
    assert_eq!(response.observation.context, request.context);
    assert!(matches!(
        response.observation.result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                ..
            }
        }
    ));
}
