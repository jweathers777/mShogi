//! Compact, original KPK board knowledge generated using native legal actions.
//!
//! This is a clock/history-free pawn-side win/draw bitbase, not adjudication.
//! Search must process native outcomes and claims first. A theoretical win may
//! still be drawn by the actual history or clock. No runtime table generation.
use mshogi_core::chess::{Color, PieceKind, Position, Square};

pub(crate) const STATES: usize = 24 * 64 * 64 * 2;
const WINS: &[u8; STATES / 8] = include_bytes!("data/chess_kpk.bin");

/// Return whether the lone pawn's side can win in the board-only KPK model.
/// `None` means the material/rights do not match; it never means a game outcome.
pub(crate) fn probe(position: &Position) -> Option<bool> {
    let (index, _) = index(position)?;
    Some(wins(index))
}

/// Table bit for a state index from `index` or `index_parts`.
pub(crate) fn wins(index: usize) -> bool {
    WINS[index / 8] & (1 << (index % 8)) != 0
}

/// State index from square indexes (a1 = 0, h8 = 63). `strong_black` names the
/// pawn's side and `strong_to_move` the turn. The caller has already checked
/// that the material is king and pawn against king with no castling rights.
pub(crate) fn index_parts(
    mut pawn: usize,
    strong_black: bool,
    mut strong_king: usize,
    mut weak_king: usize,
    strong_to_move: bool,
) -> usize {
    if strong_black {
        pawn ^= 56;
        strong_king ^= 56;
        weak_king ^= 56;
    }
    if pawn % 8 >= 4 {
        pawn ^= 7;
        strong_king ^= 7;
        weak_king ^= 7;
    }
    let pawn_slot = (pawn / 8 - 1) * 4 + pawn % 8;
    ((pawn_slot * 64 + strong_king) * 64 + weak_king) * 2 + usize::from(!strong_to_move)
}

/// Normalize pawn side to White, pawn file to a..d, preserving turn identity.
/// Raw EP after the lone pawn's double push is immaterial: no opposing pawn
/// exists to capture. The native position has already validated that EP state.
pub(crate) fn index(position: &Position) -> Option<(usize, Color)> {
    let mut kings = [0usize; 2];
    let mut pawn = None;
    for rank in 0..8 {
        for file in 0..8 {
            let sq = Square::new(file, rank).unwrap();
            let Some(piece) = position.piece_at(sq) else {
                continue;
            };
            match piece.kind {
                PieceKind::King => {
                    kings[usize::from(piece.color == Color::Black)] =
                        usize::from(rank) * 8 + usize::from(file);
                }
                PieceKind::Pawn if pawn.is_none() => {
                    pawn = Some((usize::from(rank) * 8 + usize::from(file), piece.color));
                }
                _ => return None,
            }
        }
    }
    let (pawn, strong) = pawn?;
    // Castling requires a rook in a validated Position, already excluded above;
    // keep this explicit so the bitbase's applicability remains self-contained.
    for side in [Color::White, Color::Black] {
        if position.castling_rights().kingside(side) || position.castling_rights().queenside(side) {
            return None;
        }
    }
    Some((
        index_parts(
            pawn,
            strong == Color::Black,
            kings[usize::from(strong == Color::Black)],
            kings[usize::from(strong == Color::White)],
            position.side_to_move() == strong,
        ),
        strong,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_opposition_promotion_and_rook_pawn_cases() {
        // Syzygy-confirmed positions, plus their color/file reflections. The
        // opposite-turn e4 opposition pair is deliberately the same board.
        for (fen, win) in [
            ("8/4k3/8/4K3/4P3/8/8/8 w - - 0 1", false),
            ("8/4k3/8/4K3/4P3/8/8/8 b - - 0 1", true),
            ("8/8/8/4p3/4k3/8/4K3/8 b - - 0 1", false),
            ("8/8/8/4p3/4k3/8/4K3/8 w - - 0 1", true),
            ("8/3k4/8/3K4/3P4/8/8/8 b - - 0 1", true),
            ("k7/P7/1K6/8/8/8/8/8 w - - 0 1", false),
            ("K7/P7/k7/8/8/8/8/8 w - - 0 1", true),
            ("8/1Pk5/8/8/8/8/8/K7 w - - 0 1", false),
            ("7k/1P6/1K6/8/8/8/8/8 w - - 0 1", true),
            // c8=Q stalemates, c8=R wins: the seed must admit the safe rook.
            ("8/k1P5/2K5/8/8/8/8/8 w - - 0 1", true),
        ] {
            let p = Position::from_fen(fen).unwrap();
            let original = p.clone();
            assert_eq!(probe(&p), Some(win), "{fen}");
            assert_eq!(p, original);
        }
    }

    #[test]
    fn applicability_and_clocks_do_not_impersonate_adjudication() {
        for fen in [
            "7k/8/8/8/8/8/8/K7 w - - 0 1",
            "7k/8/8/8/8/8/P7/KR6 w - - 0 1",
            "7k/p7/8/8/8/8/P7/K7 w - - 0 1",
        ] {
            assert_eq!(probe(&Position::from_fen(fen).unwrap()), None);
        }
        let ordinary = Position::from_fen("8/4k3/8/4K3/4P3/8/8/8 b - - 0 1").unwrap();
        let expired = Position::from_fen("8/4k3/8/4K3/4P3/8/8/8 b - - 150 99").unwrap();
        assert_eq!(probe(&ordinary), probe(&expired));
        // Lone-pawn EP cannot be captured; raw FEN EP is a historical field,
        // deliberately normalized only inside this advisory board knowledge.
        let ep = Position::from_fen("7k/8/8/8/4P3/8/8/K7 b - e3 0 1").unwrap();
        let no_ep = Position::from_fen("7k/8/8/8/4P3/8/8/K7 b - - 0 1").unwrap();
        assert_eq!(probe(&ep), probe(&no_ep));
    }

    #[test]
    fn independent_promotion_seeds_exclude_stalemate_and_immediate_capture() {
        use mshogi_core::chess::Classification;
        // Ka7/Kc6/Pc7: a queen removes all a-file king exits; a rook leaves a6.
        // Rook underpromotion therefore preserves a winning major-piece ending.
        let p = Position::from_fen("8/k1P5/2K5/8/8/8/8/8 w - - 0 1").unwrap();
        let mut queen = p.clone();
        queen.apply("c7c8q".parse().unwrap()).unwrap();
        assert_eq!(queen.classify(), Classification::Stalemate);
        let mut rook = p;
        rook.apply("c7c8r".parse().unwrap()).unwrap();
        assert_eq!(rook.classify(), Classification::Quiet);
        assert_eq!(
            rook.legal_actions()
                .into_iter()
                .map(|action| action.to_string())
                .collect::<Vec<_>>(),
            ["a7a6"]
        );
        // Kc7 attacks b8. All promotions can be captured before conversion.
        for promotion in ["q", "r", "b", "n"] {
            let mut p = Position::from_fen("8/1Pk5/8/8/8/8/8/K7 w - - 0 1").unwrap();
            p.apply(format!("b7b8{promotion}").parse().unwrap())
                .unwrap();
            assert!(p.legal_actions().contains(&"c7b8".parse().unwrap()));
        }
    }
}
