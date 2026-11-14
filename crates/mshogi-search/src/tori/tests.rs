//! The Tori adapter: native authority at the root, limits, forced wins with
//! exact distance, and the reported line.
use super::*;
use mshogi_core::tori::{Hand, Piece, RootMetadata};
use serde_json::Value;
fn face(v: &str) -> Face {
    Face::ALL.into_iter().find(|f| f.code() == v).unwrap()
}
fn side(v: &str) -> Side {
    match v {
        "s" | "sente" => Side::Sente,
        "g" | "gote" => Side::Gote,
        _ => panic!(),
    }
}
fn root(v: &Value) -> Position {
    let entries = v["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let p = v.as_str().unwrap().split(':').collect::<Vec<_>>();
            (
                p[2].parse().unwrap(),
                Piece {
                    side: side(p[0]),
                    face: face(p[1]),
                },
            )
        })
        .collect::<Vec<_>>();
    let hands = std::array::from_fn(|i| {
        Hand::from_counts(
            &v["hands"][if i == 0 { "s" } else { "g" }]
                .as_object()
                .unwrap()
                .iter()
                .map(|(f, n)| (face(f), n.as_u64().unwrap() as u8))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    });
    Position::from_diagram(
        &entries,
        hands,
        side(v["side_to_move"].as_str().unwrap()),
        RootMetadata::diagram(v["declared_completed_plies"].as_u64().unwrap_or(0)),
    )
    .unwrap()
}
fn fixture(id: &str) -> Value {
    let v: Value =
        serde_json::from_str(include_str!("../../../../docs/m4/tori/fixtures.json")).unwrap();
    v["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()["given"]
        .clone()
}
fn seq(id: &str) -> Game {
    let v = fixture(id);
    let mut g = Game::from_root(root(&v["root"])).unwrap();
    if let Some(actions) = v["actions"].as_array() {
        for a in actions {
            let action = Action::Move {
                from: a["from"].as_str().unwrap().parse().unwrap(),
                to: a["to"].as_str().unwrap().parse().unwrap(),
                result_face: face(a["result_face"].as_str().unwrap()),
            };
            play(&mut g, action);
        }
    }
    g
}
fn play(g: &mut Game, action: Action) {
    g.submit(GameRequest {
        expected: g.context(),
        actor: g.position().side_to_move(),
        command: GameCommand::Play { action },
    })
    .unwrap();
}
fn diagram(entries: &[(&str, Side, Face)], hands: [Hand; 2], turn: Side) -> Game {
    Game::from_root(
        Position::from_diagram(
            &entries
                .iter()
                .map(|(s, side, face)| {
                    (
                        s.parse().unwrap(),
                        Piece {
                            side: *side,
                            face: *face,
                        },
                    )
                })
                .collect::<Vec<_>>(),
            hands,
            turn,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn material_both_owners_handed_identities_and_promoted_hands() {
    assert_eq!(piece_value(Face::Phoenix), 0);
    assert_eq!(piece_value(Face::Swallow), 100);
    assert_eq!(piece_value(Face::Goose), 300);
    assert_eq!(piece_value(Face::LeftQuail), piece_value(Face::RightQuail));
    let g = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("1a", Side::Gote, Face::Phoenix),
            ("4d", Side::Sente, Face::Eagle),
        ],
        [
            Hand::from_counts(&[(Face::LeftQuail, 1)]).unwrap(),
            Hand::from_counts(&[(Face::RightQuail, 1), (Face::Swallow, 2)]).unwrap(),
        ],
        Side::Gote,
    );
    assert_eq!(
        evaluate(g.position()),
        Evaluation {
            sente: 1500,
            gote: 700
        }
    );
    assert_eq!(evaluate(g.position()).score_for(Side::Gote), -800);
}
#[test]
fn independently_expected_terminal_and_t06_history_not_board_only() {
    for (id, c) in [
        (
            "TORI-O-MATE",
            Conclusion::Checkmate {
                winner: Side::Sente,
            },
        ),
        (
            "TORI-O-QUIET",
            Conclusion::StalemateLoss { loser: Side::Gote },
        ),
        (
            "TORI-H-FORCED-FULL",
            Conclusion::RepetitionForfeit { loser: Side::Sente },
        ),
    ] {
        let g = seq(id);
        let before = g.snapshot().clone();
        let r = search(&g, DEFAULT_LIMITS).unwrap();
        assert_eq!(r.decision, Some(Decision::Finished(c)));
        assert_eq!(r.nodes, 1);
        assert_eq!(
            r.score,
            Some(terminal_score(c, g.position().side_to_move(), 0))
        );
        assert_eq!(g.snapshot().clone(), before);
    }
    let history = seq("TORI-H-COUNT");
    let fresh = Game::from_root(root(&fixture("TORI-H-COUNT")["root"])).unwrap();
    assert_eq!(history.repetition_key(), fresh.repetition_key());
    assert!(fresh.legal_actions().contains(&Action::Move {
        from: "4g".parse().unwrap(),
        to: "5g".parse().unwrap(),
        result_face: Face::Phoenix
    }));
    let before = history.snapshot().clone();
    let r = search(
        &history,
        Limits {
            depth: 2,
            nodes: MAX_NODES,
        },
    )
    .unwrap();
    let Some(Decision::Choose(a)) = r.decision else {
        panic!()
    };
    assert!(history.obligation().unwrap().permits(a));
    assert_eq!(history.snapshot().clone(), before);
    let unequal = seq("TORI-H-UNEQUAL-FULL");
    let r = search(
        &unequal,
        Limits {
            depth: 1,
            nodes: 100,
        },
    )
    .unwrap();
    let Some(Decision::Choose(a)) = r.decision else {
        panic!()
    };
    assert!(unequal.obligation().unwrap().permits(a));
}
#[test]
fn independently_proved_native_mate_in_one_uses_descendant_terminal_distance() {
    // TORI-O-MATE's independently reviewed final diagram is reached by Ea4c-4b.
    // Gote3a/5a/3b/5b blockers and Pt4d protect Ea4b, leaving no defense.
    let g = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
            ("4d", Side::Sente, Face::Pheasant),
            ("3a", Side::Gote, Face::Swallow),
            ("5a", Side::Gote, Face::Swallow),
            ("3b", Side::Gote, Face::Swallow),
            ("5b", Side::Gote, Face::Swallow),
            ("4c", Side::Sente, Face::Eagle),
        ],
        [Hand::empty(); 2],
        Side::Sente,
    );
    let before = g.snapshot().clone();
    let r = search(
        &g,
        Limits {
            depth: 1,
            nodes: 1000,
        },
    )
    .unwrap();
    assert_eq!(
        r.decision,
        Some(Decision::Choose(Action::Move {
            from: "4c".parse().unwrap(),
            to: "4b".parse().unwrap(),
            result_face: Face::Eagle
        }))
    );
    assert_eq!(r.score, Some(WIN_SCORE - 1));
    assert_eq!(r.principal_variation.len(), 1);
    assert_eq!(g.snapshot(), &before);
}

#[test]
fn a_capture_with_compulsory_promotion_and_a_drop_are_complete_native_actions() {
    let g = diagram(
        &[
            ("4g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
            ("7c", Side::Sente, Face::Falcon),
            ("7b", Side::Gote, Face::Swallow),
        ],
        [Hand::empty(); 2],
        Side::Sente,
    );
    let r = search(
        &g,
        Limits {
            depth: 1,
            nodes: 1000,
        },
    )
    .unwrap();
    // The Falcon takes the free Swallow and must promote in the zone.
    assert_eq!(
        r.decision,
        Some(Decision::Choose(Action::Move {
            from: "7c".parse().unwrap(),
            to: "7b".parse().unwrap(),
            result_face: Face::Eagle
        }))
    );
    // Material after the capture: Eagle 1000 and a Swallow in hand 100.
    assert!(
        r.score.unwrap() >= 1100 && r.score.unwrap() < 1300,
        "{:?}",
        r.score
    );
    // Sente has only a Phoenix and a Quail in hand: each good action is a drop.
    let drop = diagram(
        &[
            ("4g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
        ],
        [
            Hand::from_counts(&[(Face::RightQuail, 1)]).unwrap(),
            Hand::empty(),
        ],
        Side::Sente,
    );
    let r = search(
        &drop,
        Limits {
            depth: 4,
            nodes: 100_000,
        },
    )
    .unwrap();
    let Some(Decision::Choose(action)) = r.decision else {
        panic!()
    };
    assert!(drop.legal_actions().contains(&action));
    assert_eq!(r.principal_variation.first(), Some(&action));
}

#[test]
fn limits_stop_and_an_unchanged_game() {
    let g = Game::standard();
    let before = g.snapshot().clone();
    let ctx = g.context();
    let one = search(
        &g,
        Limits {
            depth: 1,
            nodes: MAX_NODES,
        },
    )
    .unwrap();
    assert_eq!(one.completed_depth, 1);
    assert_eq!(one.status, SearchStatus::Complete);
    assert!(one.nodes > g.legal_actions().len() as u64);
    assert!(one.decision.is_some());
    // Equal requests give equal results: each search starts empty.
    let limits = Limits {
        depth: 6,
        nodes: 20_000,
    };
    let first = search(&g, limits).unwrap();
    assert_eq!(search(&g, limits).unwrap(), first);
    assert!(first.nodes <= limits.nodes);
    if first.status == SearchStatus::NodeLimit {
        assert_eq!(first.nodes, limits.nodes);
    }
    // A node limit below one iteration gives no decision and no score.
    for n in [0, 1] {
        let r = search(&g, Limits { depth: 2, nodes: n }).unwrap();
        assert_eq!(r.decision, None);
        assert_eq!(r.score, None);
        assert_eq!(r.completed_depth, 0);
        assert!(r.principal_variation.is_empty());
        assert_eq!(r.status, SearchStatus::NodeLimit);
        assert!(r.nodes <= n);
    }
    let r = search_with_stop(&g, DEFAULT_LIMITS, || true).unwrap();
    assert_eq!(r.status, SearchStatus::Stopped);
    assert_eq!(r.completed_depth, 0);
    assert_eq!(r.decision, None);
    assert_eq!(g.snapshot().clone(), before);
    assert_eq!(g.context(), ctx);
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
        assert_eq!(search(&g, limits), Err(SearchError::InvalidLimits));
    }
}

#[test]
fn a_forced_win_and_a_forced_loss_have_their_exact_distance() {
    // The mate-in-one position of the test above, at a larger depth.
    let g = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
            ("4d", Side::Sente, Face::Pheasant),
            ("3a", Side::Gote, Face::Swallow),
            ("5a", Side::Gote, Face::Swallow),
            ("3b", Side::Gote, Face::Swallow),
            ("5b", Side::Gote, Face::Swallow),
            ("4c", Side::Sente, Face::Eagle),
        ],
        [Hand::empty(); 2],
        Side::Sente,
    );
    let r = search(
        &g,
        Limits {
            depth: 5,
            nodes: 200_000,
        },
    )
    .unwrap();
    // The mate in one stays the result at a larger depth.
    assert_eq!(r.score, Some(WIN_SCORE - 1));
    assert_eq!(r.principal_variation.len(), 1);
    // The side that is mated in one sees the loss at distance two.
    let mut lost = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
            ("4d", Side::Sente, Face::Pheasant),
            ("3a", Side::Gote, Face::Swallow),
            ("5a", Side::Gote, Face::Swallow),
            ("3b", Side::Gote, Face::Swallow),
            ("5b", Side::Gote, Face::Swallow),
            ("4c", Side::Sente, Face::Eagle),
            ("1c", Side::Gote, Face::Swallow),
        ],
        [Hand::empty(); 2],
        Side::Gote,
    );
    let r = search(
        &lost,
        Limits {
            depth: 4,
            nodes: 200_000,
        },
    )
    .unwrap();
    assert_eq!(r.score, Some(-WIN_SCORE + 2));
    // The reported line is playable in the native game and ends in its mate.
    for action in r.principal_variation.clone() {
        play(&mut lost, action);
    }
    assert_eq!(
        lost.conclusion(),
        Some(Conclusion::Checkmate {
            winner: Side::Sente
        })
    );
}

#[test]
fn self_play_gives_only_native_legal_actions_and_native_playable_lines() {
    let mut g = Game::standard();
    for _ in 0..40 {
        if g.conclusion().is_some() {
            break;
        }
        let r = search(
            &g,
            Limits {
                depth: 3,
                nodes: 5_000,
            },
        )
        .unwrap();
        let Some(Decision::Choose(action)) = r.decision else {
            panic!("no decision: {r:?}")
        };
        assert!(g.legal_actions().contains(&action));
        assert_eq!(r.principal_variation.first(), Some(&action));
        assert!(r.principal_variation.len() <= MAX_LINE);
        assert_eq!(r.perspective, g.position().side_to_move());
        let mut line = GameRecord::from_game(&g).unwrap().to_game().unwrap();
        for a in &r.principal_variation {
            play(&mut line, *a);
        }
        play(&mut g, action);
    }
}
