//! Independently specified source examples and adversarial local-boundary properties.
use super::*;

fn sq(f: u8, r: u8) -> Square {
    Square::new(f, r).unwrap()
}
fn local(profile: Profile, turn: Side, entries: &[(u8, u8, Side, Base, bool)]) -> LocalPosition {
    let e = entries
        .iter()
        .enumerate()
        .map(|(id, &(f, r, side, base, promoted))| {
            (
                sq(f, r),
                Piece::new(id as u16, side, base, promoted).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    LocalPosition::admit(
        Position::from_diagram(profile, &e, turn, RootMetadata::diagram(0)).unwrap(),
    )
    .unwrap()
}
fn play(from: (u8, u8), route: &[(u8, u8)], promote: bool) -> Action {
    Action {
        kind: ActionKind::Travel,
        from: sq(from.0, from.1),
        route: route.iter().map(|&(f, r)| sq(f, r)).collect(),
        promote,
        target: None,
    }
}

#[test]
fn actual_standard_setup_preserves_asymmetry_and_original_inventory() {
    for profile in Profile::ALL {
        let p = Position::standard(profile);
        assert_eq!(p.pieces().count(), 156);
        assert_eq!(
            p.pieces().filter(|(_, p)| p.side == Side::Sente).count(),
            78
        );
        for (f, r, base) in [
            (9, 16, Base::K),
            (8, 16, Base::DE),
            (9, 15, Base::Ln),
            (8, 15, Base::FK),
            (9, 14, Base::LHk),
            (8, 14, Base::FEg),
            (9, 13, Base::GGn),
            (8, 13, Base::VGn),
            (12, 11, Base::Dg),
            (5, 11, Base::Dg),
        ] {
            let s = sq(f, r);
            assert_eq!(p.piece(s).unwrap().base, base);
            assert_eq!(p.piece(s.rotated()).unwrap().base, base);
            assert_eq!(p.piece(s.rotated()).unwrap().side, Side::Gote);
        }
        assert_eq!(Position::install(p.state().clone()).unwrap(), p);
    }
}

#[test]
fn malformed_state_and_profile_identities_reject_before_admission() {
    assert!(Square::new(0, 1).is_none());
    assert!(Square::new(16, 17).is_none());
    assert!(serde_json::from_str::<Square>("[256,1]").is_err());
    assert!(serde_json::from_str::<Square>("[1,1,1]").is_err());
    assert!(serde_json::from_str::<Profile>("\"mshogi.tenjiku.unknown.v1\"").is_err());
    let p = Position::standard(Profile::AdamsModern);
    let mut state = p.state().clone();
    state.board.pop();
    assert_eq!(Position::install(state), Err(RootError::BoardLength));
    let mut state = p.state().clone();
    state.board[0].as_mut().unwrap().id = state.board[1].unwrap().id;
    assert_eq!(Position::install(state), Err(RootError::DuplicateIdentity));
    let mut state = p.state().clone();
    state.side = Side::Gote;
    assert_eq!(
        Position::install(state),
        Err(RootError::StandardRootNotDiagram)
    );
    let p = Piece::new(0, Side::Sente, Base::P, false).unwrap();
    assert_eq!(
        Position::from_diagram(
            Profile::AdamsModern,
            &[(sq(1, 1), p), (sq(1, 1), p)],
            Side::Sente,
            RootMetadata::diagram(0)
        ),
        Err(RootError::DuplicateSquare)
    );
    assert_eq!(
        Piece::new(1, Side::Sente, Base::K, true),
        Err(RootError::InvalidPromotion)
    );
}

#[test]
fn complete_action_json_rejects_conflicting_shape_and_caller_effects() {
    for text in [
        r#"{"kind":"travel","from":[8,8],"route":[[8,7]],"promote":false,"target":null}"#,
        r#"{"kind":"travel","from":[8,8],"route":[],"promote":false}"#,
        r#"{"kind":"travel","from":[8,8],"route":[[8,7]],"promote":false,"captures":[]}"#,
        r#"{"kind":"igui","from":[8,8],"route":[],"promote":false}"#,
        r#"{"kind":"igui","from":[8,8],"route":[[9,8]],"promote":false,"target":[9,8]}"#,
        r#"{"kind":"igui","from":[8,8],"route":[],"promote":true,"target":[9,8]}"#,
        r#"{"kind":"travel","from":[8,8],"route":[[8,7]],"promote":false,"promote":true}"#,
    ] {
        assert!(serde_json::from_str::<Action>(text).is_err(), "{text}");
    }
    for action in [
        Action::single(sq(8, 8), sq(8, 7)),
        Action::igui(sq(8, 8), sq(9, 8)),
    ] {
        assert_eq!(
            serde_json::from_str::<Action>(&serde_json::to_string(&action).unwrap()).unwrap(),
            action
        );
    }
}

#[test]
fn special_geometry_combinatorics_count_complete_paths_not_only_destinations() {
    // Source combinatorics: Lion24direct+8*8two-step; area8+64+512 plus nonduplicate rays.
    let cases = [
        (Base::Ln, 88, 88),
        (Base::LHk, 109, 109),
        (Base::FiD, 622, 622),
        (Base::VGn, 609, 609),
        (Base::FEg, 59, 75),
        (Base::HF, 56, 56),
        (Base::SE, 53, 53),
    ];
    for (base, a, b) in cases {
        for (profile, n) in [(Profile::AdamsModern, a), (Profile::MullerCompleted, b)] {
            let l = local(profile, Side::Sente, &[(8, 8, Side::Sente, base, false)]);
            assert_eq!(
                l.geometry_actions_from(sq(8, 8)).len(),
                n,
                "{base:?}/{profile:?}"
            );
        }
    }
    for (profile, n) in [(Profile::AdamsModern, 29), (Profile::MullerCompleted, 42)] {
        let l = local(
            profile,
            Side::Sente,
            &[(8, 8, Side::Sente, Base::ChS, true)],
        );
        assert_eq!(l.geometry_actions_from(sq(8, 8)).len(), n);
    }
}

#[test]
fn lion_paths_capture_in_order_without_chu_protection_or_royal_safety() {
    for profile in Profile::ALL {
        let mut l = local(
            profile,
            Side::Sente,
            &[
                (8, 8, Side::Sente, Base::Ln, false),
                (9, 8, Side::Gote, Base::K, false),
                (10, 8, Side::Gote, Base::DE, true),
                (10, 1, Side::Gote, Base::R, false),
            ],
        );
        let a = play((8, 8), &[(9, 8), (10, 8)], false);
        let (before, _) = l.simulate(&a).unwrap();
        let (e, mut undo) = l.apply(&a).unwrap();
        assert_eq!(
            e.captures().map(|c| c.square).collect::<Vec<_>>(),
            [sq(9, 8), sq(10, 8)]
        );
        assert!(e.mover_after.unwrap().side == Side::Sente);
        assert_eq!(l.position().piece(sq(10, 8)).unwrap().base, Base::Ln);
        assert_eq!(
            before.position().piece(sq(10, 8)),
            l.position().piece(sq(10, 8))
        );
        let after = l.position().clone();
        let mut redo = l.undo(&mut undo).unwrap();
        assert_eq!(l.redo(&mut redo).unwrap().0, e);
        assert_eq!(l.position(), &after);
    }
}

#[test]
fn area_capture_ends_action_and_friend_blocks_intermediate_landing() {
    for base in [Base::FiD, Base::VGn] {
        for side in [Side::Sente, Side::Gote] {
            let l = local(
                Profile::AdamsModern,
                side,
                &[
                    (8, 8, side, base, false),
                    (9, 8, side.opponent(), Base::P, false),
                ],
            );
            assert_eq!(
                l.simulate(&play((8, 8), &[(9, 8), (10, 8)], false))
                    .unwrap_err(),
                ActionError::Geometry
            );
            let l = local(
                Profile::AdamsModern,
                side,
                &[(8, 8, side, base, false), (9, 8, side, Base::P, false)],
            );
            assert_eq!(
                l.simulate(&play((8, 8), &[(9, 8), (10, 8)], false))
                    .unwrap_err(),
                ActionError::FriendlyDestination
            );
        }
    }
}

#[test]
fn fire_demon_profiles_order_burn_batch_before_suicide_and_preserve_stationary_demons() {
    for profile in Profile::ALL {
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 10, Side::Sente, Base::FiD, false),
                (9, 8, Side::Gote, Base::FiD, false),
                (8, 7, Side::Gote, Base::P, false),
                (7, 9, Side::Gote, Base::G, false),
                (7, 8, Side::Sente, Base::P, false),
            ],
        );
        let (n, e) = l.simulate(&play((8, 10), &[(8, 8)], false)).unwrap();
        assert!(e.mover_after.is_none());
        assert!(n.position().piece(sq(8, 8)).is_none());
        assert_eq!(n.position().piece(sq(9, 8)).unwrap().base, Base::FiD);
        assert!(n.position().piece(sq(7, 8)).is_some());
        if profile == Profile::AdamsModern {
            assert!(
                matches!(&e.phases[0],EffectPhase::Burn{victims}if victims.iter().map(|c|c.square).collect::<Vec<_>>()==[sq(8,7),sq(7,9)])
            );
            assert!(matches!(e.phases[1], EffectPhase::MoverDies { .. }));
        } else {
            assert_eq!(e.phases.len(), 1);
            assert!(n.position().piece(sq(8, 7)).is_some());
            assert!(n.position().piece(sq(7, 9)).is_some());
        }
    }
}

#[test]
fn capturing_demon_clears_its_hazard_and_new_promotion_never_burns_that_turn() {
    for profile in Profile::ALL {
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 10, Side::Sente, Base::FiD, false),
                (8, 8, Side::Gote, Base::FiD, false),
                (8, 7, Side::Gote, Base::P, false),
            ],
        );
        let (n, e) = l.simulate(&play((8, 10), &[(8, 8)], false)).unwrap();
        assert!(e.mover_after.is_some());
        assert!(n.position().piece(sq(8, 7)).is_none());
        assert!(matches!(e.phases[0], EffectPhase::Capture { .. }));
        assert!(matches!(e.phases[1], EffectPhase::Burn { .. }));
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 6, Side::Sente, Base::WBf, false),
                (9, 5, Side::Gote, Base::P, false),
            ],
        );
        let (n, e) = l.simulate(&play((8, 6), &[(8, 5)], true)).unwrap();
        assert_eq!(e.mover_after.unwrap().face(), Face::FiD);
        assert_eq!(e.phases.len(), 1);
        assert!(matches!(e.phases[0], EffectPhase::Promotion { .. }));
        assert!(n.position().piece(sq(9, 5)).is_some());
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 6, Side::Sente, Base::WBf, false),
                (9, 5, Side::Gote, Base::FiD, false),
            ],
        );
        let (_, e) = l.simulate(&play((8, 6), &[(8, 5)], true)).unwrap();
        assert!(e.mover_after.is_none());
        assert!(
            e.phases
                .iter()
                .all(|p| !matches!(p, EffectPhase::Promotion { .. }))
        );
    }
}

#[test]
fn complete_turn_hazard_ignores_lion_transit_and_does_not_cascade() {
    let l = local(
        Profile::AdamsModern,
        Side::Sente,
        &[
            (8, 10, Side::Sente, Base::Ln, false),
            (9, 10, Side::Gote, Base::FiD, false),
        ],
    );
    assert!(
        l.simulate(&play((8, 10), &[(8, 9), (8, 8)], false))
            .unwrap()
            .1
            .mover_after
            .is_some()
    );
    let l = local(
        Profile::AdamsModern,
        Side::Sente,
        &[
            (8, 9, Side::Sente, Base::P, false),
            (9, 8, Side::Gote, Base::FiD, false),
            (9, 9, Side::Sente, Base::G, false),
        ],
    );
    let (n, e) = l.simulate(&play((8, 9), &[(8, 8)], false)).unwrap();
    assert!(e.mover_after.is_none());
    assert!(n.position().piece(sq(9, 9)).is_some());
}

#[test]
fn ranked_interveners_and_destination_are_distinct_and_profile_bound() {
    for profile in Profile::ALL {
        for blocker in [Base::P, Base::BGn, Base::RGn, Base::VGn, Base::GGn, Base::K] {
            let l = local(
                profile,
                Side::Sente,
                &[
                    (8, 8, Side::Sente, Base::BGn, false),
                    (9, 7, Side::Sente, blocker, false),
                    (11, 5, Side::Gote, Base::P, false),
                ],
            );
            assert_eq!(
                l.simulate(&play((8, 8), &[(11, 5)], false)).is_ok(),
                blocker == Base::P
            );
        }
        for victim in [Base::P, Base::BGn, Base::RGn, Base::VGn, Base::GGn, Base::K] {
            let l = local(
                profile,
                Side::Sente,
                &[
                    (8, 8, Side::Sente, Base::BGn, false),
                    (9, 7, Side::Sente, Base::P, false),
                    (11, 5, Side::Gote, victim, false),
                ],
            );
            assert_eq!(
                l.simulate(&play((8, 8), &[(11, 5)], false)).is_ok(),
                victim == Base::P || profile == Profile::MullerCompleted && victim != Base::K
            );
        }
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 8, Side::Sente, Base::BGn, false),
                (11, 5, Side::Gote, Base::K, false),
            ],
        );
        assert!(l.simulate(&play((8, 8), &[(11, 5)], true)).is_ok()); // Eligible original BGn captures without jumping.
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 8, Side::Sente, Base::BGn, false),
                (9, 7, Side::Gote, Base::P, false),
            ],
        );
        assert!(l.simulate(&play((8, 8), &[(11, 5)], false)).is_err());
    }
}

#[test]
fn free_eagle_complete_choices_preserve_the_actual_profile_difference() {
    for profile in Profile::ALL {
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 8, Side::Sente, Base::FEg, false),
                (9, 8, Side::Sente, Base::P, false),
            ],
        );
        assert_eq!(
            l.simulate(&play((8, 8), &[(10, 8)], false)).is_ok(),
            profile == Profile::AdamsModern
        );
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 8, Side::Sente, Base::FEg, false),
                (9, 7, Side::Gote, Base::P, false),
                (10, 8, Side::Gote, Base::G, false),
            ],
        );
        assert_eq!(
            l.simulate(&play((8, 8), &[(9, 7), (10, 8)], false)).is_ok(),
            profile == Profile::MullerCompleted
        );
    }
}

#[test]
fn tetrarch_skips_only_first_square_and_has_stationary_not_adjacent_travel_capture() {
    for profile in Profile::ALL {
        let l = local(
            profile,
            Side::Sente,
            &[
                (8, 8, Side::Sente, Base::ChS, true),
                (9, 7, Side::Sente, Base::P, false),
                (10, 6, Side::Gote, Base::P, false),
                (9, 8, Side::Gote, Base::G, false),
            ],
        );
        assert!(l.simulate(&play((8, 8), &[(10, 6)], false)).is_ok());
        assert!(l.simulate(&play((8, 8), &[(11, 5)], false)).is_err());
        assert!(l.simulate(&play((8, 8), &[(9, 8)], false)).is_err());
        let (n, e) = l.simulate(&Action::igui(sq(8, 8), sq(9, 8))).unwrap();
        assert_eq!(e.action.to(), Some(sq(8, 8)));
        assert!(n.position().piece(sq(8, 8)).is_some());
        assert!(n.position().piece(sq(9, 8)).is_none());
        assert_eq!(
            l.simulate(&play((8, 8), &[(8, 4)], false)).is_ok(),
            profile == Profile::MullerCompleted
        );
        assert!(l.simulate(&play((8, 8), &[(12, 8)], false)).is_err());
    }
}

#[test]
fn mandatory_promotion_rights_and_once_promoted_originals_are_not_face_chains() {
    for (base, from, to) in [
        (Base::P, (8, 2), (8, 1)),
        (Base::L, (8, 2), (8, 1)),
        (Base::I, (8, 2), (8, 1)),
        (Base::N, (8, 4), (9, 2)),
    ] {
        let l = local(
            Profile::AdamsModern,
            Side::Sente,
            &[(from.0, from.1, Side::Sente, base, false)],
        );
        assert_eq!(
            l.simulate(&play(from, &[to], false)).unwrap_err(),
            ActionError::Promotion
        );
        assert!(l.simulate(&play(from, &[to], true)).is_ok());
        assert!(
            l.geometry_actions_from(sq(from.0, from.1))
                .contains(&play(from, &[to], false))
        );
    }
    let l = local(
        Profile::AdamsModern,
        Side::Sente,
        &[(8, 6, Side::Sente, Base::Ky, true)],
    );
    assert!(l.simulate(&play((8, 6), &[(8, 5)], true)).is_err());
    assert_eq!(
        l.simulate(&play((8, 6), &[(8, 5)], false))
            .unwrap()
            .1
            .mover_after
            .unwrap()
            .face(),
        Face::Ln
    );
    let l = local(
        Profile::AdamsModern,
        Side::Sente,
        &[(8, 6, Side::Sente, Base::Ln, false)],
    );
    assert_eq!(
        l.simulate(&play((8, 6), &[(8, 5), (8, 6)], true))
            .unwrap()
            .1
            .mover_after
            .unwrap()
            .face(),
        Face::LHk
    );
}

#[test]
fn first_refusal_belongs_to_original_and_only_its_next_move_is_restricted() {
    let mut l = local(
        Profile::AdamsModern,
        Side::Sente,
        &[
            (8, 6, Side::Sente, Base::G, false),
            (1, 1, Side::Gote, Base::K, false),
            (6, 5, Side::Sente, Base::P, false),
        ],
    );
    l.apply(&play((8, 6), &[(8, 5)], false)).unwrap();
    l.apply(&play((1, 1), &[(2, 1)], false)).unwrap();
    assert_eq!(
        l.position().piece(sq(8, 5)).unwrap().right,
        PromotionRight::FirstDeclinedNextMove
    );
    let before = l.position().clone();
    assert_eq!(
        l.apply(&play((8, 5), &[(8, 4)], true)).unwrap_err(),
        ActionError::Promotion
    );
    assert_eq!(l.position(), &before);
    l.apply(&play((8, 5), &[(8, 4)], false)).unwrap();
    l.apply(&play((2, 1), &[(3, 1)], false)).unwrap();
    l.apply(&play((8, 4), &[(8, 3)], true)).unwrap();
    assert_eq!(l.position().piece(sq(8, 3)).unwrap().face(), Face::R);
    assert_eq!(
        l.position().piece(sq(6, 5)).unwrap().right,
        PromotionRight::Unseen
    );
}

#[test]
fn reversal_is_instance_lifo_single_use_and_branch_bound() {
    let mut l = local(
        Profile::AdamsModern,
        Side::Sente,
        &[
            (8, 8, Side::Sente, Base::K, false),
            (1, 1, Side::Gote, Base::K, false),
        ],
    );
    let initial = l.position().clone();
    let (_, mut a) = l.apply(&play((8, 8), &[(8, 7)], false)).unwrap();
    let mut clone = l.clone();
    assert!(clone.undo(&mut a).is_err());
    let (_, mut b) = l.apply(&play((1, 1), &[(2, 1)], false)).unwrap();
    assert!(l.undo(&mut a).is_err());
    let mut rb = l.undo(&mut b).unwrap();
    assert!(l.undo(&mut b).is_err());
    let _ = l.undo(&mut a).unwrap();
    assert_eq!(l.position(), &initial);
    l.apply(&play((8, 8), &[(9, 8)], false)).unwrap();
    assert!(l.redo(&mut rb).is_err());
}

#[test]
fn cancellation_counter_refusal_and_inspection_preserve_every_live_field() {
    let mut l = local(
        Profile::AdamsModern,
        Side::Sente,
        &[
            (8, 8, Side::Sente, Base::Ln, false),
            (1, 1, Side::Gote, Base::K, false),
        ],
    );
    let before = l.position().clone();
    let mut polls = 0;
    assert_eq!(
        l.legal_actions_with_stop(|| {
            polls += 1;
            polls > 20
        })
        .unwrap_err(),
        ActionError::Stopped
    );
    assert_eq!(l.position(), &before);
    assert_eq!(
        l.apply_with_stop(&play((8, 8), &[(9, 8)], false), || true)
            .unwrap_err(),
        ActionError::Stopped
    );
    assert_eq!(l.position(), &before);
    assert!(!l.inspection(sq(1, 1)).is_empty());
    assert!(!l.geometry_actions_from(sq(1, 1)).is_empty());
    assert!(l.choices_from(sq(1, 1)).is_empty());
    assert_eq!(l.position(), &before);
    let mut state = before.state().clone();
    state.accepted_plies = u64::MAX;
    let mut l = LocalPosition::admit(Position::install(state).unwrap()).unwrap();
    let before = l.position().clone();
    assert!(!l.legal_actions().is_empty());
    assert!(l.simulate(&play((8, 8), &[(9, 8)], false)).is_ok());
    assert_eq!(
        l.apply(&play((8, 8), &[(9, 8)], false)).unwrap_err(),
        ActionError::Capacity
    );
    assert_eq!(l.position(), &before);
}

#[test]
fn native_sequence_rotates_and_restores_exact_state_without_assuming_rules_oracles() {
    for profile in Profile::ALL {
        let mut l = local(
            profile,
            Side::Sente,
            &[
                (8, 8, Side::Sente, Base::Ln, false),
                (3, 3, Side::Gote, Base::VGn, false),
                (1, 1, Side::Gote, Base::K, false),
                (16, 16, Side::Sente, Base::K, false),
            ],
        );
        let before = l.position().clone();
        let mut tokens = vec![];
        let mut seed = 0x8742u64;
        for _ in 0..18 {
            let choices = l.legal_actions();
            if choices.is_empty() {
                break;
            }
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let a = &choices[(seed as usize) % choices.len()];
            let (_, t) = l.apply(a).unwrap();
            tokens.push(t);
        }
        while let Some(mut t) = tokens.pop() {
            l.undo(&mut t).unwrap();
        }
        assert_eq!(l.position(), &before);
        let entries = before
            .pieces()
            .map(|(s, mut p)| {
                p.side = p.side.opponent();
                (s.rotated(), p)
            })
            .collect::<Vec<_>>();
        let rot = LocalPosition::admit(
            Position::from_diagram(profile, &entries, Side::Gote, RootMetadata::diagram(0))
                .unwrap(),
        )
        .unwrap();
        let mut expected = l
            .legal_actions()
            .into_iter()
            .map(|mut a| {
                a.from = a.from.rotated();
                for s in &mut a.route {
                    *s = s.rotated()
                }
                a.target = a.target.map(Square::rotated);
                a
            })
            .collect::<Vec<_>>();
        let mut actual = rot.legal_actions();
        expected.sort();
        actual.sort();
        assert_eq!(expected, actual);
    }
}
