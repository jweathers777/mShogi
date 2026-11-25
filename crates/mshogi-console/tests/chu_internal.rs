//! Actual native Chu subprocess and sole desktop controller; no packaged GUI claim.
use mshogi_core::{chu as c, native::*};
use mshogi_desktop_session::{
    DesktopController, SessionOperation, SharedDesktopCommand, SharedSnapshot,
};
use mshogi_engine::*;
use std::{
    thread,
    time::{Duration, Instant},
};
// These tests need one complete native iteration, not production playing depth.
// The standard root completes depth one in 38 nodes; 512 leaves room for the
// tested opening/history prefixes. Normal Resume limits are checked separately.
const CONTRACT_LIMITS: WorkLimits = WorkLimits {
    time_ms: None,
    depth: 1,
    nodes: 512,
};

fn request(id: u64, g: &c::Game, analysis: bool) -> ExecutionRequest {
    let game = CanonicalRecord::chu(c::GameRecord::from_game(g).unwrap());
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: if analysis {
                ExecutionPurpose::MoveAnalysis {
                    target: ExecutionTarget {
                        selection_id: 3,
                        event_index: game.event_count() as u64,
                        board_ply: game.known_board_actions() as u64,
                        session_version: 2,
                        label: "Chu exact prefix".into(),
                        turn: NativeSide::Chu(g.position().side_to_move()),
                    },
                }
            } else {
                ExecutionPurpose::EngineTurn {}
            },
            request_id: id,
            session_version: 2,
            game,
            limits: CONTRACT_LIMITS,
        },
    }
}
fn wait(b: &EngineBridge) -> ExecutionResponse {
    let end = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(r) = b.poll() {
            return r;
        }
        assert!(Instant::now() < end);
        thread::sleep(Duration::from_millis(1));
    }
}
fn send(c: &mut DesktopController, op: SessionOperation) -> SharedSnapshot {
    let s = c.shared_snapshot();
    let (_, r) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: s.token,
            game: s.game,
            operation: op,
        })
        .unwrap();
    let end = Instant::now() + Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if let Some(done) = r.completion() {
            done.unwrap();
            return s;
        }
        assert!(Instant::now() < end);
        thread::sleep(Duration::from_millis(1));
    }
}
fn wait_file(ctl: &mut DesktopController, id: u64) -> SharedSnapshot {
    let end = Instant::now() + Duration::from_secs(20);
    loop {
        let s = ctl.shared_snapshot();
        if s.io.id == id
            && matches!(
                s.io.phase,
                mshogi_desktop_session::IoPhase::Complete | mshogi_desktop_session::IoPhase::Failed
            )
        {
            assert_eq!(s.io.phase, mshogi_desktop_session::IoPhase::Complete);
            return s;
        }
        assert!(Instant::now() < end);
        thread::sleep(Duration::from_millis(1));
    }
}
fn failure(ctx: ExecutionContext) -> ExecutionResponse {
    ExecutionResponse {
        protocol: PROTOCOL_VERSION,
        observation: ExecutionObservation::new(
            ctx,
            ExecutionResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::Search,
                    message: "Constructed stale-result probe".into(),
                },
            },
        ),
    }
}
#[test]
fn actual_child_executes_complete_chu_choices_for_full_known_history_and_purpose() {
    let bridge = EngineBridge::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    let mut g = c::Game::standard();
    for analysis in [false, true] {
        g.submit(c::GameRequest {
            expected: g.context(),
            actor: if analysis {
                c::Side::Sente
            } else {
                c::Side::Gote
            },
            command: if analysis {
                c::GameCommand::DeclineDraw
            } else {
                c::GameCommand::OfferDraw
            },
        })
        .unwrap();
        let req = request(if analysis { 2 } else { 1 }, &g, analysis);
        assert!(req.valid());
        bridge.submit(req.clone()).unwrap();
        let r = wait(&bridge);
        assert!(r.matches(&req));
        assert_eq!(r.observation.context.game, req.context.game);
        let ExecutionResult::Chu { result } = &r.observation.result else {
            panic!("actual native Chu result required: {r:?}")
        };
        result.validate(&g).unwrap();
        let ChuResult::Choice {
            choice: action,
            telemetry,
        } = result
        else {
            panic!("{result:?}")
        };
        assert!(g.legal_actions().contains(action));
        assert!(
            telemetry.completed_depth > 0 && telemetry.completed_depth <= req.context.limits.depth
        );
        assert_eq!(telemetry.perspective, g.position().side_to_move());
        assert!(telemetry.score.is_some());
        assert_eq!(g.snapshot().board_ply, 0);
    }
    bridge.shutdown();
}
#[test]
fn clocked_child_admits_suffixes_and_resets_after_takeback() {
    let bridge = EngineBridge::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    let mut game = c::Game::standard();
    for id in 1..=8 {
        if id == 6 {
            game = c::Game::standard();
        }
        let mut req = request(id, &game, false);
        req.context.limits.time_ms = Some(5000);
        bridge.submit(req.clone()).unwrap();
        let response = wait(&bridge);
        assert!(response.matches(&req));
        let ExecutionResult::Chu { result } = response.observation.result else {
            panic!("Chu result required")
        };
        result.validate(&game).unwrap();
        let ChuResult::Choice {
            choice: action,
            telemetry,
        } = result
        else {
            panic!("complete clocked choice required")
        };
        assert_eq!(telemetry.completed_depth, 1);
        game.submit(c::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: c::GameCommand::Play { action },
        })
        .unwrap();
    }
    bridge.shutdown();
}
#[test]
fn actual_controller_readonly_analysis_save_reopen_and_stale_identity_retirement() {
    let mut ctl = DesktopController::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    let native = CanonicalRecord::chu(c::GameRecord::from_game(&c::Game::standard()).unwrap());
    send(
        &mut ctl,
        SessionOperation::Replace {
            record: native.clone(),
            human: NativeSide::Chu(c::Side::Sente),
        },
    );
    let submitted = ctl.verify_chu_execution(CONTRACT_LIMITS, true).unwrap();
    for case in 0..6 {
        let mut ctx = submitted.clone();
        match case {
            0 => ctx.request_id += 1,
            1 => ctx.session_version += 1,
            2 => ctx.limits.nodes += 1,
            3 => ctx.purpose = ExecutionPurpose::EngineTurn {},
            4 => {
                if let ExecutionPurpose::MoveAnalysis { target } = &mut ctx.purpose {
                    target.selection_id += 1
                }
            }
            _ => {
                let mut same_board = c::Game::standard();
                same_board
                    .submit(c::GameRequest {
                        expected: same_board.context(),
                        actor: c::Side::Gote,
                        command: c::GameCommand::OfferDraw,
                    })
                    .unwrap();
                ctx.game = CanonicalRecord::chu(c::GameRecord::from_game(&same_board).unwrap());
                if let ExecutionPurpose::MoveAnalysis { target } = &mut ctx.purpose {
                    target.event_index = 1;
                }
                assert!(ctx.valid_limits()); // Individually valid, wrong full history.
            }
        }
        assert!(!ctl.accept_shared(failure(ctx)));
    }
    let end = Instant::now() + Duration::from_secs(20);
    let observed = loop {
        let s = ctl.shared_snapshot();
        if s.observations.len() == 1 {
            break s;
        }
        assert!(Instant::now() < end);
        thread::sleep(Duration::from_millis(1));
    };
    assert_eq!(observed.context, native);
    assert!(observed.search_observations[0].score.is_some());
    assert!(
        observed.search_observations[0]
            .completed_depth
            .is_some_and(|depth| depth > 0 && depth <= submitted.limits.depth)
    );
    assert_eq!(observed.observations[0].observation.context, submitted);
    assert_eq!(observed.phase, mshogi_desktop_session::EnginePhase::Paused);
    assert!(!observed.observations[0].historical);
    let path = std::env::temp_dir().join(format!(
        "msh43-real-observation-{}.mshogi",
        std::process::id()
    ));
    let id = ctl.queue_shared_save(path.clone()).unwrap();
    let saved = wait_file(&mut ctl, id);
    let id = ctl
        .queue_shared_open(path.clone(), saved.open_token)
        .unwrap();
    let reopened = wait_file(&mut ctl, id);
    assert!(reopened.token.document > saved.token.document);
    assert!(reopened.token.version > saved.token.version);
    assert!(reopened.paused);
    assert_eq!(reopened.execution_purpose, None);
    // Engine results are not saved (MSH-228).
    assert!(reopened.observations.is_empty());
    assert!(!ctl.accept_shared(failure(submitted)));
    assert_eq!(reopened.context, native);
    std::fs::remove_file(path).unwrap();
    let retired = ctl
        .verify_chu_execution(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 1,
            },
            true,
        )
        .unwrap();
    // Exact history selection retires pending work; returning to an identical
    // diagram does not revive the old request or process authority.
    send(&mut ctl, SessionOperation::Inspect { event_index: 0 });
    send(&mut ctl, SessionOperation::ReturnLive {});
    assert!(!ctl.accept_shared(failure(retired)));
    let retired = ctl
        .verify_chu_execution(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 1,
            },
            false,
        )
        .unwrap();
    let action = c::Game::standard().legal_actions()[0].clone();
    send(
        &mut ctl,
        SessionOperation::Chu {
            actor: c::Side::Sente,
            command: c::GameCommand::Play { action },
        },
    );
    send(&mut ctl, SessionOperation::TakeBack {});
    assert_eq!(ctl.shared_snapshot().context, native);
    assert!(!ctl.accept_shared(failure(retired)));
    let retired = ctl
        .verify_chu_execution(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 1,
            },
            false,
        )
        .unwrap();
    send(&mut ctl, SessionOperation::Cancel {});
    assert!(!ctl.accept_shared(failure(retired)));
    let retired = ctl
        .verify_chu_execution(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 1,
            },
            true,
        )
        .unwrap();
    send(
        &mut ctl,
        SessionOperation::Replace {
            record: CanonicalRecord::chess(
                mshogi_core::chess::AdjudicatedGame::new(
                    mshogi_core::chess::ClaimPolicy::ExplicitOnly,
                )
                .record(),
            ),
            human: NativeSide::Chess(mshogi_core::chess::Color::White),
        },
    );
    assert!(!ctl.accept_shared(failure(retired)));
    assert_eq!(ctl.shared_snapshot().game, GameKind::Chess);
}
fn controller(human: c::Side) -> DesktopController {
    let mut ctl = DesktopController::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    send(
        &mut ctl,
        SessionOperation::Replace {
            record: CanonicalRecord::chu(c::GameRecord::from_game(&c::Game::standard()).unwrap()),
            human: NativeSide::Chu(human),
        },
    );
    ctl
}
fn wait_observation(ctl: &mut DesktopController, count: usize) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let s = ctl.shared_snapshot();
        if s.observations.len() >= count {
            return s;
        }
        assert!(Instant::now() < deadline, "{s:?}");
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn bounded_engine_turns_for_both_player_assignments_admit_observation_then_action() {
    engine_turns_for_both_player_assignments(false);
}

// This test deliberately checks the product's unmodified Resume budget and
// real child deadline. Debug search at 300,000 nodes exceeds that deadline;
// the same assertion body runs with bounded work above in every build.
#[test]
#[cfg_attr(
    debug_assertions,
    ignore = "production Chu Resume budget; run in release"
)]
fn actual_engine_turns_for_both_player_assignments_admit_observation_then_action() {
    engine_turns_for_both_player_assignments(true);
}

fn engine_turns_for_both_player_assignments(production_limits: bool) {
    for human in [c::Side::Sente, c::Side::Gote] {
        let mut ctl = controller(human);
        let before = ctl.shared_snapshot();
        let base = if human == c::Side::Sente {
            let CanonicalRecord::Chu(record) = &before.context else {
                panic!()
            };
            let game = record.to_game().unwrap();
            let action = game.legal_actions()[0].clone();
            send(
                &mut ctl,
                SessionOperation::Chu {
                    actor: human,
                    command: c::GameCommand::Play { action },
                },
            )
        } else {
            before
        };
        let expected_limits = if production_limits {
            send(&mut ctl, SessionOperation::Resume {});
            game::Chu::PLAY_LIMITS
        } else {
            ctl.verify_chu_execution(CONTRACT_LIMITS, false).unwrap();
            CONTRACT_LIMITS
        };
        let after = wait_observation(&mut ctl, 1);
        assert_eq!(
            after.context.known_board_actions(),
            base.context.known_board_actions() + 1
        );
        assert_eq!(after.turn.id, NativeSide::Chu(human));
        let observation = &after.observations[0].observation;
        assert_eq!(observation.context.game, base.context);
        assert_eq!(observation.context.limits, expected_limits);
        assert!(matches!(
            observation.context.purpose,
            ExecutionPurpose::EngineTurn {}
        ));
        assert!(observation.matches(&observation.context));
        let ExecutionResult::Chu {
            result: ChuResult::Choice { choice: action, .. },
        } = &observation.result
        else {
            panic!("{observation:?}")
        };
        let CanonicalRecord::Chu(record) = &after.context else {
            panic!()
        };
        let game = record.to_game().unwrap();
        assert_eq!(
            game.events().last().unwrap().command,
            c::GameCommand::Play {
                action: action.clone()
            }
        );
        assert_eq!(game.conclusion(), None);
        ctl.begin_shutdown();
        let deadline = Instant::now() + Duration::from_secs(5);
        while !ctl.shutdown_acknowledged() {
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(1));
        }
    }
}
#[test]
fn actual_child_incomplete_work_has_no_fallback_score_or_conclusion() {
    let bridge = EngineBridge::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    let game = c::Game::standard();
    let mut req = request(80, &game, true);
    req.context.limits.nodes = 1;
    bridge.submit(req.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&req));
    let ExecutionResult::Chu {
        result: ChuResult::Incomplete { telemetry },
    } = &response.observation.result
    else {
        panic!("{response:?}")
    };
    assert_eq!(telemetry.completed_depth, 0);
    assert_eq!(telemetry.nodes, 1);
    assert_eq!(telemetry.score, None);
    assert!(telemetry.principal_variation.is_empty());
    assert_eq!(game.conclusion(), None);
    bridge.shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !bridge.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}
fn started(context: &ExecutionContext) -> std::path::PathBuf {
    let marker = std::env::temp_dir().join(format!(
        "msh43-started-{}-{}.txt",
        std::process::id(),
        context.request_id
    ));
    let deadline = Instant::now() + Duration::from_secs(5);
    while !marker.is_file() {
        assert!(Instant::now() < deadline, "actual Chu child did not start");
        thread::sleep(Duration::from_millis(1));
    }
    let child: u32 = std::fs::read_to_string(&marker).unwrap().parse().unwrap();
    assert_ne!(child, std::process::id());
    marker
}
#[test]
fn actual_slow_child_cancellation_replacement_cross_game_and_cleanup_are_bounded() {
    let mut ctl =
        DesktopController::new(env!("CARGO_BIN_EXE_mshogi-internal-fixture").into()).unwrap();
    let standard = c::Position::standard();
    let position = c::Position::from_diagram(
        &standard.pieces().collect::<Vec<_>>(),
        c::Side::Sente,
        c::RootMetadata::imported(format!("msh43-controller-{}", std::process::id()), 0).unwrap(),
    )
    .unwrap();
    let native = CanonicalRecord::chu(
        c::GameRecord::from_game(&c::Game::from_root(position).unwrap()).unwrap(),
    );
    send(
        &mut ctl,
        SessionOperation::Replace {
            record: native.clone(),
            human: NativeSide::Chu(c::Side::Sente),
        },
    );
    let slow = ctl
        .verify_chu_execution(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 7,
            },
            true,
        )
        .unwrap();
    let first = started(&slow);
    let at = Instant::now();
    send(&mut ctl, SessionOperation::Cancel {});
    assert!(at.elapsed() < Duration::from_secs(5));
    assert_eq!(ctl.shared_snapshot().context, native);
    assert!(!ctl.accept_shared(failure(slow.clone())));
    std::fs::remove_file(first).unwrap();
    let replacement = ctl
        .verify_chu_execution(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 7,
            },
            true,
        )
        .unwrap();
    assert!(replacement.request_id > slow.request_id);
    let second = started(&replacement);
    send(
        &mut ctl,
        SessionOperation::Replace {
            record: CanonicalRecord::chess(
                mshogi_core::chess::AdjudicatedGame::new(
                    mshogi_core::chess::ClaimPolicy::ExplicitOnly,
                )
                .record(),
            ),
            human: NativeSide::Chess(mshogi_core::chess::Color::White),
        },
    );
    assert!(!ctl.accept_shared(failure(replacement)));
    assert_eq!(ctl.shared_snapshot().game, GameKind::Chess);
    std::fs::remove_file(second).unwrap();
    ctl.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !ctl.shutdown_acknowledged() {
        assert!(
            Instant::now() < deadline,
            "started children were not reaped"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn real_historical_analysis_saves_exact_prefix_and_reopens_stopped_without_future() {
    let mut ctl = controller(c::Side::Sente);
    let initial = ctl.shared_snapshot().context;
    let CanonicalRecord::Chu(record) = &initial else {
        panic!()
    };
    let game = record.to_game().unwrap();
    send(
        &mut ctl,
        SessionOperation::Chu {
            actor: c::Side::Sente,
            command: c::GameCommand::Play {
                action: game.legal_actions()[0].clone(),
            },
        },
    );
    ctl.verify_chu_execution(CONTRACT_LIMITS, false).unwrap();
    let played = wait_observation(&mut ctl, 1);
    assert_eq!(played.context.known_board_actions(), 2);
    send(&mut ctl, SessionOperation::Inspect { event_index: 0 });
    let selected = ctl.shared_snapshot();
    assert!(selected.historical);
    let submitted = ctl.verify_chu_execution(CONTRACT_LIMITS, true).unwrap();
    assert_eq!(submitted.game, initial);
    let ExecutionPurpose::MoveAnalysis { target } = &submitted.purpose else {
        panic!()
    };
    assert_eq!(target.event_index, 0);
    assert_eq!(target.board_ply, 0);
    let analyzed = wait_observation(&mut ctl, 2);
    assert_eq!(analyzed.context, initial);
    assert_eq!(analyzed.chu.as_ref().unwrap().live.board_ply, 2);
    assert_eq!(analyzed.chu.as_ref().unwrap().selected.board_ply, 0);
    assert!(analyzed.historical);
    assert_eq!(
        analyzed.observations.last().unwrap().observation.context,
        submitted
    );
    let path = std::env::temp_dir().join(format!(
        "msh43-historical-analysis-{}.mshogi",
        std::process::id()
    ));
    let id = ctl.queue_shared_save(path.clone()).unwrap();
    let saved = wait_file(&mut ctl, id);
    let id = ctl
        .queue_shared_open(path.clone(), saved.open_token)
        .unwrap();
    let opened = wait_file(&mut ctl, id);
    // The file opens live: the viewed position and results are not saved.
    assert_eq!(opened.context, played.context);
    assert_eq!(opened.chu.as_ref().unwrap().live.board_ply, 2);
    assert!(opened.paused);
    assert!(!opened.historical);
    assert!(opened.observations.is_empty());
    assert_eq!(opened.execution_purpose, None);
    assert!(!ctl.accept_shared(failure(submitted)));
    let live = send(&mut ctl, SessionOperation::ReturnLive {});
    assert!(!live.historical);
    assert_eq!(live.context, played.context);
    std::fs::remove_file(path).unwrap();
    ctl.begin_shutdown();
}
