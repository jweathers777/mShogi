//! Independently expected destination-projection safety and native equivalence.
use mshogi_core::chess::{ActionError, Position, STANDARD_START_FEN, Square};
fn target(p: &Position, square: &str) -> Vec<String> {
    let bytes = square.as_bytes();
    let destination = Square::new(bytes[0] - b'a', bytes[1] - b'1').unwrap();
    let mut result = p
        .legal_actions_to(destination)
        .into_iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>();
    result.sort();
    result
}
#[test]
fn destination_query_preserves_independently_expected_safety_and_special_actions() {
    for (fen, square, expected) in [
        // Both rook and king can remove the checking pawn.
        (
            "4k3/8/8/8/8/8/3p4/3RK3 w - - 0 1",
            "d2",
            vec!["d1d2", "e1d2"],
        ),
        // The king cannot capture onto the opposing rook's file.
        ("3r3k/8/8/8/8/8/3p4/3RK3 w - - 0 1", "d2", vec!["d1d2"]),
        // Moving the pinned rook would uncover check; the king may capture.
        ("4r2k/8/8/8/8/8/4Rp2/4K3 w - - 0 1", "f2", vec!["e1f2"]),
        ("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9", "d6", vec!["e5d6"]),
        // EP removes both rank blockers and exposes the king on h5.
        ("4k3/8/8/r4pPK/8/8/8/8 w - f6 0 9", "f6", vec![]),
        (
            "1r5k/2P5/8/8/8/8/8/K7 w - - 0 1",
            "b8",
            vec!["c7b8b", "c7b8n", "c7b8q", "c7b8r"],
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            "g1",
            vec!["e1g1", "h1g1"],
        ),
        // f1 is attacked: castling is forbidden while the rook slide is legal.
        ("4kr2/8/8/8/8/8/8/4K2R w K - 0 1", "g1", vec!["h1g1"]),
    ] {
        let p = Position::from_fen(fen).unwrap();
        let before = p.clone();
        assert_eq!(target(&p, square), expected, "{fen} -> {square}");
        assert_eq!(p, before);
    }
}
#[test]
fn destination_query_keeps_counter_independent_legality_and_actual_overflow() {
    let p = Position::from_fen("4k3/8/8/8/8/8/8/4K3 b - - 4294967295 4294967295").unwrap();
    assert_eq!(target(&p, "d8"), ["e8d8"]);
    let mut copy = p.clone();
    assert!(matches!(
        copy.apply("e8d8".parse().unwrap()),
        Err(ActionError::CounterOverflow)
    ));
    assert_eq!(copy, p);
}
#[test]
fn destination_projection_equals_full_native_list_at_every_square_along_real_walks() {
    for fen in [
        STANDARD_START_FEN,
        "r3k2r/1P6/8/3pP3/8/8/8/R3K2R w KQkq d6 0 9",
    ] {
        let mut p = Position::from_fen(fen).unwrap();
        for ply in 0..20 {
            let before = p.clone();
            let all = p.legal_actions();
            for rank in 0..8 {
                for file in 0..8 {
                    let square = Square::new(file, rank).unwrap();
                    let expected = all
                        .iter()
                        .copied()
                        .filter(|a| a.to == square)
                        .collect::<Vec<_>>();
                    assert_eq!(p.legal_actions_to(square), expected);
                    let captures: Vec<_> = p
                        .captures_and_promotions()
                        .into_iter()
                        .filter(|a| {
                            a.to == square
                                && (p.piece_at(a.to).is_some()
                                    || a.from.file() != a.to.file()
                                        && p.piece_at(a.from).is_some_and(|piece| {
                                            piece.kind == mshogi_core::chess::PieceKind::Pawn
                                        }))
                        })
                        .collect();
                    assert_eq!(p.legal_captures_to(square), captures);
                }
            }
            assert_eq!(p, before);
            if all.is_empty() {
                break;
            }
            let action = all[(ply * 17 + 3) % all.len()];
            p.apply(action).unwrap();
        }
    }
}

#[test]
fn native_candidate_query_matches_full_checked_public_transition_on_corpus_walks() {
    let corpus: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut positions = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let Some(fen) = case["input"]["fen"].as_str() else {
            continue;
        };
        let Ok(mut p) = Position::from_fen(fen) else {
            continue;
        };
        for ply in 0..16 {
            // The public transition retains complete structural validation and
            // Undo snapshots. Normalize only representation counters, as the
            // native query contract requires; retain all geometry and rights.
            let fen = p.to_fen();
            let mut fields: Vec<_> = fen.split_whitespace().collect();
            fields[4] = "0";
            fields[5] = "1";
            let base = Position::from_fen(&fields.join(" ")).unwrap();
            let expected: Vec<_> = base
                .candidates()
                .into_iter()
                .filter(|a| base.clone().apply(*a).is_ok())
                .collect();
            let before = p.clone();
            assert_eq!(p.legal_actions(), expected, "{} ply{ply}", case["id"]);
            assert_eq!(p.has_legal_action(), !expected.is_empty());
            let tactical: Vec<_> = expected
                .iter()
                .copied()
                .filter(|a| {
                    a.promotion.is_some()
                        || base.piece_at(a.to).is_some()
                        || (a.from.file() != a.to.file()
                            && base.piece_at(a.from).is_some_and(|piece| {
                                piece.kind == mshogi_core::chess::PieceKind::Pawn
                            }))
                })
                .collect();
            assert_eq!(p.captures_and_promotions(), tactical);
            assert_eq!(p, before);
            positions += 1;
            if expected.is_empty() {
                break;
            }
            // Advance the independently full-validated copy, not the optimized
            // query's published transition, so an unsafe candidate is exposed.
            p = base;
            p.apply(expected[(ply * 19 + 7) % expected.len()]).unwrap();
        }
    }
    assert!(positions >= 500, "lost corpus/walk coverage: {positions}");
}

#[test]
fn tactical_projection_keeps_ep_and_all_quiet_promotions_without_quiet_slides() {
    for (fen, expected) in [
        ("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9", vec!["e5d6"]),
        (
            "7k/2P5/6K1/8/8/8/8/8 w - - 0 1",
            vec!["c7c8b", "c7c8n", "c7c8q", "c7c8r"],
        ),
        ("4k3/8/8/r4pPK/8/8/8/8 w - f6 0 9", vec![]),
    ] {
        let p = Position::from_fen(fen).unwrap();
        let mut actual: Vec<_> = p
            .captures_and_promotions()
            .iter()
            .map(ToString::to_string)
            .collect();
        actual.sort();
        assert_eq!(actual, expected);
    }
}

#[test]
fn direct_capture_target_queries_have_independent_pin_ep_and_promotion_expectations() {
    for (fen, at, expected) in [
        (
            "4k3/8/8/8/8/8/3p4/3RK3 w - - 0 1",
            "d2",
            vec!["d1d2", "e1d2"],
        ),
        ("3r3k/8/8/8/8/8/3p4/3RK3 w - - 0 1", "d2", vec!["d1d2"]),
        ("4r2k/8/8/8/8/8/4Rp2/4K3 w - - 0 1", "f2", vec!["e1f2"]),
        ("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9", "d6", vec!["e5d6"]),
        ("4k3/8/8/r4pPK/8/8/8/8 w - f6 0 9", "f6", vec![]),
        (
            "1r5k/2P5/8/8/8/8/8/K7 w - - 0 1",
            "b8",
            vec!["c7b8b", "c7b8n", "c7b8q", "c7b8r"],
        ),
        ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "g1", vec![]),
    ] {
        let p = Position::from_fen(fen).unwrap();
        let b = at.as_bytes();
        let square = Square::new(b[0] - b'a', b[1] - b'1').unwrap();
        let mut actual: Vec<_> = p
            .legal_captures_to(square)
            .iter()
            .map(ToString::to_string)
            .collect();
        actual.sort();
        assert_eq!(actual, expected, "{fen}: {at}");
    }
}
