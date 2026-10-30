use super::*;
use std::sync::atomic::{AtomicU64, Ordering};
fn path(label: &str) -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "mshogi73-{}-{}-{label}.mshogi",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}
fn controller() -> DesktopController {
    let mut c = DesktopController::new("/unused/msh73-engine".into()).unwrap();
    c.shared.manual_engine = true;
    c
}
fn imported(sfen: &str) -> s::Game {
    s::Game::from_root(
        s::GameRoot::import(
            s::Position::from_sfen(sfen).unwrap(),
            s::RootCategory::ReducedMaterialDiagram,
            s::CounterSource::Supplied,
        )
        .unwrap(),
    )
}
fn native_command(g: &mut s::Game, actor: s::Side, command: s::GameCommand) {
    g.submit(s::GameRequest {
        expected: g.context(),
        actor,
        command,
    })
    .unwrap();
}
fn moves(g: &mut s::Game, moves: &[&str]) {
    for action in moves {
        native_command(
            g,
            g.position().side_to_move(),
            s::GameCommand::Play(action.parse().unwrap()),
        );
    }
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    settle(c);
    let current = c.shared_snapshot();
    let (_, receipt) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: current.token,
        game: current.game,
        operation,
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
fn settle(c: &mut DesktopController) {
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    while c.shared_snapshot().capacity.status == CapacityPhase::Checking {
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn replace(c: &mut DesktopController, g: &s::Game, human: s::Side) {
    send(
        c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(g)),
            human: NativeSide::Shogi(human),
        },
    )
    .unwrap();
}
fn wait_file(c: &mut DesktopController, id: u64) -> Result<SharedSnapshot, String> {
    let deadline = Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if snapshot.io.id == id {
            match snapshot.io.phase {
                IoPhase::Complete => return Ok(snapshot),
                IoPhase::Failed => return Err(snapshot.io.message.unwrap()),
                _ => {}
            }
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn save(c: &mut DesktopController, p: &std::path::Path) -> SharedSnapshot {
    let id = c.queue_shared_save(p.into()).unwrap();
    wait_file(c, id).unwrap()
}
fn open(c: &mut DesktopController, p: &std::path::Path) -> Result<SharedSnapshot, String> {
    let id = c
        .queue_shared_open(p.into(), c.shared_open_token())
        .unwrap();
    wait_file(c, id)
}
fn record(c: &DesktopController) -> s::GameRecord {
    s::GameRecord::from_game(&c.session.shogi().as_ref().unwrap().game)
}
fn run(context: ExecutionContext) -> ExecutionObservation {
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
}
fn analyze(c: &mut DesktopController) -> ExecutionObservation {
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
    let context = c.session.shogi().unwrap().pending.clone().unwrap();
    let response = run(context);
    assert!(c.accept_observation(response.clone()));
    settle(c);
    response
}
#[test]
fn canonical_fixture_roundtrips_preserve_native_root_history_and_all_outcomes() {
    let mut fixtures = vec![
        ("standard", s::Game::new()),
        ("imported", imported("8k/9/9/9/9/9/9/9/K8 b 2RBGSNLP 17")),
    ];
    for action in ["5d5c", "5d5c+"] {
        let mut g = imported("8k/9/9/4P4/9/9/9/9/K8 b - 1");
        moves(&mut g, &[action]);
        fixtures.push((action, g));
    }
    let mut forced = imported("8k/4P4/9/9/9/9/9/9/K8 b - 1");
    moves(&mut forced, &["5b5a+"]);
    fixtures.push(("forced", forced));
    for (name, cycle) in [
        ("fourfold", ["4c3c", "5a4a", "3c4c", "4a5a"]),
        ("continuous", ["4c5c", "5a4a", "5c4c", "4a5a"]),
    ] {
        let mut g = imported("4k4/9/5R3/9/9/9/9/9/K8 b - 1");
        for _ in 0..3 {
            moves(&mut g, &cycle);
        }
        fixtures.push((name, g));
    }
    for (name, hand) in [("declaration", "B5P"), ("false-declaration", "B4P")] {
        let mut g = imported(&format!(
            "K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b {hand} 1"
        ));
        native_command(&mut g, s::Side::Sente, s::GameCommand::DeclareEnteringKing);
        fixtures.push((name, g));
    }
    let mut resigned = s::Game::new();
    native_command(&mut resigned, s::Side::Gote, s::GameCommand::Resign);
    fixtures.push(("resigned", resigned));
    for (name, g) in fixtures {
        let mut c = controller();
        replace(&mut c, &g, s::Side::Sente);
        let expected = record(&c);
        let file = path(name);
        assert!(!save(&mut c, &file).document.dirty);
        let bytes = fs::read(&file).unwrap();
        assert_eq!(
            crate::document::DocumentEnvelope::parse(&bytes)
                .unwrap()
                .envelope_version,
            1
        );
        let before = c.shared_token();
        let opened = open(&mut c, &file).unwrap();
        assert_eq!(record(&c), expected, "{name}");
        assert_eq!(c.session.shogi().unwrap().game.context(), g.context());
        assert!(opened.paused);
        assert!(opened.token.version > before.version);
        assert!(c.session.shogi().unwrap().pending.is_none());
        assert!(!opened.document.dirty);
        fs::remove_file(file).unwrap();
    }
}
#[test]
fn imported_capture_hand_and_drop_after_reopen_preserve_counter_provenance() {
    let mut g = imported("8k/9/4+p4/9/4R4/9/9/9/K8 b - 17");
    moves(&mut g, &["5e5c+", "1a2a"]);
    assert_eq!(
        g.position().hand(s::Side::Sente).count(s::PieceKind::Pawn),
        1
    );
    let mut c = controller();
    replace(&mut c, &g, s::Side::Sente);
    let file = path("hands");
    save(&mut c, &file);
    let mut reopened = controller();
    open(&mut reopened, &file).unwrap();
    assert_eq!(record(&reopened), s::GameRecord::from_game(&g));
    assert!(
        !reopened
            .session
            .shogi()
            .unwrap()
            .game
            .root()
            .has_complete_initial_history()
    );
    send(&mut reopened, SessionOperation::Resume {}).unwrap();
    send(
        &mut reopened,
        SessionOperation::Play {
            action: "P*5e".parse().unwrap(),
        },
    )
    .unwrap();
    assert_eq!(
        reopened
            .session
            .shogi()
            .unwrap()
            .game
            .position()
            .hand(s::Side::Sente)
            .count(s::PieceKind::Pawn),
        0
    );
    assert_eq!(
        reopened.session.shogi().unwrap().game.position().next_ply(),
        20
    );
    save(&mut reopened, &file);
    let expected = record(&reopened);
    open(&mut c, &file).unwrap();
    assert_eq!(record(&c), expected);
    fs::remove_file(file).unwrap();
}
#[test]
fn same_ply_terminal_history_and_saved_selection_remain_distinct() {
    let mut c = controller();
    replace(
        &mut c,
        &imported("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1"),
        s::Side::Sente,
    );
    send(&mut c, SessionOperation::Declare {}).unwrap();
    let expected = record(&c);
    let file = path("selection");
    send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    save(&mut c, &file);
    let before = open(&mut c, &file).unwrap();
    assert!(before.historical);
    assert_eq!(before.event_index, 0);
    assert!(!before.status.concluded);
    assert_eq!(record(&c), expected);
    send(&mut c, SessionOperation::Inspect { event_index: 1 }).unwrap();
    save(&mut c, &file);
    let after = open(&mut c, &file).unwrap();
    assert!(after.historical);
    assert_eq!(after.event_index, 1);
    assert!(after.status.concluded);
    assert_eq!(
        before.context.known_board_actions(),
        after.context.known_board_actions()
    );
    assert_ne!(before.context, after.context);
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert!(c.shared_snapshot().status.concluded);
    fs::remove_file(file).unwrap();
}
#[test]
fn observations_keep_exact_retired_branch_targets_and_never_resume_on_open() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    send(
        &mut c,
        SessionOperation::Play {
            action: "7g7f".parse().unwrap(),
        },
    )
    .unwrap();
    let engine_request = c.session.shogi().unwrap().pending.clone().unwrap();
    let engine = run(engine_request);
    assert!(c.accept_observation(engine));
    settle(&mut c);
    let analysis = analyze(&mut c);
    send(&mut c, SessionOperation::Undo {}).unwrap();
    send(&mut c, SessionOperation::Undo {}).unwrap();
    send(&mut c, SessionOperation::Resume {}).unwrap();
    send(
        &mut c,
        SessionOperation::Play {
            action: "2g2f".parse().unwrap(),
        },
    )
    .unwrap();
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    let expected = record(&c);
    let responses: Vec<_> = c
        .session
        .shogi()
        .unwrap()
        .observations
        .iter()
        .map(|o| o.observation.clone())
        .collect();
    let file = path("branches");
    save(&mut c, &file);
    let runtime = c.runtime_floor();
    let opened = open(&mut c, &file).unwrap();
    assert_eq!(record(&c), expected);
    assert!(opened.paused);
    assert!(opened.analysis.is_none());
    assert!(c.runtime_floor() > runtime);
    assert!(!c.accept_observation(analysis));
    let restored = &c.session.shogi().unwrap().observations;
    assert_eq!(
        restored
            .iter()
            .map(|o| o.observation.clone())
            .collect::<Vec<_>>(),
        responses
    );
    assert!(restored.iter().all(|o| o.historical));
    assert!(restored[1].observation.context.game.event_count() > expected.event_count());
    fs::remove_file(file).unwrap();
}
#[test]
fn pending_runtime_is_not_archived_and_resume_allocates_fresh_identity() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Gote);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let old = c.session.shogi().unwrap().pending.clone().unwrap();
    let file = path("pending");
    save(&mut c, &file);
    let bytes = String::from_utf8(fs::read(&file).unwrap()).unwrap();
    assert!(!bytes.contains("\"pending\""));
    open(&mut c, &file).unwrap();
    assert!(c.session.shogi().unwrap().pending.is_none());
    assert!(!c.accept_observation(run(old.clone())));
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let fresh = c.session.shogi().unwrap().pending.clone().unwrap();
    assert!(fresh.request_id > old.request_id);
    assert!(fresh.process_generation > old.process_generation);
    assert_ne!(fresh.session_version, old.session_version);
    fs::remove_file(file).unwrap();
}
#[test]
fn hostile_archives_reject_without_changing_the_current_document() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    let good = path("valid");
    save(&mut c, &good);
    let text = fs::read_to_string(&good).unwrap();
    let expected = record(&c);
    let version = c.session.version;
    let dirty = c.session.document_revision;
    let cases = [
        text.replacen("\"envelope_version\": 1", "\"envelope_version\": 99", 1),
        text.replacen("\"game_kind\": \"shogi\"", "\"game_kind\": \"xiangqi\"", 1),
        text.replacen(
            "\"rules_profile\": \"mshogi.shogi.csa36.v1\"",
            "\"rules_profile\": \"mshogi.chess.match.v1\"",
            1,
        ),
        text.replacen(
            "\"profile_edition\": \"JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026\"",
            "\"profile_edition\": \"unapproved\"",
            1,
        ),
        text.replacen(
            "\"envelope_version\": 1",
            "\"envelope_version\": 1,\"envelope_version\":1",
            1,
        ),
        text.replacen(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
            1,
        ),
        text.replacen(
            "\"human\":{\"game_kind\":\"shogi\",\"side\":\"sente\"}",
            "\"human\":{\"game_kind\":\"chess\",\"side\":\"white\"}",
            1,
        ),
        text.replacen(
            "\"retired_execution_floor\":",
            "\"unexpected\":0,\"retired_execution_floor\":",
            1,
        ),
    ];
    let bad = path("hostile");
    for value in cases {
        assert_ne!(value, text);
        fs::write(&bad, value).unwrap();
        assert!(open(&mut c, &bad).is_err());
        assert_eq!(record(&c), expected);
        assert_eq!(c.session.version, version);
        assert_eq!(c.session.document_revision, dirty);
    }
    fs::write(&bad, vec![b' '; MAX_ARCHIVE_BYTES + 1]).unwrap();
    assert!(open(&mut c, &bad).unwrap_err().contains("capacity"));
    assert_eq!(record(&c), expected);
    fs::remove_file(bad).unwrap();
    fs::remove_file(good).unwrap();
}
#[test]
fn wrong_native_game_and_semantic_journal_or_observation_tampering_reject() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    analyze(&mut c);
    let payload = archive(c.session.shogi().unwrap(), c.runtime_floor());
    let expected = record(&c);
    let serialize = |p: &ShogiArchive| serde_json::to_string(p).unwrap();
    let mut wrong: ShogiArchive = serde_json::from_str(&serialize(&payload)).unwrap();
    wrong.current =
        CanonicalRecord::chess(AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record());
    assert!(
        decode_shogi(&serialize(&wrong), &mut || false)
            .err()
            .unwrap()
            .contains("Wrong game")
    );
    let mut wrong: ShogiArchive = serde_json::from_str(&serialize(&payload)).unwrap();
    wrong.controls[0].sequence = 2;
    assert!(decode_shogi(&serialize(&wrong), &mut || false).is_err());
    let mut wrong: ShogiArchive = serde_json::from_str(&serialize(&payload)).unwrap();
    wrong.controls[0].command = Some(SessionOperation::Play {
        action: "9a9b".parse().unwrap(),
    });
    assert!(decode_shogi(&serialize(&wrong), &mut || false).is_err());
    let mut wrong: ShogiArchive = serde_json::from_str(&serialize(&payload)).unwrap();
    if let ExecutionPurpose::MoveAnalysis { target } =
        &mut wrong.observations[0].observation.context.purpose
    {
        target.turn = NativeSide::Shogi(s::Side::Gote);
    }
    assert!(decode_shogi(&serialize(&wrong), &mut || false).is_err());
    let mut wrong: ShogiArchive = serde_json::from_str(&serialize(&payload)).unwrap();
    wrong.observations[0].observation.context.limits.nodes += 1;
    assert!(decode_shogi(&serialize(&wrong), &mut || false).is_err());
    let mut wrong: ShogiArchive = serde_json::from_str(&serialize(&payload)).unwrap();
    wrong.settings.orientation = s::Side::Gote;
    assert!(decode_shogi(&serialize(&wrong), &mut || false).is_err());
    assert_eq!(record(&c), expected);
}
#[test]
fn current_chess_preserves_semantics_and_development_formats_reject() {
    let mut c = controller();
    let file = path("chess-current");
    save(&mut c, &file);
    let bytes = fs::read(&file).unwrap();
    let envelope = crate::document::DocumentEnvelope::parse(&bytes).unwrap();
    assert_eq!(envelope.game_kind, GameKind::Chess);
    let expected = c.session.chess_mut().game.record().to_json();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    let opened = open(&mut c, &file).unwrap();
    assert_eq!(opened.game, GameKind::Chess);
    assert_eq!(c.session.chess_mut().game.record().to_json(), expected);
    assert!(opened.paused);
    let bad = path("development");
    for version in [1, 2, 3] {
        let mut old: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        old.as_object_mut().unwrap().remove("envelope_version");
        old["schema_version"] = version.into();
        old["format"] = format!("mshogi.desktop.archive.v{version}").into();
        fs::write(&bad, serde_json::to_vec(&old).unwrap()).unwrap();
        let before = c.shared_open_token();
        assert!(open(&mut c, &bad).is_err());
        assert_eq!(c.shared_open_token(), before);
        assert_eq!(c.session.chess_mut().game.record().to_json(), expected);
    }
    for p in [file, bad] {
        fs::remove_file(p).unwrap();
    }
}
#[test]
fn interrupted_and_failed_native_saves_preserve_file_and_dirty_state() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    let file = path("atomic");
    save(&mut c, &file);
    let valid = fs::read(&file).unwrap();
    send(&mut c, SessionOperation::Resign {}).unwrap();
    assert!(c.shared_document().dirty);
    let mut checkpoints = 0;
    let error = persistence::atomic_write(&file, &vec![b'x'; 100_000], &mut || {
        checkpoints += 1;
        checkpoints > 3
    })
    .unwrap_err();
    assert!(error.contains("cancelled"));
    assert_eq!(fs::read(&file).unwrap(), valid);
    let directory = path("directory");
    fs::create_dir(&directory).unwrap();
    let id = c.queue_shared_save(directory.clone()).unwrap();
    assert!(wait_file(&mut c, id).is_err());
    assert!(c.shared_document().dirty);
    assert_eq!(fs::read(&file).unwrap(), valid);
    fs::remove_dir(directory).unwrap();
    fs::remove_file(file).unwrap();
}
#[test]
fn stale_open_and_cross_game_save_completion_never_replace_current_state() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    let file = path("stale");
    save(&mut c, &file);
    let token = c.shared_open_token();
    let (state, floor) = decode_shogi(
        crate::document::DocumentEnvelope::parse(&fs::read(&file).unwrap())
            .unwrap()
            .payload
            .get(),
        &mut || false,
    )
    .unwrap();
    let old_id = c.session.document_identity;
    send(&mut c, SessionOperation::Resign {}).unwrap();
    let expected = record(&c);
    assert!(
        c.finish_native_file(NativeCompleted::Opened {
            game: OpenedNative::Shogi {
                state: Box::new(state),
                floor
            },
            path: file.clone(),
            token
        })
        .unwrap_err()
        .contains("Stale")
    );
    assert_eq!(record(&c), expected);
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
    let current_path = c.session.document_path.clone();
    c.finish_native_file(NativeCompleted::Saved {
        path: file.clone(),
        revision: 0,
        identity: old_id,
    })
    .unwrap();
    assert_eq!(c.session.document_path, current_path);
    fs::remove_file(file).unwrap();
}

#[test]
fn concurrent_save_as_keeps_new_admission_dirty_and_open_retires_it() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    let original = path("before-admission");
    save(&mut c, &original);
    let settings = ShogiSettings {
        orientation: s::Side::Gote,
        analysis_defaults: WorkLimits {
            depth: 2,
            nodes: 12,
        },
    };
    let receipt = c
        .queue_native_candidate(
            c.session.shogi().unwrap().clone(),
            Some(SessionOperation::ShogiPreferences {
                settings: settings.clone(),
            }),
            None,
            false,
        )
        .unwrap();
    let copy = path("save-as-admission");
    let id = c.queue_shared_save(copy.clone()).unwrap();
    // Deliberately accept the file completion before the candidate completion.
    let deadline = Instant::now() + std::time::Duration::from_secs(10);
    while c.io_state.phase == IoPhase::Saving {
        c.poll_files();
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert_eq!(c.io_state.id, id);
    assert_eq!(c.io_state.phase, IoPhase::Complete);
    settle(&mut c);
    assert_eq!(receipt.completion(), Some(Ok(())));
    assert_eq!(c.session.shogi().unwrap().settings, settings);
    assert!(c.shared_document().dirty);
    assert_eq!(c.session.document_path.as_ref(), Some(&copy));
    let (saved, _) = decode_shogi(
        crate::document::DocumentEnvelope::parse(&fs::read(&copy).unwrap())
            .unwrap()
            .payload
            .get(),
        &mut || false,
    )
    .unwrap();
    assert_eq!(saved.settings.orientation, s::Side::Sente);
    let id = c
        .queue_shared_open(original.clone(), c.shared_open_token())
        .unwrap();
    let receipt = c
        .queue_native_candidate(
            c.session.shogi().unwrap().clone(),
            Some(SessionOperation::Resign {}),
            None,
            false,
        )
        .unwrap();
    while c.io_state.phase == IoPhase::Opening {
        c.poll_files();
        assert!(Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert_eq!(c.io_state.id, id);
    assert_eq!(c.io_state.phase, IoPhase::Complete);
    assert!(receipt.completion().unwrap().is_err());
    c.poll_native_admission();
    assert!(c.session.shogi().unwrap().game.conclusion().is_none());
    assert!(!c.shared_document().dirty);
    for p in [original, copy] {
        fs::remove_file(p).unwrap();
    }
}
#[test]
fn persisted_preferences_keep_exact_pending_execution_and_original_limits() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
        },
    )
    .unwrap();
    let request = c.session.shogi().unwrap().pending.clone().unwrap();
    let version = c.shared_token().version;
    let settings = ShogiSettings {
        orientation: s::Side::Gote,
        analysis_defaults: WorkLimits {
            depth: 3,
            nodes: 77,
        },
    };
    send(
        &mut c,
        SessionOperation::ShogiPreferences {
            settings: settings.clone(),
        },
    )
    .unwrap();
    assert_eq!(c.shared_token().version, version);
    assert_eq!(c.session.shogi().unwrap().pending.as_ref(), Some(&request));
    assert!(c.accept_observation(run(request)));
    settle(&mut c);
    let file = path("settings");
    save(&mut c, &file);
    open(&mut c, &file).unwrap();
    let state = c.session.shogi().unwrap();
    assert_eq!(state.settings, settings);
    assert_eq!(
        state.observations[0].observation.context.limits,
        WorkLimits { depth: 1, nodes: 0 }
    );
    assert!(state.observations[0].historical);
    fs::remove_file(file).unwrap();
}

#[test]
fn raw_nested_observation_fields_remain_strict_and_identity_errors_are_explicit() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    analyze(&mut c);
    let text =
        serde_json::to_string(&archive(c.session.shogi().unwrap(), c.runtime_floor())).unwrap();
    for (before, after) in [
        ("\"application_error\":null,", ""),
        (
            "\"application_error\":null",
            "\"application_error\":null,\"application_error\":null",
        ),
        (
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
        ),
        ("\"nodes\":1000", "\"nodes\":1000,\"unknown\":true"),
        ("\"request_id\":", "\"surprise\":null,\"request_id\":"),
    ] {
        let wrong = text.replacen(before, after, 1);
        assert_ne!(wrong, text);
        assert!(decode_shogi(&wrong, &mut || false).is_err(), "{before}");
    }
    let wrong = text.replacen(s::PROFILE_ID, "mshogi.unapproved.v1", 1);
    assert!(
        decode_shogi(&wrong, &mut || false)
            .err()
            .unwrap()
            .starts_with("Wrong profile/edition:")
    );
    let wrong = text.replacen("\"events\":[]", "\"events\":[],\"events\":[]", 1);
    assert_ne!(wrong, text);
    assert!(decode_shogi(&wrong, &mut || false).is_err());
}

#[test]
fn inability_to_start_native_execution_is_a_capacity_pause_not_engine_failure() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Gote);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    // Simulate the already accepted runtime reaching its identity guard before
    // the next automatic turn starts; current document must still be saveable.
    let state = c.session.shogi_mut().unwrap();
    state.retire();
    state.paused = false;
    c.session.next_request = MAX_SAFE_ID - 64;
    let snap = c.shared_snapshot();
    assert!(snap.paused);
    assert_eq!(snap.phase, EnginePhase::Paused);
    assert_eq!(snap.capacity.status, CapacityPhase::Paused);
    assert!(
        snap.capacity
            .message
            .unwrap()
            .starts_with("Application capacity:")
    );
    let file = path("runtime-capacity");
    save(&mut c, &file);
    fs::remove_file(file).unwrap();
}

#[test]
fn product_new_game_archive_reopens_native_human_orientation_stopped_and_fresh() {
    let mut c = controller();
    let (_, receipt) = c
        .shared_new_game(
            c.shared_open_token(),
            GameKind::Shogi,
            NativeSide::Shogi(s::Side::Gote),
        )
        .unwrap();
    settle(&mut c);
    receipt.completion().unwrap().unwrap();
    let before = c.shared_snapshot();
    let pending = c.session.shogi().unwrap().pending.clone().unwrap();
    let file = path("product-new-gote");
    save(&mut c, &file);
    let reopened = open(&mut c, &file).unwrap();
    assert_eq!(reopened.human.id, NativeSide::Shogi(s::Side::Gote));
    assert_eq!(reopened.context, before.context);
    assert!(reopened.paused);
    assert!(reopened.token.document > before.token.document);
    assert!(reopened.token.version > pending.session_version);
    match &reopened.document.settings {
        NativeDesktopSettings::Shogi { settings } => {
            assert_eq!(settings.orientation, s::Side::Gote)
        }
        _ => panic!("native shogi settings"),
    }
    assert!(c.session.shogi().unwrap().pending.is_none());
    let old_response = run(pending);
    assert!(!c.accept_observation(old_response));
    send(&mut c, SessionOperation::Resume {}).unwrap();
    assert!(
        c.session
            .shogi()
            .unwrap()
            .pending
            .as_ref()
            .unwrap()
            .request_id
            > before.token.version
    );
    std::fs::remove_file(file).unwrap();
}

#[test]
fn durable_observations_have_a_semantic_schema_without_transport_framing() {
    let mut c = controller();
    replace(&mut c, &s::Game::new(), s::Side::Sente);
    let accepted = analyze(&mut c);
    let payload = archive(c.session.shogi().unwrap(), c.runtime_floor());
    let encoded = serde_json::to_value(&payload).unwrap();
    let observation = &encoded["observations"][0]["observation"];
    assert_eq!(observation["schema_version"], OBSERVATION_SCHEMA_VERSION);
    assert!(observation.get("protocol").is_none());
    assert!(observation["context"].get("protocol").is_none());
    assert_eq!(
        observation["context"],
        serde_json::to_value(&accepted.context).unwrap()
    );
    for schema in [0, OBSERVATION_SCHEMA_VERSION + 1] {
        let mut wrong = encoded.clone();
        wrong["observations"][0]["observation"]["schema_version"] = schema.into();
        assert!(decode_shogi(&wrong.to_string(), &mut || false).is_err());
    }
    let mut framed = encoded;
    framed["observations"][0]["observation"]["protocol"] = mshogi_engine::PROTOCOL_VERSION.into();
    assert!(decode_shogi(&framed.to_string(), &mut || false).is_err());
}
