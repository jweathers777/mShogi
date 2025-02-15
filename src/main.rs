#![allow(warnings)]

mod game;

use game::{
    board::Board,
    move_generator::generate_moves_for_player,
    player::Player,
    variant::load_variant,
};

fn main() {
    let variant = load_variant("variants/chess.toml");

    let board = Board::new(&variant);
    board.print_board();

    let test_position = (1, 0); // Example: Pawn at a2
    if let Some(piece) = board.get(test_position) {
        let legal_moves = generate_moves_for_player(&board, &Player::White);

        for mv in legal_moves {
            println!("{}", mv.to_notation(&variant.style));
        }
    }
}
