#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'w' => Some(Player::White),
            'b' => Some(Player::Black),
            _ => None,
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            Player::White => "w",
            Player::Black => "b",
        }
    }

    pub fn opponent(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}
