//! Native MSH-98 consumer verification; no product UI/packaging claim.
use super::*;

#[path = "shared_xiangqi_handoff_tests.rs"]
mod handoff;

fn controller() -> DesktopController {
    let mut c = DesktopController::new(PathBuf::from("/unused/msh98-engine")).unwrap();
    c.shared.manual_engine = true;
    c
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    let game = if c.shared.xiangqi.is_some() {
        GameKind::Xiangqi
    } else if c.shared.shogi.is_some() {
        GameKind::Shogi
    } else {
        GameKind::Chess
    };
    let (_, receipt) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game,
        operation,
    })?;
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result?;
            return Ok(snapshot);
        }
        assert!(
            Instant::now() < deadline,
            "native consumer admission deadline"
        );
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn replace(c: &mut DesktopController, game: &x::Game, human: x::Side) -> SharedSnapshot {
    send(
        c,
        SessionOperation::Replace {
            record: CanonicalRecord::xiangqi(x::GameRecord::from_game(game).unwrap()),
            human: NativeSide::Xiangqi(human),
        },
    )
    .unwrap()
}
fn pending(c: &DesktopController) -> SharedRequest {
    c.shared.xiangqi.as_ref().unwrap().pending.clone().unwrap()
}
fn analyze(c: &mut DesktopController, nodes: u64) -> SharedRequest {
    send(
        c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes },
        },
    )
    .unwrap();
    pending(c)
}
fn execute(request: &SharedRequest) -> SharedResponse {
    let mut output = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(request).unwrap())),
        &mut output,
    )
    .unwrap();
    let response: SharedResponse = serde_json::from_slice(&output).unwrap();
    assert!(response.matches(request));
    response
}
fn accept(c: &mut DesktopController, response: SharedResponse) -> bool {
    let accepted = c.accept_shared(response);
    if accepted {
        let deadline = Instant::now() + std::time::Duration::from_secs(20);
        while c.shared_snapshot().capacity.status == CapacityPhase::Checking {
            assert!(Instant::now() < deadline, "result archive admission");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    accepted
}
fn native(c: &DesktopController) -> CanonicalRecord {
    CanonicalRecord::xiangqi(
        x::GameRecord::from_game(&c.shared.xiangqi.as_ref().unwrap().game).unwrap(),
    )
}
fn human_play(c: &mut DesktopController, action: &str) -> SharedSnapshot {
    send(
        c,
        SessionOperation::Xiangqi {
            command: x::GameCommand::Play {
                action: action.parse().unwrap(),
            },
        },
    )
    .unwrap()
}
fn submit(game: &mut x::Game, action: &str) {
    game.submit(x::GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: x::GameCommand::Play {
            action: action.parse().unwrap(),
        },
    })
    .unwrap();
}

#[test]
fn native_identity_intersection_palaces_river_and_capability_scope() {
    let mut c = controller();
    let chess = c.shared_snapshot();
    assert!(chess.board.intersections.is_none());
    let old = chess.token;
    let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let initial = replace(&mut c, &game, x::Side::Red);
    assert_eq!(initial.game, GameKind::Xiangqi);
    assert_eq!(initial.profile.id(), x::PROFILE_ID);
    assert_eq!(
        (
            initial.sides[0].label.as_str(),
            initial.sides[1].label.as_str()
        ),
        ("Red", "Black")
    );
    assert!(initial.sides[0].first_mover);
    assert_eq!(initial.pieces.len(), 32);
    assert_eq!(initial.choices.len(), 44, "frozen XQ-B01 opening count");
    assert!(
        initial
            .choices
            .iter()
            .all(|choice| matches!(choice.origin, ChoiceOrigin::Intersection { .. }))
    );
    assert_eq!(initial.board.files.len(), 9);
    assert_eq!(initial.board.ranks.len(), 10);
    let topology = initial.board.intersections.as_ref().unwrap();
    assert_eq!(topology.points.len(), 90);
    assert_eq!(topology.river_between_ranks, (4, 5));
    assert_eq!(topology.palaces.len(), 2);
    assert_eq!(topology.palaces[0].points.len(), 9);
    assert_eq!(
        topology.palaces[0].diagonals,
        vec![vec!["d0", "e1", "f2"], vec!["f0", "e1", "d2"]]
    );
    assert_eq!(
        topology.palaces[1].diagonals,
        vec![vec!["d7", "e8", "f9"], vec!["f7", "e8", "d9"]]
    );
    assert_eq!(
        (
            &initial.orientations[0].files[0],
            &initial.orientations[0].ranks[0]
        ),
        (&"a".into(), &"9".into())
    );
    assert_eq!(
        (
            &initial.orientations[1].files[0],
            &initial.orientations[1].ranks[0]
        ),
        (&"i".into(), &"0".into())
    );
    assert!(initial.hands.is_empty());
    assert!(!initial.capabilities.hands);
    assert!(!initial.capabilities.drops);
    assert!(!initial.capabilities.promotion);
    assert!(!initial.capabilities.declaration);
    assert!(initial.capabilities.save, "MSH-99 native archive admission");
    assert!(!initial.capabilities.chess_full);
    assert_eq!(
        initial.play_limits,
        WorkLimits {
            depth: 1,
            nodes: 128
        }
    );
    assert!(initial.paused);
    assert!(
        initial
            .xiangqi
            .as_ref()
            .unwrap()
            .count_advice
            .iter()
            .all(|claim| !claim.eligible)
    );
    assert!(
        c.shared_command(SharedDesktopCommand {
            expected: old,
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
    // Save capability is now exercised through actual files by MSH-99 tests.
}

#[test]
fn native_claim_audit_and_nonmoving_resignation_preserve_board_ply_and_atomicity() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let initial = native(&c);
    let token = c.shared_token();
    assert!(
        send(
            &mut c,
            SessionOperation::Xiangqi {
                command: x::GameCommand::Play {
                    action: "a0a9".parse().unwrap()
                }
            }
        )
        .is_err()
    );
    assert_eq!(native(&c), initial);
    assert_eq!(c.shared_token(), token);
    assert!(
        send(
            &mut c,
            SessionOperation::Play {
                action: "7g7f".parse().unwrap()
            }
        )
        .is_err()
    );
    assert_eq!(native(&c), initial);
    let claim = send(
        &mut c,
        SessionOperation::Xiangqi {
            command: x::GameCommand::ClaimNaturalCount,
        },
    )
    .unwrap();
    assert_eq!((claim.event_index, claim.board_ply), (2, 0));
    assert_eq!(
        claim
            .events
            .iter()
            .map(|e| e.label.as_str())
            .collect::<Vec<_>>(),
        [
            "Red: natural-count claim submitted",
            "Red: natural-count claim rejected"
        ]
    );
    assert!(!claim.status.concluded);
    assert_eq!(claim.status.score_half_points, None);
    let moved = human_play(&mut c, "a3a4");
    assert_eq!((moved.event_index, moved.board_ply), (3, 1));
    let old = pending(&c);
    assert!(matches!(old.purpose, SharedPurpose::EngineTurn {}));
    // Black has the move. Red may still resign, through native terminal admission.
    let ended = send(&mut c, SessionOperation::Resign {}).unwrap();
    assert_eq!((ended.event_index, ended.board_ply), (5, 1));
    assert_eq!(ended.events[3].label, "Red: resigned");
    assert_eq!(ended.events[4].label, "Black wins · resignation");
    assert!(ended.status.concluded);
    assert_eq!(ended.status.score_half_points, Some([0, 2]));
    assert!(!accept(&mut c, execute(&old)));
    let undone = send(&mut c, SessionOperation::Undo {}).unwrap();
    assert_eq!((undone.event_index, undone.board_ply), (3, 1));
    assert!(!undone.status.concluded);
    assert!(undone.paused);
}

#[test]
fn actual_read_only_analysis_rejects_every_context_mismatch_and_cannot_apply_choice() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    send(&mut c, SessionOperation::Resume {}).unwrap();
    human_play(&mut c, "a3a4");
    let retired_turn = pending(&c);
    let request = analyze(&mut c, 128);
    let before = native(&c);
    assert!(matches!(
        request.purpose,
        SharedPurpose::MoveAnalysis { .. }
    ));
    let response = execute(&request);
    assert!(matches!(
        response.result,
        SharedResult::Xiangqi {
            result: XiangqiResult::Choice { .. }
        }
    ));
    let different_history = x::Game::from_root(
        x::PositionRoot::from_fen(
            &c.shared
                .xiangqi
                .as_ref()
                .unwrap()
                .game
                .position()
                .to_fen()
                .unwrap(),
        )
        .unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    assert_eq!(
        different_history.position(),
        c.shared.xiangqi.as_ref().unwrap().game.position()
    );
    for mismatch in 0..8 {
        let mut wrong = response.clone();
        match mismatch {
            0 => wrong.context.request_id += 1,
            1 => wrong.context.session_version += 1,
            2 => wrong.context.process_generation += 1,
            3 => wrong.context.limits.nodes += 1,
            4 => wrong.context.purpose = SharedPurpose::EngineTurn {},
            5 => {
                wrong.context.game =
                    CanonicalRecord::xiangqi(x::GameRecord::from_game(&different_history).unwrap())
            }
            6 => {
                if let SharedPurpose::MoveAnalysis { target } = &mut wrong.context.purpose {
                    target.selection_id += 1
                }
            }
            _ => {
                if let SharedPurpose::MoveAnalysis { target } = &mut wrong.context.purpose {
                    target.event_index += 1
                }
            }
        }
        assert!(!accept(&mut c, wrong), "mismatch {mismatch}");
        assert_eq!(native(&c), before);
    }
    assert!(!accept(&mut c, execute(&retired_turn)));
    assert!(accept(&mut c, response));
    let snapshot = c.shared_snapshot();
    assert!(snapshot.analysis.is_some());
    assert_eq!(native(&c), before);
    assert_eq!(snapshot.search_observations[0].perspective, "Black");
    assert!(snapshot.search_observations[0].selected);
    let historic = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(historic.historical);
    assert!(historic.analysis.is_none());
    assert!(!historic.search_observations.iter().any(|o| o.selected));
}

#[test]
fn cancellation_replacement_undo_and_replay_never_revive_retired_engine_work() {
    let mut c = controller();
    let root = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    replace(&mut c, &root, x::Side::Red);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    human_play(&mut c, "a3a4");
    let old = pending(&c);
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    assert!(!accept(&mut c, execute(&old)));
    send(&mut c, SessionOperation::Undo {}).unwrap();
    send(&mut c, SessionOperation::Resume {}).unwrap();
    human_play(&mut c, "a3a4");
    let fresh = pending(&c);
    assert_eq!(old.game, fresh.game);
    assert_ne!(old.session_version, fresh.session_version);
    assert!(fresh.request_id > old.request_id);
    assert!(!accept(&mut c, execute(&old)));
    assert!(accept(&mut c, execute(&fresh)));
    let current = c.shared_snapshot();
    assert_eq!(current.board_ply, 2);
    assert_eq!(current.turn.label, "Red");
    let analysis = analyze(&mut c, 128);
    // Cross-family replacement uses the same controller and retires Xiangqi work.
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
            human: NativeSide::Shogi(s::Side::Sente),
        },
    )
    .unwrap();
    assert!(!accept(&mut c, execute(&analysis)));
    assert_eq!(c.shared_snapshot().game, GameKind::Shogi);
    replace(&mut c, &root, x::Side::Red);
    assert!(!accept(&mut c, execute(&analysis)));
    assert_eq!(c.shared_snapshot().board_ply, 0);
}

#[test]
fn all_event_prefixes_remain_read_only_with_no_future_warning_or_result_inference() {
    let mut game = x::Game::from_root(
        x::PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    for action in ["e9f9", "f8e8", "f9e9", "e8f8"].into_iter().cycle().take(8) {
        submit(&mut game, action);
    }
    assert_eq!(game.events().len(), 9);
    let mut c = controller();
    replace(&mut c, &game, x::Side::Red);
    let warning = send(&mut c, SessionOperation::Inspect { event_index: 9 }).unwrap();
    assert_eq!(warning.events[0].label, "Red: e9–f9 · check");
    assert_eq!(
        warning.events[8].label,
        "Red: repetition warning · perpetual check"
    );
    let canonical = native(&c);
    assert_eq!(warning.xiangqi.as_ref().unwrap().warnings.len(), 1);
    let partial = send(&mut c, SessionOperation::Inspect { event_index: 8 }).unwrap();
    assert_eq!(partial.board_ply, 8);
    assert!(partial.historical);
    assert!(partial.xiangqi.as_ref().unwrap().warnings.is_empty());
    assert!(!partial.status.concluded);
    assert!(!partial.capabilities.move_analysis);
    let provenance = partial.xiangqi.as_ref().unwrap();
    assert_eq!(
        provenance.canonical_context_scope,
        "live_with_immutable_selection"
    );
    assert!(provenance.selected_canonical.is_none());
    assert_eq!(provenance.selected_events.len(), 8);
    assert_eq!(
        native(&c),
        canonical,
        "display labels must not rewrite the live ledger"
    );
    assert_eq!(provenance.selected_events, game.events()[..8]);
    assert!(
        provenance.count_advice.is_empty(),
        "partial command cannot imply canonical claim eligibility"
    );

    let token = partial.token;
    assert!(
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: 1,
                    nodes: 128
                }
            }
        )
        .is_err()
    );
    assert_eq!(c.shared_token(), token);
    let inspected = c
        .shared_inspection(MovementInspectionQuery {
            expected: token,
            origin: "e9".into(),
        })
        .unwrap();
    assert!(inspected.read_only);
    assert_eq!(inspected.side.label, "Red");
    assert!(inspected.choices.iter().any(|a| a.action == "e9f9"));
    let root = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(root.xiangqi.as_ref().unwrap().warnings.is_empty());
    assert_eq!(root.board_ply, 0);
    assert!(
        c.shared_inspection(MovementInspectionQuery {
            expected: token,
            origin: "e9".into()
        })
        .is_err()
    );
    let request = analyze(&mut c, 128);
    assert_eq!(request.game.event_count(), 0);
    assert_eq!(
        request.game,
        CanonicalRecord::xiangqi(
            x::GameRecord::from_game(&game.committed_prefix(0).unwrap()).unwrap()
        )
    );
    let response = execute(&request);
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert!(!accept(&mut c, response));
    assert_eq!(c.shared_snapshot().event_index, 9);
}

#[test]
fn native_opponent_inspection_is_immutable_nonactionable_and_selection_bound() {
    let game = x::Game::from_root(
        x::PositionRoot::from_fen("4k4/4r4/9/9/9/9/9/9/9/4K4 w - - 0 1").unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    let mut c = controller();
    let snapshot = replace(&mut c, &game, x::Side::Red);
    let before = native(&c);
    let inspection = c
        .shared_inspection(MovementInspectionQuery {
            expected: snapshot.token,
            origin: "e8".into(),
        })
        .unwrap();
    assert!(inspection.read_only);
    assert_eq!(inspection.side.label, "Black");
    assert_eq!(inspection.choices.len(), 7);
    assert!(
        inspection
            .choices
            .iter()
            .all(|a| a.action.starts_with("e8e"))
    );
    assert_eq!(native(&c), before);
    assert_eq!(c.shared_token(), snapshot.token);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    assert!(
        send(
            &mut c,
            SessionOperation::Xiangqi {
                command: x::GameCommand::Play {
                    action: "e8e7".parse().unwrap()
                }
            }
        )
        .is_err()
    );
    assert_eq!(native(&c), before);
    assert!(
        c.shared_inspection(MovementInspectionQuery {
            expected: snapshot.token,
            origin: "e8".into()
        })
        .is_err()
    );
}

#[test]
fn incomplete_failure_retry_and_limits_are_not_chess_outcomes() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Black,
    );
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let turn = pending(&c);
    let before = native(&c);
    let low = analyze(&mut c, 1);
    let result = execute(&low);
    assert!(matches!(
        result.result,
        SharedResult::Xiangqi {
            result: XiangqiResult::Incomplete { .. }
        }
    ));
    assert!(accept(&mut c, result));
    let incomplete = c.shared_snapshot();
    assert_eq!(incomplete.phase, EnginePhase::Incomplete);
    assert_eq!(incomplete.status.score_half_points, None);
    assert_eq!(native(&c), before);
    assert!(!accept(&mut c, execute(&turn)));
    send(&mut c, SessionOperation::Retry {}).unwrap();
    let retry = pending(&c);
    assert!(retry.request_id > low.request_id);
    assert!(matches!(retry.purpose, SharedPurpose::EngineTurn {}));
    assert!(accept(
        &mut c,
        SharedResponse {
            context: retry,
            result: SharedResult::Failure {
                failure: mshogi_engine::EngineFailure {
                    kind: mshogi_engine::FailureKind::Start,
                    message: "test-instrumented start failure".into()
                }
            }
        }
    ));
    let failed = c.shared_snapshot();
    assert_eq!(failed.phase, EnginePhase::Failed);
    assert!(!failed.status.concluded);
    assert_eq!(failed.status.score_half_points, None);
    for limits in [(0, 128), (4, 128), (1, 0), (1, 513)] {
        let token = c.shared_token();
        assert!(
            send(
                &mut c,
                SessionOperation::Analyze {
                    operation: AnalysisOperation::Moves {
                        depth: limits.0,
                        nodes: limits.1
                    }
                }
            )
            .is_err()
        );
        assert_eq!(c.shared_token(), token);
        assert_eq!(native(&c), before);
    }
}

#[test]
fn typed_native_command_boundary_rejects_unknown_duplicate_and_conflicting_fields() {
    let c = controller();
    let base = serde_json::to_string(&SharedDesktopCommand {
        expected: c.shared_token(),
        game: GameKind::Xiangqi,
        operation: SessionOperation::Xiangqi {
            command: x::GameCommand::Play {
                action: "a3a4".parse().unwrap(),
            },
        },
    })
    .unwrap();
    assert!(SharedDesktopCommand::from_json(&base).is_ok());
    for malformed in [
        base.replace("\"kind\":\"play\"", "\"kind\":\"play\",\"kind\":\"resign\""),
        base.replace("\"action\":", "\"unexpected\":true,\"action\":"),
        base.replace("\"xiangqi\"", "\"unknown_game\""),
        base.replace("\"a3a4\"", "\"a3a4junk\""),
    ] {
        assert!(
            SharedDesktopCommand::from_json(&malformed).is_err(),
            "accepted malformed command {malformed}"
        );
    }
}

#[test]
fn retained_observation_count_and_byte_limits_reject_new_work_before_mutation() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    // Actual bounded entry-point results, retained without dropping any earlier
    // accepted context. This exercises the 256 -> 257 admission boundary.
    for expected in 1..=MAX_ARCHIVE_OBSERVATIONS {
        let request = analyze(&mut c, 1);
        assert!(accept(&mut c, execute(&request)));
        assert_eq!(
            c.shared.xiangqi.as_ref().unwrap().observations.len(),
            expected
        );
    }
    let before = c.shared_token();
    let record = native(&c);
    let observations = c.shared.xiangqi.as_ref().unwrap().observations.clone();
    let error = send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap_err();
    assert!(error.contains("capacity"));
    assert_eq!(c.shared_token(), before);
    assert_eq!(native(&c), record);
    assert_eq!(
        c.shared.xiangqi.as_ref().unwrap().observations.len(),
        MAX_ARCHIVE_OBSERVATIONS
    );
    assert_eq!(
        serde_json::to_vec(&c.shared.xiangqi.as_ref().unwrap().observations).unwrap(),
        serde_json::to_vec(&observations).unwrap()
    );

    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    // Separate instrumentation: deterministic byte reservation at the native
    // ceiling, independent of count and without allocating giant diagnostics.
    c.shared.xiangqi.as_mut().unwrap().observation_bytes =
        MAX_OBSERVATION_BYTES - mshogi_engine::MAX_RESPONSE_BYTES;
    let request = analyze(&mut c, 1);
    assert!(accept(&mut c, execute(&request)));
    let before = c.shared_token();
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
    assert_eq!(c.shared_token(), before);
    assert_eq!(c.shared.xiangqi.as_ref().unwrap().observations.len(), 1);
}

#[test]
fn chess_full_and_historical_mutation_are_explicitly_unsupported_without_retiring_work() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    let request = analyze(&mut c, 128);
    let token = c.shared_token();
    assert!(
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::FullReachability {
                    target: mshogi_engine::ReachabilityTarget::Either,
                    max_positions: 100,
                    max_transitions: 1000
                }
            }
        )
        .unwrap_err()
        .contains("not Xiangqi")
    );
    assert_eq!(c.shared_token(), token);
    assert_eq!(pending(&c), request);
    send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    let before = c.shared_token();
    assert!(send(&mut c, SessionOperation::Resume {}).is_err());
    assert!(
        send(
            &mut c,
            SessionOperation::Xiangqi {
                command: x::GameCommand::ClaimNaturalCount
            }
        )
        .is_err()
    );
    assert_eq!(c.shared_token(), before);
    assert!(!accept(&mut c, execute(&request)));
}

#[test]
fn cancel_retires_pending_cross_game_admission_and_its_receipt_immediately() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    let before = native(&c);
    let candidate =
        ShogiSession::restored(&s::GameRecord::from_game(&s::Game::new()), s::Side::Sente).unwrap();
    let receipt = c
        .queue_native_candidate(candidate, None, None, true)
        .unwrap();
    assert_eq!(receipt.completion(), None);
    assert!(send(&mut c, SessionOperation::Resume {}).is_err());
    let cancelled = send(&mut c, SessionOperation::Cancel {}).unwrap();
    assert_eq!(cancelled.game, GameKind::Xiangqi);
    assert_eq!(native(&c), before);
    assert_eq!(cancelled.phase, EnginePhase::Cancelled);
    assert_eq!(cancelled.capacity.status, CapacityPhase::Available);
    assert!(receipt.completion().unwrap().is_err());
    assert!(c.shared.admission.is_none());
    assert!(c.admission_token.is_none());
    for _ in 0..3 {
        assert_eq!(c.shared_snapshot().game, GameKind::Xiangqi);
    }
}

#[test]
fn paused_terminal_capabilities_match_admission_without_erasing_claim_advice() {
    // Reuse the independently specified 100/102 WXF precedence ledger.
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/history-precedence.json"
    ))
    .unwrap();
    let mut game = x::Game::from_root(
        x::PositionRoot::from_fen(fixture["root_fen"].as_str().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    for action in fixture["moves"].as_array().unwrap().iter().take(100) {
        game.submit(x::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: x::GameCommand::Play {
                action: action.as_str().unwrap().parse().unwrap(),
            },
        })
        .unwrap();
    }
    assert!(game.natural_count_eligibility(x::Side::Red).eligible);
    let mut c = controller();
    let paused = replace(&mut c, &game, x::Side::Red);
    assert!(
        paused
            .xiangqi
            .as_ref()
            .unwrap()
            .count_advice
            .iter()
            .all(|e| e.eligible)
    );
    assert!(!paused.capabilities.claims);
    assert!(!paused.capabilities.resign);
    let before = native(&c);
    assert!(send(&mut c, SessionOperation::Resign {}).is_err());
    assert!(
        send(
            &mut c,
            SessionOperation::Xiangqi {
                command: x::GameCommand::ClaimNaturalCount
            }
        )
        .is_err()
    );
    assert_eq!(native(&c), before);
    let live = send(&mut c, SessionOperation::Resume {}).unwrap();
    assert!(live.capabilities.claims);
    assert!(live.capabilities.resign);
    let stopped = send(&mut c, SessionOperation::Cancel {}).unwrap();
    assert!(!stopped.capabilities.claims);
    assert!(!stopped.capabilities.resign);
    assert!(
        stopped
            .xiangqi
            .as_ref()
            .unwrap()
            .count_advice
            .iter()
            .all(|e| e.eligible)
    );
    assert_eq!(native(&c), before);
}
