//! Independently sourced MSH-94 history fixtures; observations are kept outside expected data.
use mshogi_core::xiangqi::{
    Action, ClaimPolicy, CommandResult, Conclusion, DrawReason, EventKind, FaultKind, Game,
    GameCommand, GameError, GameRecord, GameRequest, GameUndo, NoWinAssessment, PositionRoot, Side,
    WinReason,
};
use serde_json::{Value, json};

const CYCLE_FIXTURES: &str = include_str!("../../../tests/fixtures/xiangqi/history-cycles.json");
const COUNT_FIXTURES: &str = include_str!("../../../tests/fixtures/xiangqi/natural-count.json");
const CYCLE_ROOT: &str = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
const C: [&str; 4] = ["e9f9", "f8e8", "f9e9", "e8f8"];
const Q: [&str; 4] = ["e9a9", "f8f7", "a9e9", "f7f8"];

fn source_fixtures(text: &str) -> Vec<Value> {
    serde_json::from_str::<Value>(text).unwrap()["fixtures"]
        .as_array()
        .unwrap()
        .clone()
}
fn root_game(fen: &str, policy: ClaimPolicy) -> Game {
    Game::from_root(PositionRoot::from_fen(fen).unwrap(), policy).unwrap()
}
fn command(game: &mut Game, actor: Side, command: GameCommand) -> mshogi_core::xiangqi::Submission {
    let expected = game.context();
    game.submit(GameRequest {
        expected,
        actor,
        command,
    })
    .unwrap()
}
fn play(game: &mut Game, action: &str) -> GameUndo {
    let actor = game.position().side_to_move();
    let submission = command(
        game,
        actor,
        GameCommand::Play {
            action: action.parse().unwrap(),
        },
    );
    assert_eq!(submission.result, CommandResult::Accepted);
    submission.undo
}
fn play_fixture_prefix(game: &mut Game, fixture: &Value, count: usize) {
    for (ply, action) in fixture["moves"]
        .as_array()
        .unwrap()
        .iter()
        .take(count)
        .enumerate()
    {
        assert_eq!(
            game.conclusion(),
            None,
            "premature conclusion before ply{}",
            ply + 1
        );
        assert!(
            game.pending_adjudication().is_none(),
            "unexpected pending before ply{}: {:?}",
            ply + 1,
            game.pending_adjudication()
        );
        play(game, action.as_str().unwrap());
    }
}
fn canonical(game: &Game) -> String {
    GameRecord::from_game(game).unwrap().to_json().to_owned()
}
fn assert_record_roundtrip(game: &Game) {
    let record = GameRecord::from_game(game).unwrap();
    let validated = GameRecord::from_json(record.to_json()).unwrap();
    assert_eq!(validated, record);
    let restored = validated.to_game().unwrap();
    assert_eq!(canonical(&restored), record.to_json());
    assert_eq!(restored.position(), game.position());
    assert_eq!(restored.events(), game.events());
    assert_eq!(restored.warnings(), game.warnings());
    assert_eq!(restored.conclusion(), game.conclusion());
    assert_eq!(restored.pending_adjudication(), game.pending_adjudication());
    assert_ne!(
        restored.context(),
        game.context(),
        "restoration must establish a fresh runtime"
    );
    for count in 0..=game.events().len() {
        assert_eq!(
            restored.event_prefix(count).unwrap(),
            game.event_prefix(count).unwrap()
        );
    }
}

#[test]
fn independently_expected_check_chase_and_overlapping_cycle_histories() {
    for fixture in source_fixtures(CYCLE_FIXTURES) {
        let id = fixture["id"].as_str().unwrap();
        let mut game = root_game(fixture["root_fen"].as_str().unwrap(), ClaimPolicy::Explicit);
        for (i, action) in fixture["moves"].as_array().unwrap().iter().enumerate() {
            assert_eq!(
                game.conclusion(),
                None,
                "{id} concluded before ply{}",
                i + 1
            );
            assert!(
                game.pending_adjudication().is_none(),
                "{id} pending {:?}",
                game.pending_adjudication()
            );
            play(&mut game, action.as_str().unwrap());
            if let Some(warning) = fixture["expected_warning"].as_object()
                && i + 1 == warning["ply"].as_u64().unwrap() as usize
            {
                assert!(
                    game.warnings()
                        .iter()
                        .any(|w| w.side == Side::Red && w.active),
                    "{id} missing third-occurrence warning"
                );
                assert_eq!(game.conclusion(), None, "{id} warning must not conclude");
            }
        }
        let expected = &fixture["expected_conclusion"];
        if expected.is_null() {
            assert_eq!(game.conclusion(), None, "{id}");
        } else if expected["result"] == "draw" {
            assert_eq!(
                game.conclusion(),
                Some(Conclusion::Draw {
                    reason: DrawReason::Repetition
                }),
                "{id}"
            );
        } else {
            let fault = if id.contains("TTT") {
                FaultKind::PerpetualChase
            } else {
                FaultKind::PerpetualCheck
            };
            assert_eq!(
                game.conclusion(),
                Some(Conclusion::Win {
                    winner: Side::Black,
                    reason: WinReason::Repetition { fault }
                }),
                "{id}"
            );
        }
        assert!(
            game.pending_adjudication().is_none(),
            "{id} unresolved {:?}",
            game.pending_adjudication()
        );
        if fixture["expected_warning"].is_null() {
            assert!(
                game.warnings().is_empty(),
                "{id}: do not cherry-pick a pure latest return out of mixed contiguous history"
            );
        }
        assert_record_roundtrip(&game);
    }
}

#[test]
fn warning_and_conclusion_are_exact_event_prefixes_separate_from_board_plies() {
    let mut game = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    for action in C.into_iter().cycle().take(12) {
        play(&mut game, action);
    }
    let warning = game.warnings().first().unwrap();
    assert_eq!(warning.occurrences, [0, 4, 8]);
    let before = game.event_prefix(warning.event_index).unwrap();
    let after = game.event_prefix(warning.event_index + 1).unwrap();
    assert_eq!(before.board_ply, 8);
    assert_eq!(after.board_ply, 8);
    assert_eq!(before.position, after.position);
    assert!(before.warnings.is_empty());
    assert_eq!(after.warnings.len(), 1);
    assert_eq!(before.conclusion, None);
    assert_eq!(after.conclusion, None);
    let event = game
        .events()
        .iter()
        .find(|e| matches!(e.event, EventKind::Concluded { .. }))
        .unwrap();
    let before = game.event_prefix(event.index).unwrap();
    let after = game.event_prefix(event.index + 1).unwrap();
    assert_eq!(before.position, after.position);
    assert_eq!((before.board_ply, after.board_ply), (12, 12));
    assert_eq!(before.conclusion, None);
    assert_eq!(after.conclusion, game.conclusion());
    assert!(game.event_prefix(game.events().len() + 1).is_err());
}

#[test]
fn variation_retires_warning_without_erasing_its_authoritative_history() {
    let mut game = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    for action in C.into_iter().cycle().take(8) {
        play(&mut game, action);
    }
    let warning_event = game.warnings()[0].event_index;
    play(&mut game, Q[0]);
    assert_eq!(game.warnings().len(), 1);
    assert!(!game.warnings()[0].active);
    let retired = game.events().iter().find(|e| matches!(e.event, EventKind::WarningRetired { warning_event_index } if warning_event_index == warning_event)).unwrap();
    assert_eq!(retired.board_ply, 9);
    assert!(game.event_prefix(warning_event + 1).unwrap().warnings[0].active);
    assert!(!game.event_prefix(retired.index + 1).unwrap().warnings[0].active);
    assert_eq!(game.conclusion(), None);
    assert_record_roundtrip(&game);
}

#[test]
fn same_complete_position_with_check_or_noncheck_history_has_distinct_context() {
    let mut checked = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    let mut permitted = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    for action in C.into_iter().cycle().take(8) {
        play(&mut checked, action);
    }
    for action in Q.into_iter().cycle().take(8) {
        play(&mut permitted, action);
    }
    assert_eq!(checked.position(), permitted.position());
    assert!(!checked.warnings().is_empty());
    assert!(permitted.warnings().is_empty());
    assert_ne!(canonical(&checked), canonical(&permitted));
    let before = canonical(&permitted);
    let request = GameRequest {
        expected: checked.context(),
        actor: Side::Red,
        command: GameCommand::Resign,
    };
    assert_eq!(
        permitted.submit(request).unwrap_err(),
        GameError::StaleContext
    );
    assert_eq!(canonical(&permitted), before);
}

#[test]
fn semantic_undo_restores_actions_warning_claim_audit_and_terminal_without_reviving_requests() {
    let mut game = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    let original_context = game.context();
    let original = canonical(&game);
    let mut stack = Vec::new();
    for action in C.into_iter().cycle().take(8) {
        let before = canonical(&game);
        let undo = play(&mut game, action);
        stack.push((before, undo));
    }
    let before = canonical(&game);
    let rejected = command(&mut game, Side::Black, GameCommand::ClaimNaturalCount);
    assert_eq!(rejected.result, CommandResult::RejectedNaturalCount);
    stack.push((before, rejected.undo));
    let before = canonical(&game);
    let resignation = command(&mut game, Side::Black, GameCommand::Resign);
    stack.push((before, resignation.undo));
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Red,
            reason: WinReason::Resignation
        })
    );
    while let Some((expected, undo)) = stack.pop() {
        game.undo(undo).unwrap();
        assert_eq!(canonical(&game), expected);
        assert_ne!(game.context(), original_context);
    }
    assert_eq!(canonical(&game), original);
    assert_eq!(
        game.submit(GameRequest {
            expected: original_context,
            actor: Side::Red,
            command: GameCommand::Resign
        })
        .unwrap_err(),
        GameError::StaleContext
    );
    assert_eq!(canonical(&game), original);
}

#[test]
fn restoration_and_out_of_order_undo_cannot_revive_runtime_work() {
    let mut game = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    let first = play(&mut game, C[0]);
    let second = play(&mut game, C[1]);
    let before = canonical(&game);
    assert_eq!(game.undo(first).unwrap_err(), GameError::StaleContext);
    assert_eq!(canonical(&game), before);
    let mut restored = GameRecord::from_game(&game).unwrap().to_game().unwrap();
    assert_eq!(restored.undo(second).unwrap_err(), GameError::StaleContext);
    assert_eq!(canonical(&restored), before);
    let old = GameRequest {
        expected: game.context(),
        actor: Side::Red,
        command: GameCommand::Resign,
    };
    assert_eq!(restored.submit(old).unwrap_err(), GameError::StaleContext);
}

#[test]
fn natural_count_boundaries_use_actual_known_history_and_only_claimant_checks() {
    for fixture in source_fixtures(COUNT_FIXTURES) {
        for boundary in fixture["expected_claim_boundaries"]
            .as_array()
            .into_iter()
            .flatten()
        {
            let mut game = root_game(fixture["root_fen"].as_str().unwrap(), ClaimPolicy::Explicit);
            let plies = boundary["prefix_plies"].as_u64().unwrap() as usize;
            play_fixture_prefix(&mut game, &fixture, plies);
            let claimant = if boundary["claimant"] == "red" {
                Side::Red
            } else {
                Side::Black
            };
            let expected_count = boundary["count"].as_u64().unwrap() as usize;
            let eligible = boundary["eligible"].as_bool().unwrap();
            let evidence = game.natural_count_eligibility(claimant);
            assert_eq!(
                evidence.effective_plies, expected_count,
                "{} at{plies} {claimant:?}",
                fixture["id"]
            );
            assert_eq!(evidence.eligible, eligible);
            if fixture["id"] == "NATURAL-CAPTURE-RESET" {
                assert_eq!((evidence.known_plies, evidence.claimant_checks), (0, 0));
            }
            let position = game.position().clone();
            let warnings = game.warnings().to_vec();
            let events = game.events().len();
            assert_eq!(
                game.conclusion(),
                None,
                "eligibility query must not exercise claim"
            );
            let submission = command(&mut game, claimant, GameCommand::ClaimNaturalCount);
            assert_eq!(
                submission.result,
                if eligible {
                    CommandResult::Accepted
                } else {
                    CommandResult::RejectedNaturalCount
                }
            );
            assert_eq!(game.position(), &position);
            assert_eq!(game.warnings(), warnings);
            assert!(matches!(
                game.events()[events].event,
                EventKind::ClaimSubmitted {
                    automatic: false,
                    ..
                }
            ));
            if eligible {
                assert_eq!(
                    game.conclusion(),
                    Some(Conclusion::Draw {
                        reason: DrawReason::NaturalCount
                    })
                );
                assert!(
                    game.events()
                        .iter()
                        .any(|e| matches!(e.event, EventKind::ClaimAccepted { .. }))
                );
            } else {
                assert_eq!(game.conclusion(), None);
                assert!(matches!(
                    game.events().last().unwrap().event,
                    EventKind::ClaimRejected { .. }
                ));
            }
            assert_record_roundtrip(&game);
        }
    }
}

#[test]
fn imported_counter_never_reconstructs_missing_prior_check_or_claim_history() {
    let mut game = root_game(
        "5k2r/9/9/9/R8/9/9/9/9/3K5 w - - 100 51",
        ClaimPolicy::Explicit,
    );
    assert_eq!(game.position().counters().no_capture_plies(), Some(100));
    for claimant in [Side::Red, Side::Black] {
        let evidence = game.natural_count_eligibility(claimant);
        assert_eq!(
            (
                evidence.known_plies,
                evidence.claimant_checks,
                evidence.effective_plies,
                evidence.eligible
            ),
            (0, 0, 0, false)
        );
        assert_eq!(
            command(&mut game, claimant, GameCommand::ClaimNaturalCount).result,
            CommandResult::RejectedNaturalCount
        );
    }
    assert_eq!(game.conclusion(), None);
    assert_record_roundtrip(&game);
}

#[test]
fn unattended_policy_claims_are_recorded_red_first_when_both_are_eligible() {
    let fixture = source_fixtures(COUNT_FIXTURES)
        .into_iter()
        .find(|f| f["id"] == "NATURAL-10-CHECKS")
        .unwrap();
    let mut game = root_game(
        fixture["root_fen"].as_str().unwrap(),
        ClaimPolicy::AutoValidClaims,
    );
    play_fixture_prefix(&mut game, &fixture, 100);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Draw {
            reason: DrawReason::NaturalCount
        })
    );
    let claims: Vec<_> = game
        .events()
        .iter()
        .filter(|e| matches!(e.event, EventKind::ClaimSubmitted { .. }))
        .collect();
    assert_eq!(claims.len(), 1);
    assert_eq!(claims[0].actor, Some(Side::Red));
    assert!(matches!(
        claims[0].event,
        EventKind::ClaimSubmitted {
            automatic: true,
            ..
        }
    ));
    assert_record_roundtrip(&game);
}

#[test]
fn terminal_commands_keep_actor_semantics_and_board_ply_separate() {
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    play(&mut game, "a0a1");
    let before_offer = game.position().clone();
    // Offer follows the actor's own move, although the other player now has the turn.
    command(&mut game, Side::Red, GameCommand::OfferDraw);
    assert_eq!(game.position().side_to_move(), Side::Black);
    command(&mut game, Side::Black, GameCommand::AcceptDraw);
    assert_eq!(game.position(), &before_offer);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Draw {
            reason: DrawReason::Agreement
        })
    );
    assert!(game.events().iter().all(|e| e.board_ply == 1));
    let before = canonical(&game);
    let expected = game.context();
    assert_eq!(
        game.submit(GameRequest {
            expected,
            actor: Side::Black,
            command: GameCommand::Resign
        })
        .unwrap_err(),
        GameError::Concluded
    );
    assert_eq!(canonical(&game), before);
    assert_record_roundtrip(&game);
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    command(&mut game, Side::Black, GameCommand::Resign);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Red,
            reason: WinReason::Resignation
        })
    );
    assert_eq!(game.position().side_to_move(), Side::Red);
    assert_record_roundtrip(&game);
}

#[test]
fn native_command_refusals_are_atomic_and_unknown_no_win_is_not_a_result() {
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    assert_eq!(
        game.no_win_assessment(),
        NoWinAssessment::UnknownExactUnavailable
    );
    assert_eq!(game.conclusion(), None);
    assert_eq!(
        game.event_prefix(0).unwrap().no_win_assessment,
        NoWinAssessment::UnknownExactUnavailable
    );
    for (actor, cmd, error) in [
        (
            Side::Black,
            GameCommand::Play {
                action: "a0a1".parse::<Action>().unwrap(),
            },
            Some(GameError::WrongActor),
        ),
        (
            Side::Red,
            GameCommand::Play {
                action: "e0e9".parse().unwrap(),
            },
            None,
        ),
        (
            Side::Red,
            GameCommand::AcceptDraw,
            Some(GameError::DrawOffer),
        ),
    ] {
        let before = canonical(&game);
        let context = game.context();
        let result = game
            .submit(GameRequest {
                expected: context.clone(),
                actor,
                command: cmd,
            })
            .unwrap_err();
        if let Some(error) = error {
            assert_eq!(result, error);
        } else {
            assert!(matches!(result, GameError::Action(_)));
        }
        assert_eq!(canonical(&game), before);
        assert_eq!(game.context(), context);
    }
}

#[test]
fn canonical_records_reject_strict_parsing_forged_events_and_post_conclusion_commands() {
    let mut game = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    for action in C.into_iter().cycle().take(12) {
        play(&mut game, action);
    }
    let valid = canonical(&game);
    for malformed in [
        valid.replacen('{', "{\"game\":\"xiangqi\",", 1),
        valid.replacen(
            "\"root\":{",
            "\"root\":{\"profile\":\"mshogi.xiangqi.wxf2018.v1\",",
            1,
        ),
        valid.replacen("\"board_ply\":1", "\"board_ply\":1,\"board_ply\":1", 1),
        format!("{valid}{{}}"),
    ] {
        assert!(
            GameRecord::from_json(&malformed).is_err(),
            "strict duplicate/trailing rejection"
        );
    }
    let base: Value = serde_json::from_str(&valid).unwrap();
    for key in [
        "profile",
        "root",
        "commands",
        "events",
        "current_position",
        "conclusion",
        "pending",
        "no_win_assessment",
    ] {
        let mut changed = base.clone();
        changed.as_object_mut().unwrap().remove(key);
        assert!(
            GameRecord::from_json(&changed.to_string()).is_err(),
            "missing{key}"
        );
    }
    for (field, value) in [
        ("game", json!("chess")),
        ("profile", json!("mshogi.xiangqi.other")),
        ("edition", json!("other")),
        ("schema_version", json!(2)),
        ("mode", json!("timed")),
        ("no_win_goal", json!("resignations_count_as_win")),
        ("unexpected", json!(null)),
    ] {
        let mut changed = base.clone();
        changed[field] = value;
        assert!(
            GameRecord::from_json(&changed.to_string()).is_err(),
            "conflicting/unknown{field}"
        );
    }
    let mut forged = base.clone();
    let warning = forged["events"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|e| e["event"]["kind"] == "warning")
        .unwrap();
    warning["event"]["warning"]["occurrences"] = json!([0, 4, 7]);
    assert!(GameRecord::from_json(&forged.to_string()).is_err());
    let mut forged = base.clone();
    forged["conclusion"]["winner"] = json!("red");
    assert!(GameRecord::from_json(&forged.to_string()).is_err());
    let mut forged = base.clone();
    let extra = forged["commands"][0].clone();
    forged["commands"].as_array_mut().unwrap().push(extra);
    assert!(GameRecord::from_json(&forged.to_string()).is_err());
    let mut audited = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    command(&mut audited, Side::Black, GameCommand::ClaimNaturalCount);
    let mut forged: Value = serde_json::from_str(&canonical(&audited)).unwrap();
    forged["events"][0]["event"]["evidence"]["eligible"] = json!(true);
    assert!(GameRecord::from_json(&forged.to_string()).is_err());
}

#[test]
fn undo_of_a_locally_legal_fourth_return_restores_live_warning_without_retiring_evidence() {
    let mut game = root_game(CYCLE_ROOT, ClaimPolicy::Explicit);
    for action in C.into_iter().cycle().take(11) {
        play(&mut game, action);
    }
    let before = canonical(&game);
    let old_context = game.context();
    assert_eq!(game.position().side_to_move(), Side::Black);
    assert!(
        game.position()
            .legal_actions()
            .unwrap()
            .contains(&C[3].parse().unwrap())
    );
    let undo = play(&mut game, C[3]);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Black,
            reason: WinReason::Repetition {
                fault: FaultKind::PerpetualCheck
            },
        })
    );
    game.undo(undo).unwrap();
    assert_eq!(canonical(&game), before);
    assert_eq!(game.conclusion(), None);
    assert!(game.warnings()[0].active);
    assert_eq!(
        game.submit(GameRequest {
            expected: old_context,
            actor: Side::Black,
            command: GameCommand::Play {
                action: C[3].parse().unwrap()
            },
        })
        .unwrap_err(),
        GameError::StaleContext
    );
    play(&mut game, C[3]);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Black,
            reason: WinReason::Repetition {
                fault: FaultKind::PerpetualCheck
            },
        })
    );
}

#[test]
fn actual_mutual_cannon_checks_are_equal_fault_at_the_fourth_occurrence() {
    let f: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/history-mutual-check.json"
    ))
    .unwrap();
    let mut game = root_game(f["root_fen"].as_str().unwrap(), ClaimPolicy::Explicit);
    let cycle = ["b6d7", "d5e7", "d7b6", "e7d5"];
    for (i, a) in cycle.iter().cycle().take(12).enumerate() {
        let actor = game.position().side_to_move();
        play(&mut game, a);
        assert!(game.position().is_in_check(actor.opponent()));
        assert!(!game.position().is_in_check(actor));
        assert!(game.warnings().is_empty());
        if i < 11 {
            assert_eq!(game.conclusion(), None);
        }
    }
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Draw {
            reason: DrawReason::Repetition
        })
    );
    assert_record_roundtrip(&game);
}

#[test]
fn explicit_and_automatic_claims_obey_the_independently_established_100_102_order() {
    let f: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/history-precedence.json"
    ))
    .unwrap();
    let moves = f["moves"].as_array().unwrap();
    assert_eq!(moves.len(), 102);
    let mut explicit = root_game(f["root_fen"].as_str().unwrap(), ClaimPolicy::Explicit);
    let mut automatic = root_game(
        f["root_fen"].as_str().unwrap(),
        ClaimPolicy::AutoValidClaims,
    );
    for (i, a) in moves.iter().enumerate() {
        let name = a.as_str().unwrap();
        play(&mut explicit, name);
        if i < 100 {
            play(&mut automatic, name);
        }
        if i == 99 {
            for side in [Side::Red, Side::Black] {
                assert!(explicit.natural_count_eligibility(side).eligible);
            }
            assert_eq!(explicit.conclusion(), None);
            assert_eq!(
                automatic.conclusion(),
                Some(Conclusion::Draw {
                    reason: DrawReason::NaturalCount
                })
            );
            assert!(automatic.events().iter().any(|e| matches!(
                e.event,
                EventKind::ClaimSubmitted {
                    automatic: true,
                    ..
                }
            ) && e.actor == Some(Side::Red)));
        }
        if i < 101 {
            assert_eq!(explicit.conclusion(), None);
        }
    }
    assert_eq!(
        explicit.conclusion(),
        Some(Conclusion::Draw {
            reason: DrawReason::Repetition
        })
    );
    let before = canonical(&explicit);
    let expected = explicit.context();
    assert_eq!(
        explicit
            .submit(GameRequest {
                expected,
                actor: Side::Red,
                command: GameCommand::ClaimNaturalCount
            })
            .unwrap_err(),
        GameError::Concluded
    );
    assert_eq!(canonical(&explicit), before);
    assert_record_roundtrip(&explicit);
    assert_record_roundtrip(&automatic);
}
