//! Classical Chu material and placement, with one shared feature definition.
use super::position::ChuPosition;
use crate::alphabeta::{SearchPosition, Stage};
use mshogi_core::chu::{Face, Piece, Side, Square};

/// Material, advance and central-file weights for each native face, then tempo.
pub const COUNT: usize = 29 * 3 + 1;
/// Starting weights reproduce the MSH-201 evaluation exactly.
pub const INITIAL: [i32; COUNT] = [
    100, 4, 0, 150, 4, 0, 300, 1, 1, 300, 1, 1, 400, 1, 1, 400, 1, 1, 500, 1, 1, 400, 1, 1, 400, 1,
    1, 500, 1, 1, 5000, 0, 0, 600, 2, 4, 600, 2, 4, 700, 1, 1, 700, 1, 1, 900, 1, 1, 700, 1, 1,
    1100, 1, 1, 1100, 1, 1, 2000, 3, 8, 1500, 1, 1, 900, 1, 1, 900, 1, 1, 900, 1, 1, 5000, 0, 0,
    1100, 1, 1, 1100, 1, 1, 1300, 1, 1, 1300, 1, 1, 10,
];
/// Weights used by search. Replaced only after fitting and strength checks.
pub const DEFAULT: [i32; COUNT] = INITIAL;
/// First weight for a face, in native Face::ALL order.
pub const fn index(face: Face) -> usize {
    face as usize * 3
}
/// Placement coordinates; Lion advancement saturates at the old seventh rank.
pub fn coordinates(piece: Piece, at: Square) -> (i32, i32) {
    let advance = if piece.side == Side::Sente {
        12 - at.rank()
    } else {
        at.rank() - 1
    };
    let advance = if piece.is_lion() {
        advance.min(7)
    } else {
        advance
    };
    let file = at.file() - 1;
    (
        i32::from(advance),
        i32::from(5 - file.abs_diff(5).min(file.abs_diff(6))),
    )
}
/// Cached placement contribution, independent of material.
pub fn placement(piece: Piece, at: Square) -> i32 {
    let i = index(piece.face());
    let (advance, central) = coordinates(piece, at);
    DEFAULT[i + 1] * advance + DEFAULT[i + 2] * central
}
/// Dense features in Sente's perspective, including the side-to-move tempo.
pub fn features(position: &ChuPosition) -> [i32; COUNT] {
    let mut out = [0; COUNT];
    for rank in 1..=12 {
        for file in 1..=12 {
            let at = Square::new(file, rank).unwrap();
            if let Some(piece) = position.piece(at) {
                let sign = if piece.side == Side::Sente { 1 } else { -1 };
                let i = index(piece.face());
                let (advance, central) = coordinates(piece, at);
                out[i] += sign;
                out[i + 1] += sign * advance;
                out[i + 2] += sign * central;
            }
        }
    }
    out[COUNT - 1] = if position.side_to_move() == Side::Sente {
        1
    } else {
        -1
    };
    out
}
/// Training positions exclude threats to all royals and positive exchanges.
/// Uncertain exchange results are excluded too, as in search's conservative SEE.
pub fn quiet(position: &ChuPosition) -> bool {
    if position.in_check() {
        return false;
    }
    let mut moves = Vec::new();
    position.generate(Stage::Tactical, &mut moves);
    moves.into_iter().all(|m| !position.exchange_at_least(m, 1))
}
