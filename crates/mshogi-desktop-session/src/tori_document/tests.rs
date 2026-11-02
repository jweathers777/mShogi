use super::*;
use mshogi_engine::{EngineFailure, ExecutionContext, ExecutionTarget, FailureKind, WorkLimits};
use serde_json::{Value, json};
use std::{
    fs,
    time::{Duration, Instant},
};
fn standard() -> ValidatedToriDocument {
    ValidatedToriDocument::new(
        t::GameRecord::from_game(&t::Game::standard()).unwrap(),
        t::Side::Sente,
    )
    .unwrap()
}
fn face(s: &str) -> t::Face {
    t::Face::ALL.into_iter().find(|f| f.code() == s).unwrap()
}
fn side(s: &str) -> t::Side {
    if s == "s" || s == "sente" {
        t::Side::Sente
    } else {
        t::Side::Gote
    }
}
fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../../../docs/m4/tori/fixtures.json")).unwrap()
}
fn given(id: &str) -> Value {
    fixtures()["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()["given"]
        .clone()
}
fn root(v: &Value) -> t::Position {
    if v["root_kind"] == "standard" {
        return t::Position::standard();
    }
    let entries: Vec<_> = v["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let p: Vec<_> = v.as_str().unwrap().split(':').collect();
            (
                p[2].parse().unwrap(),
                t::Piece {
                    side: side(p[0]),
                    face: face(p[1]),
                },
            )
        })
        .collect();
    let hands = std::array::from_fn(|i| {
        t::Hand::from_counts(
            &v["hands"][if i == 0 { "s" } else { "g" }]
                .as_object()
                .unwrap()
                .iter()
                .map(|(f, n)| (face(f), n.as_u64().unwrap() as u8))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    });
    t::Position::from_diagram(
        &entries,
        hands,
        side(v["side_to_move"].as_str().unwrap()),
        t::RootMetadata::diagram(v["declared_completed_plies"].as_u64().unwrap()),
    )
    .unwrap()
}
fn action(v: &Value) -> t::Action {
    if v["kind"] == "drop" {
        t::Action::Drop {
            face: face(v["piece"].as_str().unwrap()),
            to: v["to"].as_str().unwrap().parse().unwrap(),
        }
    } else {
        t::Action::Move {
            from: v["from"].as_str().unwrap().parse().unwrap(),
            to: v["to"].as_str().unwrap().parse().unwrap(),
            result_face: face(v["result_face"].as_str().unwrap()),
        }
    }
}
fn play(g: &mut t::Game, a: t::Action) -> Result<t::GameUndo, t::GameError> {
    g.submit(t::GameRequest {
        expected: g.context(),
        actor: g.position().side_to_move(),
        command: t::GameCommand::Play { action: a },
    })
}
fn document(g: &t::Game) -> ValidatedToriDocument {
    let mut p = standard().payload;
    p.initial = CanonicalRecord::tori(g.record_prefix(0).unwrap());
    p.current = CanonicalRecord::tori(t::GameRecord::from_game(g).unwrap());
    p.controls = g
        .events()
        .iter()
        .map(|e| ToriControl::Native {
            actor: e.actor,
            command: e.command,
        })
        .collect();
    validate_payload(p, &mut || false).unwrap()
}
fn sequence(id: &str) -> t::Game {
    let f = given(id);
    let mut g = t::Game::from_root(root(&f["root"])).unwrap();
    for a in f["actions"].as_array().unwrap() {
        play(&mut g, action(a)).unwrap();
    }
    g
}
fn observation(doc: &ValidatedToriDocument, n: u64) -> ExecutionObservation {
    let game = doc.payload.current.clone();
    ExecutionObservation::new(
        ExecutionContext {
            purpose: ExecutionPurpose::MoveAnalysis {
                target: ExecutionTarget {
                    selection_id: n,
                    event_index: game.event_count() as u64,
                    board_ply: game.known_board_actions() as u64,
                    session_version: n,
                    label: "Exact Tori prefix".into(),
                    turn: mshogi_core::native::NativeSide::Tori(
                        native(&game).unwrap().side_to_move(),
                    ),
                },
            },
            request_id: n,
            session_version: n,
            process_generation: n,
            game,
            limits: WorkLimits { depth: 1, nodes: 1 },
        },
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                message: "Tori engine execution is not implemented".into(),
            },
        },
    )
}
fn roundtrip(d: &ValidatedToriDocument) {
    let r = ValidatedToriDocument::decode(d.bytes(), &mut || false).unwrap();
    assert_eq!(r.bytes(), d.bytes());
    assert_eq!(r.record(), d.record());
}
#[test]
fn actual_native_histories_preserve_repetition_ending_prefixes_and_declared_provenance() {
    for id in ["TORI-H-FORCED-FULL", "TORI-H-UNEQUAL-FULL"] {
        let g = sequence(id);
        let d = document(&g);
        roundtrip(&d);
        assert_eq!(
            native(&d.payload.current)
                .unwrap()
                .to_game()
                .unwrap()
                .snapshot(),
            g.snapshot()
        );
        for i in 0..=g.events().len() {
            assert_eq!(
                d.record().to_game().unwrap().record_prefix(i).unwrap(),
                g.record_prefix(i).unwrap()
            );
        }
    }
    let p = t::Position::from_diagram(
        &[
            (
                "7g".parse().unwrap(),
                t::Piece {
                    side: t::Side::Sente,
                    face: t::Face::Phoenix,
                },
            ),
            (
                "1a".parse().unwrap(),
                t::Piece {
                    side: t::Side::Gote,
                    face: t::Face::Phoenix,
                },
            ),
        ],
        [t::Hand::default(), t::Hand::default()],
        t::Side::Sente,
        t::RootMetadata::imported("owner supplied diagram".into(), 123).unwrap(),
    )
    .unwrap();
    let d = ValidatedToriDocument::new(
        t::GameRecord::from_game(&t::Game::from_root(p).unwrap()).unwrap(),
        t::Side::Gote,
    )
    .unwrap();
    roundtrip(&d);
    let r: Value = serde_json::from_str(d.record().to_json()).unwrap();
    assert_eq!(r["root"]["declared_completed_plies"], 123);
    assert_eq!(r["root"]["source"], "owner supplied diagram");
    assert!(r["root"]["earlier_history_unknown"].as_bool().unwrap());
}
#[test]
fn immutable_journal_retains_rewind_branch_settings_and_exact_observation() {
    let d = standard();
    let g = d.record().to_game().unwrap();
    let a = g.legal_actions()[0];
    let d = d
        .with_control(
            ToriControl::Native {
                actor: t::Side::Sente,
                command: t::GameCommand::Play { action: a },
            },
            &mut || false,
        )
        .unwrap();
    let d = d
        .with_observation(observation(&d, 7), &mut || false)
        .unwrap();
    let previous = d.clone();
    let d = d
        .with_control(
            ToriControl::Rewind {
                event_index: 0,
                reason: RewindKind::TakeBack,
            },
            &mut || false,
        )
        .unwrap();
    assert_ne!(d.record(), previous.record());
    assert_eq!(d.payload.retired[0].record, previous.payload.current);
    assert_eq!(d.observations()[0], previous.observations()[0]);
    let d = d
        .with_control(
            ToriControl::Settings {
                settings: ToriSettings {
                    orientation: t::Side::Gote,
                },
            },
            &mut || false,
        )
        .unwrap();
    roundtrip(&d);
    let mut p = d.payload.clone();
    p.retired[0].control_index += 1;
    assert!(validate_payload(p, &mut || false).is_err());
    let mut p = d.payload.clone();
    p.observations[0].context.game = d.payload.current.clone();
    assert!(validate_payload(p, &mut || false).is_err());
    let before = d.bytes().to_vec();
    assert!(
        d.with_control(
            ToriControl::Native {
                actor: t::Side::Sente,
                command: t::GameCommand::AgreeDraw
            },
            &mut || false
        )
        .is_err()
    );
    assert_eq!(d.bytes(), before);
}
#[test]
fn strict_outer_payload_commands_and_native_facts_reject_without_normalizing_duplicates() {
    let d = document(&sequence("TORI-H-UNEQUAL-FULL"));
    let good = String::from_utf8(d.bytes().to_vec()).unwrap();
    for (key, value) in [
        ("mode", "\"untimed\""),
        ("human", "\"sente\""),
        ("controls", "[]"),
        ("mode", "\"timed\""),
    ] {
        let p = good.replacen(
            "\"payload\": {",
            &format!("\"payload\": {{\"{key}\":{value},"),
            1,
        );
        assert!(
            ValidatedToriDocument::decode(p.as_bytes(), &mut || false).is_err(),
            "{key}"
        );
    }
    for field in [
        "game_kind",
        "rules_profile",
        "profile_edition",
        "native_schema_version",
        "envelope_version",
    ] {
        let mut v: Value = serde_json::from_str(&good).unwrap();
        v[field] = json!("wrong");
        assert!(
            ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err()
        );
    }
    for field in [
        "initial",
        "current",
        "human",
        "settings",
        "controls",
        "retired",
        "observations",
        "retired_execution_floor",
        "mode",
    ] {
        let mut v: Value = serde_json::from_str(&good).unwrap();
        v["payload"].as_object_mut().unwrap().remove(field);
        assert!(
            ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err(),
            "{field}"
        );
    }
    for field in ["conclusion", "obligation", "root_conclusion", "incomplete"] {
        let mut v: Value = serde_json::from_str(&good).unwrap();
        v["payload"]["current"]["payload"]
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert!(
            ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err(),
            "{field}"
        );
    }
    let mut v: Value = serde_json::from_str(&good).unwrap();
    v["payload"]["current"]["payload"]["events"][0]["replay_work"] = json!(0);
    assert!(
        ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err()
    );
    let duplicate = good.replacen("\"board_ply\": 1", "\"board_ply\": 1, \"board_ply\": 1", 1);
    // Raw native serialization is compact; test an exact nested key instead.
    let duplicate = duplicate.replacen(
        "\"root_kind\":\"diagram\"",
        "\"root_kind\":\"diagram\",\"root_kind\":\"diagram\"",
        1,
    );
    assert_ne!(duplicate, good);
    assert!(ValidatedToriDocument::decode(duplicate.as_bytes(), &mut || false).is_err());
    for raw in given("TORI-REJECT-STRICT")["raw_payloads"]
        .as_array()
        .unwrap()
    {
        assert!(t::GameRecord::from_json(raw.as_str().unwrap()).is_err());
        assert!(t::GameCommand::from_json(raw.as_str().unwrap()).is_err());
    }
    assert!(
        ValidatedToriDocument::decode(&d.bytes()[..d.bytes().len() - 1], &mut || false).is_err()
    );
}
#[test]
fn seven_independent_reject_cases_enter_actual_native_record_parser() {
    let good = standard();
    for id in [
        "TORI-REJECT-DUPLICATE-SQUARE",
        "TORI-REJECT-MISSING-ROYAL",
        "TORI-REJECT-ADJACENT-ROYALS",
        "TORI-REJECT-DEAD-SWALLOW",
        "TORI-REJECT-THREE-SWALLOWS",
        "TORI-REJECT-EXCESS-INVENTORY",
    ] {
        let f = given(id);
        let mut v: Value = serde_json::from_slice(good.bytes()).unwrap();
        // Translate supplied independent invalid root data into the native wire
        // spelling, without invoking an admissible Position constructor.
        for which in ["initial", "current"] {
            let r = &mut v["payload"][which]["payload"];
            let entries: Vec<_> = f["root"]["board"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    let p: Vec<_> = v.as_str().unwrap().split(':').collect();
                    json!({"square":p[2],"piece":{"side":side(p[0]),"face":face(p[1])}})
                })
                .collect();
            r["root"]["root_kind"] = json!("diagram");
            r["root"]["source"] = Value::Null;
            r["root"]["earlier_history_unknown"] = json!(true);
            r["root"]["historical_reachability_proven"] = json!(false);
            r["root"]["board"] = json!(entries);
            let hands: Vec<Vec<u8>> = ["s", "g"]
                .iter()
                .map(|s| {
                    t::Face::HANDS
                        .iter()
                        .map(|k| f["root"]["hands"][s][k.code()].as_u64().unwrap_or(0) as u8)
                        .collect()
                })
                .collect();
            r["root"]["hands"] = json!(hands);
            r["current_position"] = r["root"].clone();
        }
        assert!(
            ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err(),
            "{id}"
        );
    }
    let bad = good
        .bytes()
        .iter()
        .copied()
        .chain(b" trailing".iter().copied())
        .collect::<Vec<_>>();
    assert!(ValidatedToriDocument::decode(&bad, &mut || false).is_err());
}
#[test]
fn unsupported_failure_observations_are_exact_historical_unscored_evidence() {
    let d = standard();
    for kind in [
        FailureKind::UnsupportedMode,
        FailureKind::UnexpectedExit,
        FailureKind::Timeout,
        FailureKind::MissingResponse,
    ] {
        let mut o = observation(&d, 8);
        let ExecutionResult::Failure { failure } = &mut o.result else {
            unreachable!()
        };
        failure.kind = kind;
        let candidate = d.with_observation(o, &mut || false).unwrap();
        roundtrip(&candidate);
        assert_eq!(candidate.record().conclusion(), None);
    }
    let mut o = observation(&d, 8);
    if let ExecutionPurpose::MoveAnalysis { target } = &mut o.context.purpose {
        target.event_index = 1;
    }
    assert!(d.with_observation(o, &mut || false).is_err());
    let mut o = observation(&d, 8);
    o.context.limits.depth = 0;
    assert!(d.with_observation(o, &mut || false).is_err());
    let mut o = observation(&d, 8);
    o.context.game = CanonicalRecord::shogi(mshogi_core::shogi::GameRecord::from_game(
        &mshogi_core::shogi::Game::new(),
    ));
    assert!(d.with_observation(o, &mut || false).is_err());
}
#[test]
fn observation256_to257_refuses_before_acceptance_and_all_native_ending_headroom_survives() {
    let mut p = standard().payload;
    let base = standard();
    for i in 0..256 {
        p.observations.push(observation(&base, i + 1));
        p.controls
            .push(ToriControl::Observation { index: i as usize });
    }
    p.retired_execution_floor = 256;
    let d = validate_payload(p, &mut || false).unwrap();
    assert_eq!(d.usage.observations, 256);
    roundtrip(&d);
    let before = d.bytes().to_vec();
    let error = d
        .with_observation(observation(&d, 257), &mut || false)
        .unwrap_err();
    assert_eq!(
        error.resource.unwrap().facts()[0].category,
        ResourceCategory::OrdinaryObservations
    );
    assert_eq!(d.bytes(), before);
    let d = d
        .with_control(
            ToriControl::Native {
                actor: t::Side::Gote,
                command: t::GameCommand::Stop {
                    reason: t::StopReason::ResourceLimit,
                },
            },
            &mut || false,
        )
        .unwrap();
    assert!(d.record().conclusion().is_none());
    let d = d
        .with_control(
            ToriControl::Native {
                actor: t::Side::Gote,
                command: t::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    let d = d
        .with_control(ToriControl::Close {}, &mut || false)
        .unwrap();
    roundtrip(&d);
}
#[test]
fn exact_scalar_limits_reserves_and_plus_one_report_correct_resource() {
    let base = ToriDocumentUsage {
        encoded_bytes: 0,
        retained_events: 0,
        controls: 0,
        observations: 0,
        replay_work: 0,
    };
    for (cat, cap) in [
        (
            ResourceCategory::EncodedDocumentBytes,
            MAX_ARCHIVE_BYTES as u64,
        ),
        (ResourceCategory::NativeEvents, MAX_ARCHIVE_EVENTS as u64),
        (ResourceCategory::Controls, MAX_ARCHIVE_CONTROLS as u64),
        (ResourceCategory::OrdinaryObservations, 256),
        (ResourceCategory::ReplayWork, MAX_TORI_DOCUMENT_WORK),
    ] {
        for extra in [0, 1] {
            let mut u = base.clone();
            let n = cap + extra;
            match cat {
                ResourceCategory::EncodedDocumentBytes => u.encoded_bytes = n as usize,
                ResourceCategory::NativeEvents => u.retained_events = n as usize,
                ResourceCategory::Controls => u.controls = n as usize,
                ResourceCategory::OrdinaryObservations => u.observations = n as usize,
                _ => u.replay_work = n,
            }
            let result = check_limits(
                &u,
                1,
                Reserve {
                    native: 0,
                    controls: 0,
                },
            );
            if extra == 0 {
                assert!(result.is_ok());
            } else {
                let f = result.unwrap_err().resource.unwrap();
                assert_eq!(f.facts()[0].category, cat);
                assert_eq!(f.facts()[0].count, Some(n));
            }
        }
    }
    let mut u = base.clone();
    u.controls = MAX_ARCHIVE_CONTROLS - 3;
    assert!(
        check_limits(
            &u,
            1,
            Reserve {
                native: 2,
                controls: 3
            }
        )
        .is_ok()
    );
    u.controls += 1;
    assert_eq!(
        check_limits(
            &u,
            1,
            Reserve {
                native: 2,
                controls: 3
            }
        )
        .unwrap_err()
        .resource
        .unwrap()
        .facts()[0]
            .category,
        ResourceCategory::Controls
    );
    assert_eq!(
        check_limits(
            &base,
            MAX_SAFE_ID - 64,
            Reserve {
                native: 0,
                controls: 0
            }
        )
        .unwrap_err()
        .resource
        .unwrap()
        .facts()[0]
            .category,
        ResourceCategory::IdentityHeadroom
    );
}
#[test]
fn actual_control_limit_consumes_stop_resign_close_reservations_without_resetting_them() {
    let mut p = standard().payload;
    p.controls = (0..MAX_ARCHIVE_CONTROLS - 3)
        .map(|_| ToriControl::Settings {
            settings: p.settings.clone(),
        })
        .collect();
    let d = validate_payload(p, &mut || false).unwrap();
    assert!(
        d.with_control(
            ToriControl::Settings {
                settings: d.payload.settings.clone()
            },
            &mut || false
        )
        .is_err()
    );
    let d = d
        .with_control(
            ToriControl::Native {
                actor: t::Side::Sente,
                command: t::GameCommand::Stop {
                    reason: t::StopReason::Cancelled,
                },
            },
            &mut || false,
        )
        .unwrap();
    let d = d
        .with_control(
            ToriControl::Native {
                actor: t::Side::Gote,
                command: t::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    let d = d
        .with_control(ToriControl::Close {}, &mut || false)
        .unwrap();
    assert_eq!(d.usage.controls, 8192);
    roundtrip(&d);
}
fn path() -> std::path::PathBuf {
    static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "msh130-{}-{}.mshogi",
        std::process::id(),
        N.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ))
}
fn wait(w: &ToriDocumentWorker) -> (u64, Result<ToriFileResult, ToriDocumentError>) {
    let end = Instant::now() + Duration::from_secs(30);
    loop {
        if let Some(v) = w.poll() {
            return v;
        }
        assert!(Instant::now() < end);
        std::thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn actual_file_worker_first_save_replacement_failure_interruption_and_stale_receipts() {
    let d = standard();
    let file = path();
    let mut w = ToriDocumentWorker::new().unwrap();
    assert_eq!(
        w.validate(1, vec![b' '; MAX_ARCHIVE_BYTES + 1])
            .unwrap_err()
            .resource
            .unwrap()
            .facts()[0]
            .category,
        ResourceCategory::EncodedDocumentBytes
    );
    assert!(w.poll().is_none()); // rejection neither queues nor consumes the ID
    w.save(1, d.bytes().to_vec(), file.clone(), 10, 20).unwrap();
    let (id, result) = wait(&w);
    assert_eq!(id, 1);
    assert!(matches!(
        result.unwrap(),
        ToriFileResult::Saved {
            revision: 10,
            identity: 20,
            ..
        }
    ));
    assert_eq!(fs::read(&file).unwrap(), d.bytes());
    let d2 = d
        .with_control(
            ToriControl::Settings {
                settings: ToriSettings {
                    orientation: t::Side::Gote,
                },
            },
            &mut || false,
        )
        .unwrap();
    w.save(2, d2.bytes().to_vec(), file.clone(), 11, 20)
        .unwrap();
    assert!(matches!(
        wait(&w).1.unwrap(),
        ToriFileResult::Saved {
            revision: 11,
            identity: 20,
            ..
        }
    ));
    assert_eq!(fs::read(&file).unwrap(), d2.bytes());
    w.save(3, b"corrupt".to_vec(), file.clone(), 12, 20)
        .unwrap();
    assert!(wait(&w).1.is_err());
    assert_eq!(fs::read(&file).unwrap(), d2.bytes());
    // Real writer cancellation after temporary-file construction/chunk write.
    let mut checks = 0;
    assert!(
        persistence::atomic_write(&file, d.bytes(), &mut || {
            checks += 1;
            checks >= 2
        })
        .is_err()
    );
    assert_eq!(fs::read(&file).unwrap(), d2.bytes());
    let dir = path();
    fs::create_dir(&dir).unwrap();
    assert!(persistence::atomic_write(&dir, d.bytes(), &mut || false).is_err());
    assert!(dir.is_dir());
    assert_eq!(fs::read(&file).unwrap(), d2.bytes());
    fs::remove_dir(dir).unwrap();
    w.validate(4, d.bytes().to_vec()).unwrap();
    w.cancel();
    w.validate(5, d2.bytes().to_vec()).unwrap();
    let (id, result) = wait(&w);
    assert_eq!(id, 5);
    assert!(matches!(result.unwrap(), ToriFileResult::Validated(_)));
    assert!(w.validate(5, d.bytes().to_vec()).is_err());
    assert!(w.poll().is_none());
    assert!(ValidatedToriDocument::decode(d.bytes(), &mut || true).is_err());
    fs::remove_file(file).unwrap();
}
#[test]
fn no_tori_normal_play_capability_is_enabled() {
    assert!(!mshogi_engine::execution_capabilities(GameKind::Tori).is_empty());
    let mut c = DesktopController::new("/unused-engine".into()).unwrap();
    let before = c.shared_snapshot().context;
    // MSH-131 now admits native verification documents; normal New Game stays gated.
    assert!(
        c.shared_new_game(
            c.shared_open_token(),
            GameKind::Tori,
            NativeSide::Tori(t::Side::Sente)
        )
        .is_err()
    );
    assert_eq!(c.shared_snapshot().context, before);
}

#[test]
fn forged_selection_identity_and_all_strict_fixture_payloads_reject_at_desktop_boundary() {
    let d = standard()
        .with_observation(observation(&standard(), 7), &mut || false)
        .unwrap();
    let mut v: Value = serde_json::from_slice(d.bytes()).unwrap();
    v["payload"]["observations"][0]["context"]["purpose"]["target"]["selection_id"] =
        json!(u64::MAX);
    assert!(
        ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err()
    );
    for raw in given("TORI-REJECT-STRICT")["raw_payloads"]
        .as_array()
        .unwrap()
    {
        // Keep the exact original raw bytes, including repeated fields.
        let envelope = document::DocumentEnvelope::parse(standard().bytes()).unwrap();
        let mut p: Value = serde_json::from_str(envelope.payload.get()).unwrap();
        p["current"]["payload"] = json!("RAW_SENTINEL");
        let encoded = document::encode(RulesProfile::ToriModernV1, &p).unwrap();
        let encoded = String::from_utf8(encoded)
            .unwrap()
            .replace("\"RAW_SENTINEL\"", raw.as_str().unwrap());
        assert!(ValidatedToriDocument::decode(encoded.as_bytes(), &mut || false).is_err());
    }
    let mut called = 0;
    assert!(
        d.with_control(ToriControl::Close {}, &mut || {
            called += 1;
            true
        })
        .is_err()
    );
    assert_eq!(called, 1);
}
#[test]
fn per_operation_byte_and_work_reservations_cover_real_native_encoding_growth() {
    let d = standard();
    let stop = d
        .with_control(
            ToriControl::Native {
                actor: t::Side::Sente,
                command: t::GameCommand::Stop {
                    reason: t::StopReason::WorkLimit,
                },
            },
            &mut || false,
        )
        .unwrap();
    let resign = stop
        .with_control(
            ToriControl::Native {
                actor: t::Side::Gote,
                command: t::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    let close = resign
        .with_control(ToriControl::Close {}, &mut || false)
        .unwrap();
    // Apply actual encoded/replay deltas at each independently isolated policy
    // ceiling; other document quotas can make a full16MiB candidate unreachable.
    for resource in [
        ResourceCategory::EncodedDocumentBytes,
        ResourceCategory::ReplayWork,
    ] {
        let mut u = d.usage.clone();
        if resource == ResourceCategory::EncodedDocumentBytes {
            u.encoded_bytes = MAX_ARCHIVE_BYTES - 2 * NATIVE_RESERVED_BYTES - CLOSE_RESERVED_BYTES;
        } else {
            u.replay_work = MAX_TORI_DOCUMENT_WORK - 2 * NATIVE_RESERVED_WORK - CLOSE_RESERVED_WORK;
        }
        check_limits(&u, 1, reservation(&d.payload)).unwrap();
        let mut previous = &d;
        for next in [&stop, &resign, &close] {
            u.encoded_bytes += next.usage.encoded_bytes - previous.usage.encoded_bytes;
            u.replay_work += next.usage.replay_work - previous.usage.replay_work;
            u.controls += 1;
            u.retained_events += next.usage.retained_events - previous.usage.retained_events;
            check_limits(&u, 1, reservation(&next.payload)).unwrap();
            previous = next;
        }
    }
}
#[test]
fn actual_encoded_input_limit_and_aggregate_event_limit_plus_one() {
    let d = standard();
    let bound = MAX_ARCHIVE_BYTES - 2 * NATIVE_RESERVED_BYTES - CLOSE_RESERVED_BYTES;
    let mut padded = d.bytes().to_vec();
    padded.resize(bound, b' ');
    ValidatedToriDocument::decode(&padded, &mut || false).unwrap();
    padded.push(b' ');
    assert_eq!(
        ValidatedToriDocument::decode(&padded, &mut || false)
            .unwrap_err()
            .resource
            .unwrap()
            .facts()[0]
            .category,
        ResourceCategory::EncodedDocumentBytes
    );
    let g = sequence("TORI-H-UNEQUAL-FULL");
    // Event15 then resignation makes16 events, a concluded native target whose
    // 255 retained observations plus current reach exactly4096 retained events.
    let mut g = g.record_prefix(15).unwrap().to_game().unwrap();
    g.submit(t::GameRequest {
        expected: g.context(),
        actor: t::Side::Sente,
        command: t::GameCommand::Resign,
    })
    .unwrap();
    let d = document(&g);
    let mut p = d.payload.clone();
    p.retired_execution_floor = 256;
    for i in 0..255 {
        p.controls.push(ToriControl::Observation { index: i });
        p.observations.push(observation(&d, i as u64 + 1));
    }
    let accepted = validate_payload(p, &mut || false).unwrap();
    assert_eq!(accepted.usage.retained_events, 4096);
    roundtrip(&accepted);
    let before = accepted.bytes().to_vec();
    let err = accepted
        .with_observation(observation(&accepted, 256), &mut || false)
        .unwrap_err();
    assert_eq!(
        err.resource.unwrap().facts()[0].category,
        ResourceCategory::NativeEvents
    );
    assert_eq!(accepted.bytes(), before);
}
#[test]
fn measured_complete_long_documents_charge_repeated_native_work_and_refuse_before_loss() {
    let start = Instant::now();
    let mut g = t::Game::standard();
    let mut seed = 0x129c_a11a_b1e5u64;
    for _ in 0..360 {
        let choices = g.legal_actions();
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let offset = seed as usize % choices.len();
        let mut chosen = None;
        for i in 0..choices.len() {
            let a = choices[(offset + i) % choices.len()];
            match play(&mut g, a) {
                Ok(mut u) => {
                    let valid = g.conclusion().is_none();
                    let fresh = g.occurrences().len() == 1;
                    g.undo(&mut u).unwrap();
                    if valid {
                        chosen = Some(a);
                        if fresh {
                            break;
                        }
                    }
                }
                Err(t::GameError::Capacity(_)) => {
                    chosen = Some(a);
                    break;
                }
                Err(_) => {}
            }
        }
        play(
            &mut g,
            chosen.expect("live independently legal continuation"),
        )
        .unwrap();
    }
    let generated_ms = start.elapsed().as_millis();
    let start = Instant::now();
    let d = document(&g);
    let roundtrip_ms = start.elapsed().as_millis();
    roundtrip(&d);
    let native_usage = g.capacity_usage().unwrap();
    assert_eq!(native_usage.replay_work, 65_341);
    assert_eq!(g.events().len(), 360);
    assert!(
        g.events()
            .iter()
            .any(|e| e.effects.is_some_and(|x| x.captured.is_some()))
    );
    assert!(g.events().iter().any(|e| matches!(
        e.command,
        t::GameCommand::Play {
            action: t::Action::Drop { .. }
        }
    )));
    assert!(
        g.events()
            .iter()
            .any(|e| e.effects.is_some_and(|x| matches!(
                x.action,
                t::Action::Move {
                    result_face: t::Face::Eagle | t::Face::Goose,
                    ..
                }
            )))
    );
    let observed = d
        .with_observation(observation(&d, 11), &mut || false)
        .unwrap();
    let before = observed.bytes().to_vec();
    let refused = observed
        .with_observation(observation(&observed, 12), &mut || false)
        .unwrap_err();
    assert_eq!(
        refused.resource.as_ref().unwrap().facts()[0].category,
        ResourceCategory::ReplayWork
    );
    assert_eq!(observed.bytes(), before);
    let stopped = observed
        .with_control(
            ToriControl::Native {
                actor: t::Side::Sente,
                command: t::GameCommand::Stop {
                    reason: t::StopReason::ResourceLimit,
                },
            },
            &mut || false,
        )
        .unwrap();
    let resigned = stopped
        .with_control(
            ToriControl::Native {
                actor: t::Side::Gote,
                command: t::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    let closed = resigned
        .with_control(ToriControl::Close {}, &mut || false)
        .unwrap();
    roundtrip(&closed);
    let branch = d
        .with_control(
            ToriControl::Rewind {
                event_index: 180,
                reason: RewindKind::TakeBack,
            },
            &mut || false,
        )
        .unwrap();
    roundtrip(&branch);
    let worker_started = Instant::now();
    let saved = path();
    let mut worker = ToriDocumentWorker::new().unwrap();
    worker
        .save(1, closed.bytes().to_vec(), saved.clone(), 99, 100)
        .unwrap();
    assert!(matches!(
        wait(&worker).1.unwrap(),
        ToriFileResult::Saved {
            revision: 99,
            identity: 100,
            ..
        }
    ));
    assert_eq!(fs::read(&saved).unwrap(), closed.bytes());
    fs::remove_file(saved).unwrap();
    let worker_save_ms = worker_started.elapsed().as_millis();
    let root_base = standard();
    let mut roots = root_base.payload.clone();
    roots.retired_execution_floor = 256;
    for i in 0..256 {
        roots.controls.push(ToriControl::Observation { index: i });
        roots
            .observations
            .push(observation(&root_base, i as u64 + 1));
    }
    let root_started = Instant::now();
    let roots = validate_payload(roots, &mut || false).unwrap();
    let root_observations_ms = root_started.elapsed().as_millis();
    let report = json!({"seed":"0x129ca11ab1e5","generated_ms":generated_ms,"worker_save_ms":worker_save_ms,"root_observations_ms":root_observations_ms,"root256":roots.usage(),"initial_validation_ms":roundtrip_ms,"native":{"bytes":native_usage.encoded_bytes,"events":native_usage.events,"work":native_usage.replay_work},"complete_document":d.usage(),"one_observation":observed.usage(),"second_observation_refusal":{"resource":refused.resource},"stopped":stopped.usage(),"resigned_closed":closed.usage(),"retained_branch":branch.usage(),"forced8":document(&sequence("TORI-H-FORCED-FULL")).usage(),"unequal16":document(&sequence("TORI-H-UNEQUAL-FULL")).usage()});
    println!("MSH130_MEASUREMENTS {}", report);
    if let Ok(path) = std::env::var("MSH_TORI_130_MEASUREMENTS") {
        fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
}

#[test]
fn single_unsupported_mode_unknown_fields_and_forged_intermediate_facts_reject() {
    let d = document(&sequence("TORI-H-UNEQUAL-FULL"));
    let good: Value = serde_json::from_slice(d.bytes()).unwrap();
    for name in [
        "mode",
        "payload_unknown",
        "command_unknown",
        "effect",
        "conclusion",
        "root_counter",
        "declared_counter",
        "root_provenance",
        "retained_count",
        "floor",
    ] {
        let mut v = good.clone();
        match name {
            "mode" => v["payload"]["mode"] = json!("timed"),
            "payload_unknown" => v["payload"]["pending_request"] = json!({"request_id":1}),
            "command_unknown" => v["payload"]["controls"][0]["command"]["victims"] = json!([]),
            "effect" => v["payload"]["current"]["payload"]["events"][0]["effects"] = Value::Null,
            "conclusion" => {
                v["payload"]["current"]["payload"]["events"][0]["conclusion"] =
                    json!({"kind":"resignation","winner":"sente"})
            }
            "root_counter" => {
                v["payload"]["current"]["payload"]["root"]["declared_total_plies"] = json!(999)
            }
            "declared_counter" => {
                v["payload"]["current"]["payload"]["root"]["declared_completed_plies"] = json!(999);
            }
            "root_provenance" => {
                v["payload"]["current"]["payload"]["root"]["historical_reachability_proven"] =
                    json!(true)
            }
            "retained_count" => {
                v["payload"]["retired"] =
                    json!([{"control_index":0,"record":v["payload"]["current"].clone()}])
            }
            _ => v["payload"]["retired_execution_floor"] = json!(0),
        }
        assert!(
            ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err(),
            "{name}"
        );
    }
}
#[test]
fn native_record_byte_ceiling_and_desktop_precharge_reject_forged_work_before_native_replay() {
    let d = standard()
        .with_control(
            ToriControl::Native {
                actor: t::Side::Gote,
                command: t::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    let e = document::DocumentEnvelope::parse(d.bytes()).unwrap();
    let text = e.payload.get();
    let mut p: Value = serde_json::from_str(text).unwrap();
    let native = serde_json::to_string(&p["current"]["payload"]).unwrap();
    p["current"]["payload"] = json!("NATIVE_PAD");
    let doc = String::from_utf8(document::encode(RulesProfile::ToriModernV1, &p).unwrap()).unwrap();
    let mut padded = native[..native.len() - 1].to_string();
    padded.extend(std::iter::repeat_n(
        ' ',
        t::MAX_GAME_RECORD_BYTES - native.len(),
    ));
    padded.push('}');
    let at = doc.replace("\"NATIVE_PAD\"", &padded);
    ValidatedToriDocument::decode(at.as_bytes(), &mut || false).unwrap();
    padded.insert(padded.len() - 1, ' ');
    let over = doc.replace("\"NATIVE_PAD\"", &padded);
    let error = ValidatedToriDocument::decode(over.as_bytes(), &mut || false).unwrap_err();
    assert_eq!(
        error.resource.unwrap().facts()[0].category,
        ResourceCategory::CanonicalRecordBytes
    );
    let mut v: Value = serde_json::from_slice(standard().bytes()).unwrap();
    let event = json!({"command":{"kind":"play"},"replay_work":0});
    v["payload"]["current"]["payload"]["events"] = json!(vec![event.clone(); 513]);
    let error =
        ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).unwrap_err();
    assert_eq!(
        error.resource.unwrap().facts()[0].category,
        ResourceCategory::NativeEvents
    );
    // Many bounded-looking native contexts must fail raw aggregate preflight,
    // even when each forged event claims zero work and native replay would fail.
    v["payload"]["current"]["payload"]["events"] = json!(vec![event; 360]);
    v["payload"]["retired"] = json!(vec![
        json!({"control_index":0,"record":v["payload"]["current"].clone()});
        4
    ]);
    let error =
        ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).unwrap_err();
    assert_eq!(
        error.resource.unwrap().facts()[0].category,
        ResourceCategory::ReplayWork
    );
}

#[test]
fn every_native_record_is_precharged_and_wrong_inner_identity_never_falls_through() {
    let good: Value = serde_json::from_slice(standard().bytes()).unwrap();
    for (field, bad) in [
        ("game_kind", json!("shogi")),
        ("rules_profile", json!("mshogi.shogi.csa36.v1")),
        ("native_schema_version", json!(2)),
        ("envelope_version", json!(2)),
        ("profile_edition", json!("wrong")),
    ] {
        let mut v = good.clone();
        v["payload"]["current"][field] = bad;
        assert!(
            ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false).is_err(),
            "{field}"
        );
    }
    for which in ["initial", "current"] {
        let mut v = good.clone();
        v["payload"][which]["payload"]["events"] = json!(vec![
            json!({"command":{"kind":"play"},"replay_work":0});
            361
        ]);
        let e = ValidatedToriDocument::decode(&serde_json::to_vec(&v).unwrap(), &mut || false)
            .unwrap_err();
        let f = e.resource.unwrap();
        assert_eq!(f.facts()[0].category, ResourceCategory::ReplayWork);
        assert_eq!(f.facts()[0].limit, Some(t::MAX_REPLAY_WORK));
    }
}
#[test]
fn runtime_floor_width_reservation_survives_consumed_close_and_exact_byte_boundary() {
    let mut doc = standard();
    let reserve = Reserve {
        controls: 0,
        native: 0,
    };
    let extra = (MAX_SAFE_ID - 65).to_string().len() - 1;
    doc.usage.encoded_bytes = MAX_ARCHIVE_BYTES - extra;
    assert!(check_runtime_bytes(&doc.usage, 1, reserve).is_ok());
    doc.usage.encoded_bytes += 1;
    let refused = check_runtime_bytes(&doc.usage, 1, reserve).unwrap_err();
    assert!(
        refused
            .resource
            .unwrap()
            .facts()
            .iter()
            .any(|f| f.category == ResourceCategory::EncodedDocumentBytes
                && f.reserved == Some(extra as u64))
    );
    // No additional width is needed once the maximum safe spelling is present.
    doc.usage.encoded_bytes = MAX_ARCHIVE_BYTES;
    assert!(check_runtime_bytes(&doc.usage, MAX_SAFE_ID - 65, reserve).is_ok());
}

fn actual_observation(doc: &ValidatedToriDocument, n: u64, analysis: bool) -> ExecutionObservation {
    let mut context = observation(doc, n).context;
    context.limits = WorkLimits {
        depth: 1,
        nodes: 1000,
    };
    if !analysis {
        context.purpose = ExecutionPurpose::EngineTurn {};
    }
    let request = mshogi_engine::ExecutionRequest {
        protocol: mshogi_engine::PROTOCOL_VERSION,
        context,
    };
    let mut output = vec![];
    mshogi_engine::serve_once(
        std::io::Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
        &mut output,
    )
    .unwrap();
    let response: mshogi_engine::ExecutionResponse = serde_json::from_slice(&output).unwrap();
    assert!(response.matches(&request));
    response.observation
}
#[test]
fn actual_tori_choice_is_atomic_with_native_action_while_analysis_is_read_only() {
    let original = standard();
    let analysis = actual_observation(&original, 42, true);
    assert!(matches!(
        analysis.result,
        ExecutionResult::Tori {
            result: mshogi_engine::ToriResult::Choice { .. }
        }
    ));
    let read = original
        .with_observation(analysis.clone(), &mut || false)
        .unwrap();
    assert_eq!(read.record(), original.record());
    assert_eq!(read.controls().len(), 1);
    roundtrip(&read);
    let turn = actual_observation(&original, 43, false);
    let moved = original.with_observation(turn, &mut || false).unwrap();
    assert_eq!(moved.record().known_board_actions(), 1);
    assert!(matches!(
        moved.controls(),
        [
            ToriControl::Observation { index: 0 },
            ToriControl::Native {
                command: t::GameCommand::Play { .. },
                ..
            }
        ]
    ));
    roundtrip(&moved);
    let mut forged = moved.payload.clone();
    forged.controls.pop();
    forged.current = original.payload.current.clone();
    assert!(validate_payload(forged, &mut || false).is_err());
    let mut forged = read.payload.clone();
    if let ExecutionResult::Tori {
        result: mshogi_engine::ToriResult::Choice { telemetry, .. },
    } = &mut forged.observations[0].result
    {
        telemetry.score = Some(99999);
    }
    assert!(validate_payload(forged, &mut || false).is_err());
    assert_eq!(original.record().event_count(), 0);
}
#[test]
fn actual_search_observation_256_to_257_measures_archive_and_reserves_native_endings() {
    let d = standard();
    let o = actual_observation(&d, 1, true);
    let mut p = d.payload.clone();
    for n in 1..=256 {
        let mut o = o.clone();
        o.context.request_id = n;
        o.context.session_version = n;
        o.context.process_generation = n;
        if let ExecutionPurpose::MoveAnalysis { target } = &mut o.context.purpose {
            target.selection_id = n;
            target.session_version = n;
        }
        p.controls.push(ToriControl::Observation {
            index: p.observations.len(),
        });
        p.observations.push(o);
    }
    p.retired_execution_floor = 256;
    let start = Instant::now();
    let accepted = validate_payload(p, &mut || false).unwrap();
    eprintln!(
        "MSH42_REAL_OBSERVATIONS {} decode_ms={}",
        serde_json::to_string(accepted.usage()).unwrap(),
        start.elapsed().as_millis()
    );
    let bytes = accepted.bytes().to_vec();
    let e = accepted
        .with_observation(actual_observation(&d, 257, true), &mut || false)
        .unwrap_err();
    assert_eq!(
        e.resource.unwrap().facts()[0].category,
        ResourceCategory::OrdinaryObservations
    );
    assert_eq!(accepted.bytes(), bytes);
    let stopped = accepted
        .with_control(
            ToriControl::Native {
                actor: t::Side::Sente,
                command: t::GameCommand::Stop {
                    reason: t::StopReason::Cancelled,
                },
            },
            &mut || false,
        )
        .unwrap();
    let resigned = stopped
        .with_control(
            ToriControl::Native {
                actor: t::Side::Sente,
                command: t::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    let closed = resigned
        .with_control(ToriControl::Close {}, &mut || false)
        .unwrap();
    roundtrip(&closed);
    assert_eq!(closed.observations().len(), 256);
    assert!(closed.record().conclusion().is_some());
}
#[test]
fn real_observations_with_long_history_charge_replay_before_acceptance() {
    let mut game = t::Game::standard();
    let mut seed = 0x42b0_0c1eu64;
    for _ in 0..64 {
        if game.conclusion().is_some() {
            break;
        }
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let legal = game.legal_actions();
        let a = legal[(seed as usize) % legal.len()];
        play(&mut game, a).unwrap();
    }
    let base = document(&game);
    let mut candidate = base.clone();
    let start = Instant::now();
    for n in 1..=16 {
        let o = actual_observation(&candidate, n, true);
        match candidate.with_observation(o, &mut || false) {
            Ok(next) => candidate = next,
            Err(e) => {
                assert!(e.resource.is_some());
                break;
            }
        }
    }
    assert!(!candidate.observations().is_empty());
    roundtrip(&candidate);
    eprintln!(
        "MSH42_LONG_REAL_OBSERVATIONS board_actions={} {} journey_ms={}",
        game.events().len(),
        serde_json::to_string(candidate.usage()).unwrap(),
        start.elapsed().as_millis()
    );
    assert!(candidate.usage().replay_work <= MAX_TORI_DOCUMENT_WORK);
}
