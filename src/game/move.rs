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

impl Move {
    pub fn to_notation(&self, variant: &str) -> String {
        match variant {
            "Chess" => self.to_chess_notation(),
            "Shogi" | "ChuShogi" => self.to_shogi_notation(),
            _ => panic!("Unsupported variant"),
        }
    }

    /// **Chess Short Algebraic Notation**
    fn to_chess_notation(&self) -> String {
        if let Some(special) = &self.special_move {
            return match special {
                SpecialMove::CastlingKingside => "O-O".to_string(),
                SpecialMove::CastlingQueenside => "O-O-O".to_string(),
                SpecialMove::EnPassant => format!("{}x{} e.p.", self.file_rank_for_pos(self.from), self.file_rank_for_pos(self.to)),
                _ => panic!("Unsupported special move in Chess"),
            };
        }

        let capture_symbol = if !self.captured_pieces.is_empty() { "x" } else { "" };
        let piece_symbol =
            if self.piece == "P" {
                if capture_symbol == "x" {
                    &self.file_for_pos(self.from)
                } else {
                    ""
                }
            } else {
                &self.piece
            };
        let promo_suffix = self.promotion.as_ref().map_or("".to_string(), |p| format!("={}", p));

        format!(
            "{}{}{}",
            piece_symbol,
            capture_symbol,
            self.file_rank_for_pos(self.to)
        ) + &promo_suffix
    }

    /// **Shogi Variant Notation**
    fn to_shogi_notation(&self) -> String {
        let piece_symbol = &self.piece;

        if self.from == self.to {
            if self.captured_pieces.len() == 0 {
                return "pass".to_string(); // Jitto (pass move)
            } else {
                let capture_symbol = if self.from != self.to { "x" } else { "x!" };
                let mut capture_notation = format!("{}", piece_symbol);
                for (p0, p1, _) in &self.captured_pieces {
                    capture_notation += &format!("{}{}", capture_symbol, self.rank_file_for_pos((*p0,*p1)));
                }
                return capture_notation;
            }
        }

        if self.captured_pieces.len() > 1 {
            let capture_symbol = if self.from != self.to { "x" } else { "x!" };
            let mut capture_notation = format!("{}", piece_symbol);
            for (p0, p1, _) in &self.captured_pieces {
                capture_notation += &format!("{}{}", capture_symbol, self.rank_file_for_pos((*p0,*p1)));
            }
            return capture_notation;
        }

        let separator = if !self.captured_pieces.is_empty() { "x" } else { "-" };
        let formatted_to = self.rank_file_for_pos(self.to);

        // Handle drop moves (when moving from outside the board)
        if self.from.0 == usize::MAX && self.from.1 == usize::MAX {
            return format!("{}*{}", piece_symbol, formatted_to);
        }

        let promo_suffix = self.promotion.as_ref().map_or("".to_string(), |_| "+".to_string());

        format!("{}{}{}{}", piece_symbol, separator, formatted_to, promo_suffix)
    }

    fn file_for_pos(&self, pos: (usize, usize)) -> String {
        ((b'a' + pos.1 as u8) as char).to_string()
    }

    fn rank_for_pos(&self, pos: (usize, usize)) -> String {
        (pos.0 + 1).to_string()
    }

    fn file_rank_for_pos(&self, pos: (usize, usize)) -> String {
        format!("{}{}", self.file_for_pos(pos), self.rank_for_pos(pos))
    }

    fn rank_file_for_pos(&self, pos: (usize, usize)) -> String {
        format!("{}{}", self.rank_for_pos(pos), self.file_for_pos(pos))
    }

}
