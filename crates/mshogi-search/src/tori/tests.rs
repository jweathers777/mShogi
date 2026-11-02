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
fn captures_compulsory_promotion_and_drop_are_native_complete_choices() {
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
    assert_eq!(
        r.decision,
        Some(Decision::Choose(Action::Move {
            from: "7c".parse().unwrap(),
            to: "7b".parse().unwrap(),
            result_face: Face::Eagle
        }))
    );
    assert_eq!(r.score, Some(1100));
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
            depth: 1,
            nodes: 1000,
        },
    )
    .unwrap();
    assert!(
        r.principal_variation
            .iter()
            .all(|a| drop.legal_actions().contains(a))
    );
    assert!(drop.legal_actions().iter().any(|a| matches!(
        a,
        Action::Drop {
            face: Face::RightQuail,
            ..
        }
    )));
}
#[test]
fn limits_cancellation_completed_work_and_full_root_restoration() {
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
    assert_eq!(one.nodes, g.legal_actions().len() as u64 + 1);
    assert!(one.decision.is_some());
    let partial = search(
        &g,
        Limits {
            depth: 2,
            nodes: one.nodes + 1,
        },
    )
    .unwrap();
    assert_eq!(partial.status, SearchStatus::NodeLimit);
    assert_eq!(partial.decision, one.decision);
    assert_eq!(partial.score, one.score);
    assert_eq!(partial.completed_depth, 1);
    for n in [0, 1] {
        let r = search(&g, Limits { depth: 2, nodes: n }).unwrap();
        assert_eq!(r.decision, None);
        assert_eq!(r.score, None);
        assert_eq!(r.status, SearchStatus::NodeLimit);
        assert!(r.nodes <= n);
    }
    let mut calls = 0;
    let r = search_with_stop(&g, DEFAULT_LIMITS, || {
        calls += 1;
        calls > 8
    })
    .unwrap();
    assert_eq!(r.status, SearchStatus::Stopped);
    assert_eq!(r.completed_depth, 0);
    assert_eq!(g.snapshot().clone(), before);
    assert_eq!(g.context(), ctx);
    assert_eq!(
        search(&g, Limits { depth: 0, nodes: 1 }),
        Err(SearchError::InvalidLimits)
    );
    assert_eq!(
        search(
            &g,
            Limits {
                depth: MAX_DEPTH + 1,
                nodes: 1
            }
        ),
        Err(SearchError::InvalidLimits)
    );
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
fn deterministic_both_sides_apply_undo_and_bounded_self_play() {
    let mut g = Game::standard();
    let root = g.snapshot().clone();
    let mut undos = vec![];
    for _ in 0..16 {
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
            r,
            search(
                &g,
                Limits {
                    depth: 1,
                    nodes: 1000
                }
            )
            .unwrap()
        );
        assert_eq!(r.perspective, g.position().side_to_move());
        assert_eq!(before, g.snapshot().clone());
        let Some(Decision::Choose(action)) = r.decision else {
            break;
        };
        assert!(g.legal_actions().contains(&action));
        undos.push(
            g.submit(GameRequest {
                expected: g.context(),
                actor: g.position().side_to_move(),
                command: GameCommand::Play { action },
            })
            .unwrap(),
        );
    }
    assert!(g.snapshot().board_ply >= 2);
    for u in undos.iter_mut().rev() {
        g.undo(u).unwrap();
    }
    assert_eq!(g.snapshot().clone(), root);
}
#[test]
#[ignore = "bounded baseline performance probe, run explicitly in release"]
fn measure_default_standard_and_drop_heavy() {
    let drop = diagram(
        &[
            ("4g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
        ],
        [
            Hand::from_counts(&[
                (Face::Falcon, 1),
                (Face::Crane, 2),
                (Face::Pheasant, 2),
                (Face::LeftQuail, 1),
                (Face::RightQuail, 1),
                (Face::Swallow, 3),
            ])
            .unwrap(),
            Hand::empty(),
        ],
        Side::Sente,
    );
    for (name, g) in [("standard", Game::standard()), ("drop-heavy", drop)] {
        let start = std::time::Instant::now();
        let r = search(&g, DEFAULT_LIMITS).unwrap();
        eprintln!(
            "{name}: legal={} elapsed_ms={} nodes={} completed_depth={} status={:?}",
            g.legal_actions().len(),
            start.elapsed().as_millis(),
            r.nodes,
            r.completed_depth,
            r.status
        );
        assert!(r.decision.is_some());
    }
}

#[test]
fn independently_solved_minimax_tree_proves_reply_sign_full_width_and_error_unwind() {
    // Explicit arithmetic tree: A has min(7,3)=3; B has min(5,9)=5.
    // Depth1: A=8 beats B=2. Depth2 must choose B/5, not A, max9,
    // or negated score. This test has no game-rules oracle assumptions.
    struct Small {
        at: usize,
        fail: bool,
    }
    fn a(n: usize) -> Action {
        Action::Move {
            from: mshogi_core::tori::Square::new(n as u8, 1).unwrap(),
            to: mshogi_core::tori::Square::new(n as u8, 2).unwrap(),
            result_face: Face::Phoenix,
        }
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
            let Action::Move { from, .. } = action else {
                panic!()
            };
            let next = usize::from(from.file());
            if self.fail && next == 5 {
                return Err(SearchError::Invariant);
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
        Err(Halt::Error(SearchError::Invariant))
    ));
    assert_eq!(tree.at, 0);
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
fn terminal_leaf_can_shorten_every_completed_iteration() {
    // Independently specified singleton root->terminal graph. At each requested
    // depth it visits root+child only; total work is2*completed_depth, not d(d+3)/2.
    struct Singleton {
        child: bool,
    }
    let action = Action::Move {
        from: "4c".parse().unwrap(),
        to: "4b".parse().unwrap(),
        result_face: Face::Eagle,
    };
    impl Tree for Singleton {
        type Undo = ();
        fn turn(&self) -> Side {
            if self.child { Side::Gote } else { Side::Sente }
        }
        fn conclusion(&self) -> Option<Conclusion> {
            self.child.then_some(Conclusion::Checkmate {
                winner: Side::Sente,
            })
        }
        fn evaluate(&self, _: Side) -> i32 {
            panic!("terminal must precede static evaluation")
        }
        fn actions(&self) -> Vec<Action> {
            vec![Action::Move {
                from: "4c".parse().unwrap(),
                to: "4b".parse().unwrap(),
                result_face: Face::Eagle,
            }]
        }
        fn push(&mut self, _: Action) -> Result<(), SearchError> {
            self.child = true;
            Ok(())
        }
        fn pop(&mut self, _: ()) -> Result<(), SearchError> {
            self.child = false;
            Ok(())
        }
    }
    let mut tree = Singleton { child: false };
    let mut stop = || false;
    let mut work = Work {
        nodes: 0,
        limit: 100,
        stop: &mut stop,
    };
    for depth in 1..=3 {
        let result =
            minimax(&mut tree, depth, 0, Side::Sente, &mut work).unwrap_or_else(|_| panic!());
        assert_eq!(result.score, WIN_SCORE - 1);
        assert_eq!(result.pv, vec![action]);
        assert_eq!(work.nodes, 2 * u64::from(depth));
        assert!(!tree.child);
    }
}
