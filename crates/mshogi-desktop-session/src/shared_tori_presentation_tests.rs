//! Presentation expectations independent of opaque action strings and layout.
use super::*;
use crate::tori_fixture_roots as roots;
#[test]
fn all_native_faces_handed_hands_forced_choices_and_orientation_are_explicit() {
    let mut c = controller();
    replace(&mut c, &roots::contract_game(), t::Side::Sente);
    let snapshot = c.shared_snapshot();
    let mut faces = snapshot
        .pieces
        .iter()
        .map(|p| p.kind.as_str())
        .collect::<Vec<_>>();
    faces.sort();
    faces.dedup();
    assert_eq!(
        faces,
        vec!["Cr", "Ea", "Fa", "Go", "LQ", "Ph", "Pt", "RQ", "Sw"]
    );
    assert!(
        snapshot
            .pieces
            .iter()
            .find(|p| p.kind == "Ea")
            .unwrap()
            .promoted
    );
    assert!(
        snapshot
            .pieces
            .iter()
            .find(|p| p.kind == "LQ")
            .unwrap()
            .label
            .contains("Left Quail")
    );
    assert!(
        snapshot
            .hands
            .iter()
            .any(|h| h.owner.ends_with(":gote") && h.kind == "LQ" && h.count == 1)
    );
    let promotion = snapshot
        .choices
        .iter()
        .find(|a| a.from == "7c" && a.steps == ["7b"])
        .unwrap();
    assert_eq!(
        promotion.completion.as_ref().unwrap().label,
        "Compulsory promotion to Eagle (promoted Falcon)"
    );
    assert!(
        snapshot
            .choices
            .iter()
            .filter(|a| a.from == "6g")
            .all(|a| a.completion.is_none())
    );
    assert!(
        !snapshot
            .choices
            .iter()
            .any(|a| a.from == "7c" && a.steps == ["7b"] && a.completion.is_none())
    );
    let raw = native(&c);
    let flipped = send(
        &mut c,
        SessionOperation::ToriPreferences {
            settings: crate::tori_document::ToriSettings {
                orientation: t::Side::Gote,
            },
        },
    )
    .unwrap();
    assert_eq!(native(&c), raw);
    assert_eq!(
        flipped.orientations[1].files,
        vec!["1", "2", "3", "4", "5", "6", "7"]
    );
    let metadata = flipped.tori.unwrap();
    assert_eq!(metadata.declared_root_plies, "31");
    assert!(metadata.earlier_history_unknown);
    assert!(!metadata.historical_reachability_proven);
}
#[test]
fn play_only_rows_preserve_terminal_prefix_and_selected_geometry_without_replay() {
    let mut c = controller();
    replace(&mut c, &roots::contract_game(), t::Side::Sente);
    let played = send(
        &mut c,
        SessionOperation::Tori {
            actor: t::Side::Sente,
            command: t::GameCommand::Play {
                action: t::Action::Move {
                    from: "7c".parse().unwrap(),
                    to: "7b".parse().unwrap(),
                    result_face: t::Face::Eagle,
                },
            },
        },
    )
    .unwrap();
    assert!(
        played
            .pieces
            .iter()
            .any(|p| p.square == "7b" && p.kind == "Ea" && p.promoted)
    );
    assert_eq!(
        played
            .hands
            .iter()
            .find(|h| h.owner.ends_with(":sente") && h.kind == "Sw")
            .unwrap()
            .count,
        3
    );
    let stopped = send(
        &mut c,
        SessionOperation::Tori {
            actor: t::Side::Sente,
            command: t::GameCommand::Stop {
                reason: t::StopReason::Cancelled,
            },
        },
    )
    .unwrap();
    assert_eq!(stopped.moves.entries.len(), 1);
    assert_eq!(stopped.board_ply, 1);
    assert_eq!(stopped.event_index, 2);
    assert!(!stopped.status.concluded);
    assert!(stopped.status.score_half_points.is_none());
    let ended = send(&mut c, SessionOperation::Resign {}).unwrap();
    assert_eq!(ended.moves.entries.len(), 1);
    assert!(ended.status.label.contains("resignation"));
    assert_eq!(ended.moves.last_action.as_ref().unwrap().event_index, 1);
    assert!(
        matches!(&ended.moves.last_action.as_ref().unwrap().origin,ChoiceOrigin::Board{square}if square=="7c")
    );
    assert_eq!(ended.moves.last_action.as_ref().unwrap().steps, ["7b"]);
    let historical = send(&mut c, SessionOperation::Inspect { event_index: 1 }).unwrap();
    let meta = historical.tori.unwrap();
    assert!(meta.selected.conclusion.is_none());
    assert!(meta.live.conclusion.is_some());
    assert_eq!(meta.live.event_index, 3);
    assert_eq!(meta.selected.event_index, 1);
    assert!(historical.choices.is_empty());
    let root = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
    assert!(root.moves.last_action.is_none());
}
#[test]
fn both_quail_owners_keep_native_hand_identity_and_drop_destination_only() {
    for (owner, face, to) in [
        (t::Side::Sente, t::Face::RightQuail, "5e"),
        (t::Side::Gote, t::Face::LeftQuail, "5e"),
    ] {
        let mut g = roots::contract_game();
        if owner == t::Side::Gote {
            play(
                &mut g,
                t::Action::Move {
                    from: "4g".parse().unwrap(),
                    to: "5g".parse().unwrap(),
                    result_face: t::Face::Phoenix,
                },
            );
        }
        let mut c = controller();
        replace(&mut c, &g, owner);
        let before = exact(&c);
        let origin = hand_origin(owner, face);
        let inspected = c
            .shared_inspection(MovementInspectionQuery {
                expected: c.shared_token(),
                origin: origin.clone(),
            })
            .unwrap();
        assert!(inspected.read_only);
        assert_eq!(exact(&c), before);
        assert!(inspected.label.contains(if face == t::Face::LeftQuail {
            "Left Quail"
        } else {
            "Right Quail"
        }));
        let snapshot = send(
            &mut c,
            SessionOperation::Tori {
                actor: owner,
                command: t::GameCommand::Play {
                    action: t::Action::Drop {
                        face,
                        to: to.parse().unwrap(),
                    },
                },
            },
        )
        .unwrap();
        let cue = snapshot.moves.last_action.unwrap();
        assert_eq!(cue.steps, [to]);
        assert!(matches!(cue.origin,ChoiceOrigin::Hand{piece_kind,..}if piece_kind==face.code()));
        assert!(cue.effects.is_empty());
        assert!(
            snapshot
                .moves
                .entries
                .last()
                .unwrap()
                .notation
                .starts_with(face.code())
        );
    }
}
#[test]
fn exact_prefix_repetition_obligations_and_forfeit_are_native_not_inferred() {
    let mut c = controller();
    replace(&mut c, &sequence(&given("TORI-H-COUNT")), t::Side::Sente);
    let live = c.shared_snapshot();
    assert_eq!(live.tori.as_ref().unwrap().selected.occurrences, [0, 4, 8]);
    assert!(live.tori.as_ref().unwrap().selected.obligation.is_some());
    assert!(!live.status.concluded);
    let before = c.shared_token();
    let old = send(&mut c, SessionOperation::Inspect { event_index: 4 }).unwrap();
    assert_eq!(
        old.pieces
            .iter()
            .map(|p| (&p.square, &p.kind))
            .collect::<Vec<_>>(),
        live.pieces
            .iter()
            .map(|p| (&p.square, &p.kind))
            .collect::<Vec<_>>()
    );
    assert!(old.tori.unwrap().selected.obligation.is_none());
    assert!(
        c.shared_inspection(MovementInspectionQuery {
            expected: before,
            origin: "4g".into()
        })
        .is_err()
    );
    assert_eq!(old.moves.last_action.unwrap().event_index, 4);
    replace(
        &mut c,
        &sequence(&given("TORI-H-FORCED-FULL")),
        t::Side::Sente,
    );
    let s = c.shared_snapshot();
    assert!(s.status.label.contains("repetition forfeit"));
    // TORI-H-FORCED-FULL explicitly expects in_check_at8=true: the Gote
    // Eagle on4d attacks4a along its backward ray. This is a repetition
    // forfeit despite a locally legal escape, not a checkmate.
    assert!(s.tori.unwrap().selected.in_check);
    assert_eq!(s.moves.entries.len(), 8);
}
#[test]
fn large_declared_counter_is_decimal_metadata_not_lossy_frontend_number() {
    let board = [("7g", t::Side::Sente), ("1a", t::Side::Gote)].map(|(sq, side)| {
        (
            sq.parse().unwrap(),
            t::Piece {
                side,
                face: t::Face::Phoenix,
            },
        )
    });
    let g = t::Game::from_root(
        t::Position::from_diagram(
            &board,
            [t::Hand::empty(); 2],
            t::Side::Sente,
            t::RootMetadata::diagram(u64::MAX),
        )
        .unwrap(),
    )
    .unwrap();
    let mut c = controller();
    replace(&mut c, &g, t::Side::Sente);
    let s = c.shared_snapshot();
    assert_eq!(s.tori.unwrap().declared_root_plies, "18446744073709551615");
    assert!(s.moves.entries.is_empty());
    assert_eq!(s.board_ply, 0);
}

#[test]
fn nonboard_events_do_not_advance_pair_numbers_and_gote_can_be_first_known_actor() {
    let mut g = roots::contract_game();
    play(
        &mut g,
        t::Action::Move {
            from: "4g".parse().unwrap(),
            to: "5g".parse().unwrap(),
            result_face: t::Face::Phoenix,
        },
    );
    g.submit(t::GameRequest {
        expected: g.context(),
        actor: t::Side::Gote,
        command: t::GameCommand::Stop {
            reason: t::StopReason::Cancelled,
        },
    })
    .unwrap();
    play(
        &mut g,
        t::Action::Move {
            from: "4a".parse().unwrap(),
            to: "5a".parse().unwrap(),
            result_face: t::Face::Phoenix,
        },
    );
    let view = crate::moves::tori_projection(&g);
    assert_eq!(
        view.entries
            .iter()
            .map(|m| (m.event_index, m.board_ply, m.number, m.column))
            .collect::<Vec<_>>(),
        [(1, 1, 1, 0), (3, 2, 1, 1)]
    );
    let board = [("7g", t::Side::Sente), ("1a", t::Side::Gote)].map(|(sq, side)| {
        (
            sq.parse().unwrap(),
            t::Piece {
                side,
                face: t::Face::Phoenix,
            },
        )
    });
    let mut g = t::Game::from_root(
        t::Position::from_diagram(
            &board,
            [t::Hand::empty(); 2],
            t::Side::Gote,
            t::RootMetadata::diagram(37),
        )
        .unwrap(),
    )
    .unwrap();
    play(
        &mut g,
        t::Action::Move {
            from: "1a".parse().unwrap(),
            to: "2a".parse().unwrap(),
            result_face: t::Face::Phoenix,
        },
    );
    let view = crate::moves::tori_projection(&g);
    assert_eq!(
        (
            view.entries[0].number,
            view.entries[0].column,
            view.entries[0].event_index
        ),
        (1, 1, 1)
    );
}
