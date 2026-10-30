//! The R3 application contract is exercised through real native documents.
use super::*;
use mshogi_core::native::{GameKind, NativeSide};
fn controller() -> DesktopController {
    let mut controller = DesktopController::new("/unused/r3-engine".into()).unwrap();
    controller.shared.manual_engine = true;
    controller
}
fn settle(controller: &mut DesktopController, receipt: &AdmissionReceipt) {
    let deadline = Instant::now() + std::time::Duration::from_secs(10);
    while receipt.completion().is_none() {
        controller.shared_snapshot();
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    receipt.completion().unwrap().unwrap();
}
#[test]
fn exactly_one_native_document_and_uniform_metadata_survive_cross_game_replacement() {
    let mut c = controller();
    let mut previous = c.shared_token();
    for human in [
        NativeSide::Shogi(mshogi_core::shogi::Side::Gote),
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Red),
        NativeSide::Chess(Color::White),
    ] {
        let (_, receipt) = c
            .shared_new_game(c.shared_open_token(), human.game(), human)
            .unwrap();
        settle(&mut c, &receipt);
        let snapshot = c.shared_snapshot();
        assert_eq!(snapshot.game, human.game());
        assert_eq!(snapshot.human.id, human);
        assert!(snapshot.token.document > previous.document);
        assert!(snapshot.token.version > previous.version);
        assert!(snapshot.token.selection > previous.selection);
        match (&c.session, human.game()) {
            (ActiveDocument::Chess(_), GameKind::Chess)
            | (ActiveDocument::Shogi { .. }, GameKind::Shogi)
            | (ActiveDocument::Xiangqi { .. }, GameKind::Xiangqi) => {}
            _ => panic!("exactly one authoritative native variant must match the public contract"),
        }
        assert_eq!(c.session.document_epoch, snapshot.token.document);
        assert_eq!(c.session.document_revision, snapshot.document.revision);
        assert_eq!(snapshot.chess.is_some(), human.game() == GameKind::Chess);
        assert_eq!(
            snapshot.chess_match.is_some(),
            human.game() == GameKind::Chess
        );
        previous = snapshot.token;
    }
}
#[test]
fn current_chess_contract_contains_actual_claims_match_and_search_observations() {
    let game =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 100 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let mut native = DesktopSession::from_record(&game.record(), Color::White).unwrap();
    native
        .command(
            ChessMutationCommand::Analyze {
                version: native.version,
                operation: AnalysisOperation::Moves {
                    depth: 1,
                    nodes: 100,
                },
            },
            1,
        )
        .unwrap();
    let request = native.analysis_request().unwrap();
    let mut encoded = serde_json::to_vec(&request).unwrap();
    encoded.push(b'\n');
    let mut bytes = Vec::new();
    mshogi_engine::serve_once(std::io::Cursor::new(encoded), &mut bytes).unwrap();
    let response: ExecutionResponse = serde_json::from_slice(&bytes).unwrap();
    assert!(native.accept_analysis(response));
    let expected = native.game.record().to_json();
    let observation = native.observations[0].observation.clone();
    let mut c = controller();
    c.session = ActiveDocument::Chess(Box::new(native));
    let snapshot = c.shared_snapshot();
    assert!(!snapshot.chess.as_ref().unwrap().claims.is_empty());
    assert!(snapshot.capabilities.chess_full);
    assert_eq!(snapshot.chess_match.unwrap().to_json(), expected);
    assert_eq!(snapshot.observations.len(), 1);
    assert_eq!(snapshot.observations[0].observation, observation);
    assert_eq!(snapshot.analysis.as_ref().unwrap().observation, observation);
    assert_eq!(snapshot.search_observations.len(), 1);
    assert!(snapshot.search_observations[0].selected);
    assert_eq!(snapshot.search_observations[0].perspective, "White");
    assert!(snapshot.search_observations[0].nodes.is_some());
    assert!(snapshot.chess.as_ref().unwrap().live_conclusion.is_none());
}
#[test]
fn rejected_chess_worker_command_is_not_capacity_engine_failure_or_outcome() {
    let mut c = controller();
    let before = c.shared_snapshot();
    let (_, receipt) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: c.shared_token(),
            game: GameKind::Chess,
            operation: SessionOperation::ChessPlay {
                action: "e2e5".parse().unwrap(),
            },
        })
        .unwrap();
    let deadline = Instant::now() + std::time::Duration::from_secs(10);
    while receipt.completion().is_none() {
        c.shared_snapshot();
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(receipt.completion().unwrap().is_err());
    let after = c.shared_snapshot();
    assert_eq!(after.context, before.context);
    assert_eq!(after.capacity.status, CapacityPhase::Available);
    assert!(after.capacity.resource.is_none());
    assert!(after.chess.as_ref().unwrap().live_conclusion.is_none());
    assert_ne!(after.phase, EnginePhase::Failed);
}
#[test]
fn close_authority_rejects_all_native_replacement_commands_and_open_but_keeps_save_available() {
    let mut c = controller();
    let expected = c.shared_open_token();
    c.lifecycle.request_close().unwrap();
    assert!(
        c.command(ChessMutationCommand::Resign {
            version: c.session.version
        })
        .is_err()
    );
    assert!(
        c.queue_shared_open("/unused/r3.mshogi".into(), expected)
            .is_err()
    );
    assert!(
        c.shared_new_game(expected, GameKind::Chess, NativeSide::Chess(Color::White))
            .is_err()
    );
    assert!(
        c.shared_command(SharedDesktopCommand {
            expected: c.shared_token(),
            game: GameKind::Chess,
            operation: SessionOperation::Resume {}
        })
        .is_err()
    );
    let snapshot = c.application_pause_for_close();
    assert!(snapshot.capabilities.save);
    c.lifecycle.cancel_close().unwrap();
    assert!(!c.lifecycle.close_requested());
}

#[test]
fn direct_identity_refusals_publish_native_facts_without_mutating_the_game() {
    let mut c = controller();
    let before = c.shared_snapshot().context;
    c.io_state.id = MAX_SAFE_ID - 1;
    assert!(
        c.queue_shared_save("/unused/no-write.mshogi".into())
            .is_err()
    );
    let snapshot = c.shared_snapshot();
    assert_eq!(snapshot.context, before);
    let fact = &snapshot.io.resource.as_ref().unwrap().facts()[0];
    assert_eq!(fact.category, ResourceCategory::IdentityHeadroom);
    assert_eq!(fact.measurement, ResourceMeasurement::Accepted);
    c.io_state.id = 0;
    c.session.capacity.id = MAX_SAFE_ID - 32;
    assert!(
        c.command(ChessMutationCommand::Preferences {
            orientation: Color::Black,
            analysis_defaults: AnalysisOperation::Moves { depth: 1, nodes: 1 }
        })
        .is_err()
    );
    let snapshot = c.shared_snapshot();
    assert_eq!(snapshot.context, before);
    assert_eq!(
        snapshot.capacity.resource.unwrap().facts()[0].category,
        ResourceCategory::IdentityHeadroom
    );
}
#[test]
fn reopened_all_game_identities_exceed_runtime_only_revisions_without_spending_saved_watermarks() {
    fn file(c: &mut DesktopController, id: u64) {
        let deadline = Instant::now() + std::time::Duration::from_secs(10);
        loop {
            let snapshot = c.shared_snapshot();
            if snapshot.io.id == id
                && matches!(snapshot.io.phase, IoPhase::Complete | IoPhase::Failed)
            {
                assert_eq!(snapshot.io.phase, IoPhase::Complete, "{:?}", snapshot.io);
                break;
            }
            assert!(Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    let mut c = controller();
    for (index, human) in [
        NativeSide::Chess(Color::White),
        NativeSide::Shogi(mshogi_core::shogi::Side::Sente),
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Red),
    ]
    .into_iter()
    .enumerate()
    {
        let (_, receipt) = c
            .shared_new_game(c.shared_open_token(), human.game(), human)
            .unwrap();
        settle(&mut c, &receipt);
        let canonical = c.shared_snapshot().context;
        let persisted = c.session.archive_revision;
        c.session.revision += 1000;
        let path = std::env::temp_dir().join(format!(
            "msh118-r3-monotonic-{}-{index}.mshogi",
            std::process::id()
        ));
        let id = c.queue_shared_save(path.clone()).unwrap();
        file(&mut c, id);
        assert_eq!(c.session.archive_revision, persisted);
        c.session.revision += 1000;
        let old_revision = c.session.revision;
        let id = c
            .queue_shared_open(path.clone(), c.shared_open_token())
            .unwrap();
        file(&mut c, id);
        let opened = c.shared_snapshot();
        assert_eq!(opened.context, canonical);
        assert!(opened.paused);
        assert!(opened.token.revision > old_revision);
        assert!(opened.token.version > old_revision);
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn current_flat_chess_claim_keeps_the_intended_action_unplayed_and_rejects_old_transport() {
    let game =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let mut c = controller();
    c.session = ActiveDocument::Chess(Box::new(
        DesktopSession::from_record(&game.record(), Color::White).unwrap(),
    ));
    let before = c.shared_snapshot();
    let claim = before
        .chess
        .as_ref()
        .unwrap()
        .claims
        .iter()
        .find(|claim| claim.intended.is_some())
        .copied()
        .unwrap();
    let command = SharedDesktopCommand {
        expected: before.token,
        game: GameKind::Chess,
        operation: SessionOperation::ChessClaim { claim },
    };
    let json = serde_json::to_string(&command).unwrap();
    assert!(json.contains("\"kind\":\"chess_claim\""));
    assert!(!json.contains("\"command\""));
    let (_, receipt) = c
        .shared_command_with_receipt(SharedDesktopCommand::from_json(&json).unwrap())
        .unwrap();
    settle(&mut c, &receipt);
    let after = c.shared_snapshot();
    assert_eq!(after.board_ply, 0);
    assert_eq!(
        c.session.chess().game.game().position().to_fen(),
        game.position().to_fen()
    );
    assert!(matches!(
        after.chess.unwrap().live_conclusion,
        Some(MatchConclusion::Chess(ChessOutcome::Draw(
            DrawReason::Claimed(_)
        )))
    ));
    assert!(
        c.shared_command(SharedDesktopCommand::from_json(&json).unwrap())
            .is_err(),
        "the exact old outer token cannot be reused"
    );
    let old = serde_json::json!({"expected":before.token,"game":"chess","operation":{"kind":"chess","command":{"kind":"claim","version":before.token.version,"claim":claim}}});
    assert!(
        SharedDesktopCommand::from_json(&old.to_string()).is_err(),
        "retired nested command is not a compatibility transport"
    );
    let duplicate = json.replace(
        "\"claim\":",
        &format!(
            "\"claim\":{},\"claim\":",
            serde_json::to_string(&claim).unwrap()
        ),
    );
    assert!(SharedDesktopCommand::from_json(&duplicate).is_err());
    let duplicate_action = format!(
        "{{\"expected\":{},\"game\":\"chess\",\"operation\":{{\"kind\":\"chess_play\",\"action\":\"e2e4\",\"action\":\"d2d4\"}}}}",
        serde_json::to_string(&before.token).unwrap()
    );
    assert!(SharedDesktopCommand::from_json(&duplicate_action).is_err());
}

#[test]
fn current_chess_presentation_has_native_capabilities_without_a_second_application_snapshot() {
    let mut c = controller();
    let value = serde_json::to_value(c.shared_snapshot()).unwrap();
    for field in [
        "version",
        "revision",
        "document",
        "io",
        "capacity",
        "view",
        "pieces",
        "legal_choices",
        "human",
        "turn",
    ] {
        assert!(
            value["chess"].get(field).is_none(),
            "{field} belongs only to the common application snapshot"
        );
    }
    for field in [
        "claims",
        "live_conclusion",
        "selected_conclusion",
        "in_check",
        "analysis",
        "last_engine_result",
        "last_engine_turn",
        "searched_version",
        "rules_mode",
    ] {
        assert!(
            value["chess"].get(field).is_some(),
            "native Chess capability {field} remains available"
        );
    }
}
