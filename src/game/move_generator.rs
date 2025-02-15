use crate::game::board::Board;
use crate::game::r#move::{Move,SpecialMove};
use crate::game::piece::{MovePattern,Piece};
use crate::game::player::Player;

pub fn generate_moves_for_player(board: &Board, player: &Player) -> Vec<Move> {
    let mut moves = Vec::new();
    for piece in board.pieces.values().filter(|p| p.owner == *player) {
        moves.extend(generate_moves_for_piece(board, piece));
    }
    moves
}

pub fn generate_moves_for_piece(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves = Vec::new();
    let (row, col) = piece.position;

    for move_type in &piece.piece_type.moves {
        match move_type {
            MovePattern::ForwardOneNoCapture => {
                let new_row = match piece.owner {
                    Player::White => row.wrapping_add(1),
                    Player::Black => row.wrapping_sub(1),
                };
                if new_row < board.height && board.get((new_row, col)).is_none() {
                    moves.push(Move {
                        from: (row, col),
                        to: (new_row, col),
                        piece: piece.piece_type.symbol.clone(),
                        captured_pieces: vec![],
                        promotion: None,
                        special_move: None,
                    });
                }
            }
				MovePattern::ForwardTwoOnFirstMoveNoCapture => {
					 let new_row = match piece.owner {
                    Player::White => row.wrapping_add(2),
                    Player::Black => row.wrapping_sub(2),
					 };

                if let Some(target_piece) = board.get(piece.position) {
                    if new_row < board.height
                        && target_piece == piece
                        && board.get((new_row, col)).is_none()
                        && piece.move_count == 0
                    {
                        moves.push(Move {
                            from: (row, col),
                            to: (new_row, col),
                            piece: piece.piece_type.symbol.clone(),
                            captured_pieces: vec![],
                            promotion: None,
                            special_move: None,
                        });
                    }
                }
				}
            MovePattern::DiagonalCapture => {
                let directions = match piece.owner {
                    Player::White => vec![(1, -1), (1, 1)],
                    Player::Black => vec![(-1, -1), (-1, 1)],
                };
                for (dr, dc) in directions {
                    let new_row = (row as isize + dr) as usize;
                    let new_col = (col as isize + dc) as usize;
                    if new_row < board.height && new_col < board.width {
                        if let Some(target_piece) = board.get((new_row, new_col)) {
                            if target_piece.owner != piece.owner {
                                moves.push(Move {
                                    from: (row, col),
                                    to: (new_row, new_col),
                                    piece: piece.piece_type.symbol.clone(),
                                    captured_pieces: vec![(new_row, new_col, target_piece.piece_type.symbol.clone())],
                                    promotion: None,
                                    special_move: None,
                                });
                            }
                        }
                    }
                }
            }
            MovePattern::EnPassant => {
                if let Some(last_move) = board.move_history.last() {
                    if last_move.piece == "P"
                        && (last_move.to.0 as isize - last_move.from.0 as isize).abs() == 2
                        && (last_move.to.1 as isize - col as isize).abs() == 1
                        && last_move.to.0 == row
                    {
                        let new_row = match piece.owner {
                            Player::White => row.wrapping_add(1),
                            Player::Black => row.wrapping_sub(1),
                        };

                        moves.push(Move {
                            from: (row, col),
                            to: (new_row, last_move.to.1),
                            piece: piece.piece_type.symbol.clone(),
                            captured_pieces: vec![(last_move.to.0, last_move.to.1, last_move.piece.clone())],
                            promotion: None,
                            special_move: Some(SpecialMove::EnPassant),
                        });
                    }
                }
            }
            _ => {} // Extend this for other move patterns
        }
    }

    moves
}
