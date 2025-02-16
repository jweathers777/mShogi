#[cfg(test)]
mod tests {
    use crate::game::board::Board;
    use crate::game::piece::Piece;
    use crate::game::player::Player;
    use crate::game::move_generator::generate_moves_for_piece;
    use crate::game::r#move::{Move,SpecialMove};
    use crate::game::variant::load_variant;

    #[test]
    fn test_generate_chess_pawn_moves() {
        let variant = load_variant("variants/chess.toml");
        let mut board = Board::new(&variant);

        // Test initial pawn moves

        // Pawn at e2
        let mut pawn_position = (1, 4);
        let (mut pawn_row, mut pawn_col) = pawn_position;
        {
            let pawn = board.get(pawn_position).unwrap();
            let moves = generate_moves_for_piece(&board, &pawn);

            assert!(moves.len() == 2);
            // e3
            assert!(moves.iter().any(|m| m.to == (pawn_row+1, pawn_col)));
            // e4
            assert!(moves.iter().any(|m| m.to == (pawn_row+2, pawn_col)));
        }

        // Move the pawn one square forward
        {
            let pawn = board.get(pawn_position).unwrap();
            let mv = generate_moves_for_piece(&board, &pawn)
                .into_iter().find(|m| m.to == (pawn_row+1, pawn_col)).unwrap();
            board.make_move(&mv);
            pawn_position = (pawn_row+1, pawn_col);
            pawn_row += 1;
        }

        let moves_after_move = {
            let pawn = board.get(pawn_position).unwrap().clone();
            generate_moves_for_piece(&board, &pawn)
        };
        // e4, i.e. one step forward
        assert!(moves_after_move.iter().any(|m| m.to == (pawn_row+1, pawn_col)));
        // Moving forward two steps should no longer be an option
        assert!(!moves_after_move.iter().any(|m| m.to == (pawn_row+2, pawn_col)));

        // Test diagonal capture
        let opponent_pawn_position = (pawn_row+1, pawn_col+1);
        let opponent_pawn = Piece {
            position: opponent_pawn_position,
            piece_type: variant.piece_types.get("P").unwrap().clone(),
            owner: Player::Black,
            move_count: 0,
        };

        // Set black pawn on f4
        board.set(opponent_pawn_position, Some(opponent_pawn));
        {
            let pawn = board.get(pawn_position).unwrap().clone();
            let moves = generate_moves_for_piece(&board, &pawn);
            // e3xf4
            assert!(moves.iter().any(|m| m.to == opponent_pawn_position));
            let capture_move = moves.iter()
                .find(|m| m.to == opponent_pawn_position).unwrap();
            assert!(capture_move.captured_pieces.len() == 1);
        }

        // Test en passant

        // Move the pawn two more square forwards to e5
        for _ in 0..2 {
            let pawn = board.get(pawn_position).unwrap();
            pawn_row += 1;

            let mv = generate_moves_for_piece(&board, &pawn)
                .into_iter().find(|m| m.to == (pawn_row, pawn_col)).unwrap();
            board.make_move(&mv);
            pawn_position = (pawn_row, pawn_col);
        }

        // Move the black pawn on d7 to d5
        {
            let pawn = board.get((pawn_row+2, pawn_col-1)).unwrap();
            let mv = generate_moves_for_piece(&board, &pawn)
                .into_iter().find(|m| m.to == (pawn_row, pawn_col-1)).unwrap();
            board.make_move(&mv);
        }

        // Test capturing the black pawn via en passant
        {
            let pawn = board.get(pawn_position).unwrap();
            let moves = generate_moves_for_piece(&board, &pawn);
            assert!(board.get((pawn_row, pawn_col-1)).is_some());
            assert!(
                moves.iter().any(|m|
                    m.to == (pawn_row+1, pawn_col-1) &&
                    m.captured_pieces.len() == 1 &&
                    m.captured_pieces[0] == (pawn_row, pawn_col-1, String::from("P")) &&
                    m.special_move == Some(SpecialMove::EnPassant)
                )
            );
        }

        // Move the white pawn on a2 to a3
        {
            let pawn = board.get((1, 0)).unwrap();
            let mv = generate_moves_for_piece(&board, &pawn)
                .into_iter().find(|m| m.to == (2, 0)).unwrap();
            board.make_move(&mv);
        }

        // Move the black pawn on a7 to a6
        {
            let pawn = board.get((6, 0)).unwrap();
            let mv = generate_moves_for_piece(&board, &pawn)
                .into_iter().find(|m| m.to == (5, 0)).unwrap();
            board.make_move(&mv);
        }

        // Test inability to capture the black pawn via en passant after another move
        {
            let pawn = board.get(pawn_position).unwrap();
            let moves = generate_moves_for_piece(&board, &pawn);
            assert!(board.get((pawn_row, pawn_col-1)).is_some());
            assert!(
                !moves.iter().any(|m|
                    m.to == (pawn_row+1, pawn_col-1) &&
                    m.captured_pieces.len() == 1 &&
                    m.captured_pieces[0] == (pawn_row, pawn_col-1, String::from("P")) &&
                    m.special_move == Some(SpecialMove::EnPassant)
                )
            );
        }
    }
}
