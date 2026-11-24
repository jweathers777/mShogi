//! Classical Chu material and placement, with one shared feature definition.
use super::position::ChuPosition;
use crate::alphabeta::{SearchPosition, Stage};
use mshogi_core::chu::{Face, Piece, Side, Square};

/// Material, advance and central-file weights for each native face, then tempo.
pub const COUNT: usize = 29 * 3 + 1;
/// Starting weights reproduce the MSH-201 evaluation exactly.
pub const INITIAL: [i32; COUNT] = [
    100, 4, 0, // Pawn
    150, 4, 0, // Go-between
    300, 1, 1, // Lance
    300, 1, 1, // Reverse Chariot
    400, 1, 1, // Copper
    400, 1, 1, // Silver
    500, 1, 1, // Gold
    400, 1, 1, // Leopard
    400, 1, 1, // Tiger
    500, 1, 1, // Elephant
    5000, 0, 0, // King
    600, 2, 4, // Kirin
    600, 2, 4, // Phoenix
    700, 1, 1, // Side Mover
    700, 1, 1, // Vertical Mover
    900, 1, 1, // Rook
    700, 1, 1, // Bishop
    1100, 1, 1, // Dragon Horse
    1100, 1, 1, // Dragon King
    2000, 3, 8, // Lion
    1500, 1, 1, // Free King
    900, 1, 1, // White Horse
    900, 1, 1, // Whale
    900, 1, 1, // Flying Stag
    5000, 0, 0, // Prince
    1100, 1, 1, // Flying Ox
    1100, 1, 1, // Free Boar
    1300, 1, 1, // Soaring Eagle
    1300, 1, 1,  // Horned Falcon
    10, // Tempo
];
/// Fitted static Texel weights from the native MSH-52.2 corpus.
pub const DEFAULT: [i32; COUNT] = [
    100, 4, 0, // Pawn
    75, 4, 0, // Go-between
    428, -32, 1, // Lance
    492, 32, 1, // Reverse Chariot
    200, 32, -32, // Copper
    208, 32, 1, // Silver
    372, 1, 1, // Gold
    200, 32, 1, // Leopard
    336, 32, 1, // Tiger
    564, 1, 1, // Elephant
    5000, -32, 32, // King
    600, 2, 4, // Kirin
    600, 32, 4, // Phoenix
    572, 1, 1, // Side Mover
    700, 1, 32, // Vertical Mover
    964, 1, 1, // Rook
    700, 1, 1, // Bishop
    1164, -32, 1, // Dragon Horse
    1228, 1, 1, // Dragon King
    2256, 3, 32, // Lion
    1564, 32, 32, // Free King
    836, -32, 32, // White Horse
    1028, 1, 1, // Whale
    964, 32, 32, // Flying Stag
    5000, -32, -32, // Prince
    1164, 1, 32, // Flying Ox
    1356, 32, 32, // Free Boar
    1556, 32, 32, // Soaring Eagle
    1428, 32, 1, // Horned Falcon
    0, // Tempo
];
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
