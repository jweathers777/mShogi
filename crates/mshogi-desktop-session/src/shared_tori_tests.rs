//! Actual sole-controller acceptance, distinct from native rules and GUI acceptance.
use super::*;
use serde_json::{Value, json};
use std::{fs, time::Duration};
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
    serde_json::from_str(include_str!("../../../docs/m4/tori/fixtures.json")).unwrap()
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

fn controller() -> DesktopController {
    let mut c = DesktopController::new("/unused/tori131-engine".into()).unwrap();
    c.shared.manual_engine = true;
    c
}
fn current(c: &DesktopController) -> &ToriSession {
    c.session.tori().unwrap()
}
fn settle(c: &mut DesktopController, receipt: AdmissionReceipt) -> Result<SharedSnapshot, String> {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let snap = c.shared_snapshot();
        if let Some(r) = receipt.completion() {
            return r.map(|_| snap);
        }
        assert!(Instant::now() < deadline, "admission timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    let (_, r) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game: GameKind::Tori,
        operation,
    })?;
    settle(c, r)
}
fn replace(c: &mut DesktopController, game: &t::Game, human: t::Side) {
    let game_kind = c.shared_snapshot().game;
    let (_, r) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: c.shared_token(),
            game: game_kind,
            operation: SessionOperation::Replace {
                record: CanonicalRecord::tori(t::GameRecord::from_game(game).unwrap()),
                human: NativeSide::Tori(human),
            },
        })
        .unwrap();
    settle(c, r).unwrap();
}
fn native(c: &DesktopController) -> t::GameRecord {
    t::GameRecord::from_game(&current(c).game).unwrap()
}
fn exact(c: &DesktopController) -> Vec<u8> {
    current(c).document.bytes().to_vec()
}
fn command(v: &Value) -> t::GameCommand {
    match v["kind"].as_str().unwrap() {
        "resign" => t::GameCommand::Resign,
        "agree_draw" => t::GameCommand::AgreeDraw,
        "declare_impasse" => t::GameCommand::DeclareImpasse,
        _ => t::GameCommand::Play { action: action(v) },
    }
}
fn play(g: &mut t::Game, a: t::Action) {
    g.submit(t::GameRequest {
        expected: g.context(),
        actor: g.position().side_to_move(),
        command: t::GameCommand::Play { action: a },
    })
    .unwrap();
}
fn sequence(v: &Value) -> t::Game {
    let mut g = t::Game::from_root(root(&v["root"])).unwrap();
    for a in v["actions"].as_array().unwrap() {
        play(&mut g, action(a));
    }
    g
}
fn wait_file(c: &mut DesktopController, id: u64) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed) {
            return s;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn directory(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("msh131-{name}-{}", std::process::id()));
    fs::create_dir_all(&path).unwrap();
    path
}
fn context(c: &DesktopController, n: u64, analysis: bool) -> ExecutionContext {
    let state = current(c);
    let game = CanonicalRecord::tori(t::GameRecord::from_game(state.selected()).unwrap());
    ExecutionContext {
        purpose: if analysis {
            ExecutionPurpose::MoveAnalysis {
                target: ExecutionTarget {
                    selection_id: c.session.selection_id,
                    event_index: game.event_count() as u64,
                    board_ply: game.known_board_actions() as u64,
                    session_version: c.session.version,
                    label: "Constructed negative-test target".into(),
                    turn: NativeSide::Tori(state.selected().position().side_to_move()),
                },
            }
        } else {
            ExecutionPurpose::EngineTurn {}
        },
        request_id: n,
        session_version: c.session.version,
        process_generation: n,
        game,
        limits: WorkLimits {
            depth: 1,
            nodes: 100,
        },
    }
}
fn response(context: ExecutionContext) -> ExecutionObservation {
    ExecutionObservation::new(
        context,
        ExecutionResult::Failure {
            failure: mshogi_engine::EngineFailure {
                kind: mshogi_engine::FailureKind::UnsupportedMode,
                message: "Constructed context-negative observation; actual child tested separately"
                    .into(),
            },
        },
    )
}
fn submit_observation(c: &mut DesktopController, o: ExecutionObservation) -> bool {
    if !c.accept_observation(o) {
        return false;
    }
    let r = c.admission_receipt.as_ref().unwrap().clone();
    settle(c, r).is_ok()
}
fn check_board(position: &t::Position, entry: &str, present: bool) {
    let p: Vec<_> = entry.split(':').collect();
    let sq = p[2].parse().unwrap();
    let piece = t::Piece {
        side: side(p[0]),
        face: face(p[1]),
    };
    assert_eq!(position.piece(sq) == Some(piece), present, "{entry}");
}
#[test]
fn actual_controller_seventy_native_admission_cases_and_rotations() {
    let mut ids = vec![];
    for f in fixtures()["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["scope"].as_str().unwrap().starts_with("native_admission"))
    {
        let id = f["id"].as_str().unwrap();
        let v = &f["given"];
        let expected = &f["expected"];
        let g = t::Game::from_root(root(&v["root"])).unwrap();
        let actor = v["command"]["actor"]
            .as_str()
            .map(side)
            .unwrap_or(g.position().side_to_move());
        let mut c = controller();
        replace(&mut c, &g, actor);
        let before = exact(&c);
        let before_native = native(&c);
        let token = c.shared_token();
        // Native inspection is a read-only operation; may expose the opposing owner.
        let origin = t::Square::all()
            .find(|sq| current(&c).game.position().piece(*sq).is_some())
            .unwrap()
            .to_string();
        let inspected = c
            .shared_inspection(MovementInspectionQuery {
                expected: token,
                origin,
            })
            .unwrap();
        assert!(inspected.read_only);
        assert_eq!(exact(&c), before);
        assert_eq!(token, c.shared_token());
        let result = send(
            &mut c,
            SessionOperation::Tori {
                actor,
                command: command(&v["command"]),
            },
        );
        assert_eq!(
            result.is_ok(),
            expected["accepted"].as_bool().unwrap(),
            "{id}: {result:?}"
        );
        if result.is_err() {
            assert_eq!(exact(&c), before, "{id}");
            assert_eq!(native(&c), before_native);
            assert_eq!(c.session.version, token.version);
            assert!(c.session.capacity.resource.is_none());
        } else {
            let position = current(&c).game.position();
            for entry in expected["board_removed"].as_array().into_iter().flatten() {
                check_board(position, entry.as_str().unwrap(), false);
            }
            for entry in expected["board_added"].as_array().into_iter().flatten() {
                check_board(position, entry.as_str().unwrap(), true);
            }
            for entry in expected["board_unchanged"].as_array().into_iter().flatten() {
                check_board(position, entry.as_str().unwrap(), true);
            }
            for entry in expected["intermediate_unchanged"]
                .as_array()
                .into_iter()
                .flatten()
            {
                check_board(position, entry.as_str().unwrap(), true);
            }
            if let Some(delta) = expected["hand_delta"].as_object() {
                for (key, n) in delta {
                    let p: Vec<_> = key.split(':').collect();
                    let owner = side(p[0]);
                    let piece = face(p[1]);
                    assert_eq!(
                        i64::from(position.hand(owner).count(piece))
                            - i64::from(g.position().hand(owner).count(piece)),
                        n.as_i64().unwrap(),
                        "{id}"
                    );
                }
            }
            if let Some(turn) = expected["side_to_move"].as_str() {
                assert_eq!(position.side_to_move(), side(turn));
            }
            if expected.get("conclusion").is_some() {
                assert_eq!(
                    serde_json::to_value(current(&c).game.conclusion()).unwrap(),
                    expected["conclusion"],
                    "{id}"
                );
            }
            if let Some(delta) = expected["board_ply_delta"].as_u64() {
                assert_eq!(
                    current(&c).game.snapshot().board_ply - g.snapshot().board_ply,
                    delta as usize
                );
            }
            let after = native(&c);
            let stale_context = context(&c, 1000, true);
            c.session.tori_mut().unwrap().pending = Some(stale_context.clone());
            send(&mut c, SessionOperation::Undo {}).unwrap();
            assert_eq!(native(&c), before_native, "{id} exact semantic undo");
            assert_eq!(current(&c).document.retired_branches().count(), 1);
            assert!(!submit_observation(&mut c, response(stale_context)));
            // Complete action reapplication uses new authority, not an old request.
            send(
                &mut c,
                SessionOperation::Tori {
                    actor,
                    command: command(&v["command"]),
                },
            )
            .unwrap();
            assert_eq!(
                native(&c),
                after,
                "{id} replay same authoritative transition"
            );
            ValidatedToriDocument::decode(current(&c).document.bytes(), &mut || false).unwrap();
        }
        ids.push(id.to_string());
    }
    assert_eq!(ids.len(), 70);
    if let Ok(path) = std::env::var("MSH_TORI_131_ADMISSION_REPORT") {
        fs::write(path,serde_json::to_vec_pretty(&json!({"boundary":"actual DesktopController, not GUI", "executed":ids.len(),"passed":ids.len(),"fixture_ids":ids})).unwrap()).unwrap();
    }
}

#[test]
fn history_prefix_obligations_takeback_and_reopen_are_exact_and_fresh() {
    let v = given("TORI-C-HISTORY");
    let g = sequence(&v["record_B"]);
    let a = sequence(&v["record_A"]);
    let mut c = controller();
    replace(&mut c, &a, t::Side::Sente);
    let record_a = native(&c);
    let board_a = current(&c).game.repetition_key().clone();
    send(
        &mut c,
        SessionOperation::Tori {
            actor: t::Side::Sente,
            command: t::GameCommand::Play {
                action: action(&v["record_B"]["actions"][0]),
            },
        },
    )
    .unwrap();
    send(&mut c, SessionOperation::Undo {}).unwrap();
    assert_eq!(native(&c), record_a);

    let old = context(&c, 100, true);
    c.session.tori_mut().unwrap().pending = Some(old.clone());
    replace(&mut c, &g, t::Side::Sente);
    assert!(!submit_observation(&mut c, response(old.clone())));
    assert_eq!(current(&c).game.repetition_key(), &board_a);
    assert_ne!(native(&c), record_a);
    assert_eq!(current(&c).game.obligation().unwrap().side, t::Side::Sente);
    let before = exact(&c);
    assert!(
        send(
            &mut c,
            SessionOperation::Tori {
                actor: t::Side::Sente,
                command: t::GameCommand::Play {
                    action: action(&v["record_B"]["actions"][0])
                }
            }
        )
        .is_err()
    );
    assert_eq!(exact(&c), before);
    send(&mut c, SessionOperation::Inspect { event_index: 4 }).unwrap();
    assert_eq!(current(&c).selected().occurrences(), [0, 4]);
    assert!(current(&c).selected().obligation().is_none());
    assert_eq!(current(&c).selected().events().len(), 4);
    assert_eq!(current(&c).game.events().len(), 8);
    let selected = c.shared_token();
    let inspected = c
        .shared_inspection(MovementInspectionQuery {
            expected: selected,
            origin: "4g".into(),
        })
        .unwrap();
    assert!(!inspected.choices.is_empty());
    assert_eq!(c.shared_token(), selected);
    assert!(send(&mut c, SessionOperation::Undo {}).is_err());
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert_eq!(current(&c).game.occurrences(), [0, 4, 8]);
    assert!(!submit_observation(&mut c, response(old.clone())));
    send(&mut c, SessionOperation::Undo {}).unwrap();
    assert!(current(&c).game.obligation().is_none());
    assert_eq!(current(&c).game.events().len(), 7);
    // One atomic human decision restores event6, retaining complete8/7 histories.
    send(&mut c, SessionOperation::TakeBack {}).unwrap();
    assert_eq!(current(&c).game.events().len(), 6);
    assert_eq!(current(&c).document.retired_branches().count(), 2);
    let dir = directory("history");
    let path = dir.join("tori.mshogi");
    let id = c.queue_shared_save(path.clone()).unwrap();
    assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Complete));
    assert!(!c.shared_document().dirty);
    let expected = native(&c);
    let token = c.shared_token();
    let id = c.queue_shared_open(path, c.shared_open_token()).unwrap();
    let reopened = wait_file(&mut c, id);
    assert!(matches!(reopened.io.phase, IoPhase::Complete));
    assert_eq!(native(&c), expected);
    assert_eq!(current(&c).document.retired_branches().count(), 2);
    assert!(reopened.paused);
    assert!(c.session.version > token.version);
    assert!(c.session.selection_id > token.selection);
    assert!(current(&c).pending.is_none());
    assert!(!submit_observation(&mut c, response(old)));
    for op in [
        SessionOperation::Resume {},
        SessionOperation::Retry {},
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 100,
            },
        },
    ] {
        let before = exact(&c);
        assert!(send(&mut c, op).is_err());
        assert_eq!(exact(&c), before);
        assert!(current(&c).pending.is_none());
    }
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn all_identity_dimensions_wrong_game_and_return_to_same_diagram_reject() {
    let mut c = controller();
    replace(&mut c, &t::Game::standard(), t::Side::Sente);
    let good = context(&c, 100, true);
    c.session.tori_mut().unwrap().pending = Some(good.clone());
    let mut bad = vec![];
    let mut x = good.clone();
    x.request_id += 1;
    bad.push(x);
    let mut x = good.clone();
    x.session_version += 1;
    bad.push(x);
    let mut x = good.clone();
    x.process_generation += 1;
    bad.push(x);
    let mut x = good.clone();
    x.limits.nodes += 1;
    bad.push(x);
    let mut x = good.clone();
    x.purpose = ExecutionPurpose::EngineTurn {};
    bad.push(x);
    let mut x = good.clone();
    if let ExecutionPurpose::MoveAnalysis { target } = &mut x.purpose {
        target.selection_id += 1;
    }
    bad.push(x);
    let mut x = good.clone();
    x.game = CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new()));
    bad.push(x);
    let mut g = t::Game::standard();
    let a = g.legal_actions()[0];
    play(&mut g, a);
    let mut x = good.clone();
    x.game = CanonicalRecord::tori(t::GameRecord::from_game(&g).unwrap());
    bad.push(x);
    for field in 0..5 {
        let mut x = good.clone();
        if let ExecutionPurpose::MoveAnalysis { target } = &mut x.purpose {
            match field {
                0 => target.event_index += 1,
                1 => target.board_ply += 1,
                2 => target.session_version += 1,
                3 => target.turn = NativeSide::Tori(t::Side::Gote),
                _ => target.label.push_str(" changed"),
            }
        }
        bad.push(x);
    }
    let before = exact(&c);
    for x in bad {
        assert!(!submit_observation(&mut c, response(x)));
        assert_eq!(exact(&c), before);
        assert_eq!(current(&c).pending.as_ref(), Some(&good));
    }
    send(
        &mut c,
        SessionOperation::Tori {
            actor: t::Side::Sente,
            command: t::GameCommand::Play { action: a },
        },
    )
    .unwrap();
    send(&mut c, SessionOperation::Undo {}).unwrap();
    assert_eq!(
        native(&c),
        t::GameRecord::from_game(&t::Game::standard()).unwrap()
    );
    assert!(!submit_observation(&mut c, response(good.clone())));
    let version = c.session.version;
    let selection = c.session.selection_id;
    for field in 0..4 {
        let mut token = c.shared_token();
        match field {
            0 => token.document -= 1,
            1 => token.version -= 1,
            2 => token.revision -= 1,
            _ => token.selection -= 1,
        };
        assert!(
            c.shared_command(SharedDesktopCommand {
                expected: token,
                game: GameKind::Tori,
                operation: SessionOperation::Undo {}
            })
            .is_err()
        );
    }
    assert_eq!(c.session.version, version);
    assert_eq!(c.session.selection_id, selection);
    c.session.tori_mut().unwrap().pending = Some(context(&c, 101, true));
    send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert!(current(&c).pending.is_none());
    assert!(!submit_observation(&mut c, response(good)));
}

#[test]
fn distinct_administrative_stops_and_unsupported_commands_are_unscored() {
    for reason in [
        t::StopReason::Cancelled,
        t::StopReason::ResourceLimit,
        t::StopReason::EngineFailure,
        t::StopReason::WorkLimit,
    ] {
        let mut c = controller();
        let g = t::Game::from_root(root(&given("TORI-O-ADMIN")["root"])).unwrap();
        replace(&mut c, &g, t::Side::Sente);
        let before = current(&c).game.position().state().clone();
        send(
            &mut c,
            SessionOperation::Tori {
                actor: t::Side::Sente,
                command: t::GameCommand::Stop { reason },
            },
        )
        .unwrap();
        assert_eq!(current(&c).game.snapshot().incomplete, Some(reason));
        assert!(current(&c).game.conclusion().is_none());
        assert_eq!(current(&c).game.position().state(), &before);
        let s = c.shared_snapshot();
        assert!(s.status.score_half_points.is_none());
        assert!(!s.status.concluded);
        assert!(s.search_observations.is_empty());
        let before = exact(&c);
        let native = native(&c);
        for command in [t::GameCommand::AgreeDraw, t::GameCommand::DeclareImpasse] {
            assert!(
                send(
                    &mut c,
                    SessionOperation::Tori {
                        actor: t::Side::Sente,
                        command
                    }
                )
                .is_err()
            );
            assert_eq!(exact(&c), before);
        }
        assert_eq!(self::native(&c), native);
    }
}
fn raw_root(v: &Value) -> Value {
    let board: Vec<_> = v["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let p: Vec<_> = v.as_str().unwrap().split(':').collect();
            json!({"square":p[2],"piece":{"side":side(p[0]),"face":face(p[1])}})
        })
        .collect();
    let hands: Vec<Vec<_>> = ["s", "g"]
        .iter()
        .map(|owner| {
            t::Face::HANDS
                .iter()
                .map(|f| v["hands"][owner][f.code()].as_u64().unwrap_or(0))
                .collect()
        })
        .collect();
    json!({"root_kind":"diagram","source":null,"earlier_history_unknown":true,"historical_reachability_proven":false,"declared_completed_plies":v["declared_completed_plies"],"accepted_plies":0,"declared_total_plies":v["declared_completed_plies"],"side_to_move":v["side_to_move"],"board":board,"hands":hands})
}
#[test]
fn seven_reject_fixtures_actual_open_preserves_live_document_and_execution() {
    let dir = directory("reject");
    let path = dir.join("bad.mshogi");
    let mut c = controller();
    replace(&mut c, &t::Game::standard(), t::Side::Sente);
    let action = current(&c).game.legal_actions()[0];
    send(
        &mut c,
        SessionOperation::Tori {
            actor: t::Side::Sente,
            command: t::GameCommand::Play { action },
        },
    )
    .unwrap();
    send(
        &mut c,
        SessionOperation::ToriPreferences {
            settings: crate::tori_document::ToriSettings {
                orientation: t::Side::Gote,
            },
        },
    )
    .unwrap();
    let observed = context(&c, 400, true);
    c.session.tori_mut().unwrap().pending = Some(observed.clone());
    assert!(submit_observation(&mut c, response(observed)));
    let pending = context(&c, 500, true);
    c.session.tori_mut().unwrap().pending = Some(pending.clone());
    let before = exact(&c);
    let settings = current(&c).document.settings().clone();
    let token = c.shared_open_token();
    let template: Value = serde_json::from_slice(&before).unwrap();
    let mut ids = vec![];
    for f in fixtures()["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["id"].as_str().unwrap().starts_with("TORI-REJECT"))
    {
        let id = f["id"].as_str().unwrap();
        let mut raws = vec![];
        if id == "TORI-REJECT-STRICT" {
            for raw in f["given"]["raw_payloads"].as_array().unwrap() {
                let raw = raw.as_str().unwrap();
                let text = serde_json::to_string(&template).unwrap();
                let current = serde_json::to_string(&template["payload"]["current"]).unwrap();
                raws.push(text.replacen(&current, raw, 1));
            }
        } else {
            let mut v = template.clone();
            for location in ["initial", "current"] {
                v["payload"][location]["payload"]["root"] = raw_root(&f["given"]["root"]);
                v["payload"][location]["payload"]["current_position"] =
                    raw_root(&f["given"]["root"]);
            }
            raws.push(v.to_string());
        }
        for raw in raws {
            fs::write(&path, raw).unwrap();
            let id = c
                .queue_shared_open(path.clone(), c.shared_open_token())
                .unwrap();
            let result = wait_file(&mut c, id);
            assert!(
                matches!(result.io.phase, IoPhase::Failed),
                "{id} {result:?}"
            );
            assert_eq!(exact(&c), before);
            assert_eq!(current(&c).document.settings(), &settings);
            assert_eq!(current(&c).pending.as_ref(), Some(&pending));
            assert_eq!(c.shared_open_token(), token);
        }
        ids.push(id.to_string());
    }
    assert_eq!(ids.len(), 7);
    let mut timed = template.clone();
    timed["payload"]["mode"] = json!("timed");
    let mut profile = template.clone();
    profile["profile_edition"] = json!("unsupported edition");
    let mut forged = template.clone();
    forged["payload"]["current"]["payload"]["conclusion"] =
        json!({"kind":"resignation","winner":"sente"});
    for raw in [
        timed.to_string(),
        profile.to_string(),
        forged.to_string(),
        "{truncated".into(),
    ] {
        fs::write(&path, raw).unwrap();
        let id = c
            .queue_shared_open(path.clone(), c.shared_open_token())
            .unwrap();
        assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Failed));
        assert_eq!(exact(&c), before);
        assert_eq!(current(&c).pending.as_ref(), Some(&pending));
    }
    // Actual raw command parser also rejects nested duplicates/missing/unknown fields.
    let valid = serde_json::to_string(&SharedDesktopCommand {
        expected: c.shared_token(),
        game: GameKind::Tori,
        operation: SessionOperation::Tori {
            actor: t::Side::Sente,
            command: t::GameCommand::Stop {
                reason: t::StopReason::Cancelled,
            },
        },
    })
    .unwrap();
    assert!(SharedDesktopCommand::from_json(&valid).is_ok());
    for raw in [
        valid.replace(
            "\"reason\":\"cancelled\"",
            "\"reason\":\"cancelled\",\"reason\":\"work_limit\"",
        ),
        valid.replace("\"actor\":\"sente\"", "\"actor\":\"sente\",\"extra\":1"),
        valid.replace("\"actor\":\"sente\",", ""),
    ] {
        assert!(SharedDesktopCommand::from_json(&raw).is_err(), "{raw}");
    }
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn capacity_256_to257_and_reserved_close_save_reopen_do_not_reset_usage() {
    let mut c = controller();
    replace(&mut c, &t::Game::standard(), t::Side::Sente);
    let mut wire: Value = serde_json::from_slice(&exact(&c)).unwrap();
    let observations: Vec<_> = (1..=256)
        .map(|n| serde_json::to_value(response(context(&c, n, true))).unwrap())
        .collect();
    wire["payload"]["observations"] = json!(observations);
    wire["payload"]["controls"] = json!(
        (0..256)
            .map(|index| json!({"kind":"observation","index":index}))
            .collect::<Vec<_>>()
    );
    wire["payload"]["retired_execution_floor"] = json!(1000);
    let dir = directory("capacity");
    let path = dir.join("full.mshogi");
    fs::write(&path, wire.to_string()).unwrap();
    let id = c
        .queue_shared_open(path.clone(), c.shared_open_token())
        .unwrap();
    assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Complete));
    assert_eq!(current(&c).document.observations().len(), 256);
    let before = exact(&c);
    let pending = context(&c, 2000, true);
    c.session.tori_mut().unwrap().pending = Some(pending.clone());
    assert!(!submit_observation(&mut c, response(pending)));
    assert_eq!(exact(&c), before);
    assert!(c.session.capacity.resource.is_some());
    assert!(current(&c).game.conclusion().is_none());
    send(
        &mut c,
        SessionOperation::ToriPreferences {
            settings: crate::tori_document::ToriSettings {
                orientation: t::Side::Gote,
            },
        },
    )
    .unwrap();
    c.lifecycle.request_close().unwrap();
    let snapshot = c.application_pause_for_close();
    assert!(snapshot.close_requires_save);
    assert!(snapshot.paused);
    assert!(current(&c).pending.is_none());
    let close_token = c.shared_open_token();
    let controls = current(&c).document.controls().len();
    c.application_pause_for_close();
    assert_eq!(c.shared_open_token(), close_token);
    assert_eq!(current(&c).document.controls().len(), controls);
    let id = c.queue_shared_save(path.clone()).unwrap();
    let saved = wait_file(&mut c, id);
    assert!(matches!(saved.io.phase, IoPhase::Complete));
    assert!(!saved.document.dirty);
    c.lifecycle
        .approve_close(saved.open_token, c.shared_open_token())
        .unwrap();
    assert!(c.lifecycle.close_approved());
    let mut reopened = controller();
    let id = reopened
        .queue_shared_open(path, reopened.shared_open_token())
        .unwrap();
    let s = wait_file(&mut reopened, id);
    assert!(matches!(s.io.phase, IoPhase::Complete));
    assert_eq!(current(&reopened).document.observations().len(), 256);
    assert!(s.paused);
    assert!(!s.capabilities.engine_turn);
    assert!(!s.capabilities.move_analysis);
    assert!(!s.capabilities.chess_full);
    assert!(current(&reopened).pending.is_none());
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn stale_and_cancelled_open_and_save_receipts_never_replace_or_clean_newer_content() {
    let mut c = controller();
    replace(&mut c, &t::Game::standard(), t::Side::Sente);
    let dir = directory("lifecycle");
    let path = dir.join("current.mshogi");
    let id = c.queue_shared_save(path.clone()).unwrap();
    assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Complete));
    let disk = fs::read(&path).unwrap();
    let initial = c.shared_open_token();
    let before = exact(&c);
    assert!(!c.close_requires_save());
    c.lifecycle.request_close().unwrap();
    let s = c.application_pause_for_close();
    assert!(!s.close_requires_save);
    assert!(!c.application_pause_for_close().close_requires_save);
    assert!(s.document.dirty);
    assert!(
        !c.lifecycle
            .approve_close(initial, c.shared_open_token())
            .is_ok()
    );
    c.lifecycle.cancel_close().unwrap();
    let id = c
        .queue_shared_open(path.clone(), c.shared_open_token())
        .unwrap();
    c.retire_native_files().unwrap();
    assert_ne!(c.io_state.id, id);
    std::thread::sleep(Duration::from_millis(30));
    c.shared_snapshot();
    assert_eq!(c.session.document_epoch, initial.document);
    let id = c
        .queue_shared_open(path.clone(), c.shared_open_token())
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    let completed = loop {
        if let Some((actual, result)) = c.files.poll() {
            assert_eq!(actual, id);
            break result.unwrap();
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    };
    send(
        &mut c,
        SessionOperation::ToriPreferences {
            settings: crate::tori_document::ToriSettings {
                orientation: t::Side::Gote,
            },
        },
    )
    .unwrap();
    let after = exact(&c);
    let persistence::Completed::Native(completed) = completed else {
        panic!("native worker completion")
    };
    assert!(c.finish_native_file(*completed).is_err());
    assert_eq!(exact(&c), after);
    let failed = c
        .queue_shared_save(dir.join("missing-parent").join("no.mshogi"))
        .unwrap();
    assert!(matches!(
        wait_file(&mut c, failed).io.phase,
        IoPhase::Failed
    ));
    assert!(c.shared_document().dirty);
    assert_eq!(fs::read(&path).unwrap(), disk);
    let rev = c.session.document_revision;
    let identity = c.session.document_identity;
    send(
        &mut c,
        SessionOperation::ToriPreferences {
            settings: crate::tori_document::ToriSettings {
                orientation: t::Side::Sente,
            },
        },
    )
    .unwrap();
    c.finish_native_file(archive::NativeCompleted::Saved {
        path: path.clone(),
        revision: rev,
        identity,
    })
    .unwrap();
    assert!(c.shared_document().dirty);
    let id = c
        .queue_shared_open(path.clone(), c.shared_open_token())
        .unwrap();
    assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Complete));
    assert_eq!(
        native(&c),
        ValidatedToriDocument::decode(&before, &mut || false)
            .unwrap()
            .record()
            .clone()
    );
    assert!(c.queue_product_open(path, c.shared_open_token()).is_ok());
    let id = c.io_state.id;
    assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Failed));
    assert!(
        c.shared_new_game(
            c.shared_open_token(),
            GameKind::Tori,
            NativeSide::Tori(t::Side::Sente)
        )
        .is_err()
    );
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn history_identity_distinguishes_native_owners_hands_handed_faces_and_counter_provenance() {
    let given = given("TORI-H-IDENTITY");
    let base = t::Game::from_root(root(&given["base"])).unwrap();
    let mut c = controller();
    replace(&mut c, &base, t::Side::Sente);
    let key = current(&c).game.repetition_key().clone();
    let canonical = native(&c);
    for diff in given["differences"].as_array().unwrap() {
        let mut v = given["base"].clone();
        if let Some(turn) = diff.get("side_to_move") {
            v["side_to_move"] = turn.clone();
        }
        if let Some(hands) = diff.get("hands") {
            v["hands"] = hands.clone();
        }
        if let Some(pieces) = diff["replace_piece"].as_array() {
            let mut keys = vec![];
            let mut structural_keys = vec![];
            for p in pieces {
                let mut r = v.clone();
                r["board"].as_array_mut().unwrap().push(p.clone());
                // Exact fixture diagram proves structural key distinction, while
                // Sente-to-move quail attacks the nonmoving royal and cannot be live.
                let structural = root(&r);
                assert!(t::Game::from_root(structural.clone()).is_err());
                let unchanged = exact(&c);
                let structural_key = t::RepetitionKey::from_position(&structural);
                assert_ne!(structural_key, key);
                structural_keys.push(structural_key);
                let dir = directory("structural-key");
                let path = dir.join("not-a-live-root.mshogi");
                let mut invalid: Value = serde_json::from_slice(&unchanged).unwrap();
                for location in ["initial", "current"] {
                    invalid["payload"][location]["payload"]["root"] = raw_root(&r);
                    invalid["payload"][location]["payload"]["current_position"] = raw_root(&r);
                }
                fs::write(&path, invalid.to_string()).unwrap();
                let id = c.queue_shared_open(path, c.shared_open_token()).unwrap();
                assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Failed));
                assert_eq!(exact(&c), unchanged);
                fs::remove_dir_all(dir).unwrap();
                // Explicit supplementary admitted position: checked side has move.
                r["side_to_move"] = json!("gote");
                let game = t::Game::from_root(root(&r)).unwrap();
                replace(&mut c, &game, t::Side::Sente);
                keys.push(current(&c).game.repetition_key().clone());
            }
            assert_ne!(keys[0], keys[1]);
            assert_ne!(structural_keys[0], structural_keys[1]);
            continue;
        }
        let game = t::Game::from_root(root(&v)).unwrap();
        replace(&mut c, &game, t::Side::Sente);
        assert_ne!(current(&c).game.repetition_key(), &key);
        assert_ne!(native(&c), canonical);
    }
    let mut counter = given["base"].clone();
    counter["declared_completed_plies"] = json!(99);
    let game = t::Game::from_root(root(&counter)).unwrap();
    replace(&mut c, &game, t::Side::Sente);
    assert_eq!(current(&c).game.repetition_key(), &key);
    assert_ne!(native(&c), canonical);
    let before = current(&c).game.repetition_key().clone();
    let ctx = context(&c, 1000, true);
    c.session.tori_mut().unwrap().pending = Some(ctx.clone());
    assert!(submit_observation(&mut c, response(ctx)));
    assert_eq!(current(&c).game.repetition_key(), &before);
}
#[test]
fn byte_control_native_event_and_replay_limited_open_preserves_current_authority() {
    let mut c = controller();
    replace(&mut c, &t::Game::standard(), t::Side::Sente);
    let before = exact(&c);
    let token = c.shared_open_token();
    let pending = context(&c, 900, true);
    c.session.tori_mut().unwrap().pending = Some(pending.clone());
    let dir = directory("resources");
    let path = dir.join("refused.mshogi");
    let template: Value = serde_json::from_slice(&before).unwrap();
    let settings = json!({"kind":"settings","settings":{"orientation":"sente"}});
    let mut controls = template.clone();
    controls["payload"]["controls"] = json!(vec![settings; 8190]);
    let mut g = t::Game::standard();
    let a = g.legal_actions()[0];
    play(&mut g, a);
    let record = CanonicalRecord::tori(t::GameRecord::from_game(&g).unwrap());
    let mut events = template.clone();
    events["payload"]["current"] = serde_json::to_value(record).unwrap();
    let event = events["payload"]["current"]["payload"]["events"][0].clone();
    events["payload"]["current"]["payload"]["events"] = json!(vec![event.clone(); 513]);
    let mut work = events.clone();
    work["payload"]["current"]["payload"]["events"] = json!(vec![event; 400]);
    // These deliberately malformed over-limit inputs must be bounded before native replay.
    for (bytes, category) in [
        (
            vec![b' '; MAX_ARCHIVE_BYTES + 1],
            ResourceCategory::EncodedDocumentBytes,
        ),
        (
            controls.to_string().into_bytes(),
            ResourceCategory::Controls,
        ),
        (
            events.to_string().into_bytes(),
            ResourceCategory::NativeEvents,
        ),
        (work.to_string().into_bytes(), ResourceCategory::ReplayWork),
    ] {
        fs::write(&path, bytes).unwrap();
        let id = c
            .queue_shared_open(path.clone(), c.shared_open_token())
            .unwrap();
        let result = wait_file(&mut c, id);
        assert!(matches!(result.io.phase, IoPhase::Failed));
        let failure = result.io.resource.expect("typed resource fact");
        assert!(
            failure.facts().iter().any(|f| f.category == category),
            "{category:?}: {failure:?}"
        );
        assert_eq!(exact(&c), before);
        assert_eq!(c.shared_open_token(), token);
        assert_eq!(current(&c).pending.as_ref(), Some(&pending));
    }
    let good = dir.join("still-saveable.mshogi");
    let id = c.queue_shared_save(good.clone()).unwrap();
    assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Complete));
    ValidatedToriDocument::decode(&fs::read(good).unwrap(), &mut || false).unwrap();
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn measured_long_native_runtime_admission_stop_end_save_and_prepared_close() {
    // Same documented deterministic native history generator as MSH-129/130;
    // this measures the additional actual controller/worker boundary, not new rules.
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
            match g.submit(t::GameRequest {
                expected: g.context(),
                actor: g.position().side_to_move(),
                command: t::GameCommand::Play { action: a },
            }) {
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
        );
    }

    let generated_ms = start.elapsed().as_millis();
    assert_eq!(g.events().len(), 360);
    assert_eq!(g.capacity_usage().unwrap().replay_work, 65341);
    let mut c = controller();
    let start = Instant::now();
    replace(&mut c, &g, t::Side::Sente);
    let installation_ms = start.elapsed().as_millis();
    let initial_usage = current(&c).document.usage().clone();
    let next = current(&c).game.legal_actions()[0];
    let prior = exact(&c);
    assert!(
        send(
            &mut c,
            SessionOperation::Tori {
                actor: t::Side::Sente,
                command: t::GameCommand::Play { action: next }
            }
        )
        .is_err()
    );
    assert_eq!(exact(&c), prior);
    let start = Instant::now();
    send(
        &mut c,
        SessionOperation::Tori {
            actor: t::Side::Sente,
            command: t::GameCommand::Stop {
                reason: t::StopReason::ResourceLimit,
            },
        },
    )
    .unwrap();
    let stop_ms = start.elapsed().as_millis();
    let start = Instant::now();
    send(&mut c, SessionOperation::Resign {}).unwrap();
    let resign_ms = start.elapsed().as_millis();
    let start = Instant::now();
    let snapshot = c.application_pause_for_close();
    let close_snapshot_ms = start.elapsed().as_millis();
    assert!(snapshot.status.concluded);
    assert_eq!(current(&c).game.events().len(), 362);
    let dir = directory("long");
    let path = dir.join("complete.mshogi");
    let start = Instant::now();
    let id = c.queue_shared_save(path.clone()).unwrap();
    let enqueue_save_ms = start.elapsed().as_millis();
    let result = wait_file(&mut c, id);
    assert!(matches!(result.io.phase, IoPhase::Complete));
    let save_ms = start.elapsed().as_millis();
    let readback = ValidatedToriDocument::decode(&fs::read(&path).unwrap(), &mut || false).unwrap();
    assert_eq!(readback.record(), &native(&c));
    let report = json!({"boundary":"actual DesktopController+existing bounded worker", "seed":"0x129ca11ab1e5", "generated_ms":generated_ms,"installation_ms":installation_ms,"stop_ms":stop_ms,"resign_ms":resign_ms,"close_and_snapshot_ms":close_snapshot_ms,"enqueue_save_ms":enqueue_save_ms,"save_ms":save_ms,"initial_usage":initial_usage,"final_usage":current(&c).document.usage(),"saved_usage":readback.usage(),"saved_floor":readback.retired_execution_floor(),"document_budget":262144,"extra_native_replays_per_operation_upper_bound":4,"extra_replay_units_upper_bound":4*(65536+128)+512,"worker_deadline_seconds":15});
    eprintln!("MSH131_LONG {}", report);
    if let Ok(path) = std::env::var("MSH_TORI_131_LONG_REPORT") {
        fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn typed_identity_refusal_and_interrupted_save_keep_complete_accepted_document_dirty() {
    let mut c = controller();
    replace(&mut c, &t::Game::standard(), t::Side::Sente);
    let pending = context(&c, 700, true);
    c.session.tori_mut().unwrap().pending = Some(pending.clone());
    let before = exact(&c);
    c.session.capacity.id = MAX_SAFE_ID - 64;
    assert!(
        send(
            &mut c,
            SessionOperation::ToriPreferences {
                settings: crate::tori_document::ToriSettings {
                    orientation: t::Side::Gote
                }
            }
        )
        .is_err()
    );
    assert_eq!(exact(&c), before);
    assert_eq!(current(&c).pending.as_ref(), Some(&pending));
    assert!(
        c.session
            .capacity
            .resource
            .as_ref()
            .unwrap()
            .facts()
            .iter()
            .any(|f| f.category == ResourceCategory::IdentityHeadroom)
    );
    // Actual shared Save execution, interrupted after the exclusive temporary file
    // exists. No synthetic Saved receipt is produced or accepted by the controller.
    let dir = directory("interrupted");
    let path = dir.join("current.mshogi");
    fs::write(&path, &before).unwrap();
    let work = archive::NativeWork::ToriSave {
        document: current(&c).document.clone(),
        floor: c.runtime_floor(),
        path: path.clone(),
        revision: c.session.document_revision,
        identity: c.session.document_identity,
    };
    let mut observed_temporary = false;
    let result = archive::execute(work, &mut || {
        let has_tmp = fs::read_dir(&dir).unwrap().any(|e| {
            e.unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with(".mshogi-save-")
        });
        observed_temporary |= has_tmp;
        has_tmp
    });
    assert!(result.is_err());
    assert!(observed_temporary);
    assert_eq!(fs::read(&path).unwrap(), before);
    assert!(c.shared_document().dirty);
    assert!(c.shared_document().path.is_none());
    assert_eq!(exact(&c), before);
    assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn forced8_conclusion_and_unequal16_obligation_are_derived_by_final_controller_action() {
    for id in ["TORI-H-FORCED-FULL", "TORI-H-UNEQUAL-FULL"] {
        let v = given(id);
        let full = sequence(&v);
        let count = full.events().len();
        let prefix = full.record_prefix(count - 1).unwrap().to_game().unwrap();
        let actor = prefix.position().side_to_move();
        let mut c = controller();
        replace(&mut c, &prefix, actor);
        let action = action(v["actions"].as_array().unwrap().last().unwrap());
        let after = send(
            &mut c,
            SessionOperation::Tori {
                actor,
                command: t::GameCommand::Play { action },
            },
        )
        .unwrap();
        assert_eq!(native(&c), t::GameRecord::from_game(&full).unwrap());
        assert_eq!(after.event_index, count);
        assert_eq!(after.board_ply, count as u64);
        if count == 8 {
            assert_eq!(
                current(&c).game.conclusion(),
                Some(t::Conclusion::RepetitionForfeit {
                    loser: t::Side::Sente
                })
            );
            assert!(after.status.concluded);
            assert_eq!(after.status.score_half_points, Some([0, 2]));
        } else {
            assert!(current(&c).game.conclusion().is_none());
            assert_eq!(current(&c).game.occurrences(), [0, 4, 10, 16]);
            assert_eq!(current(&c).game.obligation().unwrap().side, t::Side::Sente);
            assert!(!after.status.concluded);
        }
        let live = native(&c);
        send(&mut c, SessionOperation::Inspect { event_index: 4 }).unwrap();
        assert_eq!(current(&c).selected().occurrences(), [0, 4]);
        assert!(current(&c).selected().obligation().is_none());
        assert!(current(&c).selected().conclusion().is_none());
        send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert_eq!(native(&c), live);
        send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(native(&c), t::GameRecord::from_game(&prefix).unwrap());
        assert_eq!(current(&c).document.retired_branches().count(), 1);
    }
}
#[test]
fn historical_failure_observation_saves_exact_prefix_without_future_knowledge_or_pending_work() {
    let g = sequence(&given("TORI-C-HISTORY")["record_B"]);
    let mut c = controller();
    replace(&mut c, &g, t::Side::Sente);
    send(&mut c, SessionOperation::Inspect { event_index: 4 }).unwrap();
    let prefix = current(&c).selected().record_prefix(4).unwrap();
    let ctx = context(&c, 1000, true);
    c.session.tori_mut().unwrap().pending = Some(ctx.clone());
    assert!(submit_observation(&mut c, response(ctx.clone())));
    assert_eq!(
        current(&c).document.observations()[0].context.game,
        CanonicalRecord::tori(prefix.clone())
    );
    let dir = directory("historical-observation");
    let path = dir.join("target.mshogi");
    let id = c.queue_shared_save(path.clone()).unwrap();
    assert!(matches!(wait_file(&mut c, id).io.phase, IoPhase::Complete));
    let id = c.queue_shared_open(path, c.shared_open_token()).unwrap();
    let restored = wait_file(&mut c, id);
    assert!(matches!(restored.io.phase, IoPhase::Complete));
    assert_eq!(restored.event_index, 8);
    assert!(!restored.historical);
    assert!(restored.paused);
    assert!(current(&c).pending.is_none());
    assert!(restored.observations[0].historical);
    let CanonicalRecord::Tori(target) = &restored.observations[0].observation.context.game else {
        panic!()
    };
    let target = target.to_game().unwrap();
    assert_eq!(target.occurrences(), [0, 4]);
    assert!(target.obligation().is_none());
    assert_eq!(target.events().len(), 4);
    send(&mut c, SessionOperation::Inspect { event_index: 4 }).unwrap();
    assert_eq!(
        t::GameRecord::from_game(current(&c).selected()).unwrap(),
        prefix
    );
    send(&mut c, SessionOperation::ReturnLive {}).unwrap();
    assert_eq!(current(&c).game.occurrences(), [0, 4, 8]);
    assert!(!submit_observation(&mut c, response(ctx)));
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn rejected_outgoing_replacements_preserve_tori_document_and_accepted_execution() {
    let mut c = controller();
    replace(&mut c, &t::Game::standard(), t::Side::Sente);
    let pending = context(&c, 600, true);
    c.session.tori_mut().unwrap().pending = Some(pending.clone());
    let before = exact(&c);
    let version = c.session.version;
    // Public Chess replacement refuses at admission capacity, after its old
    // implementation would already have retired the source execution.
    c.session.capacity.id = MAX_SAFE_ID - 33;
    assert!(
        send(
            &mut c,
            SessionOperation::Replace {
                record: CanonicalRecord::chess(
                    AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record()
                ),
                human: NativeSide::Chess(Color::White)
            }
        )
        .is_err()
    );
    assert_eq!(exact(&c), before);
    assert_eq!(current(&c).pending.as_ref(), Some(&pending));
    assert_eq!(c.session.version, version);
    c.session.capacity.id = 100;
    // Real existing worker paths with intentionally unsupported destination
    // preferences inject candidate failure before swap, not a fake completion.
    let receipt = c
        .queue_shogi_seed(
            archive::ShogiSeed::Record {
                record: Box::new(s::GameRecord::from_game(&s::Game::new())),
                human: s::Side::Sente,
                orient_human: false,
            },
            Some(SessionOperation::ShogiPreferences {
                settings: ShogiSettings {
                    orientation: s::Side::Gote,
                    analysis_defaults: WorkLimits {
                        depth: 99,
                        nodes: 100,
                    },
                },
            }),
            None,
            true,
        )
        .unwrap();
    assert!(settle(&mut c, receipt).is_err());
    assert_eq!(exact(&c), before);
    assert_eq!(current(&c).pending.as_ref(), Some(&pending));
    let game =
        mshogi_core::xiangqi::Game::standard(mshogi_core::xiangqi::ClaimPolicy::Explicit).unwrap();
    let record = mshogi_core::xiangqi::GameRecord::from_game(&game).unwrap();
    let draft =
        xiangqi::archive::XiangqiDraft::replacement(&record, mshogi_core::xiangqi::Side::Red);
    let receipt = c
        .queue_xiangqi_candidate(
            draft,
            Some(SessionOperation::XiangqiPreferences {
                settings: XiangqiSettings {
                    orientation: mshogi_core::xiangqi::Side::Black,
                    analysis_defaults: WorkLimits {
                        depth: 99,
                        nodes: 100,
                    },
                },
            }),
            None,
            None,
            true,
        )
        .unwrap();
    assert!(settle(&mut c, receipt).is_err());
    assert_eq!(exact(&c), before);
    assert_eq!(current(&c).pending.as_ref(), Some(&pending));
    assert_eq!(c.session.version, version);
}
