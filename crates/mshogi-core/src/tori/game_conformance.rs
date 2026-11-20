//! Actual native history/record adapter. Expected fixture answers are never execution data.
use super::{
    conformance::{action, root, side, sq},
    local_conformance::{LocalAdapter, action_value},
    *,
};
use mshogi_conformance::{
    Corpus, Fixture,
    assertions::{self, Assertion, NativeAdapter, Status},
    properties,
};
use serde_json::{Value, json};

pub(super) fn submit(
    g: &mut Game,
    actor: Side,
    command: GameCommand,
) -> Result<GameUndo, GameError> {
    g.submit(GameRequest {
        expected: g.context(),
        actor,
        command,
    })
}
pub(super) fn play(g: &mut Game, action: Action) -> Result<GameUndo, GameError> {
    submit(g, g.position().side_to_move(), GameCommand::Play { action })
}
pub(super) fn sequence(v: &Value) -> Game {
    let mut g = Game::from_root(root(&v["root"]).unwrap()).unwrap();
    for a in v["actions"].as_array().unwrap() {
        play(&mut g, action(a)).unwrap();
    }
    g
}
fn repetition(r: Option<&Repetition>) -> Value {
    r.map_or(
        Value::Null,
        |r| json!({"side":r.side,"forbidden":forbidden(r)}),
    )
}
fn conclusion(c: Option<Conclusion>) -> Value {
    match c {
        Some(Conclusion::RepetitionForfeit { loser }) => {
            json!({"kind":"repetition_forfeit","loser":loser})
        }
        Some(Conclusion::StalemateLoss { loser }) => json!({"kind":"stalemate_loss","loser":loser}),
        Some(Conclusion::Checkmate { winner }) => {
            json!({"kind":"checkmate","loser":winner.opponent()})
        }
        Some(Conclusion::Resignation { winner }) => json!({"kind":"resignation","winner":winner}),
        None => Value::Null,
    }
}
fn native_command(v: &Value) -> GameCommand {
    match v["kind"].as_str().unwrap() {
        "resign" => GameCommand::Resign,
        "agree_draw" => GameCommand::AgreeDraw,
        "declare_impasse" => GameCommand::DeclareImpasse,
        _ => GameCommand::Play { action: action(v) },
    }
}
fn forbidden(r: &Repetition) -> Value {
    json!(
        r.forbidden
            .iter()
            .copied()
            .map(action_value)
            .collect::<Vec<_>>()
    )
}
struct GameAdapter;
impl NativeAdapter for GameAdapter {
    fn observe(&self, f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
        if a.owner != "MSH-129" {
            return LocalAdapter.observe(f, a);
        }
        let field = a.field.as_str();
        if f.scope.starts_with("native_admission") {
            let mut g = Game::from_root(root(&f.given["root"]).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            let actor = f.given["command"]["actor"]
                .as_str()
                .map(side)
                .unwrap_or(g.position().side_to_move());
            let before = g.cache_snapshot();
            let context = g.context();
            let before_ply = g.snapshot().board_ply;
            let before_occ = g.occurrences().len();
            let result = submit(&mut g, actor, native_command(&f.given["command"]));
            return Ok(Some(match field {
                "accepted" => json!(result.is_ok()),
                "reason" => json!(match result {
                    Err(GameError::UnsupportedCommand) => "unsupported_command",
                    _ => return Err("unexpected terminal admission result".into()),
                }),
                "conclusion" => conclusion(g.conclusion()),
                "state_events_and_runtime_unchanged" => {
                    json!(before == g.cache_snapshot() && context == g.context() && result.is_err())
                }
                "board_ply_delta" => json!(g.snapshot().board_ply - before_ply),
                "repetition_occurrence_delta" => json!(g.occurrences().len() - before_occ),
                "reversal_contract/apply_then_undo"
                | "reversal_contract/redo"
                | "reversal_contract/retired_execution" => {
                    let mut u = result.map_err(|e| e.to_string())?;
                    let after = g.cache_snapshot();
                    let mut redo = g.undo(&mut u).map_err(|e| e.to_string())?;
                    match field {
                        "reversal_contract/apply_then_undo" => {
                            properties::snapshots(&json!(before), &json!(g.cache_snapshot()))
                        }
                        "reversal_contract/redo" => {
                            g.redo(&mut redo).map_err(|e| e.to_string())?;
                            properties::snapshots(&json!(after), &json!(g.cache_snapshot()))
                        }
                        _ => properties::snapshots(
                            &json!(format!("{context:?}")),
                            &json!(format!("{:?}", g.context())),
                        ),
                    }
                }
                _ => return Err(format!("unhandled native admission field {field}")),
            }));
        }
        if f.scope == "terminal_position" {
            let g = Game::from_root(root(&f.given["root"]).unwrap()).unwrap();
            return Ok(Some(conclusion(g.conclusion())));
        }
        if f.scope == "complete_history_sequence" {
            let mut g = sequence(&f.given);
            return Ok(Some(match field {
                "accepted_board_plies" => json!(g.snapshot().board_ply),
                "current_key_occurrences" | "live_occurrences" => json!(g.occurrences()),
                "root_key_occurrences_at6" => json!(g.event_prefix(6).unwrap().occurrences),
                "repetition" => repetition(g.repetition()),
                "conclusion" => conclusion(g.conclusion()),
                "responsible" => json!(g.repetition().ok_or("missing restriction")?.side),
                "starter_responsible" => json!(
                    side(f.given["sequence_starter"].as_str().unwrap())
                        == g.repetition().ok_or("missing restriction")?.side
                ),
                "accepted" | "restriction_cleared" | "alternative_accepted" => {
                    let result = play(&mut g, action(&f.given["next_action"]));
                    json!(
                        result.is_ok()
                            && (field != "restriction_cleared" || g.repetition().is_none())
                    )
                }
                "forbidden_attempt" => {
                    let a = action(&f.given["forbidden_attempt"]);
                    let before = g.context();
                    let snapshot = g.cache_snapshot();
                    let refused = matches!(play(&mut g, a), Err(GameError::ThirdOccurrence));
                    json!({"action":action_value(a),"accepted":!refused,"unchanged":before == g.context() && snapshot == g.cache_snapshot()})
                }
                "in_check" => json!(g.local_position().is_in_check(g.position().side_to_move())),
                "complete_board_legal_choices" => json!(
                    g.local_position()
                        .legal_actions()
                        .into_iter()
                        .map(action_value)
                        .collect::<Vec<_>>()
                ),
                "prefix4_occurrences" => json!(g.event_prefix(4).unwrap().occurrences),
                "prefix4_repetition" => repetition(g.event_prefix(4).unwrap().repetition.as_ref()),
                "strict_replay_equals_original" => {
                    let replay =
                        GameRecord::from_json(GameRecord::from_game(&g).unwrap().to_json())
                            .unwrap()
                            .to_game()
                            .unwrap();
                    json!(
                        g.cache_snapshot() == replay.cache_snapshot()
                            && g.context() != replay.context()
                    )
                }
                "undo_at6_repetition"
                | "redo_at7_repetition_side"
                | "all_native_fields_and_caches_exactly_restored" => {
                    let before = g.cache_snapshot();
                    let (mut replay, mut undos) = GameRecord::from_game(&g)
                        .unwrap()
                        .to_game_with_undo()
                        .unwrap();
                    let mut r = replay.undo(undos.last_mut().unwrap()).unwrap();
                    if field == "undo_at6_repetition" {
                        repetition(replay.repetition())
                    } else {
                        replay.redo(&mut r).unwrap();
                        if field == "redo_at7_repetition_side" {
                            json!(replay.repetition().unwrap().side)
                        } else {
                            json!(before == replay.cache_snapshot())
                        }
                    }
                }
                _ => return Err(format!("unhandled history field {field}")),
            }));
        }
        if f.scope.starts_with("history_adjudication_unit") {
            // The forced-loss clause alone: the restricted actions and the
            // complete local choices are explicit inputs.
            let r = Repetition {
                side: side(f.given["side"].as_str().unwrap()),
                forbidden: f.given["forbidden"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(action)
                    .collect(),
            };
            let local: Vec<Action> = f.given["safety_drop_legal_choices"]
                .as_array()
                .map(|as_| as_.iter().map(action).collect())
                .unwrap_or_default();
            let loss = r
                .forces_loss(&local)
                .then_some(Conclusion::RepetitionForfeit { loser: r.side });
            return Ok(Some(match field {
                "responsible" => json!(r.side),
                "conclusion" => conclusion(loss),
                "no_check_evasion_exemption" => json!(loss.is_some() && !local.is_empty()),
                "no_suicide_action_committed" => {
                    json!(local.iter().all(|a| !r.permits(*a)) && loss.is_some())
                }
                "not_stalemate" => json!(
                    !local.is_empty() && matches!(loss, Some(Conclusion::RepetitionForfeit { .. }))
                ),
                _ => return Err(format!("unhandled unit field {field}")),
            }));
        }
        if f.scope == "canonical_context_pair" {
            let mut ga = sequence(&f.given["record_A"]);
            let mut gb = sequence(&f.given["record_B"]);
            return Ok(Some(match field {
                "same_board_hands_turn" => json!(ga.repetition_key() == gb.repetition_key()),
                "different_event_and_execution_context" => json!(
                    ga.context() != gb.context() && !ga.context().same_semantics(&gb.context())
                ),
                "A_repetition" => repetition(ga.repetition()),
                "B_repetition_side" => json!(gb.repetition().unwrap().side),
                "same_action_5a4a_A_accepted_B_rejected" => {
                    let a = Action::Move {
                        from: sq("5a"),
                        to: sq("4a"),
                        result_face: Face::Phoenix,
                    };
                    json!(
                        play(&mut ga, a).is_ok()
                            && matches!(play(&mut gb, a), Err(GameError::ThirdOccurrence))
                    )
                }
                _ => return Err(format!("unhandled pair field {field}")),
            }));
        }
        if f.scope == "execution_contract" {
            let mut g = Game::standard();
            let old = g.context();
            let a = g.legal_actions()[0];
            let mut u = play(&mut g, a).unwrap();
            g.undo(&mut u).unwrap();
            return Ok(Some(properties::snapshots(
                &json!(format!("{old:?}")),
                &json!(format!("{:?}", g.context())),
            )));
        }
        if f.scope == "key_contract" {
            let p = root(&f.given["base"]).unwrap();
            let k = RepetitionKey::from_position(&p);
            return Ok(Some(match field {
                "counter_or_observation_only_changes_key" => {
                    let mut changed = p.clone();
                    changed.state.root.declared_completed_plies = 19;
                    json!(k != RepetitionKey::from_position(&changed))
                }
                "each_difference_changes_repetition_key" => {
                    let mut turn = p.clone();
                    turn.state.side = turn.state.side.opponent();
                    let mut hands = p.clone();
                    hands.state.hands[0] = Hand::from_counts(&[(Face::LeftQuail, 1)]).unwrap();
                    let mut left = p.clone();
                    left.state.board[sq("4d").index()] = Some(Piece {
                        side: Side::Sente,
                        face: Face::LeftQuail,
                    });
                    let mut right = left.clone();
                    right.state.board[sq("4d").index()].as_mut().unwrap().face = Face::RightQuail;
                    json!(
                        k != RepetitionKey::from_position(&turn)
                            && k != RepetitionKey::from_position(&hands)
                            && RepetitionKey::from_position(&left)
                                != RepetitionKey::from_position(&right)
                    )
                }
                _ => return Err(format!("unhandled key field {field}")),
            }));
        }
        if f.scope == "notation_contract" {
            return Ok(Some(match field {
                "unknown_profile_rejected_atomically" => {
                    let g = Game::standard();
                    let before = g.context();
                    let r = GameRecord::from_game(&g).unwrap();
                    json!(
                        GameRecord::from_json(
                            &r.to_json().replace(PROFILE_ID, "mshogi.tori.bunsei11.v1")
                        )
                        .is_err()
                            && g.context() == before
                    )
                }
                "seven_file_coordinate_roundtrip" => {
                    let a = action(&f.given["native"]);
                    let Action::Move { from, to, .. } = a else {
                        unreachable!()
                    };
                    json!(format!(
                        "{}→{}",
                        from.to_string().parse::<Square>().unwrap(),
                        to.to_string().parse::<Square>().unwrap()
                    ))
                }
                "quail_labels_distinct" => json!([Face::LeftQuail.code(), Face::RightQuail.code()]),
                _ => return Err(format!("unhandled notation field {field}")),
            }));
        }
        if f.scope == "strict_dispatch_contract" {
            let all = f.given["raw_payloads"].as_array().unwrap().iter().all(|p| {
                let p = p.as_str().unwrap();
                GameRecord::from_json(p).is_err() && GameCommand::from_json(p).is_err()
            });
            let g = Game::standard();
            let good = GameRecord::from_game(&g).unwrap();
            let duplicate =
                good.to_json()
                    .replacen("\"profile\":", "\"profile\":\"bogus\",\"profile\":", 1);
            return Ok(Some(json!(
                all && GameRecord::from_json(&duplicate).is_err()
            )));
        }
        Err(format!("unhandled MSH129 fixture {} / {field}", f.id))
    }
}
#[test]
fn actual_complete_native_fixture_observations() {
    let corpus = Corpus::embedded().unwrap();
    let report = assertions::run_adapter(&corpus, &GameAdapter);
    for a in &report.assertions {
        if a.status == Status::Failed {
            eprintln!("{a:?}");
        }
    }
    if let Ok(path) = std::env::var("MSH_TORI_129_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
    assert_eq!(report.failed, 0);
    assert!(
        report
            .assertions
            .iter()
            .filter(|a| matches!(a.owner.as_str(), "MSH-127" | "MSH-128" | "MSH-129"))
            .all(|a| a.status == Status::Passed)
    );
    assert_eq!(
        (
            report.executed,
            report.passed,
            report.pending,
            report.failed
        ),
        (713, 713, 14, 0)
    );
    eprintln!(
        "TORI129 actual native:713 executed/713 passed/14 pending/0 failed; {} later consumer requirements pending",
        report.later_consumer_requirements.len()
    );
}
