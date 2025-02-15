use crate::game::board::Board;
use crate::game::player::Player;
use crate::game::r#move::Move;

pub mod chess;
pub mod shogi;
pub mod chu_shogi;

pub use chess::ChessMoveGenerator;
pub use shogi::ShogiMoveGenerator;
pub use chu_shogi::ChuShogiMoveGenerator;

pub trait MoveGenerator {
    fn generate_moves(&self, board: &Board, player: Player) -> Vec<Move>;
}

