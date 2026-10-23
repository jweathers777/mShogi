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

    pub fn forward(&self) -> isize {
        match self {
            Player::White => 1,
            Player::Black => -1
        }
    }
    pub fn forward_diagonals(&self) -> Vec<(isize,isize)> {
        match self {
            Player::White => vec![(1, -1), (1, 1)],
            Player::Black => vec![(-1, -1), (-1, 1)],
        }
    }
}
