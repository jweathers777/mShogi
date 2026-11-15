//! Bounded classical weights and the exact cached score path used by Texel fitting.
//! Material values here affect evaluation only, not rules, SEE or phase.
use super::*;
use crate::chess_bitboard::PLACEMENT;

/// Number of jointly fitted coefficients.
pub const COUNT: usize = 32;
/// Names in stable parameter order. Percent weights use 100 for the prior value.
pub const NAMES: [&str; COUNT] = [
    "knight",
    "bishop",
    "rook",
    "queen",
    "pawn_placement_mg",
    "pawn_placement_eg",
    "knight_placement_mg",
    "knight_placement_eg",
    "bishop_placement_mg",
    "bishop_placement_eg",
    "rook_placement_mg",
    "rook_placement_eg",
    "queen_placement_mg",
    "queen_placement_eg",
    "king_placement_mg",
    "king_placement_eg",
    "pawns_mg",
    "pawns_eg",
    "mobility_mg",
    "mobility_eg",
    "king_safety_mg",
    "king_safety_eg",
    "threats_mg",
    "threats_eg",
    "conversion_eg",
    "king_pawn_distance",
    "pawnless_minor_scale",
    "pawnless_major_scale",
    "opposite_bishops_scale",
    "low_pawn_scale_base",
    "opposite_bishops_pawn_slope",
    "pawn_scale_slope",
];
/// Integer weights. Construct with values inside LOWER..=UPPER.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Weights(pub [i32; COUNT]);
/// Untuned classical starting point, including both prior endgame terms.
pub const INITIAL: Weights = Weights([
    320, 330, 500, 900, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    100, 100, 100, 100, 100, 100, 8, 8, 16, 24, 36, 2, 7,
]);
/// Lower bounds preserve the pawn/minor/rook/queen hierarchy and nonnegative scale slopes.
pub const LOWER: [i32; COUNT] = [
    280, 290, 450, 800, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25, 25,
    25, 25, 0, 0, 0, 0, 0, 0, 0,
];
/// Upper bounds keep scores finite and materially interpretable.
pub const UPPER: [i32; COUNT] = [
    370, 380, 600, 1100, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200,
    200, 200, 200, 200, 200, 200, 24, 64, 64, 64, 64, 16, 16,
];
/// Compiled weights; replaced only after the training/validation fit freezes.
pub const DEFAULT: Weights = Weights([
    288, 298, 532, 932, 68, 68, 68, 68, 68, 100, 116, 132, 132, 132, 100, 68, 68, 132, 132, 132,
    68, 132, 132, 132, 132, 24, 8, 0, 8, 36, 2, 7,
]);

/// Position-dependent data, independent of fitted coefficients.
#[derive(Clone)]
pub struct Prepared {
    raw: Evaluation,
    counts: [[i32; 5]; 2],
    placement: [[Tapered; 6]; 2],
    nearest: [i32; 2],
    scale_class: [usize; 2],
    bare_conversion: [bool; 2],
}
impl Prepared {
    /// Extract once. Search and offline fitting use the same integer score path.
    pub fn new(p: &ChessPosition) -> Self {
        let raw = raw_decompose(p);
        let counts =
            std::array::from_fn(|c| std::array::from_fn(|k| p.pieces[c][k].count_ones() as i32));
        let mut placement = [[Tapered::default(); 6]; 2];
        for c in 0..2 {
            for k in 0..6 {
                let mut bits = p.pieces[c][k];
                while bits != 0 {
                    let sq = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    let (mg, eg) = PLACEMENT[c][k][sq];
                    placement[c][k].add(mg, eg);
                }
            }
        }
        let pawns = p.pieces[0][PAWN] | p.pieces[1][PAWN];
        let nearest = std::array::from_fn(|c| {
            let king = p.pieces[c][KING].trailing_zeros() as usize;
            let mut bits = pawns;
            let mut best = 7;
            while bits != 0 {
                let sq = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                best = best.min(distance(king, sq));
            }
            if pawns == 0 { 0 } else { best }
        });
        let npm = [
            p.material[0] - 100 * counts[0][PAWN],
            p.material[1] - 100 * counts[1][PAWN],
        ];
        let opposite = counts[0][BISHOP] == 1 && counts[1][BISHOP] == 1 && {
            let a = p.pieces[0][BISHOP].trailing_zeros() as usize;
            let b = p.pieces[1][BISHOP].trailing_zeros() as usize;
            (a / 8 + a % 8) % 2 != (b / 8 + b % 8) % 2
        };
        // 0 full; 1 pawnless minor; 2 pawnless major; 3 pure opposite bishops;
        // 4 low-pawn opposite bishops; 5 other low-pawn material.
        let scale_class = std::array::from_fn(|c| {
            if npm[1 - c] == 0 || npm[c] == 0 {
                return 0;
            }
            if counts[c][PAWN] == 0 {
                return if npm[c] - npm[1 - c] > VALUE[BISHOP] {
                    0
                } else if npm[c] <= VALUE[ROOK] {
                    1
                } else {
                    2
                };
            }
            if opposite && npm == [VALUE[BISHOP]; 2] {
                3
            } else if opposite {
                4
            } else {
                5
            }
        });
        let bare_conversion = std::array::from_fn(|c| {
            (counts[c][ROOK] + counts[c][QUEEN] > 0)
                && p.material[c] - p.material[1 - c] >= 400
                && p.material[1 - c] == 0
        });
        Self {
            raw,
            counts,
            placement,
            nearest,
            scale_class,
            bare_conversion,
        }
    }
    /// Fixed native-material blend phase.
    pub fn phase(&self) -> i32 {
        self.raw.middle_phase
    }
    /// Exact white-relative integer score, including specialized endings and scaling.
    pub fn score(&self, weights: &Weights) -> i32 {
        self.decompose(weights).white_score()
    }
    /// Full weighted decomposition. Specialized recognizers retain their original scores.
    pub fn decompose(&self, weights: &Weights) -> Evaluation {
        let w = &weights.0;
        debug_assert!(
            w.iter()
                .enumerate()
                .all(|(i, &v)| (LOWER[i]..=UPPER[i]).contains(&v)),
            "evaluation weight outside bounds"
        );
        let mut e = self.raw;
        for (c, term) in [&mut e.white, &mut e.black].into_iter().enumerate() {
            term.material = 100 * self.counts[c][PAWN]
                + (1..5).map(|k| self.counts[c][k] * w[k - 1]).sum::<i32>();
            term.placement = Tapered::default();
            for k in 0..6 {
                term.placement.add(
                    self.placement[c][k].middle * w[4 + 2 * k] / 100,
                    self.placement[c][k].end * w[5 + 2 * k] / 100,
                );
            }
            for (j, part) in [
                &mut term.pawns,
                &mut term.mobility,
                &mut term.king_safety,
                &mut term.threats,
            ]
            .into_iter()
            .enumerate()
            {
                part.middle = part.middle * w[16 + 2 * j] / 100;
                part.end = part.end * w[17 + 2 * j] / 100;
            }
            term.endgame.end = if self.bare_conversion[c] {
                // Conversion already rewards approaching the defending king.
                // Cancel the weighted king centrality, not its old coefficient.
                (term.endgame.end + self.placement[c][KING].end) * w[24] / 100
                    - self.placement[c][KING].end * w[15] / 100
            } else {
                term.endgame.end * w[24] / 100
            } - self.nearest[c] * w[25];
            e.endgame_scale[c] = match self.scale_class[c] {
                0 => 64,
                1 => w[26],
                2 => w[27],
                3 => w[28],
                4 => (w[29] + w[30] * self.counts[c][PAWN]).min(64),
                _ => (w[29] + w[31] * self.counts[c][PAWN]).min(64),
            };
        }
        e
    }
}
