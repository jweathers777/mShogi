//! Actual bundled Tori entry point.
mod support;
use mshogi_core::{game::CanonicalRecord, tori};
use mshogi_engine::*;
#[test]
fn real_subprocess_preserves_tori_context_supplies_action_and_roundtrips_observation() {
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
            game: CanonicalRecord::tori(tori::GameRecord::from_game(&game).unwrap()),
            limits: WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 100,
            },
        },
    };
    let output = support::cli::run(
        &["internal-search"],
        format!("{}\n", serde_json::to_string(&request).unwrap()),
    );
    support::cli::success(&output);
    let response: ExecutionResponse = ExecutionResponse::decode(&output.stdout, &request).unwrap();
    assert!(response.matches(&request));
    assert_eq!(response.observation.context, request.context);
    let ExecutionResult::Tori { result } = &response.observation.result else {
        panic!("expected actual Tori result")
    };
    result.validate(&game).unwrap();
    let ToriResult::Choice {
        choice: action,
        telemetry,
    } = result
    else {
        panic!("complete depth 1 must select a legal action")
    };
    assert!(game.legal_actions().contains(action));
    assert_eq!(telemetry.perspective, tori::Side::Gote);
    assert_eq!(telemetry.completed_depth, 1);
    assert!(telemetry.score.is_some());
    let accepted_action = *action;
    // The engine's action is played into the saved game; engine results
    // themselves are not saved (MSH-228).
    use mshogi_desktop_session::{game_adapter::Tori, game_document::ValidatedGameDocument};
    let document = ValidatedGameDocument::<Tori>::new(
        tori::GameRecord::from_game(&game).unwrap(),
        tori::Side::Sente,
    )
    .unwrap()
    .with_command(
        tori::Side::Gote,
        tori::GameCommand::Play {
            action: accepted_action,
        },
        &mut || false,
    )
    .unwrap();
    let reopened = ValidatedGameDocument::<Tori>::decode(document.bytes(), &mut || false).unwrap();
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
}
