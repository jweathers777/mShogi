//! Native history admission and strict canonical boundary, independent of consumers.
use mshogi_core::xiangqi::*;
use serde_json::{Value, json};
fn submit(game: &mut Game, actor: Side, command: GameCommand) -> Result<Submission, GameError> {
    game.submit(GameRequest {
        expected: game.context(),
        actor,
        command,
    })
}
fn record(game: &Game) -> String {
    GameRecord::from_game(game).unwrap().to_json().to_owned()
}

#[test]
fn bare_general_fast_proof_is_a_closed_board_invariant_not_a_penalty_or_forced_win_table() {
    let mut admitted = 0;
    for red in Intersection::all().filter(|p| Side::Red.in_palace(*p)) {
        for black in Intersection::all().filter(|p| Side::Black.in_palace(*p)) {
            for side in [Side::Red, Side::Black] {
                let p = Position::from_diagram(
                    &[
                        (red, Piece::new(Side::Red, PieceKind::General)),
                        (black, Piece::new(Side::Black, PieceKind::General)),
                    ],
                    side,
                    Counters::unknown(),
                )
                .unwrap();
                if red.file() == black.file() {
                    assert!(p.validate_local().is_err());
                    continue;
                }
                admitted += 1;
                assert_eq!(assess_natural_win(&p), NoWinAssessment::ProvenNeitherCanWin);
                let actions = p.legal_actions().unwrap();
                assert!(!actions.is_empty());
                for a in actions {
                    let mut next = p.clone();
                    next.apply(a).unwrap();
                    assert!(
                        next.pieces()
                            .all(|(_, piece)| piece.kind == PieceKind::General)
                    );
                    assert!(!next.legal_actions().unwrap().is_empty());
                    assert!(!next.is_in_check(Side::Red));
                    assert!(!next.is_in_check(Side::Black));
                }
                let game =
                    Game::from_root(PositionRoot::analysis(p), ClaimPolicy::Explicit).unwrap();
                assert_eq!(
                    game.conclusion(),
                    Some(Conclusion::Draw {
                        reason: DrawReason::NeitherCanWin
                    })
                );
                assert_eq!(
                    GameRecord::from_json(&record(&game)).unwrap().to_json(),
                    record(&game)
                );
            }
        }
    }
    // 9x9 placements minus27 facing pairs, times2turns. Each General always
    // has a vertical palace move on its different file. No capture/check exists.
    assert_eq!(admitted, 108);
}

#[test]
fn natural_win_witness_includes_stalemate_but_excludes_f002_penalty_award() {
    for (fen, reason) in [
        (
            "4k4/9/9/9/9/4p4/9/9/3r1r3/4K4 w - - 0 1",
            NoActionReason::Stalemate,
        ),
        (
            "4k4/9/9/9/9/4p4/9/2n6/5r3/3rK4 w - - 0 1",
            NoActionReason::Checkmate,
        ),
    ] {
        let game =
            Game::from_root(PositionRoot::from_fen(fen).unwrap(), ClaimPolicy::Explicit).unwrap();
        assert_eq!(
            game.conclusion(),
            Some(Conclusion::Win {
                winner: Side::Black,
                reason: WinReason::NoLegalAction { reason }
            })
        );
        assert_eq!(
            game.no_win_assessment(),
            NoWinAssessment::ProvenNaturalWin {
                winner: Side::Black,
                witness: vec![]
            }
        );
        assert_eq!(game.events()[0].board_ply, 0);
    }
    let mut precursor = Position::from_fen("4k4/9/9/9/9/4p4/9/2n6/3r1r3/4K4 b - - 0 1").unwrap();
    let assessment = assess_natural_win(&precursor);
    if let NoWinAssessment::ProvenNaturalWin { winner, witness } = assessment {
        assert_eq!(winner, Side::Black);
        assert_eq!(witness.len(), 1);
        for a in witness {
            precursor.apply(a).unwrap();
        }
        assert!(matches!(
            precursor.local_status(),
            Ok(LocalStatus::NoLegalAction { .. })
        ));
    } else {
        panic!("missing independently constructed one-ply natural win");
    }
    let mut cycle = Game::from_root(
        PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        ClaimPolicy::Explicit,
    )
    .unwrap();
    for _ in 0..3 {
        for a in ["e9f9", "f8e8", "f9e9", "e8f8"] {
            let actor = cycle.position().side_to_move();
            submit(
                &mut cycle,
                actor,
                GameCommand::Play {
                    action: a.parse().unwrap(),
                },
            )
            .unwrap();
        }
    }
    assert!(matches!(
        cycle.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Black,
            reason: WinReason::Repetition { .. }
        })
    ));
    assert!(!matches!(
        cycle.no_win_assessment(),
        NoWinAssessment::ProvenNaturalWin {
            winner: Side::Black,
            ..
        }
    ));
    assert!(!matches!(
        cycle.no_win_assessment(),
        NoWinAssessment::ProvenNeitherCanWin
    ));
    // No claim that Black is intrinsically impossible: missing a proof is unknown.
    assert_eq!(
        assess_natural_win(&Position::initial()),
        NoWinAssessment::UnknownExactUnavailable
    );
    let bad = Position::from_fen("4k4/9/9/9/9/9/9/9/9/4K4 w - - 0 1").unwrap();
    assert_eq!(assess_natural_win(&bad), NoWinAssessment::Failure);
}

#[test]
fn every_record_field_is_required_and_native_raw_payloads_reject_duplicates() {
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    submit(
        &mut game,
        Side::Red,
        GameCommand::Play {
            action: "a0a1".parse().unwrap(),
        },
    )
    .unwrap();
    submit(&mut game, Side::Black, GameCommand::ClaimNaturalCount).unwrap();
    let text = record(&game);
    let value: Value = serde_json::from_str(&text).unwrap();
    for key in value.as_object().unwrap().keys() {
        let mut v = value.clone();
        v.as_object_mut().unwrap().remove(key);
        assert!(
            GameRecord::from_json(&v.to_string()).is_err(),
            "missing outer{key}"
        );
    }
    for (tag, field) in [("root", "side"), ("current_position", "side")] {
        let marker = format!("\"{tag}\":");
        let start = text.find(&marker).unwrap() + marker.len();
        let end = start + text[start..].find('}').unwrap() + 1;
        let raw = &text[start..end];
        let duplicate = raw.replacen(
            &format!("\"{field}\":"),
            &format!("\"{field}\":\"red\",\"{field}\":"),
            1,
        );
        let malformed = format!("{}{}{}", &text[..start], duplicate, &text[end..]);
        assert!(
            GameRecord::from_json(&malformed).is_err(),
            "duplicate native{tag}"
        );
    }
    for (key, val) in [
        ("mode", json!("timed")),
        ("game", json!("chess")),
        ("profile", json!("mshogi.xiangqi.other")),
        ("no_win_goal", json!("perpetual-fault-win")),
        ("no_win_mode", json!("bounded_exact")),
        ("fast_witness_limit", json!(2)),
    ] {
        let mut v = value.clone();
        v[key] = val;
        assert_eq!(
            GameRecord::from_json(&v.to_string()),
            Err(RecordError::Unsupported)
        );
    }
    let root_field = "\"schema_version\":1";
    let dup = text.replacen(root_field, "\"schema_version\":1,\"schema_version\":1", 1);
    assert!(GameRecord::from_json(&dup).is_err());
    let mut extra = value.clone();
    extra["unknown"] = json!(true);
    assert!(GameRecord::from_json(&extra.to_string()).is_err());
    let mut missing = value.clone();
    missing["events"][0]
        .as_object_mut()
        .unwrap()
        .remove("actor");
    assert!(GameRecord::from_json(&missing.to_string()).is_err());
    // All native parsers must run before any harmless key-order normalization.
    let pretty = serde_json::to_string_pretty(&value).unwrap();
    assert_eq!(GameRecord::from_json(&pretty).unwrap().to_json(), text);
}

#[test]
fn saveability_survives_audit_capacity_and_retains_reserved_native_close_path() {
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    for _ in 0..MAX_GAME_COMMANDS - 1 {
        let result = submit(&mut game, Side::Red, GameCommand::ClaimNaturalCount).unwrap();
        assert_eq!(result.result, CommandResult::RejectedNaturalCount);
    }
    let full = record(&game);
    let context = game.context();
    assert_eq!(
        submit(&mut game, Side::Black, GameCommand::ClaimNaturalCount).unwrap_err(),
        GameError::Capacity
    );
    assert_eq!(game.context(), context);
    assert_eq!(record(&game), full);
    assert_eq!(GameRecord::from_json(&full).unwrap().to_json(), full);
    submit(&mut game, Side::Black, GameCommand::Resign).unwrap();
    assert!(matches!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Red,
            reason: WinReason::Resignation
        })
    ));
    let closed = record(&game);
    assert_eq!(GameRecord::from_json(&closed).unwrap().to_json(), closed);
    let small = record(&Game::standard(ClaimPolicy::Explicit).unwrap());
    let padded = format!(
        "{}{}",
        small,
        " ".repeat(MAX_GAME_RECORD_BYTES - small.len())
    );
    assert!(GameRecord::from_json(&padded).is_ok());
    assert_eq!(
        GameRecord::from_json(&(padded + " ")),
        Err(RecordError::Capacity)
    );
    let mut v: Value = serde_json::from_str(&full).unwrap();
    let e = v["events"][0].clone();
    v["events"] = Value::Array(vec![e; MAX_GAME_EVENTS + 1]);
    assert_eq!(
        GameRecord::from_json(&v.to_string()),
        Err(RecordError::Capacity)
    );
    let mut v: Value = serde_json::from_str(&full).unwrap();
    let c = v["commands"][0].clone();
    v["commands"] = Value::Array(vec![c; MAX_GAME_COMMANDS + 1]);
    assert_eq!(
        GameRecord::from_json(&v.to_string()),
        Err(RecordError::Capacity)
    );
}

#[test]
fn multi_step_semantic_undo_does_not_reactivate_abandoned_branch_tokens() {
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    let root = record(&game);
    let old_context = game.context();
    let a = submit(
        &mut game,
        Side::Red,
        GameCommand::Play {
            action: "a0a1".parse().unwrap(),
        },
    )
    .unwrap()
    .undo;
    let after_a = record(&game);
    let b = submit(&mut game, Side::Black, GameCommand::ClaimNaturalCount)
        .unwrap()
        .undo;
    game.undo(b).unwrap();
    assert_eq!(record(&game), after_a);
    game.undo(a).unwrap();
    assert_eq!(record(&game), root);
    assert_ne!(game.context(), old_context);
    let abandoned = submit(
        &mut game,
        Side::Red,
        GameCommand::Play {
            action: "a0a1".parse().unwrap(),
        },
    )
    .unwrap()
    .undo;
    let current = record(&game);
    // Reopen exactly the same canonical value in a new runtime; old semantic
    // undo/request identities cannot mutate it.
    let mut reopened = GameRecord::from_json(&current).unwrap().to_game().unwrap();
    assert_eq!(reopened.undo(abandoned), Err(GameError::StaleContext));
}

#[test]
fn agreement_offer_admission_preserves_native_actor_order_without_tournament_penalties() {
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    assert_eq!(
        submit(&mut game, Side::Red, GameCommand::OfferDraw).unwrap_err(),
        GameError::DrawOffer
    );
    submit(
        &mut game,
        Side::Red,
        GameCommand::Play {
            action: "a0a1".parse().unwrap(),
        },
    )
    .unwrap();
    assert_eq!(
        submit(&mut game, Side::Black, GameCommand::OfferDraw).unwrap_err(),
        GameError::DrawOffer
    );
    submit(&mut game, Side::Red, GameCommand::OfferDraw).unwrap();
    let offered = record(&game);
    assert_eq!(
        submit(&mut game, Side::Red, GameCommand::AcceptDraw).unwrap_err(),
        GameError::DrawOffer
    );
    assert_eq!(record(&game), offered);
    submit(&mut game, Side::Black, GameCommand::DeclineDraw).unwrap();
    assert_eq!(
        submit(&mut game, Side::Red, GameCommand::OfferDraw).unwrap_err(),
        GameError::DrawOffer
    );
    submit(
        &mut game,
        Side::Black,
        GameCommand::Play {
            action: "a9a8".parse().unwrap(),
        },
    )
    .unwrap();
    submit(&mut game, Side::Black, GameCommand::OfferDraw).unwrap();
    let before = game.position().clone();
    submit(&mut game, Side::Red, GameCommand::AcceptDraw).unwrap();
    assert_eq!(game.position(), &before);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Draw {
            reason: DrawReason::Agreement
        })
    );
    assert!(GameRecord::from_json(&record(&game)).is_ok());
    let mut v: Value = serde_json::from_str(&offered).unwrap();
    v["commands"][1]["command"] = json!({"kind":"withdraw_draw"});
    assert!(GameRecord::from_json(&v.to_string()).is_err());
}

#[test]
fn chase_warning_retires_on_proved_check_variation_not_on_unanswered_chase() {
    let root = "4R4/5k3/c8/9/9/9/9/9/9/3K5 w - - 0 1";
    let mut game =
        Game::from_root(PositionRoot::from_fen(root).unwrap(), ClaimPolicy::Explicit).unwrap();
    for _ in 0..2 {
        for a in ["e9a9", "a7e7", "a9e9", "e7a7"] {
            let actor = game.position().side_to_move();
            submit(
                &mut game,
                actor,
                GameCommand::Play {
                    action: a.parse().unwrap(),
                },
            )
            .unwrap();
        }
    }
    let warning = game.warnings()[0].clone();
    assert!(warning.active);
    assert_eq!(warning.fault, FaultKind::PerpetualChase);
    let before = game.events().len();
    submit(
        &mut game,
        Side::Red,
        GameCommand::Play {
            action: "e9f9".parse().unwrap(),
        },
    )
    .unwrap();
    assert!(!game.warnings()[0].active);
    assert!(game.events()[before..].iter().any(|e|matches!(e.event,EventKind::WarningRetired{warning_event_index} if warning_event_index==warning.event_index)));
    let after = game.position().clone();
    let old = game.event_prefix(before).unwrap();
    assert!(old.warnings[0].active);
    let current = game.event_prefix(game.events().len()).unwrap();
    assert!(!current.warnings[0].active);
    assert_eq!(current.position, after);
    assert!(GameRecord::from_json(&record(&game)).is_ok());
}

#[test]
fn unanswered_general_attack_cannot_invent_an_exemption_for_prohibited_cannon_chase() {
    let root = "5k3/9/9/9/9/3C5/9/4c4/4K4/3n5 w - - 0 1";
    let mut game =
        Game::from_root(PositionRoot::from_fen(root).unwrap(), ClaimPolicy::Explicit).unwrap();
    let cycle = ["e1d1", "e2d2", "d1e1", "d2e2"];
    for a in cycle.iter().cycle().take(8) {
        let actor = game.position().side_to_move();
        submit(
            &mut game,
            actor,
            GameCommand::Play {
                action: a.parse().unwrap(),
            },
        )
        .unwrap();
    }
    let original = game.warnings()[0].event_index;
    for a in cycle {
        let actor = game.position().side_to_move();
        submit(
            &mut game,
            actor,
            GameCommand::Play {
                action: a.parse().unwrap(),
            },
        )
        .unwrap();
        assert!(
            game.warnings()
                .iter()
                .any(|w| w.event_index == original && w.active)
        );
    }
    assert!(matches!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Black,
            reason: WinReason::Repetition {
                fault: FaultKind::PerpetualChase
            }
        })
    ));
    assert!(GameRecord::from_json(&record(&game)).is_ok());
}

#[test]
fn a_proven_nonchase_retires_now_but_an_unanswered_real_chase_does_not() {
    let root = "4R4/5k3/c8/9/9/9/9/9/9/3K5 w - - 0 1";
    for (first, reply, active_at9) in [("e9h9", None, false), ("e9a9", Some("f8f7"), true)] {
        let mut game =
            Game::from_root(PositionRoot::from_fen(root).unwrap(), ClaimPolicy::Explicit).unwrap();
        for a in ["e9a9", "a7e7", "a9e9", "e7a7"].iter().cycle().take(8) {
            let actor = game.position().side_to_move();
            submit(
                &mut game,
                actor,
                GameCommand::Play {
                    action: a.parse().unwrap(),
                },
            )
            .unwrap();
        }
        let old = game.warnings()[0].event_index;
        submit(
            &mut game,
            Side::Red,
            GameCommand::Play {
                action: first.parse().unwrap(),
            },
        )
        .unwrap();
        assert_eq!(
            game.warnings()
                .iter()
                .find(|w| w.event_index == old)
                .unwrap()
                .active,
            active_at9
        );
        if let Some(reply) = reply {
            submit(
                &mut game,
                Side::Black,
                GameCommand::Play {
                    action: reply.parse().unwrap(),
                },
            )
            .unwrap();
            assert!(
                !game
                    .warnings()
                    .iter()
                    .find(|w| w.event_index == old)
                    .unwrap()
                    .active
            );
        }
        assert!(GameRecord::from_json(&record(&game)).is_ok());
    }
}
