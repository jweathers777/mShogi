use std::collections::HashMap;
use std::fs;
use toml;
use serde::Deserialize;
use crate::game::{board::Board, piece::{MovePattern, Piece, PieceType}, player::Player};

#[derive(Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub style: String,
    pub board_size: (usize, usize),
    pub player_names: [String; 2],
    pub piece_types: HashMap<String, PieceType>,
    pub pieces: HashMap<(usize, usize), Piece>,
}

#[derive(Debug, Deserialize)]
struct VariantData {
    name: String,
    style: String,
    board_size: [usize; 2],
    player_names: [String; 2],
    piece_types: HashMap<String, PieceTypeData>,
    starting_position: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct PieceTypeData {
    symbol: String,
    moves: Vec<String>,
    promotion: Option<Vec<String>>,
}

pub fn load_variant(file_path: &str) -> Variant {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let parsed: VariantData = toml::from_str(&content).expect("Failed to parse TOML");

    let piece_types: HashMap<String, PieceType> =
        parsed.piece_types.iter().map(|(name, data)| {
            let moves = data.moves.iter()
                .filter_map(|m| MovePattern::from_str(m))
                .collect();
            let piece_type = PieceType {
                name: name.clone(),
                symbol: data.symbol.clone(),
                moves,
                promotion: data.promotion.clone(),
            };
            (data.symbol.clone(), piece_type)
        }).collect();

    let pieces = parsed.starting_position.iter().map(|(pos_str, piece_code)| {
        let position = parse_position(pos_str)
            .expect(&format!("Invalid position format: {}", pos_str));

        let (owner_char, piece_type_code) = piece_code.split_at(1);
        let owner = Player::from_char(owner_char.chars().next().unwrap())
            .expect(&format!("Invalid owner character: {}", owner_char));

        let piece_type = piece_types.get(piece_type_code)
            .expect(&format!("Invalid piece type: {}", piece_type_code));

        (position, Piece { piece_type: piece_type.clone(), position, owner, move_count: 0 })
    }).collect();

    Variant {
        name: parsed.name,
        style: parsed.style,
        board_size: (parsed.board_size[0], parsed.board_size[1]),
        player_names: parsed.player_names,
        piece_types,
        pieces,
    }
}

fn parse_position(position: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = position.split(',').collect();
    if parts.len() == 2 {
        if let (Ok(row), Ok(col)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            return Some((row, col));
        }
    }
    None
}
