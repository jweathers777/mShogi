use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Piece {
    pub symbol: String,
    pub moves: Vec<String>,
    pub promotion: Option<Vec<String>>,
}
