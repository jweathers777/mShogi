use super::*;
use mshogi_core::chu::{Base, Piece, RootMetadata};
fn diagram(entries: &[(&str, Side, Base, bool)], turn: Side) -> Game {
    Game::from_root(
        Position::from_diagram(
            &entries
                .iter()
                .map(|(s, side, b, p)| (s.parse().unwrap(), Piece::new(*side, *b, *p).unwrap()))
                .collect::<Vec<_>>(),
            turn,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
fn tactical() -> Game {
    diagram(
        &[
            ("12l", Side::Sente, Base::King, false),
            ("1a", Side::Gote, Base::King, false),
            ("6f", Side::Sente, Base::Lion, false),
            ("6e", Side::Gote, Base::Pawn, false),
            ("5e", Side::Gote, Base::Rook, false),
        ],
        Side::Sente,
    )
}
#[test]
fn material_is_native_face_owned_and_not_an_outcome() {
    let g = tactical();
    let e = evaluate(g.position());
    assert_eq!(
        e,
        Evaluation {
            sente: 7000,
            gote: 6000
        }
    );
    assert_eq!(e.score_for(Side::Gote), -1000);
    assert_eq!(piece_value(Face::Prince), piece_value(Face::King));
    assert_eq!(g.conclusion(), None);
}
#[test]
fn complete_compound_capture_search_restores_exact_context() {
    let g = tactical();
    let before = GameRecord::from_game(&g).unwrap();
    let context = g.context();
    let snapshot = g.snapshot().clone();
    let r = search(
        &g,
        Limits {
            depth: 1,
            nodes: 512,
        },
    )
    .unwrap();
    assert_eq!(r.completed_depth, 1);
    assert_eq!(r.status, SearchStatus::Complete);
    assert_eq!(r.score, Some(WIN_SCORE - 1));
    let Some(Decision::Choose(a)) = r.decision else {
        panic!("{r:?}")
    };
    assert_eq!(a.from, "6f".parse().unwrap());
    assert_eq!(a.route.len(), 2);
    assert!(a.route.contains(&"6e".parse().unwrap()));
    assert!(a.route.contains(&"5e".parse().unwrap()));
    assert!(g.legal_actions().contains(&a));
    assert_eq!(GameRecord::from_game(&g).unwrap(), before);
    assert_eq!(g.context(), context);
    assert_eq!(g.snapshot(), &snapshot);
}
#[test]
fn zero_and_partial_work_never_supply_a_fallback() {
    let g = tactical();
    for n in [0, 1, 2] {
        let r = search(&g, Limits { depth: 1, nodes: n }).unwrap();
        assert_eq!(r.status, SearchStatus::NodeLimit);
        assert_eq!(r.decision, None);
        assert_eq!(r.score, None);
        assert!(r.principal_variation.is_empty());
        assert_eq!(r.nodes, n);
    }
    let r = search_with_stop(
        &g,
        Limits {
            depth: 1,
            nodes: 512,
        },
        || true,
    )
    .unwrap();
    assert_eq!(r.status, SearchStatus::Stopped);
    assert_eq!(r.nodes, 0);
    assert_eq!(r.decision, None);
    let mut checks = 0;
    let r = search_with_stop(
        &g,
        Limits {
            depth: 1,
            nodes: 512,
        },
        || {
            checks += 1;
            checks > 5
        },
    )
    .unwrap();
    assert_eq!(r.status, SearchStatus::Stopped);
    assert_eq!(r.decision, None);
    assert_eq!(g.events().len(), 0);
}
#[test]
fn native_conclusions_and_draws_have_native_terminal_scores() {
    for conclusion in [
        Conclusion::RoyalCapture {
            winner: Side::Sente,
        },
        Conclusion::Mate {
            winner: Side::Sente,
        },
        Conclusion::Material {
            winner: Side::Sente,
        },
        Conclusion::Resignation {
            winner: Side::Sente,
        },
    ] {
        assert_eq!(terminal_score(conclusion, Side::Sente, 2), WIN_SCORE - 2);
        assert_eq!(terminal_score(conclusion, Side::Gote, 2), -WIN_SCORE + 2);
    }
    assert_eq!(terminal_score(Conclusion::AgreedDraw, Side::Sente, 0), 0);
    let mut g = tactical();
    g.submit(GameRequest {
        expected: g.context(),
        actor: Side::Gote,
        command: GameCommand::Resign,
    })
    .unwrap();
    let r = search(
        &g,
        Limits {
            depth: 2,
            nodes: 512,
        },
    )
    .unwrap();
    assert_eq!(
        r.decision,
        Some(Decision::Finished(Conclusion::Resignation {
            winner: Side::Sente
        }))
    );
    assert_eq!(r.score, Some(WIN_SCORE));
    assert_eq!(r.nodes, 1);
    assert_eq!(r.completed_depth, 1);
}
fn fixture(id: &str) -> serde_json::Value {
    let v: serde_json::Value =
        serde_json::from_str(include_str!("../../../../docs/m4/chu/fixtures.json")).unwrap();
    v["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()["given"]
        .clone()
}
fn fixture_game(id: &str) -> Game {
    let v = fixture(id);
    let r = &v["root"];
    let entries = r["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            (
                serde_json::from_value(p["at"].clone()).unwrap(),
                Piece::from_code(
                    serde_json::from_value(p["side"].clone()).unwrap(),
                    p["piece"].as_str().unwrap(),
                )
                .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    Game::from_root(
        Position::from_diagram(
            &entries,
            serde_json::from_value(r["turn"].clone()).unwrap(),
            RootMetadata::diagram(r["declared_board_plies"].as_u64().unwrap()),
        )
        .unwrap(),
    )
    .unwrap()
}
fn fixture_action(v: &serde_json::Value) -> Action {
    Action {
        from: serde_json::from_value(v["from"].clone()).unwrap(),
        route: serde_json::from_value(v["route"].clone()).unwrap(),
        promote: v["promote"].as_bool().unwrap(),
    }
}
#[test]
fn selected_fixture_mate_and_prince_response_remain_native() {
    // CHU-MATE independently expects Gote mate, while a complete Elephant flip
    // in CHU-MATE-PRINCE-RESPONSE creates the second royal and prevents mate.
    let g = fixture_game("CHU-MATE");
    assert_eq!(
        g.conclusion(),
        Some(Conclusion::Mate { winner: Side::Gote })
    );
    let r = search(
        &g,
        Limits {
            depth: 2,
            nodes: 512,
        },
    )
    .unwrap();
    assert_eq!(r.score, Some(-WIN_SCORE));
    assert_eq!(r.nodes, 1);
    let mut g = fixture_game("CHU-MATE-PRINCE-RESPONSE");
    assert_eq!(g.conclusion(), None);
    let action = fixture_action(&fixture("CHU-MATE-PRINCE-RESPONSE")["response"]);
    g.submit(GameRequest {
        expected: g.context(),
        actor: Side::Sente,
        command: GameCommand::Play { action },
    })
    .unwrap();
    assert_eq!(g.royal_squares(Side::Sente).len(), 2);
    assert_eq!(g.conclusion(), None);
    let r = search(
        &g,
        Limits {
            depth: 1,
            nodes: 512,
        },
    )
    .unwrap();
    assert!(matches!(r.decision, Some(Decision::Choose(_))));
}
#[test]
fn same_board_different_history_and_pre_fourth_admission() {
    let mut g = fixture_game("CHU-REP-QUIET");
    let input = fixture("CHU-REP-QUIET");
    for a in input["commands"].as_array().unwrap().iter().take(10) {
        g.submit(GameRequest {
            expected: g.context(),
            actor: g.position().side_to_move(),
            command: GameCommand::Play {
                action: fixture_action(a),
            },
        })
        .unwrap();
    }
    let fresh = Game::from_root(
        Position::from_diagram(
            &g.position().pieces().collect::<Vec<_>>(),
            g.position().side_to_move(),
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(g.repetition_key(), fresh.repetition_key());
    assert_ne!(g.events(), fresh.events());
    let forbidden = fixture_action(&input["commands"][10]);
    assert!(fresh.legal_actions().contains(&forbidden));
    assert!(!g.legal_actions().contains(&forbidden));
    let before = GameRecord::from_game(&g).unwrap();
    let r = search(
        &g,
        Limits {
            depth: 1,
            nodes: 512,
        },
    )
    .unwrap();
    let Some(Decision::Choose(a)) = r.decision else {
        panic!("{r:?}")
    };
    assert_ne!(a, forbidden);
    assert!(g.legal_actions().contains(&a));
    assert_eq!(GameRecord::from_game(&g).unwrap(), before);
}
#[test]
fn incomplete_later_iteration_retains_only_last_complete_pv() {
    let g = tactical();
    let first = search(
        &g,
        Limits {
            depth: 1,
            nodes: 512,
        },
    )
    .unwrap();
    let r = search(
        &g,
        Limits {
            depth: 2,
            nodes: first.nodes + 3,
        },
    )
    .unwrap();
    assert_eq!(r.status, SearchStatus::NodeLimit);
    assert_eq!(r.completed_depth, 1);
    assert_eq!(r.decision, first.decision);
    assert_eq!(r.score, first.score);
    assert_eq!(r.principal_variation, first.principal_variation);
}
#[test]
fn native_counter_resource_failure_never_becomes_legal_choice_or_outcome() {
    let g = Game::from_root(
        Position::from_diagram(
            &[
                (
                    "12l".parse().unwrap(),
                    Piece::new(Side::Sente, Base::King, false).unwrap(),
                ),
                (
                    "1a".parse().unwrap(),
                    Piece::new(Side::Gote, Base::King, false).unwrap(),
                ),
                (
                    "6f".parse().unwrap(),
                    Piece::new(Side::Sente, Base::Lion, false).unwrap(),
                ),
                (
                    "1b".parse().unwrap(),
                    Piece::new(Side::Gote, Base::Rook, false).unwrap(),
                ),
            ],
            Side::Sente,
            RootMetadata::diagram(u64::MAX),
        )
        .unwrap(),
    )
    .unwrap();
    let before = GameRecord::from_game(&g).unwrap();
    assert!(matches!(
        search(
            &g,
            Limits {
                depth: 1,
                nodes: 512
            }
        ),
        Err(SearchError::Transition(GameError::Capacity(_)))
    ));
    assert_eq!(GameRecord::from_game(&g).unwrap(), before);
    assert_eq!(g.conclusion(), None);
}
#[test]
fn independently_solved_tree_checks_minimax_iterative_work_and_exact_reversal() {
    // Hand-solved graph: root A->{8,2}, B->{7,3,5,9} encoded below as
    // A's children7/3, B's children5/9. Depth1 prefers8(A), depth2 prefers5(B).
    fn a(n: u8) -> Action {
        Action {
            from: mshogi_core::chu::Square::new(n, 6).unwrap(),
            route: vec![mshogi_core::chu::Square::new(n, 5).unwrap()],
            promote: false,
        }
    }
    struct Small {
        at: usize,
        fail: bool,
    }
    impl Tree for Small {
        type Undo = usize;
        fn turn(&self) -> Side {
            if [1, 2].contains(&self.at) {
                Side::Gote
            } else {
                Side::Sente
            }
        }
        fn conclusion(&self) -> Option<Conclusion> {
            None
        }
        fn evaluate(&self, root: Side) -> i32 {
            let n = [0, 8, 2, 7, 3, 5, 9][self.at];
            if root == Side::Sente { n } else { -n }
        }
        fn actions(&self) -> Vec<Action> {
            match self.at {
                0 => vec![a(1), a(2)],
                1 => vec![a(3), a(4)],
                2 => vec![a(5), a(6)],
                _ => vec![],
            }
        }
        fn push(&mut self, action: Action) -> Result<usize, SearchError> {
            let next = usize::from(action.from.file());
            if self.fail && next == 5 {
                return Err(SearchError::NoBoardActions);
            }
            let before = self.at;
            self.at = next;
            Ok(before)
        }
        fn pop(&mut self, undo: usize) -> Result<(), SearchError> {
            self.at = undo;
            Ok(())
        }
    }
    let mut tree = Small { at: 0, fail: false };
    let mut stop = || false;
    let mut work = Work {
        nodes: 0,
        limit: 100,
        stop: &mut stop,
    };
    let r = minimax(&mut tree, 1, 0, Side::Sente, &mut work).unwrap_or_else(|_| panic!());
    assert_eq!((r.score, r.pv), (8, vec![a(1)]));
    assert_eq!(work.nodes, 3);
    assert_eq!(tree.at, 0);
    work.nodes = 0;
    let r = minimax(&mut tree, 2, 0, Side::Sente, &mut work).unwrap_or_else(|_| panic!());
    assert_eq!((r.score, r.pv), (5, vec![a(2), a(5)]));
    assert_eq!(work.nodes, 7);
    assert_eq!(tree.at, 0);
    work.nodes = 0;
    tree.fail = true;
    assert!(matches!(
        minimax(&mut tree, 2, 0, Side::Sente, &mut work),
        Err(Halt::Error(SearchError::NoBoardActions))
    ));
    assert_eq!(tree.at, 0);
}
