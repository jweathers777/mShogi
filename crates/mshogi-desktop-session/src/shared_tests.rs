use super::*;
fn controller() -> DesktopController {
    let mut c = DesktopController::new(PathBuf::from("/unused/msh72-engine")).unwrap();
    c.shared.manual_engine = true;
    c
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
    let (_, receipt) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game,
        operation: op,
    })?;
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result?;
            return Ok(snapshot);
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
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
fn analyze(c: &mut DesktopController) -> ExecutionContext {
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
fn incomplete(context: ExecutionContext) -> ExecutionObservation {
    ExecutionObservation {
        schema_version: OBSERVATION_SCHEMA_VERSION,
        result: ExecutionResult::Shogi {
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
    assert_eq!(chess.play_limits, chess.chess.as_ref().unwrap().limits);
    assert_eq!(chess.board_ply, 0);
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
    assert!(shogi.capabilities.save);
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
fn presented_budget_matches_submitted_play_and_board_ply_ignores_terminal_events() {
    let mut c = controller();
    let initial = replace(&mut c, s::Game::new(), s::Side::Gote);
    assert_eq!(
        initial.play_limits,
        WorkLimits {
            depth: 2,
            nodes: 5000
        }
    );
    assert_eq!(initial.board_ply, 0);
    let playing = send(&mut c, SessionOperation::Resume {}).unwrap();
    assert_eq!(
        c.shared
            .shogi
            .as_ref()
            .unwrap()
            .pending
            .as_ref()
            .unwrap()
            .limits,
        playing.play_limits
    );
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    let ended = send(&mut c, SessionOperation::Resign {}).unwrap();
    assert_eq!(ended.event_index, 1);
    assert_eq!(ended.board_ply, 0);
    assert_eq!(
        serde_json::to_value(&ended).unwrap()["play_limits"]["nodes"],
        5000
    );
    let prefix = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(prefix.historical);
    assert_eq!(prefix.board_ply, 0);
    assert!(!prefix.status.concluded);

    replace(&mut c, s::Game::new(), s::Side::Sente);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let moved = send(
        &mut c,
        SessionOperation::Play {
            action: "7g7f".parse().unwrap(),
        },
    )
    .unwrap();
    assert_eq!((moved.event_index, moved.board_ply), (1, 1));
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    let ended = send(&mut c, SessionOperation::Resign {}).unwrap();
    assert_eq!((ended.event_index, ended.board_ply), (2, 1));
    let prefix = send(&mut c, SessionOperation::Inspect { event_index: 1 }).unwrap();
    assert_eq!((prefix.event_index, prefix.board_ply), (1, 1));
    assert!(!prefix.status.concluded);
    let root = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert_eq!((root.event_index, root.board_ply), (0, 0));
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
    assert_eq!(before.version, c.shared_token().version);
    assert_eq!(before.selection, c.shared_token().selection);
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
            .contains("Gote loses by false entering-king declaration")
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
            4 => response.context.purpose = ExecutionPurpose::EngineTurn {},
            5 => response.context.game = CanonicalRecord::shogi(s::GameRecord::from_game(&fresh)),
            _ => {
                if let ExecutionPurpose::MoveAnalysis { target } = &mut response.context.purpose {
                    target.selection_id += 1;
                }
            }
        }
        assert!(!c.accept_observation(response));
    }
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    let second = analyze(&mut c);
    assert!(second.request_id > first.request_id);
    assert!(!c.accept_observation(incomplete(first.clone())));
    // Use actual executor below as authoritative telemetry IDs are not presentation constants.
    let mut output = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!(
            "{}\n",
            serde_json::to_string(&execution_request(second.clone())).unwrap()
        )),
        &mut output,
    )
    .unwrap();
    let result = decode_execution_observation(&output);
    assert!(c.accept_observation(result));
    settle(&mut c);
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
    assert!(!c.accept_observation(incomplete(second)));
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert!(!c.accept_observation(incomplete(historic)));
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
    assert_eq!(before.version, c.shared_token().version);
    assert_eq!(before.selection, c.shared_token().selection);
}
#[test]
fn replacement_retires_chess_files_and_pending_admission() {
    let mut c = controller();
    c.queue_open(PathBuf::from("/unused/absent.mshogi"), c.open_token())
        .unwrap();
    // Stage a real immutable candidate without polling it. The public command
    // also returns a snapshot and may already accept a fast worker completion;
    // that legitimately completed command is not pending work to retire.
    let mut candidate = c.session.clone();
    candidate
        .command_unchecked(
            DesktopCommand::Play {
                version: c.session.version,
                action: "e2e4".parse().unwrap(),
            },
            0,
        )
        .unwrap();
    c.queue_admission(candidate).unwrap();
    let receipt = c.admission_receipt.clone().unwrap();
    assert!(receipt.completion().is_none());
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
    assert!(!c.accept_observation(incomplete(old)));
    let mut bytes = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!(
            "{}\n",
            serde_json::to_string(&execution_request(fresh.clone())).unwrap()
        )),
        &mut bytes,
    )
    .unwrap();
    assert!(c.accept_observation(decode_execution_observation(&bytes)));
    settle(&mut c);
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
        observation: incomplete(request),
        application_error: None,
        historical: false,
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
    assert_eq!(c.shared_token().version, token.version);
    assert_eq!(c.shared_token().selection, token.selection);
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
    assert!(c.accept_observation({
        let mut bytes = vec![];
        mshogi_engine::serve_once(
            std::io::Cursor::new(format!(
                "{}\n",
                serde_json::to_string(&execution_request(context.clone())).unwrap()
            )),
            &mut bytes,
        )
        .unwrap();
        decode_execution_observation(&bytes)
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
    assert!(c.accept_observation(incomplete(old.clone())));
    settle(&mut c);
    assert_eq!(c.shogi_snapshot().phase, EnginePhase::Incomplete);
    assert_eq!(c.shogi_snapshot().context, before);
    assert!(!c.shogi_snapshot().status.concluded);
    send(&mut c, SessionOperation::Retry {}).unwrap();
    let fresh = c.shared.shogi.as_ref().unwrap().pending.clone().unwrap();
    assert!(fresh.request_id > old.request_id);
    assert!(!c.accept_observation(incomplete(old)));
    assert!(c.accept_observation(ExecutionObservation {
        schema_version: OBSERVATION_SCHEMA_VERSION,
        context: fresh,
        result: ExecutionResult::Failure {
            failure: mshogi_engine::EngineFailure {
                kind: mshogi_engine::FailureKind::Search,
                message: "separate unscored process failure".into()
            }
        }
    }));
    settle(&mut c);
    assert_eq!(c.shogi_snapshot().phase, EnginePhase::Failed);
    assert_eq!(c.shogi_snapshot().context, before);
    assert!(!c.shogi_snapshot().status.concluded);
}

fn settle(c: &mut DesktopController) {
    let end = Instant::now() + std::time::Duration::from_secs(20);
    while c.shared_snapshot().capacity.status == CapacityPhase::Checking {
        assert!(Instant::now() < end);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn new_product_game(
    c: &mut DesktopController,
    game: GameKind,
    human: NativeSide,
) -> SharedSnapshot {
    let (_, receipt) = c
        .shared_new_game(c.shared_open_token(), game, human)
        .unwrap();
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result.unwrap();
            return snapshot;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
#[test]
fn product_new_games_are_native_qualified_oriented_and_live_with_fresh_identities() {
    let mut c = controller();
    let mut previous = c.shared_snapshot().token;
    for human in [
        NativeSide::Shogi(s::Side::Sente),
        NativeSide::Shogi(s::Side::Gote),
        NativeSide::Chess(Color::Black),
        NativeSide::Chess(Color::White),
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Red),
        NativeSide::Xiangqi(mshogi_core::xiangqi::Side::Black),
    ] {
        let current = new_product_game(&mut c, human.game(), human);
        assert_eq!(current.game, human.game());
        assert_eq!(current.human.id, human);
        assert!(current.token.document > previous.document);
        assert!(current.token.version > previous.version);
        assert!(!current.paused);
        assert!(!current.historical);
        assert!(current.document.path.is_none());
        assert!(current.document.dirty);
        assert_eq!(current.open_token, c.shared_open_token());
        match human {
            NativeSide::Xiangqi(side) => {
                use mshogi_core::xiangqi as x;
                assert_eq!(current.profile.id(), x::PROFILE_ID);
                assert_eq!(current.pieces.len(), 32);
                assert_eq!(current.choices.len(), 44);
                assert_eq!(current.board.intersections.unwrap().points.len(), 90);
                assert!(current.hands.is_empty());
                assert!(!current.capabilities.promotion && !current.capabilities.declaration);
                assert_eq!(current.turn.id, NativeSide::Xiangqi(x::Side::Red));
                let state = c.shared.xiangqi.as_ref().unwrap();
                assert_eq!(state.settings.orientation, side);
                assert_eq!(current.capabilities.board_action, side == x::Side::Red);
                assert_eq!(state.pending.is_some(), side == x::Side::Black);
            }
            NativeSide::Shogi(side) => {
                assert_eq!(current.pieces.len(), 40);
                assert_eq!(current.choices.len(), 30);
                assert!(!current.capabilities.declaration);
                assert_eq!(current.turn.id, NativeSide::Shogi(s::Side::Sente));
                assert_eq!(c.shared.shogi.as_ref().unwrap().settings.orientation, side);
                assert_eq!(current.capabilities.board_action, side == s::Side::Sente);
                assert_eq!(
                    c.shared.shogi.as_ref().unwrap().pending.is_some(),
                    side == s::Side::Gote
                );
            }
            NativeSide::Chess(side) => {
                assert_eq!(current.chess.unwrap().document.settings.orientation, side)
            }
        }
        previous = current.token;
    }
}
#[test]
fn product_new_rejects_stale_or_conflicting_choices_before_retiring_live_work() {
    let mut c = controller();
    let stale = c.shared_open_token();
    let before = new_product_game(&mut c, GameKind::Shogi, NativeSide::Shogi(s::Side::Gote));
    let pending = c.shared.shogi.as_ref().unwrap().pending.clone();
    assert!(
        c.shared_new_game(stale, GameKind::Chess, NativeSide::Chess(Color::White))
            .is_err()
    );
    assert!(
        c.shared_new_game(
            c.shared_open_token(),
            GameKind::Chess,
            NativeSide::Shogi(s::Side::Sente)
        )
        .is_err()
    );
    assert_eq!(c.shared_token(), before.token);
    assert_eq!(c.shared.shogi.as_ref().unwrap().pending, pending);
    assert_eq!(c.shared.shogi.as_ref().unwrap().game.events().len(), 0);
}
#[test]
fn product_declaration_capability_is_advisory_eligibility_but_native_terminal_semantics_remain() {
    let mut c = controller();
    let initial = new_product_game(&mut c, GameKind::Shogi, NativeSide::Shogi(s::Side::Sente));
    assert!(!initial.declaration_advice[0].eligible);
    assert!(!initial.capabilities.declaration);
    let eligibility = replace(
        &mut c,
        imported("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1"),
        s::Side::Sente,
    );
    assert!(eligibility.capabilities.declaration);
    let declared = send(&mut c, SessionOperation::Declare {}).unwrap();
    assert_eq!(
        declared.status.label,
        "Sente wins by entering-king declaration"
    );
    assert!(!declared.capabilities.declaration);
    assert_eq!(declared.events[0].board_ply, 0);
    new_product_game(&mut c, GameKind::Shogi, NativeSide::Shogi(s::Side::Gote));
    let resigned = send(&mut c, SessionOperation::Resign {}).unwrap();
    assert_eq!(resigned.status.label, "Gote resigned · Sente wins");
    assert_eq!(resigned.events[0].board_ply, 0);
    assert_eq!(resigned.turn.id, NativeSide::Shogi(s::Side::Sente));
}
#[cfg(feature = "desktop-verification")]
#[test]
fn product_verification_fixtures_use_real_native_context_and_choices() {
    for fixture in [
        "shogi-standard",
        "shogi-gote",
        "shogi-hands",
        "shogi-promotion",
        "shogi-forced",
        "shogi-declaration",
        "shogi-concluded",
    ] {
        let mut c =
            DesktopController::verification("/unused/verification-engine".into(), fixture).unwrap();
        c.shared.manual_engine = true;
        let view = c.shared_snapshot();
        assert_eq!(view.game, GameKind::Shogi);
        assert!(view.context.game_kind() == GameKind::Shogi);
        match fixture {
            "shogi-hands" => {
                assert_eq!(view.events.len(), 2);
                assert_eq!(view.hands.iter().filter(|p| p.count > 0).count(), 2);
                assert!(
                    view.choices
                        .iter()
                        .any(|c| matches!(c.origin, ChoiceOrigin::Hand { .. }))
                );
            }
            "shogi-promotion" => assert_eq!(
                view.choices
                    .iter()
                    .filter(|c| c.from == "5d" && c.steps == ["5c"])
                    .count(),
                2
            ),
            "shogi-forced" => assert!(view.choices.iter().any(|c| c.action == "5b5a+")),
            "shogi-declaration" => assert!(view.capabilities.declaration),
            "shogi-concluded" => assert_eq!(view.status.label, "Sente resigned · Gote wins"),
            _ => assert_eq!(view.pieces.len(), 40),
        }
    }
}

#[test]
fn native_search_presentation_preserves_target_purpose_and_real_terminal_distance() {
    let mut c = controller();
    replace(
        &mut c,
        imported("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1"),
        s::Side::Sente,
    );
    let context = analyze(&mut c);
    let mut bytes = serde_json::to_vec(&execution_request(context.clone())).unwrap();
    bytes.push(b'\n');
    let mut output = vec![];
    mshogi_engine::serve_once(std::io::Cursor::new(bytes), &mut output).unwrap();
    assert!(c.accept_observation(decode_execution_observation(&output)));
    settle(&mut c);
    let view = c.shared_snapshot();
    assert_eq!(view.event_index, 0);
    assert!(
        !view.status.concluded,
        "read-only declaration analysis cannot declare"
    );
    let search = &view.search_observations[0];
    assert_eq!(search.purpose, "move_analysis");
    assert_eq!(search.event_index, 0);
    assert_eq!(search.board_ply, 0);
    assert_eq!(search.perspective, "Sente");
    assert_eq!(search.engine, "mShogi");
    assert_eq!(
        search.score.as_deref(),
        Some("Win in 1 command edge (including declarations)")
    );
    assert_eq!(search.completed_depth, Some(1));
    assert_eq!(search.nodes, Some(2));
    assert_eq!(search.variation, ["Sente declares entering king"]);
    assert!(search.selected);
    assert!(!search.historical);
    assert_eq!(search.status, "Complete");
    let inspected = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(!inspected.search_observations[0].selected);
    assert!(inspected.analysis.is_none());
}

#[test]
fn xiangqi_product_creation_retires_prior_work_and_keeps_archive_admission() {
    use mshogi_core::xiangqi as x;
    for shogi in [false, true] {
        let mut c = controller();
        if shogi {
            replace(&mut c, s::Game::new(), s::Side::Gote);
        }
        c.shared_snapshot();
        let token = c.shared_token();
        let open = c.shared_open_token();
        let pending = c.shared.shogi.as_ref().and_then(|g| g.pending.clone());
        let after = new_product_game(&mut c, GameKind::Xiangqi, NativeSide::Xiangqi(x::Side::Red));
        assert_eq!(after.game, GameKind::Xiangqi);
        assert_ne!(after.token, token);
        assert!(!after.paused);
        assert!(after.capabilities.board_action);
        assert!(c.shared.shogi.is_none());
        if let Some(old) = pending {
            assert_ne!(old.game, after.context);
        }
        assert!(
            c.shared_new_game(open, GameKind::Xiangqi, NativeSide::Xiangqi(x::Side::Black))
                .is_err()
        );
        assert!(after.capabilities.save, "MSH-99 native archive support");
    }
}

#[test]
fn current_wire_frame_is_checked_before_a_durable_native_observation_is_admitted() {
    let mut c = controller();
    replace(&mut c, s::Game::new(), s::Side::Sente);
    let context = analyze(&mut c);
    let observation = incomplete(context.clone());
    let before = c.shared_snapshot().context;
    assert!(!c.accept_shared(ExecutionResponse {
        protocol: mshogi_engine::PROTOCOL_VERSION - 1,
        observation: observation.clone(),
    }));
    assert_eq!(c.shared_snapshot().context, before);
    assert!(c.shared.shogi.as_ref().unwrap().observations.is_empty());
    assert_eq!(c.shared.shogi.as_ref().unwrap().pending, Some(context));
    assert!(c.accept_shared(ExecutionResponse {
        protocol: mshogi_engine::PROTOCOL_VERSION,
        observation,
    }));
    settle(&mut c);
    let saved = c.shared.shogi.as_ref().unwrap();
    assert_eq!(saved.observations.len(), 1);
    assert_eq!(
        saved.observations[0].observation.schema_version,
        OBSERVATION_SCHEMA_VERSION
    );
    assert_eq!(saved.record(), before);
}
