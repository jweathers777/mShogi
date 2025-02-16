use crate::game::r#move::{Move,SpecialMove};

#[derive(Debug, Clone)]
pub struct Notator {
    pub style: String, // "Chess", "Shogi", etc.
    pub board_size: (usize, usize),
}

impl Notator {
    pub fn new(style: &str, board_size: (usize, usize)) -> Self {
        Self {
            style: style.to_string(),
            board_size,
        }
    }

    /// Converts (row, col) into notation (e.g., `"e2"` for Chess or `"5e"` for Shogi).
    pub fn pos_to_notation(&self, pos: (usize, usize)) -> String {
        match self.style.as_str() {
            "Chess" => format!("{}{}", (b'a' + pos.1 as u8) as char, pos.0 + 1),
            "Shogi" => format!("{}{}", pos.0 + 1, (b'a' + pos.1 as u8) as char),
            _ => panic!("Unsupported notation style"),
        }
    }

    /// Converts a notation (e.g., `"e2"`) into a (row, col) position.
    pub fn notation_to_pos(&self, notation: &str) -> (usize, usize) {
        match self.style.as_str() {
            "Chess" => {
                let mut chars = notation.chars();
                let file = chars.next().unwrap() as u8 - b'a';
                let rank = chars.next().unwrap().to_digit(10).unwrap() as usize - 1;
                (rank, file as usize)
            }
            "Shogi" => {
                let mut chars = notation.chars();
                let rank = chars.next().unwrap().to_digit(10).unwrap() as usize - 1;
                let file = chars.next().unwrap() as u8 - b'a';
                (rank, file as usize)
            }
            _ => panic!("Unsupported notation style"),
        }
    }

    /// Converts a `Move` into its notated string.
    pub fn move_to_notation(&self, mv: &Move) -> String {
        match self.style.as_str() {
            "Chess" => self.chess_move_notation(mv),
            "Shogi" => self.shogi_move_notation(mv),
            _ => panic!("Unsupported variant"),
        }
    }

    fn chess_move_notation(&self, mv: &Move) -> String {
        if let Some(special) = &mv.special_move {
            return match special {
                SpecialMove::CastlingKingside => "O-O".to_string(),
                SpecialMove::CastlingQueenside => "O-O-O".to_string(),
                SpecialMove::EnPassant => format!("{}x{} e.p.", self.pos_to_notation(mv.from), self.pos_to_notation(mv.to)),
                _ => panic!("Unsupported special move in Chess"),
            };
        }

        let capture_symbol = if !mv.captured_pieces.is_empty() { "x" } else { "" };
        let piece_symbol = if mv.piece == "P" {
            if capture_symbol == "x" {
                self.pos_to_notation(mv.from).chars().next().unwrap().to_string()
            } else {
                "".to_string()
            }
        } else {
            mv.piece.clone()
        };
        let promo_suffix = mv.promotion.as_ref().map_or("".to_string(), |p| format!("={}", p));

        format!("{}{}{}", piece_symbol, capture_symbol, self.pos_to_notation(mv.to)) + &promo_suffix
    }

    fn shogi_special_capture_notation(
        &self,
        piece_symbol: &str,
        capture_symbol: &str,
        mv: &Move
    ) -> String
    {
        let mut capture_notation = format!("{}", piece_symbol);
        for (p0, p1, _) in &mv.captured_pieces {
            let formatted_to = self.pos_to_notation((*p0,*p1));
            capture_notation += &format!("{}{}", capture_symbol, formatted_to);
        }
        capture_notation
    }

    fn shogi_move_notation(&self, mv: &Move) -> String {
        let piece_symbol = &mv.piece;

        // Handle special capture situations such as jitto or
        // igui or multiple captures
        let mut capture_symbol = "x";
        if mv.from == mv.to {
            if mv.captured_pieces.len() == 0 {
                return "--".to_string(); // jitto (pass move)
            } else {
                // igui (capture and return to starting position)
                return self.shogi_special_capture_notation(piece_symbol, "x!", mv);
            }
        }

        // Handle multiple captures
        if mv.captured_pieces.len() > 1 {
            return self.shogi_special_capture_notation(piece_symbol, "x", mv);
        }

        let separator = if !mv.captured_pieces.is_empty() { "x" } else { "-" };
        let formatted_to = self.pos_to_notation(mv.to);

        // Handle drop moves
        if mv.from == (usize::MAX, usize::MAX) {
            return format!("{}*{}", piece_symbol, formatted_to);
        }

        let promo_suffix = mv.promotion.as_ref().map_or("".to_string(), |_| "+".to_string());

        format!("{}{}{}{}", piece_symbol, separator, formatted_to, promo_suffix)
    }
}
