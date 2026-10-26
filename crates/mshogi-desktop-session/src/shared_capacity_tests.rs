//! Concrete shogi persistence admission boundaries, separate from rules conformance.
use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

fn controller() -> DesktopController {
    let mut c = DesktopController::new("/unused/msh73-capacity-engine".into()).unwrap();
    c.shared.manual_engine = true;
    c
}
fn path(label: &str) -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "msh73-capacity-{}-{}-{label}.mshogi",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}
fn settle(c: &mut DesktopController) {
    let deadline = Instant::now() + std::time::Duration::from_secs(30);
    while c.shared_snapshot().capacity.status == CapacityPhase::Checking {
        assert!(
            Instant::now() < deadline,
            "native admission did not complete"
        );
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<(), String> {
    settle(c);
    let current = c.shared_snapshot();
    let (_, receipt) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: current.token,
        game: current.game,
        operation,
    })?;
    settle(c);
    receipt.completion().expect("settled stable receipt")
}
fn replace(c: &mut DesktopController, game: &s::Game) {
    send(
        c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(game)),
            human: NativeSide::Shogi(s::Side::Sente),
        },
    )
    .unwrap();
}
fn wait_file(c: &mut DesktopController, id: u64) -> Result<(), String> {
    let deadline = Instant::now() + std::time::Duration::from_secs(30);
    loop {
        let snap = c.shared_snapshot();
        if snap.io.id == id {
            match snap.io.phase {
                IoPhase::Complete => return Ok(()),
                IoPhase::Failed => return Err(snap.io.message.unwrap()),
                _ => {}
            }
        }
        assert!(Instant::now() < deadline, "file worker did not complete");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn actual_response(request: SharedRequest) -> SharedResponse {
    let mut bytes = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
        &mut bytes,
    )
    .unwrap();
    let response: SharedResponse = serde_json::from_slice(&bytes).unwrap();
    assert!(response.matches(&request));
    response
}
fn analyze(c: &mut DesktopController) {
    send(
        c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
        },
    )
    .unwrap();
    let request = c.shared.shogi.as_ref().unwrap().pending.clone().unwrap();
    assert!(c.accept_shared(actual_response(request)));
    settle(c);
    assert_eq!(
        c.shared_snapshot().capacity.status,
        CapacityPhase::Available
    );
}
fn record(c: &DesktopController) -> s::GameRecord {
    s::GameRecord::from_game(&c.shared.shogi.as_ref().unwrap().game)
}

#[test]
fn actual_256_observations_save_reopen_and_reject_257_without_changing_accepted_document() {
    let mut c = controller();
    replace(&mut c, &s::Game::new());
    for count in 1..=MAX_ARCHIVE_OBSERVATIONS {
        analyze(&mut c);
        assert_eq!(c.shared.shogi.as_ref().unwrap().observations.len(), count);
    }
    let retained = record(&c);
    let revision = c.shared_snapshot().document.revision;
    let controls = c.shared.shogi.as_ref().unwrap().controls.len();
    let error = send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
        },
    )
    .unwrap_err();
    assert!(error.starts_with("Application capacity:"), "{error}");
    assert!(error.contains("256"));
    assert!(!error.contains("retry"));
    assert_eq!(record(&c), retained);
    assert_eq!(c.shared_snapshot().document.revision, revision);
    assert_eq!(c.shared.shogi.as_ref().unwrap().controls.len(), controls);
    assert!(c.shared.shogi.as_ref().unwrap().pending.is_none());
    let file = path("256-observations");
    let id = c.queue_shared_save(file.clone()).unwrap();
    wait_file(&mut c, id).unwrap();
    assert!(!c.shared_snapshot().document.dirty);
    let size = fs::metadata(&file).unwrap().len();
    let mut reopened = controller();
    let token = reopened.shared_open_token();
    let id = reopened.queue_shared_open(file.clone(), token).unwrap();
    wait_file(&mut reopened, id).unwrap();
    assert_eq!(record(&reopened), retained);
    let state = reopened.shared.shogi.as_ref().unwrap();
    assert_eq!(state.observations.len(), MAX_ARCHIVE_OBSERVATIONS);
    assert!(state.observations.iter().all(|o| o.historical));
    assert!(state.pending.is_none());
    assert!(state.paused);
    let revision = reopened.shared_snapshot().document.revision;
    let error = send(&mut reopened, SessionOperation::Resume {}).unwrap_err();
    assert!(error.starts_with("Application capacity:"));
    assert_eq!(reopened.shared_snapshot().document.revision, revision);
    assert_eq!(
        reopened.shared.shogi.as_ref().unwrap().observations.len(),
        256
    );
    let copy = path("256-save-as");
    let id = reopened.queue_shared_save(copy.clone()).unwrap();
    wait_file(&mut reopened, id).unwrap();
    assert!(!reopened.shared_snapshot().document.dirty);
    eprintln!(
        "MSH73 sizes standard256-observation archive={size} bytes; reopened save-as={} bytes",
        fs::metadata(&copy).unwrap().len()
    );
    fs::remove_file(file).unwrap();
    fs::remove_file(copy).unwrap();
}

#[test]
fn control_boundary_reserves_the_last_slot_for_close_and_never_truncates() {
    let mut state =
        ShogiSession::restored(&s::GameRecord::from_game(&s::Game::new()), s::Side::Sente).unwrap();
    for _ in 0..MAX_ARCHIVE_CONTROLS - 2 {
        let command = SessionOperation::Cancel {};
        state.apply_command(command.clone()).unwrap();
        state.control(Some(command), None, false);
    }
    let command = SessionOperation::Analyze {
        operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
    };
    state.apply_command(command.clone()).unwrap();
    state.control(Some(command), None, false);
    assert_eq!(state.controls.len(), MAX_ARCHIVE_CONTROLS - 1);
    assert!(needs_stop(&state));
    check_capacity(&state, &mut || false).unwrap();
    let mut too_many = state.clone();
    too_many.control(
        Some(SessionOperation::ShogiPreferences {
            settings: state.settings.clone(),
        }),
        None,
        false,
    );
    assert!(
        check_capacity(&too_many, &mut || false)
            .unwrap_err()
            .contains("stop/close")
    );
    state.retire();
    state.paused = true;
    state.control(None, None, true);
    assert_eq!(state.controls.len(), MAX_ARCHIVE_CONTROLS);
    check_capacity(&state, &mut || false).unwrap();
    let bytes = encode(&state, 100).unwrap();
    let envelope: ArchiveV2 = serde_json::from_slice(&bytes).unwrap();
    let (restored, _) = decode_shogi(envelope.payload.get(), &mut || false).unwrap();
    assert_eq!(restored.controls.len(), MAX_ARCHIVE_CONTROLS);
    assert!(restored.paused);
    state.control(Some(SessionOperation::Cancel {}), None, false);
    assert!(
        check_capacity(&state, &mut || false)
            .unwrap_err()
            .starts_with("Application capacity:")
    );
    eprintln!("MSH73 sizes8192-control archive={} bytes", bytes.len());
}

#[test]
fn conservative_reconstruction_budget_is_enforced_before_accepting_a_journal() {
    let mut state =
        ShogiSession::restored(&s::GameRecord::from_game(&s::Game::new()), s::Side::Sente).unwrap();
    // Each historical selection reserves a complete native ledger reconstruction
    // at the measured four-unit native-event weight, even for the cheap root prefix.
    for _ in 0..MAX_REPLAY_ACTIONS / (s::MAX_RECORD_EVENTS * NATIVE_EVENT_WORK) {
        let operation = SessionOperation::Inspect { event_index: 0 };
        state.apply_command(operation.clone()).unwrap();
        state.control(Some(operation), None, false);
    }
    check_capacity(&state, &mut || false).unwrap();
    let accepted = encode(&state, 100).unwrap();
    let envelope: ArchiveV2 = serde_json::from_slice(&accepted).unwrap();
    decode_shogi(envelope.payload.get(), &mut || false).unwrap();
    state.control(
        Some(SessionOperation::Inspect { event_index: 0 }),
        None,
        false,
    );
    let error = check_capacity(&state, &mut || false).unwrap_err();
    assert!(error.starts_with("Application capacity:"));
    assert!(error.contains("replay"));
    assert!(state.controls.len() < MAX_ARCHIVE_CONTROLS);
}

fn long_game() -> s::Game {
    // Same deterministic capacity walk as the already established native record
    // test: legal choices only; no assertion about strength or rule conformance.
    let mut game = s::Game::new();
    let mut seed = 0x67_c5a3_6d91_u64;
    for ply in 1..=320 {
        let choices = game.position().legal_actions();
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let start = (seed % choices.len() as u64) as usize;
        game = (0..choices.len())
            .find_map(|offset| {
                let mut candidate = game.clone();
                candidate
                    .submit(s::GameRequest {
                        expected: candidate.context(),
                        actor: candidate.position().side_to_move(),
                        command: s::GameCommand::Play(choices[(start + offset) % choices.len()]),
                    })
                    .unwrap();
                (ply == 320 || candidate.conclusion().is_none()).then_some(candidate)
            })
            .expect("established deterministic capacity walk has a continuation");
    }
    game
}

#[test]
fn real_maximum_native_ledger_has_bounded_record_context_response_and_archive_sizes() {
    let game = long_game();
    assert_eq!(game.events().len(), 320);
    assert!(game.conclusion().is_some());
    let native = s::GameRecord::from_game(&game);
    let json = native.to_json().unwrap();
    assert!(json.len() < s::MAX_RECORD_BYTES);
    assert_eq!(s::GameRecord::from_json(&json).unwrap(), native);
    let mut oversized: serde_json::Value = serde_json::from_str(&json).unwrap();
    oversized["events"] = serde_json::Value::Array(vec![
        oversized["events"][0].clone();
        s::MAX_RECORD_EVENTS + 1
    ]);
    assert_eq!(
        s::GameRecord::from_json(&oversized.to_string()),
        Err(s::RecordError::Capacity)
    );
    assert_eq!(
        s::GameRecord::from_json(&" ".repeat(s::MAX_RECORD_BYTES + 1)),
        Err(s::RecordError::Capacity)
    );
    let mut c = controller();
    replace(&mut c, &game);
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
        },
    )
    .unwrap();
    let request = c.shared.shogi.as_ref().unwrap().pending.clone().unwrap();
    let response = actual_response(request.clone());
    let request_size = serde_json::to_vec(&request).unwrap().len();
    let response_size = serde_json::to_vec(&response).unwrap().len();
    assert!(request_size < mshogi_engine::MAX_REQUEST_BYTES);
    assert!(response_size < mshogi_engine::MAX_RESPONSE_BYTES);
    assert!(c.accept_shared(response));
    settle(&mut c);
    let state = c.shared.shogi.as_ref().unwrap();
    check_capacity(state, &mut || false).unwrap();
    let bytes = encode(state, c.runtime_floor()).unwrap();
    assert!(bytes.len() < MAX_ARCHIVE_BYTES);
    eprintln!(
        "MSH73 sizes320-action native={} request={request_size} response={response_size} archive={} bytes",
        json.len(),
        bytes.len()
    );
}

#[test]
fn long_history_observations_enforce_both_encoded_byte_and_replay_capacity_without_data_loss() {
    let game = long_game();
    let mut c = controller();
    replace(&mut c, &game);
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap();
    let request = c.shared.shogi.as_ref().unwrap().pending.clone().unwrap();
    let actual = actual_response(request);
    assert!(matches!(
        actual.result,
        SharedResult::Shogi {
            result: ShogiResult::Finished { .. }
        }
    ));
    assert!(c.accept_shared(actual.clone()));
    settle(&mut c);
    let base = c.shared.shogi.as_ref().unwrap().clone();
    // The position is concluded and analysis is read-only. Build a repetitive,
    // valid fixture from the real native result with distinct execution contexts;
    // Save/Open below independently validates every observation and command.
    // Binary search avoids running hundreds of identical320-event searches.
    let build = |count: usize| {
        let mut state = base.clone();
        state.controls.clear();
        state.observations.clear();
        for index in 0..count {
            let mut response = actual.clone();
            let id = 100 + index as u64 * 4;
            response.context.request_id = id;
            response.context.session_version = id + 1;
            response.context.process_generation = id + 2;
            let SharedPurpose::MoveAnalysis { target } = &mut response.context.purpose else {
                unreachable!()
            };
            target.session_version = id + 1;
            target.selection_id = id + 3;
            assert!(response.matches(&response.context));
            state.control(
                Some(SessionOperation::Analyze {
                    operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
                }),
                None,
                false,
            );
            state.observations.push(PresentedObservation {
                response,
                application_error: None,
                historical: false,
            });
            state.control(None, Some(index), false);
        }
        state.execution_floor = 100 + count as u64 * 4;
        state
    };
    let mut byte_low = 0;
    let mut byte_high = MAX_ARCHIVE_OBSERVATIONS;
    assert!(
        check_capacity(&build(byte_high), &mut || false)
            .unwrap_err()
            .contains("16MiB")
    );
    while byte_low + 1 < byte_high {
        let midpoint = (byte_low + byte_high) / 2;
        if encode(&build(midpoint), MAX_SAFE_ID - 17).unwrap().len() <= MAX_ARCHIVE_BYTES {
            byte_low = midpoint;
        } else {
            byte_high = midpoint;
        }
    }
    let rejected_size = encode(&build(byte_high), MAX_SAFE_ID - 17).unwrap().len();
    assert!(rejected_size > MAX_ARCHIVE_BYTES);
    assert!(
        check_capacity(&build(byte_high), &mut || false)
            .unwrap_err()
            .contains("16MiB")
    );
    assert!(encode(&build(byte_low), MAX_SAFE_ID - 17).unwrap().len() <= MAX_ARCHIVE_BYTES);
    // The measured fifteen-second Open regression at205 full-history contexts
    // motivated the smaller independent reconstruction policy. It must pause
    // ordinary work before that structurally byte-saveable document is admitted.
    let mut low = 0;
    let mut high = byte_low;
    assert!(
        check_capacity(&build(high), &mut || false)
            .unwrap_err()
            .contains("replay")
    );
    while low + 1 < high {
        let midpoint = (low + high) / 2;
        match check_capacity(&build(midpoint), &mut || false) {
            Ok(()) => low = midpoint,
            Err(error) => {
                assert!(error.contains("replay"), "{error}");
                high = midpoint;
            }
        }
    }
    assert!(low > 1 && high < byte_low);
    let accepted = build(low);
    assert!(encode(&accepted, MAX_SAFE_ID - 17).unwrap().len() < MAX_ARCHIVE_BYTES);
    let floor = accepted.execution_floor;
    c.shared.shogi = Some(accepted);
    c.session.version = floor + 1;
    c.session.next_request = floor + 1;
    c.session.selection_id = floor + 1;
    c.session.archive_revision = floor + 1;
    let before = record(&c);
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap();
    let request = c.shared.shogi.as_ref().unwrap().pending.clone().unwrap();
    let revision = c.shared_snapshot().document.revision;
    assert!(c.accept_shared(actual_response(request)));
    settle(&mut c);
    let snap = c.shared_snapshot();
    assert_eq!(snap.capacity.status, CapacityPhase::Paused);
    assert!(snap.capacity.message.unwrap().contains("replay"));
    assert_eq!(snap.document.revision, revision);
    assert_eq!(record(&c), before);
    assert_eq!(c.shared.shogi.as_ref().unwrap().observations.len(), low);
    eprintln!("MSH73 replay-capacity candidate rejected, beginning Save/Open");
    let file = path("byte-boundary");
    let id = c.queue_shared_save(file.clone()).unwrap();
    wait_file(&mut c, id).unwrap();
    let size = fs::metadata(&file).unwrap().len();
    let mut reopened = controller();
    let token = reopened.shared_open_token();
    let id = reopened.queue_shared_open(file.clone(), token).unwrap();
    wait_file(&mut reopened, id).unwrap();
    assert_eq!(record(&reopened), record(&c));
    assert_eq!(
        reopened.shared.shogi.as_ref().unwrap().observations.len(),
        low
    );
    eprintln!(
        "MSH73 replay-boundary accepted={low} retained-archive={size} byte-boundary accepted={byte_low} rejected={byte_high} rejected-candidate={rejected_size} bytes"
    );
    fs::remove_file(file).unwrap();
}

#[test]
fn reopened_active_journal_retains_stop_reservation_across_preferences() {
    let mut state =
        ShogiSession::restored(&s::GameRecord::from_game(&s::Game::new()), s::Side::Sente).unwrap();
    for _ in 0..MAX_ARCHIVE_CONTROLS - 2 {
        let command = SessionOperation::Cancel {};
        state.apply_command(command.clone()).unwrap();
        state.control(Some(command), None, false);
    }
    let command = SessionOperation::Resume {};
    state.apply_command(command.clone()).unwrap();
    state.control(Some(command), None, false);
    assert!(needs_stop(&state));
    assert_eq!(state.controls.len(), MAX_ARCHIVE_CONTROLS - 1);
    check_capacity(&state, &mut || false).unwrap();
    let file = path("active-journal-reservation");
    fs::write(&file, encode(&state, 100).unwrap()).unwrap();
    let mut c = controller();
    let token = c.shared_open_token();
    let id = c.queue_shared_open(file.clone(), token).unwrap();
    wait_file(&mut c, id).unwrap();
    let opened = c.shared.shogi.as_ref().unwrap();
    assert!(opened.paused);
    assert!(opened.pending.is_none());
    assert!(opened.durable_needs_stop);
    let mut settings = opened.settings.clone();
    settings.orientation = s::Side::Gote;
    let revision = c.shared_snapshot().document.revision;
    let error = send(&mut c, SessionOperation::ShogiPreferences { settings }).unwrap_err();
    assert!(error.starts_with("Application capacity:"));
    assert_eq!(c.shared_snapshot().document.revision, revision);
    assert_eq!(
        c.shared.shogi.as_ref().unwrap().controls.len(),
        MAX_ARCHIVE_CONTROLS - 1
    );
    assert_eq!(
        c.shared.shogi.as_ref().unwrap().settings.orientation,
        s::Side::Sente
    );
    send(&mut c, SessionOperation::Cancel {}).unwrap();
    let state = c.shared.shogi.as_ref().unwrap();
    assert_eq!(state.controls.len(), MAX_ARCHIVE_CONTROLS);
    assert!(!state.durable_needs_stop);
    let id = c.queue_shared_save(file.clone()).unwrap();
    wait_file(&mut c, id).unwrap();
    let token = c.shared_open_token();
    let id = c.queue_shared_open(file.clone(), token).unwrap();
    wait_file(&mut c, id).unwrap();
    assert_eq!(
        c.shared.shogi.as_ref().unwrap().controls.len(),
        MAX_ARCHIVE_CONTROLS
    );
    fs::remove_file(file).unwrap();
}

#[test]
fn runtime_only_revision_exhaustion_preserves_save_and_fresh_reopen_path() {
    let mut c = controller();
    replace(&mut c, &s::Game::new());
    analyze(&mut c);
    let expected = record(&c);
    c.session.revision = MAX_SAFE_ID - 20;
    let file = path("runtime-revision-growth");
    let id = c.queue_shared_save(file.clone()).unwrap();
    wait_file(&mut c, id).unwrap();
    assert!(!c.shared_snapshot().document.dirty);
    let bytes = fs::read(&file).unwrap();
    let envelope: ArchiveV2 = serde_json::from_slice(&bytes).unwrap();
    let saved: ShogiArchive = serde_json::from_str(envelope.payload.get()).unwrap();
    assert!(saved.retired_execution_floor < MAX_SAFE_ID - 64);
    let mut reopened = controller();
    let token = reopened.shared_open_token();
    let id = reopened.queue_shared_open(file.clone(), token).unwrap();
    wait_file(&mut reopened, id).unwrap();
    assert_eq!(record(&reopened), expected);
    assert!(reopened.shared_snapshot().paused);
    send(&mut reopened, SessionOperation::Resume {}).unwrap();
    assert!(!reopened.shared_snapshot().paused);
    fs::remove_file(file).unwrap();
}

#[test]
fn replay_capacity_retains_one_exact_stop_action_of_headroom() {
    let game = long_game().prefix(319).unwrap();
    let record = s::GameRecord::from_game(&game);
    let mut state = ShogiSession::restored(&record, game.position().side_to_move()).unwrap();
    let base_work =
        (state.initial_record.event_count() + state.game.events().len()) * NATIVE_EVENT_WORK;
    let selections =
        (MAX_REPLAY_ACTIONS - base_work - 3) / (s::MAX_RECORD_EVENTS * NATIVE_EVENT_WORK);
    for _ in 0..selections {
        let command = SessionOperation::Inspect { event_index: 0 };
        state.apply_command(command.clone()).unwrap();
        state.control(Some(command), None, false);
    }
    let command = SessionOperation::ReturnLive {};
    state.apply_command(command.clone()).unwrap();
    state.control(Some(command), None, false);
    let spent = base_work + selections * s::MAX_RECORD_EVENTS * NATIVE_EVENT_WORK + 1;
    for _ in 0..MAX_REPLAY_ACTIONS - spent - 2 {
        let command = SessionOperation::Cancel {};
        state.apply_command(command.clone()).unwrap();
        state.control(Some(command), None, false);
    }
    let command = SessionOperation::Resume {};
    state.apply_command(command.clone()).unwrap();
    state.control(Some(command), None, false);
    assert!(needs_stop(&state));
    check_capacity(&state, &mut || false).unwrap();
    let mut no_headroom = state.clone();
    let command = SessionOperation::ShogiPreferences {
        settings: state.settings.clone(),
    };
    no_headroom.apply_command(command.clone()).unwrap();
    no_headroom.control(Some(command), None, false);
    assert!(
        check_capacity(&no_headroom, &mut || false)
            .unwrap_err()
            .contains("replay")
    );
    state.retire();
    state.paused = true;
    state.control(None, None, true);
    check_capacity(&state, &mut || false).unwrap();
    let encoded = encode(&state, 100).unwrap();
    let envelope: ArchiveV2 = serde_json::from_slice(&encoded).unwrap();
    let (reopened, _) = decode_shogi(envelope.payload.get(), &mut || false).unwrap();
    assert_eq!(reopened.controls.len(), state.controls.len());
    assert_eq!(s::GameRecord::from_game(&reopened.game), record);
    state.control(Some(SessionOperation::Cancel {}), None, false);
    assert!(
        check_capacity(&state, &mut || false)
            .unwrap_err()
            .contains("replay")
    );
}
