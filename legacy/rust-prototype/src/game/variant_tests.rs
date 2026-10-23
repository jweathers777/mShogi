#[cfg(test)]
mod tests {
    use crate::game::variant::Variant;

    #[test]
    fn test_load_chess_variant() {
        let variant = Variant::load_from_file("variants/chess.toml");
        assert_eq!(variant.name, "Chess");
        assert_eq!(variant.board_size, (8, 8));
        assert!(variant.piece_types.contains_key("P"));
        assert!(variant.pieces.contains_key(&(1, 0)));
    }

    #[test]
    fn test_load_shogi_variant() {
        let variant = Variant::load_from_file("variants/shogi.toml");
        assert_eq!(variant.name, "Shogi");
        assert_eq!(variant.board_size, (9, 9));
        assert!(variant.piece_types.contains_key("L"));
        assert!(variant.pieces.contains_key(&(6, 0)));
    }
}
