//! Material-only evaluation v1. No positional terms, tempo, training or strength claim.

use mshogi_core::chess::{Color, PieceKind, Position, Square};

/// Identity to record alongside search/experiment settings.
pub const EVALUATOR_ID: &str = "mshogi.chess.material.v1";

/// Transparent untrained centipawn values. Kings contribute zero: their capture
/// is illegal and terminal scores belong to search using rules adjudication.
/// These are fixed baseline parameters, not measured exchange values.
pub const fn piece_value(kind: PieceKind) -> i32 {
    match kind {
        PieceKind::Pawn => 100,
        PieceKind::Knight => 320,
        PieceKind::Bishop => 330,
        PieceKind::Rook => 500,
        PieceKind::Queen => 900,
        PieceKind::King => 0,
    }
}

/// Material contributions, in centipawns, with no outcome or mate-score encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Evaluation {
    /// Sum of White's piece values, excluding its king.
    pub white_material: i32,
    /// Sum of Black's piece values, excluding its king.
    pub black_material: i32,
}

impl Evaluation {
    /// Positive favors `side`; swapping perspective negates the score exactly.
    pub const fn score_for(self, side: Color) -> i32 {
        match side {
            Color::White => self.white_material - self.black_material,
            Color::Black => self.black_material - self.white_material,
        }
    }
}

/// Evaluate material only, without generating moves or examining history,
/// counters, claims, clocks or terminal classification. Validated position piece
/// bounds keep these sums well inside i32. Deterministic for any given position.
///
/// A score of zero is an equal material assessment, **not** a draw conclusion.
/// Mate/stalemate diagrams also receive raw material scores; callers needing
/// game/search scores must use the rules-owned game outcomes first (MSH-20).
///
/// ```
/// use mshogi_core::chess::{Color, Position};
/// use mshogi_search::chess::evaluate;
/// let p = Position::from_fen("7k/8/8/8/8/8/8/KQ6 w - - 0 1")?;
/// assert_eq!(evaluate(&p).score_for(Color::White), 900);
/// assert_eq!(evaluate(&p).score_for(Color::Black), -900);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn evaluate(position: &Position) -> Evaluation {
    let mut result = Evaluation {
        white_material: 0,
        black_material: 0,
    };
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(piece) = position.piece_at(Square::new(file, rank).unwrap()) {
                let total = match piece.color {
                    Color::White => &mut result.white_material,
                    Color::Black => &mut result.black_material,
                };
                *total += piece_value(piece.kind);
            }
        }
    }
    result
}
