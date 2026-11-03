//! Actual production internal entry point and sole desktop controller. No Chu engine.
use mshogi_core::{chu as c, native::*};
use mshogi_desktop_session::{
    DesktopController, SessionOperation, SharedDesktopCommand, SharedSnapshot,
};
use mshogi_engine::*;
use std::{
    thread,
    time::{Duration, Instant},
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
            process_generation: id,
            game,
            limits: WorkLimits { depth: 1, nodes: 1 },
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
fn failure(ctx: ExecutionContext) -> ExecutionResponse {
    ExecutionResponse {
        protocol: PROTOCOL_VERSION,
        observation: ExecutionObservation::new(
            ctx,
            ExecutionResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::UnsupportedMode,
                    message: "Constructed stale-result probe".into(),
                },
            },
        ),
    }
}
#[test]
fn actual_child_rejects_chu_execution_unscored_for_full_known_history_and_purpose() {
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
        assert!(execution_capabilities(GameKind::Chu).is_empty());
        bridge.submit(req.clone()).unwrap();
        let r = wait(&bridge);
        assert!(r.matches(&req));
        assert_eq!(r.observation.context.game, req.context.game);
        let ExecutionResult::Failure { failure } = r.observation.result else {
            panic!("invented Chu result")
        };
        assert_eq!(failure.kind, FailureKind::UnsupportedMode);
        assert!(failure.message.contains("no move, score"));
    }
    bridge.shutdown();
}
#[test]
fn actual_controller_unsupported_observation_stale_identity_cancellation_and_cross_game_replacement()
 {
    let mut ctl = DesktopController::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    let native = CanonicalRecord::chu(c::GameRecord::from_game(&c::Game::standard()).unwrap());
    send(
        &mut ctl,
        SessionOperation::Replace {
            record: native.clone(),
            human: NativeSide::Chu(c::Side::Sente),
        },
    );
    let submitted = ctl
        .verify_chu_execution(WorkLimits { depth: 1, nodes: 1 }, true)
        .unwrap();
    for case in 0..7 {
        let mut ctx = submitted.clone();
        match case {
            0 => ctx.request_id += 1,
            1 => ctx.process_generation += 1,
            2 => ctx.session_version += 1,
            3 => ctx.limits.nodes += 1,
            4 => ctx.purpose = ExecutionPurpose::EngineTurn {},
            5 => {
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
    assert_eq!(observed.search_observations[0].score, None);
    assert_eq!(observed.phase, mshogi_desktop_session::EnginePhase::Failed);
    assert!(observed.observations[0].historical);
    let path = std::env::temp_dir().join(format!(
        "msh78-real-observation-{}.mshogi",
        std::process::id()
    ));
    let wait_file = |ctl: &mut DesktopController, id| {
        let end = Instant::now() + Duration::from_secs(20);
        loop {
            let s = ctl.shared_snapshot();
            if s.io.id == id
                && matches!(
                    s.io.phase,
                    mshogi_desktop_session::IoPhase::Complete
                        | mshogi_desktop_session::IoPhase::Failed
                )
            {
                assert_eq!(s.io.phase, mshogi_desktop_session::IoPhase::Complete);
                break s;
            }
            assert!(Instant::now() < end);
            thread::sleep(Duration::from_millis(1));
        }
    };
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
    assert_eq!(reopened.observations.len(), 1);
    assert!(reopened.observations[0].historical);
    assert!(!ctl.accept_shared(failure(submitted)));
    assert_eq!(reopened.context, native);
    std::fs::remove_file(path).unwrap();
    let retired = ctl
        .verify_chu_execution(WorkLimits { depth: 1, nodes: 1 }, true)
        .unwrap();
    // Exact history selection retires pending work; returning to an identical
    // diagram does not revive the old request or process authority.
    send(&mut ctl, SessionOperation::Inspect { event_index: 0 });
    send(&mut ctl, SessionOperation::ReturnLive {});
    assert!(!ctl.accept_shared(failure(retired)));
    let retired = ctl
        .verify_chu_execution(WorkLimits { depth: 1, nodes: 1 }, false)
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
        .verify_chu_execution(WorkLimits { depth: 1, nodes: 1 }, false)
        .unwrap();
    send(&mut ctl, SessionOperation::Cancel {});
    assert!(!ctl.accept_shared(failure(retired)));
    let retired = ctl
        .verify_chu_execution(WorkLimits { depth: 1, nodes: 1 }, true)
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
