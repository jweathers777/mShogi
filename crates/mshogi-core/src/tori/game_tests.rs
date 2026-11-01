//! Independent native history, authority, replay, mutation and resource evidence.
use super::{
    conformance::{action, root, sq},
    game_conformance::{play, sequence, submit},
    *,
};
use mshogi_conformance::Corpus;
use serde_json::{Value, json};
fn fixture(id: &str) -> Value {
    let corpus = Corpus::embedded().unwrap();
    corpus
        .fixtures()
        .iter()
        .find(|f| f.id == id)
        .unwrap()
        .given
        .clone()
}
fn record(g: &Game) -> String {
    GameRecord::from_game(g).unwrap().to_json().into()
}
fn rejection(g: &mut Game, actor: Side, command: GameCommand, error: GameError) {
    let before = g.cache_snapshot();
    let context = g.context();
    let bytes = record(g);
    assert_eq!(submit(g, actor, command).unwrap_err(), error);
    assert_eq!(before, g.cache_snapshot());
    assert_eq!(context, g.context());
    assert_eq!(bytes, record(g));
}
#[test]
fn independent_forced8_and_unequal16_integrate_board_choices_history_and_precedence() {
    let mut forced = sequence(&fixture("TORI-H-FORCED-FULL"));
    assert_eq!(forced.occurrences(), [0, 4, 8]);
    assert_eq!(forced.obligation().unwrap().occurrences, [0, 4, 8]);
    assert_eq!(
        forced.conclusion(),
        Some(Conclusion::RepetitionForfeit { loser: Side::Sente })
    );
    assert!(forced.local_position().is_in_check(Side::Sente));
    let only = Action::Move {
        from: sq("4a"),
        to: sq("5a"),
        result_face: Face::Phoenix,
    };
    assert_eq!(forced.local_position().legal_actions(), [only]);
    assert!(forced.legal_actions().is_empty());
    rejection(
        &mut forced,
        Side::Sente,
        GameCommand::Play { action: only },
        GameError::Concluded,
    );
    let mut unequal = sequence(&fixture("TORI-H-UNEQUAL-FULL"));
    let at10 = unequal.event_prefix(10).unwrap();
    assert_eq!(at10.obligation.as_ref().unwrap().occurrences, [0, 4, 10]);
    assert_eq!(unequal.obligation().unwrap().occurrences, [4, 10, 16]);
    assert!(unequal.obligation().unwrap().permits(Action::Move {
        from: sq("4g"),
        to: sq("5g"),
        result_face: Face::Phoenix
    }));
    assert!(!at10.obligation.as_ref().unwrap().permits(Action::Move {
        from: sq("4g"),
        to: sq("5g"),
        result_face: Face::Phoenix
    }));
    assert!(
        play(
            &mut unequal,
            Action::Move {
                from: sq("4g"),
                to: sq("5g"),
                result_face: Face::Phoenix
            }
        )
        .is_ok()
    );
    for (id, expected) in [
        (
            "TORI-O-MATE",
            Conclusion::Checkmate {
                winner: Side::Sente,
            },
        ),
        (
            "TORI-O-QUIET",
            Conclusion::StalemateLoss { loser: Side::Gote },
        ),
    ] {
        let g = Game::from_root(root(&fixture(id)["root"]).unwrap()).unwrap();
        assert_eq!(g.conclusion(), Some(expected));
        assert_eq!(g.occurrences(), [0]);
        assert_eq!(g.events().len(), 0);
        assert_eq!(
            record(
                &GameRecord::from_json(&record(&g))
                    .unwrap()
                    .to_game()
                    .unwrap()
            ),
            record(&g)
        );
    }
}
#[test]
fn repetition_equality_and_semantic_context_are_distinct_and_root_counters_are_not_history() {
    let f = fixture("TORI-H-COUNT");
    let a = Game::from_root(root(&f["root"]).unwrap()).unwrap();
    let mut b = sequence(&f);
    assert_eq!(a.repetition_key(), b.repetition_key());
    assert!(!a.context().same_semantics(&b.context()));
    let forbidden = action(&f["actions"][0]);
    rejection(
        &mut b,
        Side::Sente,
        GameCommand::Play { action: forbidden },
        GameError::RepetitionDeparture,
    );
    let p = b.position();
    let entries = Square::all()
        .filter_map(|s| p.piece(s).map(|v| (s, v)))
        .collect::<Vec<_>>();
    let imported = Position::from_diagram(
        &entries,
        [p.hand(Side::Sente), p.hand(Side::Gote)],
        p.side_to_move(),
        RootMetadata::imported("independent diagram; earlier play absent".into(), 900).unwrap(),
    )
    .unwrap();
    let imported = Game::from_root(imported).unwrap();
    assert_eq!(imported.repetition_key(), b.repetition_key());
    assert_eq!(imported.occurrences(), [0]);
    assert!(imported.obligation().is_none());
    assert_eq!(imported.position().declared_total_plies(), 900);
    assert_ne!(record(&a), record(&imported));
}
#[test]
fn native_commands_are_atomic_out_of_turn_resignation_and_unscored_stops_preserve_board_identity() {
    let mut g = Game::standard();
    let before = record(&g);
    rejection(
        &mut g,
        Side::Sente,
        GameCommand::AgreeDraw,
        GameError::UnsupportedCommand,
    );
    rejection(
        &mut g,
        Side::Gote,
        GameCommand::DeclareImpasse,
        GameError::UnsupportedCommand,
    );
    let a = g.legal_actions()[0];
    rejection(
        &mut g,
        Side::Gote,
        GameCommand::Play { action: a },
        GameError::WrongActor,
    );
    rejection(
        &mut g,
        Side::Sente,
        GameCommand::Play {
            action: Action::Drop {
                face: Face::Phoenix,
                to: sq("4d"),
            },
        },
        GameError::Action(ActionError::InvalidDropFace),
    );
    for reason in [
        StopReason::Cancelled,
        StopReason::EngineFailure,
        StopReason::ResourceLimit,
        StopReason::WorkLimit,
    ] {
        let mut g = GameRecord::from_json(&before).unwrap().to_game().unwrap();
        let key = g.repetition_key().clone();
        let old = g.context();
        let mut u = submit(&mut g, Side::Gote, GameCommand::Stop { reason }).unwrap();
        assert_eq!(g.snapshot().board_ply, 0);
        assert_eq!(g.repetition_key(), &key);
        assert_eq!(g.occurrences(), [0]);
        assert!(g.conclusion().is_none());
        assert_eq!(g.snapshot().incomplete, Some(reason));
        assert_ne!(g.context(), old);
        assert_eq!(g.events()[0].effects, None);
        assert_eq!(g.events()[0].replay_work, 2);
        rejection(
            &mut g,
            Side::Gote,
            GameCommand::Stop { reason },
            GameError::AlreadyStopped,
        );
        assert_eq!(
            record(
                &GameRecord::from_json(&record(&g))
                    .unwrap()
                    .to_game()
                    .unwrap()
            ),
            record(&g)
        );
        let mut redo = g.undo(&mut u).unwrap();
        assert_eq!(record(&g), before);
        g.redo(&mut redo).unwrap();
        assert_eq!(g.snapshot().incomplete, Some(reason));
        submit(&mut g, Side::Gote, GameCommand::Resign).unwrap();
        assert_eq!(
            g.conclusion(),
            Some(Conclusion::Resignation {
                winner: Side::Sente
            })
        );
        assert_eq!(g.snapshot().board_ply, 0);
        assert_eq!(g.occurrences(), [0]);
        rejection(
            &mut g,
            Side::Sente,
            GameCommand::Resign,
            GameError::Concluded,
        );
    }
}
#[test]
fn exact_reversal_every_prefix_replay_and_selection_never_restore_native_authority() {
    let given = fixture("TORI-H-UNEQUAL-FULL");
    let mut g = Game::from_root(root(&given["root"]).unwrap()).unwrap();
    let mut undos = vec![];
    let mut snapshots = vec![g.cache_snapshot()];
    let mut records = vec![record(&g)];
    let stale = g.context();
    for a in given["actions"].as_array().unwrap() {
        undos.push(play(&mut g, action(a)).unwrap());
        snapshots.push(g.cache_snapshot());
        records.push(record(&g));
    }
    let live = g.context();
    for (i, bytes) in records.iter().enumerate() {
        let prefix = g.record_prefix(i).unwrap();
        assert_eq!(prefix.to_json(), bytes);
        let reconstructed = prefix.to_game().unwrap();
        assert_eq!(reconstructed.events().len(), i);
        assert_ne!(reconstructed.context(), g.context());
    }
    assert_eq!(g.record_prefix(9999).unwrap_err(), RecordError::Prefix);
    let live_snapshot = g.cache_snapshot();
    let prefix = g.select_prefix(4).unwrap();
    assert_eq!(prefix.occurrences, [0, 4]);
    assert!(prefix.obligation.is_none());
    assert_eq!(prefix.board_ply, 4);
    assert_eq!(g.cache_snapshot(), live_snapshot);
    assert_ne!(g.context(), live);
    assert_eq!(
        g.submit(GameRequest {
            expected: live,
            actor: Side::Sente,
            command: GameCommand::Resign
        })
        .unwrap_err(),
        GameError::StaleContext
    );
    let mut redos = vec![];
    for i in (0..undos.len()).rev() {
        redos.push(g.undo(&mut undos[i]).unwrap());
        assert_eq!(g.cache_snapshot(), snapshots[i]);
        assert_eq!(record(&g), records[i]);
    }
    assert!(stale.same_semantics(&g.context()));
    assert_ne!(stale, g.context());
    for (i, r) in (1..).zip(redos.iter_mut().rev()) {
        g.redo(r).unwrap();
        assert_eq!(g.cache_snapshot(), snapshots[i]);
    }
    assert_eq!(g.cache_snapshot(), live_snapshot);
    let restored = GameRecord::from_json(&record(&g))
        .unwrap()
        .to_game()
        .unwrap();
    assert_eq!(restored.cache_snapshot(), g.cache_snapshot());
    assert_ne!(restored.context(), g.context());
    let before = g.context();
    assert!(g.select_prefix(9999).is_err());
    assert_eq!(before, g.context());
    assert!(
        !g.local_position()
            .inspect(InspectionOrigin::Board(sq("4a")))
            .is_empty()
    );
}
#[test]
fn undo_redo_reject_foreign_used_out_of_order_and_retired_branches_atomically() {
    let mut a = Game::standard();
    let mut b = Game::standard();
    let action = a.legal_actions()[0];
    let mut u1 = play(&mut a, action).unwrap();
    let before = b.context();
    assert_eq!(b.undo(&mut u1).unwrap_err(), GameError::InvalidToken);
    assert_eq!(b.context(), before);
    let action2 = a.legal_actions()[0];
    let mut u2 = play(&mut a, action2).unwrap();
    let before = a.context();
    assert_eq!(a.undo(&mut u1).unwrap_err(), GameError::InvalidToken);
    assert_eq!(a.context(), before);
    let mut r2 = a.undo(&mut u2).unwrap();
    assert_eq!(a.undo(&mut u2).unwrap_err(), GameError::InvalidToken);
    let different = a
        .legal_actions()
        .into_iter()
        .find(|a| *a != action2)
        .unwrap();
    play(&mut a, different).unwrap();
    let before = a.context();
    assert_eq!(a.redo(&mut r2).unwrap_err(), GameError::InvalidToken);
    assert_eq!(a.context(), before);
}
fn rotate_action(a: Action) -> Action {
    match a {
        Action::Move {
            from,
            to,
            result_face,
        } => Action::Move {
            from: from.rotated(),
            to: to.rotated(),
            result_face,
        },
        Action::Drop { face, to } => Action::Drop {
            face,
            to: to.rotated(),
        },
    }
}
#[test]
fn native_rotation_metamorphism_preserves_repetition_responsibility_and_forced_loss() {
    for id in ["TORI-H-FORCED-FULL", "TORI-H-UNEQUAL-FULL"] {
        let f = fixture(id);
        let original = root(&f["root"]).unwrap();
        let entries = Square::all()
            .filter_map(|s| {
                original.piece(s).map(|p| {
                    (
                        s.rotated(),
                        Piece {
                            side: p.side.opponent(),
                            face: p.face,
                        },
                    )
                })
            })
            .collect::<Vec<_>>();
        let rotated = Position::from_diagram(
            &entries,
            [original.hand(Side::Gote), original.hand(Side::Sente)],
            original.side_to_move().opponent(),
            RootMetadata::diagram(0),
        )
        .unwrap();
        let mut a = Game::from_root(original).unwrap();
        let mut b = Game::from_root(rotated).unwrap();
        for v in f["actions"].as_array().unwrap() {
            let action = action(v);
            play(&mut a, action).unwrap();
            play(&mut b, rotate_action(action)).unwrap();
            assert_eq!(a.occurrences(), b.occurrences());
            assert_eq!(
                a.obligation().map(|o| o.side.opponent()),
                b.obligation().map(|o| o.side)
            );
            for square in Square::all() {
                assert_eq!(
                    a.position().piece(square).map(|p| Piece {
                        side: p.side.opponent(),
                        face: p.face
                    }),
                    b.position().piece(square.rotated())
                );
            }
        }
        assert_eq!(a.conclusion().is_some(), b.conclusion().is_some());
        assert_eq!(
            record(
                &GameRecord::from_json(&record(&b))
                    .unwrap()
                    .to_game()
                    .unwrap()
            ),
            record(&b)
        );
    }
}
#[test]
fn strict_record_all_fields_nested_duplicates_missing_tampering_and_resources() {
    let g = sequence(&fixture("TORI-H-UNEQUAL-FULL"));
    let good = record(&g);
    let original: Value = serde_json::from_str(&good).unwrap();
    let context = g.context();
    // EVERY canonical object field: remove it or duplicate its raw key. Parsing
    // happens before any Value mutation can erase malformed wire duplicates.
    fn visit(v: &Value, path: &mut Vec<String>, original: &Value) {
        match v {
            Value::Object(map) => {
                for (key, value) in map {
                    let mut changed = original.clone();
                    let mut cursor = &mut changed;
                    for p in path.iter() {
                        cursor = if cursor.is_array() {
                            &mut cursor[p.parse::<usize>().unwrap()]
                        } else {
                            &mut cursor[p]
                        };
                    }
                    cursor.as_object_mut().unwrap().remove(key);
                    assert!(
                        GameRecord::from_json(&changed.to_string()).is_err(),
                        "missing {path:?}/{key}"
                    );
                    path.push(key.clone());
                    visit(value, path, original);
                    path.pop();
                }
            }
            Value::Array(vs) => {
                for (i, v) in vs.iter().enumerate() {
                    path.push(i.to_string());
                    visit(v, path, original);
                    path.pop();
                }
            }
            _ => {}
        }
    }
    visit(&original, &mut vec![], &original);
    for key in [
        "format",
        "game",
        "profile",
        "edition",
        "schema_version",
        "root_kind",
        "source",
        "earlier_history_unknown",
        "side_to_move",
        "square",
        "piece",
        "side",
        "face",
        "index",
        "board_ply",
        "actor",
        "command",
        "kind",
        "action",
        "from",
        "to",
        "result_face",
        "effects",
        "captured",
        "occurrences",
        "obligation",
        "conclusion",
        "incomplete",
        "replay_work",
    ] {
        let needle = format!("\"{key}\":");
        if !good.contains(&needle) {
            continue;
        }
        let duplicate = good.replacen(&needle, &format!("\"{key}\":null,{needle}"), 1);
        assert!(
            GameRecord::from_json(&duplicate).is_err(),
            "duplicate {key}"
        );
    }
    for (key, value) in [
        ("game", json!("shogi")),
        ("profile", json!("mshogi.tori.bunsei11.v1")),
        ("edition", json!("new")),
        ("schema_version", json!(2)),
        ("mode", json!("timed")),
        ("invented", json!(true)),
    ] {
        let mut v = original.clone();
        v[key] = value;
        assert!(GameRecord::from_json(&v.to_string()).is_err());
    }
    for (index, key, value) in [
        (0, "board_ply", json!(0)),
        (9, "occurrences", json!([0, 10])),
        (9, "obligation", Value::Null),
        (
            15,
            "conclusion",
            json!({"kind":"resignation","winner":"gote"}),
        ),
        (3, "replay_work", json!(1)),
        (0, "effects", Value::Null),
    ] {
        let mut v = original.clone();
        v["events"][index][key] = value;
        assert!(
            GameRecord::from_json(&v.to_string()).is_err(),
            "tamper {index}/{key}"
        );
    }
    let mut v = original.clone();
    v["current_position"]["hands"][0][0] = json!(1);
    assert!(GameRecord::from_json(&v.to_string()).is_err());
    let mut v = original.clone();
    v["root"]["historical_reachability_proven"] = json!(true);
    assert!(GameRecord::from_json(&v.to_string()).is_err());
    let mut v = original.clone();
    v["events"] = json!(vec![original["events"][0].clone(); MAX_GAME_EVENTS + 1]);
    assert_eq!(
        GameRecord::from_json(&v.to_string()).unwrap_err(),
        RecordError::Capacity
    );
    let small = record(&Game::standard());
    let padded = format!(
        "{}{}",
        small,
        " ".repeat(MAX_GAME_RECORD_BYTES - small.len())
    );
    assert!(GameRecord::from_json(&padded).is_ok());
    assert_eq!(
        GameRecord::from_json(&(padded + " ")).unwrap_err(),
        RecordError::Capacity
    );
    assert_eq!(context, g.context());
    assert_eq!(good, record(&g));
}
#[test]
fn counter_refusal_is_unscored_saveable_and_does_not_manufacture_no_actions() {
    let f = fixture("TORI-H-COUNT");
    let mut p = root(&f["root"]).unwrap();
    p.state.root.declared_completed_plies = u64::MAX;
    let mut g = Game::from_root(p).unwrap();
    assert!(!g.legal_actions().is_empty());
    let a = g.legal_actions()[0];
    rejection(
        &mut g,
        Side::Sente,
        GameCommand::Play { action: a },
        GameError::Capacity(Resource::Counter),
    );
    assert!(g.conclusion().is_none());
    assert_eq!(
        record(
            &GameRecord::from_json(&record(&g))
                .unwrap()
                .to_game()
                .unwrap()
        ),
        record(&g)
    );
    submit(
        &mut g,
        Side::Gote,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    submit(&mut g, Side::Gote, GameCommand::Resign).unwrap();
}
#[test]
fn long_deterministic_real_history_reaches_replay_admission_bound_and_retains_stop_resign_headroom()
{
    // Fixed-seed ordinary complete legal moves, favouring previously unseen keys
    // to exercise a long real game rather than a fabricated event array.
    let mut g = Game::standard();
    let mut seed = 0x129c_a11a_b1e5_u64;
    let started = std::time::Instant::now();
    let mut accepted = 0;
    let mut snapshots = vec![g.cache_snapshot()];
    let mut undos = vec![];
    loop {
        let action = choose_live(&g, &mut seed);
        let before = g.context();
        let bytes = record(&g);
        match play(&mut g, action) {
            Ok(u) => {
                accepted += 1;
                undos.push(u);
                snapshots.push(g.cache_snapshot());
                assert_eq!(g.events().len(), accepted);
                assert_eq!(g.position().declared_total_plies(), accepted as u64);
            }
            Err(GameError::Capacity(Resource::ReplayWork)) => {
                assert_eq!(before, g.context());
                assert_eq!(bytes, record(&g));
                break;
            }
            Err(e) => panic!("unexpected admission error:{e:?}"),
        }
    }
    assert_eq!(accepted, 360); // root1 + sum(2..361)=65,341; next adds362 >65,534.
    let captures = g
        .events()
        .iter()
        .filter(|e| e.effects.is_some_and(|f| f.captured.is_some()))
        .count();
    let drops = g
        .events()
        .iter()
        .filter(|e| {
            matches!(
                e.command,
                GameCommand::Play {
                    action: Action::Drop { .. }
                }
            )
        })
        .count();
    let promotions = g
        .events()
        .iter()
        .filter(|e| {
            e.effects.is_some_and(|f| match f.action {
                Action::Move { from, .. } => g
                    .event_prefix(e.index)
                    .unwrap()
                    .local_position()
                    .position()
                    .piece(from)
                    .is_some_and(|p| p.face != f.resulting_piece.face),
                _ => false,
            })
        })
        .count();
    assert!(captures > 0 && drops > 0 && promotions > 0);
    let admission_ms = started.elapsed().as_millis();
    let before = g.capacity_usage().unwrap();
    assert_eq!(before.replay_work, 65_341);
    assert!(before.encoded_bytes < MAX_GAME_RECORD_BYTES - 8192);
    let long = record(&g);
    let replay_started = std::time::Instant::now();
    let reconstructed = GameRecord::from_json(&long).unwrap().to_game().unwrap();
    let replay_ms = replay_started.elapsed().as_millis();
    assert_eq!(g.cache_snapshot(), reconstructed.cache_snapshot());
    let before_record = record(&g);
    let before_usage = g.capacity_usage().unwrap();
    submit(
        &mut g,
        Side::Sente,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    let stopped = g.capacity_usage().unwrap();
    assert!(stopped.encoded_bytes - before_usage.encoded_bytes <= 4096);
    assert!(g.conclusion().is_none());
    assert_eq!(g.snapshot().board_ply, accepted);
    submit(&mut g, Side::Gote, GameCommand::Resign).unwrap();
    let final_usage = g.capacity_usage().unwrap();
    assert!(final_usage.encoded_bytes - stopped.encoded_bytes <= 4096);
    assert_eq!(
        record(
            &GameRecord::from_json(&record(&g))
                .unwrap()
                .to_game()
                .unwrap()
        ),
        record(&g)
    );
    // Complete exact undo sequence was produced by real live admission; admin
    // events above intentionally belong to their own undo chain.
    let (mut restored, mut replay_undos) = GameRecord::from_json(&before_record)
        .unwrap()
        .to_game_with_undo()
        .unwrap();
    let mut redos = vec![];
    for (i, u) in replay_undos.iter_mut().enumerate().rev() {
        redos.push(restored.undo(u).unwrap());
        assert_eq!(restored.cache_snapshot(), snapshots[i]);
    }
    for (i, r) in redos.iter_mut().rev().enumerate() {
        restored.redo(r).unwrap();
        assert_eq!(restored.cache_snapshot(), snapshots[i + 1]);
    }
    eprintln!(
        "TORI129 capacity real: {accepted}board/{events}events/{bytes}bytes/{work}units; captures{captures}/drops{drops}/promotions{promotions}; generation+candidate probes {admission_ms}ms; strict parse+two replays {replay_ms}ms; stopped {stopped:?}; resigned {final_usage:?}",
        events = before.events,
        bytes = before.encoded_bytes,
        work = before.replay_work
    );
    if let Ok(path) = std::env::var("MSH_TORI_129_CAPACITY_REPORT") {
        std::fs::write(path,serde_json::to_vec_pretty(&json!({"seed":"0x129ca11ab1e5","board_actions":accepted,"events":before.events,"captures":captures,"drops":drops,"promotions":promotions,"encoded_bytes":before.encoded_bytes,"replay_work":before.replay_work,"generation_with_candidate_probes_ms":admission_ms,"strict_parse_plus_two_replays_ms":replay_ms,"stop_bytes":stopped.encoded_bytes,"resignation_bytes":final_usage.encoded_bytes,"record":serde_json::from_str::<Value>(&long).unwrap()})).unwrap()).unwrap();
    }
}

fn choose_live(g: &Game, seed: &mut u64) -> Action {
    let choices = g.legal_actions();
    assert!(!choices.is_empty());
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    let start = *seed as usize % choices.len();
    let mut chosen = None;
    for i in 0..choices.len() {
        let action = choices[(start + i) % choices.len()];
        let mut candidate = g.transaction_for_test();
        match play(&mut candidate, action) {
            Ok(_) if candidate.conclusion().is_none() => {
                chosen = Some(action);
                if candidate.occurrences().len() == 1 {
                    break;
                }
            }
            Err(GameError::Capacity(_)) => {
                chosen = Some(action);
                break;
            }
            _ => {}
        }
    }
    chosen.expect("live continuation")
}
#[test]
fn real_event_limit_preserves_two_slots_for_stop_and_resignation() {
    let mut g = Game::standard();
    let mut seed = 0x129c_a11a_b1e5_u64;
    for _ in 0..255 {
        submit(
            &mut g,
            Side::Sente,
            GameCommand::Stop {
                reason: StopReason::WorkLimit,
            },
        )
        .unwrap();
        let a = choose_live(&g, &mut seed);
        play(&mut g, a).unwrap();
    }
    assert_eq!(g.events().len(), 510);
    assert_eq!(g.snapshot().board_ply, 255);
    assert!(g.conclusion().is_none());
    let a = choose_live(&g, &mut seed);
    let actor = g.position().side_to_move();
    rejection(
        &mut g,
        actor,
        GameCommand::Play { action: a },
        GameError::Capacity(Resource::Events),
    );
    let ordinary = g.capacity_usage().unwrap();
    submit(
        &mut g,
        Side::Gote,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    assert_eq!(g.events().len(), 511);
    assert!(g.conclusion().is_none());
    submit(&mut g, Side::Sente, GameCommand::Resign).unwrap();
    assert_eq!(g.events().len(), 512);
    assert_eq!(g.snapshot().board_ply, 255);
    assert_eq!(
        record(
            &GameRecord::from_json(&record(&g))
                .unwrap()
                .to_game()
                .unwrap()
        ),
        record(&g)
    );
    eprintln!(
        "TORI129 real event boundary:{ordinary:?}; final {:?}",
        g.capacity_usage().unwrap()
    );
}
