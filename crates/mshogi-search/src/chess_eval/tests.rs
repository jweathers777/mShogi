use super::*;
use crate::alphabeta::SearchPosition;
use crate::chess_bitboard::tests::TestRng;
use crate::chess_conventional_eval as reference;
use mshogi_core::chess::{Color, Position};

fn same_terms(new: &Terms, old: &reference::Terms, fen: &str) {
    assert_eq!(new.material, old.material, "material {fen}");
    for (name, a, b) in [
        ("placement", new.placement, old.placement),
        ("pawns", new.pawns, old.pawns),
        ("mobility", new.mobility, old.mobility),
        ("king_safety", new.king_safety, old.king_safety),
        ("threats", new.threats, old.threats),
        ("endgame", new.endgame, old.endgame),
    ] {
        assert_eq!((a.middle, a.end), (b.middle, b.end), "{name} {fen}");
    }
}

fn compare(native: &Position, fast: &ChessPosition) {
    let fen = native.to_fen();
    let old = reference::evaluate(native);
    let new = decompose(fast);
    same_terms(&new.white, &old.white, &fen);
    same_terms(&new.black, &old.black, &fen);
    assert_eq!(new.middle_phase, old.middle_phase, "{fen}");
    assert_eq!(
        new.specialized
            .map(|(kind, strong, score)| (format!("{kind:?}"), strong, score)),
        old.specialized.map(|e| (
            format!("{:?}", e.kind),
            usize::from(e.strong_side == Color::Black),
            e.score
        )),
        "{fen}"
    );
    assert_eq!(
        evaluate(fast),
        old.score_for(native.side_to_move()),
        "{fen}"
    );
}

/// The bitboard evaluation equals the earlier native-position evaluation, term
/// by term, on positions from random legal games and on specialized endings.
#[test]
fn equals_the_reference_evaluation_on_random_and_special_positions() {
    let mut rng = TestRng(0xE7A1_0000_1234_5678);
    let starts = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "8/5pk1/6p1/8/3R4/6P1/5PK1/3r4 w - - 0 1",
        "8/8/4k3/8/3PK3/8/8/8 w - - 0 1",
        "8/8/8/8/5k2/2B5/3N4/4K3 w - - 0 1",
        "8/8/1k6/8/8/2Q5/p7/1K6 w - - 0 1",
        "4k3/8/4r3/8/4P3/4K3/8/R7 w - - 0 1",
        "6k1/8/6K1/8/8/7B/7P/8 w - - 0 1",
        "8/8/8/4k3/8/8/2NN4/4K3 b - - 0 1",
        "2k5/8/8/8/8/5q2/2P5/1K6 b - - 0 1",
        "4k3/pp3ppp/8/3P4/8/8/PP3PPP/4K3 w - - 0 1",
    ];
    let mut positions = 0;
    for game in 0..130 {
        let fen = starts[game % starts.len()];
        let mut native = Position::from_fen(fen).unwrap_or_else(|e| panic!("{fen}: {e:?}"));
        let mut fast = ChessPosition::from_position(&native);
        for _ in 0..150 {
            compare(&native, &fast);
            positions += 1;
            let legal = fast.legal_moves();
            if legal.is_empty() || native.halfmove_clock() >= 100 {
                break;
            }
            // Prefer captures sometimes, so the walks reach endings.
            let captures: Vec<_> = legal
                .iter()
                .copied()
                .filter(|&m| fast.is_tactical(m))
                .collect();
            let m = if !captures.is_empty() && rng.next().is_multiple_of(3) {
                captures[(rng.next() % captures.len() as u64) as usize]
            } else {
                legal[(rng.next() % legal.len() as u64) as usize]
            };
            native.apply(fast.action(m)).unwrap();
            assert!(fast.make(m));
        }
    }
    assert!(positions > 8_000, "{positions}");
}
