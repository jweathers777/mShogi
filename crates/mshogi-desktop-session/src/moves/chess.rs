//! PGN specification8.2.3 English SAN, from native legal alternatives/state.
use mshogi_core::chess::*;
pub(super) fn notation(position: &Position, action: Action) -> String {
    let piece = position
        .piece_at(action.from)
        .expect("admitted move source");
    let capture = position.piece_at(action.to).is_some()
        || (piece.kind == PieceKind::Pawn && action.from.file() != action.to.file());
    let mut text = String::new();
    if piece.kind == PieceKind::King && action.from.file().abs_diff(action.to.file()) == 2 {
        text.push_str(if action.to.file() > action.from.file() {
            "O-O"
        } else {
            "O-O-O"
        });
    } else {
        if piece.kind != PieceKind::Pawn {
            text.push(letter(piece.kind));
            let alternatives: Vec<_> = position
                .legal_actions()
                .into_iter()
                .filter(|candidate| {
                    candidate.from != action.from
                        && candidate.to == action.to
                        && position.piece_at(candidate.from) == Some(piece)
                })
                .collect();
            if !alternatives.is_empty() {
                if alternatives
                    .iter()
                    .all(|a| a.from.file() != action.from.file())
                {
                    text.push(char::from(b'a' + action.from.file()));
                } else if alternatives
                    .iter()
                    .all(|a| a.from.rank() != action.from.rank())
                {
                    text.push(char::from(b'1' + action.from.rank()));
                } else {
                    text.push_str(&action.from.to_string());
                }
            }
        } else if capture {
            text.push(char::from(b'a' + action.from.file()));
        }
        if capture {
            text.push('x');
        }
        text.push_str(&action.to.to_string());
        if let Some(promotion) = action.promotion {
            text.push('=');
            text.push(match promotion {
                Promotion::Queen => 'Q',
                Promotion::Rook => 'R',
                Promotion::Bishop => 'B',
                Promotion::Knight => 'N',
            });
        }
    }
    let mut after = position.clone();
    after.apply(action).expect("admitted action");
    match after.classify() {
        Classification::Check => text.push('+'),
        Classification::Checkmate => text.push('#'),
        _ => {}
    }
    text
}
fn letter(kind: PieceKind) -> char {
    match kind {
        PieceKind::King => 'K',
        PieceKind::Queen => 'Q',
        PieceKind::Rook => 'R',
        PieceKind::Bishop => 'B',
        PieceKind::Knight => 'N',
        PieceKind::Pawn => 'P',
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn san(fen: &str, action: &str, expected: &str) {
        let p: Position = fen.parse().unwrap();
        let a: Action = action.parse().unwrap();
        assert!(p.legal_actions().contains(&a));
        assert_eq!(notation(&p, a), expected);
    }
    #[test]
    fn pgn_independent_examples() {
        san(STANDARD_START_FEN, "e2e4", "e4");
        san("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1g1", "O-O");
        san("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1c1", "O-O-O");
        san("7k/8/8/3pP3/8/8/8/K7 w - d6 0 1", "e5d6", "exd6");
        san("7k/P7/8/8/8/8/8/K7 w - - 0 1", "a7a8n", "a8=N");
        san("7k/P7/8/8/8/8/8/K7 w - - 0 1", "a7a8q", "a8=Q+");
        san("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", "g6g7", "Qg7#");
    }
    #[test]
    fn actual_legal_disambiguation_not_attacks() {
        san("7k/8/8/8/8/8/8/KN3N2 w - - 0 1", "b1d2", "Nbd2");
        san("7k/8/8/8/8/1N6/8/KN6 w - - 0 1", "b1d2", "N1d2");
        san("7k/8/8/1N6/8/1N3N2/8/K7 w - - 0 1", "b3d4", "Nb3d4");
        san("k3r3/8/8/8/8/8/4N3/4K1N1 w - - 0 1", "g1f3", "Nf3");
    }
}
