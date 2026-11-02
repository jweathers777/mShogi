//! Actual bundled native Tori entry point and retained historical failure compatibility.
use mshogi_core::{native::CanonicalRecord, tori};
use mshogi_engine::*;
use std::{
    io::Write,
    process::{Command, Stdio},
};
#[test]
fn real_subprocess_preserves_tori_context_supplies_native_action_and_roundtrips_observation() {
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
    let ExecutionResult::Tori { result } = &response.observation.result else {
        panic!("expected actual Tori result")
    };
    result.validate(&game).unwrap();
    let ToriResult::Choice { action, telemetry } = result else {
        panic!("complete depth1 must select a legal action")
    };
    assert!(game.legal_actions().contains(action));
    assert_eq!(telemetry.perspective, tori::Side::Gote);
    assert_eq!(telemetry.completed_depth, 1);
    assert!(telemetry.score.is_some());
    let accepted_action = *action;
    // Engine-turn admission commits the revalidated action exactly once. Decode
    // then replays that journal; it never resumes pending execution.
    use mshogi_desktop_session::tori_document::ValidatedToriDocument;
    let document = ValidatedToriDocument::new(
        tori::GameRecord::from_game(&game).unwrap(),
        tori::Side::Sente,
    )
    .unwrap()
    .with_observation(response.observation.clone(), &mut || false)
    .unwrap();
    let reopened = ValidatedToriDocument::decode(document.bytes(), &mut || false).unwrap();
    assert_eq!(reopened.observations(), &[response.observation]);
    game.submit(tori::GameRequest {
        expected: game.context(),
        actor: tori::Side::Gote,
        command: tori::GameCommand::Play {
            action: accepted_action,
        },
    })
    .unwrap();
    assert_eq!(
        reopened.record().to_json(),
        tori::GameRecord::from_game(&game).unwrap().to_json()
    );
    assert!(reopened.record().conclusion().is_none());
    // Old accepted unsupported failures keep their meaning and remain readable
    // after real execution is introduced. They never acquire move/score/outcome.
    let mut old_context = request.context;
    old_context.game = CanonicalRecord::tori(tori::GameRecord::from_game(&game).unwrap());
    old_context.request_id += 1;
    old_context.session_version += 1;
    old_context.process_generation += 1;
    let old_failure = ExecutionObservation::new(
        old_context,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                message: "historical pre-MSH42 unsupported execution".into(),
            },
        },
    );
    let document = reopened
        .with_observation(old_failure.clone(), &mut || false)
        .unwrap();
    let reopened = ValidatedToriDocument::decode(document.bytes(), &mut || false).unwrap();
    assert_eq!(reopened.observations().last(), Some(&old_failure));
}
