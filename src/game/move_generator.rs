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

    for move_pattern in &piece.piece_type.moves {
        match move_pattern {
            MovePattern::ForwardOneNoCapture => forward_one_no_capture(board, piece, &mut moves),
            MovePattern::FirstForwardTwoNoCapture => first_forward_two_no_capture(board, piece, &mut moves),
            MovePattern::DiagonalCapture => diagonal_capture(board, piece, &mut moves),
            MovePattern::EnPassant => en_passant(board, piece, &mut moves),
            MovePattern::ElJump => el_jump(board, piece, &mut moves),
            _ => {} // Extend this for other move patterns
        }
    }

    moves
}

pub fn forward_one_no_capture(board: &Board, piece: &Piece, moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
    let delta_row = match piece.owner {
        Player::White => 1,
        Player::Black => -1
    };
    let new_row = (row as isize + delta_row) as usize;

    if new_row > 0
        && new_row < board.height
        && board.get((new_row, col)).is_none()
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

pub fn first_forward_two_no_capture(board: &Board, piece: &Piece, moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
    let delta_row = match piece.owner {
        Player::White => 1,
        Player::Black => -1
    };
    let mid_row = (row as isize + delta_row) as usize;
    let new_row = (mid_row as isize + delta_row) as usize;

    if new_row > 0
        && new_row < board.height
        && mid_row > 0
        && mid_row < board.height
        && board.get((mid_row, col)).is_none()
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

pub fn diagonal_capture(board: &Board, piece: &Piece, moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
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

pub fn en_passant(board: &Board, piece: &Piece, moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
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
            let (target_row, target_col) = last_move.to;

            if let Some(target_piece) = board.get(last_move.to) {
                // Capture enemy piece
                if target_piece.owner != piece.owner {
                    moves.push(Move {
                        from: (row, col),
                        to: (new_row, target_col),
                        piece: piece.piece_type.symbol.clone(),
                        captured_pieces: vec![(target_row, target_col, last_move.piece.clone())],
                        promotion: None,
                        special_move: Some(SpecialMove::EnPassant),
                    });
                }
            }
        }
    }
}

pub fn el_jump(board: &Board, piece: &Piece,  moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
    let l_moves = [
        (2, 1), (2, -1), (-2, 1), (-2, -1), // Vertical L-jumps
        (1, 2), (1, -2), (-1, 2), (-1, -2)  // Horizontal L-jumps
    ];

    for (dr, dc) in l_moves.iter() {
        let new_row = row.wrapping_add(*dr as usize);
        let new_col = col.wrapping_add(*dc as usize);

        // Ensure the move is within bounds
        if new_row < board.height && new_col < board.width {
            if let Some(target_piece) = board.get((new_row, new_col)) {
                // Capture enemy piece
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
            } else {
                // Normal move (empty square)
                moves.push(Move {
                    from: (row, col),
                    to: (new_row, new_col),
                    piece: piece.piece_type.symbol.clone(),
                    captured_pieces: vec![],
                    promotion: None,
                    special_move: None,
                });
            }
        }
    }
}
