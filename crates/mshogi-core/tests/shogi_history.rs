//! Rule-derived ledgers are maintained separately from native observations.
use mshogi_core::shogi::{
    Action, Conclusion, CounterSource, DeclarationEligibility, DrawReason, Game, GameCommand,
    GameError, GameRequest, GameRoot, HistoryOrigin, Position, RootCategory, RootError, Side,
    WinReason,
};
use serde_json::Value;

fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/shogi/history-outcomes.json"
    ))
    .unwrap()
}
fn side(s: &str) -> Side {
    match s {
        "sente" => Side::Sente,
        "gote" => Side::Gote,
        _ => panic!("unknown side {s}"),
    }
}
fn root(case: &Value) -> GameRoot {
    let category = match case["root_category"].as_str().unwrap() {
        "reduced_material" => RootCategory::ReducedMaterialDiagram,
        "full_material" => RootCategory::FullMaterialDiagram,
        _ => panic!("unknown category"),
    };
    let source = match case["counter_provenance"].as_str().unwrap() {
        "supplied" => CounterSource::Supplied,
        "defaulted" => CounterSource::Defaulted,
        _ => panic!("unknown source"),
    };
    GameRoot::import(
        Position::from_sfen(case["root_sfen"].as_str().unwrap()).unwrap(),
        category,
        source,
    )
    .unwrap()
}
fn submit(game: &mut Game, actor: Side, command: GameCommand) -> mshogi_core::shogi::GameUndo {
    game.submit(GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .unwrap()
}
fn expect_conclusion(actual: Option<Conclusion>, expected: &Value, label: &str) {
    if expected.is_null() {
        assert_eq!(actual, None, "{label}");
        return;
    }
    let actual = actual.unwrap_or_else(|| panic!("{label}: expected {expected}, got ongoing"));
    match (actual, expected["kind"].as_str().unwrap()) {
        (Conclusion::Win { winner, reason }, kind) => {
            assert_eq!(
                winner,
                side(expected["winner"].as_str().unwrap()),
                "{label}"
            );
            assert_eq!(
                winner.opponent(),
                side(expected["loser"].as_str().unwrap()),
                "{label}"
            );
            assert!(
                matches!(
                    (reason, kind),
                    (WinReason::ContinuousCheck { .. }, "continuous_check")
                        | (WinReason::NoLegalAction { .. }, "no_legal_action")
                        | (WinReason::EnteringKing, "entering_king")
                        | (WinReason::FalseDeclaration, "false_declaration")
                        | (WinReason::Resignation, "resignation")
                ),
                "{label}: {reason:?} vs {kind}"
            );
            assert_eq!(
                actual.score_half_points()[if winner == Side::Sente { 0 } else { 1 }],
                2
            );
        }
        (Conclusion::Draw(DrawReason::Fourfold { .. }), "repetition")
        | (Conclusion::Draw(DrawReason::Length), "length") => {
            assert_eq!(actual.score_half_points(), [1, 1])
        }
        _ => panic!("{label}: {actual:?} vs {expected}"),
    }
}

#[test]
fn independent_history_and_import_ledgers_and_exact_reversal() {
    let fixtures = corpus();
    assert_eq!(fixtures["format"], "mshogi.shogi.history-fixtures.v1");
    for case in fixtures["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let expected = &case["expected"];
        let mut game = Game::from_root(root(case));
        assert!(!game.root().has_complete_initial_history(), "{id}");
        assert_eq!(
            game.position().is_in_check(game.position().side_to_move()),
            expected["root_in_check"].as_bool().unwrap(),
            "{id}"
        );
        let mut prefixes = vec![game.clone()];
        for (index, action) in case["moves"].as_array().unwrap().iter().enumerate() {
            assert_eq!(
                game.conclusion(),
                None,
                "premature conclusion {id} prefix {index}"
            );
            let before = game.clone();
            let actor = game.position().side_to_move();
            let command = GameCommand::Play(action.as_str().unwrap().parse().unwrap());
            let undo = submit(&mut game, actor, command);
            let after = game.clone();
            assert_eq!(
                game.events().last().unwrap().gave_check,
                Some(expected["checks_after_actions"][index].as_bool().unwrap()),
                "{id} at {}",
                index + 1
            );
            game.undo(undo).unwrap();
            assert_eq!(game, before, "full undo {id} at {}", index + 1);
            submit(&mut game, actor, command);
            assert_eq!(game, after, "reapply {id} at {}", index + 1);
            prefixes.push(game.clone());
        }
        expect_conclusion(game.conclusion(), &expected["conclusion"], id);
        assert_eq!(
            u64::from(game.position().completed_actions()),
            expected["completed_actions"].as_u64().unwrap(),
            "{id}"
        );
        assert_eq!(
            game.occurrences(),
            expected["final_occurrence_indices"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap() as usize)
                .collect::<Vec<_>>(),
            "{id}"
        );
        for (index, expected_prefix) in prefixes.iter().enumerate() {
            assert_eq!(
                game.prefix(index).as_ref(),
                Ok(expected_prefix),
                "{id} prefix {index}"
            );
        }
        if let Some(Conclusion::Win {
            reason: WinReason::NoLegalAction { in_check },
            ..
        }) = game.conclusion()
        {
            assert_eq!(
                in_check,
                game.position().is_in_check(game.position().side_to_move()),
                "{id}"
            );
        }
        if game.conclusion().is_some() {
            let before = game.clone();
            for command in [GameCommand::Resign, GameCommand::DeclareEnteringKing] {
                assert_eq!(
                    game.submit(GameRequest {
                        expected: game.context(),
                        actor: Side::Sente,
                        command
                    })
                    .unwrap_err(),
                    GameError::Concluded,
                    "{id}"
                );
                assert_eq!(game, before, "terminal rejection {id}");
            }
        }
    }
}

#[test]
fn independently_expected_declaration_boundaries_and_same_ply_prefixes() {
    for case in corpus()["declaration_cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let e = &case["expected"];
        let mut game = Game::from_root(root(case));
        let original = game.clone();
        let actor = side(case["actor"].as_str().unwrap());
        let evidence = game.declaration_eligibility(actor);
        assert_eq!(
            evidence,
            DeclarationEligibility {
                own_turn: e["own_turn"].as_bool().unwrap(),
                king_in_camp: e["king_in_enemy_camp"].as_bool().unwrap(),
                camp_pieces: e["other_owned_camp_pieces"].as_u64().unwrap() as u8,
                points: e["points"].as_u64().unwrap() as u16,
                required_points: e["threshold"].as_u64().unwrap() as u16,
                not_in_check: !e["in_check"].as_bool().unwrap(),
                time_available: e["time_condition_satisfied"].as_bool().unwrap()
            },
            "{id}"
        );
        assert_eq!(
            evidence.eligible(),
            e["eligible"].as_bool().unwrap(),
            "{id}"
        );
        assert_eq!(game, original, "query changed {id}");
        let undo = submit(&mut game, actor, GameCommand::DeclareEnteringKing);
        assert_eq!(
            game.position(),
            original.position(),
            "command changed board/counter {id}"
        );
        assert_ne!(game.context(), original.context(), "same-ply context {id}");
        assert_eq!(game.events()[0].declaration, Some(evidence), "{id}");
        expect_conclusion(game.conclusion(), &e["formal_conclusion"], id);
        assert_eq!(game.prefix(0), Ok(original.clone()), "pre-command {id}");
        assert_eq!(game.prefix(1), Ok(game.clone()), "post-command {id}");
        game.undo(undo).unwrap();
        assert_eq!(game, original, "command undo {id}");
    }
}

#[test]
fn root_provenance_is_not_a_bare_positions_origin() {
    let mut position = Position::initial();
    position.apply("7g7f".parse().unwrap()).unwrap();
    assert_eq!(position.history_origin(), HistoryOrigin::Initial);
    assert_eq!(
        GameRoot::import(
            position.clone(),
            RootCategory::Standard,
            CounterSource::Supplied
        ),
        Err(RootError::StandardRequiresInitialRoot)
    );
    assert_eq!(
        GameRoot::import(
            position.clone(),
            RootCategory::FullMaterialDiagram,
            CounterSource::Defaulted
        ),
        Err(RootError::CounterProvenance)
    );
    assert_eq!(
        GameRoot::import(
            position.clone(),
            RootCategory::ReducedMaterialDiagram,
            CounterSource::Supplied
        ),
        Err(RootError::CategoryMismatch)
    );
    let imported = Game::from_root(
        GameRoot::import(
            position,
            RootCategory::FullMaterialDiagram,
            CounterSource::Supplied,
        )
        .unwrap(),
    );
    assert!(!imported.root().has_complete_initial_history());
    assert_eq!(
        imported.position().history_origin(),
        HistoryOrigin::Diagram {
            next_ply: 2,
            counter_source: CounterSource::Supplied
        }
    );
    let mut complete = Game::new();
    submit(
        &mut complete,
        Side::Sente,
        GameCommand::Play("7g7f".parse().unwrap()),
    );
    assert!(complete.root().has_complete_initial_history());
    assert_eq!(complete.position().to_sfen(), imported.position().to_sfen());
    assert_ne!(complete.context(), imported.context());
    assert_eq!(
        complete.position().repetition_identity(),
        imported.position().repetition_identity()
    );
    let both_checked = Position::from_sfen("k8/9/9/9/9/9/9/9/Kr7 b R 1").unwrap();
    assert!(both_checked.is_in_check(Side::Sente));
    // A broad diagram with the nonmoving king in check remains analyzable.
    let nonmoving_checked = Position::from_sfen("k8/9/R8/9/9/9/9/9/K8 b - 1").unwrap();
    assert!(nonmoving_checked.is_in_check(Side::Gote));
    assert!(!nonmoving_checked.legal_actions().is_empty());
    assert_eq!(
        GameRoot::import(
            nonmoving_checked,
            RootCategory::ReducedMaterialDiagram,
            CounterSource::Supplied
        ),
        Err(RootError::NonmovingKingAttacked)
    );
}

#[test]
fn rejection_is_atomic_and_resignation_does_not_require_own_turn() {
    let mut game = Game::new();
    let first_context = game.context();
    let before = game.clone();
    assert_eq!(
        game.submit(GameRequest {
            expected: game.context(),
            actor: Side::Gote,
            command: GameCommand::Play("3c3d".parse().unwrap())
        })
        .unwrap_err(),
        GameError::WrongActor
    );
    assert_eq!(game, before);
    assert!(matches!(
        game.submit(GameRequest {
            expected: game.context(),
            actor: Side::Sente,
            command: GameCommand::Play("7g7e".parse().unwrap())
        }),
        Err(GameError::IllegalAction(_))
    ));
    assert_eq!(game, before);
    submit(
        &mut game,
        Side::Sente,
        GameCommand::Play("7g7f".parse().unwrap()),
    );
    let advanced = game.clone();
    assert_eq!(
        game.submit(GameRequest {
            expected: first_context,
            actor: Side::Sente,
            command: GameCommand::DeclareEnteringKing
        })
        .unwrap_err(),
        GameError::StaleContext
    );
    assert_eq!(game, advanced);
    let undo = submit(&mut game, Side::Sente, GameCommand::Resign);
    assert_eq!(game.position(), advanced.position());
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Gote,
            reason: WinReason::Resignation
        })
    );
    game.undo(undo).unwrap();
    assert_eq!(game, advanced);
    let mut other = game.clone();
    let other_actor = other.position().side_to_move();
    let token = submit(&mut other, other_actor, GameCommand::Resign);
    assert!(game.undo(token).is_err());
    assert_eq!(game, advanced);
    // A malformed USI terminal word is not a board action.
    assert!("resign".parse::<Action>().is_err());
    assert!("win".parse::<Action>().is_err());
}

#[test]
fn identical_saved_position_does_not_supply_repetition_history() {
    let fixtures = corpus();
    let loop_case = &fixtures["cases"][0];
    let mut complete = Game::from_root(root(loop_case));
    for action in loop_case["moves"].as_array().unwrap() {
        let actor = complete.position().side_to_move();
        submit(
            &mut complete,
            actor,
            GameCommand::Play(action.as_str().unwrap().parse().unwrap()),
        );
    }
    let fresh = Game::from_root(
        GameRoot::import(
            complete.position().clone(),
            RootCategory::ReducedMaterialDiagram,
            CounterSource::Supplied,
        )
        .unwrap(),
    );
    assert_eq!(complete.position().to_sfen(), fresh.position().to_sfen());
    assert_eq!(
        complete.position().repetition_identity(),
        fresh.position().repetition_identity()
    );
    assert_eq!(complete.occurrences(), vec![0, 4, 8, 12]);
    assert_eq!(fresh.occurrences(), vec![0]);
    assert!(complete.conclusion().is_some());
    assert_eq!(fresh.conclusion(), None);
    assert_ne!(complete.context(), fresh.context());
    // The old root counter survives local moves; import establishes the present counter.
    assert_ne!(
        complete.position().history_origin(),
        fresh.position().history_origin()
    );
}
