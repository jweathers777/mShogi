use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml;

use crate::game::piece::Piece;

#[derive(Debug, Deserialize, Clone)]
pub struct Variant {
    pub name: String,
    pub board_size: (usize, usize),
    pub player_names: [String; 2],
    pub pieces: HashMap<String, Piece>,
    pub starting_position: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct VariantsWrapper {
    variants: HashMap<String, Variant>,
}

pub fn load_variant(file_path: &str, variant_name: &str) -> Variant {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let parsed: VariantsWrapper = toml::from_str(&content).expect("Failed to parse TOML");

    parsed.variants.get(variant_name)
        .expect(&format!("Variant {} not found", variant_name))
        .clone()
}
