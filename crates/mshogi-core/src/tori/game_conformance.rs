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
fn obligation(o: Option<&Obligation>) -> Value {
    o.map_or(Value::Null, |o| json!({"side":o.side,"at_ply":o.at_ply(),"forbidden_departures":o.forbidden_departures.iter().copied().map(action_value).collect::<Vec<_>>()}))
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
fn obligation_actions(o: &Obligation) -> Value {
    json!(
        o.forbidden_departures
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
                "root_key_occurrences" | "live_occurrences" => json!(g.occurrences()),
                "obligation" => obligation(g.obligation()),
                "conclusion" => conclusion(g.conclusion()),
                "responsible" => json!(g.obligation().ok_or("missing obligation")?.side),
                "creator_not_responsible" => json!(g.events().last().unwrap().actor != g.obligation().unwrap().side),
                "accepted" | "obligation_consumed" => {
                    let result = play(&mut g, action(&f.given["next_action"]));
                    json!(result.is_ok() && (field == "accepted" || g.obligation().is_none()))
                }
                "next_forbidden_attempt" => {
                    // The actual previously taken departure is the query input.
                    let a = action(&f.given["actions"][0]);
                    let before = g.context();
                    let accepted = play(&mut g, a).is_ok();
                    json!({"action":action_value(a),"accepted":accepted,"unchanged":before == g.context()})
                }
                "in_check_at8" => json!(g.local_position().is_in_check(g.position().side_to_move())),
                "complete_board_legal_choices_at8" => json!(g.local_position().legal_actions().into_iter().map(action_value).collect::<Vec<_>>()),
                "latest_three_at16" => json!(g.obligation().unwrap().occurrences),
                "forbidden_at10" => obligation_actions(g.event_prefix(10).unwrap().obligation.as_ref().unwrap()),
                "forbidden_at16" => obligation_actions(g.obligation().unwrap()),
                "chosen_3f_at10_accepted" => json!(g.events().get(10).is_some_and(|e| matches!(e.command, GameCommand::Play { action: Action::Move { to, .. } } if to == sq("3f")))),
                "next_5g_accepted" => json!(play(&mut g, Action::Move { from: sq("4g"), to: sq("5g"), result_face: Face::Phoenix }).is_ok()),
                "prefix4_occurrences" => json!(g.event_prefix(4).unwrap().occurrences),
                "prefix4_obligation" => obligation(g.event_prefix(4).unwrap().obligation.as_ref()),
                "strict_replay_equals_original" => {
                    let replay = GameRecord::from_json(GameRecord::from_game(&g).unwrap().to_json()).unwrap().to_game().unwrap();
                    json!(g.cache_snapshot() == replay.cache_snapshot() && g.context() != replay.context())
                }
                "undo_at7_obligation" | "redo_at8_obligation_side" | "all_native_fields_and_caches_exactly_restored" => {
                    let before = g.cache_snapshot();
                    let (mut replay, mut undos) = GameRecord::from_game(&g).unwrap().to_game_with_undo().unwrap();
                    let mut r = replay.undo(undos.last_mut().unwrap()).unwrap();
                    if field == "undo_at7_obligation" { obligation(replay.obligation()) } else {
                        replay.redo(&mut r).unwrap();
                        if field == "redo_at8_obligation_side" { json!(replay.obligation().unwrap().side) } else { json!(before == replay.cache_snapshot()) }
                    }
                }
                _ => return Err(format!("unhandled history field {field}")),
            }));
        }
        if f.scope.starts_with("history_adjudication_unit") {
            let indices: Vec<usize> =
                serde_json::from_value(f.given["occurrences"].clone()).unwrap();
            let get_departure = |i: usize| {
                if let Some(ds) = f.given.get("departures") {
                    action(&ds[i.to_string()])
                } else {
                    action(&f.given["departures_at_0_and_4"][usize::from(i != 0)])
                }
            };
            let o = Obligation::derive(
                side(f.given["key_side"].as_str().unwrap()),
                &indices,
                get_departure,
            )
            .unwrap();
            let local: Vec<Action> = f.given["safety_drop_legal_choices"]
                .as_array()
                .map(|as_| as_.iter().map(action).collect())
                .unwrap_or_default();
            let loss = o
                .forces_loss(&local)
                .then_some(Conclusion::RepetitionForfeit { loser: o.side });
            return Ok(Some(match field {
                "responsible" => json!(o.side),
                "conclusion" => conclusion(loss),
                "no_check_evasion_exemption" => json!(loss.is_some() && !local.is_empty()),
                "no_suicide_action_committed" => {
                    json!(local.iter().all(|a| !o.permits(*a)) && loss.is_some())
                }
                "not_stalemate" => json!(
                    !local.is_empty() && matches!(loss, Some(Conclusion::RepetitionForfeit { .. }))
                ),
                "latest_three" => json!(o.occurrences),
                "forbidden_departures" => obligation_actions(&o),
                "departure_4g3g_allowed" => json!(o.permits(Action::Move {
                    from: sq("4g"),
                    to: sq("3g"),
                    result_face: Face::Phoenix
                })),
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
                "A_obligation" => obligation(ga.obligation()),
                "B_obligation_side" => json!(gb.obligation().unwrap().side),
                "same_action_4g5g_A_accepted_B_rejected" => {
                    let a = Action::Move {
                        from: sq("4g"),
                        to: sq("5g"),
                        result_face: Face::Phoenix,
                    };
                    json!(
                        play(&mut ga, a).is_ok()
                            && matches!(play(&mut gb, a), Err(GameError::RepetitionDeparture))
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
