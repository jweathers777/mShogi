//! Expected arithmetic/choices established independently before product search.
use mshogi_core::shogi::*;
use mshogi_search::{search::SearchStatus, shogi::*};
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
fn independent_material_arithmetic_covers_promoted_board_and_both_hands() {
    let p = Position::from_sfen("8k/9/9/9/9/9/9/9/K8 b 2R2B4G4S4N4L18P 1").unwrap();
    assert_eq!(
        evaluate(&p),
        Evaluation {
            sente: 10800,
            gote: 0
        }
    );
    assert_eq!(evaluate(&p).score_for(Side::Gote), -10800);
    let mut board = [None; 81];
    board["9i".parse::<Square>().unwrap().index()] =
        Piece::new(Side::Sente, PieceKind::King, false);
    board["1a".parse::<Square>().unwrap().index()] = Piece::new(Side::Gote, PieceKind::King, false);
    for (i, kind) in [
        PieceKind::Pawn,
        PieceKind::Lance,
        PieceKind::Knight,
        PieceKind::Silver,
        PieceKind::Bishop,
        PieceKind::Rook,
    ]
    .into_iter()
    .enumerate()
    {
        board[Square::new((i + 1) as u8, 5).unwrap().index()] = Piece::new(Side::Sente, kind, true);
    }
    board["8h".parse::<Square>().unwrap().index()] =
        Piece::new(Side::Sente, PieceKind::Gold, false);
    let hands = [
        Hand::from_counts(&[(PieceKind::Pawn, 2)]).unwrap(),
        Hand::from_counts(&[
            (PieceKind::Rook, 1),
            (PieceKind::Bishop, 1),
            (PieceKind::Gold, 1),
            (PieceKind::Silver, 1),
            (PieceKind::Knight, 1),
            (PieceKind::Lance, 1),
            (PieceKind::Pawn, 3),
        ])
        .unwrap(),
    ];
    let p = Position::from_diagram(board, hands, Side::Gote, Some(17)).unwrap();
    assert_eq!(
        evaluate(&p),
        Evaluation {
            sente: 4600,
            gote: 3300
        }
    );
    assert_eq!(evaluate(&p).score_for(Side::Sente), 1300);
    assert_eq!(evaluate(&p).score_for(Side::Gote), -1300);
    assert_ne!(EVALUATOR_ID, SEARCH_ID);
}
#[test]
fn independent_capture_demote_promotion_and_mating_drop_selections() {
    for (sfen, action, score) in [
        ("8k/9/9/4p4/4R4/9/9/9/K8 b - 1", "5e5d", 900),
        ("8k/9/9/4+p4/4R4/9/9/9/K8 b - 1", "5e5d", 900),
        ("8k/9/4P4/9/9/9/9/9/K8 b - 1", "5c5b+", 500),
        ("3lkl3/3p1p3/4K4/9/9/9/9/9/9 b G 1", "G*5b", WIN_SCORE - 1),
    ] {
        let game = game(sfen);
        let result = run(&game, 1);
        assert_eq!(
            choice(&result),
            Choice::Move(action.parse().unwrap()),
            "{sfen}"
        );
        assert_eq!(result.score, Some(score));
    }
    let result = run(&game("8k/9/9/4p4/4R4/9/9/9/K8 b - 1"), 2);
    assert_eq!(result.score, Some(900));
    assert_eq!(
        result.principal_variation,
        vec![
            Choice::Move("5e5d".parse().unwrap()),
            Choice::Move("1a1b".parse().unwrap())
        ]
    );
}
#[test]
fn declaration_is_a_native_terminal_choice_at_root_and_static_leaves() {
    let root = "K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1";
    let mut native = game(root);
    assert_eq!(evaluate(native.position()).score_for(Side::Sente), 6700);
    assert_eq!(native.declaration_eligibility(Side::Sente).points, 28);
    let result = run(&native, 4);
    assert_eq!(choice(&result), Choice::Declare { actor: Side::Sente });
    assert_eq!(result.score, Some(WIN_SCORE - 1));
    assert_eq!(result.nodes, 2);
    assert_eq!(result.completed_depth, 1);
    let before = native.position().clone();
    native
        .submit(GameRequest {
            expected: native.context(),
            actor: Side::Sente,
            command: choice(&result).command(),
        })
        .unwrap();
    assert_eq!(native.position(), &before);
    assert_eq!(
        native.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Sente,
            reason: WinReason::EnteringKing
        })
    );
    // With Gote to move all three remote king replies admit Sente's declaration.
    // B7b's ray is blocked by own promoted pawn6c, so1i1h is legal and lexical first.
    let leaf = game(&root.replace(" b ", " w "));
    let result = run(&leaf, 1);
    assert_eq!(result.score, Some(-WIN_SCORE + 2));
    assert_eq!(
        result.principal_variation,
        vec![
            Choice::Move("1i1h".parse().unwrap()),
            Choice::Declare { actor: Side::Sente }
        ]
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
    assert_eq!(choice(&imported), Choice::Move("4a3a".parse().unwrap()));
    assert_eq!(imported.score, Some(-800));
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
fn native_terminal_winner_precedes_static_score_and_declared_length() {
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
        assert_eq!(evaluate(game.position()).score_for(Side::Sente), -1600);
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
fn exact_work_completion_cancellation_and_root_restoration() {
    let game = Game::new();
    let before = game.clone();
    for (nodes, depth, completed, visits) in [
        (0, 2, 0, 0),
        (1, 2, 0, 1),
        (31, 1, 1, 31),
        (32, 2, 1, 32),
        (962, 2, 2, 962),
    ] {
        let result = search(&game, Limits { depth, nodes }).unwrap();
        assert_eq!(result.nodes, visits);
        assert_eq!(result.completed_depth, completed);
        assert_eq!(result.score, if completed == 0 { None } else { Some(0) });
        if completed == 0 {
            assert!(result.decision.is_none());
            assert!(result.principal_variation.is_empty());
        } else {
            assert_eq!(choice(&result), Choice::Move("1g1f".parse().unwrap()));
        }
        assert_eq!(game, before);
    }
    for stop_at in [1, 5, 65, 200] {
        let mut polls = 0;
        let result = search_with_stop(
            &game,
            Limits {
                depth: 2,
                nodes: MAX_NODES,
            },
            || {
                polls += 1;
                polls >= stop_at
            },
        )
        .unwrap();
        assert_eq!(result.status, SearchStatus::Stopped);
        assert!(result.nodes < 962);
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
fn full_width_preserves_optional_choices_and_drop_branching() {
    let optional = game("8k/9/9/4P4/9/9/9/9/K8 b - 1");
    // Three king actions, plus both5d5c alternatives: six visited nodes.
    let result = run(&optional, 1);
    assert_eq!(result.nodes, 6);
    assert_eq!(result.score, Some(500));
    assert_eq!(choice(&result), Choice::Move("5d5c+".parse().unwrap()));
    let hands = game("8k/9/9/9/9/9/9/9/K8 b RBGSNLPrbgsnlp 1");
    let result = run(&hands, 1);
    assert_eq!(
        result.nodes,
        1 + hands.position().legal_actions().len() as u64
    );
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
    // Rook can instead promote via4c1c+ for1000 material, with no capture/mate.
    let result = run(&known, 1);
    assert_eq!(result.score, Some(1000));
    assert_eq!(choice(&result), Choice::Move("4c1c+".parse().unwrap()));
}
