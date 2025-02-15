use crate::game::variant::Variant;
use crate::game::piece::Piece;

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub squares: Vec<Option<Piece>>, // Now storing actual Pieces instead of strings
}

impl Board {
    pub fn new(variant: &Variant) -> Self {
        let mut board = Self {
            width: variant.board_size.0,
            height: variant.board_size.1,
            squares: vec![None; variant.board_size.0 * variant.board_size.1],
        };

        for (position, piece_name) in &variant.starting_position {
            let coords: Vec<usize> = position
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if coords.len() == 2 {
                // Convert piece name to actual Piece
                if let Some(piece) = variant.pieces.get(piece_name) {
                    board.set(coords[0], coords[1], Some(piece.clone()));
                } else {
                    panic!("Unknown piece: {}", piece_name);
                }
            }
        }

        board
    }

	 pub fn get(&self, row: usize, col: usize) -> Option<&Piece> {
		  if row < self.height && col < self.width {
				self.squares[row * self.width + col].as_ref()
		  } else {
				None
		  }
    }

    pub fn set(&mut self, row: usize, col: usize, piece: Option<Piece>) {
        if row < self.height && col < self.width {
            self.squares[row * self.width + col] = piece;
        }
    }

    pub fn print_board(&self) {
        let max_symbol_length = self.squares.iter()
            .filter_map(|piece| piece.as_ref().map(|p| p.symbol.len()))
            .max()
            .unwrap_or(1); // Default to 1 for empty squares

        let (rows, cols) = (self.height, self.width);

        // Print column labels
        print!("   ");
        for col in 0..cols {
            print!("{:width$} ", (b'a' + col as u8) as char, width = max_symbol_length);
        }
        println!();

        for row in 0..rows {
            // Print row labels
            print!("{:2} ", rows - row);

            for col in 0..cols {
                match &self.squares[row * cols + col] {
                    Some(piece) => print!("{:width$} ", piece.symbol, width = max_symbol_length),
                    None => print!("{:width$} ", ".", width = max_symbol_length),
                }
            }
            println!();
        }
    }
}
