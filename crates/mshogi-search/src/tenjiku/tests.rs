use super::*;
use crate::alphabeta::SearchPosition;
use mshogi_core::tenjiku::{
    Base, GameRecord, GameStatus, Piece, Position, Profile, RootMetadata, Square,
};
fn sq(f: u8, r: u8) -> Square {
    Square::new(f, r).unwrap()
}
fn diagram(profile: Profile, entries: &[(u8, u8, Side, Base, bool)], turn: Side) -> Game {
    let pieces = entries
        .iter()
        .enumerate()
        .map(|(id, (f, r, s, b, p))| (sq(*f, *r), Piece::new(id as u16, *s, *b, *p).unwrap()))
        .collect::<Vec<_>>();
    Game::from_root(
        Position::from_diagram(profile, &pieces, turn, RootMetadata::diagram(0)).unwrap(),
    )
    .unwrap()
}
fn corner(profile: Profile) -> Game {
    diagram(
        profile,
        &[
            (16, 16, Side::Sente, Base::K, false),
            (16, 14, Side::Sente, Base::P, false),
            (1, 1, Side::Gote, Base::K, false),
            (1, 3, Side::Gote, Base::P, false),
        ],
        Side::Sente,
    )
}
fn play(g: &mut Game, a: Action) {
    g.submit(GameRequest {
        expected: g.context(),
        actor: g.position().side_to_move(),
        command: GameCommand::Play { action: a },
    })
    .unwrap();
}
#[test]
fn royal_values_match_and_material_stays_below_the_terminal_band() {
    assert_eq!(piece_value(Face::K), piece_value(Face::CP));
    // Even all 156 admitted pieces at the greatest material value cannot reach the terminal band.
    assert!(Face::ALL.into_iter().map(piece_value).max().unwrap() * 156 < WIN_SCORE);
}
#[test]
fn royal_capture_is_a_native_win_with_full_root_restoration() {
    for profile in Profile::ALL {
        // The adjacent opposing King can be captured by our King. Exposure is legal.
        let mut g = diagram(
            profile,
            &[
                (8, 8, Side::Sente, Base::K, false),
                (16, 14, Side::Sente, Base::P, false),
                (9, 8, Side::Gote, Base::K, false),
                (1, 3, Side::Gote, Base::P, false),
            ],
            Side::Sente,
        );
        let before = GameRecord::from_game(&g).unwrap();
        let ctx = g.context();
        let snapshot = g.snapshot().clone();
        let r = search(
            &g,
            Limits {
                depth: 1,
                nodes: 512,
            },
        )
        .unwrap();
        assert_eq!(r.score, Some(WIN_SCORE - 1));
        assert_eq!(r.completed_depth, 1);
        let Some(Decision::Choose(action)) = r.decision else {
            panic!("{r:?}")
        };
        assert_eq!(action, Action::single(sq(8, 8), sq(9, 8)));
        assert_eq!(r.principal_variation, vec![action.clone()]);
        assert_eq!(GameRecord::from_game(&g).unwrap(), before);
        assert_eq!(g.context(), ctx);
        assert_eq!(g.snapshot(), &snapshot);
        play(&mut g, action);
        assert_eq!(
            g.conclusion(),
            Some(Conclusion::RoyalCapture {
                winner: Side::Sente
            })
        );
    }
}
#[test]
fn zero_partial_stop_and_later_interruption_publish_only_completed_work() {
    for profile in Profile::ALL {
        let g = corner(profile);
        let before = GameRecord::from_game(&g).unwrap();
        let ctx = g.context();
        for n in [0, 1, 2] {
            let r = search(&g, Limits { depth: 1, nodes: n }).unwrap();
            assert_eq!(r.nodes, n);
            assert_eq!(r.status, SearchStatus::NodeLimit);
            assert_eq!(r.decision, None);
            assert_eq!(r.score, None);
            assert!(r.principal_variation.is_empty());
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
        let first = search(
            &g,
            Limits {
                depth: 1,
                nodes: 512,
            },
        )
        .unwrap();
        let later = search(
            &g,
            Limits {
                depth: 2,
                nodes: first.nodes + 2,
            },
        )
        .unwrap();
        assert_eq!(later.completed_depth, 1);
        assert_eq!(later.status, SearchStatus::NodeLimit);
        assert_eq!(later.decision, first.decision);
        assert_eq!(later.score, first.score);
        assert_eq!(later.principal_variation, first.principal_variation);
        // Stop well inside native replay/generation/admission, rather than only before search.
        for after in [2, 20, 200] {
            let mut polls = 0;
            let r = search_with_stop(
                &g,
                Limits {
                    depth: 1,
                    nodes: 512,
                },
                || {
                    polls += 1;
                    polls >= after
                },
            )
            .unwrap();
            assert_eq!(r.status, SearchStatus::Stopped);
            assert_eq!(r.decision, None);
            assert_eq!(r.score, None);
            assert_eq!(polls, after);
        }
        assert_eq!(GameRecord::from_game(&g).unwrap(), before);
        assert_eq!(g.context(), ctx);
    }
}
#[test]
fn same_board_history_repetition_is_not_a_search_cache_key() {
    for profile in Profile::ALL {
        let mut g = corner(profile);
        for (f, t) in [((16, 16), (15, 16)), ((1, 1), (2, 1)), ((15, 16), (16, 16))] {
            play(&mut g, Action::single(sq(f.0, f.1), sq(t.0, t.1)));
        }
        let fresh = Game::from_root(
            Position::from_diagram(
                profile,
                &g.position().pieces().collect::<Vec<_>>(),
                Side::Gote,
                RootMetadata::diagram(0),
            )
            .unwrap(),
        )
        .unwrap();
        let forbidden = Action::single(sq(2, 1), sq(1, 1));
        assert!(fresh.legal_actions().contains(&forbidden));
        assert!(!g.legal_actions().contains(&forbidden));
        assert_ne!(
            (g.native_root(), g.events()),
            (fresh.native_root(), fresh.events())
        );
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
        assert_eq!(r.perspective, Side::Gote);
    }
}
#[test]
fn sourced_no_action_root_is_unscored() {
    // MSH-83 TEN-NO-ACTIONS-CONCRETE: independently deduced before native implementation.
    for profile in Profile::ALL {
        let g = diagram(
            profile,
            &[
                (1, 1, Side::Sente, Base::K, false),
                (2, 1, Side::Sente, Base::P, false),
                (1, 2, Side::Sente, Base::L, false),
                (2, 2, Side::Sente, Base::N, false),
                (16, 16, Side::Gote, Base::K, false),
                (15, 14, Side::Gote, Base::P, false),
            ],
            Side::Sente,
        );
        assert_eq!(g.status(), GameStatus::NoBoardActions);
        assert_eq!(g.conclusion(), None);
        assert_eq!(
            search(
                &g,
                Limits {
                    depth: 1,
                    nodes: 512
                }
            ),
            Err(SearchError::NoBoardActions)
        );
    }
}
#[test]
fn genuine_terminal_and_runtime_retirement_are_distinct() {
    for profile in Profile::ALL {
        let mut g = corner(profile);
        g.retire_context().unwrap();
        assert_eq!(g.conclusion(), None);
        assert!(matches!(
            search(
                &g,
                Limits {
                    depth: 1,
                    nodes: 512
                }
            )
            .unwrap()
            .decision,
            Some(Decision::Choose(_))
        ));
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
    for draw in [
        Conclusion::SimultaneousRoyalLoss,
        Conclusion::BothBareRoyals,
        Conclusion::Agreement,
    ] {
        for side in [Side::Sente, Side::Gote] {
            assert_eq!(terminal_score(draw, side, 2), 0)
        }
    }
    assert_eq!(
        terminal_score(
            Conclusion::BareRoyalLoss {
                winner: Side::Sente
            },
            Side::Gote,
            2
        ),
        -WIN_SCORE + 2
    );
}
#[test]
fn ordered_lion_route_captures_both_native_royals_in_one_complete_action() {
    for profile in Profile::ALL {
        let mut g = diagram(
            profile,
            &[
                (16, 16, Side::Sente, Base::K, false),
                (16, 14, Side::Sente, Base::P, false),
                (8, 8, Side::Sente, Base::Ln, false),
                (8, 7, Side::Gote, Base::K, false),
                (8, 6, Side::Gote, Base::DE, true),
                (1, 3, Side::Gote, Base::P, false),
            ],
            Side::Sente,
        );
        let before = GameRecord::from_game(&g).unwrap();
        let r = search(
            &g,
            Limits {
                depth: 1,
                nodes: 1000,
            },
        )
        .unwrap();
        assert_eq!(r.score, Some(WIN_SCORE - 1));
        let Some(Decision::Choose(a)) = r.decision else {
            panic!("{r:?}")
        };
        assert_eq!(a.from, sq(8, 8));
        assert_eq!(a.route, vec![sq(8, 7), sq(8, 6)]);
        assert_eq!(r.principal_variation, vec![a.clone()]);
        assert_eq!(GameRecord::from_game(&g).unwrap(), before);
        play(&mut g, a);
        assert!(g.royals(Side::Gote).is_empty());
        assert_eq!(
            g.conclusion(),
            Some(Conclusion::RoyalCapture {
                winner: Side::Sente
            })
        );
    }
}
#[test]
fn dying_demon_leaf_uses_selected_profile_and_native_effect_order() {
    // MSH-83 TEN-BURN-DYING: Adams burns enemy bystanders before destruction;
    // Muller suppresses that collateral. Here the bystander is the last enemy King.
    for profile in Profile::ALL {
        let mut g = diagram(
            profile,
            &[
                (16, 16, Side::Sente, Base::K, false),
                (16, 14, Side::Sente, Base::P, false),
                (8, 11, Side::Sente, Base::FiD, false),
                (8, 7, Side::Gote, Base::K, false),
                (9, 8, Side::Gote, Base::FiD, false),
                (1, 3, Side::Gote, Base::P, false),
            ],
            Side::Sente,
        );
        let before = GameRecord::from_game(&g).unwrap();
        let a = Action {
            route: vec![sq(8, 10), sq(8, 9), sq(8, 8)],
            ..Action::single(sq(8, 11), sq(8, 8))
        };
        let mut fast = position::TenjikuPosition::from_game(&g);
        let hash = fast.hash();
        let m = fast.find(&a).unwrap();
        assert!(fast.make(m));
        let mut undo = g
            .submit(GameRequest {
                expected: g.context(),
                actor: Side::Sente,
                command: GameCommand::Play { action: a },
            })
            .unwrap();
        for at in Square::all() {
            assert_eq!(fast.piece(at), g.position().piece(at));
        }
        match profile {
            Profile::AdamsModern => {
                assert_eq!(
                    g.conclusion(),
                    Some(Conclusion::RoyalCapture {
                        winner: Side::Sente
                    })
                );
                assert_eq!(fast.rule_score(), Some(-WIN_SCORE + 1));
            }
            Profile::MullerCompleted => {
                assert_eq!(g.conclusion(), None);
                assert!(g.position().piece(sq(8, 7)).unwrap().is_royal());
            }
        }
        fast.unmake();
        assert_eq!(fast.hash(), hash);
        g.undo(&mut undo).unwrap();
        assert_eq!(GameRecord::from_game(&g).unwrap(), before);
    }
}
#[test]
fn counter_resource_refusal_is_unscored_and_preserves_the_input_owner() {
    for profile in Profile::ALL {
        let ordinary = corner(profile);
        let g = Game::from_root(
            Position::from_diagram(
                profile,
                &ordinary.position().pieces().collect::<Vec<_>>(),
                Side::Sente,
                RootMetadata::diagram(u64::MAX),
            )
            .unwrap(),
        )
        .unwrap();
        let before = GameRecord::from_game(&g).unwrap();
        let ctx = g.context();
        assert_eq!(
            search(
                &g,
                Limits {
                    depth: 1,
                    nodes: 512
                }
            ),
            Err(SearchError::Transition(GameError::Capacity(
                mshogi_core::tenjiku::Resource::Counter
            )))
        );
        assert_eq!(GameRecord::from_game(&g).unwrap(), before);
        assert_eq!(g.context(), ctx);
        assert_eq!(g.conclusion(), None);
    }
}
