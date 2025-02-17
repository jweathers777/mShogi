use crate::game::board::Board;
use crate::game::piece::{MovePattern, Piece};
use crate::game::player::Player;
use crate::game::r#move::{Move,SpecialMove};
use crate::game::variant::Variant;
use crate::game::move_generator::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_variant_and_board(variant_name: &str) -> (Variant, Board) {
        let variant = Variant::load_from_file(&format!("variants/{}.toml", variant_name));
        let board = Board::new(&variant);
        (variant, board)
    }

    #[test]
    fn test_forward_one_no_capture() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let from_position = variant.notation_to_pos("e2");
        let to_position = variant.notation_to_pos("e3");
        let pawn = board.get(from_position).unwrap();
        let mut moves = Vec::new();
        forward_one_no_capture(&board, pawn, &mut moves);
        assert!(moves.iter().any(|m| m.to == to_position));
    }

    #[test]
    fn test_first_forward_two_no_capture() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let from_position = variant.notation_to_pos("e2");
        let to_position = variant.notation_to_pos("e4");
        let pawn = board.get(from_position).unwrap();
        let mut moves = Vec::new();
        first_forward_two_no_capture(&board, pawn, &mut moves);
        assert!(moves.iter().any(|m| m.to == to_position));
    }

    #[test]
    fn test_diagonal_capture() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let from_position = variant.notation_to_pos("e2");
        let enemy_from_position = variant.notation_to_pos("f7");
        let enemy_to_position = variant.notation_to_pos("f3");
        let (r,c) = enemy_to_position;
        // Put an enemy pawn on f3
        board.make_move(&Move {
            from: enemy_from_position,
            to: enemy_to_position,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });
        let pawn = board.get(from_position).unwrap();
        let mut moves = Vec::new();
        diagonal_capture(&board, pawn, &mut moves);
        assert!(moves.len() == 1);
        assert!(moves[0].to == enemy_to_position);
        assert!(moves[0].captured_pieces.len() == 1);
        assert!(moves[0].captured_pieces[0] == (r,c,"P".to_string()));
    }

    #[test]
    fn test_en_passant() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let start_position = variant.notation_to_pos("e2");
        let pawn_position = variant.notation_to_pos("e5");
        let enemy_from_position = variant.notation_to_pos("d7");
        let enemy_to_position = variant.notation_to_pos("d5");
        let (r,c) = enemy_to_position;

        // Put our pawn on e5
        board.make_move(&Move {
            from: start_position,
            to: pawn_position,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });

        // Move our enemy pawn from d7 to d5
        board.make_move(&Move {
            from: enemy_from_position,
            to: enemy_to_position,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });

        let pawn = board.get(pawn_position).unwrap();
        let dest_position = variant.notation_to_pos("d6");

        let mut moves = Vec::new();
        en_passant(&board, pawn, &mut moves);
        assert!(moves.len() == 1);
        assert!(moves.iter().any(|m| m.special_move == Some(SpecialMove::EnPassant)));
        assert!(moves[0].to == dest_position);
        assert!(moves[0].captured_pieces.len() == 1);
        assert!(moves[0].captured_pieces[0] == (r,c,"P".to_string()));
    }

    #[test]
    fn test_diagonal() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let first_bishop_position = variant.notation_to_pos("c1");
        let second_bishop_position = variant.notation_to_pos("e5");
        let mut moves = Vec::new();

        // Verify that the starting position is blocked
        // and so has no moves available
        {
            let bishop = board.get(first_bishop_position).unwrap();
            diagonal(&board, bishop, &mut moves);
            assert!(moves.is_empty());
        }

        // Drop the bishop down in the center of board
        board.make_move(&Move {
            from: first_bishop_position,
            to: second_bishop_position,
            piece: "B".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });

        let bishop = board.get(second_bishop_position).unwrap();
        diagonal(&board, bishop, &mut moves);
        assert!(moves.len() == 8);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 0).count() == 6);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 1).count() == 2);
    }

    #[test]
    fn test_orthogonal() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let first_rook_position = variant.notation_to_pos("a1");
        let second_rook_position = variant.notation_to_pos("e5");
        let mut moves = Vec::new();

        // Verify that the starting position is blocked
        // and so has no moves available
        {
            let rook = board.get(first_rook_position).unwrap();
            orthogonal(&board, rook, &mut moves);
            assert!(moves.is_empty());
        }

        // Drop the rook down in the center of board
        board.make_move(&Move {
            from: first_rook_position,
            to: second_rook_position,
            piece: "R".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });

        let rook = board.get(second_rook_position).unwrap();
        orthogonal(&board, rook, &mut moves);
        assert!(moves.len() == 11);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 0).count() == 10);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 1).count() == 1);
    }

    #[test]
    fn test_adjacent() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let first_king_position = variant.notation_to_pos("e1");
        let second_king_position = variant.notation_to_pos("e4");
        let enemy_from_position = variant.notation_to_pos("e7");
        let enemy_to_position = variant.notation_to_pos("e5");
        let mut moves = Vec::new();

        // Verify that the starting position is blocked
        // and so has no moves available
        {
            let king = board.get(first_king_position).unwrap();
            adjacent(&board, king, &mut moves);
            assert!(moves.is_empty());
        }

        // Drop the king down in the center of board
        board.make_move(&Move {
            from: first_king_position,
            to: second_king_position,
            piece: "K".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });

        // Move our enemy pawn forward two spaces
        board.make_move(&Move {
            from: enemy_from_position,
            to: enemy_to_position,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });

        let king = board.get(second_king_position).unwrap();
        adjacent(&board, king, &mut moves);
        assert!(moves.len() == 8);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 0).count() == 7);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 1).count() == 1);
    }

    #[test]
    fn test_any_direction() {
        let (variant, mut board) = setup_variant_and_board("chess");
        let first_queen_position = variant.notation_to_pos("d1");
        let second_queen_position = variant.notation_to_pos("e5");
        let mut moves = Vec::new();

        // Verify that the starting position is blocked
        // and so has no moves available
        {
            let queen = board.get(first_queen_position).unwrap();
            any_direction(&board, queen, &mut moves);
            assert!(moves.is_empty());
        }

        // Drop the queen down in the center of board
        board.make_move(&Move {
            from: first_queen_position,
            to: second_queen_position,
            piece: "Q".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });

        let queen = board.get(second_queen_position).unwrap();
        any_direction(&board, queen, &mut moves);
        assert!(moves.len() == 19);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 0).count() == 16);
        assert!(moves.iter().filter(|m| m.captured_pieces.len() == 1).count() == 3);
    }
}
