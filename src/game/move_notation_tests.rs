#[cfg(test)]
mod tests {
    use crate::game::r#move::{Move, SpecialMove};

    #[test]
    fn test_chess_move_notation() {
        let mv = Move {
            from: (6, 2),
            to: (4, 2),
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        };
        assert_eq!(mv.to_notation("Chess"), "c5");

        let mv_capture = Move {
            from: (6, 2),
            to: (4, 3),
            piece: "P".to_string(),
            captured_pieces: vec![(4, 3, "N".to_string())],
            promotion: None,
            special_move: None,
        };
        assert_eq!(mv_capture.to_notation("Chess"), "cxd5");

        let mv_promo = Move {
            from: (6, 3),
            to: (7, 3),
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: Some("Q".to_string()),
            special_move: None,
        };
        assert_eq!(mv_promo.to_notation("Chess"), "d8=Q");
    }

    #[test]
    fn test_shogi_move_notation() {
        let mv = Move {
            from: (6, 4),
            to: (5, 4),
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        };
        assert_eq!(mv.to_notation("Shogi"), "P-6e");

        let mv_capture = Move {
            from: (6, 4),
            to: (5, 4),
            piece: "P".to_string(),
            captured_pieces: vec![(5, 4, "N".to_string())],
            promotion: None,
            special_move: None,
        };
        assert_eq!(mv_capture.to_notation("Shogi"), "Px6e");
    }

    #[test]
    fn test_chu_shogi_lion_moves() {
        let mv_lion = Move {
            from: (7, 5),
            to: (5, 5),
            piece: "Ln".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: Some(SpecialMove::LionSecondMove),
        };
        assert_eq!(mv_lion.to_notation("ChuShogi"), "Ln-6f");

        let mv_lion_igui = Move {
            from: (7, 5),
            to: (7, 5),
            piece: "Ln".to_string(),
            captured_pieces: vec![(6, 6, "P".to_string())],
            promotion: None,
            special_move: None,
        };
        assert_eq!(mv_lion_igui.to_notation("ChuShogi"), "Lnx!7g");

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
        assert_eq!(mv_lion_double.to_notation("ChuShogi"), "Lnx3ix4h");
    }
}
