use crate::game::board::Board;
use crate::game::r#move::{Move,SpecialMove};
use crate::game::piece::{MovePattern,Piece};
use crate::game::player::Player;

pub fn generate_moves_for_player(board: &Board, player: &Player) -> Vec<Move> {
    let mut moves = Vec::new();

    let mut player_pieces: Vec<&Piece> = board.pieces.values()
        .filter(|p| p.owner == *player)
        .collect();

    player_pieces.sort_by_key(|p| p.position);

    for piece in player_pieces {
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
            MovePattern::Diagonal => diagonal(board, piece, &mut moves),
            MovePattern::Orthogonal => orthogonal(board, piece, &mut moves),
            MovePattern::OrthogonalOneDiagonalOneJump => orthogonal_one_diagonal_one_jump(board, piece, &mut moves),
            MovePattern::Adjacent => adjacent(board, piece, &mut moves),
            MovePattern::AnyDirection => any_direction(board, piece, &mut moves),
            _ => {} // Extend this for other move patterns
        }
    }

    moves
}

pub fn forward_one_no_capture(board: &Board, piece: &Piece, moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
    let delta_row = piece.owner.forward();
    let new_row = row as isize + delta_row;
    if board.is_within_row_bounds(new_row) {
        let new_row = new_row as usize;
        if board.get((new_row, col)).is_none() {
            moves.push(create_move(&piece, (row,col), (new_row, col), vec![], None, None));
        }
    }
}

pub fn first_forward_two_no_capture(board: &Board, piece: &Piece, moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
    let delta_row = piece.owner.forward();
    let mid_row = row as isize + delta_row;
    let new_row = mid_row + delta_row;

    if board.is_within_row_bounds(mid_row) && board.is_within_row_bounds(new_row) {
        let mid_row = mid_row as usize;
        let new_row = new_row as usize;
        if board.get((mid_row, col)).is_none()
            && board.get((new_row, col)).is_none()
            && piece.move_count == 0
        {
            moves.push(create_move(&piece, (row,col), (new_row, col), vec![], None, None));
        }
    }
}

pub fn diagonal_capture(board: &Board, piece: &Piece, moves: &mut Vec<Move>) {
    let (row, col) = piece.position;
    let directions = piece.owner.forward_diagonals();
    for (dr, dc) in directions {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if board.is_within_bounds(new_row, new_col) {
            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if let Some(target_piece) = board.get((new_row, new_col)) {
                if target_piece.owner != piece.owner {
                    moves.push(create_move(
                        &piece,
                        (row, col),
                        (new_row, new_col),
                        vec![&target_piece],
                        None,
                        None,
                    ));
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
            let delta_row = piece.owner.forward();
            let new_row = row as isize + delta_row;
            if board.is_within_row_bounds(new_row) {
                let new_row = new_row as usize;
                let (target_row, target_col) = last_move.to;

                if let Some(target_piece) = board.get(last_move.to) {
                    // Capture enemy piece
                    if target_piece.owner != piece.owner {
                        moves.push(create_move(
                            &piece,
                            (row, col),
                            (new_row, target_col),
                            vec![&target_piece],
                            None,
                            Some(SpecialMove::EnPassant),
                        ));
                    }
                }
            }
        }
    }
}

fn create_move(
    piece: &Piece,
    from: (usize, usize),
    to: (usize, usize),
    captured_pieces: Vec<&Piece>,
    promotion: Option<String>,
    special_move: Option<SpecialMove>,
) -> Move {
    Move {
        from,
        to,
        piece: piece.piece_type.symbol.clone(),
        captured_pieces: captured_pieces.into_iter()
            .map(|p| (p.position.0, p.position.1, p.piece_type.symbol.clone()))
            .collect(),
        promotion,
        special_move,
    }
}

fn directional_moves(
    board: &Board,
    piece: &Piece,
    directions: &[(isize, isize)],
    moves: &mut Vec<Move>,
    sliding: bool,
)
{
    let (row, col) = piece.position;

    for &(dr, dc) in directions {
        let mut new_row = row as isize;
        let mut new_col = col as isize;

        loop {
            new_row += dr;
            new_col += dc;

            if !board.is_within_bounds(new_row, new_col) {
                break;
            }

            let new_pos = (new_row as usize, new_col as usize);
            if let Some(target_piece) = board.get(new_pos) {
                if target_piece.owner != piece.owner {
                    moves.push(create_move(
                        piece,
                        (row, col),
                        new_pos,
                        vec![&target_piece],
                        None,
                        None,
                    ));
                }
                break; // Stop sliding if we hit a piece
            } else {
                moves.push(create_move(
                    piece,
                    (row, col),
                    new_pos,
                    vec![],
                    None,
                    None,
                ));
            }

            if !sliding {
                break; // Stop if this is a step move
            }
        }
    }
}

pub fn diagonal(board: &Board, piece: &Piece,  moves: &mut Vec<Move>) {
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    directional_moves(board, piece, &directions, moves, true);
}

pub fn orthogonal(board: &Board, piece: &Piece,  moves: &mut Vec<Move>) {
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    directional_moves(board, piece, &directions, moves, true);
}

pub fn orthogonal_one_diagonal_one_jump(
    board: &Board, piece: &Piece,  moves: &mut Vec<Move>
)
{
    let directions = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2),
    ];
    directional_moves(board, piece, &directions, moves, false);
}

pub fn any_direction(board: &Board, piece: &Piece,  moves: &mut Vec<Move>) {
    let directions = [
        (-1, -1), (-1, 1), (1, -1), (1, 1),
        (0, -1), (0, 1), (-1, 0), (1, 0),
    ];
    directional_moves(board, piece, &directions, moves, true);
}

pub fn adjacent(board: &Board, piece: &Piece,  moves: &mut Vec<Move>) {
    let directions = [
        (-1, -1), (-1, 1), (1, -1), (1, 1),
        (0, -1), (0, 1), (-1, 0), (1, 0),
    ];
    directional_moves(board, piece, &directions, moves, false);
}
