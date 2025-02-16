use crate::game::board::Board;
use crate::game::piece::{MovePattern, Piece};
use crate::game::player::Player;
use crate::game::r#move::{Move,SpecialMove};
use crate::game::variant::Variant;
use crate::game::move_generator::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_board(variant_name: &str) -> Board {
        let variant = Variant::load_from_file(&format!("variants/{}.toml", variant_name));
        Board::new(&variant)
    }

    #[test]
    fn test_forward_one_no_capture() {
        let mut board = setup_board("chess");
        let pawn_position = (1, 4);
        let pawn = board.get(pawn_position).unwrap();
        let mut moves = Vec::new();
        forward_one_no_capture(&board, pawn, &mut moves);
        assert!(moves.iter().any(|m| m.to == (2, 4)));
    }

    #[test]
    fn test_first_forward_two_no_capture() {
        let mut board = setup_board("chess");
        let pawn_position = (1, 4);
        let pawn = board.get(pawn_position).unwrap();
        let mut moves = Vec::new();
        first_forward_two_no_capture(&board, pawn, &mut moves);
        assert!(moves.iter().any(|m| m.to == (3, 4)));
    }

    #[test]
    fn test_diagonal_capture() {
        let mut board = setup_board("chess");
        let pawn_position = (1, 4);
        let enemy_position = (2, 5);
        let mut enemy_piece = board.get(enemy_position).unwrap().clone();
        enemy_piece.owner = Player::Black;
        board.set(enemy_position, Some(enemy_piece));
        let pawn = board.get(pawn_position).unwrap();
        let mut moves = Vec::new();
        diagonal_capture(&board, pawn, &mut moves);
        assert!(moves.iter().any(|m| m.to == enemy_position));
    }

    #[test]
    fn test_en_passant() {
        let mut board = setup_board("chess");
        let pawn_position = (4, 4);
        let enemy_pawn_position = (4, 3);
        let mut enemy_pawn = board.get(enemy_pawn_position).unwrap().clone();
        enemy_pawn.owner = Player::Black;
        board.set(enemy_pawn_position, Some(enemy_pawn));
        board.move_history.push(Move {
            from: (6, 3),
            to: enemy_pawn_position,
            piece: "P".to_string(),
            captured_pieces: vec![],
            promotion: None,
            special_move: None,
        });
        let pawn = board.get(pawn_position).unwrap();
        let mut moves = Vec::new();
        en_passant(&board, pawn, &mut moves);
        assert!(moves.iter().any(|m| m.special_move == Some(SpecialMove::EnPassant)));
    }

    #[test]
    fn test_diagonal() {
        let mut board = setup_board("chess");
        let bishop_position = (0, 2);
        let bishop = board.get(bishop_position).unwrap();
        let mut moves = Vec::new();
        diagonal(&board, bishop, &mut moves);
        assert!(!moves.is_empty());
    }

    #[test]
    fn test_orthogonal() {
        let mut board = setup_board("chess");
        let rook_position = (0, 0);
        let rook = board.get(rook_position).unwrap();
        let mut moves = Vec::new();
        orthogonal(&board, rook, &mut moves);
        assert!(!moves.is_empty());
    }

    #[test]
    fn test_adjacent() {
        let mut board = setup_board("chess");
        let king_position = (0, 4);
        let king = board.get(king_position).unwrap();
        let mut moves = Vec::new();
        adjacent(&board, king, &mut moves);
        assert!(moves.len() > 0);
    }

    #[test]
    fn test_any_direction() {
        let mut board = setup_board("chess");
        let queen_position = (0, 3);
        let queen = board.get(queen_position).unwrap();
        let mut moves = Vec::new();
        any_direction(&board, queen, &mut moves);
        assert!(moves.len() > 0);
    }
}
