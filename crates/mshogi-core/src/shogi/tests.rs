use super::*;

fn sq(text: &str) -> Square {
    text.parse().unwrap()
}
fn action(text: &str) -> Action {
    text.parse().unwrap()
}
fn diagram(
    pieces: &[(&str, Side, PieceKind, bool)],
    hands: [Hand; 2],
    side: Side,
    counter: Option<u32>,
) -> Position {
    let mut board = [None; 81];
    board[sq("9a").index()] = Piece::new(Side::Gote, PieceKind::King, false);
    board[sq("1i").index()] = Piece::new(Side::Sente, PieceKind::King, false);
    for &(square, owner, kind, promoted) in pieces {
        if kind == PieceKind::King {
            board[sq(if owner == Side::Sente { "1i" } else { "9a" }).index()] = None;
        }
        board[sq(square).index()] = Piece::new(owner, kind, promoted);
    }
    Position::from_diagram(board, hands, side, counter).unwrap()
}
fn material(position: &Position) -> [u16; 7] {
    let mut counts = [0; 7];
    for kind in PieceKind::HAND_KINDS {
        let i = kind.hand_index().unwrap();
        counts[i] = u16::from(position.hand(Side::Sente).count(kind))
            + u16::from(position.hand(Side::Gote).count(kind));
        counts[i] += position
            .board()
            .iter()
            .flatten()
            .filter(|p| p.kind == kind)
            .count() as u16;
    }
    counts
}

#[test]
fn initial_placement_sfen_and_declared_origin() {
    let p = Position::initial();
    assert_eq!(p.to_sfen(), INITIAL_SFEN);
    assert_eq!(p.history_origin(), HistoryOrigin::Initial);
    assert_eq!(p.side_to_move(), Side::Sente);
    assert_eq!(p.next_ply(), 1);
    assert_eq!(p.completed_actions(), 0);
    assert_eq!(p.board().iter().flatten().count(), 40);
    for (square, side, kind) in [
        ("5i", Side::Sente, PieceKind::King),
        ("2h", Side::Sente, PieceKind::Rook),
        ("8h", Side::Sente, PieceKind::Bishop),
        ("5a", Side::Gote, PieceKind::King),
        ("8b", Side::Gote, PieceKind::Rook),
        ("2b", Side::Gote, PieceKind::Bishop),
        ("7g", Side::Sente, PieceKind::Pawn),
        ("7c", Side::Gote, PieceKind::Pawn),
    ] {
        assert_eq!(p.piece(sq(square)), Piece::new(side, kind, false));
    }
    assert_eq!(material(&p), [2, 2, 4, 4, 4, 4, 18]);
    let imported = Position::from_sfen(INITIAL_SFEN).unwrap();
    assert_eq!(imported.to_sfen(), p.to_sfen());
    assert_ne!(imported.history_origin(), p.history_origin());
    assert_eq!(
        imported.history_origin(),
        HistoryOrigin::Diagram {
            next_ply: 1,
            counter_source: CounterSource::Supplied
        }
    );
}

#[test]
fn square_and_complete_usi_choices_roundtrip() {
    assert_eq!(Square::new(9, 1).unwrap().index(), 0);
    assert_eq!(Square::new(1, 9).unwrap().index(), 80);
    for square in Square::all() {
        assert_eq!(square.to_string().parse::<Square>(), Ok(square));
    }
    for invalid in ["0a", "1j", "a1", "1A", "11", "1a ", "１a", "", "10a"] {
        assert!(invalid.parse::<Square>().is_err(), "{invalid}");
    }
    for text in [
        "7g7f", "2b3c+", "R*5e", "B*1a", "P*9h", "S*4d", "G*2b", "N*5f", "L*3c",
    ] {
        assert_eq!(action(text).to_string(), text);
    }
    for invalid in [
        "resign", "win", "0000", "7g7f++", "7g7f-", "p*5e", "K*5e", "P*5e+", "5e5d ", "Ｐ*5e",
        "7g7", "7g7f\n",
    ] {
        assert!(invalid.parse::<Action>().is_err(), "{invalid}");
    }
    // Parsing describes a choice; board resolution rejects the null action.
    assert_eq!(
        action("5e5e"),
        Action::Move {
            from: sq("5e"),
            to: sq("5e"),
            promote: false
        }
    );
}

#[test]
fn custom_sfen_preserves_hands_promotions_counter_and_unknown_prehistory() {
    let text = "k8/9/2+s6/3+P5/4+B4/5+r3/9/9/8K w R2G3Pbl 320";
    let p = Position::from_sfen(text).unwrap();
    assert_eq!(p.to_sfen(), text);
    assert_eq!(p.next_ply(), 320);
    assert_eq!(p.completed_actions(), 319);
    assert_eq!(
        p.history_origin(),
        HistoryOrigin::Diagram {
            next_ply: 320,
            counter_source: CounterSource::Supplied
        }
    );
    assert_eq!(
        p.piece(sq("7c")),
        Piece::new(Side::Gote, PieceKind::Silver, true)
    );
    assert_eq!(
        p.piece(sq("6d")),
        Piece::new(Side::Sente, PieceKind::Pawn, true)
    );
    assert_eq!(p.hand(Side::Sente).count(PieceKind::Gold), 2);
    assert_eq!(p.hand(Side::Gote).count(PieceKind::Bishop), 1);
    assert_eq!(Position::from_sfen(&p.to_sfen()).unwrap(), p);
    let reordered = Position::from_sfen("k8/9/9/9/9/9/9/9/8K b p2GPR 987").unwrap();
    assert_eq!(reordered.to_sfen(), "k8/9/9/9/9/9/9/9/8K b R2GPp 987");
    let defaulted = diagram(&[], [Hand::empty(); 2], Side::Sente, None);
    assert_eq!(
        defaulted.history_origin(),
        HistoryOrigin::Diagram {
            next_ply: 1,
            counter_source: CounterSource::Defaulted
        }
    );
    let beyond_limit = Position::from_sfen("k8/9/9/9/9/9/9/9/8K b - 321").unwrap();
    assert_eq!(beyond_limit.next_ply(), 321); // analysis import, no match classification
}

#[test]
fn malformed_and_locally_invalid_imports_are_rejected_without_touching_existing_state() {
    let p = Position::initial();
    let before = p.clone();
    let invalid = [
        "",
        "9/9/9/9/9/9/9/9/9 b - 1", // missing kings
        "k8/9/9/9/9/9/9/9/8K b -",
        "k8/9/9/9/9/9/9/9/8K  b - 1",
        "k8/9/9/9/9/9/9/9/8K b - 1 ",
        "k8/9/9/9/9/9/9/9/8K\tb - 1",
        "k8/9/9/9/9/9/9/9/8K x - 1",
        "k8/9/9/9/9/9/9/9/8K b - 0",
        "k8/9/9/9/9/9/9/9/8K b - 01",
        "k8/9/9/9/9/9/9/9/8K b - +1",
        "k8/9/9/9/9/9/9/9/8K b - 4294967296",
        "k8/9/9/9/9/9/9/9/8K b - -1",
        "k8/9/9/9/9/9/9/9/8K b K 1",
        "k8/9/9/9/9/9/9/9/8K b PP 1",
        "k8/9/9/9/9/9/9/9/8K b 1P 1",
        "k8/9/9/9/9/9/9/9/8K b 0P 1",
        "k8/9/9/9/9/9/9/9/8K b P2 1",
        "k8/9/9/9/9/9/9/9/8K b 19P 1",
        "k8/9/9/9/9/9/9/9/8K b 2PpP 1",
        "k8/9/9/9/9/9/9/9/8K b +P 1",
        "k8/9/9/9/9/9/9/9/7+K b - 1",
        "k8/9/9/9/9/9/9/9/7+G1K b - 1",
        "k8/9/9/9/9/9/9/9/8K/9 b - 1",
        "k8/9/9/9/9/9/9/9/7K b - 1",
        "k8/9/9/9/9/9/9/9/8KK b - 1",
        "k8/9/9/9/9/9/9/9/44K b - 1",
        "k8/9/9/9/9/9/9/9/7+ b - 1",
        "k8/9/9/9/9/9/9/9/7QK b - 1",
        "k7P/9/9/9/9/9/9/9/8K b - 1",   // dead pawn
        "k8/8N/9/9/9/9/9/9/8K b - 1",   // dead knight
        "k8/9/9/9/9/9/9/9/l7K b - 1",   // dead gote lance
        "k8/9/8P/9/8P/9/9/9/8K b - 1",  // nifu
        "k8/9/9/9/4R4/9/9/9/8K b 2R 1", // combined inventory
        "k8/9/9/9/9/9/9/9/7ＫK b - 1",
    ];
    for text in invalid {
        assert!(Position::from_sfen(text).is_err(), "{text}");
        assert_eq!(p, before);
    }
    assert_eq!(
        Hand::from_counts(&[(PieceKind::Pawn, 19)]),
        Err(PositionError::InvalidHand)
    );
    assert_eq!(
        Hand::from_counts(&[(PieceKind::Pawn, 0), (PieceKind::Pawn, 1)]),
        Err(PositionError::InvalidHand)
    );
    let mut board = *p.board();
    board[sq("5e").index()] = Some(Piece {
        side: Side::Sente,
        kind: PieceKind::Gold,
        promoted: true,
    });
    assert_eq!(
        Position::from_diagram(board, [Hand::empty(); 2], Side::Sente, None),
        Err(PositionError::Promotion)
    );
}

#[test]
fn sparse_diagrams_are_local_and_do_not_claim_king_safety_or_reachability() {
    // Adjacent kings and an attack on the side that just moved are accepted as
    // structural diagrams. Legal-action generation must handle their safety.
    assert!(Position::from_sfen("4k4/4K4/9/9/9/9/9/9/9 b - 50").is_ok());
    assert!(Position::from_sfen("k8/9/9/9/9/9/9/9/R7K b - 1").is_ok());
    assert!(Position::from_sfen("k8/9/8+P/9/8P/9/9/9/8K b - 1").is_ok());
}

#[test]
fn opening_capture_demotes_transfers_and_nested_undo_is_exact() {
    let mut p = Position::initial();
    let root = p.clone();
    let expected_material = material(&p);
    let mut snapshots = Vec::new();
    let mut records = Vec::new();
    for text in ["7g7f", "3c3d", "8h2b+", "3a2b", "B*4e", "B*5e"] {
        snapshots.push(p.clone());
        records.push(p.apply_structural(action(text)).unwrap());
        assert_eq!(material(&p), expected_material);
        assert_eq!(p.history_origin(), HistoryOrigin::Initial);
        if text == "8h2b+" {
            assert_eq!(
                p.piece(sq("2b")),
                Piece::new(Side::Sente, PieceKind::Bishop, true)
            );
            assert_eq!(p.hand(Side::Sente).count(PieceKind::Bishop), 1);
        }
        if text == "3a2b" {
            assert_eq!(p.hand(Side::Gote).count(PieceKind::Bishop), 1);
            assert_eq!(
                p.piece(sq("2b")),
                Piece::new(Side::Gote, PieceKind::Silver, false)
            );
        }
    }
    assert_eq!(
        p.to_sfen(),
        "lnsgkg1nl/1r5s1/pppppp1pp/6p2/4bB3/2P6/PP1PPPPPP/7R1/LNSGKGSNL b - 7"
    );
    // Explicitly check dropped resources are the capturer's unpromoted pieces.
    assert_eq!(
        p.piece(sq("4e")),
        Piece::new(Side::Sente, PieceKind::Bishop, false)
    );
    assert_eq!(
        p.piece(sq("5e")),
        Piece::new(Side::Gote, PieceKind::Bishop, false)
    );
    while let Some(record) = records.pop() {
        p.undo_structural(record).unwrap();
        assert_eq!(p, snapshots.pop().unwrap());
    }
    assert_eq!(p, root);
}

#[test]
fn every_promotable_base_capture_returns_its_own_kind_not_gold() {
    for kind in PieceKind::HAND_KINDS
        .into_iter()
        .filter(|k| k.can_promote())
    {
        let mut p = diagram(
            &[
                ("5e", Side::Sente, PieceKind::Rook, false),
                ("5d", Side::Gote, kind, true),
            ],
            [Hand::empty(); 2],
            Side::Sente,
            Some(83),
        );
        let before = p.clone();
        let undo = p.apply_structural(action("5e5d")).unwrap();
        assert_eq!(p.hand(Side::Sente).count(kind), 1);
        assert_eq!(p.hand(Side::Gote).count(kind), 0);
        assert_eq!(p.side_to_move(), Side::Gote);
        assert_eq!(p.next_ply(), 84);
        assert_eq!(p.history_origin(), before.history_origin());
        assert_eq!(material(&p), material(&before));
        p.undo_structural(undo).unwrap();
        assert_eq!(p, before);
    }
}

#[test]
fn forced_optional_and_leaving_zone_promotions_for_both_sides() {
    for (side, origin, destination) in [(Side::Sente, "5b", "5a"), (Side::Gote, "5h", "5i")] {
        for kind in [PieceKind::Pawn, PieceKind::Lance] {
            let mut p = diagram(
                &[(origin, side, kind, false)],
                [Hand::empty(); 2],
                side,
                Some(1),
            );
            let before = p.clone();
            let unpromoted = Action::Move {
                from: sq(origin),
                to: sq(destination),
                promote: false,
            };
            assert_eq!(
                p.apply_structural(unpromoted).unwrap_err(),
                ActionError::MissingPromotion
            );
            assert_eq!(p, before);
            let undo = p
                .apply_structural(Action::Move {
                    from: sq(origin),
                    to: sq(destination),
                    promote: true,
                })
                .unwrap();
            assert_eq!(p.piece(sq(destination)), Piece::new(side, kind, true));
            p.undo_structural(undo).unwrap();
            assert_eq!(p, before);
        }
    }
    for (side, from, to) in [(Side::Sente, "5d", "4b"), (Side::Gote, "5f", "4h")] {
        let mut p = diagram(
            &[(from, side, PieceKind::Knight, false)],
            [Hand::empty(); 2],
            side,
            None,
        );
        assert_eq!(
            p.apply_structural(Action::Move {
                from: sq(from),
                to: sq(to),
                promote: false
            })
            .unwrap_err(),
            ActionError::MissingPromotion
        );
        p.apply_structural(Action::Move {
            from: sq(from),
            to: sq(to),
            promote: true,
        })
        .unwrap();
    }
    for (side, from, to) in [(Side::Sente, "5c", "4d"), (Side::Gote, "5g", "4f")] {
        for promote in [false, true] {
            let mut p = diagram(
                &[(from, side, PieceKind::Silver, false)],
                [Hand::empty(); 2],
                side,
                None,
            );
            p.apply_structural(Action::Move {
                from: sq(from),
                to: sq(to),
                promote,
            })
            .unwrap();
            assert_eq!(p.piece(sq(to)).unwrap().promoted, promote);
        }
    }
}

#[test]
fn geometry_matches_independent_center_movement_sets_and_ray_blocks() {
    let cases = [
        (
            PieceKind::King,
            false,
            vec!["4d", "5d", "6d", "4e", "6e", "4f", "5f", "6f"],
        ),
        (
            PieceKind::Gold,
            false,
            vec!["4d", "5d", "6d", "4e", "6e", "5f"],
        ),
        (PieceKind::Silver, false, vec!["4d", "5d", "6d", "4f", "6f"]),
        (PieceKind::Knight, false, vec!["4c", "6c"]),
        (PieceKind::Pawn, false, vec!["5d"]),
        (PieceKind::Lance, false, vec!["5a", "5b", "5c", "5d"]),
    ];
    for side in [Side::Sente, Side::Gote] {
        for (kind, promoted, expected) in &cases {
            let mut p = diagram(
                &[("5e", side, *kind, *promoted)],
                [Hand::empty(); 2],
                side,
                None,
            );
            // Center king test has only one king of this side.
            if *kind == PieceKind::King {
                p.board[sq(if side == Side::Sente { "1i" } else { "9a" }).index()] = None;
            }
            let got: Vec<_> = Square::all()
                .filter(|&s| p.geometrically_reaches(sq("5e"), s))
                .collect();
            let mut want: Vec<_> = expected
                .iter()
                .map(|s| {
                    let square = sq(s);
                    if side == Side::Gote {
                        Square::new(square.file(), 10 - square.rank()).unwrap()
                    } else {
                        square
                    }
                })
                .collect();
            want.sort();
            assert_eq!(got, want, "{side:?} {kind:?}");
        }
    }
    for kind in [
        PieceKind::Pawn,
        PieceKind::Lance,
        PieceKind::Knight,
        PieceKind::Silver,
    ] {
        let p = diagram(
            &[("5e", Side::Sente, kind, true)],
            [Hand::empty(); 2],
            Side::Sente,
            None,
        );
        let expected = ["6d", "5d", "4d", "6e", "4e", "5f"].map(sq);
        let got: Vec<_> = Square::all()
            .filter(|&s| p.geometrically_reaches(sq("5e"), s))
            .collect();
        assert_eq!(got, expected);
    }
    let rook = diagram(
        &[
            ("5e", Side::Sente, PieceKind::Rook, true),
            ("5c", Side::Gote, PieceKind::Gold, false),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        None,
    );
    assert!(rook.geometrically_reaches(sq("5e"), sq("5c")));
    assert!(!rook.geometrically_reaches(sq("5e"), sq("5b")));
    assert!(rook.geometrically_reaches(sq("5e"), sq("4d")));
    assert!(!rook.geometrically_reaches(sq("5e"), sq("3c")));
    let bishop = diagram(
        &[
            ("5e", Side::Sente, PieceKind::Bishop, true),
            ("3c", Side::Gote, PieceKind::Gold, false),
        ],
        [Hand::empty(); 2],
        Side::Sente,
        None,
    );
    assert!(bishop.geometrically_reaches(sq("5e"), sq("3c")));
    assert!(!bishop.geometrically_reaches(sq("5e"), sq("2b")));
    assert!(bishop.geometrically_reaches(sq("5e"), sq("5d")));
    assert!(!bishop.geometrically_reaches(sq("5e"), sq("5c")));
}

#[test]
fn rejected_choices_are_atomic_and_undo_requires_exact_context() {
    let mut p = Position::initial();
    for (text, error) in [
        ("5e5d", ActionError::EmptySource),
        ("3c3d", ActionError::WrongSide),
        ("7g7g", ActionError::SameSquare),
        ("7g7i", ActionError::FriendlyDestination),
        ("7g7e", ActionError::IllegalGeometry),
        ("2h2d", ActionError::IllegalGeometry),
        ("7g7f+", ActionError::UnexpectedPromotion),
        ("P*5e", ActionError::MissingResource),
    ] {
        let before = p.clone();
        assert_eq!(
            p.apply_structural(action(text)).unwrap_err(),
            error,
            "{text}"
        );
        assert_eq!(p, before);
    }
    let one = p.apply_structural(action("7g7f")).unwrap();
    let _two = p.apply_structural(action("3c3d")).unwrap();
    let before = p.clone();
    assert_eq!(p.undo_structural(one), Err(UndoError::PositionMismatch));
    assert_eq!(p, before);
    let mut overflow = Position::from_sfen("k8/9/9/9/9/9/9/9/8K b - 4294967295").unwrap();
    let before = overflow.clone();
    assert_eq!(
        overflow.apply_structural(action("1i1h")).unwrap_err(),
        ActionError::CounterOverflow
    );
    assert_eq!(overflow, before);
    let mut king_capture = diagram(
        &[("9b", Side::Sente, PieceKind::Rook, false)],
        [Hand::empty(); 2],
        Side::Sente,
        None,
    );
    assert_eq!(
        king_capture.apply_structural(action("9b9a")).unwrap_err(),
        ActionError::KingCapture
    );
}

#[test]
fn drop_resources_nifu_and_dead_ranks_are_native_for_both_sides() {
    for side in [Side::Sente, Side::Gote] {
        let hand = Hand::from_counts(&[
            (PieceKind::Pawn, 2),
            (PieceKind::Lance, 1),
            (PieceKind::Knight, 1),
        ])
        .unwrap();
        let mut hands = [Hand::empty(); 2];
        hands[side.index()] = hand;
        let pawn_rank = if side == Side::Sente { "5g" } else { "5c" };
        let mut p = diagram(
            &[
                (pawn_rank, side, PieceKind::Pawn, false),
                ("4e", side, PieceKind::Pawn, true),
            ],
            hands,
            side,
            None,
        );
        for (kind, to, error) in [
            (PieceKind::Pawn, "5e", ActionError::Nifu),
            (
                PieceKind::Pawn,
                if side == Side::Sente { "6a" } else { "6i" },
                ActionError::DeadDrop,
            ),
            (
                PieceKind::Lance,
                if side == Side::Sente { "6a" } else { "6i" },
                ActionError::DeadDrop,
            ),
            (
                PieceKind::Knight,
                if side == Side::Sente { "6b" } else { "6h" },
                ActionError::DeadDrop,
            ),
            (PieceKind::Pawn, "4e", ActionError::OccupiedDrop),
            (PieceKind::King, "5e", ActionError::MissingResource),
        ] {
            let before = p.clone();
            assert_eq!(
                p.apply_structural(Action::Drop { kind, to: sq(to) })
                    .unwrap_err(),
                error
            );
            assert_eq!(p, before);
        }
        let before = p.clone();
        let undo = p
            .apply_structural(Action::Drop {
                kind: PieceKind::Pawn,
                to: sq("4f"),
            })
            .unwrap();
        assert_eq!(p.hand(side).count(PieceKind::Pawn), 1); // promoted pawn did not block the file
        assert_eq!(p.piece(sq("4f")), Piece::new(side, PieceKind::Pawn, false));
        p.undo_structural(undo).unwrap();
        assert_eq!(p, before);
    }
}
