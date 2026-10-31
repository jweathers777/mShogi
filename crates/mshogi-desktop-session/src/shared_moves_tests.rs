//! MSH-123 real native/controller projections. These are not packaged GUI tests.
use super::*;
use mshogi_core::xiangqi as x;
use std::sync::atomic::{AtomicU64, Ordering};

fn controller() -> DesktopController {
    let mut c = DesktopController::new("/unused/msh123-engine".into()).unwrap();
    c.shared.manual_engine = true;
    c
}

fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    let game = c.shared_snapshot().game;
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
        assert!(Instant::now() < deadline, "Moves admission deadline");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn replace(
    c: &mut DesktopController,
    record: CanonicalRecord,
    human: NativeSide,
) -> SharedSnapshot {
    send(c, SessionOperation::Replace { record, human }).unwrap()
}

fn wait_file(c: &mut DesktopController, id: u64) -> SharedSnapshot {
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if snapshot.io.id == id && matches!(snapshot.io.phase, IoPhase::Complete | IoPhase::Failed)
        {
            assert_eq!(
                snapshot.io.phase,
                IoPhase::Complete,
                "{:?}",
                snapshot.io.message
            );
            return snapshot;
        }
        assert!(Instant::now() < deadline, "Moves archive deadline");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn reopen(c: &mut DesktopController) -> SharedSnapshot {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let file = std::env::temp_dir().join(format!(
        "msh123-moves-{}-{}.mshogi",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let save = c.queue_shared_save(file.clone()).unwrap();
    wait_file(c, save);
    let open = c
        .queue_shared_open(file.clone(), c.shared_open_token())
        .unwrap();
    let snapshot = wait_file(c, open);
    std::fs::remove_file(file).unwrap();
    snapshot
}

fn cue(snapshot: &SharedSnapshot, action: &str, event: usize, origin: &str, target: &str) {
    let geometry = snapshot
        .moves
        .last_action
        .as_ref()
        .expect("selected native completed action");
    assert_eq!(geometry.action, action);
    assert_eq!(geometry.event_index, event);
    assert_eq!(geometry.steps, [target]);
    match &geometry.origin {
        ChoiceOrigin::Board { square } => assert_eq!(square, origin),
        ChoiceOrigin::Intersection { point } => assert_eq!(point, origin),
        other => panic!("expected native board origin, got {other:?}"),
    }
}

fn request(c: &mut DesktopController) -> ExecutionContext {
    send(
        c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 32,
            },
        },
    )
    .unwrap();
    if let Some(state) = c.session.shogi() {
        state.pending.clone().unwrap()
    } else {
        c.session.xiangqi().unwrap().pending.clone().unwrap()
    }
}

fn execute(context: &ExecutionContext) -> ExecutionObservation {
    let mut output = Vec::new();
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!(
            "{}\n",
            serde_json::to_string(&execution_request(context.clone())).unwrap()
        )),
        &mut output,
    )
    .unwrap();
    let observation = decode_execution_observation(&output);
    assert!(observation.matches(context));
    observation
}

#[test]
fn shared_moves_chess_real_admission_reopen_and_takeback_do_not_fabricate_cues() {
    let mut c = controller();
    let root = c.shared_snapshot();
    assert!(root.moves.entries.is_empty() && root.moves.last_action.is_none());
    assert_eq!(
        root.event_index, 1,
        "the native Chess match has its Start command before board actions"
    );
    let played = send(
        &mut c,
        SessionOperation::ChessPlay {
            action: "e2e4".parse().unwrap(),
        },
    )
    .unwrap();
    assert_eq!(played.moves.entries.len(), 1);
    let first = &played.moves.entries[0];
    assert_eq!((first.number, first.column, first.event_index), (1, 0, 2));
    assert_eq!((&*first.notation, &*first.action), ("e4", "e2e4"));
    cue(&played, "e2e4", 2, "e2", "e4");
    // The manual-engine fixture applies to Shogi/Xiangqi; Chess still owns a
    // real supervised launch of the deliberately absent path. Cancellation can
    // win that race, or the explicit process-start failure can arrive first.
    // Neither permitted result may invent or remove a completed board action.
    match send(&mut c, SessionOperation::Cancel {}) {
        Ok(cancelled) => assert_eq!(cancelled.phase, EnginePhase::Cancelled),
        Err(message) => {
            assert_eq!(message, "There is no active engine search to cancel.");
            // Admission rejection retires Chess execution via the existing
            // shared native safety path; it publishes Paused, not a fabricated
            // move/result or a claim that the failed launch remains running.
            let retired = c.shared_snapshot();
            assert_eq!(retired.phase, EnginePhase::Paused);
            assert!(retired.paused && retired.execution_purpose.is_none());
        }
    }
    let after_cancel = c.shared_snapshot();
    assert_eq!(after_cancel.context, played.context);
    assert_eq!(
        serde_json::to_value(&after_cancel.moves.entries).unwrap(),
        serde_json::to_value(&played.moves.entries).unwrap()
    );
    cue(&after_cancel, "e2e4", 2, "e2", "e4");
    let before_open = c.shared_snapshot();
    let opened = reopen(&mut c);
    assert_ne!(opened.token, before_open.token);
    assert!(opened.paused && opened.execution_purpose.is_none());
    cue(&opened, "e2e4", 2, "e2", "e4");
    let selected_root = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(selected_root.historical && selected_root.moves.last_action.is_none());
    assert_eq!(
        selected_root.moves.entries.len(),
        1,
        "viewing root does not delete known move rows"
    );
    let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    cue(&live, "e2e4", 2, "e2", "e4");
    let taken = send(&mut c, SessionOperation::TakeBack {}).unwrap();
    assert_eq!(taken.board_ply, 0);
    assert!(taken.paused && taken.moves.entries.is_empty() && taken.moves.last_action.is_none());
    assert!(
        c.shared_command(SharedDesktopCommand {
            expected: opened.token,
            game: GameKind::Chess,
            operation: SessionOperation::Resume {}
        })
        .is_err()
    );
}

#[test]
fn shared_moves_shogi_exact_prefix_analysis_reopen_and_replacement_keep_identity() {
    let mut game = s::Game::new();
    for action in ["7g7f", "3c3d", "2g2f"] {
        game.submit(s::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: s::GameCommand::Play(action.parse().unwrap()),
        })
        .unwrap();
    }
    let mut c = controller();
    replace(
        &mut c,
        CanonicalRecord::shogi(s::GameRecord::from_game(&game)),
        NativeSide::Shogi(s::Side::Sente),
    );
    let selected = send(&mut c, SessionOperation::Inspect { event_index: 1 }).unwrap();
    assert_eq!(
        selected
            .moves
            .entries
            .iter()
            .map(|m| (m.number, m.column, m.notation.as_str(), m.event_index))
            .collect::<Vec<_>>(),
        [
            (1, 0, "７６歩", 1),
            (1, 1, "３４歩", 2),
            (2, 0, "２６歩", 3)
        ]
    );
    cue(&selected, "7g7f", 1, "7g", "7f");
    let old_request = request(&mut c);
    assert_eq!(
        old_request.game.event_count(),
        1,
        "selected target excludes the two future actions"
    );
    match &old_request.purpose {
        ExecutionPurpose::MoveAnalysis { target } => {
            assert_eq!((target.event_index, target.board_ply), (1, 1));
            assert_eq!(target.selection_id, c.shared_snapshot().token.selection);
            assert!(
                target.selection_id > selected.token.selection,
                "starting analysis retires the preceding execution identity"
            );
        }
        other => panic!("unexpected purpose {other:?}"),
    }
    let late = execute(&old_request);
    let selected_two = send(&mut c, SessionOperation::Inspect { event_index: 2 }).unwrap();
    cue(&selected_two, "3c3d", 2, "3c", "3d");
    assert!(!c.accept_observation(late.clone()));
    let opened = reopen(&mut c);
    assert!(opened.historical && opened.paused && opened.execution_purpose.is_none());
    assert_ne!(opened.token, selected_two.token);
    cue(&opened, "3c3d", 2, "3c", "3d");
    assert!(!c.accept_observation(late.clone()));
    let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    cue(&live, "2g2f", 3, "2g", "2f");
    let fresh = replace(
        &mut c,
        CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
        NativeSide::Shogi(s::Side::Sente),
    );
    assert!(fresh.moves.entries.is_empty() && fresh.moves.last_action.is_none());
    assert!(
        c.shared_command(SharedDesktopCommand {
            expected: opened.token,
            game: GameKind::Shogi,
            operation: SessionOperation::Inspect { event_index: 1 }
        })
        .is_err()
    );
    assert!(!c.accept_observation(late));
}

#[test]
fn shared_moves_xiangqi_action_plus_warning_preserves_exact_eight_and_nine_prefixes() {
    // Frozen F022-CC's independently reviewed cycle: third occurrence warns,
    // while the eighth ordinary action and its derived warning are distinct.
    let mut game = x::Game::from_root(
        x::PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    for action in ["e9f9", "f8e8", "f9e9", "e8f8"].into_iter().cycle().take(8) {
        game.submit(x::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: x::GameCommand::Play {
                action: action.parse().unwrap(),
            },
        })
        .unwrap();
    }
    assert_eq!(game.events().len(), 9);
    let mut c = controller();
    let record = CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap());
    replace(&mut c, record.clone(), NativeSide::Xiangqi(x::Side::Red));
    let before_warning = send(&mut c, SessionOperation::Inspect { event_index: 8 }).unwrap();
    assert_eq!(before_warning.event_index, 8);
    assert_eq!(
        before_warning.moves.entries.len(),
        8,
        "a warning is not a ninth move"
    );
    let last = before_warning.moves.entries.last().unwrap();
    assert_eq!(
        (last.event_index, last.command_start, last.command_end),
        (8, 7, 9)
    );
    assert_eq!(
        (
            &*before_warning.moves.entries[0].notation,
            &*before_warning.moves.entries[1].notation,
            &*last.notation
        ),
        ("R5=4", "K6=5", "K5=6")
    );
    cue(&before_warning, "e8f8", 8, "e8", "f8");
    assert!(before_warning.xiangqi.as_ref().unwrap().warnings.is_empty());
    let token = before_warning.token;
    assert!(
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: 1,
                    nodes: 32
                }
            }
        )
        .is_err()
    );
    assert_eq!(
        c.shared_token(),
        token,
        "rejection cannot normalize the selected prefix"
    );
    let warned = send(&mut c, SessionOperation::Inspect { event_index: 9 }).unwrap();
    assert_eq!(warned.xiangqi.as_ref().unwrap().warnings.len(), 1);
    assert_eq!(
        serde_json::to_value(&before_warning.pieces).unwrap(),
        serde_json::to_value(&warned.pieces).unwrap(),
        "same diagram, different native warning history"
    );
    cue(&warned, "e8f8", 8, "e8", "f8");
    assert_ne!(warned.token.selection, before_warning.token.selection);
    let pending = request(&mut c);
    assert_eq!(pending.game, record);
    match pending.purpose {
        ExecutionPurpose::MoveAnalysis { target } => {
            assert_eq!((target.event_index, target.board_ply), (9, 8))
        }
        other => panic!("unexpected purpose {other:?}"),
    }
    let root = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(root.moves.last_action.is_none());
    assert!(root.xiangqi.as_ref().unwrap().warnings.is_empty());
    assert_eq!(
        serde_json::to_value(&root.pieces).unwrap(),
        serde_json::to_value(&warned.pieces).unwrap(),
        "the cycle returns to the root diagram without erasing known history"
    );
    let root_request = request(&mut c);
    assert_eq!(root_request.game.event_count(), 0);
    assert_ne!(
        root_request.game, warned.context,
        "same-board root has no future warning history"
    );
}
