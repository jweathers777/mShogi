use super::*;
fn controller() -> DesktopController {
    DesktopController::new(PathBuf::from("/unused/msh72-engine")).unwrap()
}
fn imported(fen: &str) -> s::Game {
    s::Game::from_root(
        s::GameRoot::import(
            s::Position::from_sfen(fen).unwrap(),
            s::RootCategory::ReducedMaterialDiagram,
            s::CounterSource::Supplied,
        )
        .unwrap(),
    )
}
fn send(c: &mut DesktopController, op: SessionOperation) -> Result<SharedSnapshot, String> {
    let game = if c.shared.shogi.is_some() {
        GameKind::Shogi
    } else {
        GameKind::Chess
    };
    c.shared_command(SharedDesktopCommand {
        expected: c.shared_token(),
        game,
        operation: op,
    })
}
fn replace(c: &mut DesktopController, g: s::Game, human: s::Side) -> SharedSnapshot {
    send(
        c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&g)),
            human: NativeSide::Shogi(human),
        },
    )
    .unwrap()
}
fn analyze(c: &mut DesktopController) -> SharedRequest {
    send(
        c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        },
    )
    .unwrap();
    c.shared.shogi.as_ref().unwrap().pending.clone().unwrap()
}
fn incomplete(context: SharedRequest) -> SharedResponse {
    SharedResponse {
        result: SharedResult::Shogi {
            result: ShogiResult::Incomplete {
                telemetry: mshogi_engine::ShogiTelemetry {
                    evaluator_id: "mshogi.shogi.material.v1".into(),
                    search_id: "mshogi.shogi.full-width.v1".into(),
                    perspective: match &context.game {
                        CanonicalRecord::Shogi(r) => r.side_to_move(),
                        _ => unreachable!(),
                    },
                    score: None,
                    completed_depth: 0,
                    nodes: 0,
                    principal_variation: vec![],
                    status: mshogi_engine::StopReason::NodeLimit,
                },
            },
        },
        context,
    }
}
#[test]
fn identities_native_geometry_and_chess_compatibility() {
    let mut c = controller();
    let chess = c.shared_snapshot();
    assert_eq!(chess.game, GameKind::Chess);
    assert_eq!(chess.sides[0].label, "White");
    assert_eq!(chess.choices.len(), 20);
    assert!(chess.capabilities.chess_full);
    assert!(!chess.capabilities.hands);
    let old = chess.token;
    let shogi = replace(&mut c, s::Game::new(), s::Side::Sente);
    assert_eq!(shogi.profile.id(), s::PROFILE_ID);
    assert_eq!(shogi.edition, s::PROFILE_EDITION);
    assert_eq!(shogi.sides[0].label, "Sente");
    assert_eq!(shogi.sides[1].label, "Gote");
    assert_eq!(shogi.choices.len(), 30);
    assert_eq!(shogi.pieces.len(), 40);
    assert!(shogi.sides[0].first_mover);
    assert_ne!(chess.sides[0].key, shogi.sides[0].key);
    assert_eq!(
        (
            &shogi.orientations[0].files[0],
            &shogi.orientations[0].ranks[0]
        ),
        (&"9".into(), &"a".into())
    );
    assert_eq!(
        (
            &shogi.orientations[1].files[0],
            &shogi.orientations[1].ranks[0]
        ),
        (&"1".into(), &"i".into())
    );
    assert!(!shogi.capabilities.save);
    assert!(!shogi.capabilities.chess_full);
    assert!(!shogi.capabilities.claims);
    assert_eq!(shogi.hands.len(), 14);
    assert!(
        c.shared_command(SharedDesktopCommand {
            expected: old,
            game: GameKind::Chess,
            operation: SessionOperation::Cancel {}
        })
        .is_err()
    );
    assert!(
        c.shared_command(SharedDesktopCommand {
            expected: c.shared_token(),
            game: GameKind::Chess,
            operation: SessionOperation::Cancel {}
        })
        .is_err()
    );
    assert!(
        c.command(DesktopCommand::Resign {
            version: c.session.version
        })
        .is_err()
    );
    assert!(c.queue_save(PathBuf::from("/unused/save")).is_err());
    assert!(
        c.queue_open(PathBuf::from("/unused/open"), c.open_token())
            .is_err()
    );
    let snapshot = c.pause_for_close();
    assert!(snapshot.document.dirty);
    assert!(c.shared.shogi.as_ref().unwrap().paused);
}
#[test]
fn actual_complete_drop_promotion_capture_and_atomic_rejection() {
    let mut c = controller();
    let view = replace(
        &mut c,
        imported("8k/9/9/4P4/9/9/9/9/K8 b - 1"),
        s::Side::Sente,
    );
    let alternatives: Vec<_> = view
        .choices
        .iter()
        .filter(|c| c.from == "5d" && c.steps == ["5c"])
        .map(|c| c.action.as_str())
        .collect();
    assert_eq!(alternatives.len(), 2);
    assert!(alternatives.contains(&"5d5c"));
    assert!(alternatives.contains(&"5d5c+"));
    let forced = replace(
        &mut c,
        imported("8k/4P4/9/9/9/9/9/9/K8 b - 1"),
        s::Side::Sente,
    );
    assert!(forced.choices.iter().any(|c| c.action == "5b5a+"));
    assert!(!forced.choices.iter().any(|c| c.action == "5b5a"));
    replace(
        &mut c,
        imported("8k/9/4+p4/9/4R4/9/9/9/K8 b - 17"),
        s::Side::Sente,
    );
    send(&mut c, SessionOperation::Resume {}).unwrap();
    assert_eq!(c.shogi_snapshot().phase, EnginePhase::Idle);
    let before = c.shared_token();
    assert!(
        send(
            &mut c,
            SessionOperation::Play {
                action: "5e4d".parse().unwrap()
            }
        )
        .is_err()
    );
    assert_eq!(before, c.shared_token());
    let captured = send(
        &mut c,
        SessionOperation::Play {
            action: "5e5c+".parse().unwrap(),
        },
    )
    .unwrap();
    assert_eq!(
        captured
            .pieces
            .iter()
            .find(|p| p.square == "5c")
            .unwrap()
            .text,
        "+R"
    );
    assert_eq!(
        captured
            .hands
            .iter()
            .find(|h| h.owner.ends_with(":sente") && h.kind == "pawn")
            .unwrap()
            .count,
        1
    );
    let drops = replace(
        &mut c,
        imported("8k/9/9/9/9/9/9/9/K8 b 2RBGSNLP 1"),
        s::Side::Sente,
    );
    let drop = drops.choices.iter().find(|c| c.action == "P*5e").unwrap();
    assert!(matches!(drop.origin, ChoiceOrigin::Hand { .. }));
    assert!(drop.from.starts_with("hand:"));
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let dropped = send(
        &mut c,
        SessionOperation::Play {
            action: "P*5e".parse().unwrap(),
        },
    )
    .unwrap();
    assert_eq!(
        dropped
            .hands
            .iter()
            .find(|h| h.owner.ends_with(":sente") && h.kind == "pawn")
            .unwrap()
            .count,
        0
    );
    assert_eq!(dropped.turn.label, "Gote");
}
#[test]
fn declaration_advice_terminal_history_and_native_wrong_turn_semantics() {
    let mut c = controller();
    let root = imported("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1");
    let view = replace(&mut c, root.clone(), s::Side::Sente);
    assert!(view.declaration_advice[0].eligible);
    assert!(view.events.is_empty());
    let ended = send(&mut c, SessionOperation::Declare {}).unwrap();
    assert!(ended.status.label.contains("Sente wins"));
    assert_eq!(ended.event_index, 1);
    assert_eq!(ended.events[0].board_ply, 0);
    assert_eq!(ended.turn.label, "Sente");
    let before = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(before.historical);
    assert!(!before.status.concluded);
    assert_eq!(
        before.context,
        CanonicalRecord::shogi(s::GameRecord::from_game(&root))
    );
    assert!(!before.capabilities.declaration);
    assert!(send(&mut c, SessionOperation::Declare {}).is_err());
    let after = send(&mut c, SessionOperation::Inspect { event_index: 1 }).unwrap();
    assert!(after.status.concluded);
    assert_eq!(after.event_index, 1);
    assert_eq!(after.context.known_board_actions(), 0);
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    let undone = send(&mut c, SessionOperation::Undo {}).unwrap();
    assert!(!undone.status.concluded);
    assert!(undone.paused);
    assert!(undone.token.selection > ended.token.selection);
    replace(&mut c, root, s::Side::Gote);
    let wrong = send(&mut c, SessionOperation::Declare {}).unwrap();
    assert_eq!(wrong.events.len(), 1);
    assert!(
        wrong
            .status
            .label
            .contains("Sente wins — false declaration")
    );
    assert_eq!(wrong.events[0].board_ply, 0);
}
#[test]
fn exact_analysis_context_retirement_and_same_board_history() {
    let mut c = controller();
    let mut root = imported("4k4/9/5R3/9/9/9/9/9/K8 b - 1");
    for text in ["4c3c", "5a4a", "3c4c", "4a5a"] {
        root.submit(s::GameRequest {
            expected: root.context(),
            actor: root.position().side_to_move(),
            command: s::GameCommand::Play(text.parse().unwrap()),
        })
        .unwrap();
    }
    replace(&mut c, root.clone(), s::Side::Sente);
    let first = analyze(&mut c);
    let record = first.game.clone();
    let fresh = imported(&root.position().to_sfen());
    assert_eq!(fresh.position().to_sfen(), root.position().to_sfen());
    assert_ne!(
        CanonicalRecord::shogi(s::GameRecord::from_game(&fresh)),
        record
    );
    for altered in 0..7 {
        let mut response = incomplete(first.clone());
        match altered {
            0 => response.context.request_id += 1,
            1 => response.context.session_version += 1,
            2 => response.context.process_generation += 1,
            3 => response.context.limits.nodes += 1,
            4 => response.context.purpose = SharedPurpose::EngineTurn {},
            5 => response.context.game = CanonicalRecord::shogi(s::GameRecord::from_game(&fresh)),
            _ => {
                if let SharedPurpose::MoveAnalysis { target } = &mut response.context.purpose {
                    target.selection_id += 1;
                }
            }
        }
        assert!(!c.accept_shared(response));
    }
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    let second = analyze(&mut c);
    assert!(second.request_id > first.request_id);
    assert!(!c.accept_shared(incomplete(first.clone())));
    // Use actual executor below as authoritative telemetry IDs are not presentation constants.
    let mut output = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(&second).unwrap())),
        &mut output,
    )
    .unwrap();
    let result = serde_json::from_slice(&output).unwrap();
    assert!(c.accept_shared(result));
    assert_eq!(
        c.shared.shogi.as_ref().unwrap().game.context(),
        root.context()
    );
    assert!(c.shogi_snapshot().analysis.is_some());
    send(&mut c, SessionOperation::Inspect { event_index: 2 }).unwrap();
    let historic = analyze(&mut c);
    assert_eq!(historic.game.event_count(), 2);
    assert_eq!(
        historic.game,
        CanonicalRecord::shogi(s::GameRecord::from_game(&root.prefix(2).unwrap()))
    );
    assert!(!c.accept_shared(incomplete(second)));
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert!(!c.accept_shared(incomplete(historic)));
    let before = c.shared_token();
    assert!(
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::FullReachability {
                    target: mshogi_engine::ReachabilityTarget::Either,
                    max_positions: 1,
                    max_transitions: 1
                }
            }
        )
        .is_err()
    );
    assert_eq!(before, c.shared_token());
}
#[test]
fn replacement_retires_chess_files_and_pending_admission() {
    let mut c = controller();
    c.queue_open(PathBuf::from("/unused/absent.mshogi"), c.open_token())
        .unwrap();
    let (_, receipt) = c
        .command_with_receipt(DesktopCommand::Play {
            version: c.session.version,
            action: "e2e4".parse().unwrap(),
        })
        .unwrap();
    replace(&mut c, s::Game::new(), s::Side::Sente);
    assert!(receipt.completion().unwrap().is_err());
    for _ in 0..20 {
        c.snapshot();
        c.shared_snapshot();
    }
    assert!(c.shared.shogi.is_some());
    assert_eq!(c.shared_snapshot().context.event_count(), 0);
}
#[test]
fn semantic_undo_replay_does_not_revive_retired_work() {
    let mut c = controller();
    replace(
        &mut c,
        imported("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1"),
        s::Side::Sente,
    );
    send(&mut c, SessionOperation::Declare {}).unwrap();
    let old = analyze(&mut c);
    send(&mut c, SessionOperation::Undo {}).unwrap();
    send(&mut c, SessionOperation::Declare {}).unwrap();
    let fresh = analyze(&mut c);
    assert_eq!(fresh.game, old.game);
    assert_ne!(fresh.session_version, old.session_version);
    assert!(fresh.request_id > old.request_id);
    assert_ne!(fresh.purpose, old.purpose);
    assert!(!c.accept_shared(incomplete(old)));
    let mut bytes = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(&fresh).unwrap())),
        &mut bytes,
    )
    .unwrap();
    assert!(c.accept_shared(serde_json::from_slice(&bytes).unwrap()));
    assert!(c.shogi_snapshot().status.concluded);
}
#[test]
fn command_json_strict_native_payload_and_encoded_capacity() {
    let mut c = controller();
    let command = SharedDesktopCommand {
        expected: c.shared_token(),
        game: GameKind::Chess,
        operation: SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
            human: NativeSide::Shogi(s::Side::Sente),
        },
    };
    let json = serde_json::to_string(&command).unwrap();
    assert!(
        SharedDesktopCommand::from_json(&json).is_ok(),
        "{:?}",
        SharedDesktopCommand::from_json(&json)
    );
    for text in [
        json.replacen(
            "\"game\":\"chess\"",
            "\"game\":\"chess\",\"game\":\"chess\"",
            1,
        ),
        json.replacen(
            "\"envelope_version\":1",
            "\"envelope_version\":1,\"envelope_version\":1",
            1,
        ),
        json.replacen(
            "\"rules_profile\":\"mshogi.shogi.csa36.v1\"",
            "\"rules_profile\":\"mshogi.chess.match.v1\"",
            1,
        ),
        json.replacen(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
            1,
        ),
        json.replacen(
            "\"profile_edition\":",
            "\"unsupported\":0,\"profile_edition\":",
            1,
        ),
    ] {
        // Ensure the mutation matched: otherwise this would be a vacuous parser test.
        assert_ne!(text, json);
        assert!(SharedDesktopCommand::from_json(&text).is_err());
    }
    assert!(
        SharedDesktopCommand::from_json(
            &" ".repeat(mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES + 4097)
        )
        .is_err()
    );
    let old = c.shared_token();
    let mut wrong = command;
    if let SessionOperation::Replace { human, .. } = &mut wrong.operation {
        *human = NativeSide::Chess(Color::White);
    }
    assert!(c.shared_command(wrong).is_err());
    assert_eq!(old, c.shared_token());
}
#[test]
fn observation_capacity_refuses_new_work_before_changing_session() {
    let mut c = controller();
    replace(&mut c, s::Game::new(), s::Side::Sente);
    let request = analyze(&mut c);
    let observation = PresentedObservation {
        response: incomplete(request),
        application_error: None,
    };
    let state = c.shared.shogi.as_mut().unwrap();
    state.retire();
    state.observations = vec![observation; 256];
    let token = c.shared_token();
    let context = c.shogi_snapshot().context;
    assert!(
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves { depth: 1, nodes: 1 }
            }
        )
        .unwrap_err()
        .contains("capacity")
    );
    assert_eq!(c.shared_token(), token);
    assert_eq!(c.shogi_snapshot().context, context);
    assert_eq!(c.shogi_snapshot().observations.len(), 256);
    assert!(c.shared.shogi.as_ref().unwrap().pending.is_none());
}

#[test]
fn close_document_preconditions_advance_for_shogi_events_observations_and_replacement() {
    let mut c = controller();
    let initial = c.snapshot().document.revision;
    replace(&mut c, s::Game::new(), s::Side::Sente);
    let replaced = c.pause_for_close().document.revision;
    assert!(replaced > initial);
    send(&mut c, SessionOperation::Resign {}).unwrap();
    let terminal = c.pause_for_close().document.revision;
    assert!(terminal > replaced);
    let context = analyze(&mut c);
    assert!(c.accept_shared({
        let mut bytes = vec![];
        mshogi_engine::serve_once(
            std::io::Cursor::new(format!("{}\n", serde_json::to_string(&context).unwrap())),
            &mut bytes,
        )
        .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }));
    let observed = c.snapshot().document.revision;
    assert!(observed > terminal);
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::chess(
                AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record(),
            ),
            human: NativeSide::Chess(Color::White),
        },
    )
    .unwrap();
    assert!(c.snapshot().document.revision > observed);
}
#[test]
fn command_diagnostics_are_bounded_without_splitting_utf8() {
    let huge = "界".repeat(5000);
    let command = format!(
        "{{\"expected\":{{\"{huge}\":1}},\"game\":\"chess\",\"operation\":{{\"kind\":\"cancel\"}}}}"
    );
    let error = SharedDesktopCommand::from_json(&command).unwrap_err();
    assert!(error.len() <= mshogi_core::native::MAX_RECORD_DIAGNOSTIC_BYTES);
    assert!(error.len() > 100);
}
#[test]
fn incomplete_failure_and_retry_preserve_native_history_without_fallback() {
    let mut c = controller();
    replace(&mut c, s::Game::new(), s::Side::Gote);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let old = c.shared.shogi.as_ref().unwrap().pending.clone().unwrap();
    let before = c.shogi_snapshot().context;
    assert!(c.accept_shared(incomplete(old.clone())));
    assert_eq!(c.shogi_snapshot().phase, EnginePhase::Incomplete);
    assert_eq!(c.shogi_snapshot().context, before);
    assert!(!c.shogi_snapshot().status.concluded);
    send(&mut c, SessionOperation::Retry {}).unwrap();
    let fresh = c.shared.shogi.as_ref().unwrap().pending.clone().unwrap();
    assert!(fresh.request_id > old.request_id);
    assert!(!c.accept_shared(incomplete(old)));
    assert!(c.accept_shared(SharedResponse {
        context: fresh,
        result: SharedResult::Failure {
            failure: mshogi_engine::EngineFailure {
                kind: mshogi_engine::FailureKind::Search,
                message: "separate unscored process failure".into()
            }
        }
    }));
    assert_eq!(c.shogi_snapshot().phase, EnginePhase::Failed);
    assert_eq!(c.shogi_snapshot().context, before);
    assert!(!c.shogi_snapshot().status.concluded);
}
