use super::conformance::{action, root};
use super::*;
use serde_json::{Value, json};

fn fixture(id: &str) -> Value {
    let v: Value =
        serde_json::from_str(include_str!("../../../../docs/m4/chu/fixtures.json")).unwrap();
    v["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()
        .clone()
}
fn game(id: &str) -> Game {
    Game::from_root(root(&fixture(id)["given"]["root"]).unwrap()).unwrap()
}
fn play(g: &mut Game, a: Action) -> Result<GameUndo, GameError> {
    g.submit(GameRequest {
        expected: g.context(),
        actor: g.position().side_to_move(),
        command: GameCommand::Play { action: a },
    })
}
fn control(g: &mut Game, actor: Side, command: GameCommand) -> Result<GameUndo, GameError> {
    g.submit(GameRequest {
        expected: g.context(),
        actor,
        command,
    })
}
fn bytes(g: &Game) -> String {
    GameRecord::from_game(g).unwrap().to_json().to_owned()
}

#[test]
fn lifo_full_game_undo_redo_and_branch_retirement() {
    let f = fixture("CHU-REP-QUIET");
    let mut g = game("CHU-REP-QUIET");
    let before = g.cache_snapshot();
    let old = g.context();
    let mut tokens = vec![];
    for a in f["given"]["commands"].as_array().unwrap().iter().take(4) {
        tokens.push(play(&mut g, action(a)).unwrap());
    }
    let after = g.cache_snapshot();
    assert_eq!(g.undo(&mut tokens[0]).unwrap_err(), GameError::InvalidToken);
    let mut redos = vec![];
    for t in tokens.iter_mut().rev() {
        redos.push(g.undo(t).unwrap());
    }
    assert_eq!(g.cache_snapshot(), before);
    assert_ne!(g.context(), old);
    assert!(g.context().same_semantics(&old));
    for r in redos.iter_mut().rev() {
        g.redo(r).unwrap();
    }
    assert_eq!(g.cache_snapshot(), after);
    assert_eq!(g.redo(&mut redos[0]).unwrap_err(), GameError::InvalidToken);
    let mut t = play(&mut g, action(&f["given"]["commands"][4])).unwrap();
    let mut r = g.undo(&mut t).unwrap();
    let alternate = g
        .legal_actions()
        .into_iter()
        .find(|a| *a != action(&f["given"]["commands"][4]))
        .unwrap();
    play(&mut g, alternate).unwrap();
    assert_eq!(g.redo(&mut r).unwrap_err(), GameError::InvalidToken);
}
#[test]
fn stale_actor_selection_and_reopening_never_revive_work() {
    let mut g = game("CHU-REP-QUIET");
    let a = g.legal_actions()[0].clone();
    let c = g.context();
    let before = g.cache_snapshot();
    assert_eq!(
        g.submit(GameRequest {
            expected: c.clone(),
            actor: Side::Gote,
            command: GameCommand::Play { action: a.clone() }
        })
        .unwrap_err(),
        GameError::WrongActor
    );
    assert_eq!(g.cache_snapshot(), before);
    g.select_prefix(0).unwrap();
    assert_eq!(
        g.submit(GameRequest {
            expected: c,
            actor: Side::Sente,
            command: GameCommand::Play { action: a.clone() }
        })
        .unwrap_err(),
        GameError::StaleContext
    );
    let retired = g.context();
    g.retire_context().unwrap();
    assert_ne!(retired, g.context());
    let saved = GameRecord::from_game(&g).unwrap();
    let reopened = saved.to_game().unwrap();
    assert!(g.context().same_semantics(&reopened.context()));
    assert_ne!(g.context(), reopened.context());
    play(&mut g, a).unwrap();
    assert_eq!(g.event_prefix(0).unwrap().board_ply, 0);
    assert_eq!(g.event_prefix(1).unwrap().board_ply, 1);
    assert!(
        g.record_prefix(0)
            .unwrap()
            .to_game()
            .unwrap()
            .events()
            .is_empty()
    );
}
#[test]
fn counter_exhaustion_is_unscored_and_speculation_is_counter_independent() {
    let f = fixture("CHU-MATERIAL-DECLINE-COUNTERCAPTURE");
    let mut p = root(&f["given"]["root"]).unwrap();
    p.state.root = RootMetadata::diagram(u64::MAX);
    let mut g = Game::from_root(p).unwrap();
    assert_eq!(g.conclusion(), None);
    assert!(!g.immediate_material_replies().is_empty());
    let before = g.cache_snapshot();
    let c = g.context();
    let a = action(&f["given"]["action"]);
    assert_eq!(
        play(&mut g, a).unwrap_err(),
        GameError::Capacity(Resource::Counter)
    );
    assert_eq!(g.cache_snapshot(), before);
    assert_eq!(g.context(), c);
    control(
        &mut g,
        Side::Gote,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    assert_eq!(g.conclusion(), None);
    control(&mut g, Side::Sente, GameCommand::Resign).unwrap();
    assert_eq!(
        g.conclusion(),
        Some(Conclusion::Resignation { winner: Side::Gote })
    );
    assert!(GameRecord::from_json(&bytes(&g)).is_ok());
}
#[test]
fn observational_queries_do_not_change_record_or_admission_budget() {
    let g = game("CHU-MATE-PRINCE-RESPONSE");
    let before = bytes(&g);
    let cache = g.cache_snapshot();
    let usage = g.capacity_usage().unwrap();
    for _ in 0..3 {
        let _ = g.has_capture_all(Side::Sente).unwrap();
        let _ = g.immediate_material_replies();
        let _ = g.status();
        let _ = g.legal_actions();
    }
    assert_eq!(bytes(&g), before);
    assert_eq!(g.cache_snapshot(), cache);
    assert_eq!(g.capacity_usage().unwrap(), usage);
}
#[test]
fn event_capacity_refuses_before_commit_and_reserves_real_stop_and_resign() {
    let mut g = game("CHU-MATERIAL-AGREEMENT");
    for _ in 0..(MAX_GAME_EVENTS - 2) / 2 {
        control(&mut g, Side::Sente, GameCommand::OfferDraw).unwrap();
        control(&mut g, Side::Gote, GameCommand::DeclineDraw).unwrap();
    }
    let before = bytes(&g);
    let c = g.context();
    let cache = g.cache_snapshot();
    assert_eq!(
        control(&mut g, Side::Sente, GameCommand::OfferDraw).unwrap_err(),
        GameError::Capacity(Resource::Events)
    );
    assert_eq!(bytes(&g), before);
    assert_eq!(g.context(), c);
    assert_eq!(g.cache_snapshot(), cache);
    assert_eq!(g.occurrences(), [0]);
    assert_eq!(g.snapshot().board_ply, 0);
    control(
        &mut g,
        Side::Sente,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    control(&mut g, Side::Gote, GameCommand::Resign).unwrap();
    assert_eq!(g.events().len(), MAX_GAME_EVENTS);
    assert!(g.capacity_usage().unwrap().encoded_bytes <= MAX_GAME_RECORD_BYTES);
    assert!(GameRecord::from_json(&bytes(&g)).is_ok());
}
#[test]
fn strict_canonical_mutations_reject_without_changing_live_state() {
    let f = fixture("CHU-LION-C2-3");
    let mut g = game("CHU-LION-C2-3");
    for a in f["given"]["commands"].as_array().unwrap() {
        play(&mut g, action(a)).unwrap();
    }
    let original = bytes(&g);
    let context = g.context();
    let v: Value = serde_json::from_str(&original).unwrap();
    let mut invalid = vec![];
    for (field, value) in [
        ("format", json!("other")),
        ("game", json!("chess")),
        ("profile", json!("unknown")),
        ("edition", json!("other")),
        ("schema_version", json!(99)),
        ("mode", json!("timed")),
        ("unknown", json!(true)),
    ] {
        let mut bad = v.clone();
        bad[field] = value;
        invalid.push(bad.to_string());
    }
    for field in [
        "root",
        "root_conclusion",
        "events",
        "incomplete",
        "draw_offer",
        "conclusion",
        "obligations",
    ] {
        let mut bad = v.clone();
        bad.as_object_mut().unwrap().remove(field);
        invalid.push(bad.to_string());
    }
    let mut bad = v.clone();
    bad["events"][0]["effects"]["captures"] = json!([]);
    invalid.push(bad.to_string());
    let mut bad = v.clone();
    bad["events"].as_array_mut().unwrap().reverse();
    invalid.push(bad.to_string());
    let mut bad = v.clone();
    bad["root"]["state"]["root"]["kind"] = json!("standard");
    invalid.push(bad.to_string());
    let mut bad = v.clone();
    bad["root"]["state"]["root"]
        .as_object_mut()
        .unwrap()
        .remove("kind");
    invalid.push(bad.to_string());
    let mut bad = v.clone();
    bad["current_position"]["accepted_plies"] = json!(99);
    invalid.push(bad.to_string());
    let mut bad = v.clone();
    bad["conclusion"] = json!({"kind":"material","winner":"sente"});
    invalid.push(bad.to_string());
    invalid.push(original.replacen(
        "\"side\":\"sente\"",
        "\"side\":\"gote\",\"side\":\"sente\"",
        1,
    ));
    invalid.push(format!("{}{}", original, " ".repeat(MAX_GAME_RECORD_BYTES)));
    for input in invalid {
        assert!(GameRecord::from_json(&input).is_err());
        assert_eq!(bytes(&g), original);
        assert_eq!(g.context(), context);
    }
    let (mut restored, mut handles) = GameRecord::from_json(&original)
        .unwrap()
        .to_game_with_undo()
        .unwrap();
    assert_eq!(bytes(&restored), original);
    for h in handles.iter_mut().rev() {
        restored.undo(h).unwrap();
    }
    assert!(restored.events().is_empty());
}
#[test]
fn deterministic_real_sequences_preserve_exact_logical_caches_and_replay() {
    for seed in [17u64, 61, 124] {
        let mut g = Game::standard();
        let mut rng = seed;
        for _ in 0..32 {
            let choices = g.legal_actions();
            if choices.is_empty() {
                break;
            }
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = choices[(rng as usize) % choices.len()].clone();
            let before = g.cache_snapshot();
            let c = g.context();
            let mut u = play(&mut g, a).unwrap();
            let after = g.cache_snapshot();
            let mut r = g.undo(&mut u).unwrap();
            assert_eq!(g.cache_snapshot(), before);
            assert_ne!(c, g.context());
            g.redo(&mut r).unwrap();
            assert_eq!(g.cache_snapshot(), after);
            let record = GameRecord::from_game(&g).unwrap();
            let restored = record.to_game().unwrap();
            assert_eq!(bytes(&restored), record.to_json());
            assert!(g.context().same_semantics(&restored.context()));
            assert_ne!(g.context(), restored.context());
        }
    }
}
#[test]
fn standard_provenance_cannot_be_forged_on_both_serialized_copies() {
    let g = game("CHU-MATERIAL-AGREEMENT");
    let mut v: Value = serde_json::from_str(&bytes(&g)).unwrap();
    v["root"]["state"]["root"]["kind"] = json!("standard");
    v["current_position"]["root"]["kind"] = json!("standard");
    assert_eq!(
        GameRecord::from_json(&v.to_string()).unwrap_err(),
        RecordError::Root
    );
}
#[test]
fn real_long_history_hits_work_limit_atomically_and_remains_closable_and_replayable() {
    let mut g = game("CHU-REP-QUIET");
    let mut rng = 3917u64;
    let mut refused = false;
    for _ in 0..1000 {
        let side = g.position().side_to_move();
        let actions: Vec<_> = g
            .legal_actions()
            .into_iter()
            .filter(|a| {
                let p = g.position().piece(a.from).unwrap();
                p.base == Base::King
                    && a.to().is_some_and(|s| {
                        if side == Side::Sente {
                            s.rank() >= 8
                        } else {
                            s.rank() <= 5
                        }
                    })
                    && g.position().piece(a.to().unwrap()).is_none()
            })
            .collect();
        assert!(!actions.is_empty());
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a = actions[(rng as usize) % actions.len()].clone();
        let context = g.context();
        let before = g.cache_snapshot();
        match play(&mut g, a) {
            Ok(_) => {}
            Err(GameError::Capacity(Resource::ReplayWork)) => {
                assert_eq!(g.context(), context);
                assert_eq!(g.cache_snapshot(), before);
                refused = true;
                break;
            }
            Err(e) => panic!("unexpected native refusal {e}"),
        }
    }
    assert!(refused);
    let usage = g.capacity_usage().unwrap();
    let timer = std::time::Instant::now();
    let restored = GameRecord::from_json(&bytes(&g))
        .unwrap()
        .to_game()
        .unwrap();
    assert!(g.context().same_semantics(&restored.context()));
    eprintln!(
        "Chu long native record: events={}, bytes={}, work={}, parse+replay_ms={}",
        usage.events,
        usage.encoded_bytes,
        usage.replay_work,
        timer.elapsed().as_millis()
    );
    control(
        &mut g,
        Side::Sente,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    control(&mut g, Side::Gote, GameCommand::Resign).unwrap();
    assert!(GameRecord::from_json(&bytes(&g)).is_ok());
}
#[test]
fn exact_native_record_byte_limit_accepts_padding_and_rejects_one_more_byte() {
    let g = game("CHU-REP-QUIET");
    let native = bytes(&g);
    let mut exact = native.clone();
    exact.extend(std::iter::repeat_n(
        ' ',
        MAX_GAME_RECORD_BYTES - native.len(),
    ));
    assert_eq!(exact.len(), MAX_GAME_RECORD_BYTES);
    let admitted = GameRecord::from_json(&exact).unwrap();
    assert_eq!(admitted.to_json(), native);
    exact.push(' ');
    assert_eq!(
        GameRecord::from_json(&exact).unwrap_err(),
        RecordError::Capacity
    );
    assert_eq!(bytes(&g), native);
}
