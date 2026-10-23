use crate::game::board::Board;
use crate::game::piece::Piece;
use crate::game::player::Player;
use crate::game::r#move::Move;
use crate::game::variant::Variant;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_board() -> Board {
        let variant = Variant::load_from_file("variants/chess.toml");
        Board::new(&variant)
    }

    #[test]
    fn test_unmake_last_move_simple() {
        let mut board = setup_board();
        let from_position = board.variant.notation_to_pos("e2");
        let to_position = board.variant.notation_to_pos("e4");
        let move_1 = Move {
            from: from_position,
            to: to_position,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        };

        board.make_move(&move_1);
        assert!(board.pieces.contains_key(&to_position));
        assert!(!board.pieces.contains_key(&from_position));

        board.unmake_last_move();
        assert!(board.pieces.contains_key(&from_position));
        assert!(!board.pieces.contains_key(&to_position));
    }

    #[test]
    fn test_unmake_last_move_with_capture() {
        let mut board = setup_board();
        let from_pos_1 = board.variant.notation_to_pos("e2");
        let to_pos_1 = board.variant.notation_to_pos("e4");
        let from_pos_2 = board.variant.notation_to_pos("d7");
        let to_pos_2 = board.variant.notation_to_pos("d5");
        let from_pos_3 = board.variant.notation_to_pos("e4");
        let to_pos_3 = board.variant.notation_to_pos("d5");

        let move_1 = Move {
            from: from_pos_1,
            to: to_pos_1,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        };

        let move_2 = Move {
            from: from_pos_2,
            to: to_pos_2,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        };

        let (r,c) = to_pos_3;
        let move_3 = Move {
            from: from_pos_3,
            to: to_pos_3,
            piece: "P".to_string(),
            captured_pieces: vec![(r, c, "P".to_string())],
            promotion: None,
            special_move: None,
        };

        board.make_move(&move_1); // e2-e4
        board.make_move(&move_2); // d7-d5
        board.make_move(&move_3); // e4xd5

        assert!(board.pieces.contains_key(&to_pos_3));
        assert!(!board.pieces.contains_key(&from_pos_3));
        assert!(board.pieces.get(&to_pos_3).unwrap().owner == Player::White);

        board.unmake_last_move();
        assert!(board.pieces.contains_key(&to_pos_1));
        assert!(board.pieces.contains_key(&to_pos_2));
        assert!(board.pieces.contains_key(&from_pos_3));
        assert!(board.pieces.get(&to_pos_3).unwrap().owner == Player::Black);
    }
}
