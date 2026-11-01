use super::{
    conformance::{self, action, root, snapshot, sq},
    transition::TransitionError,
    *,
};
use mshogi_conformance::{Corpus, properties};
use serde_json::{Value, json};

fn diagram(entries: &[(&str, Side, Face)], hands: [Hand; 2], side: Side, counter: u64) -> Position {
    Position::from_diagram(
        &entries
            .iter()
            .map(|&(s, side, face)| (sq(s), Piece { side, face }))
            .collect::<Vec<_>>(),
        hands,
        side,
        RootMetadata::diagram(counter),
    )
    .unwrap()
}
fn move_to(from: &str, to: &str, result_face: Face) -> Action {
    Action::Move {
        from: sq(from),
        to: sq(to),
        result_face,
    }
}
fn resources(p: &Position) -> Value {
    json!(
        Face::BASES
            .into_iter()
            .map(|f| Square::all()
                .filter_map(|s| p.piece(s))
                .filter(|v| v.face.base() == f)
                .count()
                + [Side::Sente, Side::Gote]
                    .into_iter()
                    .map(|s| usize::from(p.hand(s).count(f)))
                    .sum::<usize>())
            .collect::<Vec<_>>()
    )
}
fn runtime(p: &Position) -> (u64, u64, u64, u64) {
    (p.node, p.next_node, p.branch, p.revision)
}
fn rotate(p: &Position) -> Position {
    let entries: Vec<_> = Square::all()
        .filter_map(|s| {
            p.piece(s).map(|piece| {
                (
                    s.rotated(),
                    Piece {
                        side: piece.side.opponent(),
                        face: piece.face,
                    },
                )
            })
        })
        .collect();
    let mut out = Position::from_diagram(
        &entries,
        [p.hand(Side::Gote), p.hand(Side::Sente)],
        p.side_to_move().opponent(),
        RootMetadata::diagram(p.root().declared_completed_plies()),
    )
    .unwrap();
    out.state.accepted_plies = p.accepted_plies();
    out
}
fn rotated_action(a: Action) -> Action {
    match a {
        Action::Move {
            from,
            to,
            result_face,
        } => Action::Move {
            from: from.rotated(),
            to: to.rotated(),
            result_face,
        },
        Action::Drop { face, to } => Action::Drop {
            face,
            to: to.rotated(),
        },
    }
}
fn all_structural_candidates(p: &Position) -> Vec<Action> {
    let mut out = Vec::new();
    for from in Square::all() {
        if let Some(piece) = p.piece(from).filter(|v| v.side == p.side_to_move()) {
            for to in Square::all() {
                let action = Action::Move {
                    from,
                    to,
                    result_face: piece.face.moved_face(piece.side, from, to),
                };
                if p.resolve_structural(action).is_ok() {
                    out.push(action);
                }
            }
        }
    }
    for face in Face::HANDS {
        if p.hand(p.side_to_move()).count(face) > 0 {
            for to in Square::all() {
                let action = Action::Drop { face, to };
                if p.resolve_structural(action).is_ok() {
                    out.push(action);
                }
            }
        }
    }
    out
}
#[test]
fn coordinate_domains_and_both_owner_axes() {
    for f in 0..=8 {
        for r in 0..=8 {
            assert_eq!(
                Square::new(f, r).is_some(),
                (1..=7).contains(&f) && (1..=7).contains(&r)
            );
        }
    }
    let all: Vec<_> = Square::all().collect();
    assert_eq!(all.len(), 49);
    for s in all {
        assert_eq!(s.to_string().parse::<Square>(), Ok(s));
        assert_eq!(s.rotated().rotated(), s);
        assert_eq!(s.rotated().file(), 8 - s.file());
        assert_eq!(s.rotated().rank(), 8 - s.rank());
    }
    for bad in ["0a", "8b", "4h", "11a", "４a", "4A", "4a ", "", "4"] {
        assert!(bad.parse::<Square>().is_err());
    }
    assert_eq!(sq("4d").offset(Side::Sente, 1, 2), Some(sq("3b")));
    assert_eq!(sq("4d").offset(Side::Gote, 1, 2), Some(sq("5f")));
    for side in [Side::Sente, Side::Gote] {
        assert_eq!(sq("4d").offset(side, i8::MIN, i8::MAX), None);
    }
}
#[test]
fn roots_preserve_identity_inventory_and_counter_provenance() {
    let p = Position::standard();
    assert_eq!(p.profile_id(), PROFILE_ID);
    assert_eq!(p.profile_edition(), PROFILE_EDITION);
    assert_eq!(conformance::board(&p).len(), 32);
    assert!(!p.root().earlier_history_unknown());
    assert!(p.root().historical_reachability_proven());
    assert_eq!(p.root().kind(), RootKind::Standard);
    assert_eq!(p.accepted_plies(), 0);
    let entries = [
        (
            sq("7g"),
            Piece {
                side: Side::Sente,
                face: Face::Phoenix,
            },
        ),
        (
            sq("1a"),
            Piece {
                side: Side::Gote,
                face: Face::Phoenix,
            },
        ),
    ];
    let meta = RootMetadata::imported("explicit experiment source".into(), 987).unwrap();
    let mut q =
        Position::from_diagram(&entries, [Hand::empty(); 2], Side::Gote, meta.clone()).unwrap();
    assert_eq!(q.root(), &meta);
    assert!(q.root().earlier_history_unknown());
    assert!(!q.root().historical_reachability_proven());
    assert_eq!(q.declared_total_plies(), 987);
    let (_, mut u) = q
        .apply_structural(move_to("1a", "1b", Face::Phoenix))
        .unwrap();
    assert_eq!(q.declared_total_plies(), 988);
    assert_eq!(q.root(), &meta);
    let mut r = q.undo_structural(&mut u).unwrap();
    assert_eq!(q.accepted_plies(), 0);
    assert_eq!(q.root(), &meta);
    q.redo_structural(&mut r).unwrap();
    assert_eq!(q.root(), &meta);
    assert_eq!(
        Position::from_diagram(&entries, [Hand::empty(); 2], Side::Sente, p.root().clone())
            .unwrap_err(),
        RootError::StandardRootNotDiagram
    );
    assert!(RootMetadata::imported("".into(), 0).is_err());
    assert!(RootMetadata::imported("x".repeat(1025), 0).is_err());
    assert!(RootMetadata::imported("é".repeat(512), u64::MAX).is_ok());
    for f in [Face::Phoenix, Face::Eagle, Face::Goose] {
        assert_eq!(
            Hand::from_counts(&[(f, 0)]),
            Err(RootError::InvalidHandFace)
        );
    }
    assert_eq!(
        Hand::from_counts(&[(Face::Falcon, 0), (Face::Falcon, 1)]),
        Err(RootError::DuplicateHandFace)
    );
    assert_eq!(
        Hand::from_counts(&[(Face::Swallow, 17)]),
        Err(RootError::ExcessInventory)
    );
    let excessive = Hand::from_counts(&[(Face::LeftQuail, 2)]).unwrap();
    let mut board = entries.to_vec();
    board.push((
        sq("4d"),
        Piece {
            side: Side::Gote,
            face: Face::LeftQuail,
        },
    ));
    assert_eq!(
        Position::from_diagram(
            &board,
            [excessive, Hand::empty()],
            Side::Sente,
            RootMetadata::diagram(0)
        )
        .unwrap_err(),
        RootError::ExcessInventory
    );
    for face in [Face::Eagle, Face::Goose] {
        let cap = face.cap();
        let mut board = entries.to_vec();
        for s in Square::all()
            .filter(|s| !entries.iter().any(|e| e.0 == *s))
            .take(usize::from(cap) + 1)
        {
            board.push((
                s,
                Piece {
                    side: Side::Sente,
                    face,
                },
            ));
        }
        assert_eq!(
            Position::from_diagram(
                &board,
                [Hand::empty(); 2],
                Side::Sente,
                RootMetadata::diagram(0)
            )
            .unwrap_err(),
            RootError::ExcessInventory
        );
    }
}
#[test]
fn all_40_accepted_board_transitions_execute_reversal_two_resignations_stay_pending() {
    let c = Corpus::embedded().unwrap();
    let mut count = 0;
    for f in c.fixtures().iter().filter(|f| {
        f.scope.starts_with("native_admission")
            && f.expected.get("accepted") == Some(&json!(true))
            && matches!(f.given["command"]["kind"].as_str(), Some("move" | "drop"))
    }) {
        let mut p = root(&f.given["root"]).unwrap();
        let a = action(&f.given["command"]);
        let before = p.state().clone();
        let material = resources(&p);
        let r0 = p.revision;
        let (effect, mut undo) = p
            .apply_structural(a)
            .unwrap_or_else(|e| panic!("{}: {e:?}", f.id));
        let applied = p.state().clone();
        assert_eq!(p.accepted_plies(), 1);
        assert_eq!(p.side_to_move(), before.side.opponent());
        assert_eq!(p.root(), &before.root);
        assert_eq!(effect.action, a);
        properties::preserves_material(&material, &resources(&p)).unwrap();
        if let Some(probes) = f.expected.get("native_movement_probes") {
            for (text, expected) in probes.as_object().unwrap() {
                assert_eq!(
                    p.geometrically_reaches(sq(&text[..2]), sq(&text[2..])),
                    expected.as_bool().unwrap()
                );
            }
        }
        let mut redo = p.undo_structural(&mut undo).unwrap();
        assert_eq!(p.state(), &before);
        assert!(p.revision > r0);
        let (repeated, _) = p.redo_structural(&mut redo).unwrap();
        assert_eq!(p.state(), &applied);
        assert_eq!(effect, repeated);
        // Shared helper invokes real native operations; no expected snapshots echoed.
        let mut q = root(&f.given["root"]).unwrap();
        properties::round_trip(
            &mut q,
            |p| p.state().clone(),
            |p| p.apply_structural(a).map(|(_, u)| u),
            |p, mut u| p.undo_structural(&mut u).map(|_| ()),
        )
        .unwrap();
        count += 1;
    }
    assert_eq!(count, 40);
    assert_eq!(
        c.fixtures()
            .iter()
            .filter(|f| f.expected.get("accepted") == Some(&json!(true))
                && f.given["command"]["kind"] == "resign")
            .count(),
        2
    );
}
#[test]
fn native_rotation_symmetry_for_every_face_square_and_fixture_transition() {
    // Independent native rotation, not the harness JSON transformer.
    for f in Corpus::embedded().unwrap().fixtures().iter().filter(|f| {
        f.expected.get("accepted") == Some(&json!(true))
            && matches!(f.given["command"]["kind"].as_str(), Some("move" | "drop"))
    }) {
        let mut p = root(&f.given["root"]).unwrap();
        let mut r = rotate(&p);
        let a = action(&f.given["command"]);
        let (e, _) = p.apply_structural(a).unwrap();
        let (re, _) = r.apply_structural(rotated_action(a)).unwrap();
        assert_eq!(snapshot(&rotate(&p)), snapshot(&r), "{}", f.id);
        assert_eq!(e.resulting_piece.face, re.resulting_piece.face);
        assert_eq!(e.captured.map(|v| v.face), re.captured.map(|v| v.face));
    }
    for face in Face::ALL {
        for origin in Square::all() {
            let mut p = Position::standard();
            p.state.board = [None; 49];
            p.state.board[origin.index()] = Some(Piece {
                side: Side::Sente,
                face,
            });
            // Geometry-only internal setup intentionally does not masquerade as an admitted root.
            let mut r = p.clone();
            r.state.board = [None; 49];
            r.state.board[origin.rotated().index()] = Some(Piece {
                side: Side::Gote,
                face,
            });
            for to in Square::all() {
                assert_eq!(
                    p.geometrically_reaches(origin, to),
                    r.geometrically_reaches(origin.rotated(), to.rotated())
                );
            }
        }
    }
}
#[test]
fn actual_zone_drop_wait_exit_within_and_swallow_promotion() {
    for owner in [Side::Sente, Side::Gote] {
        let mut p = diagram(
            &[
                ("7g", Side::Sente, Face::Phoenix),
                ("1a", Side::Gote, Face::Phoenix),
                ("6e", Side::Sente, Face::Crane),
            ],
            [
                Hand::from_counts(&[(Face::Falcon, 1), (Face::Swallow, 1)]).unwrap(),
                Hand::empty(),
            ],
            Side::Sente,
            18,
        );
        if owner == Side::Gote {
            p = rotate(&p);
        }
        let transform = |a| {
            if owner == Side::Gote {
                rotated_action(a)
            } else {
                a
            }
        };
        let seq = [
            Action::Drop {
                face: Face::Falcon,
                to: sq("4b"),
            },
            move_to("1a", "1b", Face::Phoenix),
            move_to("6e", "6d", Face::Crane),
            move_to("1b", "1a", Face::Phoenix),
        ];
        for a in seq {
            p.apply_structural(transform(a)).unwrap();
        }
        let fa_square = if owner == Side::Sente {
            sq("4b")
        } else {
            sq("4b").rotated()
        };
        assert_eq!(p.piece(fa_square).unwrap().face, Face::Falcon);
        p.apply_structural(transform(move_to("4b", "3c", Face::Eagle)))
            .unwrap();
        p.apply_structural(transform(move_to("1a", "1b", Face::Phoenix)))
            .unwrap();
        p.apply_structural(transform(Action::Drop {
            face: Face::Swallow,
            to: sq("5b"),
        }))
        .unwrap();
        p.apply_structural(transform(move_to("1b", "1a", Face::Phoenix)))
            .unwrap();
        p.apply_structural(transform(move_to("5b", "5a", Face::Goose)))
            .unwrap();
        assert_eq!(p.hand(owner).count(Face::Falcon), 0);
        assert_eq!(p.hand(owner).count(Face::Swallow), 0);
    }
}
#[test]
fn all_nonroyal_faces_capture_demote_and_preserve_handed_inventory() {
    for victim in Face::ALL.into_iter().filter(|f| *f != Face::Phoenix) {
        for owner in [Side::Sente, Side::Gote] {
            let mut p = diagram(
                &[
                    ("7g", Side::Sente, Face::Phoenix),
                    ("1a", Side::Gote, Face::Phoenix),
                    ("4e", Side::Sente, Face::Crane),
                    ("4d", Side::Gote, victim),
                ],
                [Hand::empty(); 2],
                Side::Sente,
                0,
            );
            let mut a = move_to("4e", "4d", Face::Crane);
            if owner == Side::Gote {
                p = rotate(&p);
                a = rotated_action(a);
            }
            let original = p.state().clone();
            let (e, mut u) = p.apply_structural(a).unwrap();
            assert_eq!(
                e.captured,
                Some(Piece {
                    side: owner.opponent(),
                    face: victim
                })
            );
            assert_eq!(e.hand_change, Some((victim.base(), 1)));
            assert_eq!(p.hand(owner).count(victim.base()), 1);
            let mut r = p.undo_structural(&mut u).unwrap();
            assert_eq!(p.state(), &original);
            p.redo_structural(&mut r).unwrap();
            assert_eq!(p.hand(owner).count(victim.base()), 1);
        }
    }
}
#[test]
fn basic_invalid_actions_reject_all_state_and_runtime_atomically() {
    use TransitionError::*;
    let p = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("1a", Side::Gote, Face::Phoenix),
            ("4d", Side::Sente, Face::Eagle),
            ("3e", Side::Gote, Face::Crane),
            ("4c", Side::Sente, Face::Swallow),
            ("2b", Side::Gote, Face::Falcon),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        0,
    );
    let cases = [
        (move_to("6c", "6b", Face::Crane), EmptySource),
        (move_to("2b", "2c", Face::Falcon), WrongOwner),
        (move_to("4d", "4d", Face::Eagle), SameSquare),
        (move_to("4d", "4c", Face::Eagle), FriendlyDestination),
        (move_to("4d", "1a", Face::Eagle), RoyalCapture),
        (move_to("4d", "2f", Face::Eagle), Geometry),
        (move_to("4c", "4b", Face::Swallow), ResultFace),
        (
            Action::Drop {
                face: Face::Goose,
                to: sq("6d"),
            },
            InvalidDropFace,
        ),
        (
            Action::Drop {
                face: Face::Phoenix,
                to: sq("6d"),
            },
            InvalidDropFace,
        ),
        (
            Action::Drop {
                face: Face::Crane,
                to: sq("6d"),
            },
            MissingHand,
        ),
        (
            Action::Drop {
                face: Face::Crane,
                to: sq("3e"),
            },
            OccupiedDrop,
        ),
    ];
    for (a, error) in cases {
        let mut q = p.clone();
        let rt = runtime(&q);
        let state = q.state().clone();
        assert_eq!(q.apply_structural(a).unwrap_err(), error);
        assert_eq!(q.state(), &state);
        assert_eq!(runtime(&q), rt);
        properties::rejection(
            &mut q,
            |p| (p.state().clone(), runtime(p)),
            |p| p.apply_structural(a),
        )
        .unwrap();
    }
    let corpus = Corpus::embedded().unwrap();
    // Classify from the selected contract, not from whether native resolution
    // happens to reject. Safety/drop restrictions are deliberately NOT in this set.
    let structural_rejections = [
        ("TORI-G-EAGLE-BLOCK", Geometry),
        ("TORI-G-RAY-NO-BYPASS", Geometry),
        ("TORI-G-FRIEND-BLOCK", Geometry),
        ("TORI-D-EMPTY", OccupiedDrop),
        ("TORI-D-NO-HAND", MissingHand),
        ("TORI-D-PROMOTED", InvalidDropFace),
        ("TORI-P-NO-OPTION", ResultFace),
    ];
    let mut checked = 0;
    for (base, error) in structural_rejections {
        for id in [base.to_owned(), format!("{base}-ROTATED")] {
            let f = corpus.fixtures().iter().find(|f| f.id == id).unwrap();
            let mut p = root(&f.given["root"]).unwrap();
            let a = action(&f.given["command"]);
            let before = p.state().clone();
            let r = runtime(&p);
            assert_eq!(p.apply_structural(a).unwrap_err(), error, "{id}");
            assert_eq!(p.state(), &before, "{id}");
            assert_eq!(runtime(&p), r, "{id}");
            properties::rejection(
                &mut p,
                |p| (p.state().clone(), runtime(p)),
                |p| p.apply_structural(a),
            )
            .unwrap();
            checked += 1;
        }
    }
    assert_eq!(checked, 14);
}
#[test]
fn undo_redo_are_single_use_bound_lifo_and_branch_safe() {
    let mut p = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("1a", Side::Gote, Face::Phoenix),
            ("4e", Side::Sente, Face::Crane),
            ("4d", Side::Gote, Face::Eagle),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        41,
    );
    let initial = p.state().clone();
    let a = move_to("4e", "4d", Face::Crane);
    let b = move_to("1a", "1b", Face::Phoenix);
    let c = Action::Drop {
        face: Face::Falcon,
        to: sq("3b"),
    };
    let (_, mut ua) = p.apply_structural(a).unwrap();
    let (_, mut ub) = p.apply_structural(b).unwrap();
    let rt = runtime(&p);
    let before = p.state().clone();
    assert_eq!(
        p.undo_structural(&mut ua).unwrap_err(),
        TransitionError::InvalidToken
    );
    assert_eq!(p.state(), &before);
    assert_eq!(runtime(&p), rt);
    let mut other = p.clone();
    assert_eq!(
        other.undo_structural(&mut ub).unwrap_err(),
        TransitionError::InvalidToken
    );
    let (_, mut uc) = p.apply_structural(c).unwrap();
    let applied = p.state().clone();
    let mut rc = p.undo_structural(&mut uc).unwrap();
    let mut rb = p.undo_structural(&mut ub).unwrap();
    let mut ra = p.undo_structural(&mut ua).unwrap();
    assert_eq!(p.state(), &initial);
    assert_eq!(
        p.undo_structural(&mut ua).unwrap_err(),
        TransitionError::InvalidToken
    );
    assert_eq!(
        p.redo_structural(&mut rc).unwrap_err(),
        TransitionError::InvalidToken
    );
    let (_, mut newa) = p.redo_structural(&mut ra).unwrap();
    let (_, mut newb) = p.redo_structural(&mut rb).unwrap();
    let (_, mut newc) = p.redo_structural(&mut rc).unwrap();
    assert_eq!(p.state(), &applied);
    assert_eq!(
        p.redo_structural(&mut rc).unwrap_err(),
        TransitionError::InvalidToken
    );
    assert_eq!(
        p.undo_structural(&mut uc).unwrap_err(),
        TransitionError::InvalidToken
    );
    p.undo_structural(&mut newc).unwrap();
    p.undo_structural(&mut newb).unwrap();
    let mut stale = p.undo_structural(&mut newa).unwrap();
    assert_eq!(p.state(), &initial);
    let (_, mut branch) = p.apply_structural(a).unwrap();
    p.undo_structural(&mut branch).unwrap();
    assert_eq!(p.state(), &initial);
    assert_eq!(
        p.redo_structural(&mut stale).unwrap_err(),
        TransitionError::InvalidToken
    );
    assert!(p.revision >= 14);
}
#[test]
fn explicit_capacity_boundaries_preserve_state_and_tokens() {
    let mut p = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("1a", Side::Gote, Face::Phoenix),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        u64::MAX - 1,
    );
    let (_, mut u) = p
        .apply_structural(move_to("7g", "7f", Face::Phoenix))
        .unwrap();
    assert_eq!(p.declared_total_plies(), u64::MAX);
    let state = p.state().clone();
    let rt = runtime(&p);
    assert_eq!(
        p.apply_structural(move_to("1a", "1b", Face::Phoenix))
            .unwrap_err(),
        TransitionError::Capacity
    );
    assert_eq!(p.state(), &state);
    assert_eq!(runtime(&p), rt);
    let mut r = p.undo_structural(&mut u).unwrap();
    p.redo_structural(&mut r).unwrap();
    assert_eq!(p.state(), &state);
    for which in 0..4 {
        let mut q = Position::standard();
        match which {
            0 => q.state.accepted_plies = u64::MAX,
            1 => q.next_node = u64::MAX,
            2 => q.branch = u64::MAX,
            _ => q.revision = u64::MAX,
        };
        let before = q.state().clone();
        let rt = runtime(&q);
        assert_eq!(
            q.apply_structural(move_to("1e", "1d", Face::Swallow))
                .unwrap_err(),
            TransitionError::Capacity
        );
        assert_eq!(q.state(), &before);
        assert_eq!(runtime(&q), rt);
    }
    let mut p = Position::standard();
    let (_, mut u) = p
        .apply_structural(move_to("1e", "1d", Face::Swallow))
        .unwrap();
    p.revision = u64::MAX;
    let before = p.state().clone();
    assert_eq!(
        p.undo_structural(&mut u).unwrap_err(),
        TransitionError::Capacity
    );
    assert_eq!(p.state(), &before);
}
#[test]
fn deterministic_generated_structural_sequences_restore_all_native_state() {
    fn run() -> (Vec<Action>, StructuralState) {
        let mut p = Position::standard();
        let initial = p.state().clone();
        let material = resources(&p);
        let mut seed = 0x544f52495f313237u64;
        let mut tokens = Vec::new();
        let mut actions = Vec::new();
        let mut snapshots = vec![initial.clone()];
        for _ in 0..512 {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            let choices = all_structural_candidates(&p);
            if choices.is_empty() {
                break;
            }
            let a = choices[(seed as usize) % choices.len()];
            let (effect, u) = p.apply_structural(a).unwrap();
            assert_eq!(effect.actor, p.side_to_move().opponent());
            properties::preserves_material(&material, &resources(&p)).unwrap();
            tokens.push(u);
            actions.push(a);
            snapshots.push(p.state().clone());
        }
        assert_eq!(actions.len(), 512);
        let final_state = p.state().clone();
        let mut redo = Vec::new();
        for (i, u) in tokens.iter_mut().enumerate().rev() {
            redo.push(p.undo_structural(u).unwrap());
            assert_eq!(p.state(), &snapshots[i]);
        }
        assert_eq!(p.state(), &initial);
        for (i, r) in redo.iter_mut().rev().enumerate() {
            let (e, _) = p.redo_structural(r).unwrap();
            assert_eq!(e.action, actions[i]);
            assert_eq!(p.state(), &snapshots[i + 1]);
        }
        assert_eq!(p.state(), &final_state);
        (actions, final_state)
    }
    assert_eq!(run(), run());
}
#[test]
fn incomplete_legality_boundary_is_explicit_not_silently_partial() {
    // Structural layer intentionally does not implement128's root or safety/drop rules.
    let c = Corpus::embedded().unwrap();
    for id in [
        "TORI-REJECT-ADJACENT-ROYALS",
        "TORI-REJECT-DEAD-SWALLOW",
        "TORI-REJECT-THREE-SWALLOWS",
    ] {
        assert!(root(&c.fixtures().iter().find(|f| f.id == id).unwrap().given["root"]).is_ok());
    }
    for id in [
        "TORI-S-THREE",
        "TORI-S-LAST",
        "TORI-S-DROP-MATE",
        "TORI-CHECK-PIN",
        "TORI-O-ROYAL",
    ] {
        let f = c.fixtures().iter().find(|f| f.id == id).unwrap();
        let mut p = root(&f.given["root"]).unwrap();
        assert!(
            p.apply_structural(action(&f.given["command"])).is_ok(),
            "{id}"
        );
    }
}
