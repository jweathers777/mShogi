//! Offline classical evaluation features in Sente's perspective.
use super::{initial_piece_value, position::TenjikuPosition};
use crate::alphabeta::{SearchPosition, Stage};
use mshogi_core::tenjiku::{Face, Side, Square};

const FACES: usize = Face::ALL.len();
/// Material for each face, four placement terms, and side-to-move tempo.
pub const COUNT: usize = FACES + 5;
pub(super) const PAWN_ADVANCE: usize = FACES;
pub(super) const LION_ADVANCE: usize = FACES + 1;
pub(super) const LION_CENTRAL: usize = FACES + 2;
pub(super) const OTHER_CENTRAL: usize = FACES + 3;
pub(super) const TEMPO: usize = FACES + 4;

/// Playing weights indexed by the 44 movement faces and the five terms above.
pub const DEFAULT: [i32; COUNT] = [
    5000, // K_material
    5000, // FiD_material
    2556, // GGn_material
    2896, // VGn_material
    1384, // RGn_material
    1672, // BGn_material
    2940, // LHk_material
    1968, // Ln_material
    4000, // WBf_material
    2440, // FEg_material
    1820, // FK_material
    1300, // SE_material
    1172, // HF_material
    1300, // ChS_material
    1324, // DK_material
    876,  // DH_material
    546,  // R_material
    350,  // B_material
    696,  // Ph_material
    772,  // VSo_material
    382,  // VM_material
    642,  // SSo_material
    350,  // SM_material
    150,  // RC_material
    150,  // L_material
    984,  // Ky_material
    692,  // G_material
    250,  // DE_material
    200,  // BT_material
    200,  // FL_material
    200,  // S_material
    150,  // C_material
    504,  // I_material
    400,  // N_material
    100,  // Dg_material
    100,  // P_material
    2068, // HT_material
    1004, // FO_material
    1100, // FB_material
    1476, // WH_material
    1188, // W_material
    5000, // CP_material
    900,  // FS_material
    1400, // MGn_material
    0,    // pawn_dog_advance
    32,   // lion_hawk_demon_advance
    32,   // lion_hawk_demon_central
    1,    // other_central
    42,   // tempo
];
/// Frozen weights of the evaluation-round predecessor.
pub fn initial() -> [i32; COUNT] {
    let mut weights = [0; COUNT];
    for face in Face::ALL {
        weights[face as usize] = initial_piece_value(face);
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
            Face::P | Face::Dg => out[PAWN_ADVANCE] += sign * advance,
            Face::Ln | Face::LHk | Face::FiD => {
                out[LION_ADVANCE] += sign * advance.min(10);
                out[LION_CENTRAL] += sign * central;
            }
            _ => out[OTHER_CENTRAL] += sign * central,
        }
    }
    out[TEMPO] = if position.side_to_move() == Side::Sente {
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
