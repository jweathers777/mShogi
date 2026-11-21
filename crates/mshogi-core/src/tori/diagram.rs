//! Compact Tori diagram: the board as `owner:face:square` entries, the hands
//! as face-code counts and the side to move, as JSON. The strength tools keep
//! positions in this form (tactical suite, Texel corpus). Counters and history
//! are not part of a diagram.
use super::{Face, Hand, Piece, Position, RootMetadata, Side, Square};
use serde_json::{Map, Value, json};

const fn side_code(side: Side) -> &'static str {
    match side {
        Side::Sente => "s",
        Side::Gote => "g",
    }
}
fn side_of(code: &str) -> Result<Side, String> {
    match code {
        "s" => Ok(Side::Sente),
        "g" => Ok(Side::Gote),
        _ => Err(format!("unknown side {code:?}")),
    }
}
fn face_of(code: &str) -> Result<Face, String> {
    Face::ALL
        .into_iter()
        .find(|f| f.code() == code)
        .ok_or_else(|| format!("unknown face {code:?}"))
}

impl Position {
    /// The compact diagram of this position.
    pub fn compact_diagram(&self) -> Value {
        let board: Vec<String> = Square::all()
            .filter_map(|s| {
                self.piece(s)
                    .map(|p| format!("{}:{}:{s}", side_code(p.side), p.face.code()))
            })
            .collect();
        let hands: Map<String, Value> = [Side::Sente, Side::Gote]
            .into_iter()
            .map(|side| {
                let hand = self.hand(side);
                let counts: Map<String, Value> = Face::HANDS
                    .into_iter()
                    .filter(|&f| hand.count(f) > 0)
                    .map(|f| (f.code().to_string(), json!(hand.count(f))))
                    .collect();
                (side_code(side).to_string(), Value::Object(counts))
            })
            .collect();
        json!({"board": board, "hands": hands,
               "side_to_move": side_code(self.side_to_move())})
    }

    /// The diagram root of a compact diagram, structurally validated.
    pub fn from_compact_diagram(d: &Value) -> Result<Self, String> {
        let mut board = Vec::new();
        for entry in d["board"].as_array().ok_or("missing board")? {
            let parts: Vec<&str> = entry.as_str().ok_or("board entry")?.split(':').collect();
            let [side, face, square] = parts[..] else {
                return Err(format!("board entry {entry}"));
            };
            let square: Square = square
                .parse()
                .map_err(|_| format!("board square {square:?}"))?;
            board.push((
                square,
                Piece {
                    side: side_of(side)?,
                    face: face_of(face)?,
                },
            ));
        }
        let mut hands = [Hand::empty(); 2];
        for (index, side) in ["s", "g"].into_iter().enumerate() {
            let mut counts = Vec::new();
            for (face, count) in d["hands"][side].as_object().ok_or("missing hand")? {
                let count = count.as_u64().ok_or("hand count")?;
                counts.push((
                    face_of(face)?,
                    u8::try_from(count).map_err(|e| e.to_string())?,
                ));
            }
            hands[index] = Hand::from_counts(&counts).map_err(|e| format!("{e:?}"))?;
        }
        let side = side_of(d["side_to_move"].as_str().ok_or("missing side to move")?)?;
        Self::from_diagram(&board, hands, side, RootMetadata::diagram(0))
            .map_err(|e| format!("{e:?}"))
    }

    /// The diagram root of a compact diagram in JSON text.
    pub fn from_compact_diagram_text(text: &str) -> Result<Self, String> {
        Self::from_compact_diagram(&serde_json::from_str(text).map_err(|e| e.to_string())?)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{LocalPosition, Position};

    #[test]
    fn compact_diagram_round_trip_keeps_board_hands_and_side() {
        let mut local = LocalPosition::admit(Position::standard()).unwrap();
        for _ in 0..30 {
            let a = local.legal_actions()[3 % local.legal_actions().len()];
            local.apply(a).unwrap();
        }
        let d = local.position().compact_diagram();
        let back = Position::from_compact_diagram_text(&d.to_string()).unwrap();
        assert_eq!(back.compact_diagram(), d);
        assert_eq!(back.side_to_move(), local.position().side_to_move());
        assert!(Position::from_compact_diagram_text("{}").is_err());
        assert!(
            Position::from_compact_diagram_text(
                r#"{"board":["x:Ph:4g"],"hands":{"s":{},"g":{}},"side_to_move":"s"}"#
            )
            .is_err()
        );
    }
}
