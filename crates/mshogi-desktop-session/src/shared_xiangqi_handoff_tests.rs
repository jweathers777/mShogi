//! Aggregate MSH-76 journeys across existing native boundaries, not GUI evidence.
use super::*;

fn warned_game() -> x::Game {
    // Frozen, independently reviewed WXF expectation; no comparator-derived oracle.
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/history-cycles.json"
    ))
    .unwrap();
    let fixture = fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "F022-CC")
        .unwrap();
    let mut game = x::Game::from_root(
        x::PositionRoot::from_fen(fixture["root_fen"].as_str().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    for action in fixture["moves"].as_array().unwrap() {
        submit(&mut game, action.as_str().unwrap());
    }
    assert_eq!(game.events().len(), 9);
    assert_eq!(game.warnings().len(), 1);
    assert!(game.conclusion().is_none());
    game
}

fn wait_file(c: &mut DesktopController, id: u64) -> SharedSnapshot {
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if snapshot.io.id == id {
            match snapshot.io.phase {
                IoPhase::Complete => return snapshot,
                IoPhase::Failed => panic!("archive handoff failed: {:?}", snapshot.io.message),
                _ => {}
            }
        }
        assert!(Instant::now() < deadline, "archive handoff deadline");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[test]
fn warning_inspection_archive_reopen_and_resume_preserve_one_native_context() {
    let mut c = controller();
    let game = warned_game();
    replace(&mut c, &game, x::Side::Black);
    let canonical = native(&c);
    let observed = execute(&analyze(&mut c, 128));
    assert!(accept(&mut c, observed.clone()));
    assert_eq!(native(&c), canonical, "analysis is read-only");
    let late = execute(&analyze(&mut c, 128));
    // Analysis remembers its submitted limits; set the desired document defaults last.
    let settings = XiangqiSettings {
        orientation: x::Side::Black,
        analysis_defaults: WorkLimits {
            depth: 2,
            nodes: 64,
        },
    };
    send(
        &mut c,
        SessionOperation::XiangqiPreferences {
            settings: settings.clone(),
        },
    )
    .unwrap();
    let selected = send(&mut c, SessionOperation::Inspect { event_index: 8 }).unwrap();
    let old_query = MovementInspectionQuery {
        expected: selected.token,
        origin: "e9".into(),
    };
    let inspected = c.shared_inspection(old_query.clone()).unwrap();
    assert!(inspected.read_only);
    assert_eq!(inspected.side.label, "Red", "opponent of assigned Black");
    assert!(inspected.choices.iter().any(|a| a.action == "e9f9"));
    assert!(selected.xiangqi.as_ref().unwrap().warnings.is_empty());
    assert!(
        selected
            .xiangqi
            .as_ref()
            .unwrap()
            .selected_canonical
            .is_none()
    );
    assert!(!selected.capabilities.move_analysis, "intra-command prefix");

    let file = std::env::temp_dir().join(format!("msh76-handoff-{}.json", std::process::id()));
    let save = c.queue_shared_save(file.clone()).unwrap();
    wait_file(&mut c, save);
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
            human: NativeSide::Shogi(s::Side::Sente),
        },
    )
    .unwrap();
    assert!(!accept(&mut c, late.clone()));
    let floor = c.session.next_request;
    let open = c
        .queue_shared_open(file.clone(), c.shared_open_token())
        .unwrap();
    let reopened = wait_file(&mut c, open);
    assert_ne!(reopened.token, selected.token);
    assert!(c.session.next_request > floor);
    assert_eq!(native(&c), canonical);
    assert_eq!(reopened.game, GameKind::Xiangqi);
    assert_eq!(reopened.profile.id(), x::PROFILE_ID);
    assert!(reopened.historical && reopened.paused);
    assert_eq!(reopened.event_index, 8);
    assert!(reopened.xiangqi.as_ref().unwrap().warnings.is_empty());
    assert_eq!(reopened.xiangqi.as_ref().unwrap().selected_events.len(), 8);
    assert_eq!(
        reopened.xiangqi.as_ref().unwrap().canonical_context_scope,
        "live_with_immutable_selection"
    );
    let state = c.shared.xiangqi.as_ref().unwrap();
    assert_eq!(state.settings, settings);
    assert!(state.pending.is_none() && state.analysis.is_none());
    assert_eq!(state.observations.len(), 1);
    assert!(state.observations[0].historical);
    assert_eq!(state.observations[0].observation, observed);
    assert!(c.shared_inspection(old_query).is_err());
    assert!(!accept(&mut c, late.clone()));
    let fresh_inspection = c
        .shared_inspection(MovementInspectionQuery {
            expected: reopened.token,
            origin: "e9".into(),
        })
        .unwrap();
    assert_eq!(
        serde_json::to_value(&fresh_inspection.choices).unwrap(),
        serde_json::to_value(&inspected.choices).unwrap()
    );
    assert!(send(&mut c, SessionOperation::Resume {}).is_err());
    let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert!(live.paused && !live.historical);
    assert_eq!(live.xiangqi.as_ref().unwrap().warnings.len(), 1);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let turn = pending(&c);
    assert!(matches!(turn.purpose, ExecutionPurpose::EngineTurn {}));
    assert_eq!(turn.game, canonical);
    assert!(!accept(&mut c, late));
    assert!(accept(&mut c, execute(&turn)));
    assert_eq!(c.shared_snapshot().board_ply, 9);
    std::fs::remove_file(file).unwrap();
}

#[test]
fn same_board_history_cancel_and_cross_game_replacement_cannot_revive_results() {
    let mut c = controller();
    let chess = c.shared_snapshot().context;
    let warned = warned_game();
    let root = warned.committed_prefix(0).unwrap();
    assert_eq!(
        warned.position().pieces().collect::<Vec<_>>(),
        root.position().pieces().collect::<Vec<_>>()
    );
    assert_eq!(
        warned.position().side_to_move(),
        root.position().side_to_move()
    );
    replace(&mut c, &warned, x::Side::Black);
    let warned_request = analyze(&mut c, 128);
    let warned_response = execute(&warned_request);
    let historical = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(historical.xiangqi.as_ref().unwrap().warnings.is_empty());
    let root_request = analyze(&mut c, 128);
    assert_ne!(warned_request.game, root_request.game);
    assert_eq!(root_request.game.event_count(), 0);
    assert!(!accept(&mut c, warned_response.clone()));
    let cancelled_response = execute(&root_request);
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    let fresh = analyze(&mut c, 128);
    assert_eq!(fresh.game, root_request.game);
    assert!(fresh.request_id > root_request.request_id);
    assert!(!accept(&mut c, cancelled_response));
    let mut wrong_game = execute(&fresh);
    wrong_game.context.game = chess;
    assert!(!accept(&mut c, wrong_game));
    let valid_response = execute(&fresh);
    let wrong_profile = serde_json::to_string(&valid_response)
        .unwrap()
        .replace(x::PROFILE_ID, "mshogi.xiangqi.unsupported.v1");
    assert!(serde_json::from_str::<ExecutionObservation>(&wrong_profile).is_err());
    assert_eq!(pending(&c), fresh, "rejection preserves valid pending work");
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
            human: NativeSide::Shogi(s::Side::Sente),
        },
    )
    .unwrap();
    assert!(!accept(&mut c, valid_response.clone()));
    replace(&mut c, &warned, x::Side::Black);
    assert!(!accept(&mut c, warned_response));
    assert!(!accept(&mut c, valid_response));
    let live = native(&c);
    let current = analyze(&mut c, 128);
    assert_eq!(current.game, live);
    assert!(accept(&mut c, execute(&current)));
    assert_eq!(native(&c), live);
    assert_eq!(c.shared_snapshot().xiangqi.unwrap().warnings.len(), 1);
}
