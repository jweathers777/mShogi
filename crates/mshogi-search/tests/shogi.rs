//! Expected arithmetic/choices established independently before product search.
use mshogi_core::shogi::*;
use mshogi_search::alphabeta::timing::TimeBudget;
use mshogi_search::{search::SearchStatus, shogi::*};

fn timed_choice(game: &Game, ms: u64, nodes: u64) -> SearchResult {
    search_with_state(
        game,
        Limits {
            depth: MAX_DEPTH,
            nodes,
        },
        Some(TimeBudget::from_hard_ms(ms)),
        &mut new_engine(),
        || false,
    )
    .unwrap()
}

fn assert_move(game: &Game, result: &SearchResult) {
    let Some(Decision::Choose(Choice::Move(action))) = result.decision else {
        panic!("expected legal move: {result:?}")
    };
    assert!(game.position().legal_actions().contains(&action));
    let mut rules_position = game.clone();
    rules_position
        .submit(GameRequest {
            expected: rules_position.context(),
            actor: rules_position.position().side_to_move(),
            command: GameCommand::Play(action),
        })
        .unwrap();
    if result.completed_depth == 0 {
        assert_eq!(result.score, None);
        assert_eq!(result.principal_variation, vec![Choice::Move(action)]);
        assert_ne!(result.status, SearchStatus::Complete);
    }
}

#[test]
fn timed_minimum_and_expired_budget_return_moves_including_in_check() {
    for (rules_position, checked) in [
        (Game::new(), false),
        (game("4k4/9/4R4/9/9/9/9/9/K8 w - 1"), true),
    ] {
        assert_eq!(
            rules_position
                .position()
                .is_in_check(rules_position.position().side_to_move()),
            checked
        );
        assert!(rules_position.conclusion().is_none());
        let before = rules_position.clone();
        for ms in [0, 1] {
            for nodes in [0, 1, MAX_NODES] {
                let result = timed_choice(&rules_position, ms, nodes);
                assert_move(&rules_position, &result);
                assert!(result.nodes <= nodes);
                if ms == 0 || nodes == 0 {
                    assert_eq!(result.completed_depth, 0);
                    assert_eq!(result.nodes, 0);
                    assert_eq!(
                        choice(&result),
                        Choice::Move(rules_position.position().legal_actions()[0])
                    );
                }
            }
        }
        assert_eq!(rules_position, before);
    }
}

#[test]
fn msh187_failed_311_event_history_has_a_choice_with_one_millisecond() {
    let mut rules_position = Game::new();
    for action in include_str!("fixtures/msh187-round2-failed-0002.usi")
        .lines()
        .filter(|l| !l.starts_with('#'))
        .flat_map(str::split_whitespace)
    {
        submit(&mut rules_position, action);
    }
    assert_eq!(rules_position.events().len(), 311);
    assert!(rules_position.conclusion().is_none());
    let before = rules_position.clone();
    for ms in [0, 1] {
        assert_move(
            &rules_position,
            &timed_choice(&rules_position, ms, MAX_NODES),
        );
    }
    assert_eq!(rules_position, before);
}

#[test]
fn timed_fallback_respects_external_cancel_and_terminal_declaration() {
    let live = Game::new();
    for stop_at in [1, 2] {
        let mut polls = 0;
        let result = search_with_state(
            &live,
            Limits {
                depth: MAX_DEPTH,
                nodes: 0,
            },
            Some(TimeBudget::from_hard_ms(0)),
            &mut new_engine(),
            || {
                polls += 1;
                polls >= stop_at
            },
        )
        .unwrap();
        assert!(result.decision.is_none());
        assert!(result.score.is_none());
    }
    let terminal = game("k6rr/9/9/9/9/9/9/9/8K b - 322");
    let result = timed_choice(&terminal, 0, 0);
    assert_eq!(
        result.decision,
        terminal.conclusion().map(Decision::Finished)
    );
    assert_eq!((result.completed_depth, result.nodes), (0, 0));
    assert_eq!(result.score, Some(-WIN_SCORE));
    assert_eq!(result.status, SearchStatus::Complete);
    let declaring = game("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1");
    assert!(declaring.declaration_eligibility(Side::Sente).eligible());
    assert_move(&declaring, &timed_choice(&declaring, 0, 0));
    let result = timed_choice(&declaring, 0, 2);
    assert_eq!(choice(&result), Choice::Declare { actor: Side::Sente });
    assert_eq!((result.completed_depth, result.nodes), (1, 2));
    assert_eq!(result.score, Some(WIN_SCORE - 1));
}
fn game(sfen: &str) -> Game {
    Game::from_root(
        GameRoot::import(
            Position::from_sfen(sfen).unwrap(),
            RootCategory::ReducedMaterialDiagram,
            CounterSource::Supplied,
        )
        .unwrap(),
    )
}
fn submit(game: &mut Game, action: &str) {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: GameCommand::Play(action.parse().unwrap()),
    })
    .unwrap();
}
fn run(game: &Game, depth: u16) -> SearchResult {
    let before = game.clone();
    let r = search(
        game,
        Limits {
            depth,
            nodes: MAX_NODES,
        },
    )
    .unwrap();
    assert_eq!(*game, before);
    r
}
fn choice(result: &SearchResult) -> Choice {
    let Some(Decision::Choose(c)) = result.decision else {
        panic!("{result:?}")
    };
    c
}
#[test]
fn independent_capture_demote_promotion_and_mating_drop_selections() {
    // Sente's King stands on its castled square (see the optional-promotion test).
    for (sfen, action) in [
        ("8k/9/9/4p4/4R4/9/9/1K7/9 b - 1", "5e5d"),
        ("8k/9/9/4+p4/4R4/9/9/1K7/9 b - 1", "5e5d"),
        ("8k/9/4P4/9/9/9/9/1K7/9 b - 1", "5c5b+"),
        ("3lkl3/3p1p3/4K4/9/9/9/9/9/9 b G 1", "G*5b"),
    ] {
        let game = game(sfen);
        let result = run(&game, 1);
        assert_eq!(
            choice(&result),
            Choice::Move(action.parse().unwrap()),
            "{sfen}"
        );
        if action == "G*5b" {
            assert_eq!(result.score, Some(WIN_SCORE - 1));
        } else {
            assert!(result.score.unwrap() > 0);
        }
    }
    let result = run(&game("8k/9/9/4p4/4R4/9/9/9/K8 b - 1"), 2);
    assert!(result.score.unwrap() > 0);
    assert_eq!(
        result.principal_variation.first(),
        Some(&Choice::Move("5e5d".parse().unwrap()))
    );
}
#[test]
fn declaration_is_a_terminal_choice_at_root() {
    let root = "K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1";
    let mut rules_position = game(root);
    assert_eq!(
        rules_position.declaration_eligibility(Side::Sente).points,
        28
    );
    let result = run(&rules_position, 4);
    assert_eq!(choice(&result), Choice::Declare { actor: Side::Sente });
    assert_eq!(result.score, Some(WIN_SCORE - 1));
    assert_eq!(result.nodes, 2);
    assert_eq!(result.completed_depth, 1);
    let before = rules_position.position().clone();
    rules_position
        .submit(GameRequest {
            expected: rules_position.context(),
            actor: Side::Sente,
            command: choice(&result).command(),
        })
        .unwrap();
    assert_eq!(rules_position.position(), &before);
    assert_eq!(
        rules_position.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Sente,
            reason: WinReason::EnteringKing
        })
    );
    let mut invalid = game(&root.replace("B5P", "B4P"));
    assert!(!invalid.declaration_eligibility(Side::Sente).eligible());
    invalid
        .submit(GameRequest {
            expected: invalid.context(),
            actor: Side::Sente,
            command: GameCommand::DeclareEnteringKing,
        })
        .unwrap();
    let result = run(&invalid, 1);
    assert_eq!(result.score, Some(-WIN_SCORE));
    assert!(matches!(
        result.decision,
        Some(Decision::Finished(Conclusion::Win {
            reason: WinReason::FalseDeclaration,
            ..
        }))
    ));
}
#[test]
fn same_board_different_history_changes_choice_and_score() {
    let mut known = game("4k4/9/5R3/9/9/9/9/9/K8 b - 1");
    let cycle = ["4c3c", "5a4a", "3c4c", "4a5a"];
    for action in cycle.iter().cycle().take(11) {
        submit(&mut known, action);
    }
    assert_eq!(known.position().to_sfen(), "5k3/9/5R3/9/9/9/9/9/K8 w - 12");
    let fresh = game(&known.position().to_sfen());
    assert_eq!(known.position().to_sfen(), fresh.position().to_sfen());
    assert_eq!(
        known.position().repetition_identity(),
        fresh.position().repetition_identity()
    );
    assert_ne!(known.context(), fresh.context());
    let repeated = run(&known, 1);
    assert_eq!(choice(&repeated), Choice::Move("4a5a".parse().unwrap()));
    assert_eq!(repeated.score, Some(0));
    let imported = run(&fresh, 1);
    assert!(imported.score.unwrap() < 0);
    submit(&mut known, "4a5a");
    assert!(matches!(
        run(&known, 1).decision,
        Some(Decision::Finished(Conclusion::Draw(DrawReason::Fourfold {
            first: 0,
            fourth: 12
        })))
    ));
}
#[test]
fn the_game_adjudicates_a_cycle_across_captures_and_drops() {
    use mshogi_search::alphabeta::SearchPosition;
    use mshogi_search::shogi::position::ShogiPosition;

    let mut game = game("8k/9/9/9/4Gg3/9/9/9/K8 b g 1");
    let mut search = ShogiPosition::from_game(&game);
    // Two captures and two drops restore all three golds, both hands, and
    // the side to move. Adjudication uses the full game history.
    let cycle = ["5e4e", "G*5e", "G*5d", "5e5d", "4e5e", "5d4e"];
    for &token in cycle.iter().cycle().take(18) {
        let m = search.find(token.parse().unwrap()).unwrap();
        assert!(search.make(m));
        submit(&mut game, token);
    }
    assert_eq!(search.rule_score(), None);
    assert_eq!(ShogiPosition::from_game(&game).rule_score(), None);
    assert_eq!(
        run(&game, 1).decision,
        Some(Decision::Finished(Conclusion::Draw(DrawReason::Fourfold {
            first: 0,
            fourth: 18,
        })))
    );
}
#[test]
fn terminal_winner_precedes_static_score_and_declared_length() {
    for (root, cycle, score) in [
        (
            "4k4/9/4R4/9/9/9/9/9/K8 w - 1",
            ["5a4a", "5c4c", "4a5a", "4c5c"],
            WIN_SCORE,
        ),
        (
            "4k4/9/5R3/9/9/9/9/9/K8 b - 1",
            ["4c5c", "5a4a", "5c4c", "4a5a"],
            -WIN_SCORE,
        ),
    ] {
        let mut game = game(root);
        for action in cycle.iter().cycle().take(12) {
            submit(&mut game, action);
        }
        let result = run(&game, 1);
        assert_eq!(result.score, Some(score));
        assert!(matches!(
            result.decision,
            Some(Decision::Finished(Conclusion::Win {
                winner: Side::Gote,
                reason: WinReason::ContinuousCheck {
                    first: 0,
                    fourth: 12
                }
            }))
        ));
        if score > 0 {
            assert_eq!(
                game.events().last().unwrap().actor,
                Side::Sente,
                "mover loses yet current Gote wins"
            );
        }
    }
    for sfen in [
        "k6r1/9/9/9/9/9/9/r8/8K b - 321",
        "k6rr/9/9/9/9/9/9/9/8K b - 322",
    ] {
        let game = game(sfen);
        let result = run(&game, 1);
        assert_eq!(result.score, Some(-WIN_SCORE));
        assert!(matches!(
            result.decision,
            Some(Decision::Finished(Conclusion::Win {
                winner: Side::Gote,
                reason: WinReason::NoLegalAction { .. }
            }))
        ));
    }
    let length = game("4k4/9/5R3/9/9/9/9/9/K8 b - 321");
    let result = run(&length, 1);
    assert_eq!(result.score, Some(0));
    assert_eq!(
        result.decision,
        Some(Decision::Finished(Conclusion::Draw(DrawReason::Length)))
    );
    let before_length = game("4k4/9/5R3/9/9/9/9/9/K8 b - 320");
    let result = run(&before_length, 1);
    assert_eq!(result.score, Some(0));
    assert!(matches!(result.decision, Some(Decision::Choose(_))));
}
#[test]
fn bounded_completion_cancellation_and_root_restoration() {
    let game = Game::new();
    let before = game.clone();
    for nodes in [0, 1, 30, 100, 1000, 5000] {
        let result = search(&game, Limits { depth: 4, nodes }).unwrap();
        assert!(result.nodes <= nodes);
        if result.completed_depth == 0 {
            assert!(result.decision.is_none());
            assert!(result.score.is_none());
            assert!(result.principal_variation.is_empty());
        } else {
            let Choice::Move(action) = choice(&result) else {
                panic!()
            };
            assert!(game.position().legal_actions().contains(&action));
            assert_eq!(
                result.principal_variation.first(),
                Some(&Choice::Move(action))
            );
            assert!(result.score.is_some());
        }
        assert_eq!(game, before);
    }
    for stop_at in [1, 5, 65, 200] {
        let mut polls = 0;
        let result = search_with_stop(
            &game,
            Limits {
                depth: 64,
                nodes: MAX_NODES,
            },
            || {
                polls += 1;
                polls >= stop_at
            },
        )
        .unwrap();
        assert_eq!(result.status, SearchStatus::Stopped);
        assert_eq!(game, before);
        if result.completed_depth == 0 {
            assert!(result.score.is_none());
            assert!(result.decision.is_none());
        }
    }
    for limits in [
        Limits { depth: 0, nodes: 1 },
        Limits {
            depth: MAX_DEPTH + 1,
            nodes: 1,
        },
        Limits {
            depth: 1,
            nodes: MAX_NODES + 1,
        },
    ] {
        assert_eq!(search(&game, limits), Err(SearchError::InvalidLimits));
        assert_eq!(game, before);
    }
}

#[test]
fn optional_promotion_and_drop_branching_produce_lines() {
    // The Sente King stands on its castled square: tuned king-placement
    // terms (MSH-188) make a King step from the corner worth more than a
    // depth-one promotion, which is not what these cases test.
    let optional = game("8k/9/9/4P4/9/9/9/1K7/9 b - 1");
    let result = run(&optional, 1);
    assert!(result.score.unwrap() > 0);
    assert_eq!(choice(&result), Choice::Move("5d5c+".parse().unwrap()));
    let hands = game("8k/9/9/9/9/9/9/9/K8 b RBGSNLPrbgsnlp 1");
    let result = run(&hands, 1);
    let mut replay = hands.clone();
    for c in result.principal_variation {
        replay
            .submit(GameRequest {
                expected: replay.context(),
                actor: replay.position().side_to_move(),
                command: c.command(),
            })
            .unwrap();
    }
    assert_eq!(result.completed_depth, 1);
}
#[test]
fn search_does_not_reward_a_legal_checking_move_that_loses_by_history() {
    let mut known = game("4k4/9/4R4/9/9/9/9/9/K8 w - 1");
    for action in ["5a4a", "5c4c", "4a5a", "4c5c"].iter().cycle().take(11) {
        submit(&mut known, action);
    }
    let mut losing = known.clone();
    submit(&mut losing, "4c5c");
    assert!(matches!(
        losing.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Gote,
            reason: WinReason::ContinuousCheck { .. }
        })
    ));
    // The checking repetition loses; the engine must choose a legal alternative.
    let result = run(&known, 1);
    assert!(result.score.unwrap() > 0);
    assert_ne!(choice(&result), Choice::Move("4c5c".parse().unwrap()));
}
