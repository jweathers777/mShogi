//! Tunable classical Tori evaluation: names, bounds and weights.
//!
//! The evaluation is linear in these integer weights. Each feature is the
//! side-to-move value minus the opponent's (`ToriPosition::features`). Ranks
//! are counted from the owner's back rank (0..7) and columns from the owner's
//! left (0..7); Gote's view is Sente's view turned by half a turn. `INITIAL`
//! reproduces the earlier untuned evaluation exactly: material in hundredths
//! of a Swallow, hand pieces at value plus 10, Swallow advance 4 per rank, 8
//! per own piece next to the Phoenix and a tempo of 10. `DEFAULT` is the
//! compiled weight set.
use super::position::{HAND_FACE, SWALLOW, ToriPosition, VALUE};
use crate::alphabeta::{SearchPosition, Stage};

/// Number of weights.
pub const COUNT: usize = 148;
/// Board values of the faces Falcon..Goose (`Face::ALL` order without the Phoenix).
pub const BOARD: usize = 0;
/// Hand values, in `Face::HANDS` order.
pub const HAND: usize = 8;
/// Rank tables: `RANK + face * 7 + rank`.
pub const RANK: usize = 14;
/// Column tables: `COLUMN + face * 7 + column`.
pub const COLUMN: usize = 77;
/// Own pieces next to the own Phoenix.
pub const PHOENIX_ADJACENT_OWN: usize = 140;
/// Squares next to the own Phoenix attacked by the opponent.
pub const PHOENIX_ADJACENT_ATTACKED: usize = 141;
/// Opponent hand pieces times empty squares next to the own Phoenix.
pub const PHOENIX_DROP_THREAT: usize = 142;
/// Opponent pieces within two squares of the own Phoenix.
pub const PHOENIX_NEAR_ENEMY: usize = 143;
/// Reachable squares (empty or enemy) of the Eagle, Left Quail and Right Quail.
pub const MOBILITY: usize = 144;
/// Side to move.
pub const TEMPO: usize = 147;
/// First per-node term; the terms before it are static per piece.
pub const PER_NODE: usize = PHOENIX_ADJACENT_OWN;

/// Weight names in stable order.
pub const NAMES: [&str; COUNT] = [
    "board_falcon",
    "board_eagle",
    "board_crane",
    "board_pheasant",
    "board_left_quail",
    "board_right_quail",
    "board_swallow",
    "board_goose",
    "hand_falcon",
    "hand_crane",
    "hand_pheasant",
    "hand_left_quail",
    "hand_right_quail",
    "hand_swallow",
    "phoenix_rank0",
    "phoenix_rank1",
    "phoenix_rank2",
    "phoenix_rank3",
    "phoenix_rank4",
    "phoenix_rank5",
    "phoenix_rank6",
    "falcon_rank0",
    "falcon_rank1",
    "falcon_rank2",
    "falcon_rank3",
    "falcon_rank4",
    "falcon_rank5",
    "falcon_rank6",
    "eagle_rank0",
    "eagle_rank1",
    "eagle_rank2",
    "eagle_rank3",
    "eagle_rank4",
    "eagle_rank5",
    "eagle_rank6",
    "crane_rank0",
    "crane_rank1",
    "crane_rank2",
    "crane_rank3",
    "crane_rank4",
    "crane_rank5",
    "crane_rank6",
    "pheasant_rank0",
    "pheasant_rank1",
    "pheasant_rank2",
    "pheasant_rank3",
    "pheasant_rank4",
    "pheasant_rank5",
    "pheasant_rank6",
    "left_quail_rank0",
    "left_quail_rank1",
    "left_quail_rank2",
    "left_quail_rank3",
    "left_quail_rank4",
    "left_quail_rank5",
    "left_quail_rank6",
    "right_quail_rank0",
    "right_quail_rank1",
    "right_quail_rank2",
    "right_quail_rank3",
    "right_quail_rank4",
    "right_quail_rank5",
    "right_quail_rank6",
    "swallow_rank0",
    "swallow_rank1",
    "swallow_rank2",
    "swallow_rank3",
    "swallow_rank4",
    "swallow_rank5",
    "swallow_rank6",
    "goose_rank0",
    "goose_rank1",
    "goose_rank2",
    "goose_rank3",
    "goose_rank4",
    "goose_rank5",
    "goose_rank6",
    "phoenix_column0",
    "phoenix_column1",
    "phoenix_column2",
    "phoenix_column3",
    "phoenix_column4",
    "phoenix_column5",
    "phoenix_column6",
    "falcon_column0",
    "falcon_column1",
    "falcon_column2",
    "falcon_column3",
    "falcon_column4",
    "falcon_column5",
    "falcon_column6",
    "eagle_column0",
    "eagle_column1",
    "eagle_column2",
    "eagle_column3",
    "eagle_column4",
    "eagle_column5",
    "eagle_column6",
    "crane_column0",
    "crane_column1",
    "crane_column2",
    "crane_column3",
    "crane_column4",
    "crane_column5",
    "crane_column6",
    "pheasant_column0",
    "pheasant_column1",
    "pheasant_column2",
    "pheasant_column3",
    "pheasant_column4",
    "pheasant_column5",
    "pheasant_column6",
    "left_quail_column0",
    "left_quail_column1",
    "left_quail_column2",
    "left_quail_column3",
    "left_quail_column4",
    "left_quail_column5",
    "left_quail_column6",
    "right_quail_column0",
    "right_quail_column1",
    "right_quail_column2",
    "right_quail_column3",
    "right_quail_column4",
    "right_quail_column5",
    "right_quail_column6",
    "swallow_column0",
    "swallow_column1",
    "swallow_column2",
    "swallow_column3",
    "swallow_column4",
    "swallow_column5",
    "swallow_column6",
    "goose_column0",
    "goose_column1",
    "goose_column2",
    "goose_column3",
    "goose_column4",
    "goose_column5",
    "goose_column6",
    "phoenix_adjacent_own",
    "phoenix_adjacent_attacked",
    "phoenix_drop_threat",
    "phoenix_near_enemy",
    "mobility_eagle",
    "mobility_left_quail",
    "mobility_right_quail",
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
    while i < 6 {
        w[HAND + i] = VALUE[HAND_FACE[i]] + 10;
        i += 1;
    }
    let mut rank = 0;
    while rank < 7 {
        w[RANK + SWALLOW * 7 + rank] = 4 * rank as i32;
        rank += 1;
    }
    w[PHOENIX_ADJACENT_OWN] = 8;
    w[TEMPO] = 10;
    w
}

/// Per-node terms are fitted only when this is true (as in MSH-196, they
/// were fitted and tested separately from the static terms; MSH-206 kept
/// them). Otherwise they stay at `INITIAL`.
const PER_NODE_FITTED: bool = true;

const fn bounds(upper: bool) -> [i32; COUNT] {
    let init = initial();
    let mut b = [0; COUNT];
    let mut i = 0;
    while i < COUNT {
        b[i] = if i < RANK {
            if upper { init[i] * 8 / 5 } else { init[i] / 2 }
        } else if i < PER_NODE {
            if upper { 150 } else { -150 }
        } else if i == TEMPO {
            if upper { 60 } else { 0 }
        } else if !PER_NODE_FITTED {
            init[i]
        } else if i < MOBILITY {
            if upper { 100 } else { -100 }
        } else if upper {
            30
        } else {
            0
        };
        i += 1;
    }
    b
}

/// Untuned weights: exactly the earlier evaluation.
pub const INITIAL: [i32; COUNT] = initial();
/// Inclusive lower bounds.
pub const LOWER: [i32; COUNT] = bounds(false);
/// Inclusive upper bounds.
pub const UPPER: [i32; COUNT] = bounds(true);
/// Compiled weights used by the search: the MSH-206 (Tori Evaluation Round)
/// fit2, static and per-node terms.
pub const DEFAULT: [i32; COUNT] = [
    636, 1136, 488, 204, 404, 372, 100, 220, 934, 634, 358, 486, 518, 118, -150, 0, 104, -120, -24,
    150, -150, -96, -40, 32, 48, 150, 150, 150, -128, -128, 24, 150, 0, -48, -16, -134, 0, 48, 120,
    72, 72, -8, -96, -16, 48, 104, 24, 64, 80, -64, -80, -8, -8, 134, -120, -56, -64, -16, -64, 56,
    150, -112, 72, 24, -44, 0, 12, 24, 12, 24, 150, 8, 150, 128, 150, -144, -96, -150, 24, 96, 150,
    150, 150, 32, -80, 56, 96, 64, 48, 120, -150, -150, -96, -150, 150, 16, 150, -24, -72, 72, 150,
    136, 136, 24, -150, -48, -32, 0, 40, -32, -16, -134, -24, -8, 16, 150, 150, 72, -150, -56, -24,
    -24, 0, 0, 80, -64, -80, -40, 56, 72, 16, -32, -16, 8, 32, 48, 128, 40, 136, 80, 8, -64, 0,
    -24, 30, 16, 30, 8,
];

/// Owner-relative rank and column of a square (native index `row * 7 + column`,
/// Sente moves toward row 0).
pub const fn rank_column(side: usize, square: usize) -> (usize, usize) {
    let (row, column) = (square / 7, square % 7);
    if side == 0 {
        (6 - row, column)
    } else {
        (row, 6 - column)
    }
}

/// Static weight indices of a piece: its board value (none for the
/// Phoenix), its rank and its column.
pub const fn piece_indices(side: usize, face: usize, square: usize) -> [Option<usize>; 3] {
    let (rank, column) = rank_column(side, square);
    [
        if face == 0 {
            None
        } else {
            Some(BOARD + face - 1)
        },
        Some(RANK + face * 7 + rank),
        Some(COLUMN + face * 7 + column),
    ]
}

/// Static value of a piece with a weight vector.
pub const fn piece_score(weights: &[i32; COUNT], side: usize, face: usize, square: usize) -> i32 {
    let indices = piece_indices(side, face, square);
    let mut total = 0;
    let mut i = 0;
    while i < 3 {
        if let Some(index) = indices[i] {
            total += weights[index];
        }
        i += 1;
    }
    total
}

/// Score from the side to move with a weight vector.
pub fn score(position: &ToriPosition, weights: &[i32; COUNT]) -> i32 {
    let mut total = 0;
    position.features(&mut |index, value| total += weights[index] * value);
    total
}

/// Quiet for fitting: not in check and no capture or promotion that wins
/// material by static exchange.
pub fn quiet(position: &ToriPosition) -> bool {
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
    in_hand: u8,
}

/// Tori model of the shared Texel tool. Positions are compact diagrams.
pub struct ToriTexel;
impl crate::texel::TexelModel for ToriTexel {
    type Prepared = Prepared;
    const KIND: &'static str = "tori";
    const POSITION_FIELD: &'static str = "diagram";
    // Drops keep long games open, so early scores predict outcomes weakly.
    const K_RANGE: (f64, f64) = (0.01, 0.71);
    const STEPS: &'static [i32] = &[32, 16, 8, 4, 2, 1];
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
        // A compact diagram has no counters: board, hands and side only.
        position.to_string()
    }
    fn prepare(position: &str) -> Result<Option<Prepared>, String> {
        let native = mshogi_core::tori::Position::from_compact_diagram_text(position)?;
        let p = ToriPosition::from_position(&native);
        if !quiet(&p) {
            return Ok(None);
        }
        let sign = if p.side_to_move() == mshogi_core::tori::Side::Sente {
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
        Ok(Some(Prepared {
            features,
            in_hand: p.pieces_in_hand(),
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
        &["in_hand_0_2", "in_hand_3_6", "in_hand_7_plus"]
    }
    fn bucket(prepared: &Prepared) -> usize {
        match prepared.in_hand {
            0..=2 => 0,
            3..=6 => 1,
            _ => 2,
        }
    }
}
