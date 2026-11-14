//! Target-only legal capture query, checked against independent expectations
//! and against the full native legal-action list.
use mshogi_core::chess::{PieceKind, Position, STANDARD_START_FEN, Square};

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
#[test]
fn capture_query_equals_full_native_list_at_every_square_along_real_walks() {
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
                    let captures: Vec<_> = all
                        .iter()
                        .copied()
                        .filter(|a| {
                            a.to == square
                                && (p.piece_at(a.to).is_some()
                                    || a.from.file() != a.to.file()
                                        && p.piece_at(a.from)
                                            .is_some_and(|piece| piece.kind == PieceKind::Pawn))
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
