#[cfg(test)]
mod tests {
    use super::load_variant;

    #[test]
    fn test_load_shogi_variant() {
        let variant = load_variant("variants/shogi.toml");
        assert_eq!(variant.board_size, (9, 9));
        assert_eq!(variant.player_names[0], "Sente");
        assert_eq!(variant.player_names[1], "Gote");
    }

    #[test]
    fn test_load_chu_shogi_variant() {
        let variant = load_variant("variants/chu_shogi.toml");
        assert_eq!(variant.board_size, (12, 12));
        assert_eq!(variant.player_names[0], "Sente");
        assert_eq!(variant.player_names[1], "Gote");
    }
}
