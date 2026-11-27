//! Tunable classical Shogi evaluation: names, bounds and weights.
//!
//! The evaluation is linear in these integer weights. Each feature is the
//! side-to-move value minus the opponent's (`ShogiPosition::features`).
//! `INITIAL` reproduces the earlier untuned evaluation exactly: material,
//! hand pieces at value plus 10, Pawn advance 4 per rank, 8 per own piece
//! next to the king and a tempo of 10. `DEFAULT` is the compiled weight set.
use super::position::{HAND_FACE, ShogiPosition, VALUE};
use crate::alphabeta::{SearchPosition, Stage};

/// Number of weights.
pub const COUNT: usize = 133;
/// First index of the board values (faces rook..tokin).
pub const BOARD: usize = 0;
/// First index of the hand values by piece count.
pub const HAND: usize = 13;
/// First index of the bonus for holding at least one piece of a kind.
pub const HAND_FIRST: usize = 20;
/// First index of the rank tables (group * 9 + advanced ranks).
pub const PST: usize = 27;
/// First index of the king's advanced rank.
pub const KING_RANK: usize = 108;
/// First index of the king's file distance from the centre file.
pub const KING_FILE: usize = 117;
/// First index of slider mobility (lance, bishop, rook, horse, dragon).
pub const MOBILITY: usize = 122;
/// Own pieces next to the king.
pub const KING_ADJACENT_OWN: usize = 127;
/// Squares next to the king attacked by the opponent.
pub const KING_ADJACENT_ATTACKED: usize = 128;
/// Opponent pieces within two squares of the king.
pub const KING_NEAR_ENEMY: usize = 129;
/// Own Golds, promoted minors and Silvers within two squares of the king.
pub const KING_NEAR_DEFENDERS: usize = 130;
/// Opponent non-Pawn hand pieces times empty squares next to the king.
pub const KING_DROP_THREAT: usize = 131;
/// Side to move.
pub const TEMPO: usize = 132;

/// Weight names in stable order.
pub const NAMES: [&str; COUNT] = [
    "board_rook",
    "board_bishop",
    "board_gold",
    "board_silver",
    "board_knight",
    "board_lance",
    "board_pawn",
    "board_dragon",
    "board_horse",
    "board_promoted_silver",
    "board_promoted_knight",
    "board_promoted_lance",
    "board_tokin",
    "hand_rook",
    "hand_bishop",
    "hand_gold",
    "hand_silver",
    "hand_knight",
    "hand_lance",
    "hand_pawn",
    "hand_first_rook",
    "hand_first_bishop",
    "hand_first_gold",
    "hand_first_silver",
    "hand_first_knight",
    "hand_first_lance",
    "hand_first_pawn",
    "pst_pawn_rank0",
    "pst_pawn_rank1",
    "pst_pawn_rank2",
    "pst_pawn_rank3",
    "pst_pawn_rank4",
    "pst_pawn_rank5",
    "pst_pawn_rank6",
    "pst_pawn_rank7",
    "pst_pawn_rank8",
    "pst_lance_rank0",
    "pst_lance_rank1",
    "pst_lance_rank2",
    "pst_lance_rank3",
    "pst_lance_rank4",
    "pst_lance_rank5",
    "pst_lance_rank6",
    "pst_lance_rank7",
    "pst_lance_rank8",
    "pst_knight_rank0",
    "pst_knight_rank1",
    "pst_knight_rank2",
    "pst_knight_rank3",
    "pst_knight_rank4",
    "pst_knight_rank5",
    "pst_knight_rank6",
    "pst_knight_rank7",
    "pst_knight_rank8",
    "pst_silver_rank0",
    "pst_silver_rank1",
    "pst_silver_rank2",
    "pst_silver_rank3",
    "pst_silver_rank4",
    "pst_silver_rank5",
    "pst_silver_rank6",
    "pst_silver_rank7",
    "pst_silver_rank8",
    "pst_goldlike_rank0",
    "pst_goldlike_rank1",
    "pst_goldlike_rank2",
    "pst_goldlike_rank3",
    "pst_goldlike_rank4",
    "pst_goldlike_rank5",
    "pst_goldlike_rank6",
    "pst_goldlike_rank7",
    "pst_goldlike_rank8",
    "pst_bishop_rank0",
    "pst_bishop_rank1",
    "pst_bishop_rank2",
    "pst_bishop_rank3",
    "pst_bishop_rank4",
    "pst_bishop_rank5",
    "pst_bishop_rank6",
    "pst_bishop_rank7",
    "pst_bishop_rank8",
    "pst_rook_rank0",
    "pst_rook_rank1",
    "pst_rook_rank2",
    "pst_rook_rank3",
    "pst_rook_rank4",
    "pst_rook_rank5",
    "pst_rook_rank6",
    "pst_rook_rank7",
    "pst_rook_rank8",
    "pst_horse_rank0",
    "pst_horse_rank1",
    "pst_horse_rank2",
    "pst_horse_rank3",
    "pst_horse_rank4",
    "pst_horse_rank5",
    "pst_horse_rank6",
    "pst_horse_rank7",
    "pst_horse_rank8",
    "pst_dragon_rank0",
    "pst_dragon_rank1",
    "pst_dragon_rank2",
    "pst_dragon_rank3",
    "pst_dragon_rank4",
    "pst_dragon_rank5",
    "pst_dragon_rank6",
    "pst_dragon_rank7",
    "pst_dragon_rank8",
    "king_rank0",
    "king_rank1",
    "king_rank2",
    "king_rank3",
    "king_rank4",
    "king_rank5",
    "king_rank6",
    "king_rank7",
    "king_rank8",
    "king_file_distance0",
    "king_file_distance1",
    "king_file_distance2",
    "king_file_distance3",
    "king_file_distance4",
    "mobility_lance",
    "mobility_bishop",
    "mobility_rook",
    "mobility_horse",
    "mobility_dragon",
    "king_adjacent_own",
    "king_adjacent_attacked",
    "king_near_enemy",
    "king_near_defenders",
    "king_drop_threat",
    "tempo",
];

const fn initial() -> [i32; COUNT] {
    let mut w = [0; COUNT];
    let mut face = 1;
    while face < VALUE.len() {
        w[BOARD + face - 1] = VALUE[face];
        face += 1;
    }
    let mut i = 0;
    while i < HAND_FACE.len() {
        w[HAND + i] = VALUE[HAND_FACE[i]] + 10;
        i += 1;
    }
    // The Pawn rank table is the first one.
    let mut rank = 0;
    while rank < 9 {
        w[PST + rank] = 4 * rank as i32;
        rank += 1;
    }
    w[KING_ADJACENT_OWN] = 8;
    w[TEMPO] = 10;
    w
}
/// Untuned weights: exactly the earlier evaluation.
pub const INITIAL: [i32; COUNT] = initial();
/// Inclusive lower bounds.
pub const LOWER: [i32; COUNT] = [
    400, 350, 250, 200, 150, 150, 50, 500, 450, 250, 250, 250, 250, 405, 355, 255, 205, 155, 155,
    55, -200, -200, -200, -200, -200, -200, -200, -150, -150, -150, -150, -150, -150, -150, -150,
    -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150,
    -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150,
    -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150,
    -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150, -150,
    -150, -150, -150, -150, -150, -150, -150, -150, -150, -200, -200, -200, -200, -200, -200, -200,
    -200, -200, -200, -200, -200, -200, -200, -10, -10, -10, -10, -10, -20, -80, -80, -20, -40, 0,
];
/// Inclusive upper bounds.
pub const UPPER: [i32; COUNT] = [
    1280, 1120, 800, 640, 480, 480, 160, 1600, 1440, 800, 800, 800, 800, 1458, 1278, 918, 738, 558,
    558, 198, 300, 300, 300, 300, 300, 300, 300, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150,
    150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150,
    150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150,
    150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150,
    150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 150, 200, 200, 200, 200, 200,
    200, 200, 200, 200, 200, 200, 200, 200, 200, 30, 30, 30, 30, 30, 60, 20, 20, 80, 10, 80,
];
/// Compiled weights used by the search.
pub const DEFAULT: [i32; COUNT] = [
    672, 732, 532, 432, 236, 204, 132, 1096, 996, 404, 436, 308, 404, 842, 742, 574, 346, 246, 246,
    55, 32, 0, 64, 160, 64, 32, 96, 96, -92, -56, -20, -16, 150, -40, -150, 32, -96, 0, -64, -64,
    -96, -32, -150, -150, 0, 0, -150, 0, 32, 64, 128, -150, 0, 0, -96, 0, 0, 32, 96, 64, 64, 64,
    150, -96, 64, 32, -64, 96, 96, 64, 64, -150, -128, -32, -32, 32, 64, -96, 150, 150, -96, -96,
    -96, -64, 0, -150, -32, 96, 150, 150, -150, 0, 150, 32, 96, 64, -32, 32, -32, -32, 32, -32,
    -96, -96, 32, 0, 150, 64, -200, -64, -32, 0, 160, 200, 200, 200, 200, -64, -200, 0, 160, -160,
    30, 0, 30, 30, 30, 60, -80, -80, 80, -32, 0,
];

/// Score from the side to move with a weight vector.
pub fn score(position: &ShogiPosition, weights: &[i32; COUNT]) -> i32 {
    let mut total = 0;
    position.features(&mut |index, value| total += weights[index] * value);
    total
}

/// Quiet for fitting: not in check and no capture or promotion that wins
/// material by static exchange.
pub fn quiet(position: &ShogiPosition) -> bool {
    if position.in_check() {
        return false;
    }
    let mut moves = Vec::new();
    position.generate(Stage::Tactical, &mut moves);
    moves.into_iter().all(|m| position.see(m) <= 0)
}

/// Sparse features from Sente's view, for fitting.
pub struct Prepared {
    features: Vec<(u16, i16)>,
    hand_pieces: u8,
}

/// Shogi model of the shared Texel tool.
pub struct ShogiTexel;
impl crate::texel::TexelModel for ShogiTexel {
    type Prepared = Prepared;
    const KIND: &'static str = "shogi";
    const POSITION_FIELD: &'static str = "sfen";
    const STEPS: &'static [i32] = &[32, 16, 8, 4, 2, 1];
    // Shogi material swings are larger than Chess outcomes per point.
    const K_RANGE: (f64, f64) = (0.01, 0.71);
    fn names() -> &'static [&'static str] {
        &NAMES
    }
    fn initial() -> Vec<i32> {
        INITIAL.to_vec()
    }
    fn lower() -> Vec<i32> {
        LOWER.to_vec()
    }
    fn upper() -> Vec<i32> {
        UPPER.to_vec()
    }
    fn key(position: &str) -> String {
        // Board, side and hands; the fourth SFEN field is only the move counter.
        position
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()
            .join(" ")
    }
    fn prepare(position: &str) -> Result<Option<Prepared>, String> {
        let rules_position =
            mshogi_core::shogi::Position::from_sfen(position).map_err(|e| format!("{e:?}"))?;
        let p = ShogiPosition::from_position(&rules_position);
        if !quiet(&p) {
            return Ok(None);
        }
        let sign = if p.side_to_move() == mshogi_core::shogi::Side::Sente {
            1
        } else {
            -1
        };
        let mut dense = [0i32; COUNT];
        p.features(&mut |index, value| dense[index] += value);
        let features = dense
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v != 0)
            .map(|(i, &v)| (i as u16, (sign * v) as i16))
            .collect();
        let hand_pieces = mshogi_core::shogi::PieceKind::HAND_KINDS
            .iter()
            .map(|&k| {
                rules_position
                    .hand(mshogi_core::shogi::Side::Sente)
                    .count(k)
                    + rules_position.hand(mshogi_core::shogi::Side::Gote).count(k)
            })
            .sum();
        Ok(Some(Prepared {
            features,
            hand_pieces,
        }))
    }
    fn score(prepared: &Prepared, weights: &[i32]) -> i32 {
        prepared
            .features
            .iter()
            .map(|&(i, v)| weights[usize::from(i)] * i32::from(v))
            .sum()
    }
    fn buckets() -> &'static [&'static str] {
        &["hands_0_2", "hands_3_7", "hands_8_or_more"]
    }
    fn bucket(prepared: &Prepared) -> usize {
        match prepared.hand_pieces {
            0..=2 => 0,
            3..=7 => 1,
            _ => 2,
        }
    }
}
