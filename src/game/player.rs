#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    White,  // Chess
    Black,  // Chess
    Sente,  // Shogi Variants
    Gote,   // Shogi Variants
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
            Player::Sente => Player::Gote,
            Player::Gote => Player::Sente,
        }
    }
}
