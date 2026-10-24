//! Expected arithmetic is hand-derived from the documented untrained parameters.
use mshogi_core::chess::{Classification, Color, PieceKind, Position, Square};
use mshogi_search::chess::{EVALUATOR_ID, evaluate, piece_value};

fn position(fen: &str) -> Position {
    Position::from_fen(fen).unwrap()
}

#[test]
fn transparent_values_and_both_perspectives() {
    assert_eq!(EVALUATOR_ID, "mshogi.chess.material.v1");
    for (kind, expected) in [
        (PieceKind::Pawn, 100),
        (PieceKind::Knight, 320),
        (PieceKind::Bishop, 330),
        (PieceKind::Rook, 500),
        (PieceKind::Queen, 900),
        (PieceKind::King, 0),
    ] {
        assert_eq!(piece_value(kind), expected);
    }
    let p = position("6nk/5p2/8/8/8/8/P1B5/KRQ5 w - - 0 1");
    let before = p.clone();
    let e = evaluate(&p);
    assert_eq!(e.white_material, 1830); // 100 + 330 + 500 + 900.
    assert_eq!(e.black_material, 420); // 100 + 320.
    assert_eq!(e.score_for(Color::White), 1410);
    assert_eq!(e.score_for(Color::Black), -1410);
    assert_eq!(p, before);
}

#[test]
fn moving_side_counters_rights_and_raw_ep_do_not_change_material() {
    let white = position("7k/8/8/8/8/8/8/KQ6 w - - 0 1");
    let black = position("7k/8/8/8/8/8/8/KQ6 b - - 150 999");
    assert_eq!(evaluate(&white), evaluate(&black));
    assert_eq!(evaluate(&white).score_for(white.side_to_move()), 900);
    assert_eq!(evaluate(&black).score_for(black.side_to_move()), -900);
    for fen in [
        "rnbqkbnr/pppp1ppp/8/4p3/8/8/PPPPPPPP/RNBQKBNR w KQkq e6 0 2",
        "rnbqkbnr/pppp1ppp/8/4p3/8/8/PPPPPPPP/RNBQKBNR w - - 0 2",
    ] {
        assert_eq!(evaluate(&position(fen)).score_for(Color::White), 0);
    }
}

// Independently reflect ranks and swap piece colors in a textual FEN. This
// transforms every piece/color, never reuses the evaluator's accumulation loop.
fn color_rank_reflection(p: &Position) -> Position {
    let fen = p.to_fen();
    let fields: Vec<_> = fen.split(' ').collect();
    assert_eq!(fields[2], "-");
    assert_eq!(fields[3], "-");
    let board = fields[0]
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
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/");
    position(&format!(
        "{board} {} - - {} {}",
        if p.side_to_move() == Color::White {
            "b"
        } else {
            "w"
        },
        fields[4],
        fields[5]
    ))
}

#[test]
fn reflected_colors_negate_white_score_and_preserve_moving_side_score() {
    for fen in [
        "6nk/5p2/8/8/8/8/P1B5/KRQ5 w - - 0 1",
        "7k/8/8/8/8/8/8/K7 b - - 0 1",
        "7k/8/8/8/8/8/1q6/K7 w - - 0 1",
    ] {
        let p = position(fen);
        let reflected = color_rank_reflection(&p);
        assert_eq!(color_rank_reflection(&reflected), p);
        assert_eq!(
            evaluate(&reflected).score_for(Color::White),
            -evaluate(&p).score_for(Color::White)
        );
        assert_eq!(
            evaluate(&reflected).score_for(reflected.side_to_move()),
            evaluate(&p).score_for(p.side_to_move())
        );
    }
}

#[test]
fn material_does_not_adjudicate_terminal_diagrams_or_claims() {
    for (fen, class, score) in [
        (
            "7k/6Q1/5K2/8/8/8/8/8 b - - 150 1",
            Classification::Checkmate,
            -900,
        ),
        (
            "7k/5Q2/6K1/8/8/8/8/8 b - - 150 1",
            Classification::Stalemate,
            -900,
        ),
        ("7k/8/8/8/8/8/8/K7 w - - 100 1", Classification::Quiet, 0),
    ] {
        let p = position(fen);
        assert_eq!(p.classify(), class);
        assert_eq!(evaluate(&p).score_for(p.side_to_move()), score);
    }
}

#[test]
fn promotion_contribution_changes_by_the_documented_amount() {
    let mut p = position("7k/P7/8/8/8/8/8/K7 w - - 0 1");
    let before = p.clone();
    let action = mshogi_core::chess::Action {
        from: Square::new(0, 6).unwrap(),
        to: Square::new(0, 7).unwrap(),
        promotion: Some(mshogi_core::chess::Promotion::Queen),
    };
    let undo = p.apply(action).unwrap();
    assert_eq!(evaluate(&p).score_for(Color::White), 900);
    p.undo(&undo).unwrap();
    assert_eq!(p, before);
    assert_eq!(evaluate(&p).score_for(Color::White), 100);
}
