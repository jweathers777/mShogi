//! Exercise the unchanged, independently established MSH-2 input contracts.

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{Color, FenErrorCategory, Piece, PieceKind, Position, Square},
};
use serde_json::Value;

const INITIAL: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const AFTER_E4: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
const BARE: &str = "7k/8/8/8/8/8/8/K7 w - - 0 1";

fn square(file: u8, rank: u8) -> Square {
    Square::new(file, rank).unwrap()
}

#[test]
fn independent_corpus_fen_contracts_and_canonical_positions() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    assert_eq!(corpus["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let mut parsed = 0;
    let mut rejected = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert_eq!(case["rules_profile"], CHESS_POSITION_PROFILE_ID, "{id}");
        let input = case["input"]["fen"].as_str().unwrap();
        let expected = &case["expected"];
        let canonical = match case["operation"].as_str().unwrap() {
            "position" => input, // All 35 independent position inputs are canonical.
            "parse_fen" if expected["accepted"] == true => {
                expected["canonical_fen"].as_str().unwrap()
            }
            "parse_fen" => {
                let error = Position::from_fen(input).expect_err(id);
                assert_eq!(error.category().to_string(), expected["error"], "{id}");
                assert_eq!(Position::from_fen(input).unwrap_err(), error, "{id}");
                assert!(error.to_string().starts_with("FEN "), "{id}");
                let mut caller = Position::from_fen(INITIAL).unwrap();
                let before = caller.clone();
                assert_eq!(caller.set_fen(input), Err(error), "{id}");
                assert_eq!(caller, before, "{id}: failed input mutated caller state");
                rejected += 1;
                continue;
            }
            other => panic!("{id}: unknown operation {other}"),
        };
        let position = Position::from_fen(input).unwrap_or_else(|e| panic!("{id}: {e}"));
        assert_eq!(
            position.to_fen(),
            canonical,
            "{id}: independent canonical answer"
        );
        assert_eq!(
            canonical.parse::<Position>().unwrap(),
            position,
            "{id}: semantic round trip"
        );
        let mut caller = Position::from_fen(INITIAL).unwrap();
        caller.set_fen(input).unwrap();
        assert_eq!(caller, position, "{id}: full successful replacement");
        parsed += 1;
    }
    assert_eq!((parsed, rejected), (41, 27));
    // This test intentionally does not consume move/perft/classification answers:
    // MSH-4 implements positions, not the later behavior those fixtures exercise.
}

#[test]
fn getters_expose_the_full_position_tuple() {
    let position = Position::from_fen(AFTER_E4).unwrap();
    assert_eq!(position.side_to_move(), Color::Black);
    assert_eq!(position.en_passant_target(), Some(square(4, 2)));
    assert_eq!(position.halfmove_clock(), 0);
    assert_eq!(position.fullmove_number(), 1);
    for color in [Color::White, Color::Black] {
        assert!(position.castling_rights().kingside(color));
        assert!(position.castling_rights().queenside(color));
    }
    assert_eq!(position.castling_rights().to_string(), "KQkq");
    for (file, kind) in [
        PieceKind::Rook,
        PieceKind::Knight,
        PieceKind::Bishop,
        PieceKind::Queen,
        PieceKind::King,
        PieceKind::Bishop,
        PieceKind::Knight,
        PieceKind::Rook,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            position.piece_at(square(file as u8, 0)),
            Some(Piece {
                color: Color::White,
                kind
            })
        );
        assert_eq!(
            position.piece_at(square(file as u8, 7)),
            Some(Piece {
                color: Color::Black,
                kind
            })
        );
    }
    assert_eq!(position.piece_at(square(4, 1)), None);
    assert_eq!(
        position.piece_at(square(4, 3)),
        Some(Piece {
            color: Color::White,
            kind: PieceKind::Pawn
        })
    );
    assert_eq!(position.en_passant_target().unwrap().to_string(), "e3");
    for file in 0..=7 {
        for rank in 0..=7 {
            let sq = square(file, rank);
            assert_eq!((sq.file(), sq.rank()), (file, rank));
        }
    }
    for (file, rank) in [(8, 0), (0, 8), (255, 7), (7, 255)] {
        assert_eq!(Square::new(file, rank), None);
    }
}

#[test]
fn every_specified_ascii_whitespace_and_leading_zeroes_normalize() {
    for separator in [" ", "\t", "\n", "\r", "\x0b", "\x0c", " \r\n\t\x0b\x0c"] {
        let input = format!("{separator}{}{separator}", BARE.replace(' ', separator));
        assert_eq!(Position::from_fen(&input).unwrap().to_fen(), BARE);
    }
    let input = format!(
        "7k/8/8/8/8/8/8/K7 w - - {}5 {}39",
        "0".repeat(100),
        "0".repeat(100)
    );
    assert_eq!(
        Position::from_fen(&input).unwrap().to_fen(),
        "7k/8/8/8/8/8/8/K7 w - - 5 39"
    );
}

#[test]
fn earliest_category_wins_across_fields_and_no_failure_mutates_state() {
    use FenErrorCategory::{Range, Structure, Syntax};
    for (input, category) in [
        ("8/8/8/8/8/8/8/8 w - - 4294967296 junk", Syntax),
        ("8/8/8/8/8/8/8/8 w - zz 4294967296 1", Syntax),
        ("8/8/8/8/8/8/8/8 w - - 4294967296 1", Range),
        ("8/8/8/8/8/8/8/8 w - - 0 0", Range),
        ("8/8/8/8/8/8/8/8 w - - 0 1", Structure),
        ("7k/8/8/8/8/8/8/K7 w - - 0 4294967296", Range),
        ("7k/8/8/8/8/8/8/K7 w - - +0 1", Syntax),
        ("7k/8/8/8/8/8/8/K7 w - - 0 1e2", Syntax),
        ("7k/8/8/8/8/8/8/K7 w - a9 0 1", Syntax),
        ("7k/8/8/8/8/8/8/K7 w - a4 0 1", Structure),
        ("7k/8/8/8/8/8/8/K7 w - - 0 1\0", Syntax),
    ] {
        let mut caller = Position::from_fen(AFTER_E4).unwrap();
        let before = caller.clone();
        let error = caller.set_fen(input).unwrap_err();
        assert_eq!(error.category(), category, "{input}");
        assert_eq!(caller, before, "{input}");
    }
}

#[test]
fn castling_flags_are_permissions_not_derived_from_current_paths() {
    // Home pieces do not invent rights; blocked/attacked paths do not erase them.
    let no_rights = Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1").unwrap();
    assert_eq!(no_rights.castling_rights().to_string(), "-");
    for rights in ["K", "Q", "k", "q", "Kq", "Qk", "KQkq"] {
        let input = format!("r3k2r/8/8/8/2b5/8/8/RN2K2R w {rights} - 0 1");
        assert_eq!(
            Position::from_fen(&input)
                .unwrap()
                .castling_rights()
                .to_string(),
            rights
        );
    }
    for input in [
        "4k3/8/8/8/8/8/8/4K2r w K - 0 1", // Wrong-color rook.
        "4k2r/8/8/8/8/8/8/K7 b q - 0 1",  // Missing queenside rook.
        "3k3r/8/8/8/8/8/8/K7 b k - 0 1",  // King off its home square.
    ] {
        assert_eq!(
            Position::from_fen(input).unwrap_err().category(),
            FenErrorCategory::Structure
        );
    }
}

#[test]
fn raw_ep_and_counters_belong_to_saved_identity_but_not_a_history_claim() {
    let with_ep = Position::from_fen(AFTER_E4).unwrap();
    let without_ep = Position::from_fen(&AFTER_E4.replace(" e3 ", " - ")).unwrap();
    assert_ne!(with_ep, without_ep); // Same board and legal actions, different raw state.
    let later_number = Position::from_fen(&AFTER_E4.replace(" 0 1", " 0 2")).unwrap();
    assert_ne!(with_ep, later_number);
    let a = Position::from_fen(BARE).unwrap();
    let b = Position::from_fen(&BARE.replace(" 0 1", " 1 1")).unwrap();
    assert_ne!(a, b);
    let max = Position::from_fen("7k/8/8/8/8/8/8/K7 w - - 4294967295 4294967295").unwrap();
    assert_eq!(
        (max.halfmove_clock(), max.fullmove_number()),
        (u32::MAX, u32::MAX)
    );
    // Both colors may retain an uncapturable but locally consistent target.
    let black_ep = Position::from_fen("7k/8/8/8/3P4/8/8/K7 b - d3 0 2").unwrap();
    assert_eq!(black_ep.en_passant_target(), Some(square(3, 2)));
    for input in [
        "7k/8/8/8/8/8/8/K7 b - d3 0 2",     // No double-pushed pawn.
        "7k/8/8/8/3P4/3n4/8/K7 b - d3 0 2", // Occupied target.
        "7k/8/8/8/3P4/8/3B4/K7 b - d3 0 2", // Occupied origin.
        "7k/8/8/8/3P4/8/8/K7 b - d3 1 2",   // Pawn move did not reset clock.
    ] {
        assert_eq!(
            Position::from_fen(input).unwrap_err().category(),
            FenErrorCategory::Structure
        );
    }
}

#[test]
fn local_king_safety_uses_geometric_attacks_including_pinned_pieces() {
    for input in [
        "7k/7R/8/8/8/8/8/K7 w - - 0 1",      // Rook file.
        "7k/8/8/8/8/2B5/8/K7 w - - 0 1",     // Bishop diagonal.
        "7k/8/8/8/3Q4/8/8/K7 w - - 0 1",     // Queen diagonal.
        "7k/8/8/8/7Q/8/8/K7 w - - 0 1",      // Queen file.
        "7k/5N2/8/8/8/8/8/K7 w - - 0 1",     // Knight jump.
        "7k/6P1/8/8/8/8/8/K7 w - - 0 1",     // White pawn attack.
        "7k/8/8/8/8/8/1p6/K7 b - - 0 1",     // Black pawn attack.
        "4r3/8/8/8/3k4/8/4N3/4K3 w - - 0 1", // Pinned e2 knight still attacks d4.
    ] {
        let error = Position::from_fen(input).unwrap_err();
        assert_eq!(error.category(), FenErrorCategory::Structure, "{input}");
        assert!(
            error.to_string().contains("nonmoving king"),
            "{input}: {error}"
        );
    }
    for input in [
        "7k/6p1/8/8/8/2B5/8/K7 w - - 0 1",   // Bishop ray blocked.
        "7k/7p/8/8/7R/8/8/K7 w - - 0 1",     // Rook ray blocked.
        "7k/8/8/8/6P1/8/8/K7 w - - 0 1",     // Pawn does not slide up diagonal.
        "7k/8/8/8/7P/8/8/K7 w - - 0 1",      // Pawn does not attack up its file.
        "7k/8/8/8/8/8/8/K7 b - - 0 1",       // Enemy king is not a sliding piece.
        "4r2k/8/8/8/8/8/8/4K3 w - - 0 1",    // Moving-side check is allowed.
        "4r2k/8/8/8/1b6/8/8/R3K3 w - - 0 1", // Moving-side double check allowed.
    ] {
        Position::from_fen(input).unwrap_or_else(|error| panic!("{input}: {error}"));
    }
}

#[test]
fn bounded_malformed_inputs_never_panic_or_partially_replace() {
    // Deterministic robustness checks, not expected rule answers or parser fuzzing infrastructure.
    let mut seed = 0x4d534834u64;
    let alphabet = b"PNBRQKpnbrqk0123456789/ -wbahe+\t\r\n";
    let initial = Position::from_fen(INITIAL).unwrap();
    for length in 0..192 {
        for _ in 0..16 {
            let mut input = String::new();
            for _ in 0..length {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                input.push(alphabet[(seed >> 32) as usize % alphabet.len()] as char);
            }
            let mut caller = initial.clone();
            match caller.set_fen(&input) {
                Err(_) => assert_eq!(caller, initial),
                Ok(()) => assert_eq!(Position::from_fen(&caller.to_fen()).unwrap(), caller),
            }
        }
    }
}
