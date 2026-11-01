use super::{
    conformance::{self, action, root, sq},
    local_conformance::runtime,
    *,
};
use mshogi_conformance::{Corpus, properties};
use serde_json::{Value, json};
use std::collections::HashSet;

fn diagram(entries: &[(&str, Side, Face)], hands: [Hand; 2], turn: Side, counter: u64) -> Position {
    Position::from_diagram(
        &entries
            .iter()
            .map(|&(s, side, face)| (sq(s), Piece { side, face }))
            .collect::<Vec<_>>(),
        hands,
        turn,
        RootMetadata::diagram(counter),
    )
    .unwrap()
}
fn m(from: &str, to: &str, result_face: Face) -> Action {
    Action::Move {
        from: sq(from),
        to: sq(to),
        result_face,
    }
}
fn d(face: Face, to: &str) -> Action {
    Action::Drop { face, to: sq(to) }
}
fn fixture(id: &str) -> mshogi_conformance::Fixture {
    Corpus::embedded()
        .unwrap()
        .fixtures()
        .iter()
        .find(|f| f.id == id)
        .unwrap()
        .clone()
}
fn local_fixture(id: &str) -> LocalPosition {
    LocalPosition::admit(root(&fixture(id).given["root"]).unwrap()).unwrap()
}
fn rotate(p: &LocalPosition) -> LocalPosition {
    let p = p.position();
    let entries: Vec<_> = Square::all()
        .filter_map(|s| {
            p.piece(s).map(|v| {
                (
                    s.rotated(),
                    Piece {
                        side: v.side.opponent(),
                        face: v.face,
                    },
                )
            })
        })
        .collect();
    let mut q = Position::from_diagram(
        &entries,
        [p.hand(Side::Gote), p.hand(Side::Sente)],
        p.side_to_move().opponent(),
        RootMetadata::diagram(p.root().declared_completed_plies()),
    )
    .unwrap();
    q.state.accepted_plies = p.accepted_plies();
    LocalPosition::admit(q).unwrap()
}
fn ra(a: Action) -> Action {
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
fn set(a: impl IntoIterator<Item = Action>) -> HashSet<Action> {
    a.into_iter().collect()
}
fn exact(p: &LocalPosition, expected: &[Action]) {
    let listed = p.legal_actions();
    assert_eq!(listed.len(), set(listed.iter().copied()).len());
    assert_eq!(set(listed.iter().copied()), set(expected.iter().copied()));
    assert_eq!(listed, p.legal_actions());
    let r = rotate(p);
    assert_eq!(
        set(r.legal_actions()),
        set(expected.iter().copied().map(ra))
    );
}
fn material(p: &LocalPosition) -> Value {
    let p = p.position();
    json!(
        Face::BASES
            .into_iter()
            .map(|f| Square::all()
                .filter_map(|s| p.piece(s))
                .filter(|v| v.face.base() == f)
                .count()
                + [Side::Sente, Side::Gote]
                    .into_iter()
                    .map(|side| usize::from(p.hand(side).count(f)))
                    .sum::<usize>())
            .collect::<Vec<_>>()
    )
}
fn unchanged(p: &LocalPosition) -> (StructuralState, (u64, u64, u64, u64)) {
    (p.position().state().clone(), runtime(p.position()))
}
#[test]
fn independent_complete_choices_single_double_check_capture_and_block() {
    // Hand-enumerated before comparison: a corner Phoenix has exactly three exits,
    // and the opposite royal is six files/ranks away, so none can be attacked.
    let bare = LocalPosition::admit(diagram(
        &[
            ("1a", Side::Sente, Face::Phoenix),
            ("7g", Side::Gote, Face::Phoenix),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        0,
    ))
    .unwrap();
    exact(
        &bare,
        &[
            m("1a", "1b", Face::Phoenix),
            m("1a", "2a", Face::Phoenix),
            m("1a", "2b", Face::Phoenix),
        ],
    );
    let hands = [
        Hand::from_counts(&[(Face::Crane, 2)]).unwrap(),
        Hand::empty(),
    ];
    let entries = [
        ("4g", Side::Sente, Face::Phoenix),
        ("1a", Side::Gote, Face::Phoenix),
        ("4a", Side::Gote, Face::LeftQuail),
    ];
    let check = LocalPosition::admit(diagram(&entries, hands, Side::Sente, 0)).unwrap();
    let mut expected = vec![
        m("4g", "5f", Face::Phoenix),
        m("4g", "3f", Face::Phoenix),
        m("4g", "5g", Face::Phoenix),
        m("4g", "3g", Face::Phoenix),
    ];
    expected.extend(
        ["4b", "4c", "4d", "4e", "4f"]
            .into_iter()
            .map(|s| d(Face::Crane, s)),
    );
    exact(&check, &expected);
    assert_eq!(check.status(), LocalStatus::Check);
    let mut double = entries.to_vec();
    double.push(("2e", Side::Gote, Face::Goose));
    let double = LocalPosition::admit(diagram(&double, hands, Side::Sente, 0)).unwrap();
    assert_eq!(double.attackers(sq("4g"), Side::Gote).len(), 2);
    exact(&double, &expected[..4]); // Blocking the ray never stops the Goose jump.
    let pinned = local_fixture("TORI-CHECK-PIN");
    assert_eq!(
        pinned
            .inspect(InspectionOrigin::Board(sq("4f")))
            .into_iter()
            .map(|v| v.action)
            .collect::<Vec<_>>(),
        vec![m("4f", "4e", Face::Crane)]
    );
}
#[test]
fn pinned_attackers_still_defend_squares_and_discovered_self_check_rejects() {
    for owner in [Side::Sente, Side::Gote] {
        // Gote Crane4b is pinned to Phoenix4a by Sente LeftQuail4f. Its geometric
        // threat3c still forbids Sente Phoenix3d→3c under the selected T05 map.
        let mut p = LocalPosition::admit(diagram(
            &[
                ("3d", Side::Sente, Face::Phoenix),
                ("4a", Side::Gote, Face::Phoenix),
                ("4f", Side::Sente, Face::LeftQuail),
                ("4b", Side::Gote, Face::Crane),
            ],
            [Hand::empty(); 2],
            Side::Sente,
            0,
        ))
        .unwrap();
        let mut target = sq("3c");
        let mut pinned = sq("4b");
        let mut command = m("3d", "3c", Face::Phoenix);
        if owner == Side::Gote {
            p = rotate(&p);
            target = target.rotated();
            pinned = pinned.rotated();
            command = ra(command);
        }
        assert!(p.attackers(target, owner.opponent()).contains(&pinned));
        assert!(
            !p.inspect(InspectionOrigin::Board(pinned))
                .iter()
                .any(|v| matches!(v.action,Action::Move{to,..}if to==target))
        );
        let before = unchanged(&p);
        assert_eq!(p.apply(command).unwrap_err(), ActionError::SelfCheck);
        assert_eq!(unchanged(&p), before);
        let mut actual = local_fixture(if owner == Side::Sente {
            "TORI-CHECK-PIN"
        } else {
            "TORI-CHECK-PIN-ROTATED"
        });
        let f = fixture(if owner == Side::Sente {
            "TORI-CHECK-PIN"
        } else {
            "TORI-CHECK-PIN-ROTATED"
        });
        assert_eq!(
            actual.apply(action(&f.given["command"])).unwrap_err(),
            ActionError::SelfCheck
        );
    }
}
#[test]
fn source_derived_attack_tables_match_native_threats_for_all_faces_and_owners() {
    let c = Corpus::embedded().unwrap();
    for f in c
        .fixtures()
        .iter()
        .filter(|f| f.scope == "exhaustive_local_geometry")
    {
        let face = conformance::face(f.given["piece"].as_str().unwrap());
        for owner in [Side::Sente, Side::Gote] {
            let base = diagram(
                &[
                    ("7g", Side::Sente, Face::Phoenix),
                    ("1a", Side::Gote, Face::Phoenix),
                ],
                [Hand::empty(); 2],
                Side::Gote,
                0,
            );
            let mut p = base;
            if face == Face::Phoenix {
                p.state.board[sq("7g").index()] = None;
            }
            p.state.board[sq("4d").index()] = Some(Piece {
                side: Side::Sente,
                face,
            });
            let mut p = LocalPosition::admit(p).unwrap();
            if owner == Side::Gote {
                p = rotate(&p);
            }
            let mut expected = HashSet::new();
            // Only fixture endpoint expansion, not another rules/attack engine.
            let world = |x: i64, y: i64| -> Option<Square> {
                let (file, rank) = if owner == Side::Sente {
                    (4 - x, 4 - y)
                } else {
                    (4 + x, 4 + y)
                };
                Square::new(u8::try_from(file).ok()?, u8::try_from(rank).ok()?)
            };
            for field in ["complete_steps", "complete_jumps"] {
                for offset in f.expected[field].as_array().unwrap() {
                    if let Some(s) = world(offset[0].as_i64().unwrap(), offset[1].as_i64().unwrap())
                    {
                        expected.insert(s);
                    }
                }
            }
            for offset in f.expected["complete_rays"].as_array().unwrap() {
                for n in 1..=6 {
                    if let Some(s) = world(
                        offset[0].as_i64().unwrap() * n,
                        offset[1].as_i64().unwrap() * n,
                    ) {
                        expected.insert(s);
                    }
                }
            }
            for ray in f.expected["complete_limited_rays"].as_array().unwrap() {
                for n in ray["lengths"].as_array().unwrap() {
                    if let Some(s) = world(
                        ray["direction"][0].as_i64().unwrap() * n.as_i64().unwrap(),
                        ray["direction"][1].as_i64().unwrap() * n.as_i64().unwrap(),
                    ) {
                        expected.insert(s);
                    }
                }
            }
            let actual: HashSet<_> = Square::all()
                .filter(|s| p.attackers(*s, owner).contains(&sq("4d")))
                .collect();
            assert_eq!(actual, expected, "{} {owner:?}", f.id);
        }
    }
    // The admitted corpus separately establishes blocked rays vs ignored jump
    // intermediates; all of those execute through the local conformance adapter.
}
fn swallow_root(pheasant: bool, defending_crane: bool, defender_hand: bool) -> LocalPosition {
    let mut entries = vec![
        ("7g", Side::Sente, Face::Phoenix),
        ("4a", Side::Gote, Face::Phoenix),
        ("3a", Side::Gote, Face::Swallow),
        ("5a", Side::Gote, Face::Swallow),
        ("3b", Side::Gote, Face::Swallow),
        ("5b", Side::Gote, Face::Swallow),
    ];
    if pheasant {
        entries.push(("4d", Side::Sente, Face::Pheasant));
    }
    if defending_crane {
        entries.push(("3c", Side::Gote, Face::Crane));
    }
    let hand = if defender_hand {
        Hand::from_counts(&Face::HANDS.map(|f| (f, 1))).unwrap()
    } else {
        Hand::empty()
    };
    LocalPosition::admit(diagram(
        &entries,
        [
            Hand::from_counts(&[(Face::Swallow, 1), (Face::Falcon, 1)]).unwrap(),
            hand,
        ],
        Side::Sente,
        0,
    ))
    .unwrap()
}
#[test]
fn swallow_drop_mate_uses_all_safe_capture_escape_and_drop_defenses_without_recursion() {
    for owner in [Side::Sente, Side::Gote] {
        let transform = |a| if owner == Side::Sente { a } else { ra(a) };
        // Four occupied king exits, protected checker: even a full defender hand
        // cannot interpose on this adjacent check. These drops MUST reject.
        for hands in [false, true] {
            let mut p = swallow_root(true, false, hands);
            if owner == Side::Gote {
                p = rotate(&p);
            }
            let original = unchanged(&p);
            assert_eq!(
                p.apply(transform(d(Face::Swallow, "4b"))).unwrap_err(),
                ActionError::SwallowDropMate
            );
            assert_eq!(unchanged(&p), original);
        }
        // Remove the Pheasant protection: royal capture is the sole safe reply.
        let mut p = swallow_root(false, false, true);
        if owner == Side::Gote {
            p = rotate(&p);
        }
        p.apply(transform(d(Face::Swallow, "4b"))).unwrap();
        exact(&p, &[transform(m("4a", "4b", Face::Phoenix))]);
        assert_eq!(p.status(), LocalStatus::Check);
        // Restore protection but add a Crane that can capture the checking drop.
        let mut p = swallow_root(true, true, true);
        if owner == Side::Gote {
            p = rotate(&p);
        }
        p.apply(transform(d(Face::Swallow, "4b"))).unwrap();
        exact(&p, &[transform(m("3c", "4b", Face::Crane))]);
        // The independently reviewed escape fixture has exactly one king escape.
        let mut p = local_fixture(if owner == Side::Sente {
            "TORI-D-LEGAL-DEFENSE"
        } else {
            "TORI-D-LEGAL-DEFENSE-ROTATED"
        });
        p.apply(transform(d(Face::Swallow, "4b"))).unwrap();
        exact(&p, &[transform(m("4a", "5b", Face::Phoenix))]);
        // A Falcon drop gives mate in the protected-box position and is allowed;
        // unlike a Swallow, it has no immediate-mate drop prohibition or timer.
        let mut p = swallow_root(true, false, false);
        if owner == Side::Gote {
            p = rotate(&p);
        }
        p.apply(transform(d(Face::Falcon, "4b"))).unwrap();
        assert_eq!(p.status(), LocalStatus::Checkmate);
        let target = if owner == Side::Sente {
            sq("4b")
        } else {
            sq("4b").rotated()
        };
        assert_eq!(p.position().piece(target).unwrap().face, Face::Falcon);
    }
}
#[test]
fn local_root_checks_preserve_custom_provenance_and_allow_moving_side_check() {
    let c = Corpus::embedded().unwrap();
    for (id, error) in [
        (
            "TORI-REJECT-ADJACENT-ROYALS",
            AdmissionError::AdjacentRoyals,
        ),
        ("TORI-REJECT-DEAD-SWALLOW", AdmissionError::StrandedSwallow),
        ("TORI-REJECT-THREE-SWALLOWS", AdmissionError::SwallowFile),
    ] {
        let f = c.fixtures().iter().find(|f| f.id == id).unwrap();
        let p = root(&f.given["root"]).unwrap();
        assert_eq!(LocalPosition::admit(p).unwrap_err(), error);
    }
    for face in Face::ALL.into_iter().filter(|f| *f != Face::Phoenix) {
        let target = match face {
            Face::Pheasant => "4b",
            Face::Goose => "2b",
            _ => "4c",
        };
        let entries = [
            ("7g", Side::Sente, Face::Phoenix),
            (target, Side::Gote, Face::Phoenix),
            ("4d", Side::Sente, face),
        ];
        assert_eq!(
            LocalPosition::admit(diagram(&entries, [Hand::empty(); 2], Side::Sente, 0))
                .unwrap_err(),
            AdmissionError::NonmovingRoyalAttacked
        );
        let p =
            LocalPosition::admit(diagram(&entries, [Hand::empty(); 2], Side::Gote, 123)).unwrap();
        assert!(p.is_in_check(Side::Gote));
        assert!(p.position().root().earlier_history_unknown());
        assert!(!p.position().root().historical_reachability_proven());
        assert_eq!(p.position().root().declared_completed_plies(), 123);
    }
    let p = local_fixture("TORI-S-GOOSE");
    assert!(p.is_legal(d(Face::Swallow, "4d")));
    let p = local_fixture("TORI-S-ENEMY");
    assert!(p.is_legal(d(Face::Swallow, "4d")));
    let p = local_fixture("TORI-S-TWO");
    assert!(p.is_legal(d(Face::Swallow, "4d")));
    let mut p = local_fixture("TORI-S-THREE");
    assert_eq!(
        p.apply(d(Face::Swallow, "4d")).unwrap_err(),
        ActionError::SwallowFile
    );
    let p = LocalPosition::standard();
    assert!(!p.position().root().earlier_history_unknown());
    assert_eq!(p.position().accepted_plies(), 0);
    // Zone-dropped Falcon and Swallow are valid root faces, with no inferred past.
    for face in [Face::Falcon, Face::Swallow] {
        LocalPosition::admit(diagram(
            &[
                ("7g", Side::Sente, Face::Phoenix),
                ("1a", Side::Gote, Face::Phoenix),
                ("4b", Side::Sente, face),
            ],
            [Hand::empty(); 2],
            Side::Sente,
            55,
        ))
        .unwrap();
    }
}
#[test]
fn terminal_root_observations_are_native_not_match_conclusions() {
    for (id, status) in [
        ("TORI-O-MATE", LocalStatus::Checkmate),
        ("TORI-O-QUIET", LocalStatus::QuietNoAction),
    ] {
        let p = local_fixture(id);
        assert_eq!(p.status(), status);
        assert!(p.legal_actions().is_empty());
        assert_eq!(p.position().accepted_plies(), 0);
        assert_eq!(rotate(&p).status(), status);
    }
    assert_eq!(LocalPosition::standard().status(), LocalStatus::Ongoing);
    assert_eq!(
        local_fixture("TORI-CHECK-EVASION").status(),
        LocalStatus::Check
    );
}
#[test]
fn inspection_is_owner_specific_immutable_complete_and_not_live_permission() {
    let mut p = local_fixture("TORI-CHECK-EVASION");
    let e = p.apply(m("4g", "5g", Face::Phoenix)).unwrap();
    let mut undo = e.1;
    let before = unchanged(&p);
    for from in Square::all() {
        let choices = p.inspect(InspectionOrigin::Board(from));
        for choice in &choices {
            let actor = p.position().piece(from).unwrap().side;
            assert_eq!(choice.effects.actor, actor);
            assert!(matches!(choice.action,Action::Move{from:s,..} if s==from));
        }
        if p.position()
            .piece(from)
            .is_some_and(|v| v.side == p.position().side_to_move())
        {
            assert_eq!(
                set(choices.into_iter().map(|v| v.action)),
                set(p
                    .legal_actions()
                    .into_iter()
                    .filter(|a| matches!(a,Action::Move{from:s,..}if *s==from)))
            );
        }
    }
    for side in [Side::Sente, Side::Gote] {
        for face in Face::ALL {
            p.inspect(InspectionOrigin::Hand { side, face });
        }
    }
    assert_eq!(unchanged(&p), before);
    p.undo(&mut undo).unwrap(); // Queries did not retire this real token.
    let p = local_fixture("TORI-CHECK-EVASION");
    let original = unchanged(&p);
    let expected: [Action; 5] = ["4b", "4c", "4d", "4e", "4f"].map(|s| m("4a", s, Face::LeftQuail));
    assert_eq!(
        set(p
            .inspect(InspectionOrigin::Board(sq("4a")))
            .into_iter()
            .map(|v| v.action)),
        set(expected)
    );
    assert_eq!(unchanged(&p), original);
    let mut p = LocalPosition::admit(diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("1a", Side::Gote, Face::Phoenix),
        ],
        [
            Hand::empty(),
            Hand::from_counts(&[(Face::LeftQuail, 2), (Face::Swallow, 1)]).unwrap(),
        ],
        Side::Sente,
        88,
    ))
    .unwrap();
    let before = unchanged(&p);
    let inspected = p.inspect(InspectionOrigin::Hand {
        side: Side::Gote,
        face: Face::LeftQuail,
    });
    assert!(!inspected.is_empty());
    assert_eq!(
        inspected.len(),
        set(inspected.iter().map(|v| v.action)).len()
    );
    assert!(inspected.iter().all(
        |v| v.effects.actor == Side::Gote && v.effects.resulting_piece.face == Face::LeftQuail
    ));
    assert_eq!(
        p.apply(inspected[0].action).unwrap_err(),
        ActionError::MissingHand
    );
    assert_eq!(unchanged(&p), before);
    // On an actual terminal/check position, an advisory nonchecking Swallow drop
    // for the other owner is not incorrectly banned for an unrelated existing mate.
    let f = fixture("TORI-O-MATE");
    let mut root = root(&f.given["root"]).unwrap();
    root.state.hands[Side::Sente.index()] = Hand::from_counts(&[(Face::Swallow, 1)]).unwrap();
    let p = LocalPosition::admit(root).unwrap();
    assert!(
        p.inspect(InspectionOrigin::Hand {
            side: Side::Sente,
            face: Face::Swallow
        })
        .iter()
        .any(|c| c.action == d(Face::Swallow, "6e"))
    );
}
#[test]
fn exact_legal_apply_undo_redo_material_and_atomic_capacity() {
    let c = Corpus::embedded().unwrap();
    let mut count = 0;
    for f in c.fixtures().iter().filter(|f| {
        f.expected.get("accepted") == Some(&json!(true))
            && matches!(f.given["command"]["kind"].as_str(), Some("move" | "drop"))
    }) {
        let mut p = LocalPosition::admit(root(&f.given["root"]).unwrap()).unwrap();
        let a = action(&f.given["command"]);
        let before = p.position().state().clone();
        let initial = material(&p);
        assert!(p.legal_actions().contains(&a));
        let (effects, mut undo) = p.apply(a).unwrap();
        let applied = p.position().state().clone();
        properties::preserves_material(&initial, &material(&p)).unwrap();
        let mut redo = p.undo(&mut undo).unwrap();
        assert_eq!(p.position().state(), &before);
        let (repeated, _) = p.redo(&mut redo).unwrap();
        assert_eq!(effects, repeated);
        assert_eq!(p.position().state(), &applied);
        let mut copy = LocalPosition::admit(root(&f.given["root"]).unwrap()).unwrap();
        properties::round_trip(
            &mut copy,
            |p| p.position().state().clone(),
            |p| p.apply(a).map(|(_, u)| u).map_err(|e| e.to_string()),
            |p, mut u| p.undo(&mut u).map(|_| ()).map_err(|e| e.to_string()),
        )
        .unwrap();
        count += 1;
    }
    assert_eq!(count, 40);
    let mut p = LocalPosition::admit(diagram(
        &[
            ("1a", Side::Sente, Face::Phoenix),
            ("7g", Side::Gote, Face::Phoenix),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        u64::MAX,
    ))
    .unwrap();
    let before = unchanged(&p);
    let choices = p.legal_actions();
    assert_eq!(choices.len(), 3);
    assert_eq!(p.status(), LocalStatus::Ongoing);
    assert!(p.is_legal(choices[0]));
    assert_eq!(p.apply(choices[0]).unwrap_err(), ActionError::Capacity);
    assert_eq!(unchanged(&p), before);
    assert_eq!(p.perft(2, 13).unwrap().leaves, 9);
    // A capacity-neutral board-defense probe must not mistake a full declared
    // counter on the accepted post-state for the absence of a legal defense.
    let f = fixture("TORI-D-LEGAL-DEFENSE");
    let mut r = root(&f.given["root"]).unwrap();
    r.state.root.declared_completed_plies = u64::MAX - 1;
    let mut p = LocalPosition::admit(r).unwrap();
    p.apply(action(&f.given["command"])).unwrap();
    assert_eq!(p.position().declared_total_plies(), u64::MAX);
    assert_eq!(p.status(), LocalStatus::Check);
    assert_eq!(p.legal_actions(), vec![m("4a", "5b", Face::Phoenix)]);
}
#[test]
fn complete_generator_membership_and_rotated_effects_properties() {
    for id in ["TORI-D-LEGAL-DEFENSE", "TORI-CHECK-PIN", "TORI-S-THREE"] {
        let p = local_fixture(id);
        let r = rotate(&p);
        assert_eq!(
            set(r.legal_actions()),
            set(p.legal_actions().into_iter().map(ra))
        );
        for position in [&p, &r] {
            let listed = set(position.legal_actions());
            for from in Square::all() {
                for to in Square::all() {
                    for result_face in Face::ALL {
                        let a = Action::Move {
                            from,
                            to,
                            result_face,
                        };
                        assert_eq!(listed.contains(&a), position.is_legal(a));
                    }
                }
            }
            for to in Square::all() {
                for face in Face::ALL {
                    let a = Action::Drop { face, to };
                    assert_eq!(listed.contains(&a), position.is_legal(a));
                }
            }
        }
        for a in p.legal_actions() {
            let mut q = p.clone();
            let mut qr = r.clone();
            let (e, _) = q.apply(a).unwrap();
            let (er, _) = qr.apply(ra(a)).unwrap();
            assert_eq!(e.actor.opponent(), er.actor);
            assert_eq!(e.resulting_piece.face, er.resulting_piece.face);
            assert_eq!(e.captured.map(|v| v.face), er.captured.map(|v| v.face));
            assert_eq!(set(rotate(&q).legal_actions()), set(qr.legal_actions()));
        }
    }
}
#[test]
fn deterministic_legal_sequences_restore_every_prefix_and_inspection_is_nonmutating() {
    fn run() -> (Vec<Action>, StructuralState) {
        let mut p = LocalPosition::standard();
        let initial = p.position().state().clone();
        let mut snapshots = vec![initial.clone()];
        let mut tokens = Vec::new();
        let mut moves = Vec::new();
        let material_before = material(&p);
        let mut seed = 0x544f5249313238u64;
        for _ in 0..192 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let choices = p.legal_actions();
            if choices.is_empty() {
                break;
            }
            let a = choices[(seed as usize) % choices.len()];
            let before = unchanged(&p);
            if let Action::Move { from, .. } = a {
                p.inspect(InspectionOrigin::Board(from));
            }
            assert_eq!(unchanged(&p), before);
            let (_, u) = p.apply(a).unwrap();
            assert!(!p.is_in_check(p.position().side_to_move().opponent()));
            properties::preserves_material(&material_before, &material(&p)).unwrap();
            tokens.push(u);
            moves.push(a);
            snapshots.push(p.position().state().clone());
        }
        assert!(moves.len() >= 32);
        eprintln!(
            "TORI-128 deterministic legal sequence: {} accepted actions",
            moves.len()
        );
        let final_state = p.position().state().clone();
        let mut redos = Vec::new();
        for (i, u) in tokens.iter_mut().enumerate().rev() {
            redos.push(p.undo(u).unwrap());
            assert_eq!(p.position().state(), &snapshots[i]);
        }
        assert_eq!(p.position().state(), &initial);
        for (i, r) in redos.iter_mut().rev().enumerate() {
            let (e, _) = p.redo(r).unwrap();
            assert_eq!(e.action, moves[i]);
            assert_eq!(p.position().state(), &snapshots[i + 1]);
        }
        assert_eq!(p.position().state(), &final_state);
        (moves, final_state)
    }
    assert_eq!(run(), run());
}
#[test]
fn bounded_perft_and_divide_use_independent_counts_and_preserve_native_paths() {
    let p = LocalPosition::admit(diagram(
        &[
            ("1a", Side::Sente, Face::Phoenix),
            ("7g", Side::Gote, Face::Phoenix),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        0,
    ))
    .unwrap();
    let before = unchanged(&p);
    // Each distant corner king has3moves. No first move can influence the other,
    // so depth2=3×3=9, visited=1+3+9=13; not measured generator expectations.
    assert_eq!(
        p.perft(0, 1).unwrap(),
        PerftReport {
            depth: 0,
            leaves: 1,
            visited: 1
        }
    );
    assert_eq!(p.perft(1, 4).unwrap().leaves, 3);
    assert_eq!(
        p.perft(2, 13).unwrap(),
        PerftReport {
            depth: 2,
            leaves: 9,
            visited: 13
        }
    );
    let divide = p.divide(2, 13).unwrap();
    assert_eq!(divide.total, p.perft(2, 13).unwrap());
    assert_eq!(divide.entries.len(), 3);
    assert!(divide.entries.iter().all(|row| row.leaves == 3));
    assert_eq!(divide, p.divide(2, 13).unwrap());
    for error in [p.perft(2, 12).unwrap_err(), p.divide(2, 12).unwrap_err()] {
        let PerftError::WorkLimit { path } = error else {
            panic!("wrong failure")
        };
        assert_eq!(path.len(), 2);
        let mut probe = p.clone();
        for a in path {
            assert!(probe.is_legal(a));
            probe.apply(a).unwrap();
        }
    }
    for (depth, limit) in [(9, 100), (1, 0), (1, MAX_PERFT_VISITS + 1)] {
        assert_eq!(p.perft(depth, limit), Err(PerftError::Limits));
    }
    assert_eq!(p.divide(0, 10), Err(PerftError::Limits));
    for id in ["TORI-O-MATE", "TORI-O-QUIET"] {
        let end = local_fixture(id);
        assert_eq!(end.perft(0, 1).unwrap().leaves, 1);
        assert_eq!(end.perft(1, 1).unwrap().leaves, 0);
    }
    assert_eq!(unchanged(&p), before);
}

#[test]
fn only_swallow_has_last_rank_drop_ban_and_inspection_keeps_forced_faces() {
    for owner in [Side::Sente, Side::Gote] {
        let hand = Hand::from_counts(&Face::HANDS.map(|f| (f, 1))).unwrap();
        let mut p = LocalPosition::admit(diagram(
            &[
                ("7g", Side::Sente, Face::Phoenix),
                ("1a", Side::Gote, Face::Phoenix),
            ],
            [hand, Hand::empty()],
            Side::Sente,
            19,
        ))
        .unwrap();
        if owner == Side::Gote {
            p = rotate(&p);
        }
        let endpoint = if owner == Side::Sente { "4a" } else { "4g" };
        for face in Face::HANDS {
            let a = d(face, endpoint);
            let inspection = p.inspect(InspectionOrigin::Hand { side: owner, face });
            if face == Face::Swallow {
                assert!(!p.is_legal(a));
                assert!(!inspection.iter().any(|c| c.action == a));
                let mut copy = p.clone();
                assert_eq!(copy.apply(a).unwrap_err(), ActionError::SwallowLastRank);
            } else {
                assert!(p.is_legal(a));
                let observed = inspection.iter().find(|c| c.action == a).unwrap();
                assert_eq!(observed.effects.actor, owner);
                assert_eq!(observed.effects.resulting_piece.face, face);
                let mut copy = p.clone();
                let (effects, _) = copy.apply(a).unwrap();
                assert_eq!(effects, observed.effects);
            }
        }
        let p = local_fixture(if owner == Side::Sente {
            "TORI-P-DROP-EXIT"
        } else {
            "TORI-P-DROP-EXIT-ROTATED"
        });
        let from = if owner == Side::Sente {
            sq("4b")
        } else {
            sq("4f")
        };
        let observed = p.inspect(InspectionOrigin::Board(from));
        assert!(!observed.is_empty());
        assert!(
            observed
                .iter()
                .all(|c| c.effects.resulting_piece.face == Face::Eagle)
        );
    }
}
