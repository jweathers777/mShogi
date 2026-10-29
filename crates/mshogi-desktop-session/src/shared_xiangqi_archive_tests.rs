//! MSH-99 native archive evidence. These tests exercise no product Xiangqi UI.
use super::*;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FILE: AtomicU64 = AtomicU64::new(0);
fn path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "msh99-{}-{}-{label}.mshogi",
        std::process::id(),
        NEXT_FILE.fetch_add(1, Ordering::Relaxed)
    ))
}
fn controller() -> DesktopController {
    let mut c = DesktopController::new("/unused/msh99-engine".into()).unwrap();
    c.shared.manual_engine = true;
    c
}
fn settle(c: &mut DesktopController) {
    let deadline = Instant::now() + std::time::Duration::from_secs(30);
    while c.shared_snapshot().capacity.status == CapacityPhase::Checking {
        assert!(Instant::now() < deadline, "archive admission deadline");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn receipt(c: &mut DesktopController, r: AdmissionReceipt) -> Result<(), String> {
    settle(c);
    r.completion().expect("settled admission receipt")
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<(), String> {
    let game = if c.shared.xiangqi.is_some() {
        GameKind::Xiangqi
    } else if c.shared.shogi.is_some() {
        GameKind::Shogi
    } else {
        GameKind::Chess
    };
    let (_, r) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game,
        operation,
    })?;
    receipt(c, r)
}
fn replace(c: &mut DesktopController, game: &x::Game, human: x::Side) {
    send(
        c,
        SessionOperation::Replace {
            record: CanonicalRecord::xiangqi(x::GameRecord::from_game(game).unwrap()),
            human: NativeSide::Xiangqi(human),
        },
    )
    .unwrap();
}
fn state(c: &DesktopController) -> &XiangqiSession {
    c.shared.xiangqi.as_ref().unwrap()
}
fn canonical(c: &DesktopController) -> x::GameRecord {
    x::GameRecord::from_game(&state(c).game).unwrap()
}
fn wait_file(c: &mut DesktopController, id: u64) -> Result<(), String> {
    let deadline = Instant::now() + std::time::Duration::from_secs(30);
    loop {
        c.shared_snapshot();
        assert!(Instant::now() < deadline, "archive file deadline");
        assert_eq!(c.io_state.id, id);
        match c.io_state.phase {
            IoPhase::Saving | IoPhase::Opening => {
                std::thread::sleep(std::time::Duration::from_millis(1))
            }
            IoPhase::Failed => return Err(c.io_state.message.clone().unwrap()),
            _ => return Ok(()),
        }
    }
}
fn save(c: &mut DesktopController, file: &std::path::Path) {
    let id = c.queue_shared_save(file.to_owned()).unwrap();
    wait_file(c, id).unwrap();
}
fn open(c: &mut DesktopController, file: &std::path::Path) -> Result<(), String> {
    let token = c.shared_open_token();
    let id = c.queue_shared_open(file.to_owned(), token)?;
    wait_file(c, id)
}
fn submit(game: &mut x::Game, actor: x::Side, command: x::GameCommand) {
    game.submit(x::GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .unwrap();
}
fn play(game: &mut x::Game, action: &str) {
    submit(
        game,
        game.position().side_to_move(),
        x::GameCommand::Play {
            action: action.parse().unwrap(),
        },
    );
}
fn root(fen: &str) -> x::Game {
    x::Game::from_root(
        x::PositionRoot::from_fen(fen).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap()
}
fn cycle(id: &str) -> x::Game {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/history-cycles.json"
    ))
    .unwrap();
    let f = fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap();
    let mut g = root(f["root_fen"].as_str().unwrap());
    for action in f["moves"].as_array().unwrap() {
        play(&mut g, action.as_str().unwrap());
    }
    g
}
fn count_game() -> x::Game {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/natural-count.json"
    ))
    .unwrap();
    let f = fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "NATURAL-10-CHECKS")
        .unwrap();
    let mut g = root(f["root_fen"].as_str().unwrap());
    for action in f["moves"].as_array().unwrap().iter().take(100) {
        play(&mut g, action.as_str().unwrap());
    }
    g
}
fn actual(request: &SharedRequest) -> SharedResponse {
    let mut bytes = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(request).unwrap())),
        &mut bytes,
    )
    .unwrap();
    let response: SharedResponse = serde_json::from_slice(&bytes).unwrap();
    assert!(response.matches(request));
    response
}
fn analyze(c: &mut DesktopController) -> SharedResponse {
    send(
        c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap();
    let response = actual(state(c).pending.as_ref().unwrap());
    assert!(c.accept_shared(response.clone()));
    settle(c);
    assert_eq!(
        c.shared_snapshot().capacity.status,
        CapacityPhase::Available
    );
    response
}
fn exact(
    game: &x::Game,
    target: Option<usize>,
    limits: x::ExactLimits,
    cancelled: bool,
) -> x::ExactObservation {
    let mut a = x::ExactAnalysis::new().unwrap();
    if let Some(event_count) = target {
        a.select(x::ExactTarget::History { event_count }).unwrap();
    }
    a.begin(game, limits).unwrap().run(|| cancelled)
}
fn retain(c: &mut DesktopController, observation: &x::ExactObservation) -> Result<(), String> {
    let token = c.shared_token();
    let r = c.retain_xiangqi_exact_evidence(token, observation)?;
    receipt(c, r)
}
fn payload(bytes: &[u8]) -> String {
    serde_json::from_slice::<Value>(bytes).unwrap()["payload"].to_string()
}

#[test]
fn actual_native_ledgers_roundtrip_warning_fourth_claim_resignation_agreement_and_fast_proof() {
    let mut ordinary = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    play(&mut ordinary, "a3a4");
    let warned = cycle("F022-CC");
    assert_eq!(warned.warnings().len(), 1);
    assert!(warned.conclusion().is_none());
    let fourth = cycle("F023-CCC");
    assert!(matches!(
        fourth.conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Black,
            ..
        })
    ));
    let mut count = count_game();
    assert!(count.natural_count_eligibility(x::Side::Red).eligible);
    submit(&mut count, x::Side::Red, x::GameCommand::ClaimNaturalCount);
    assert_eq!(
        count.conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::NaturalCount
        })
    );
    let mut resigned = x::GameRecord::from_game(&ordinary)
        .unwrap()
        .to_game()
        .unwrap();
    submit(&mut resigned, x::Side::Red, x::GameCommand::Resign);
    let mut agreed = x::GameRecord::from_game(&ordinary)
        .unwrap()
        .to_game()
        .unwrap();
    submit(&mut agreed, x::Side::Red, x::GameCommand::OfferDraw);
    submit(&mut agreed, x::Side::Black, x::GameCommand::AcceptDraw);
    let fast = root("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1");
    assert_eq!(
        fast.no_win_assessment(),
        x::NoWinAssessment::ProvenNeitherCanWin
    );
    for (label, game) in [
        ("ordinary", ordinary),
        ("warning", warned),
        ("fourth", fourth),
        ("count", count),
        ("resignation", resigned),
        ("agreement", agreed),
        ("fast", fast),
    ] {
        let mut c = controller();
        replace(&mut c, &game, x::Side::Red);
        let bytes = encode(state(&c), state(&c).execution_floor).unwrap();
        let top: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(top["schema_version"], 2);
        assert_eq!(top["format"], "mshogi.desktop.archive.v2");
        assert_eq!(top["rules_profile"], x::PROFILE_ID);
        let (restored, _) = decode(&payload(&bytes), &mut || false).unwrap();
        assert_eq!(
            x::GameRecord::from_game(&restored.game).unwrap(),
            x::GameRecord::from_game(&game).unwrap(),
            "{label}"
        );
        assert_eq!(restored.game.warnings(), game.warnings(), "{label}");
        assert_eq!(restored.game.conclusion(), game.conclusion(), "{label}");
        assert_eq!(restored.game.root(), game.root(), "{label}");
    }
}

#[test]
fn event_prefix_selection_and_same_board_histories_survive_open_without_future_facts() {
    let warned = cycle("F022-CC");
    let mut c = controller();
    replace(&mut c, &warned, x::Side::Red);
    let live = canonical(&c);
    let warning_index = warned.warnings()[0].event_index;
    send(
        &mut c,
        SessionOperation::Inspect {
            event_index: warning_index,
        },
    )
    .unwrap();
    assert!(state(&c).viewed.as_ref().unwrap().warnings.is_empty());
    assert!(
        state(&c).viewed_game.is_none(),
        "derived event sits inside native command"
    );
    let file = path("historical");
    save(&mut c, &file);
    let old_token = c.shared_token();
    let old_context = state(&c).game.context();
    open(&mut c, &file).unwrap();
    assert_ne!(c.shared_token(), old_token);
    assert_ne!(state(&c).game.context(), old_context);
    assert!(state(&c).paused && state(&c).pending.is_none());
    assert_eq!(
        state(&c).viewed.as_ref().unwrap().event_count,
        warning_index
    );
    assert!(state(&c).viewed.as_ref().unwrap().warnings.is_empty());
    assert_eq!(canonical(&c), live);
    assert!(send(&mut c, SessionOperation::Resume {}).is_err());
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert!(state(&c).viewed.is_none());
    assert_eq!(state(&c).game.warnings().len(), 1);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    assert!(!state(&c).paused);
    std::fs::remove_file(file).unwrap();

    let mut audited = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let plain = x::GameRecord::from_game(&audited).unwrap();
    submit(
        &mut audited,
        x::Side::Black,
        x::GameCommand::ClaimNaturalCount,
    );
    assert_eq!(
        audited.position(),
        &plain.to_game().unwrap().position().clone()
    );
    replace(&mut c, &audited, x::Side::Red);
    let file = path("same-board-audit");
    save(&mut c, &file);
    open(&mut c, &file).unwrap();
    assert_ne!(canonical(&c), plain);
    assert_eq!(canonical(&c).known_board_actions(), 0);
    assert_eq!(canonical(&c).event_count(), 2);
    assert!(
        state(&c)
            .game
            .events()
            .iter()
            .all(|e| e.actor == Some(x::Side::Black))
    );
    std::fs::remove_file(file).unwrap();
}

#[test]
fn saved_actual_engine_and_analysis_observations_are_historical_and_old_work_stays_retired() {
    let mut c = controller();
    let mut game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    play(&mut game, "a3a4");
    replace(&mut c, &game, x::Side::Red);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    let engine = actual(state(&c).pending.as_ref().unwrap());
    assert!(c.accept_shared(engine.clone()));
    settle(&mut c);
    let live = canonical(&c);
    let analysis = analyze(&mut c);
    assert_eq!(canonical(&c), live);
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
    let old = c.shared_token();
    let floor = c.session.next_request;
    let file = path("observations");
    save(&mut c, &file);
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap();
    let unaccepted = actual(state(&c).pending.as_ref().unwrap());
    open(&mut c, &file).unwrap();
    assert_ne!(c.shared_token(), old);
    assert!(c.session.next_request > floor);
    assert_eq!(canonical(&c), live);
    assert_eq!(state(&c).settings, settings);
    assert_eq!(state(&c).observations.len(), 2);
    assert!(state(&c).observations.iter().all(|o| o.historical));
    assert_eq!(state(&c).observations[0].response, engine);
    assert_eq!(state(&c).observations[1].response, analysis);
    assert!(state(&c).paused && state(&c).pending.is_none() && state(&c).analysis.is_none());
    let reopened = canonical(&c);
    assert!(
        !c.accept_shared(unaccepted),
        "work pending before Open must stay retired"
    );
    assert!(!c.accept_shared(engine));
    assert!(!c.accept_shared(analysis));
    assert_eq!(canonical(&c), reopened);
    send(&mut c, SessionOperation::Resume {}).unwrap();
    assert!(!state(&c).paused);
    std::fs::remove_file(file).unwrap();
}

#[test]
fn completed_exact_proof_witness_unknown_cancel_failure_remain_read_only_historical_reports() {
    let bounds = x::ExactLimits {
        states: 5000,
        transitions: 50_000,
        millis: 60_000,
        traversal: x::ExactTraversal::BreadthFirst,
        ..x::ExactLimits::default()
    };
    let positive = root("5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1");
    let negative = root("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1");
    let standard = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let cases = [
        ("positive", &positive, exact(&positive, None, bounds, false)),
        ("negative", &negative, exact(&negative, None, bounds, false)),
        (
            "unknown",
            &standard,
            exact(
                &standard,
                None,
                x::ExactLimits {
                    states: 0,
                    ..bounds
                },
                false,
            ),
        ),
        ("cancelled", &standard, exact(&standard, None, bounds, true)),
        (
            "failure",
            &standard,
            exact(
                &standard,
                None,
                x::ExactLimits {
                    states: x::MAX_EXACT_STATES + 1,
                    ..bounds
                },
                false,
            ),
        ),
    ];
    for (label, game, observation) in cases {
        match label {
            "positive" => assert!(
                matches!(observation.report().verdict, x::ExactVerdict::NaturalWin { ref witness, .. } if witness.len() == 2)
            ),
            "negative" => assert_eq!(observation.report().verdict, x::ExactVerdict::NeitherCanWin),
            "unknown" => assert_eq!(
                observation.report().verdict,
                x::ExactVerdict::Unknown {
                    reason: x::ExactStop::States
                }
            ),
            "cancelled" => assert_eq!(
                observation.report().verdict,
                x::ExactVerdict::Unknown {
                    reason: x::ExactStop::Cancelled
                }
            ),
            "failure" => assert_eq!(
                observation.report().verdict,
                x::ExactVerdict::Failure {
                    reason: x::ExactFailure::UnsupportedLimits
                }
            ),
            _ => unreachable!(),
        }
        let mut c = controller();
        replace(&mut c, game, x::Side::Red);
        let before = canonical(&c);
        retain(&mut c, &observation).unwrap();
        assert_eq!(canonical(&c), before, "{label} changed game");
        let file = path(label);
        save(&mut c, &file);
        open(&mut c, &file).unwrap();
        assert_eq!(canonical(&c), before, "{label} reopened as outcome");
        assert_eq!(state(&c).exact_observations.len(), 1);
        assert_eq!(
            state(&c).exact_observations[0].report(),
            observation.report()
        );
        assert_eq!(
            state(&c).exact_observations[0].source(),
            &CanonicalRecord::xiangqi(observation.request().source_record().clone())
        );
        assert!(state(&c).pending.is_none() && state(&c).paused);
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn exact_historical_event_target_and_stale_attachment_are_preserved() {
    let game = cycle("F022-CC");
    let target = game.warnings()[0].event_index;
    let observation = exact(
        &game,
        Some(target),
        x::ExactLimits {
            states: 0,
            ..x::ExactLimits::default()
        },
        false,
    );
    let mut c = controller();
    replace(&mut c, &game, x::Side::Red);
    let stale = c.shared_token();
    send(
        &mut c,
        SessionOperation::Inspect {
            event_index: target,
        },
    )
    .unwrap();
    assert!(
        c.retain_xiangqi_exact_evidence(stale, &observation)
            .is_err()
    );
    retain(&mut c, &observation).unwrap();
    let file = path("exact-history");
    save(&mut c, &file);
    open(&mut c, &file).unwrap();
    assert_eq!(state(&c).exact_observations[0].event_count(), target);
    assert_eq!(state(&c).viewed.as_ref().unwrap().event_count, target);
    assert!(state(&c).viewed.as_ref().unwrap().warnings.is_empty());
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert_eq!(state(&c).game.warnings().len(), 1);
    assert_eq!(state(&c).exact_observations[0].event_count(), target);
    std::fs::remove_file(file).unwrap();
}

#[test]
fn strict_duplicates_conflicts_derived_conclusions_and_journal_tampering_are_rejected() {
    let game = cycle("F022-CC");
    let mut c = controller();
    replace(&mut c, &game, x::Side::Red);
    analyze(&mut c);
    let text = payload(&encode(state(&c), state(&c).execution_floor).unwrap());
    let base: Value = serde_json::from_str(&text).unwrap();
    for (key, value) in [
        ("human", json!({"game_kind":"chess","side":"white"})),
        ("retired_execution_floor", json!(MAX_SAFE_ID)),
        ("unsupported", json!(true)),
    ] {
        let mut corrupt = base.clone();
        corrupt[key] = value;
        assert!(
            decode(&corrupt.to_string(), &mut || false).is_err(),
            "accepted{key}"
        );
    }
    for engine_protocol in [json!(0), json!(2), json!(999)] {
        let mut corrupt = base.clone();
        corrupt["engine_protocol"] = engine_protocol;
        assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    }
    for field in ["request_id", "session_version", "process_generation"] {
        let mut corrupt = base.clone();
        let current = corrupt["observations"][0]["response"]["context"][field]
            .as_u64()
            .unwrap();
        corrupt["observations"][0]["response"]["context"][field] = json!(current + 1);
        assert!(
            decode(&corrupt.to_string(), &mut || false).is_err(),
            "{field}"
        );
    }
    let mut corrupt = base.clone();
    corrupt["observations"][0]["response"]["context"]["purpose"] = json!({"kind":"engine_turn"});
    assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    let mut corrupt = base.clone();
    corrupt["observations"][0]["response"]["context"]["limits"]["nodes"] = json!(2);
    assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    let mut corrupt = base.clone();
    corrupt["current"]["payload"]["conclusion"] = json!({"kind":"draw","reason":"agreement"});
    assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    let mut corrupt = base.clone();
    corrupt["current"]["rules_profile"] = json!("mshogi.shogi.csa36.v1");
    assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    let mut corrupt = base.clone();
    corrupt["observations"][0]["response"]["context"]["purpose"]["target"]["event_index"] =
        json!(0);
    assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    let mut corrupt = base.clone();
    corrupt["controls"][0]["native_events_before"] = json!(0);
    assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    let mut missing = base.clone();
    missing
        .as_object_mut()
        .unwrap()
        .remove("exact_observations");
    assert!(decode(&missing.to_string(), &mut || false).is_err());
    for native_field in ["events", "commands"] {
        let mut excessive = base.clone();
        let bound = if native_field == "events" {
            x::MAX_GAME_EVENTS
        } else {
            x::MAX_GAME_COMMANDS
        };
        excessive["current"]["payload"][native_field] = Value::Array(vec![json!({}); bound + 1]);
        let failure = decode(&excessive.to_string(), &mut || false)
            .err()
            .expect("native resource refusal");
        assert!(
            failure.starts_with("Application capacity:"),
            "{native_field}: {failure}"
        );
    }
    for needle in ["\"human\":", "\"no_win_mode\":", "\"origin\":"] {
        let field = match needle {
            "\"human\":" => "\"human\":null,\"human\":",
            "\"no_win_mode\":" => "\"no_win_mode\":\"sound_fast_only\",\"no_win_mode\":",
            _ => "\"origin\":\"analysis_diagram\",\"origin\":",
        };
        let dup = text.replacen(needle, field, 1);
        assert_ne!(dup, text);
        assert!(
            decode(&dup, &mut || false).is_err(),
            "duplicate{needle} lost"
        );
    }
}

#[test]
fn reused_validation_never_hides_changed_history_or_malformed_later_observations() {
    let game = cycle("F022-CC");
    let mut c = controller();
    replace(&mut c, &game, x::Side::Red);
    for _ in 0..2 {
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: 1,
                    nodes: 128,
                },
            },
        )
        .unwrap();
        let response = actual(state(&c).pending.as_ref().unwrap());
        assert!(
            matches!(&response.result, SharedResult::Xiangqi { result: XiangqiResult::Choice { telemetry, .. } } if !telemetry.principal_variation.is_empty())
        );
        assert!(c.accept_shared(response));
        settle(&mut c);
    }
    let observation = exact(
        &game,
        None,
        x::ExactLimits {
            states: 0,
            ..x::ExactLimits::default()
        },
        false,
    );
    retain(&mut c, &observation).unwrap();
    let base: Value = serde_json::from_str(&payload(
        &encode(state(&c), state(&c).execution_floor).unwrap(),
    ))
    .unwrap();
    let text = base.to_string();
    let (restored, _) = decode(&text, &mut || false).unwrap();
    assert_eq!(restored.observations.len(), 2);
    assert_eq!(restored.exact_observations, state(&c).exact_observations);
    assert_eq!(
        restored.observations[1].response,
        state(&c).observations[1].response
    );

    // Earlier records and observations have populated validation reuse by now.
    // Alter only a later source: duplicate native/wrapper fields must still reach
    // the strict parser, and surrounding observation/request fields stay strict.
    for (collection, index) in [("observations", 1), ("exact_observations", 0)] {
        let original = base[collection][index].to_string();
        for (needle, replacement) in [
            (
                "\"no_win_mode\":",
                "\"no_win_mode\":\"sound_fast_only\",\"no_win_mode\":",
            ),
            (
                "\"native_schema_version\":",
                "\"native_schema_version\":1,\"native_schema_version\":",
            ),
        ] {
            let changed = original.replacen(needle, replacement, 1);
            assert_ne!(original, changed);
            assert!(decode(&text.replacen(&original, &changed, 1), &mut || false).is_err());
        }
        let mut corrupt = base.clone();
        corrupt[collection][index]["unexpected"] = json!(true);
        assert!(decode(&corrupt.to_string(), &mut || false).is_err());
    }
    let mut missing = base.clone();
    missing["observations"][1]
        .as_object_mut()
        .unwrap()
        .remove("application_error");
    assert!(decode(&missing.to_string(), &mut || false).is_err());
    let mut duplicate = base.clone();
    duplicate["observations"][1]["response"]["context"]["unexpected"] = json!(true);
    assert!(decode(&duplicate.to_string(), &mut || false).is_err());

    let prefix = game.committed_prefix(0).unwrap();
    assert_eq!(
        prefix.position().pieces().collect::<Vec<_>>(),
        game.position().pieces().collect::<Vec<_>>()
    );
    let mut wrong_history = base.clone();
    wrong_history["observations"][1]["response"]["context"]["game"] = serde_json::to_value(
        CanonicalRecord::xiangqi(x::GameRecord::from_game(&prefix).unwrap()),
    )
    .unwrap();
    assert!(decode(&wrong_history.to_string(), &mut || false).is_err());
    let mut changed_variation = base.clone();
    changed_variation["observations"][1]["response"]["result"]["result"]["telemetry"]["principal_variation"]
        .as_array_mut().unwrap().push(serde_json::to_value(XiangqiChoice::Move { action: "a0a9".parse().unwrap() }).unwrap());
    assert!(
        decode(&changed_variation.to_string(), &mut || false).is_err(),
        "a new illegal variation cannot reuse the earlier valid PV check"
    );
}

#[test]
fn failed_and_interrupted_save_open_preserve_file_dirty_state_and_live_document() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    let file = path("atomic");
    save(&mut c, &file);
    let prior = std::fs::read(&file).unwrap();
    send(
        &mut c,
        SessionOperation::XiangqiPreferences {
            settings: XiangqiSettings {
                orientation: x::Side::Black,
                ..XiangqiSettings::default()
            },
        },
    )
    .unwrap();
    assert!(c.shared_snapshot().document.dirty);
    let before = canonical(&c);
    let token = c.shared_token();
    let mut checks = 0;
    let err = persistence::atomic_write(&file, &vec![b'x'; 20_000], &mut || {
        checks += 1;
        checks >= 2
    });
    assert!(err.is_err());
    assert_eq!(std::fs::read(&file).unwrap(), prior);
    assert!(c.shared_snapshot().document.dirty);
    let bad_parent = path("missing-parent").join("no-file");
    let id = c.queue_shared_save(bad_parent).unwrap();
    assert!(wait_file(&mut c, id).is_err());
    assert!(c.shared_snapshot().document.dirty);
    assert_eq!(std::fs::read(&file).unwrap(), prior);
    let malformed = path("malformed");
    let mut oversized = prior.clone();
    oversized.resize(MAX_ARCHIVE_BYTES + 1, b' ');
    let mut wrong: Value = serde_json::from_slice(&prior).unwrap();
    wrong["rules_profile"] = json!("wrong");
    let mut unsupported: Value = serde_json::from_slice(&prior).unwrap();
    unsupported["schema_version"] = json!(999);
    for bytes in [
        b"{broken".to_vec(),
        oversized,
        wrong.to_string().into_bytes(),
        unsupported.to_string().into_bytes(),
    ] {
        std::fs::write(&malformed, bytes).unwrap();
        assert!(open(&mut c, &malformed).is_err());
        assert_eq!(canonical(&c), before);
        let after = c.shared_token();
        assert_eq!(
            (after.document, after.version, after.selection),
            (token.document, token.version, token.selection)
        );
        assert!(
            after.revision >= token.revision,
            "I/O progress may advance snapshot revision"
        );
        assert!(c.shared_snapshot().document.dirty);
    }
    assert!(decode(&payload(&prior), &mut || true).is_err());
    assert_eq!(canonical(&c), before);
    assert!(c.shared_snapshot().document.dirty);
    std::fs::remove_file(file).unwrap();
    std::fs::remove_file(malformed).unwrap();
}

#[test]
fn product_open_restores_xiangqi_with_fresh_stopped_identity_and_keeps_stale_failures_atomic() {
    let mut xq = controller();
    replace(
        &mut xq,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    send(&mut xq, SessionOperation::Inspect { event_index: 0 }).unwrap();
    let file = path("product-open");
    save(&mut xq, &file);
    let mut chess = controller();
    let before = chess.shared_snapshot().token;
    let id = chess
        .queue_product_open(file.clone(), chess.shared_open_token())
        .unwrap();
    wait_file(&mut chess, id).unwrap();
    let opened = chess.shared_snapshot();
    assert_eq!(opened.context, xq.shared_snapshot().context);
    assert_eq!(opened.game, GameKind::Xiangqi);
    assert!(opened.paused && opened.historical);
    assert!(opened.execution_purpose.is_none());
    assert!(opened.token.version > before.version);
    assert!(opened.token.document > before.document);
    assert!(!opened.document.dirty);
    let stale = xq.shared_open_token();
    send(
        &mut xq,
        SessionOperation::XiangqiPreferences {
            settings: XiangqiSettings {
                orientation: x::Side::Black,
                ..XiangqiSettings::default()
            },
        },
    )
    .unwrap();
    assert!(xq.queue_shared_open(file.clone(), stale).is_err());
    let before = canonical(&xq);
    let count = state(&xq).observations.len();
    send(
        &mut xq,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap();
    let request = state(&xq).pending.as_ref().unwrap().clone();
    let response = SharedResponse {
        context: request,
        result: SharedResult::Failure {
            failure: mshogi_engine::EngineFailure {
                kind: mshogi_engine::FailureKind::MalformedResponse,
                message: "x".repeat(mshogi_engine::MAX_RESPONSE_BYTES + 1),
            },
        },
    };
    assert!(!xq.accept_shared(response));
    assert_eq!(canonical(&xq), before);
    assert_eq!(state(&xq).observations.len(), count);
    save(&mut xq, &file);
    std::fs::remove_file(file).unwrap();
}

#[test]
fn ordinary_observation_256_to_257_rejects_new_work_and_still_saves_reopens_all_evidence() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    let mut template = None;
    for index in 0..MAX_ARCHIVE_OBSERVATIONS {
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: PLAY_LIMITS.depth,
                    nodes: PLAY_LIMITS.nodes,
                },
            },
        )
        .unwrap();
        let context = state(&c).pending.clone().unwrap();
        // Repeat a real completed native result at the identical history under
        // fresh transport identities. This is fixture replay, not256 searches.
        let result = template
            .get_or_insert_with(|| actual(&context).result)
            .clone();
        assert!(
            matches!(&result, SharedResult::Xiangqi { result: XiangqiResult::Choice { telemetry, .. } } if !telemetry.principal_variation.is_empty())
        );
        assert!(c.accept_shared(SharedResponse { context, result }));
        settle(&mut c);
        assert_eq!(state(&c).observations.len(), index + 1);
    }
    assert_eq!(state(&c).observations.len(), 256);
    let before = canonical(&c);
    let revision = c.session.document_revision;
    let err = send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap_err();
    assert!(err.starts_with("Application capacity:"));
    assert_eq!(canonical(&c), before);
    assert_eq!(state(&c).observations.len(), 256);
    assert_eq!(c.session.document_revision, revision);
    assert!(state(&c).pending.is_none());
    let file = path("obs256");
    save(&mut c, &file);
    let began = Instant::now();
    open(&mut c, &file).unwrap();
    println!(
        "256 ordinary observations: {} encoded bytes, reopen {}ms",
        std::fs::metadata(&file).unwrap().len(),
        began.elapsed().as_millis()
    );
    assert_eq!(state(&c).observations.len(), 256);
    assert!(state(&c).observations.iter().all(|o| o.historical));
    assert!(
        send(&mut c, SessionOperation::Resume {})
            .unwrap_err()
            .starts_with("Application capacity:")
    );
    save(&mut c, &file);
    std::fs::remove_file(file).unwrap();
}

#[test]
fn ordinary_and_exact_observations_share_capacity_without_discarding_either_kind() {
    let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let mut c = controller();
    replace(&mut c, &game, x::Side::Red);
    analyze(&mut c);
    let mut a = x::ExactAnalysis::new().unwrap();
    for _ in 1..MAX_ARCHIVE_OBSERVATIONS {
        let o = a
            .begin(
                &game,
                x::ExactLimits {
                    states: 0,
                    ..x::ExactLimits::default()
                },
            )
            .unwrap()
            .run(|| false);
        retain(&mut c, &o).unwrap();
    }
    let extra = a
        .begin(
            &game,
            x::ExactLimits {
                states: 0,
                ..x::ExactLimits::default()
            },
        )
        .unwrap()
        .run(|| false);
    let revision = c.session.document_revision;
    assert!(
        retain(&mut c, &extra)
            .unwrap_err()
            .starts_with("Application capacity:")
    );
    assert_eq!(state(&c).observations.len(), 1);
    assert_eq!(state(&c).exact_observations.len(), 255);
    assert_eq!(c.session.document_revision, revision);
    let file = path("mixed256");
    save(&mut c, &file);
    let began = Instant::now();
    open(&mut c, &file).unwrap();
    println!(
        "1 ordinary+255 exact observations: {} encoded bytes, reopen {}ms",
        std::fs::metadata(&file).unwrap().len(),
        began.elapsed().as_millis()
    );
    assert_eq!(state(&c).observations.len(), 1);
    assert_eq!(state(&c).exact_observations.len(), 255);
    std::fs::remove_file(file).unwrap();
}

#[test]
fn distinct_native_variations_pay_reconstruction_before_result_commit_and_still_reopen() {
    let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let mut c = controller();
    replace(&mut c, &game, x::Side::Red);
    let mut template = None;
    let mut refused = None;
    for action in game.position().legal_actions().unwrap() {
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: 1,
                    nodes: 128,
                },
            },
        )
        .unwrap();
        let context = state(&c).pending.clone().unwrap();
        let mut result = template
            .get_or_insert_with(|| actual(&context).result)
            .clone();
        let SharedResult::Xiangqi {
            result: XiangqiResult::Choice { choice, telemetry },
        } = &mut result
        else {
            panic!("scored native template")
        };
        // Native legal-proposal stress fixtures, not claimed engine searches or
        // evaluation facts for the substituted alternatives.
        *choice = XiangqiChoice::Move { action };
        telemetry.principal_variation = vec![*choice];
        let response = SharedResponse { context, result };
        assert!(response.matches(&response.context));
        let count = state(&c).observations.len();
        let before = canonical(&c);
        assert!(c.accept_shared(response.clone()));
        settle(&mut c);
        assert_eq!(canonical(&c), before);
        if state(&c).observations.len() == count {
            assert_eq!(c.shared_snapshot().capacity.status, CapacityPhase::Paused);
            assert!(
                c.shared_snapshot()
                    .capacity
                    .message
                    .unwrap()
                    .contains("Application capacity:")
            );
            refused = Some(response);
            break;
        }
        assert!(
            state(&c)
                .observations
                .last()
                .unwrap()
                .application_error
                .is_none()
        );
    }
    // Independent policy arithmetic:29 * (512+16 native validation +8
    // observation +2 controls)=15,602;30 costs16,140 before any stop reserve.
    assert_eq!(state(&c).observations.len(), 29);
    let refused = refused.expect("30th distinct variation exceeds replay capacity");
    let expected: Vec<_> = state(&c)
        .observations
        .iter()
        .map(|o| o.response.clone())
        .collect();

    // Test-only candidate bypass demonstrates that raw Open preflight rejects
    // the exact same unsupported work, before native replay; live state is intact.
    let mut beyond = XiangqiDraft::capture(state(&c)).materialize().unwrap();
    beyond.analysis_intent = Some((refused.context.game.clone(), refused.context.limits));
    let binding = ExecutionBinding::of(&refused.context);
    let before = (beyond.game.events().len(), beyond.board_actions());
    beyond.apply_response(refused).unwrap();
    beyond.journal(before, None, Some(29), None, false);
    beyond.controls.last_mut().unwrap().execution = Some(binding);
    let unsupported = payload(&encode(&beyond, MAX_SAFE_ID - 128).unwrap());
    assert!(
        decode(&unsupported, &mut || false)
            .err()
            .unwrap()
            .starts_with("Application capacity:")
    );

    let file = path("distinct-pvs");
    save(&mut c, &file);
    let began = Instant::now();
    open(&mut c, &file).unwrap();
    println!(
        "29 distinct native variations: {} bytes, reopen {}ms",
        std::fs::metadata(&file).unwrap().len(),
        began.elapsed().as_millis()
    );
    assert_eq!(
        state(&c)
            .observations
            .iter()
            .map(|o| o.response.clone())
            .collect::<Vec<_>>(),
        expected
    );
    assert!(state(&c).pending.is_none() && state(&c).paused);
    std::fs::remove_file(file).unwrap();
}

#[test]
fn positive_exact_witness_replay_is_charged_before_attachment_and_raw_open() {
    let game = root("5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1");
    let observation = exact(
        &game,
        None,
        x::ExactLimits {
            states: 5000,
            transitions: 50_000,
            millis: 60_000,
            traversal: x::ExactTraversal::BreadthFirst,
            ..x::ExactLimits::default()
        },
        false,
    );
    assert!(
        matches!(&observation.report().verdict, x::ExactVerdict::NaturalWin { witness, .. } if witness.len()==2)
    );
    let mut c = controller();
    replace(&mut c, &game, x::Side::Red);
    let evidence = XiangqiExactEvidence::from_native(&observation);
    let mut checks = 0;
    let cancelled = evidence.validate(state(&c), &mut || {
        checks += 1;
        checks >= 2
    });
    assert!(cancelled.unwrap_err().contains("cancelled"));
    let mut too_long = evidence.clone();
    let x::ExactVerdict::NaturalWin { witness, .. } = &mut too_long.report.verdict else {
        unreachable!()
    };
    // Structural resource guard before any attempt to replay a forged long path.
    witness.resize(243, witness[0]);
    assert!(
        too_long
            .validate(state(&c), &mut || false)
            .unwrap_err()
            .starts_with("Application capacity:")
    );
    for _ in 0..24 {
        retain(&mut c, &observation).unwrap();
    }
    let before = canonical(&c);
    let revision = c.session.document_revision;
    //24*(512 reconstruction+64*2 witness actions+8 observation+1control)=15,576.
    assert!(
        retain(&mut c, &observation)
            .unwrap_err()
            .starts_with("Application capacity:")
    );
    assert_eq!(state(&c).exact_observations.len(), 24);
    assert_eq!(c.session.document_revision, revision);
    assert_eq!(canonical(&c), before);
    let mut beyond = XiangqiDraft::capture(state(&c)).materialize().unwrap();
    beyond
        .exact_observations
        .push(XiangqiExactEvidence::from_native(&observation));
    beyond.journal(
        (beyond.game.events().len(), beyond.board_actions()),
        None,
        None,
        Some(24),
        false,
    );
    let unsupported = payload(&encode(&beyond, MAX_SAFE_ID - 128).unwrap());
    assert!(
        decode(&unsupported, &mut || false)
            .err()
            .unwrap()
            .starts_with("Application capacity:")
    );
    let file = path("positive-witness-capacity");
    save(&mut c, &file);
    let began = Instant::now();
    open(&mut c, &file).unwrap();
    println!(
        "24 exact positive witnesses: {} bytes, reopen {}ms",
        std::fs::metadata(&file).unwrap().len(),
        began.elapsed().as_millis()
    );
    assert_eq!(state(&c).exact_observations.len(), 24);
    assert_eq!(canonical(&c), before);
    assert!(
        state(&c)
            .exact_observations
            .iter()
            .all(|e| e.report() == observation.report())
    );
    std::fs::remove_file(file).unwrap();
}

#[test]
fn control_replay_and_native_count_boundaries_reserve_complete_stop_and_terminal_paths() {
    let mut s = XiangqiDraft::replacement(
        &x::GameRecord::from_game(&x::Game::standard(x::ClaimPolicy::Explicit).unwrap()).unwrap(),
        x::Side::Red,
    )
    .materialize()
    .unwrap();
    s.apply_command(SessionOperation::Resume {}).unwrap();
    s.journal((0, 0), Some(SessionOperation::Resume {}), None, None, false);
    while s.controls.len() < MAX_ARCHIVE_CONTROLS - 1 {
        let operation = SessionOperation::XiangqiPreferences {
            settings: XiangqiSettings::default(),
        };
        s.apply_command(operation.clone()).unwrap();
        s.journal((0, 0), Some(operation), None, None, false);
    }
    check_capacity(&s, &mut || false).unwrap();
    let last = s.controls.last().unwrap().clone();
    s.controls.push(last);
    assert!(
        check_capacity(&s, &mut || false)
            .unwrap_err()
            .contains("stop/close")
    );
    s.controls.pop();
    assert!(s.close_pause());
    assert_eq!(s.controls.len(), MAX_ARCHIVE_CONTROLS);
    check_capacity(&s, &mut || false).unwrap();
    assert!(!s.close_pause());
    let bytes = encode(&s, 1).unwrap();
    decode(&payload(&bytes), &mut || false).unwrap();

    let mut g = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    for _ in 0..x::MAX_GAME_COMMANDS - 1 {
        submit(&mut g, x::Side::Red, x::GameCommand::ClaimNaturalCount);
    }
    let before = x::GameRecord::from_game(&g).unwrap();
    assert!(
        g.submit(x::GameRequest {
            expected: g.context(),
            actor: x::Side::Black,
            command: x::GameCommand::ClaimNaturalCount
        })
        .is_err()
    );
    assert_eq!(x::GameRecord::from_game(&g).unwrap(), before);
    submit(&mut g, x::Side::Black, x::GameCommand::Resign);
    let mut c = controller();
    replace(&mut c, &g, x::Side::Red);
    let bytes = encode(state(&c), state(&c).execution_floor).unwrap();
    decode(&payload(&bytes), &mut || false).unwrap();
}

#[test]
fn exact_weighted_replay_threshold_reserves_one_real_close_control() {
    let game = count_game();
    let record = x::GameRecord::from_game(&game).unwrap();
    let events = record.event_count();
    assert_eq!(events, 100);
    let mut s = XiangqiDraft::replacement(&record, x::Side::Red)
        .materialize()
        .unwrap();
    // This frozen fixture has100 events and100 board actions. Each full
    // native reconstruction costs100*4+100*12=1600; same-ply audits are
    // deliberately distinct from the more expensive board transitions.
    for _ in 0..7 {
        let op = SessionOperation::Inspect { event_index: 0 };
        s.apply_command(op.clone()).unwrap();
        s.journal((events, 100), Some(op), None, None, false);
    }
    for op in [SessionOperation::ReturnLive {}, SessionOperation::Resume {}] {
        s.apply_command(op.clone()).unwrap();
        s.journal((events, 100), Some(op), None, None, false);
    }
    for _ in 0..1597 {
        let op = SessionOperation::XiangqiPreferences {
            settings: XiangqiSettings::default(),
        };
        s.apply_command(op.clone()).unwrap();
        s.journal((events, 100), Some(op), None, None, false);
    }
    assert!(needs_stop(&s));
    assert_eq!(
        2 * (100 * 4 + 100 * 12) + 7 * (100 * 4 + 100 * 12) + 2 + 1597 + 1,
        16_000
    );
    check_capacity(&s, &mut || false).unwrap();
    let op = SessionOperation::XiangqiPreferences {
        settings: XiangqiSettings::default(),
    };
    s.journal((events, 100), Some(op), None, None, false);
    assert!(
        check_capacity(&s, &mut || false)
            .unwrap_err()
            .contains("replay")
    );
    s.controls.pop();
    assert!(s.close_pause());
    check_capacity(&s, &mut || false).unwrap();
    let bytes = encode(&s, 1).unwrap();
    let (restored, _) = decode(&payload(&bytes), &mut || false).unwrap();
    assert_eq!(restored.controls.len(), s.controls.len());
    assert!(!needs_stop(&restored));
}

#[test]
fn actual_encoded_open_byte_boundary_and_structural_candidate_byte_guard_are_exact() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    let mut bytes = encode(state(&c), state(&c).execution_floor).unwrap();
    // Whitespace is legal JSON. This tests the actual file-byte boundary without
    // inventing accepted history merely to defeat the smaller replay budget.
    bytes.resize(MAX_ARCHIVE_BYTES, b' ');
    let file = path("exact-byte-limit");
    std::fs::write(&file, &bytes).unwrap();
    open(&mut c, &file).unwrap();
    let before = canonical(&c);
    let token = c.shared_open_token();
    bytes.push(b' ');
    std::fs::write(&file, &bytes).unwrap();
    assert!(
        open(&mut c, &file)
            .unwrap_err()
            .starts_with("Application capacity:")
    );
    assert_eq!(canonical(&c), before);
    assert_eq!(c.shared_open_token(), token);
    std::fs::remove_file(file).unwrap();

    // Isolate the byte counter with a deliberately malformed, unaccepted large
    // diagnostic. Normal admission rejects it at the transport/parser boundary;
    // these values must never be represented as accepted native observations.
    let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let mut s = XiangqiDraft::replacement(&x::GameRecord::from_game(&game).unwrap(), x::Side::Red)
        .materialize()
        .unwrap();
    let context = SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
        request_id: 1,
        session_version: 1,
        process_generation: 1,
        game: CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap()),
        limits: PLAY_LIMITS,
    };
    s.observations.push(PresentedObservation {
        response: SharedResponse {
            context,
            result: SharedResult::Failure {
                failure: mshogi_engine::EngineFailure {
                    kind: mshogi_engine::FailureKind::Start,
                    message: String::new(),
                },
            },
        },
        application_error: None,
        historical: false,
    });
    let base_size = encode(&s, MAX_SAFE_ID - 17).unwrap().len();
    let SharedResult::Failure { failure } = &mut s.observations[0].response.result else {
        unreachable!()
    };
    failure.message = "x".repeat(MAX_ARCHIVE_BYTES - base_size);
    assert_eq!(
        encode(&s, MAX_SAFE_ID - 17).unwrap().len(),
        MAX_ARCHIVE_BYTES
    );
    check_capacity(&s, &mut || false).unwrap();
    let SharedResult::Failure { failure } = &mut s.observations[0].response.result else {
        unreachable!()
    };
    failure.message.push('x');
    assert_eq!(
        encode(&s, MAX_SAFE_ID - 17).unwrap().len(),
        MAX_ARCHIVE_BYTES + 1
    );
    assert!(
        check_capacity(&s, &mut || false)
            .unwrap_err()
            .contains("16MiB")
    );
}

#[test]
fn measured_long_warning_claim_and_historical_observations_pause_before_replay_budget() {
    let mut warning = cycle("F022-CC");
    // Same-board rejected claim audits retain the independently frozen warning.
    for _ in 0..40 {
        submit(
            &mut warning,
            x::Side::Red,
            x::GameCommand::ClaimNaturalCount,
        );
    }
    assert_eq!(warning.warnings().len(), 1);
    let mut claim = count_game();
    submit(&mut claim, x::Side::Red, x::GameCommand::ClaimNaturalCount);
    for (label, game) in [
        ("warning-and-audits", warning),
        ("accepted-natural-count", claim),
    ] {
        let mut c = controller();
        replace(&mut c, &game, x::Side::Red);
        send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        let historical = analyze(&mut c);
        assert_eq!(historical.context.game.event_count(), 0);
        send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        let mut accepted = 1;
        loop {
            let command = SessionOperation::Analyze {
                operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
            };
            match send(&mut c, command) {
                Ok(()) => {
                    let response = actual(state(&c).pending.as_ref().unwrap());
                    assert!(c.accept_shared(response));
                    settle(&mut c);
                    if c.shared_snapshot().capacity.status == CapacityPhase::Paused {
                        break;
                    }
                    accepted += 1;
                }
                Err(e) => {
                    assert!(e.starts_with("Application capacity:"), "{e}");
                    break;
                }
            }
            assert!(accepted < 256);
        }
        assert!(accepted > 1);
        assert_eq!(state(&c).observations.len(), accepted);
        let before = canonical(&c);
        let file = path("measured");
        eprintln!(
            "MSH99 {label}: saving{} observations,{} controls,{} bytes",
            accepted,
            state(&c).controls.len(),
            encode(state(&c), state(&c).execution_floor).unwrap().len()
        );
        save(&mut c, &file);
        let bytes = std::fs::metadata(&file).unwrap().len();
        let started = Instant::now();
        eprintln!("MSH99 {label}: opening accepted complete archive");
        open(&mut c, &file).unwrap();
        let elapsed = started.elapsed();
        assert_eq!(canonical(&c), before);
        assert_eq!(state(&c).observations.len(), accepted);
        assert_eq!(state(&c).observations[0].response, historical);
        assert_eq!(state(&c).game.warnings(), game.warnings());
        eprintln!(
            "MSH99 measured {label} native-events={} controls={} observations={} encoded-bytes={bytes} reopen-millis={}",
            game.events().len(),
            state(&c).controls.len(),
            accepted,
            elapsed.as_millis()
        );
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn consumed_invalid_pv_is_retained_unscored_without_applying_even_its_first_move() {
    for read_only in [false, true] {
        let mut c = controller();
        let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
        replace(
            &mut c,
            &game,
            if read_only {
                x::Side::Red
            } else {
                x::Side::Black
            },
        );
        if read_only {
            send(
                &mut c,
                SessionOperation::Analyze {
                    operation: AnalysisOperation::Moves {
                        depth: 1,
                        nodes: 128,
                    },
                },
            )
            .unwrap();
        } else {
            send(&mut c, SessionOperation::Resume {}).unwrap();
        }
        let request = state(&c).pending.clone().unwrap();
        let mut response = actual(&request);
        let SharedResult::Xiangqi {
            result: XiangqiResult::Choice { telemetry, .. },
        } = &mut response.result
        else {
            panic!("actual scored result expected")
        };
        telemetry.principal_variation.push(XiangqiChoice::Move {
            action: "a0a9".parse().unwrap(),
        });
        assert!(
            response.matches(&request),
            "metadata validity alone cannot establish native PV legality"
        );
        let before = canonical(&c);
        assert!(c.accept_shared(response.clone()));
        settle(&mut c);
        let snapshot = c.shared_snapshot();
        assert_eq!(snapshot.phase, EnginePhase::Failed);
        assert!(snapshot.paused && !snapshot.status.concluded);
        assert!(state(&c).pending.is_none());
        assert!(snapshot.analysis.is_none());
        assert_eq!(snapshot.search_observations[0].score, None);
        assert!(snapshot.observations[0].application_error.is_some());
        assert_eq!(snapshot.observations[0].response, response);
        assert_eq!(canonical(&c), before);
        assert_eq!(c.shared_snapshot().phase, EnginePhase::Failed);
        let file = path("rejected-pv");
        save(&mut c, &file);
        open(&mut c, &file).unwrap();
        assert_eq!(canonical(&c), before);
        assert_eq!(state(&c).observations[0].response, response);
        assert!(state(&c).observations[0].application_error.is_some());
        assert!(state(&c).observations[0].historical);
        std::fs::remove_file(file).unwrap();
    }
}

#[test]
fn response_admission_refusal_cannot_restore_consumed_execution_as_thinking() {
    let mut c = controller();
    replace(
        &mut c,
        &x::Game::standard(x::ClaimPolicy::Explicit).unwrap(),
        x::Side::Red,
    );
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves { depth: 1, nodes: 1 },
        },
    )
    .unwrap();
    let response = actual(state(&c).pending.as_ref().unwrap());
    // Separate controller instrumentation: force a semantic admission refusal
    // after the bridge's response has already been delivered.
    c.shared.xiangqi.as_mut().unwrap().analysis_intent = None;
    let before = canonical(&c);
    assert!(c.accept_shared(response));
    settle(&mut c);
    let snapshot = c.shared_snapshot();
    assert_eq!(snapshot.phase, EnginePhase::Failed);
    assert!(snapshot.paused && snapshot.execution_purpose.is_none());
    assert!(snapshot.diagnostic.unwrap().contains("result rejected"));
    assert_eq!(canonical(&c), before);
    assert!(state(&c).observations.is_empty());
}

#[test]
fn actual_pretty_canonical_wrapper_overhead_is_bounded_before_acceptance() {
    let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let state = XiangqiDraft::replacement(&x::GameRecord::from_game(&game).unwrap(), x::Side::Red)
        .materialize()
        .unwrap();
    let bytes = encode(&state, 100).unwrap();
    encoded_native_bounds(&bytes).unwrap();
    // Original native JSON remains valid; only the actual enclosing wrapper's
    // whitespace pushes its encoded size over the canonical parser ceiling.
    let text = String::from_utf8(bytes).unwrap();
    let needle = "\"envelope_version\": 1";
    let oversized = text.replacen(
        needle,
        &format!(
            "{}{}",
            " ".repeat(mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES),
            needle
        ),
        1,
    );
    assert!(oversized.len() < MAX_ARCHIVE_BYTES);
    assert!(
        encoded_native_bounds(oversized.as_bytes())
            .unwrap_err()
            .starts_with("Application capacity:")
    );
    #[derive(Deserialize)]
    struct Original {
        payload: Box<RawValue>,
    }
    let original: Original = serde_json::from_str(&oversized).unwrap();
    assert!(decode(original.payload.get(), &mut || false).is_err());
}
