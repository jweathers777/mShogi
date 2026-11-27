//! Offline classical evaluation features in Sente's perspective.
use super::{piece_value, position::TenjikuPosition};
use crate::alphabeta::{SearchPosition, Stage};
use mshogi_core::tenjiku::{Face, Side, Square};

const FACES: usize = Face::ALL.len();
/// Material for each face, four placement terms, and side-to-move tempo.
pub const COUNT: usize = FACES + 5;
/// Weights of the accepted search evaluator.
pub fn initial() -> [i32; COUNT] {
    let mut weights = [0; COUNT];
    for face in Face::ALL {
        weights[face as usize] = piece_value(face);
    }
    weights[FACES..].copy_from_slice(&[4, 2, 6, 1, 10]);
    weights
}

/// Reconstruct the terms of the incremental evaluator from the full board.
pub fn features(position: &TenjikuPosition) -> [i32; COUNT] {
    let mut out = [0; COUNT];
    for at in Square::all() {
        let Some(piece) = position.piece(at) else {
            continue;
        };
        let sign = if piece.side == Side::Sente { 1 } else { -1 };
        let advance = i32::from(if piece.side == Side::Sente {
            16 - at.rank()
        } else {
            at.rank() - 1
        });
        let central = i32::from(7 - at.file().abs_diff(8).min(at.file().abs_diff(9)));
        let face = piece.face();
        out[face as usize] += sign;
        match face {
            Face::K | Face::CP => {}
            Face::P | Face::Dg => out[FACES] += sign * advance,
            Face::Ln | Face::LHk | Face::FiD => {
                out[FACES + 1] += sign * advance.min(10);
                out[FACES + 2] += sign * central;
            }
            _ => out[FACES + 3] += sign * central,
        }
    }
    out[COUNT - 1] = if position.side_to_move() == Side::Sente {
        1
    } else {
        -1
    };
    out
}

/// Search's royal-threat and compound exchange tests exclude unstable boards.
/// Callers also exclude games waiting for a bare-royal reply.
pub fn quiet(position: &TenjikuPosition) -> bool {
    if position.in_check() || position.rule_score().is_some() {
        return false;
    }
    let mut moves = Vec::new();
    position.generate(Stage::Tactical, &mut moves);
    moves.into_iter().all(|m| !position.exchange_at_least(m, 1))
}
