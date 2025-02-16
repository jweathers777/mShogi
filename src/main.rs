#![allow(warnings)]

mod game;

use game::{
    board::Board,
    r#move::Move,
    move_generator::generate_moves_for_player,
    player::Player,
    variant::{load_variant, Variant},
};

fn show_moves_for_board(board: &Board, variant: &Variant) {
    board.print_board();

    for mv in generate_moves_for_player(&board, &Player::White) {
        println!("{}", mv.to_notation(&variant.style));
    }
}


fn main() {
    let variant = load_variant("variants/chess.toml");

    let mut board = Board::new(&variant);

    let bishop_move = Move {
        from: (0,2),
        to: (3,3),
        piece: String::from("B"),
        captured_pieces: vec![],
        promotion: None,
        special_move: None,
    };
    board.make_move(&bishop_move);

    let rook_move = Move {
        from: (0,0),
        to: (3,1),
        piece: String::from("R"),
        captured_pieces: vec![],
        promotion: None,
        special_move: None,
    };
    board.make_move(&rook_move);

    let king_move = Move {
        from: (0,4),
        to: (4,6),
        piece: String::from("K"),
        captured_pieces: vec![],
        promotion: None,
        special_move: None,
    };
    board.make_move(&king_move);

    show_moves_for_board(&board, &variant);
}
