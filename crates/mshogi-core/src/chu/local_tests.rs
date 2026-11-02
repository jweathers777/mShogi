use super::*;
fn sq(f: u8, r: u8) -> Square {
    Square::new(f, r).unwrap()
}
fn piece(side: Side, code: &str) -> Piece {
    Piece::from_code(side, code).unwrap()
}
fn local(entries: &[(u8, u8, Side, &str)], turn: Side) -> LocalPosition {
    LocalPosition::admit(
        Position::from_diagram(
            &entries
                .iter()
                .map(|&(f, r, s, p)| (sq(f, r), piece(s, p)))
                .collect::<Vec<_>>(),
            turn,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
}

#[test]
fn original_identity_survives_every_once_only_promotion() {
    assert_eq!(Base::ALL.len(), 21);
    assert_eq!(Face::ALL.len(), 29);
    assert_eq!(Base::ALL.iter().filter(|b| b.promotable()).count(), 18);
    for base in Base::ALL {
        for side in [Side::Sente, Side::Gote] {
            let p = Piece::new(side, base, false).unwrap();
            assert_eq!(p.base, base);
            assert_eq!(p.code(), base.code());
            if base.promotable() {
                let promoted = Piece::new(side, base, true).unwrap();
                assert_eq!(promoted.base, base);
                assert_eq!(Piece::from_code(side, &promoted.code()), Some(promoted));
            } else {
                assert!(Piece::new(side, base, true).is_none())
            }
        }
    }
    assert!(!piece(Side::Sente, "GB+").is_royal());
    assert!(piece(Side::Sente, "DE+").is_royal());
    assert!(piece(Side::Sente, "Ki+").is_lion());
    assert!(!piece(Side::Sente, "DK+").is_lion());
}
#[test]
fn standard_is_exactly46_owned_originals_per_side_and_rotation() {
    let p = Position::standard();
    assert_eq!(p.pieces().count(), 92);
    for side in [Side::Sente, Side::Gote] {
        assert_eq!(p.pieces().filter(|(_, p)| p.side == side).count(), 46);
    }
    for (s, piece) in p.pieces() {
        assert_eq!(
            p.piece(s.rotated()),
            Some(Piece {
                side: piece.side.opponent(),
                ..piece
            })
        );
    }
    assert_eq!(p.piece(sq(7, 12)), Some(piece(Side::Sente, "K")));
    assert_eq!(p.piece(sq(6, 12)), Some(piece(Side::Sente, "DE")));
    assert_eq!(p.piece(sq(7, 10)), Some(piece(Side::Sente, "Ln")));
}
#[test]
fn bounded_square_codec_and_rotation() {
    for s in Square::all() {
        assert_eq!(s.rotated().rotated(), s);
        assert_eq!(s.to_string().parse(), Ok(s));
        assert_eq!(
            serde_json::from_str::<Square>(&serde_json::to_string(&s).unwrap()).unwrap(),
            s
        );
    }
    for s in ["0a", "13a", "1m", "01a", "1", "é"] {
        assert!(s.parse::<Square>().is_err());
    }
    assert!(serde_json::from_str::<Square>("[13,1]").is_err());
}
#[test]
fn empty_central_complete_lion_sets_are88_41_42() {
    for (code, count) in [("Ln", 88), ("DK+", 41), ("DH+", 42)] {
        let p = local(&[(6, 6, Side::Sente, code)], Side::Sente);
        assert_eq!(p.choices_from(sq(6, 6)).len(), count, "{code}");
    }
}
#[test]
fn full_routes_keep_jump_walk_capture_return_distinct() {
    let mut p = local(
        &[
            (6, 6, Side::Sente, "Ln"),
            (6, 5, Side::Gote, "Si"),
            (6, 4, Side::Gote, "R"),
        ],
        Side::Sente,
    );
    let original = p.clone();
    let jump = Action::single(sq(6, 6), sq(6, 4));
    let (e, mut undo) = p.apply(&jump).unwrap();
    assert_eq!(
        e.captures.iter().map(|c| c.square).collect::<Vec<_>>(),
        vec![sq(6, 4)]
    );
    assert_eq!(p.position().piece(sq(6, 5)), Some(piece(Side::Gote, "Si")));
    p.undo(&mut undo).unwrap();
    assert_eq!(p, original);
    let walk = Action {
        from: sq(6, 6),
        route: vec![sq(6, 5), sq(6, 4)],
        promote: false,
    };
    let (e, mut undo) = p.apply(&walk).unwrap();
    assert_eq!(
        e.captures.iter().map(|c| c.square).collect::<Vec<_>>(),
        vec![sq(6, 5), sq(6, 4)]
    );
    p.undo(&mut undo).unwrap();
    let ret = Action {
        from: sq(6, 6),
        route: vec![sq(6, 5), sq(6, 6)],
        promote: false,
    };
    let (e, _) = p.apply(&ret).unwrap();
    assert!(!e.quiet_return);
    assert_eq!(p.position().piece(sq(6, 6)), Some(piece(Side::Sente, "Ln")));
    assert!(p.position().piece(sq(6, 5)).is_none());
}
#[test]
fn promotion_is_optional_entry_capture_reentry_pawn_rescue_not_lance_rescue() {
    let p = local(&[(6, 5, Side::Sente, "R")], Side::Sente);
    let a = Action::single(sq(6, 5), sq(6, 4));
    assert!(p.choices_from(sq(6, 5)).contains(&a));
    assert!(
        p.choices_from(sq(6, 5))
            .contains(&Action { promote: true, ..a })
    );
    for (code, allowed) in [("P", true), ("L", false), ("GB", false)] {
        let p = local(&[(6, 2, Side::Sente, code)], Side::Sente);
        let a = Action {
            from: sq(6, 2),
            route: vec![sq(6, 1)],
            promote: true,
        };
        assert_eq!(p.choices_from(sq(6, 2)).contains(&a), allowed);
    }
    let mut p = local(
        &[(6, 3, Side::Sente, "R"), (6, 2, Side::Gote, "P")],
        Side::Sente,
    );
    assert!(
        p.apply(&Action {
            from: sq(6, 3),
            route: vec![sq(6, 2)],
            promote: true
        })
        .is_ok()
    );
    let mut p = local(&[(6, 3, Side::Sente, "R")], Side::Sente);
    let before = p.clone();
    assert_eq!(
        p.apply(&Action {
            from: sq(6, 3),
            route: vec![sq(6, 2)],
            promote: true
        })
        .unwrap_err(),
        ActionError::Promotion
    );
    assert_eq!(p, before);
}
#[test]
fn royal_capture_exposure_and_dead_originals_are_capture_based() {
    let mut p = local(
        &[
            (6, 6, Side::Sente, "K"),
            (6, 1, Side::Gote, "R"),
            (12, 1, Side::Gote, "K"),
            (5, 1, Side::Sente, "P"),
            (4, 1, Side::Sente, "L"),
        ],
        Side::Sente,
    );
    assert!(p.apply(&Action::single(sq(6, 6), sq(6, 5))).is_ok());
    assert!(p.position().piece(sq(5, 1)).unwrap().dead(sq(5, 1)));
    assert!(p.position().piece(sq(4, 1)).unwrap().dead(sq(4, 1)));
}
#[test]
fn defended_distant_lion_and_hidden_ray_are_not_chess_pins() {
    let mut p = local(
        &[
            (3, 3, Side::Gote, "Ln"),
            (1, 1, Side::Sente, "Ln"),
            (7, 7, Side::Sente, "B"),
        ],
        Side::Gote,
    );
    let before = p.clone();
    assert_eq!(
        p.apply(&Action::single(sq(3, 3), sq(1, 1))).unwrap_err(),
        ActionError::LionLaw
    );
    assert_eq!(p, before);
    let mut adjacent = local(
        &[
            (6, 6, Side::Sente, "Ln"),
            (6, 5, Side::Gote, "Ln"),
            (6, 1, Side::Gote, "R"),
        ],
        Side::Sente,
    );
    assert!(adjacent.apply(&Action::single(sq(6, 6), sq(6, 5))).is_ok());
}
#[test]
fn intermediate_pawn_gobetween_not_qualifying_but_silver_is() {
    for (code, expected) in [("P", false), ("GB", false), ("Si", true)] {
        let mut p = local(
            &[
                (6, 6, Side::Sente, "Ln"),
                (6, 5, Side::Gote, code),
                (6, 4, Side::Gote, "Ln"),
                (6, 1, Side::Gote, "R"),
            ],
            Side::Sente,
        );
        assert_eq!(
            p.apply(&Action {
                from: sq(6, 6),
                route: vec![sq(6, 5), sq(6, 4)],
                promote: false
            })
            .is_ok(),
            expected,
            "{code}"
        );
    }
}
#[test]
fn non_lion_capture_creates_defended_next_turn_window_and_rejection_preserves_it() {
    let mut p = local(
        &[
            (4, 4, Side::Sente, "Ki"),
            (8, 10, Side::Sente, "Ln"),
            (7, 11, Side::Sente, "G"),
            (5, 5, Side::Sente, "G"),
            (6, 4, Side::Gote, "Ln"),
            (12, 10, Side::Gote, "R"),
            (12, 1, Side::Gote, "K"),
        ],
        Side::Sente,
    );
    p.apply(&Action {
        from: sq(4, 4),
        route: vec![sq(6, 4)],
        promote: true,
    })
    .unwrap();
    assert!(p.protection().squares.contains(&sq(6, 4)));
    assert!(p.protection().squares.contains(&sq(8, 10)));
    let before = p.clone();
    assert_eq!(
        p.apply(&Action::single(sq(12, 10), sq(8, 10))).unwrap_err(),
        ActionError::LionLaw
    );
    assert_eq!(p, before);
    p.apply(&Action::single(sq(12, 1), sq(11, 1))).unwrap();
    assert!(p.protection().squares.is_empty());
}
#[test]
fn malformed_and_friendly_compound_actions_are_atomic() {
    let mut p = local(
        &[(6, 6, Side::Sente, "Ln"), (6, 5, Side::Sente, "P")],
        Side::Sente,
    );
    for a in [
        Action {
            from: sq(6, 6),
            route: vec![],
            promote: false,
        },
        Action {
            from: sq(6, 6),
            route: vec![sq(6, 5), sq(6, 6)],
            promote: false,
        },
        Action {
            from: sq(6, 6),
            route: vec![sq(6, 4), sq(6, 3)],
            promote: false,
        },
        Action {
            from: sq(6, 6),
            route: vec![sq(5, 5), sq(6, 6)],
            promote: true,
        },
    ] {
        let before = p.clone();
        assert!(p.apply(&a).is_err());
        assert_eq!(p, before);
    }
}
#[test]
fn all_central_actions_exactly_reverse_and_tokens_cannot_cross_clones_or_branches() {
    for code in ["Ln", "DK+", "DH+", "R", "P", "DE"] {
        let original = local(
            &[(6, 6, Side::Sente, code), (8, 5, Side::Gote, "P")],
            Side::Sente,
        );
        for a in original.legal_actions() {
            let mut p = original.clone();
            let (effects, mut undo) = p.apply(&a).unwrap();
            let after = p.clone();
            let mut foreign = p.clone();
            assert_eq!(
                foreign.undo(&mut undo).unwrap_err(),
                ReversalError::InvalidToken
            );
            let mut redo = p.undo(&mut undo).unwrap();
            assert_eq!(p, original);
            assert_eq!(p.undo(&mut undo).unwrap_err(), ReversalError::InvalidToken);
            let (again, _) = p.redo(&mut redo).unwrap();
            assert_eq!(again, effects);
            assert_eq!(p, after);
            let mut p = original.clone();
            let (_, mut undo) = p.apply(&a).unwrap();
            let mut redo = p.undo(&mut undo).unwrap();
            p.apply(&a).unwrap();
            assert_eq!(p.redo(&mut redo).unwrap_err(), ReversalError::InvalidToken);
        }
    }
}
#[test]
fn actor_rotation_preserves_every_local_choice_and_effect() {
    for code in [
        "P", "Cu", "Si", "G", "BT", "DE", "Ki", "Ph", "RC+", "L+", "DK+", "DH+", "Ln",
    ] {
        let p = local(
            &[(6, 6, Side::Sente, code), (7, 5, Side::Gote, "Si")],
            Side::Sente,
        );
        let entries: Vec<_> = p
            .position()
            .pieces()
            .map(|(s, p)| {
                (
                    s.rotated(),
                    Piece {
                        side: p.side.opponent(),
                        ..p
                    },
                )
            })
            .collect();
        let q = LocalPosition::admit(
            Position::from_diagram(&entries, Side::Gote, RootMetadata::diagram(0)).unwrap(),
        );
        let a: std::collections::BTreeSet<_> = p
            .legal_actions()
            .into_iter()
            .map(|a| Action {
                from: a.from.rotated(),
                route: a.route.into_iter().map(Square::rotated).collect(),
                promote: a.promote,
            })
            .collect();
        assert_eq!(a, q.legal_actions().into_iter().collect(), "{code}");
    }
}

#[test]
fn structural_diagram_is_not_reachability_and_bad_roots_reject() {
    let root = RootMetadata::imported("independent diagram".into(), 83).unwrap();
    let p =
        Position::from_diagram(&[(sq(1, 1), piece(Side::Sente, "P"))], Side::Gote, root).unwrap();
    assert!(p.root().earlier_history_unknown());
    assert!(!p.root().historical_reachability_proven());
    assert_eq!(p.declared_total_plies(), 83);
    assert_eq!(p.accepted_plies(), 0);
    assert_eq!(p.pieces().count(), 1);
    assert!(Position::from_diagram(&[], Side::Sente, Position::standard().root().clone()).is_err());
    assert!(
        Position::from_diagram(
            &[
                (sq(1, 1), piece(Side::Sente, "K")),
                (sq(2, 1), piece(Side::Sente, "K"))
            ],
            Side::Sente,
            RootMetadata::diagram(0)
        )
        .is_err()
    );
    assert!(
        Position::from_diagram(
            &[
                (sq(1, 1), piece(Side::Sente, "P")),
                (sq(1, 1), piece(Side::Gote, "P"))
            ],
            Side::Sente,
            RootMetadata::diagram(0)
        )
        .is_err()
    );
    assert!(
        Position::from_diagram(
            &[(
                sq(1, 1),
                Piece {
                    side: Side::Sente,
                    base: Base::King,
                    promoted: true
                }
            )],
            Side::Sente,
            RootMetadata::diagram(0)
        )
        .is_err()
    );
    assert!(RootMetadata::imported(String::new(), 0).is_err());
    assert!(RootMetadata::imported("é".repeat(513), 0).is_err());
}

#[test]
fn counter_exhaustion_is_a_resource_refusal_and_preserves_everything() {
    let position = Position::from_diagram(
        &[(sq(6, 6), piece(Side::Sente, "K"))],
        Side::Sente,
        RootMetadata::diagram(u64::MAX),
    )
    .unwrap();
    let mut local = LocalPosition::admit(position);
    let before = local.clone();
    assert_eq!(
        local
            .apply(&Action::single(sq(6, 6), sq(6, 5)))
            .unwrap_err(),
        ActionError::Capacity
    );
    assert_eq!(local, before);
    assert_eq!(local.position().declared_total_plies(), u64::MAX);
    assert_eq!(local.position().accepted_plies(), 0);
}

#[test]
fn opponent_inspection_uses_native_owner_and_cannot_mutate_or_consume_rights() {
    let mut p = local(
        &[
            (4, 4, Side::Sente, "Ki"),
            (5, 5, Side::Sente, "G"),
            (6, 4, Side::Gote, "Ln"),
            (6, 1, Side::Gote, "R"),
        ],
        Side::Sente,
    );
    p.apply(&Action {
        from: sq(4, 4),
        route: vec![sq(6, 4)],
        promote: true,
    })
    .unwrap();
    let before = p.clone();
    let choices = p.inspection(sq(6, 4));
    assert!(!choices.is_empty());
    assert!(
        choices
            .iter()
            .all(|c| c.effects.actor == Side::Sente && c.effects.mover_before.is_lion())
    );
    assert_eq!(p, before);
    assert_eq!(
        p.apply(&Action::single(sq(6, 4), sq(7, 4))).unwrap_err(),
        ActionError::WrongOwner
    );
    assert_eq!(p, before);
    assert_eq!(
        p.apply(&Action::single(sq(6, 1), sq(6, 4))).unwrap_err(),
        ActionError::LionLaw
    );
    assert_eq!(p, before);
}

#[test]
fn explicit_lion_rights_need_actual_owner_defense_canonical_squares_and_turn() {
    let p = local(
        &[(6, 6, Side::Sente, "Ln"), (5, 7, Side::Sente, "G")],
        Side::Gote,
    );
    let right = LionProtection {
        owner: Some(Side::Sente),
        squares: vec![sq(6, 6)],
    };
    assert!(LocalPosition::from_parts(p.position().clone(), right.clone()).is_ok());
    assert!(
        LocalPosition::from_parts(
            p.position().clone(),
            LionProtection {
                owner: Some(Side::Gote),
                ..right.clone()
            }
        )
        .is_err()
    );
    assert!(
        LocalPosition::from_parts(
            p.position().clone(),
            LionProtection {
                owner: Some(Side::Sente),
                squares: vec![sq(6, 6), sq(6, 6)]
            }
        )
        .is_err()
    );
    assert!(
        LocalPosition::from_parts(
            p.position().clone(),
            LionProtection {
                owner: Some(Side::Sente),
                squares: vec![sq(5, 7)]
            }
        )
        .is_err()
    );
    assert!(
        LocalPosition::from_parts(
            p.position().clone(),
            LionProtection {
                owner: None,
                ..right
            }
        )
        .is_err()
    );
    let undefended = local(&[(6, 6, Side::Sente, "Ln")], Side::Gote);
    assert!(
        LocalPosition::from_parts(
            undefended.position().clone(),
            LionProtection {
                owner: Some(Side::Sente),
                squares: vec![sq(6, 6)]
            }
        )
        .is_err()
    );
}

#[test]
fn restricted_lion_cannot_turn_and_can_jump_over_both_owners() {
    for (code, middle, end, turned) in [
        ("DK+", sq(5, 5), sq(4, 4), sq(6, 4)),
        ("DH+", sq(6, 5), sq(6, 4), sq(5, 4)),
    ] {
        for owner in [Side::Sente, Side::Gote] {
            let p = local(
                &[
                    (6, 6, Side::Sente, code),
                    (middle.file(), middle.rank(), owner, "P"),
                ],
                Side::Sente,
            );
            let mut jumped = p.clone();
            let (effect, _) = jumped.apply(&Action::single(sq(6, 6), end)).unwrap();
            assert!(effect.captures.is_empty());
            assert_eq!(jumped.position().piece(middle), Some(piece(owner, "P")));
            let walk = Action {
                from: sq(6, 6),
                route: vec![middle, end],
                promote: false,
            };
            let mut walked = p.clone();
            assert_eq!(walked.apply(&walk).is_ok(), owner == Side::Gote);
            let mut turn = p.clone();
            assert_eq!(
                turn.apply(&Action {
                    from: sq(6, 6),
                    route: vec![middle, turned],
                    promote: false
                })
                .unwrap_err(),
                ActionError::Geometry
            );
            assert_eq!(turn, p);
        }
    }
}

#[test]
fn lion_window_qualifying_capture_override_and_undefended_absence() {
    let mut p = local(
        &[
            (5, 1, Side::Sente, "VM"),
            (4, 3, Side::Sente, "Si"),
            (5, 4, Side::Sente, "Ln"),
            (1, 4, Side::Sente, "R"),
            (1, 1, Side::Gote, "Ln"),
            (3, 2, Side::Gote, "Ki+"),
        ],
        Side::Sente,
    );
    p.apply(&Action::single(sq(1, 4), sq(1, 1))).unwrap();
    assert_eq!(p.protection().squares, vec![sq(5, 4)]);
    let (effect, _) = p
        .apply(&Action {
            from: sq(3, 2),
            route: vec![sq(4, 3), sq(5, 4)],
            promote: false,
        })
        .unwrap();
    assert_eq!(effect.captures.len(), 2);
    assert_eq!(effect.captures[0].piece.base, Base::Silver);
    assert!(p.protection().squares.is_empty());
    p.apply(&Action::single(sq(5, 1), sq(5, 4))).unwrap();
    let mut p = local(
        &[
            (4, 4, Side::Sente, "R"),
            (8, 10, Side::Sente, "Ln"),
            (6, 4, Side::Gote, "Ln"),
            (12, 10, Side::Gote, "R"),
        ],
        Side::Sente,
    );
    p.apply(&Action::single(sq(4, 4), sq(6, 4))).unwrap();
    assert!(p.protection().squares.is_empty());
    assert!(p.apply(&Action::single(sq(12, 10), sq(8, 10))).is_ok());
}

#[test]
fn original_removed_pawn_or_gobetween_defender_still_bars_distant_lion() {
    for code in ["P", "GB"] {
        let mut p = local(
            &[
                (3, 1, Side::Sente, "Ln"),
                (2, 2, Side::Gote, code),
                (2, 3, Side::Gote, "Ln"),
            ],
            Side::Sente,
        );
        let before = p.clone();
        assert_eq!(
            p.apply(&Action {
                from: sq(3, 1),
                route: vec![sq(2, 2), sq(2, 3)],
                promote: false
            })
            .unwrap_err(),
            ActionError::LionLaw
        );
        assert_eq!(p, before);
    }
}
