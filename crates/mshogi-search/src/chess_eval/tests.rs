//! Independent expectations for the evaluation arithmetic.
//!
//! Commit a01eb633 holds the test that proved this evaluation equal, term by
//! term, to the earlier native-position evaluation on more than 8,000
//! positions. The expected values below are derived by hand from the term
//! definitions or from independent endgame knowledge.
use super::*;
use crate::alphabeta::SearchPosition;
use crate::chess_bitboard::tests::TestRng;
use mshogi_core::chess::Position;

fn board(fen: &str) -> ChessPosition {
    ChessPosition::from_position(&Position::from_fen(fen).unwrap())
}
fn eval(fen: &str) -> Evaluation {
    decompose(&board(fen))
}
/// Swap colors and flip the board. Only for positions without castling rights
/// and without an en passant square.
fn reflect(fen: &str) -> String {
    let parts: Vec<_> = fen.split_whitespace().collect();
    assert_eq!((parts[2], parts[3]), ("-", "-"), "{fen}");
    let rows: Vec<String> = parts[0]
        .split('/')
        .rev()
        .map(|rank| {
            rank.chars()
                .map(|c| {
                    if c.is_ascii_uppercase() {
                        c.to_ascii_lowercase()
                    } else {
                        c.to_ascii_uppercase()
                    }
                })
                .collect()
        })
        .collect();
    format!(
        "{} {} - - {} {}",
        rows.join("/"),
        if parts[1] == "w" { "b" } else { "w" },
        parts[4],
        parts[5]
    )
}
fn tapered(middle: i32, end: i32) -> Tapered {
    Tapered { middle, end }
}
fn difference(e: &Evaluation) -> i32 {
    e.white.contribution(e.middle_phase) - e.black.contribution(e.middle_phase)
}

#[test]
fn term_arithmetic_and_perspective() {
    // Bare king centrality: Kd4 has 6, Kh8 has 0; zero phase gives 14 * 6 = 84.
    let fen = "7k/8/8/8/3K4/8/8/8 w - - 0 1";
    let e = eval(fen);
    assert_eq!(e.middle_phase, 0);
    assert_eq!(e.white.placement, tapered(-76, 84));
    assert_eq!(e.white_score(), 84);
    assert_eq!(evaluate(&board(fen)), 84);
    assert_eq!(evaluate(&board(&fen.replace(" w ", " b "))), -84);
    // a2 pawn: 100 material, 0 placement, -8 isolated + 10 passed + 20 escort.
    let e = eval("7k/8/8/8/8/8/P7/K7 w - - 0 1");
    assert_eq!(e.white.material, 100);
    assert_eq!(e.white.pawns.end, 22);
    assert_eq!(difference(&e), 122);
    // a6 pawn: 40 placement, -8 isolated + 122 passed + 4 escort + 64 runway.
    let e = eval("7k/8/P7/8/8/8/8/K7 w - - 0 1");
    assert_eq!(e.white.placement.end, 40);
    assert_eq!(e.white.pawns.end, 182);
    assert_eq!(difference(&e), 322);
    // Ra1 attacks 14 squares; the one that its own king occupies does not count.
    let e = eval("7k/8/8/8/8/8/8/R6K w - - 0 1");
    assert_eq!(e.white.mobility, tapered(13 * 2 + 16, 13 * 4 + 12));
}

#[test]
fn pawn_structure_shelter_and_threat_terms() {
    // The adjacent e and f pawns are connected; neither side has a passed pawn.
    let e = eval("7k/4pp2/8/8/8/8/4PP2/K7 w - - 0 1");
    assert_eq!(e.white.pawns, tapered(8, 12));
    // Two isolated a pawns blocked by the enemy a pawn: doubled -12/-16 and
    // two isolated penalties -20/-16.
    let e = eval("7k/p7/8/8/8/P7/P7/K7 w - - 0 1");
    assert_eq!(e.white.pawns, tapered(-32, -32));
    let shelter = eval("7k/8/8/8/8/8/5P2/6K1 w - - 0 1");
    let exposed = eval("7k/8/8/8/5P2/8/8/6K1 w - - 0 1");
    assert_eq!(
        shelter.white.king_safety.middle - exposed.white.king_safety.middle,
        10
    );
    // Undefended queen a7 attacked by rook a1: 900 / 16 = 56.
    let e = eval("7k/q7/8/8/8/8/8/R6K w - - 0 1");
    assert_eq!(e.white.threats, tapered(56, 56));
    // d3 cannot advance safely to d4, which e5 attacks. c4 is ahead and cannot
    // support it; c2 can.
    let behind = eval("7k/8/8/4p3/2P5/3P4/8/K7 w - - 0 1");
    let supported = eval("7k/8/8/4p3/8/3P4/2P5/K7 w - - 0 1");
    assert_eq!(behind.white.pawns, tapered(19, 54));
    assert_eq!(supported.white.pawns, tapered(13, 36));
    // a6 has a clear, unattacked runway worth 64. A rook on b8 controls a8: no
    // runway, and the 122 base loses 122 / 4 = 30.
    let clear = eval("7k/8/P7/8/8/8/8/K7 w - - 0 1");
    let controlled = eval("1r5k/8/P7/8/8/8/8/K7 w - - 0 1");
    assert_eq!(clear.white.pawns.end - controlled.white.pawns.end, 64 + 30);
    // Defending a8 with a bishop does not make the square safe.
    assert_eq!(
        eval("1r5k/8/P7/8/8/8/6B1/6K1 w - - 0 1").white.pawns.end,
        controlled.white.pawns.end
    );
    // a2 behind a3 is doubled, not a second passed pawn:
    // doubled -16, isolated -16, a3 passed 17, escort 16.
    assert_eq!(eval("7k/8/8/8/8/P7/P7/K7 w - - 0 1").white.pawns.end, 1);
}

#[test]
fn pawn_storm_and_king_pressure() {
    // An enemy pawn two ranks in front of the king: (4 - 2) * 6 = 12, or
    // (4 - 2) * 3 = 6 when an own pawn blocks it. Four ranks away: nothing.
    let quiet = eval("7k/8/8/6p1/8/8/8/6K1 w - - 0 1")
        .white
        .king_safety
        .middle;
    let storm = eval("7k/8/8/8/8/6p1/8/6K1 w - - 0 1")
        .white
        .king_safety
        .middle;
    // The g3 pawn also attacks two ring squares (f2, h2): weight 2 each, one
    // attacker: 4 * (2 + 1) = 12 pressure. The g file is not empty of enemy
    // pawns in both cases, so the shelter term is equal.
    assert_eq!(quiet - storm, 12 + 12);
    // Kf6 supports a queen check on g7 next to the black king.
    let e = eval("7k/8/5KQ1/8/8/8/8/8 w - - 0 1");
    assert!(e.black.king_safety.middle < -100);
}

#[test]
fn conversion_preference_with_a_major_piece() {
    // Enemy king in the corner: edge 6 gives 60; king distance 7 gives 0.
    let edge = eval("7k/8/8/8/8/8/8/KQ6 w - - 0 1");
    let center = eval("8/8/8/4k3/8/8/8/KQ6 w - - 0 1");
    assert_eq!(edge.white.endgame, tapered(0, 60));
    assert!(edge.white.endgame.end > center.white.endgame.end);
    assert_eq!(
        eval("7k/8/8/8/8/8/8/KN6 w - - 0 1").white.endgame,
        Tapered::default()
    );
    for major in ["Q", "R"] {
        let a = eval(&format!("7k/8/8/8/3K4/8/8/{major}7 w - - 0 1"));
        let b = eval(&format!("7k/8/5K2/8/8/8/8/{major}7 w - - 0 1"));
        // The king's own centrality is cancelled against a bare king; two
        // steps nearer are worth 24.
        assert_eq!(
            b.white.placement.end + b.white.endgame.end
                - a.white.placement.end
                - a.white.endgame.end,
            24
        );
        assert!(b.white_score() > a.white_score());
    }
}

#[test]
fn specialized_endings_follow_turn_color_and_material_guards() {
    // Tablebase facts: the e4 opposition position is a draw with White to move
    // and a win with Black to move.
    for (fen, kind, zero) in [
        (
            "8/4k3/8/4K3/4P3/8/8/8 w - - 0 1",
            EndgameKind::KingPawnKing,
            true,
        ),
        (
            "8/4k3/8/4K3/4P3/8/8/8 b - - 0 1",
            EndgameKind::KingPawnKing,
            false,
        ),
        (
            "k7/P7/1K6/8/8/8/8/2B5 w - - 0 1",
            EndgameKind::WrongBishopRookPawn,
            true,
        ),
        (
            "8/8/8/8/8/2K5/NN6/7k w - - 0 1",
            EndgameKind::TwoKnightsDrawish,
            true,
        ),
    ] {
        let e = eval(fen);
        assert_eq!(e.specialized.unwrap().0, kind, "{fen}");
        assert_eq!(e.white_score() == 0, zero, "{fen}");
        assert!(e.white_score().abs() < 1000);
        assert_eq!(eval(&reflect(fen)).white_score(), -e.white_score(), "{fen}");
    }
    // Right bishop, far defender, additional pawn, pawn not on a rook file:
    // none of these is the corner fortress.
    for fen in [
        "k7/P7/1K6/8/8/8/8/1B6 w - - 0 1",
        "7k/P7/1K6/8/8/8/8/2B5 w - - 0 1",
        "k7/P7/1K6/8/8/8/1P6/2B5 w - - 0 1",
        "k7/2P5/1K6/8/8/8/8/2B5 w - - 0 1",
    ] {
        assert!(eval(fen).specialized.is_none(), "{fen}");
        assert!(eval(&reflect(fen)).specialized.is_none(), "{fen}");
    }
    // The bare king must go to a corner of the bishop's color: the d2 bishop
    // mates on h8, not on a8.
    let correct = eval("7k/8/8/8/3K4/8/3B4/4N3 w - - 0 1");
    let wrong = eval("k7/8/8/8/3K4/8/3B4/4N3 w - - 0 1");
    assert_eq!(
        correct.specialized.unwrap().0,
        EndgameKind::BishopKnightConversion
    );
    assert!(correct.white_score() > wrong.white_score());
}

#[test]
fn passed_pawn_race_uses_the_turn() {
    // Tablebase fact: a6 wins with White to move; Black to move draws. The
    // king on e7 needs four moves to a8 and the pawn three pushes.
    let w = eval("8/4k2p/8/P7/8/8/8/7K w - - 0 1");
    let b = eval("8/4k2p/8/P7/8/8/8/7K b - - 0 1");
    assert!(w.specialized.is_none() && b.specialized.is_none());
    assert_eq!(w.white.pawns.end - b.white.pawns.end, 27);
    assert_eq!(w.white.pawns.middle, b.white.pawns.middle);
    assert_eq!(w.black, b.black);
    // An enemy rook behind the pawn follows it; a rook on b1 does not.
    let rear = eval("7k/8/8/P7/8/8/7K/r7 w - - 0 1");
    let side = eval("7k/8/8/P7/8/8/7K/1r6 w - - 0 1");
    assert_eq!(side.white.pawns.end - rear.white.pawns.end, 27);
    // An occupied a6 gives no runway premium.
    let blocked = eval("7k/8/N7/P7/8/8/7K/8 w - - 0 1");
    let clear = eval("7k/8/8/P7/8/8/7K/8 w - - 0 1");
    assert!(blocked.white.pawns.end < clear.white.pawns.end);
}

/// Color symmetry: the reflected position has the same terms with the colors
/// exchanged. Checked on positions from random legal games.
#[test]
fn evaluation_is_symmetric_under_color_reflection() {
    let mut rng = TestRng(0xE7A1_0000_1234_5678);
    let starts = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        "8/5pk1/6p1/8/3R4/6P1/5PK1/3r4 w - - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "4k3/pp3ppp/8/3P4/8/8/PP3PPP/4K3 w - - 0 1",
    ];
    let mut checked = 0;
    for game in 0..60 {
        let mut native = Position::from_fen(starts[game % starts.len()]).unwrap();
        let mut fast = ChessPosition::from_position(&native);
        for _ in 0..100 {
            if native.en_passant_target().is_none() {
                let fen = native.to_fen();
                let e = decompose(&fast);
                let mirrored = eval(&reflect(&fen));
                assert_eq!(e.white, mirrored.black, "{fen}");
                assert_eq!(e.black, mirrored.white, "{fen}");
                assert_eq!(e.white_score(), -mirrored.white_score(), "{fen}");
                assert_eq!(evaluate(&fast), evaluate(&board(&reflect(&fen))), "{fen}");
                checked += 1;
            }
            let legal = fast.legal_moves();
            if legal.is_empty() || native.halfmove_clock() >= 100 {
                break;
            }
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
    assert!(checked > 3_000, "{checked}");
}
