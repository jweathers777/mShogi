#[cfg(test)]
mod tests {
    use crate::game::r#move::{Move, SpecialMove};
    use crate::game::variant::Variant;

    #[test]
    fn test_chess_move_notation() {
        let variant = Variant::load_from_file(&format!("variants/chess.toml"));

        let mv = Move {
            from: (6, 2),
            to: (4, 2),
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        };
        assert_eq!(variant.move_to_notation(&mv), "c5");

        let mv_capture = Move {
            from: (6, 2),
            to: (4, 3),
            piece: "P".to_string(),
            captured_pieces: vec![(4, 3, "N".to_string())],
            promotion: None,
            special_move: None,
        };
        assert_eq!(variant.move_to_notation(&mv_capture), "cxd5");

        let mv_promo = Move {
            from: (6, 3),
            to: (7, 3),
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: Some("Q".to_string()),
            special_move: None,
        };
        assert_eq!(variant.move_to_notation(&mv_promo), "d8=Q");
    }

    #[test]
    fn test_chess_castling_notation() {
        let variant = Variant::load_from_file(&format!("variants/chess.toml"));

        let castling_kingside = Move {
            from: (0, 4),
            to: (0, 6),
            piece: "K".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: Some(SpecialMove::CastlingKingside),
        };
        assert_eq!(variant.move_to_notation(&castling_kingside), "O-O");

        let castling_queenside = Move {
            from: (0, 4),
            to: (0, 2),
            piece: "K".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: Some(SpecialMove::CastlingQueenside),
        };
        assert_eq!(variant.move_to_notation(&castling_queenside), "O-O-O");
    }

    #[test]
    fn test_shogi_move_notation() {
        let variant = Variant::load_from_file(&format!("variants/shogi.toml"));

        let mv = Move {
            from: (6, 4),
            to: (5, 4),
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        };
        assert_eq!(variant.move_to_notation(&mv), "P-6e");

        let mv_capture = Move {
            from: (6, 4),
            to: (5, 4),
            piece: "P".to_string(),
            captured_pieces: vec![(5, 4, "N".to_string())],
            promotion: None,
            special_move: None,
        };
        assert_eq!(variant.move_to_notation(&mv_capture), "Px6e");
    }

    #[test]
    fn test_chu_shogi_lion_moves() {
        let variant = Variant::load_from_file(&format!("variants/chu_shogi.toml"));

        let mv_lion = Move {
            from: (7, 5),
            to: (5, 5),
            piece: "Ln".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: Some(SpecialMove::LionSecondMove),
        };
        assert_eq!(variant.move_to_notation(&mv_lion), "Ln-6f");

        let mv_lion_igui = Move {
            from: (7, 5),
            to: (7, 5),
            piece: "Ln".to_string(),
            captured_pieces: vec![(6, 6, "P".to_string())],
            promotion: None,
            special_move: None,
        };
        assert_eq!(variant.move_to_notation(&mv_lion_igui), "Lnx!7g");

        let mv_lion_double = Move {
            from: (3, 6),
            to: (3, 7),
            piece: "Ln".to_string(),
            captured_pieces: vec![
                (2, 8, "N".to_string()),
                (3, 7, "P".to_string())
            ],
            promotion: None,
            special_move: None,
        };
        assert_eq!(variant.move_to_notation(&mv_lion_double), "Lnx3ix4h");
    }
}
