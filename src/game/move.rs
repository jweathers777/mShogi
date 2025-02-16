#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialMove {
    CastlingKingside,
    CastlingQueenside,
    EnPassant,
    LionSecondMove,  // For Chu Shogi Lion second move
}

#[derive(Debug, Clone)]
pub struct Move {
    pub from: (usize, usize),   // Start square (row, col)
    pub to: (usize, usize),     // End square (row, col)
    pub piece: String,          // The piece making the move (e.g., "P", "K", "+P")
    pub captured_pieces: Vec<(usize, usize, String)>, // Captured pieces (position, piece symbol)
    pub promotion: Option<String>, // Promotion symbol (e.g., "+P")
    pub special_move: Option<SpecialMove>, // Castling, en passant, etc.
}
