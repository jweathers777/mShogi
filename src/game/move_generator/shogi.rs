use crate::game::{board::Board, r#move::Move, player::Player};
use crate::game::move_generator::MoveGenerator;

pub struct ShogiMoveGenerator;

impl MoveGenerator for ShogiMoveGenerator {
    fn generate_moves(&self, board: &Board, player: Player) -> Vec<Move> {
        let mut moves = Vec::new();
        let direction = match player {
            Player::Sente => -1,
            Player::Gote => 1,
            _ => panic!("Invalid player for Shogi"),
        };

        for (index, piece) in board.squares.iter().enumerate() {
            if let Some(p) = piece {
                if p.symbol == "P" {
                    let (row, col) = (index / board.width, index % board.width);
                    let new_row = (row as isize + direction) as usize;
                    if new_row < board.height && board.get(new_row, col).is_none() {
                        moves.push(Move {
                            from: (row, col),
                            to: (new_row, col),
                            piece: "P".to_string(),
                            captured_pieces: vec![],
                            promotion: if new_row == 0 || new_row == 8 { Some("+P".to_string()) } else { None },
                            special_move: None,
                        });
                    }
                }
            }
        }

        moves
    }
}
