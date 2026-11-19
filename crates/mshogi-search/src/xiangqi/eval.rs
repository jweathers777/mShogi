//! Tunable classical Xiangqi evaluation: names, bounds and weights.
//!
//! The evaluation is linear in these integer weights. Each feature is the
//! side-to-move value minus the opponent's (`XiangqiPosition::features`).
//! Ranks are counted from the owner's back rank (0..9) and files by distance
//! from the centre file (0..4). `INITIAL` reproduces the earlier untuned
//! evaluation exactly: material, Soldier advance 20 per rank beyond its home
//! rank and 8 per central file step after crossing, Horse 4 per rank (to 6)
//! and 6 per central step, Chariot 2 per rank and 3 per central step, Cannon
//! 2 per rank (to 6) and 4 per central step, and a tempo of 10. `DEFAULT` is
//! the compiled weight set.
use super::position::XiangqiPosition;
use crate::alphabeta::{SearchPosition, Stage};

/// Number of weights.
pub const COUNT: usize = 80;
/// Material of Advisor, Elephant, Horse, Chariot, Cannon, Soldier.
pub const MATERIAL: usize = 0;
/// Soldier rank table (10).
pub const SOLDIER_RANK: usize = 6;
/// Soldier file distance table after crossing the river (5).
pub const SOLDIER_FILE: usize = 16;
/// Horse rank table (10).
pub const HORSE_RANK: usize = 21;
/// Horse file distance table (5).
pub const HORSE_FILE: usize = 31;
/// Chariot rank table (10).
pub const ROOK_RANK: usize = 36;
/// Chariot file distance table (5).
pub const ROOK_FILE: usize = 46;
/// Cannon rank table (10).
pub const CANNON_RANK: usize = 51;
/// Cannon file distance table (5).
pub const CANNON_FILE: usize = 61;
/// General on its second rank.
pub const GENERAL_RANK1: usize = 66;
/// General on its third rank.
pub const GENERAL_RANK2: usize = 67;
/// General off the centre file.
pub const GENERAL_OFF_FILE: usize = 68;
/// Advisor on the palace centre.
pub const ADVISOR_CENTER: usize = 69;
/// Elephant on its central square.
pub const ELEPHANT_CENTER: usize = 70;
/// Horse reachable squares (empty or enemy).
pub const MOBILITY_HORSE: usize = 71;
/// Chariot reachable squares (empty or enemy).
pub const MOBILITY_ROOK: usize = 72;
/// Cannon quiet moves plus captures.
pub const MOBILITY_CANNON: usize = 73;
/// Own Horses times all Chariots, Horses and Cannons on the board.
pub const HORSE_PHASE: usize = 74;
/// Own Cannons times all Chariots, Horses and Cannons on the board.
pub const CANNON_PHASE: usize = 75;
/// Enemy Chariots on the own General's file.
pub const ENEMY_ROOK_FILE: usize = 76;
/// Enemy Cannons on the own General's file.
pub const ENEMY_CANNON_FILE: usize = 77;
/// Palace squares next to the own General attacked by the opponent.
pub const GENERAL_ADJACENT_ATTACKED: usize = 78;
/// Side to move.
pub const TEMPO: usize = 79;

/// Weight names in stable order.
pub const NAMES: [&str; COUNT] = [
    "material_advisor",
    "material_elephant",
    "material_horse",
    "material_rook",
    "material_cannon",
    "material_soldier",
    "soldier_rank0",
    "soldier_rank1",
    "soldier_rank2",
    "soldier_rank3",
    "soldier_rank4",
    "soldier_rank5",
    "soldier_rank6",
    "soldier_rank7",
    "soldier_rank8",
    "soldier_rank9",
    "soldier_crossed_file0",
    "soldier_crossed_file1",
    "soldier_crossed_file2",
    "soldier_crossed_file3",
    "soldier_crossed_file4",
    "horse_rank0",
    "horse_rank1",
    "horse_rank2",
    "horse_rank3",
    "horse_rank4",
    "horse_rank5",
    "horse_rank6",
    "horse_rank7",
    "horse_rank8",
    "horse_rank9",
    "horse_file0",
    "horse_file1",
    "horse_file2",
    "horse_file3",
    "horse_file4",
    "rook_rank0",
    "rook_rank1",
    "rook_rank2",
    "rook_rank3",
    "rook_rank4",
    "rook_rank5",
    "rook_rank6",
    "rook_rank7",
    "rook_rank8",
    "rook_rank9",
    "rook_file0",
    "rook_file1",
    "rook_file2",
    "rook_file3",
    "rook_file4",
    "cannon_rank0",
    "cannon_rank1",
    "cannon_rank2",
    "cannon_rank3",
    "cannon_rank4",
    "cannon_rank5",
    "cannon_rank6",
    "cannon_rank7",
    "cannon_rank8",
    "cannon_rank9",
    "cannon_file0",
    "cannon_file1",
    "cannon_file2",
    "cannon_file3",
    "cannon_file4",
    "general_rank1",
    "general_rank2",
    "general_off_file",
    "advisor_center",
    "elephant_center",
    "mobility_horse",
    "mobility_rook",
    "mobility_cannon",
    "horse_phase",
    "cannon_phase",
    "enemy_rook_file",
    "enemy_cannon_file",
    "general_adjacent_attacked",
    "tempo",
];

const fn initial() -> [i32; COUNT] {
    let mut w = [0; COUNT];
    let material = [200, 200, 400, 900, 450, 100];
    let mut i = 0;
    while i < 6 {
        w[MATERIAL + i] = material[i];
        i += 1;
    }
    let mut r = 0;
    while r < 10 {
        w[SOLDIER_RANK + r] = if r > 3 { 20 * (r as i32 - 3) } else { 0 };
        w[HORSE_RANK + r] = 4 * if r < 6 { r as i32 } else { 6 };
        w[ROOK_RANK + r] = 2 * r as i32;
        w[CANNON_RANK + r] = 2 * if r < 6 { r as i32 } else { 6 };
        r += 1;
    }
    let mut d = 0;
    while d < 5 {
        let central = 4 - d as i32;
        w[SOLDIER_FILE + d] = 8 * central;
        w[HORSE_FILE + d] = 6 * central;
        w[ROOK_FILE + d] = 3 * central;
        w[CANNON_FILE + d] = 4 * central;
        d += 1;
    }
    w[TEMPO] = 10;
    w
}

const fn bounds(upper: bool) -> [i32; COUNT] {
    let init = initial();
    let mut b = [0; COUNT];
    let mut i = 0;
    while i < COUNT {
        b[i] = if i < 2 {
            // Advisor and Elephant: the first fit (MSH-196) put both at a
            // lower bound of 100 and lost its game test; keep them at least
            // three quarters of their classical value.
            if upper {
                init[i] * 8 / 5
            } else {
                init[i] * 3 / 4
            }
        } else if i < 6 {
            if upper { init[i] * 8 / 5 } else { init[i] / 2 }
        } else if i < GENERAL_RANK1 {
            if upper { 150 } else { -150 }
        } else if i < MOBILITY_HORSE {
            if upper { 100 } else { -100 }
        } else if i < TEMPO {
            // Per-node terms stay at zero: two fits that used them (Chariot
            // mobility and phase terms at their bounds) lost their game
            // tests (MSH-196). Only static terms and the tempo are tuned.
            0
        } else if upper {
            60
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
/// Compiled weights used by the search: the MSH-196 (Xiangqi Evaluation
/// Round) static-term fit, docs/m5/evidence/msh196/fit3.json.
pub const DEFAULT: [i32; COUNT] = [
    150, 150, 512, 1440, 498, 108, 0, 0, 0, -16, 20, 112, 126, 150, 142, -32, -32, -8, 40, 16, -56,
    -48, -44, 56, 142, 120, 132, 120, 120, 120, 134, 24, 90, 60, 62, -8, 48, 10, 76, 134, 150, 150,
    150, 150, 134, 142, 150, 150, 118, 118, 72, 112, 74, 148, 134, 128, 90, 132, 92, 116, 84, 48,
    12, 24, 36, 24, 0, -32, -24, -16, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// Score from the side to move with a weight vector.
pub fn score(position: &XiangqiPosition, weights: &[i32; COUNT]) -> i32 {
    let mut total = 0;
    position.features(&mut |index, value| total += weights[index] * value);
    total
}

/// Quiet for fitting: not in check and no capture that wins material by
/// static exchange.
pub fn quiet(position: &XiangqiPosition) -> bool {
    if position.in_check() {
        return false;
    }
    let mut moves = Vec::new();
    position.generate(Stage::Tactical, &mut moves);
    moves.into_iter().all(|m| position.see(m) <= 0)
}

/// Sparse features from Red's view, for fitting.
pub struct Prepared {
    features: Vec<(u16, i16)>,
    majors: u8,
}

/// Xiangqi model of the shared Texel tool.
pub struct XiangqiTexel;
impl crate::texel::TexelModel for XiangqiTexel {
    type Prepared = Prepared;
    const KIND: &'static str = "xiangqi";
    const POSITION_FIELD: &'static str = "fen";
    const STEPS: &'static [i32] = &[32, 16, 8, 4, 2, 1];
    // Most self-play games are drawn, so outcomes depend weakly on score.
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
        // Board and side; counters do not change the evaluation.
        position
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()
            .join(" ")
    }
    fn prepare(position: &str) -> Result<Option<Prepared>, String> {
        let native =
            mshogi_core::xiangqi::PositionRoot::from_fen(position).map_err(|e| format!("{e:?}"))?;
        let p = XiangqiPosition::from_position(native.position());
        if !quiet(&p) {
            return Ok(None);
        }
        let sign = if p.side_to_move() == mshogi_core::xiangqi::Side::Red {
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
            majors: p.majors(),
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
        &["majors_0_4", "majors_5_8", "majors_9_12"]
    }
    fn bucket(prepared: &Prepared) -> usize {
        match prepared.majors {
            0..=4 => 0,
            5..=8 => 1,
            _ => 2,
        }
    }
}
