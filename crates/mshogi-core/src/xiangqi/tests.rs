use super::*;

fn point(text: &str) -> Intersection {
    text.parse().unwrap()
}
fn sparse(side: Side, counters: Counters) -> Position {
    Position::from_diagram(
        &[
            (point("d0"), Piece::new(Side::Red, PieceKind::General)),
            (point("f9"), Piece::new(Side::Black, PieceKind::General)),
            (point("a1"), Piece::new(Side::Red, PieceKind::Rook)),
            (point("a2"), Piece::new(Side::Black, PieceKind::Horse)),
        ],
        side,
        counters,
    )
    .unwrap()
}

#[test]
fn intersections_native_palaces_river_and_both_forward_directions() {
    assert_eq!(Intersection::all().count(), 90);
    for p in Intersection::all() {
        assert_eq!(p.to_string().parse::<Intersection>(), Ok(p));
        assert_eq!(Intersection::new(p.file(), p.rank()), Some(p));
        assert!(p.index() < 90);
    }
    for bad in ["a", "j0", "a:", "a10", "A0", " a0", "é0", "９0", "a0\n"] {
        assert!(bad.parse::<Intersection>().is_err(), "{bad:?}");
    }
    assert!(Intersection::new(9, 0).is_none());
    assert!(Intersection::new(0, 10).is_none());
    assert_eq!(Side::Red.forward(), 1);
    assert_eq!(Side::Black.forward(), -1);
    assert_eq!(Side::Red.opponent(), Side::Black);
    assert_eq!(
        Intersection::all()
            .filter(|&p| Side::Red.in_palace(p))
            .count(),
        9
    );
    assert_eq!(
        Intersection::all()
            .filter(|&p| Side::Black.in_palace(p))
            .count(),
        9
    );
    for p in ["d0", "f0", "d2", "f2", "e1"] {
        assert!(Side::Red.in_palace(point(p)));
    }
    for p in ["c0", "g0", "e3", "e9"] {
        assert!(!Side::Red.in_palace(point(p)));
    }
    assert!(!Side::Red.has_crossed_river(point("a4")));
    assert!(Side::Red.has_crossed_river(point("a5")));
    assert!(!Side::Black.has_crossed_river(point("i5")));
    assert!(Side::Black.has_crossed_river(point("i4")));
}

#[test]
fn exact_initial_inventory_and_scoped_fen_roundtrip() {
    let position = Position::initial();
    assert_eq!(position.to_fen().unwrap(), INITIAL_FEN);
    assert_eq!(position.side_to_move(), Side::Red);
    assert_eq!(position.pieces().count(), 32);
    for side in [Side::Red, Side::Black] {
        for kind in [
            PieceKind::General,
            PieceKind::Advisor,
            PieceKind::Elephant,
            PieceKind::Horse,
            PieceKind::Rook,
            PieceKind::Cannon,
            PieceKind::Soldier,
        ] {
            assert_eq!(
                position
                    .pieces()
                    .filter(|(_, p)| p.side == side && p.kind == kind)
                    .count(),
                kind.cap() as usize
            );
        }
    }
    assert_eq!(
        position.piece(point("b0")),
        Some(Piece::new(Side::Red, PieceKind::Horse))
    );
    assert_eq!(
        position.piece(point("c9")),
        Some(Piece::new(Side::Black, PieceKind::Elephant))
    );
    let text = INITIAL_FEN.replace(" w - - 0 1", " b - - 123 99");
    let imported = Position::from_fen(&text).unwrap();
    assert_eq!(imported.side_to_move(), Side::Black);
    assert_eq!(imported.counters().no_capture_plies(), Some(123));
    assert_eq!(imported.counters().fullmove_number(), Some(99));
    assert_eq!(imported.to_fen().unwrap(), text);
}

#[test]
fn strict_position_notation_rejects_aliases_width_spacing_and_counter_errors() {
    for text in [
        INITIAL_FEN.replace(" w ", " r "),
        INITIAL_FEN.replace(" - - ", " KQ - "),
        INITIAL_FEN.replace(" - - ", " - e3 "),
        INITIAL_FEN.replace(" 0 1", " 00 1"),
        INITIAL_FEN.replace(" 0 1", " 0 0"),
        INITIAL_FEN.replace(" 0 1", " -1 1"),
        INITIAL_FEN.replace(" 0 1", " 4294967296 1"),
        INITIAL_FEN.replace(" 0 1", " 0 4294967296"),
        INITIAL_FEN.replace('n', "h"),
        INITIAL_FEN.replace('b', "e"),
        INITIAL_FEN.replace("/9/", "/45/"),
        INITIAL_FEN.replace("/9/", "/8/"),
        INITIAL_FEN.replace("/9/", "/99/"),
        INITIAL_FEN.replace(' ', "  "),
        format!("{INITIAL_FEN}\n"),
        format!("{INITIAL_FEN} extra"),
        format!(" {INITIAL_FEN}"),
        "x".repeat(MAX_FEN_BYTES + 1),
        "９".into(),
    ] {
        assert!(Position::from_fen(&text).is_err(), "{text:?}");
    }
}

#[test]
fn structural_diagram_domain_is_not_live_admission_or_reachability() {
    let red_general = Piece::new(Side::Red, PieceKind::General);
    let black_general = Piece::new(Side::Black, PieceKind::General);
    assert_eq!(
        Position::from_diagram(&[], Side::Red, Counters::unknown()),
        Err(PositionError::GeneralCount)
    );
    assert_eq!(
        Position::from_diagram(
            &[(point("d0"), red_general), (point("d0"), black_general)],
            Side::Red,
            Counters::unknown()
        ),
        Err(PositionError::DuplicateIntersection)
    );
    let mut pieces = vec![(point("a0"), red_general), (point("i9"), black_general)];
    // Deliberately outside both palaces; valid structural diagnostic, not live root.
    let diagram = Position::from_diagram(&pieces, Side::Red, Counters::unknown()).unwrap();
    assert_eq!(diagram.to_fen(), Err(PositionError::UnknownCounters));
    assert_eq!(diagram.counters(), Counters::unknown());
    for at in ["a1", "b1", "c1"] {
        pieces.push((point(at), Piece::new(Side::Red, PieceKind::Rook)));
    }
    assert_eq!(
        Position::from_diagram(&pieces, Side::Red, Counters::unknown()),
        Err(PositionError::ExcessMaterial)
    );
    assert_eq!(Counters::new(Some(0), Some(0)), Err(PositionError::Counter));
}

#[test]
fn root_codec_preserves_explicit_provenance_unknown_counters_and_no_history() {
    for root in [
        PositionRoot::standard(),
        PositionRoot::from_fen(INITIAL_FEN).unwrap(),
        PositionRoot::analysis(sparse(Side::Black, Counters::unknown())),
    ] {
        let encoded = root.to_json();
        assert!(encoded.len() < 512);
        assert_eq!(PositionRoot::from_json(&encoded).unwrap(), root);
        assert!(encoded.contains("starts_at_this_position"));
    }
    assert_ne!(
        PositionRoot::standard(),
        PositionRoot::from_fen(INITIAL_FEN).unwrap()
    );
    let root = PositionRoot::analysis(sparse(Side::Red, Counters::unknown()));
    assert!(root.to_json().contains("\"no_capture_plies\":null"));
    assert_eq!(root.origin(), RootOrigin::AnalysisDiagram);
    assert_eq!(
        root.position().to_fen(),
        Err(PositionError::UnknownCounters)
    );
}

#[test]
fn analysis_root_counters_require_explicit_null_never_missing_field_defaults() {
    for counters in [
        Counters::unknown(),
        Counters::new(Some(9), Some(3)).unwrap(),
    ] {
        let encoded = PositionRoot::analysis(sparse(Side::Red, counters)).to_json();
        for field in ["no_capture_plies", "fullmove_number"] {
            let mut value: serde_json::Value = serde_json::from_str(&encoded).unwrap();
            value.as_object_mut().unwrap().remove(field);
            assert_eq!(
                PositionRoot::from_json(&value.to_string()),
                Err(PositionError::Json)
            );
        }
    }
}

#[test]
fn root_codec_direct_typed_parser_rejects_duplicate_unknown_missing_and_conflicting_fields() {
    let encoded = PositionRoot::standard().to_json();
    let value: serde_json::Value = serde_json::from_str(&encoded).unwrap();
    let object = value.as_object().unwrap();
    for (field, v) in object {
        // Direct string injection preserves duplicates into the strict parser.
        let duplicate = format!(
            "{{{}:{v},{}",
            serde_json::to_string(field).unwrap(),
            &encoded[1..]
        );
        assert!(
            PositionRoot::from_json(&duplicate).is_err(),
            "duplicate {field}"
        );
        let mut missing = object.clone();
        missing.remove(field);
        assert!(
            PositionRoot::from_json(&serde_json::to_string(&missing).unwrap()).is_err(),
            "missing {field}"
        );
    }
    for suffix in ["\"events\":[]", "\"moves\":[]", "\"future\":true"] {
        assert!(PositionRoot::from_json(&format!("{{{suffix},{}", &encoded[1..])).is_err());
    }
    for (field, wrong) in [
        ("profile", "mshogi.shogi.csa36.v1"),
        ("schema", "mshogi.xiangqi.game.v1"),
        ("edition", "WXF-2020"),
        ("history", "complete"),
        ("side", "white"),
        ("origin", "unknown"),
    ] {
        let mut bad = object.clone();
        bad.insert(field.into(), serde_json::Value::String(wrong.into()));
        assert!(
            PositionRoot::from_json(&serde_json::to_string(&bad).unwrap()).is_err(),
            "{field}"
        );
    }
    let unknown = PositionRoot::analysis(sparse(Side::Red, Counters::unknown())).to_json();
    assert!(PositionRoot::from_json(&unknown.replace("analysis_diagram", "imported_fen")).is_err());
    assert!(
        PositionRoot::from_json(&unknown.replace("analysis_diagram", "standard_initial")).is_err()
    );
    assert!(PositionRoot::from_json(&format!("{encoded} {{}}")).is_err());
    assert!(PositionRoot::from_json(&" ".repeat(MAX_POSITION_BYTES + 1)).is_err());
}

#[test]
fn complete_action_codec_has_no_drop_promotion_or_terminal_command() {
    for from in Intersection::all() {
        for to in Intersection::all() {
            let action = Action { from, to };
            assert_eq!(action.to_string().parse::<Action>(), Ok(action));
        }
    }
    for bad in [
        "resign", "win", "null", "a0a1q", "a0a1+", "P*a1", "a0j0", "a0a10", "é0a1", "a0a1\n",
    ] {
        assert!(bad.parse::<Action>().is_err(), "{bad:?}");
    }
    let action: Action = "a1a2".parse().unwrap();
    assert_eq!(serde_json::to_string(&action).unwrap(), "\"a1a2\"");
    assert_eq!(serde_json::from_str::<Action>("\"a1a2\"").unwrap(), action);
    assert!(
        serde_json::from_str::<Action>("{\"from\":\"a1\",\"to\":\"a2\",\"capture\":true}").is_err()
    );
}

#[test]
fn private_transition_derives_capture_resets_count_and_restores_every_value() {
    let before = sparse(Side::Red, Counters::new(Some(37), Some(12)).unwrap());
    let root = PositionRoot::analysis(before.clone());
    let mut position = before.clone();
    let prepared = position
        .prepare_structural("a1a2".parse().unwrap())
        .unwrap();
    assert_eq!(position, before);
    assert_eq!(
        prepared.effects().captured,
        Some(Piece::new(Side::Black, PieceKind::Horse))
    );
    let undo = position.commit_resolved(prepared).unwrap();
    assert_eq!(undo.effects().moving.kind, PieceKind::Rook);
    assert_eq!(position.piece(point("a1")), None);
    assert_eq!(
        position.piece(point("a2")),
        Some(Piece::new(Side::Red, PieceKind::Rook))
    );
    assert_eq!(position.side_to_move(), Side::Black);
    assert_eq!(
        position.counters(),
        Counters::new(Some(0), Some(12)).unwrap()
    );
    assert_eq!(root.position(), &before); // immutable root provenance is not current state
    position.undo(undo).unwrap();
    assert_eq!(position, before);
    assert_eq!(position.to_fen(), before.to_fen());
}

#[test]
fn private_transition_preserves_unknowns_and_increments_black_fullmove() {
    let mut position = sparse(Side::Black, Counters::new(Some(4), Some(9)).unwrap());
    let before = position.clone();
    let undo = position
        .commit_resolved(
            position
                .prepare_structural("f9f8".parse().unwrap())
                .unwrap(),
        )
        .unwrap();
    assert_eq!(
        position.counters(),
        Counters::new(Some(5), Some(10)).unwrap()
    );
    position.undo(undo).unwrap();
    assert_eq!(position, before);
    let mut unknown = sparse(Side::Black, Counters::unknown());
    unknown
        .commit_resolved(unknown.prepare_structural("f9f8".parse().unwrap()).unwrap())
        .unwrap();
    assert_eq!(unknown.counters(), Counters::unknown());
    assert!(PositionRoot::from_json(&PositionRoot::analysis(unknown).to_json()).is_ok());
}

#[test]
fn private_atomic_rejections_and_counter_limits_are_not_game_outcomes() {
    let position = sparse(Side::Red, Counters::new(Some(u32::MAX), Some(1)).unwrap());
    for (action, reason) in [
        ("a1a1", ActionError::SameIntersection),
        ("b1b2", ActionError::EmptySource),
        ("f9f8", ActionError::WrongSide),
        ("a1d0", ActionError::FriendlyDestination),
        ("a1f9", ActionError::GeneralCapture),
        ("a1b1", ActionError::CounterOverflow),
    ] {
        assert_eq!(
            position
                .prepare_structural(action.parse().unwrap())
                .unwrap_err(),
            reason
        );
        assert_eq!(
            position,
            sparse(Side::Red, Counters::new(Some(u32::MAX), Some(1)).unwrap())
        );
    }
    // An observed capture resets the notation counter, so MAX does not overflow.
    assert!(position.prepare_structural("a1a2".parse().unwrap()).is_ok());
    let black = sparse(Side::Black, Counters::new(Some(0), Some(u32::MAX)).unwrap());
    assert_eq!(
        black
            .prepare_structural("f9f8".parse().unwrap())
            .unwrap_err(),
        ActionError::CounterOverflow
    );
}

#[test]
fn stale_preparation_and_out_of_order_undo_are_atomic() {
    let mut position = sparse(Side::Red, Counters::new(Some(0), Some(1)).unwrap());
    let prepared = position
        .prepare_structural("a1a2".parse().unwrap())
        .unwrap();
    let first = position
        .commit_resolved(
            position
                .prepare_structural("d0e0".parse().unwrap())
                .unwrap(),
        )
        .unwrap();
    let current = position.clone();
    assert_eq!(
        position.commit_resolved(prepared).unwrap_err(),
        ActionError::StalePosition
    );
    assert_eq!(position, current);
    position
        .commit_resolved(
            position
                .prepare_structural("f9f8".parse().unwrap())
                .unwrap(),
        )
        .unwrap();
    let current = position.clone();
    assert_eq!(position.undo(first), Err(UndoError::PositionMismatch));
    assert_eq!(position, current);
}

#[test]
fn local_sets_and_perft_ignore_counter_exhaustion_but_commit_is_atomic() {
    for (side, quiet) in [(Side::Red, "d0d1"), (Side::Black, "f9f8")] {
        let mut position = sparse(side, Counters::new(Some(u32::MAX), Some(u32::MAX)).unwrap());
        let before = position.clone();
        let mut ordinary = position.clone();
        ordinary.counters = Counters::new(Some(0), Some(1)).unwrap();
        assert_eq!(position.legal_actions(), ordinary.legal_actions());
        assert_eq!(position.local_status(), ordinary.local_status());
        assert_eq!(position.perft(2), ordinary.perft(2));
        let action = quiet.parse().unwrap();
        assert!(position.legal_actions().unwrap().contains(&action));
        assert_eq!(
            position.apply(action).unwrap_err(),
            ActionError::CounterOverflow
        );
        assert_eq!(position, before);
        assert_eq!(
            position
                .apply(quiet[..2].repeat(2).parse().unwrap())
                .unwrap_err(),
            ActionError::SameIntersection
        );
        assert_eq!(position, before);
    }
    let mut capture = sparse(Side::Red, Counters::new(Some(u32::MAX), Some(1)).unwrap());
    let before = capture.clone();
    let undo = capture.apply("a1a2".parse().unwrap()).unwrap();
    assert_eq!(capture.counters().no_capture_plies(), Some(0));
    capture.undo(undo).unwrap();
    assert_eq!(capture, before);
}

#[test]
fn complete_public_action_admission_is_atomic_for_all_coordinate_pairs_and_stale_values() {
    let mut position = Position::initial();
    let before = position.clone();
    let legal = position.legal_actions().unwrap();
    for from in Intersection::all() {
        for to in Intersection::all() {
            let action = Action { from, to };
            match position.apply(action) {
                Ok(undo) => {
                    assert!(legal.contains(&action));
                    position.undo(undo).unwrap();
                }
                Err(_) => assert!(!legal.contains(&action)),
            }
            assert_eq!(position, before);
        }
    }
    let stale = position.resolve_action("a0a1".parse().unwrap()).unwrap();
    let first = position.apply("b0a2".parse().unwrap()).unwrap();
    let changed = position.clone();
    assert_eq!(
        position.apply_resolved(stale).unwrap_err(),
        ActionError::StalePosition
    );
    assert_eq!(position, changed);
    position.undo(first).unwrap();
    assert_eq!(position, before);
}

#[test]
fn structural_geography_and_nonmoving_threats_are_not_false_empty_legal_sets() {
    // Explicit off-home diagnostics remain constructible, but not live roots.
    for (side, kind, invalid) in [
        (Side::Red, PieceKind::Advisor, "e0"),
        (Side::Black, PieceKind::Advisor, "e9"),
        (Side::Red, PieceKind::Elephant, "c2"),
        (Side::Black, PieceKind::Elephant, "g7"),
        (Side::Red, PieceKind::Soldier, "b3"),
        (Side::Black, PieceKind::Soldier, "h6"),
    ] {
        let position = Position::from_diagram(
            &[
                (point("d0"), Piece::new(Side::Red, PieceKind::General)),
                (point("f9"), Piece::new(Side::Black, PieceKind::General)),
                (point(invalid), Piece::new(side, kind)),
            ],
            side,
            Counters::unknown(),
        )
        .unwrap();
        assert_eq!(
            position.validate_local(),
            Err(LocalPositionError::PieceGeography { at: point(invalid) })
        );
        assert!(position.legal_actions().is_err());
        assert!(position.local_status().is_err());
        assert!(position.perft(0).is_err());
        assert!(position.divide(0).is_err());
    }
    let facing = Position::from_fen("4k4/9/9/9/9/9/9/9/9/4K4 w - - 0 1").unwrap();
    assert!(facing.is_in_check(Side::Red));
    assert!(facing.is_in_check(Side::Black));
    assert_eq!(
        facing.validate_local(),
        Err(LocalPositionError::NonMovingGeneralAttacked)
    );
    assert!(facing.legal_actions().is_err());
    assert_eq!(Position::initial().perft(4), Err(PerftError::DepthLimit));
}

#[test]
fn raw_counter_preserves_check_plies_and_root_provenance_through_reversal() {
    let root = PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap();
    let encoded_root = root.to_json();
    let mut position = root.position().clone();
    let mut records = Vec::new();
    for _ in 0..6 {
        for name in ["e9f9", "f8e8", "f9e9", "e8f8"] {
            records.push(position.apply(name.parse().unwrap()).unwrap());
        }
    }
    assert_eq!(position.counters().no_capture_plies(), Some(24));
    assert_eq!(position.counters().fullmove_number(), Some(13));
    assert_eq!(root.to_json(), encoded_root);
    while let Some(undo) = records.pop() {
        position.undo(undo).unwrap();
    }
    assert_eq!(&position, root.position());
    assert_eq!(root.to_json(), encoded_root);
}
