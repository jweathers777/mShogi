use super::*;
use mshogi_core::chu::{Base, GameRecord, Piece, Position, RootMetadata};
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
fn the_prince_has_the_king_value() {
    assert_eq!(piece_value(Face::Prince), piece_value(Face::King));
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
fn node_limit_uses_root_fallback_and_cancellation_has_no_move() {
    let g = tactical();
    let first = g.legal_actions()[0].clone();
    for n in [0, 1] {
        let r = search(&g, Limits { depth: 1, nodes: n }).unwrap();
        assert_eq!(r.status, SearchStatus::NodeLimit);
        assert_eq!(r.completed_depth, 0);
        assert_eq!(r.decision, Some(Decision::Choose(first.clone())));
        assert_eq!(r.score, None);
        assert_eq!(r.principal_variation, [first.clone()]);
        assert_eq!(r.nodes, n);
    }
    let partial = search(&g, Limits { depth: 1, nodes: 2 }).unwrap();
    let Some(Decision::Choose(best)) = partial.decision else {
        panic!("{partial:?}");
    };
    assert_eq!(partial.status, SearchStatus::NodeLimit);
    assert_eq!(partial.completed_depth, 0);
    assert_eq!(partial.score, None);
    assert_ne!(best, first);
    assert_eq!(best.from, "6f".parse().unwrap());
    assert_eq!(best.route.len(), 2);
    assert!(best.route.contains(&"6e".parse().unwrap()));
    assert!(best.route.contains(&"5e".parse().unwrap()));
    assert_eq!(partial.principal_variation, [best]);

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
            checks > 1
        },
    )
    .unwrap();
    assert_eq!(r.status, SearchStatus::Stopped);
    assert_eq!(r.decision, None);

    let mut polls = 0;
    let r = search_with_stop(&g, Limits { depth: 1, nodes: 2 }, || {
        polls += 1;
        polls >= 3
    })
    .unwrap();
    assert_eq!(r.nodes, 2);
    assert_eq!(r.status, SearchStatus::Stopped);
    assert_eq!(r.decision, None);
    assert!(r.principal_variation.is_empty());
    assert_eq!(g.events().len(), 0);
}
#[test]
fn conclusions_and_draws_have_terminal_scores() {
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
fn chu_search_detects_mate_and_preserves_prince_response() {
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
fn counter_resource_failure_never_becomes_legal_choice_or_outcome() {
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
                nodes: 20_000
            }
        ),
        Err(SearchError::Transition(GameError::Capacity(_)))
    ));
    assert_eq!(GameRecord::from_game(&g).unwrap(), before);
    assert_eq!(g.conclusion(), None);
}
#[test]
fn independently_specified_no_action_root_stays_unscored() {
    let game = fixture_game("CHU-NOACTION");
    let before = GameRecord::from_game(&game).unwrap();
    assert_eq!(game.conclusion(), None);
    assert!(game.legal_actions().is_empty());
    assert_eq!(
        search(&game, DEFAULT_LIMITS),
        Err(SearchError::NoBoardActions)
    );
    assert_eq!(GameRecord::from_game(&game).unwrap(), before);
}
#[test]
fn search_uses_prince_promotion_to_escape_the_mate_fixture() {
    let mut game = fixture_game("CHU-MATE-PRINCE-RESPONSE");
    let result = search(
        &game,
        Limits {
            depth: 2,
            nodes: 20_000,
        },
    )
    .unwrap();
    let Some(Decision::Choose(action)) = result.decision else {
        panic!("{result:?}")
    };
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: GameCommand::Play { action },
    })
    .unwrap();
    // The hand-established fixture has no ordinary one-royal escape. Creating
    // a Prince survives even though both royals are individually threatened.
    assert_eq!(game.royal_squares(Side::Sente).len(), 2);
    assert_eq!(game.conclusion(), None);
}
