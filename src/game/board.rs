use std::collections::HashMap;
use crate::game::{r#move::Move, piece::Piece, player::Player, variant::Variant};

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub pieces: HashMap<(usize, usize), Piece>,
    pub move_history: Vec<Move>,
    pub captured_pieces: HashMap<Player, Vec<Piece>>,
}

impl Board {
    pub fn new(variant: &Variant) -> Self {
        let mut board = Self {
            width: variant.board_size.0,
            height: variant.board_size.1,
            pieces: HashMap::new(),
            move_history: vec![],
            captured_pieces: HashMap::from(
                [(Player::White, vec![]), (Player::Black, vec![])]
            ),
        };

        for (position, piece) in &variant.pieces {
            let mut piece = piece.clone();
            piece.position = *position;
            board.pieces.insert(*position, piece);
        }

        board
    }

    pub fn is_within_bounds(&self, row: isize, col: isize) -> bool {
        row >= 0 && col >= 0 && row < self.height as isize && col < self.width as isize
    }

    pub fn is_within_row_bounds(&self, row: isize) -> bool {
        row >= 0 && row < self.height as isize
    }

    pub fn is_within_col_bounds(&self, col: isize) -> bool {
        col >= 0 && col < self.width as isize
    }

	 pub fn get(&self, position: (usize, usize)) -> Option<&Piece> {
        self.pieces.get(&position)
    }

    pub fn set(&mut self, position: (usize, usize), piece: Option<Piece>) {
        match piece {
            Some(mut p) => {
                p.position = position;
                self.pieces.insert(position, p);
            }
            None => {
                self.pieces.remove(&position);
            }
        }
    }

	 pub fn make_move(&mut self, mv: &Move) {
        // Capture pieces
        for &(cap_row, cap_col, ref _cap_piece) in &mv.captured_pieces {
            if let Some(captured_piece) = self.pieces.remove(&(cap_row, cap_col)) {
                self.captured_pieces
                    .entry(captured_piece.owner)
                    .or_insert_with(Vec::new)
                    .push(captured_piece);
            }
        }

        // Move the piece
        if let Some(mut piece) = self.pieces.remove(&mv.from) {
            piece.move_count += 1;
            piece.position = mv.to;
            self.pieces.insert(mv.to, piece);
        }

        self.move_history.push(mv.clone());
    }

    pub fn print_captured_pieces(&self) {
        println!("Captured Pieces:");
        for (player, pieces) in &self.captured_pieces {
            print!("{}: ", player.prefix());
            for piece in pieces {
                print!("{} ", piece.piece_type.symbol);
            }
            println!();
        }
    }

    pub fn print_board(&self) {
        let max_symbol_length = self.pieces.values()
            .map(|p| p.piece_type.symbol.len())
            .max()
            .unwrap_or(1); // Default to 1 for empty squares

        // Print column labels
        print!("   ");
        for col in 0..self.width {
            print!(" {:width$} ", (b'a' + col as u8) as char, width = max_symbol_length);
        }
        println!();

        for row in (0..self.height).rev() {
            // Print row labels
            print!("{:2} ", row + 1);

            for col in 0..self.width {
                if let Some(piece) = self.pieces.get(&(row, col)) {
                    print!("{}{:width$} ", piece.owner.prefix(),
                        piece.piece_type.symbol, width = max_symbol_length);
                } else {
                    print!(" {:width$} ", ".", width = max_symbol_length);
                }
            }
            println!();
        }
    }
}
