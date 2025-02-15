#![allow(warnings)]

mod game;

use game::variant::load_variant;
use game::board::Board;
use game::player::Player;
use crate::game::move_generator::*;


fn main() {
    let chess_variant = load_variant("variants/chess.toml", "Chess");
    println!("Loaded Chess Variant: {:?}", chess_variant.name);

    let chess_board = Board::new(&chess_variant);
    println!("Chess Board:");
    chess_board.print_board();

    //let shogi_variant = load_variant("variants/shogi.toml", "Shogi");
    //println!("\n\nLoaded Shogi Variant: {:?}", shogi_variant.name);

    //let shogi_board = Board::new(&shogi_variant);
    //println!("Shogi Board:");
    //shogi_board.print_board();

    //let chu_shogi_variant = load_variant("variants/chu_shogi.toml", "ChuShogi");
    //println!("\n\nLoaded Chu Shogi Variant: {:?}", chu_shogi_variant.name);

    //let chu_shogi_board = Board::new(&chu_shogi_variant);
    //println!("Chu Shogi Board:");
    //chu_shogi_board.print_board();

    let chess_generator = ChessMoveGenerator;
    let moves = chess_generator.generate_moves(&chess_board, Player::White);
    for mv in moves {
            println!("{}", mv.to_notation("Chess"));
    }
}
