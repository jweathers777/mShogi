//! MSH-125 native lifecycle expectations. No UI rules, fake results or board equality authority.
use super::*;
use mshogi_core::xiangqi as x;
use std::sync::atomic::{AtomicU64, Ordering};
const GAMES: [GameKind; 3] = [GameKind::Chess, GameKind::Shogi, GameKind::Xiangqi];
fn peek(c: &DesktopController) -> SharedSnapshot {
    match &c.session {
        ActiveDocument::Chess(_) => c.chess_snapshot(c.session.snapshot()),
        ActiveDocument::Shogi { .. } => c.shogi_snapshot(),
        ActiveDocument::Xiangqi { .. } => c.xiangqi_snapshot(),
    }
}
fn pump(c: &mut DesktopController) {
    c.poll_native_admission();
    c.poll_files();
}
fn settle(c: &mut DesktopController, receipt: &AdmissionReceipt) -> Result<(), String> {
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        pump(c);
        if let Some(result) = receipt.completion() {
            result?;
            if c.io_state.phase != IoPhase::Viewing {
                return Ok(());
            }
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    let (_, receipt) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game: peek(c).game,
        operation,
    })?;
    settle(c, &receipt)?;
    Ok(peek(c))
}
fn running(game: GameKind, engine_turn: bool) -> DesktopController {
    let mut c = DesktopController::new("/unused/msh125-manual-engine".into()).unwrap();
    c.shared.manual_engine = true;
    let human = match game {
        GameKind::Chess => NativeSide::Chess(if engine_turn {
            Color::Black
        } else {
            Color::White
        }),
        GameKind::Shogi => NativeSide::Shogi(if engine_turn {
            s::Side::Gote
        } else {
            s::Side::Sente
        }),
        GameKind::Xiangqi => NativeSide::Xiangqi(if engine_turn {
            x::Side::Black
        } else {
            x::Side::Red
        }),
    };
    let (_, receipt) = c
        .shared_new_game(c.shared_open_token(), game, human)
        .unwrap();
    settle(&mut c, &receipt).unwrap();
    assert!(!peek(&c).paused);
    c
}
fn pending(c: &mut DesktopController) -> ExecutionContext {
    if matches!(c.session, ActiveDocument::Chess(_)) {
        if let Some(p) = c.session.chess().analysis_pending.clone().or(c
            .session
            .chess()
            .pending
            .clone())
        {
            return p;
        }
        return c.session.chess_mut().engine_request().unwrap().context;
    }
    c.shared_snapshot(); // Manual mode allocates native work without a process submission.
    if let Some(s) = c.session.shogi() {
        s.pending.clone().unwrap()
    } else {
        c.session.xiangqi().unwrap().pending.clone().unwrap()
    }
}
fn retired(c: &mut DesktopController, context: ExecutionContext) {
    let before = peek(c).context;
    let analysis = matches!(
        context.purpose,
        ExecutionPurpose::MoveAnalysis { .. } | ExecutionPurpose::ChessFullReachability { .. }
    );
    let observation = ExecutionObservation::new(
        context,
        ExecutionResult::Failure {
            failure: mshogi_engine::EngineFailure {
                kind: mshogi_engine::FailureKind::Start,
                message: "retired work".into(),
            },
        },
    );
    let accepted = if matches!(c.session, ActiveDocument::Chess(_)) {
        let response = ExecutionResponse {
            protocol: mshogi_engine::PROTOCOL_VERSION,
            observation,
        };
        if analysis {
            c.session.chess_mut().accept_analysis(response)
        } else {
            c.session.chess_mut().accept(response, 1)
        }
    } else {
        c.accept_observation(observation)
    };
    assert!(!accepted);
    assert_eq!(peek(c).context, before);
}
static TEMP: AtomicU64 = AtomicU64::new(1);
fn path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "msh125-{}-{}.mshogi",
        std::process::id(),
        TEMP.fetch_add(1, Ordering::Relaxed)
    ))
}
fn file(c: &mut DesktopController, id: u64) {
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        pump(c);
        if c.io_state.id == id && matches!(c.io_state.phase, IoPhase::Complete | IoPhase::Failed) {
            assert_eq!(c.io_state.phase, IoPhase::Complete, "{:?}", c.io_state);
            return;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
#[test]
fn browse_only_human_return_restores_actions_and_exact_live_context_in_one_admission() {
    for game in GAMES {
        let mut c = running(game, false);
        let before = peek(&c);
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        let history = peek(&c);
        assert!(history.historical && history.paused);
        assert!(!history.capabilities.board_action);
        // Selecting another exact prefix retains the original running intent.
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert!(
            !live.historical && !live.paused && live.capabilities.board_action,
            "{game:?}"
        );
        assert_eq!(live.context, before.context);
        assert!(live.token.selection > history.token.selection);
        assert!(
            c.shared_command(SharedDesktopCommand {
                expected: history.token,
                game,
                operation: SessionOperation::Resign {}
            })
            .is_err()
        );
    }
}
#[test]
fn engine_return_allocates_fresh_purpose_version_request_generation_and_rejects_old_work() {
    for game in GAMES {
        let mut c = running(game, true);
        let old = pending(&mut c);
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        retired(&mut c, old.clone());
        let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert!(!live.paused && !live.historical);
        let fresh = pending(&mut c);
        assert!(
            fresh.request_id > old.request_id
                && fresh.process_generation > old.process_generation
                && fresh.session_version > old.session_version
        );
        assert_eq!(fresh.game, old.game);
        assert_eq!(fresh.limits, old.limits);
        assert!(matches!(fresh.purpose, ExecutionPurpose::EngineTurn {}));
        retired(&mut c, old);
    }
}
#[test]
fn historical_analysis_is_retired_before_return_but_cancel_and_failure_override_browse_intent() {
    for game in GAMES {
        for stop in [false, true] {
            let mut c = running(game, false);
            send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
            send(
                &mut c,
                SessionOperation::Analyze {
                    operation: AnalysisOperation::Moves {
                        depth: 1,
                        nodes: 32,
                    },
                },
            )
            .unwrap();
            let analysis = pending(&mut c);
            assert!(matches!(
                analysis.purpose,
                ExecutionPurpose::MoveAnalysis { .. }
            ));
            if stop {
                send(&mut c, SessionOperation::Cancel {}).unwrap();
            }
            let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
            assert_eq!(live.paused, stop, "{game:?}");
            retired(&mut c, analysis);
        }
        let mut c = running(game, false);
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        c.pause_native_failure("native operation failed".into());
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        assert!(
            send(&mut c, SessionOperation::ReturnLive {})
                .unwrap()
                .paused
        );
    }
}
#[test]
fn reopened_history_never_carries_browse_intent_and_records_fresh_resume_without_format_change() {
    for game in GAMES {
        let mut c = running(game, false);
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        let p = path();
        let id = c.queue_shared_save(p.clone()).unwrap();
        file(&mut c, id);
        let id = c
            .queue_product_open(p.clone(), c.shared_open_token())
            .unwrap();
        file(&mut c, id);
        let loaded = peek(&c);
        assert!(loaded.paused && loaded.historical && !loaded.document.dirty);
        let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert!(live.paused && !live.historical);
        send(&mut c, SessionOperation::Resume {}).unwrap();
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        let before = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert!(!before.paused);
        let id = c.queue_shared_save(p.clone()).unwrap();
        file(&mut c, id);
        let id = c
            .queue_product_open(p.clone(), c.shared_open_token())
            .unwrap();
        file(&mut c, id);
        let restored = peek(&c);
        assert!(restored.paused && !restored.historical);
        assert_eq!(restored.context, before.context);
        assert!(
            restored.token.document > loaded.token.document
                && restored.token.version > before.token.version
        );
        std::fs::remove_file(p).unwrap();
    }
}
#[test]
fn takeback_and_capacity_stops_cannot_be_overridden_by_browsing() {
    for game in GAMES {
        let mut c = running(game, false);
        let operation = match game {
            GameKind::Chess => SessionOperation::ChessPlay {
                action: "e2e4".parse().unwrap(),
            },
            GameKind::Shogi => SessionOperation::Play {
                action: "7g7f".parse().unwrap(),
            },
            GameKind::Xiangqi => SessionOperation::Xiangqi {
                command: x::GameCommand::Play {
                    action: "a3a4".parse().unwrap(),
                },
            },
        };
        send(&mut c, operation).unwrap();
        let old = pending(&mut c);
        let backed = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert!(backed.paused);
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        assert!(
            send(&mut c, SessionOperation::ReturnLive {})
                .unwrap()
                .paused
        );
        retired(&mut c, old);
        send(&mut c, SessionOperation::Resume {}).unwrap();
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        c.pause_native_capacity("Application capacity: no slot available".into());
        let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert!(live.paused);
        assert!(live.status.score_half_points.is_none());
    }
}
#[test]
fn clean_close_stop_is_not_falsely_saved_and_newer_dirty_authority_still_requires_guard() {
    for game in GAMES {
        let mut c = running(game, false);
        let p = path();
        let id = c.queue_shared_save(p.clone()).unwrap();
        file(&mut c, id);
        assert!(!peek(&c).document.dirty);
        let before = std::fs::read(&p).unwrap();
        c.lifecycle.request_close().unwrap();
        let close = c.application_pause_for_close();
        assert!(!close.close_requires_save, "{game:?}");
        assert_eq!(std::fs::read(&p).unwrap(), before); // No implicit Save of close controls.
        c.session.document_revision += 1;
        assert!(peek(&c).close_requires_save);
        assert!(
            c.lifecycle
                .approve_close(close.open_token, c.shared_open_token())
                .is_err()
        );
        c.lifecycle.cancel_close().unwrap();
        assert!(peek(&c).close_requires_save);
        std::fs::remove_file(p).unwrap();
    }
}

#[test]
fn failed_or_cancelled_engine_turn_keeps_explicit_recovery_after_history_navigation() {
    for game in GAMES {
        for failed in [false, true] {
            let mut c = running(game, true);
            let request = pending(&mut c);
            if failed {
                let observation = ExecutionObservation::new(
                    request,
                    ExecutionResult::Failure {
                        failure: mshogi_engine::EngineFailure {
                            kind: mshogi_engine::FailureKind::Start,
                            message: "known engine failure".into(),
                        },
                    },
                );
                if game == GameKind::Chess {
                    assert!(c.session.chess_mut().accept(
                        ExecutionResponse {
                            protocol: mshogi_engine::PROTOCOL_VERSION,
                            observation
                        },
                        1
                    ));
                } else {
                    assert!(c.accept_observation(observation));
                    while c.admission_token.is_some() {
                        pump(&mut c);
                        std::thread::yield_now();
                    }
                }
            } else {
                send(&mut c, SessionOperation::Cancel {}).unwrap();
            }
            let stopped = peek(&c);
            assert_eq!(
                stopped.phase,
                if failed {
                    EnginePhase::Failed
                } else {
                    EnginePhase::Cancelled
                }
            );
            send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
            send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
            let returned = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
            assert_eq!(returned.phase, stopped.phase, "{game:?}");
            assert!(returned.execution_purpose.is_none());
            assert_eq!(returned.context, stopped.context);
            assert_eq!(returned.diagnostic, stopped.diagnostic);
            // Native explicit Retry remains valid, with fresh identities only.
            send(&mut c, SessionOperation::Retry {}).unwrap();
            let fresh = pending(&mut c);
            assert!(fresh.request_id > 0);
        }
    }
}
#[test]
fn concluded_return_is_view_only_and_resignation_retains_native_unscored_chess_meaning() {
    for game in GAMES {
        let mut c = running(game, false);
        let ended = send(&mut c, SessionOperation::Resign {}).unwrap();
        assert!(ended.status.concluded);
        if game == GameKind::Chess {
            assert!(ended.status.score_half_points.is_none());
        }
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert!(live.status.concluded && live.paused);
        assert_eq!(live.context, ended.context);
        assert!(live.execution_purpose.is_none());
        assert!(send(&mut c, SessionOperation::Resume {}).is_err());
    }
}

#[test]
fn newer_historical_analysis_stop_supersedes_older_engine_failure() {
    for game in GAMES {
        for cancel in [true, false] {
            let mut c = running(game, true);
            let request = pending(&mut c);
            let old = ExecutionObservation::new(
                request,
                ExecutionResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Start,
                        message: "older engine failure".into(),
                    },
                },
            );
            if game == GameKind::Chess {
                assert!(c.session.chess_mut().accept(
                    ExecutionResponse {
                        protocol: mshogi_engine::PROTOCOL_VERSION,
                        observation: old
                    },
                    1
                ));
            } else {
                assert!(c.accept_observation(old));
                while c.admission_token.is_some() {
                    pump(&mut c);
                    std::thread::yield_now();
                }
            }
            send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
            send(
                &mut c,
                SessionOperation::Analyze {
                    operation: AnalysisOperation::Moves {
                        depth: 1,
                        nodes: 32,
                    },
                },
            )
            .unwrap();
            let analysis = pending(&mut c);
            if cancel {
                send(&mut c, SessionOperation::Cancel {}).unwrap();
            } else {
                let newer = ExecutionObservation::new(
                    analysis.clone(),
                    ExecutionResult::Failure {
                        failure: mshogi_engine::EngineFailure {
                            kind: mshogi_engine::FailureKind::Start,
                            message: "newer analysis failure".into(),
                        },
                    },
                );
                if game == GameKind::Chess {
                    assert!(c.session.chess_mut().accept_analysis(ExecutionResponse {
                        protocol: mshogi_engine::PROTOCOL_VERSION,
                        observation: newer
                    }));
                } else {
                    assert!(c.accept_observation(newer));
                    while c.admission_token.is_some() {
                        pump(&mut c);
                        std::thread::yield_now();
                    }
                }
            }
            let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
            assert!(live.paused);
            let diagnostic = live.diagnostic.as_deref().unwrap_or("");
            assert!(
                !diagnostic.contains("older engine failure"),
                "{game:?}: {diagnostic}"
            );
            if cancel {
                if game == GameKind::Chess {
                    assert!(diagnostic.to_lowercase().contains("cancel"));
                    assert_eq!(live.phase, EnginePhase::Paused);
                } else {
                    assert_eq!(live.phase, EnginePhase::Cancelled);
                }
            } else {
                assert!(
                    diagnostic.contains("newer analysis failure"),
                    "{game:?}: {diagnostic}"
                );
            }
            assert!(live.execution_purpose.is_none());
            retired(&mut c, analysis);
        }
    }
}
#[test]
fn finishing_concluded_chess_full_analysis_retires_work_without_reopening_game() {
    let mut c = running(GameKind::Chess, false);
    let ended = send(&mut c, SessionOperation::Resign {}).unwrap();
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::FullReachability {
                target: mshogi_engine::ReachabilityTarget::Either,
                max_positions: 1,
                max_transitions: 1,
            },
        },
    )
    .unwrap();
    let analysis = pending(&mut c);
    assert!(matches!(
        analysis.purpose,
        ExecutionPurpose::ChessFullReachability { .. }
    ));
    let finished = send(&mut c, SessionOperation::Resume {}).unwrap();
    assert!(finished.status.concluded && !finished.capabilities.board_action);
    assert!(finished.execution_purpose.is_none());
    assert_eq!(finished.context, ended.context);
    assert_eq!(
        finished.status.score_half_points,
        ended.status.score_half_points
    );
    retired(&mut c, analysis);
}

#[test]
fn untouched_never_saved_chess_requires_guard_without_fabricating_canonical_changes() {
    let mut c = DesktopController::new("/unused/msh125-initial-engine".into()).unwrap();
    c.shared.manual_engine = true;
    let initial = peek(&c);
    assert!(!initial.document.dirty && initial.document.path.is_none());
    assert!(initial.close_requires_save);
    c.lifecycle.request_close().unwrap();
    let guarded = c.application_pause_for_close();
    assert!(guarded.close_requires_save);
    assert_eq!(guarded.context, initial.context);
    assert_eq!(guarded.document.dirty, initial.document.dirty);
    c.lifecycle.cancel_close().unwrap();
    assert!(peek(&c).close_requires_save);
    let p = path();
    let id = c.queue_shared_save(p.clone()).unwrap();
    file(&mut c, id);
    assert!(!peek(&c).close_requires_save);
    assert!(!peek(&c).document.dirty);
    assert!(peek(&c).document.path.is_some());
    std::fs::remove_file(p).unwrap();
}
